use derive_more::{Debug, From};

use crate::config;


#[derive(From, Debug)]
pub enum Error {

    #[from]
    Config(config::error::ConfigError),

    #[from]
    Tokio(tokio::io::Error)
}