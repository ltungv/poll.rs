use opentelemetry::{trace::TraceError, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self, RandomIdGenerator, Sampler},
    Resource,
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
) -> Result<Option<OpenTelemetryLayer<S, opentelemetry_sdk::trace::Tracer>>, TraceError>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let jaeger_endpoint = match config.jaeger_endpoint() {
        None => return Ok(None),
        Some(v) => v,
    };

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(jaeger_endpoint);

    let trace_config = trace::config()
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            config.service_name().to_string(),
        )]));

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(trace_config)
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .map(|t| tracing_opentelemetry::layer().with_tracer(t))
        .map(Some)
}
