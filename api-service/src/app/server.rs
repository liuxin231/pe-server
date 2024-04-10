use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::{middleware, serve, Router};
use tokio::net::TcpListener;
use tower_http::limit::RequestBodyLimitLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use common::log::setup_logger;
use migration::sea_orm::{DatabaseBackend, DatabaseConnection, Statement};
use migration::{ConnectionTrait, MigratorTrait};

use crate::app::callback::Callback;
use crate::app::config::AppConfig;
use crate::app::middleware::handle_response;
use crate::app::router::get_all_routers;
use crate::app::state::AppState;
use crate::app::swagger::ApiDoc;

pub struct AppServer {
    pub app_config_file_path: String,
    pub log_config_file_path: String,
    pub postgres_config_file_path: String,
}
impl AppServer {
    pub async fn init_all(&self) -> anyhow::Result<()> {
        self.init_logger()?;
        let app_config = self.init_app_config()?;
        let db_connection = self.init_postgres_connection().await?;
        let app_state = AppState {
            db_conn: Arc::new(db_connection),
        };
        self.init_postgres_tables(app_state.db_conn.as_ref())
            .await?;
        let listener = TcpListener::bind(app_config.get_addr()).await?;
        log::info!(
            "Tcp listener successfully, listener address: {:?}",
            listener.local_addr()?
        );
        tokio::join!(self.init_server(listener, app_state), Callback::setup());
        Ok(())
    }
    async fn init_server(&self, tcp_listener: TcpListener, app_state: AppState) {
        let routers = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .nest("/api", get_all_routers(app_state))
            .fallback(Callback::fallback)
            .layer(middleware::from_fn(handle_response))
            .layer(DefaultBodyLimit::disable())
            .layer(RequestBodyLimitLayer::new(
                1024 * 1024 * 1024, /* 1024mb */
            ));
        serve(tcp_listener, routers).await.unwrap();
    }
    fn init_logger(&self) -> anyhow::Result<()> {
        setup_logger(&self.log_config_file_path)?;
        log::info!("Initializes the global logger as a log4rs logger configured via a file");
        Ok(())
    }
    fn init_app_config(&self) -> anyhow::Result<AppConfig> {
        let app_config = AppConfig::init(&self.app_config_file_path)?;
        log::info!("Initialize configuration completed");
        Ok(app_config)
    }
    async fn init_postgres_connection(&self) -> anyhow::Result<DatabaseConnection> {
        log::info!("start connecting to database...");
        let db_connection =
            common::connection::postgres::init_connection(&self.postgres_config_file_path).await?;
        log::info!("Database connection successful");
        Ok(db_connection)
    }
    async fn init_postgres_tables(&self, conn: &DatabaseConnection) -> anyhow::Result<()> {
        log::info!("Initialize database tables...");
        self.clear_migrations_version(conn).await;
        migration::Migrator::up(conn, None).await?;
        log::info!("Successfully initialized database table");
        Ok(())
    }
    async fn clear_migrations_version(&self, conn: &DatabaseConnection) {
        let result = conn
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                "drop table if exists seaql_migrations;",
            ))
            .await;
        match result {
            Ok(_) => {
                log::info!("clear migrations success.")
            }
            Err(err) => {
                log::error!("clear migrations error: {}", err.to_string());
            }
        }
    }
}
