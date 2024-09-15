pub mod models;

use std::time::Duration;

use anyhow::Context;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    extract::{FromRef, MatchedPath, Path, State},
    http::Request,
    routing::{get, put},
    Json, Router,
};
use models::{Form, Question, QuestionAndAnswer, User};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{to_document, DateTime};
use mongodb::{bson::doc, Client, Database};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
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
    let app_state = AppState::new().await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "Error occurred while creating app state");
        panic!("Error occurred while creating app state");
    });
    tracing::info!("App state initialized");

    let app = Router::new()
        .route("/get_user/:id", get(get_user))
        .route("/create_user", put(create_user))
        .route("/create_form", put(create_form))
        .route("/push_form_answers", put(push_form_answers))
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
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "Axum failed to serve app");
        panic!("Axum failed to serve app");
    });
    tracing::warn!("Axum stop serving app");
}

#[tracing::instrument]
async fn get_user(Path(id): Path<ObjectId>, State(db): State<Database>) -> Response {
    let document = db
        .collection::<User>("users")
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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CreateUserPayload {
    first_name: String,
    last_name: String,
    national_health_identifer: String,
    email_address: String,
    password: String,
    is_patient: bool,
}

#[tracing::instrument]
async fn create_user(
    State(db): State<Database>,
    Json(payload): Json<CreateUserPayload>,
) -> Response {
    let user = User {
        id: None,
        first_name: payload.first_name,
        last_name: payload.last_name,
        national_health_identifer: payload.national_health_identifer,
        email_address: payload.email_address,
        // TODO!!!!!!: password hashing (not that passwords are used at all currently)
        hashed_password: payload.password,
        is_patient: payload.is_patient,
        caregivers: Vec::new(),
        form_templates: Vec::new(),
    };
    let result = db.collection::<User>("users").insert_one(user).await;
    match result {
        Ok(result) => (
            StatusCode::OK,
            Json(json! ({ "created_id": result.inserted_id })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CreateFormPayload {
    // TODO!: get user id from request header instead once we can do that - don't let people mess with each other's forms
    user_id: ObjectId,
    title: String,
    pub questions: Vec<Question>,
}

#[tracing::instrument]
async fn create_form(
    State(db): State<Database>,
    Json(payload): Json<CreateFormPayload>,
) -> Response {
    let id = ObjectId::new();
    let form = Form {
        id: Some(id),
        title: payload.title,
        created_by: payload.user_id,
        created_at: DateTime::now(),
        questions: payload.questions,
        events: Vec::new(),
    };
    let form_document = match to_document(&form) {
        Ok(doc) => doc,
        Err(e) => {
            tracing::error!(error = %e, "Failed to convert Form to BSON document");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };
    let result = db
        .collection::<User>("users")
        .update_one(
            doc! { "_id": payload.user_id },
            doc! { "$push": { "form_templates": form_document } },
        )
        .await;
    match result {
        Ok(result) => {
            if result.modified_count == 0 {
                // TODO this shouldn't be relevant for this endpoint once user ID comes from auth header instead
                return StatusCode::BAD_REQUEST.into_response();
            }
            (StatusCode::OK, Json(json! ({ "created_id": id }))).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct FormAnswersPayload {
    // TODO!: get user id from request header instead once we can do that - don't let people mess with each other's forms
    user_id: ObjectId,
    form_id: ObjectId,
    answers: Vec<QuestionAndAnswer>,
}

// TODO!: input validation wrt. string length, etc
#[tracing::instrument]
async fn push_form_answers(
    State(db): State<Database>,
    Json(payload): Json<FormAnswersPayload>,
) -> Response {
    let answers_document = match payload
        .answers
        .iter()
        .map(to_document)
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(doc) => doc,
        Err(e) => {
            tracing::error!(error = %e, "Failed to convert form answers to BSON document");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };
    let result = db
        .collection::<User>("users")
        .update_one(
            doc! { "_id": payload.user_id, "form_templates._id": payload.form_id },
            doc! { "$push": { "form_templates.$.events": { "FormSubmitted": {
                "answers": answers_document,
                "submitted_by": payload.user_id,
                "submitted_at": DateTime::now()
            }} } },
        )
        .await;
    match result {
        Ok(result) => {
            if result.modified_count == 0 {
                return StatusCode::BAD_REQUEST.into_response();
            }
            StatusCode::OK.into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
