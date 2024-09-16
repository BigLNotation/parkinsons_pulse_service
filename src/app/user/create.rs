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
