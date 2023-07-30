use clap::Parser;
use config::{builder::DefaultState, ConfigBuilder, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CommandLine {
    pub config_path: PathBuf,
}

impl CommandLine {
    pub fn load_configurations<'a>(self) -> anyhow::Result<Config<'a>> {
        let config = ConfigBuilder::<DefaultState>::default()
            .add_source(File::from(self.config_path))
            .build()?;

        let data: Config = config.try_deserialize()?;
        Ok(data)
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Config<'a> {
    pub postgres: Database<'a>,
    pub web: Web<'a>,
    pub jwt_secret: &'a str,
    pub hash_secret: &'a str,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Database<'a> {
    pub database_url: &'a str,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Web<'a> {
    pub bind: &'a str,
}
