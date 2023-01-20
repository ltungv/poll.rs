use opentelemetry::global;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::{conf::Configuration, ENV_LOG_FILTER};

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn setup_tracing(config: &Configuration) -> Result<(), anyhow::Error> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    LogTracer::init().expect("Failed to set tracing logger");

    Ok(tracing::subscriber::set_global_default(
        Registry::default()
            .with(
                EnvFilter::try_from_env(ENV_LOG_FILTER)
                    .unwrap_or_else(|_| EnvFilter::new(config.tracing().log_level())),
            )
            .with(JsonStorageLayer)
            .with(BunyanFormattingLayer::new(
                config.tracing().service_name().to_string(),
                std::io::stdout,
            ))
            .with(tracing_opentelemetry::layer().with_tracer(config.tracing().tracer()?)),
    )?)
}
