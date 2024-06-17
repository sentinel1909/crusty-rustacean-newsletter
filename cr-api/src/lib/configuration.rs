// src/lib/configuration.rs

// dependencies
use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use confik::{Configuration, EnvSource, Error, FileSource};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

// a struct to hold a type for settings
#[derive(Clone, Deserialize, Configuration)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailClientSettings,
    pub redis: RedisSettings,
}

// a struct to hold a type for the Redis related settings
#[derive(Clone, Debug, Deserialize, Configuration)]
pub struct RedisSettings {
    #[confik(secret)]
    pub uri: String,
}

// a struct to hold a type for application settings
#[derive(Clone, Deserialize, Configuration)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    #[confik(secret)]
    pub hmac_secret: String,
}

// a struct to hold a type for email client settings
#[derive(Clone, Deserialize, Configuration)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    #[confik(secret)]
    pub authorization_token: String,
    pub timeout_milliseconds: u64,
}

// implement sender and timeout functions for Email Client
impl EmailClientSettings {
    pub fn client(self) -> EmailClient {
        let sender_email = self.sender().expect("Invalid sender email address.");
        let timeout = self.timeout();
        EmailClient::new(
            self.base_url,
            sender_email,
            self.authorization_token.into(),
            timeout,
        )
    }

    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }

    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}

// a struct to hold a type for database settings
#[derive(Clone, Deserialize, Configuration)]
pub struct DatabaseSettings {
    pub username: String,
    #[confik(secret)]
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

// implementation for functions to return database connection options
impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.database_name);
        options
            .clone()
            .log_statements(tracing_log::log::LevelFilter::Trace);
        options
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.as_str())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

// an enum to hold app environment options, be it local or production
pub enum Environment {
    Local,
    Production,
}

// implementation to return a string with values for either local or production
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

// implementation to convert environment string names into the Environment enum
impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
Use either `local` or `production`.",
                other
            )),
        }
    }
}

// function to read in values from the configuration files
pub fn get_configuration() -> Result<Settings, Error> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.toml", environment.as_str());
    let settings = Settings::builder()
        .override_with(FileSource::new(configuration_directory.join("base.toml")).allow_secrets())
        .override_with(
            FileSource::new(configuration_directory.join(environment_filename)).allow_secrets(),
        )
        .override_with(
            EnvSource::new()
                .with_prefix("APP")
                .with_separator("_")
                .with_separator("__"),
        )
        .try_build()?;
    Ok(settings)
}
