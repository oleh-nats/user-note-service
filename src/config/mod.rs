use clap::Parser;
use config::{builder::DefaultState, ConfigBuilder, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CommandLine {
    pub config_path: PathBuf,
}

impl CommandLine {
    pub fn load_configurations(self) -> anyhow::Result<Config> {
        let config = ConfigBuilder::<DefaultState>::default()
            .add_source(File::from(self.config_path))
            .build()?;

        let data: Config = config.try_deserialize()?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub postgres: Database,
    pub web: Web,
    pub jwt_secret: String,
    pub hash_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub database_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Web {
    pub bind: String,
}
