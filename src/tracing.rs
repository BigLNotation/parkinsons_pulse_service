use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initializes tracing and logging for service
///
/// # Panics
/// This function panics upon failing to create and or init tracing.
///
pub fn init_tracing() {
    // TODO!: Finish OpenTelemetry tracing so a collector can get it
    let stdout_layer = tracing_subscriber::fmt::Layer::default();

    tracing_subscriber::registry()
        .with(stdout_layer)
        .try_init()
        .expect("Could not init tracing registry");
}
