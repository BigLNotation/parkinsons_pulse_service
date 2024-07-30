use std::sync::Arc;
use std::time::Duration;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::serve::Serve;
use axum::{extract::MatchedPath, http::Request, routing::get, Json, Router};
use mongodb::{Client, Database};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;

use crate::config;

#[derive(Clone)]
pub struct State {
    pub db: Arc<Database>,
}

impl State {
    /// # Errors
    ///
    /// Will return 'Err' if there is no `DATABASE_URL` environment variable or the app
    /// cannot connect to the database
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = config::get_database_url();
        let client = Client::with_uri_str(database_url).await?;
        let db = client.database("capstone");
        Ok(State { db: Arc::new(db) })
    }
}

/// Runs app
///
/// This is where all the routes are defined and gets bound to ports
///
/// # Panics
/// This panics upon failed to bind to port or if axum fails to serve app.
///
pub async fn run() -> Serve<Router, Router> {
    let app_state = State::new().await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "Failed to connect to database");
        panic!("Failed to connect to database");
    });
    tracing::info!("Connected to database");

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
            .on_response(|response: &Response, latency: Duration, _span: &tracing::Span| {
                    tracing::debug!(response = ?response.headers(), body = ?response.body(), latency = ?latency);
            })
            .on_failure(|error: ServerErrorsFailureClass, latency: Duration, _span: &tracing::Span| {
                    tracing::error!(error = ?error, latency = ?latency, "Request returned a failure");
            })
    );
    tracing::info!("Created app router");

    let api_addr = config::get_api_addr();
    tracing::info!(%api_addr, "Binding app to address");

    let listener = TcpListener::bind(api_addr).await.unwrap_or_else(|e| {
        // TODO!: add tracing to panic
        tracing::error!(error = %e, "Failed to bind to tcp listener");
        panic!("Failed to bind to tcp listener");
    });
    tracing::info!("Bound to address successfully");

    tracing::info!("Serving app");
    axum::serve(listener, app)
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
