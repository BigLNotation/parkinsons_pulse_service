use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::Database;
use serde_json::json;

use crate::app::models::{dto::user::CreateUserPayload, User};

#[tracing::instrument]
pub async fn create_user(
    State(db): State<Database>,
    Json(payload): Json<CreateUserPayload>,
) -> Response {
    let user = User::from(
        payload.first_name,
        payload.last_name,
        payload.national_health_identifier,
        payload.email_address,
        payload.password,
        payload.is_patient,
    );
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
