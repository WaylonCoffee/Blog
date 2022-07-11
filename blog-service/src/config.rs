use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: WebConfig,
    pub pg: deadpool_postgres::Config,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        cfg.try_deserialize()
    }
}
