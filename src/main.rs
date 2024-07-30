#[tokio::main]
async fn main() {
    parkinsons_pulse_service::metrics::logging::init();
    tracing::info!("Initialised tracing");

    let (app_server, metrics_server) = tokio::join!(
        parkinsons_pulse_service::app::run(),
        parkinsons_pulse_service::metrics::run(),
    );

    app_server.await.expect("App endpoint stopped being served");
    metrics_server
        .await
        .expect("Metrics endpoint stopped being served");
}
