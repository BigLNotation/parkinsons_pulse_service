use axum::{routing::get, Router, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use serde_json::{json};

/// Runs app
/// 
/// This is where all the routes are defined and gets bound to ports
/// 
/// # Panics
/// This panics upon failed to bind to port or if axum fails to serve app.
/// 
pub async fn run() {
    let app = Router::new()
        .route("/", get(hello_world));
    tracing::info!("Created app router");

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            tracing::info!("Bound to port successfully");
            listener
        },
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