#[tokio::main]
async fn main() {
    parkinsons_pulse_service::tracing::init();
    tracing::info!("Initialised tracing");

    parkinsons_pulse_service::app::run().await;
}
