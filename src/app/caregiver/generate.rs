use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId, to_document, Bson},
    Database,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app::{
    auth::{self, middleware::Auth},
    models::{dto::form::CreateFormPayload, CaregiverToken, Form, User},
};

#[tracing::instrument]
#[axum::debug_handler]
pub async fn generate(State(db): State<Database>, Auth(auth): Auth) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to create a form"),
        )
            .into_response();
    };

    let caregiver_token = CaregiverToken::new(auth.id.clone());

    let result = db
        .collection::<CaregiverToken>("caregiver_tokens")
        .insert_one(caregiver_token.clone())
        .await;
    match result {
        Ok(..) => (
            StatusCode::OK,
            Json(json! ({ "created_token": caregiver_token.token })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
