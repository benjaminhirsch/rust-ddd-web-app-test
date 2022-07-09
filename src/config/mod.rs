use dotenv::dotenv;
use eyre::WrapErr;
use eyre::{Report, Result};
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i64,
    pub database_url: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<config::Config, Report> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let config_builder = config::Config::builder();
        config_builder
            .add_source(config::Environment::default())
            .build()
            .context("Loading configuration from environment")
    }
}
