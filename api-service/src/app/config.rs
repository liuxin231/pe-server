use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    server: ServerConfig,
}
#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    host: Option<String>,
    port: Option<u32>,
}

impl AppConfig {
    pub fn get_addr(&self) -> String {
        let addr = format!(
            "{}:{}",
            self.server.host.clone().unwrap_or("0.0.0.0".to_string()),
            self.server.port.unwrap_or(8080)
        );
        addr
    }
    pub fn init(path: &str) -> anyhow::Result<AppConfig> {
        common::utils::config_util::deserialize_config::<AppConfig>(path)
    }
}
