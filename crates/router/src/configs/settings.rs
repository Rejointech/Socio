use crate::core::errors::{ApplicationError, ApplicationResult};
use config::{Config, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Settings {
    pub server: Server,
    pub master_database: Database,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Server {
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
    pub pool_size: u32,
    pub connection_timeout: u64,
}

impl Settings {
    pub fn new() -> ApplicationResult<Self> {
        Self::with_config_path("config/development.toml")
    }

    pub fn with_config_path(config_path: &str) -> ApplicationResult<Self> {
        let config_path = PathBuf::from(config_path);

        let config = Config::builder()
            .add_source(File::from(config_path).required(true))
            .build()?;

        serde_path_to_error::deserialize(config).map_err(|error| {
            eprintln!("Unable to deserialize application configuration: {error}");
            ApplicationError::from(error.into_inner())
        })
    }
}
