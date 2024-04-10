use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::time::Duration;

pub async fn init_connection(path: &str) -> anyhow::Result<DatabaseConnection> {
    #[derive(Debug, Deserialize)]
    struct Config {
        url: String,
        max_connections: Option<u32>,
        min_connections: Option<u32>,
        connect_timeout: Option<u64>,
    }
    let config = crate::utils::config_util::deserialize_config::<Config>(path)?;
    let mut opt = ConnectOptions::new(config.url);
    opt.max_connections(config.max_connections.unwrap_or(5))
        .min_connections(config.min_connections.unwrap_or(5))
        .connect_timeout(Duration::from_secs(config.connect_timeout.unwrap_or(10)));
    let db = Database::connect(opt).await?;
    Ok(db)
}
