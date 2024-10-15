use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::{
    bson::doc,
    Database,
};
use serde_json::json;

use crate::app::{
    auth::{middleware::Auth},
    models::CaregiverToken,
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
