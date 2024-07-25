pub mod state;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::MatchedPath, http::Request, routing::get, Json, Router};
use serde_json::json;
use state::AppState;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;

/// Runs app
///
/// This is where all the routes are defined and gets bound to ports
///
/// # Panics
/// This panics upon failed to bind to port or if axum fails to serve app.
///
pub async fn run() {
    let app_state = match AppState::new().await {
        Ok(value) => {
            tracing::info!("Connected to database");
            value
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to connect to database");
            panic!("Failed to connect to database");
        }
    };
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/fail", get(failure))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::debug_span!("http_request",
                    method = ?request.method(),
                    matched_path,
                )
            })
            .on_request(|request: &Request<_>, _span: &tracing::Span| {
                tracing::debug!(header = ?request.headers(), body = ?request.body());
            })
            .on_response(
                |response: &Response, latency: Duration, _span: &tracing::Span| {
                        tracing::debug!(response = ?response.headers(), body = ?response.body(), latency = ?latency);
                },
            )
            .on_failure(
                |error: ServerErrorsFailureClass, latency: Duration, _span: &tracing::Span| {
                    tracing::error!(error = ?error, latency = ?latency, "Request returned a failure");
                },
            ),
    );
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
        Ok(()) => tracing::warn!("Axum stop serving app"),
        Err(e) => {
            tracing::error!(error = %e, "Axum failed to serve app");
            panic!("Axum failed to serve app");
        }
    };
}

#[tracing::instrument]
async fn failure() -> impl IntoResponse {
    // TODO: Remove example after first endpoint made
    tracing::error!("I failed :(");

    StatusCode::INTERNAL_SERVER_ERROR
}

#[tracing::instrument]
async fn hello_world() -> impl IntoResponse {
    // TODO: Remove example after first endpoint made
    tracing::info!("Hello world!");
    foo();

    (StatusCode::OK, Json(json!({"message": "Hello World!"})))
}

#[tracing::instrument]
fn foo() {
    // TODO: Remove example after first endpoint made
    tracing::warn!("foo!");
}
