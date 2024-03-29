use std::{io, path::Path};

use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use tracing_subscriber::EnvFilter;

use crate::{CONFIG_BASE_NAME, CONFIG_DIRECTORY, ENV_PREFIX, ENV_RUN_MODE};

#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Config(#[from] config::ConfigError),
}

pub struct ConfigurationBuilder<'a> {
    env_prefix: &'a str,
    env_run_mode: &'a str,
    config_directory: &'a Path,
    config_base_name: &'a str,
}

impl<'a> Default for ConfigurationBuilder<'a> {
    fn default() -> Self {
        Self {
            env_prefix: ENV_PREFIX,
            env_run_mode: ENV_RUN_MODE,
            config_directory: Path::new(CONFIG_DIRECTORY),
            config_base_name: CONFIG_BASE_NAME,
        }
    }
}

impl<'a> ConfigurationBuilder<'a> {
    pub fn build(self) -> Result<Configuration, ConfigurationError> {
        let environment = RunMode::from_env(self.env_run_mode, RunMode::Local);
        let current_dir = std::env::current_dir()?;
        let config_dir = current_dir.join(self.config_directory);

        let fname_base = resolve_file_name(&config_dir, self.config_base_name)?;
        let fname_env = resolve_file_name(&config_dir, environment.as_str())?;

        let configuration = config::Config::builder()
            // Base configuration
            .add_source(config::File::with_name(&fname_base).required(true))
            // Environment specific configuration
            .add_source(config::File::with_name(&fname_env).required(false))
            // Configuration from environment variables (with a prefix of POLL and '__' as separator)
            // E.g. `POLL__APPLICATION__PORT=5001 would set `configuration.application.port`
            .add_source(
                config::Environment::with_prefix(self.env_prefix)
                    .prefix_separator("__")
                    .separator("__"),
            )
            .build()?;
        Ok(configuration.try_deserialize::<Configuration>()?)
    }

    pub fn config_directory(&mut self, config_directory: &'a Path) -> &mut Self {
        self.config_directory = config_directory;
        self
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Configuration {
    application: ApplicationConfiguration,
    cookie: CookieConfiguration,
    database: DatabaseConfiguration,
    tracing: TracingConfiguration,
}

impl Configuration {
    pub fn application(&self) -> &ApplicationConfiguration {
        &self.application
    }

    pub fn cookie(&self) -> &CookieConfiguration {
        &self.cookie
    }

    pub fn database(&self) -> &DatabaseConfiguration {
        &self.database
    }

    pub fn tracing(&self) -> &TracingConfiguration {
        &self.tracing
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationConfiguration {
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    url: String,
    flash_message_minimum_level: actix_web_flash_messages::Level,
}

impl ApplicationConfiguration {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn flash_message_minimum_level(&self) -> actix_web_flash_messages::Level {
        self.flash_message_minimum_level
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CookieConfiguration {
    signing_key: Secret<String>,
    session_cookie_name: String,
    flash_message_cookie_name: String,
}

impl CookieConfiguration {
    pub fn signing_key(&self) -> &Secret<String> {
        &self.signing_key
    }

    pub fn session_cookie_name(&self) -> &str {
        &self.session_cookie_name
    }

    pub fn flash_message_cookie_name(&self) -> &str {
        &self.flash_message_cookie_name
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseConfiguration {
    require_ssl: bool,
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    username: String,
    password: Secret<String>,
    database: String,
}

impl DatabaseConfiguration {
    pub fn require_ssl(&self) -> bool {
        self.require_ssl
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        self.password.expose_secret()
    }

    pub fn database(&self) -> &str {
        &self.database
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct TracingConfiguration {
    service_name: String,
    log_level: String,
    jaeger_endpoint: Option<String>,
}

impl TracingConfiguration {
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn env_filter(&self) -> EnvFilter {
        EnvFilter::new(&self.log_level)
    }

    pub fn jaeger_endpoint(&self) -> &Option<String> {
        &self.jaeger_endpoint
    }
}

/// The possible runtime environment for our application.
pub enum RunMode {
    Local,
    Production,
}

impl RunMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RunMode::Local => "local",
            RunMode::Production => "production",
        }
    }

    fn from_env(env: &str, default: RunMode) -> Self {
        std::env::var(env)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            .and_then(RunMode::try_from)
            .unwrap_or(default)
    }
}

impl TryFrom<String> for RunMode {
    type Error = io::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{other} is not a supported environment"),
            )),
        }
    }
}

fn resolve_file_name<P>(base_path: P, name: &str) -> Result<String, std::io::Error>
where
    P: AsRef<Path>,
{
    base_path
        .as_ref()
        .join(name)
        .to_str()
        .map(str::to_string)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid configuration file name",
            )
        })
}
