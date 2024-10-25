pub mod auth;
pub mod caregiver;
pub mod form;
pub mod models;

use axum::extract::{Path, State};
use axum::http::Method;
use axum_extra::headers::Origin;
use dotenvy::dotenv;
use models::{CaregiverToken, User};
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use std::time::Duration;

use anyhow::Context;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, ORIGIN};
use axum::{
    extract::{FromRef, MatchedPath},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use mongodb::{bson::doc, Client, Database};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::config;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    /// # Errors
    ///
    /// Will return 'Err' if the app cannot connect to the database
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = config::get_database_url();
        let client = Client::with_uri_str(database_url.clone()).await?;
        let db = client.database("capstone");
        // Ping DB to make sure we can connect
        db.run_command(doc! { "ping": 1 })
            .await
            .inspect_err(
                |e| tracing::error!(error = %e, "Failed to connect to database at {database_url}"),
            )
            .with_context(|| format!("Failed to connect to database at {database_url}"))?;
        create_unique_email_address_index(&db)
            .await
            .inspect_err(
                |e| tracing::error!(error = %e, "Failed to create unique index on caregiver tokens"),
            )
            .with_context(|| String::from("Failed to create unique index on caregiver token"))?;
        create_unique_caregiver_token_index(&db)
            .await
            .inspect_err(
                |e| tracing::error!(error = %e, "Failed to create unique index on caregiver token"),
            )
            .with_context(|| String::from("Failed to create unique index on caregiver token"))?;
        tracing::info!("Connected to database at {database_url}");
        Ok(AppState { db })
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.db.clone()
    }
}

/// Runs app
///
/// This is where all the routes are defined and gets bound to ports
///
/// # Panics
/// This panics upon failed to bind to port or if axum fails to serve app.
///
pub async fn run() {
    dotenv().ok();

    let app_state = AppState::new().await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "Error occurred while creating app state");
        panic!("Error occurred while creating app state");
    });

    tracing::info!("App state initialized");

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/form", form::router())
        .nest("/auth", auth::router())
        .nest("/caregiver", caregiver::router())
        .with_state(app_state)
        .layer(CookieManagerLayer::new())
        .layer(
          CorsLayer::new()
            .allow_origin(
              config::get_origin_domain(),
            )
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::DELETE, Method::PATCH])
            .allow_headers([CONTENT_TYPE, AUTHORIZATION])
            .allow_credentials(true),
        )
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
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "Axum failed to serve app");
        panic!("Axum failed to serve app");
    });
    tracing::warn!("Axum stop serving app");
}

#[tracing::instrument]
async fn hello_world() -> impl IntoResponse {
    tracing::info!("Hello world!");

    (StatusCode::OK, Json(json!({"message": "Hello World!"})))
}

#[tracing::instrument]
fn foo() {
    // TODO: Remove example after first endpoint made
    tracing::warn!("foo!");
}

#[derive(Serialize, Deserialize, Debug)]
struct ExampleDocument {
    #[serde(rename = "_id")]
    id: u32, // For actual documents we'll use ObjectId, this is just to make manual testing of this example simpler
    string: String,
    number: i32,
}

#[tracing::instrument]
async fn read_example(Path(id): Path<u32>, State(db): State<Database>) -> Response {
    // TODO: Remove example after first endpoint made
    let document = db
        .collection::<ExampleDocument>("testCollection")
        .find_one(doc! { "_id": id })
        .await;
    match document {
        Ok(document) => (StatusCode::OK, Json::from(document)).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[tracing::instrument]
async fn write_example(
    State(db): State<Database>,
    Json(document): Json<ExampleDocument>,
) -> Response {
    // TODO: Remove example after first endpoint made
    // For this example we don't distinguish adding a new document and overwriting an existing one, for simplicity
    let result = db
        .collection::<ExampleDocument>("testCollection")
        .update_one(
            doc! { "_id": document.id },
            doc! {"$set": doc!{"string": document.string, "number": document.number}},
        )
        .upsert(true)
        .await;
    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn create_unique_email_address_index(db: &Database) -> anyhow::Result<()> {
    let collection = db.collection::<User>("users");
    let index_model = IndexModel::builder()
        .keys(doc! { "email_address": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    collection.create_index(index_model).await?;
    Ok(())
}

async fn create_unique_caregiver_token_index(db: &Database) -> anyhow::Result<()> {
    let collection = db.collection::<CaregiverToken>("caregiver_tokens");
    let index_model = IndexModel::builder()
        .keys(doc! { "token": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    collection.create_index(index_model).await?;
    Ok(())
}
