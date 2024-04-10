use crate::app::server::AppServer;

mod app;
mod file;
mod knowledge;
mod pe;
mod test;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_server = AppServer {
        app_config_file_path: "config/api-service.yaml".to_string(),
        log_config_file_path: "config/log.yaml".to_string(),
        postgres_config_file_path: "config/postgres.yaml".to_string(),
    };
    app_server.init_all().await?;
    Ok(())
}
