use tracing::info;

#[tokio::main]
async fn main() {
    parkinsons_pulse_service::tracing::init_tracing();

    info!("Hello world!");
}
