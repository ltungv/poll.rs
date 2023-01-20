use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, Registry};

use crate::conf::Configuration;

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn setup_tracing(config: &Configuration) -> Result<(), anyhow::Error> {
    LogTracer::init().expect("Failed to set tracing logger");
    let registry = Registry::default()
        .with(config.tracing().env_filter())
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(
            config.tracing().service_name().to_string(),
            std::io::stdout,
        ));
    match config.tracing().jaeger_tracer()? {
        Some(tracer) => {
            let registry = registry.with(tracing_opentelemetry::layer().with_tracer(tracer));
            tracing::subscriber::set_global_default(registry)?;
        }
        None => tracing::subscriber::set_global_default(registry)?,
    }
    Ok(())
}
