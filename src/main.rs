#[tokio::main]
async fn main() {
    parkinsons_pulse_service::metrics::logging::init();
    tracing::info!("Initialised tracing");

    let _ = tokio::join!(
        parkinsons_pulse_service::app::run(),
        parkinsons_pulse_service::metrics::run(),
    );
}
