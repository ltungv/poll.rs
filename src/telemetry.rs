use opentelemetry::{
    global,
    sdk::{
        self,
        trace::{self, RandomIdGenerator, Sampler},
    },
    trace::TraceError,
};
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, registry::LookupSpan, Registry};

use crate::conf::{Configuration, TracingConfiguration};

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn setup_tracing(config: &Configuration) -> Result<(), anyhow::Error> {
    LogTracer::init().expect("Failed to set tracing logger");
    tracing::subscriber::set_global_default(
        Registry::default()
            .with(config.tracing().env_filter())
            .with(JsonStorageLayer)
            .with(BunyanFormattingLayer::new(
                config.tracing().service_name().to_string(),
                std::io::stdout,
            ))
            .with(otel_jaeger_tracing_layer(config.tracing())?),
    )?;
    Ok(())
}

fn otel_jaeger_tracing_layer<S>(
    config: &TracingConfiguration,
) -> Result<Option<OpenTelemetryLayer<S, sdk::trace::Tracer>>, TraceError>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    if !config.jaeger_enabled() {
        return Ok(None);
    }
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(config.jaeger_endpoint())
        .with_service_name(config.service_name())
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
        .map(|t| tracing_opentelemetry::layer().with_tracer(t))
        .map(Some)
}
