use opentelemetry::{global, trace::TracerProvider, Key, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    metrics::{
        reader::DefaultTemporalitySelector, Aggregation, Instrument, MeterProviderBuilder,
        PeriodicReader, SdkMeterProvider, Stream,
    },
    runtime,
    trace::{BatchConfig, Config, RandomIdGenerator, Sampler, Tracer},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{SERVICE_NAME},
};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::config::get_optl_collecter_address;

/// Initializes tracing and logging for service
///
/// # Features
/// `local_log`: local fmt logging, enabled by default \
/// `jaeger_tracing`: This feature flag enabled jaeger tracing
///
/// # Panics
/// This function panics upon failing to create and or init tracing.
///
pub fn init() {
    let mut layers = Vec::new();
    let mut opentelemetry_layers = Vec::new();
    #[cfg(feature = "jaeger_tracing")]
    {
        let optl_tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                Config::default().with_resource(Resource::new(vec![KeyValue::new(
                    SERVICE_NAME,
                    "parkinsons_pulse_service",
                )])),
            )
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(
                        get_optl_collecter_address()
                    ),
            )
            .with_batch_config(BatchConfig::default())
            .install_batch(runtime::Tokio)
            .expect("Failed to initialize tracer provider.");

        global::set_tracer_provider(optl_tracer.clone());
        let opentelemetry_layer = OpenTelemetryLayer::new(optl_tracer.tracer("otel-subscriber"));
        opentelemetry_layers.push(opentelemetry_layer);
    }

    #[cfg(feature = "local_log")]
    {
        let stdout_layer = tracing_subscriber::fmt::Layer::default().boxed();

        layers.push(stdout_layer);
    }

    #[cfg(feature = "journal_log")]
    if let Ok(journal_layer) = tracing_journald::Layer::new() {
        layers.push(journal_layer.boxed());
    }

    tracing_subscriber::registry()
        .with(layers)
        .with(opentelemetry_layers)
        .try_init()
        .expect("Could not init tracing registry");

    #[cfg(feature = "jaeger_tracing")]
    tracing::info!("Jaeger tracing enabled");

    #[cfg(feature = "local_log")]
    tracing::info!("Local logging fmt enabled");
}
