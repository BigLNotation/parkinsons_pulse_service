#[cfg(feature = "jaeger_tracing")]
use opentelemetry::global;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

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

    #[cfg(feature = "jaeger_tracing")]
    {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let jaeger_tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name(env!("CARGO_PKG_NAME"))
            .install_simple()
            .expect("Failed to generate Jaeger tracing pipeline");

        let jaeger_layer = tracing_opentelemetry::layer()
            .with_tracer(jaeger_tracer)
            .boxed();

        layers.push(jaeger_layer);
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
        .try_init()
        .expect("Could not init tracing registry");

    #[cfg(feature = "jaeger_tracing")]
    tracing::info!("Jaeger tracing enabled");

    #[cfg(feature = "local_log")]
    tracing::info!("Local logging fmt enabled");
}