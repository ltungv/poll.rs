use std::{
    io,
    path::{Path, PathBuf},
};

use opentelemetry::{
    sdk::{
        self,
        trace::{self, RandomIdGenerator, Sampler},
    },
    trace::TraceError,
};
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use tracing_subscriber::EnvFilter;

use crate::{CONFIG_BASE_NAME, CONFIG_DIRECTORY, ENV_LOG_FILTER, ENV_PREFIX, ENV_RUN_MODE};

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
    config_directory: &'a str,
    config_base_name: &'a str,
}

impl<'a> Default for ConfigurationBuilder<'a> {
    fn default() -> Self {
        Self {
            env_prefix: ENV_PREFIX,
            env_run_mode: ENV_RUN_MODE,
            config_directory: CONFIG_DIRECTORY,
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
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Configuration {
    application: ApplicationConfiguration,
    database: DatabaseConfiguration,
    tracing: TracingConfiguration,
}

impl Configuration {
    pub fn application(&self) -> &ApplicationConfiguration {
        &self.application
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
    template_directory: PathBuf,
    template_file_extension: String,
}

impl ApplicationConfiguration {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn template_directory(&self) -> &Path {
        &self.template_directory
    }

    pub fn template_file_extension(&self) -> &str {
        &self.template_file_extension
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
    database: Option<String>,
}

impl DatabaseConfiguration {
    pub fn pg_pool(&self) -> PgPool {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        let connection_options = {
            let opts = PgConnectOptions::new()
                .host(&self.host)
                .username(&self.username)
                .password(self.password.expose_secret())
                .port(self.port)
                .ssl_mode(ssl_mode);

            match &self.database {
                Some(database) => opts.database(database),
                None => opts,
            }
        };

        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(connection_options)
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct TracingConfiguration {
    service_name: String,
    log_level: String,
    jaeger_enabled: bool,
    jaeger_endpoint: String,
}

impl TracingConfiguration {
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn env_filter(&self) -> EnvFilter {
        EnvFilter::try_from_env(ENV_LOG_FILTER).unwrap_or_else(|_| EnvFilter::new(&self.log_level))
    }

    pub fn jaeger_tracer(&self) -> Result<Option<sdk::trace::Tracer>, TraceError> {
        if !self.jaeger_enabled {
            return Ok(None);
        }
        opentelemetry_jaeger::new_agent_pipeline()
            .with_endpoint(&self.jaeger_endpoint)
            .with_service_name(&self.service_name)
            .with_max_packet_size(16_384)
            .with_auto_split_batch(true)
            .with_instrumentation_library_tags(false)
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_id_generator(RandomIdGenerator::default())
                    .with_max_events_per_span(64)
                    .with_max_attributes_per_span(16),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .map(Some)
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
                format!("{} is not a supported environment", other),
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
