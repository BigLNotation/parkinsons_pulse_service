use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};

use serde_json::json;

/// Runs app
///
/// This is where all the routes are defined and gets bound to ports
///
/// # Panics
/// This panics upon failed to bind to port or if axum fails to serve app.
///
pub async fn run() {
    let app = Router::new().route("/", get(hello_world));
    tracing::info!("Created app router");

    let api_addr = crate::config::get_api_addr();
    tracing::info!(%api_addr, "Binding to address");

    let listener = match tokio::net::TcpListener::bind(api_addr).await {
        Ok(listener) => {
            tracing::info!("Bound to port successfully");
            listener
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to bind to tcp listener");
            panic!("Failed to bind to tcp listener");
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => tracing::warn!("Axum stop serving app"),
        Err(e) => {
            tracing::error!(error = %e, "Axum failed to serve app");
            panic!("Axum failed to serve app");
        }
    };
}

#[tracing::instrument]
async fn hello_world() -> impl IntoResponse {
    tracing::info!("Hello world!");

    (StatusCode::OK, Json(json!({"message": "Hello World!"})))
}
