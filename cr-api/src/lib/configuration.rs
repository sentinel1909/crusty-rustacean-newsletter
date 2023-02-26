//! src/lib/configuration.rs

use std::u16;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
  pub database: DatabaseSettings,
  pub application_port: u16
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
  pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  // Initialize our configuration reader
  let settings = config::Config::builder()
    // Add configuration values form a file name `configuration.yaml`
    .add_source(
        config::File::new("configuration.yaml", config::FileFormat::Yaml)
    )
    .build()?;
  // Try to convert the configuration values it read into
  // our Settings type
  settings.try_deserialize::<Settings>()
}