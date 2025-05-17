
#[derive(Debug, derive_more::From)]
pub enum ConfigError {
    FileReadError(std::io::Error),
    ParsingError(toml::de::Error),
}