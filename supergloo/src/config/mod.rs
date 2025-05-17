pub mod error;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct GlooConfig {
    pub host_address: String,
    pub port: u16,
}

impl GlooConfig {
    pub fn from_toml(path: &str) -> Result<Self, error::ConfigError> {
        let raw_config = std::fs::read_to_string(path)?;

        let config = toml::from_str(&raw_config)?;

        Ok(config)
    }
}