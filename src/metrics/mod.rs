use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use tokio::net::TcpListener;

use crate::config;

pub mod logging;

/// runs end point for metrics
///
/// # Panics
/// Panics if fails to serve endpoint
pub async fn run() {
    let metrics_app = Router::new().route("/health", get(check_health));

    let metrics_addr = config::get_metrics_addr();
    tracing::info!(%metrics_addr, "Binding metrics to address");

    let metrics_listener = TcpListener::bind(metrics_addr).await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "Failed to bind to tcp listener");
        panic!("Failed to bind to tcp listener");
    });
    tracing::info!("Bound to address successfully");

    tracing::info!("Serving metrics");
    axum::serve(metrics_listener, metrics_app).await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "Axum failed to serve metrics");
        panic!("Axum failed to serve metrics");
    });
    tracing::warn!("Axum stop serving metrics");
}

async fn check_health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "Healthy"})))
}
