use opentelemetry::global;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initializes tracing and logging for service
///
/// # Panics
/// This function panics upon failing to create and or init tracing.
///
pub fn init_tracing() {
    if cfg!(enable_tracing) {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("mini-redis")
            .install_simple()
            .expect("Failed to generate Jaeger tracing pipeline");

        let jaeger_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        let stdout_layer = tracing_subscriber::fmt::Layer::default();

        tracing_subscriber::registry()
            .with(jaeger_layer)
            .with(stdout_layer)
            .try_init()
            .expect("Could not init tracing registry");
        
        tracing::info!("Jaeger tracing enabled");
    }
    else {
        let stdout_layer = tracing_subscriber::fmt::Layer::default();

        tracing_subscriber::registry()
            .with(stdout_layer)
            .try_init()
            .expect("Could not init tracing registry");
    }
}
