use anyhow::{bail, Context};
use config::Config;
use domain::HmacSecret;
use serde_aux::prelude::*;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
use std::str::FromStr;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: HmacSecret,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub url: String,
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> Result<SqliteConnectOptions, sqlx::Error> {
        SqliteConnectOptions::from_str(&self.url)
            .map(|opt| opt.log_statements(tracing::log::LevelFilter::Trace))
    }
}

impl Settings {
    pub fn read_from_file() -> anyhow::Result<Settings> {
        let base_path = std::env::current_dir().context("Failed to get current dir.")?;
        let mut configuration_dir = base_path.join("configuration");
        if !configuration_dir.exists() {
            configuration_dir = base_path.join("../configuration");
        }

        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".to_string())
            .try_into()
            .context("Failed to parse APP_ENVIRONMENT.")?;

        let config = Config::builder()
            .add_source(config::File::from(configuration_dir.join("base")).required(true))
            .add_source(
                config::File::from(configuration_dir.join(environment.as_str())).required(true),
            )
            // Add in settings from environment variables (with a prefix of 'LB_' and '__' as separator)
            // E.g. `LB_APPLICATION__BASE_URL=http://example.com` would set `Settings.application.base_url`
            .add_source(
                config::Environment::with_prefix("LB")
                    .prefix_separator("_")
                    .separator("__"),
            );

        Ok(config.build()?.try_deserialize()?)
    }
}

#[derive(Debug)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => {
                bail!("{other} is not a supported environment. Use either `local` or `production`.")
            }
        }
    }
}
