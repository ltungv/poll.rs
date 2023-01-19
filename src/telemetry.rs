use opentelemetry::{
    global,
    sdk::trace::{self, RandomIdGenerator, Sampler},
};
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

    let tracing_config = config.tracing();
    Ok(tracing::subscriber::set_global_default(
        Registry::default()
            .with(
                EnvFilter::try_from_env(ENV_LOG_FILTER)
                    .unwrap_or_else(|_| EnvFilter::new(tracing_config.log_level())),
            )
            .with(JsonStorageLayer)
            .with(BunyanFormattingLayer::new(
                tracing_config.service_name().to_string(),
                std::io::stdout,
            ))
            .with(
                tracing_opentelemetry::layer().with_tracer(if tracing_config.jaeger_enabled() {
                    // TODO: get settings from configuration file
                    opentelemetry_jaeger::new_agent_pipeline()
                        .with_endpoint(tracing_config.jaeger_endpoint())
                        .with_service_name(tracing_config.service_name())
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
                        .install_batch(opentelemetry::runtime::Tokio)?
                } else {
                    opentelemetry_jaeger::new_agent_pipeline().install_simple()?
                }),
            ),
    )?)
}
