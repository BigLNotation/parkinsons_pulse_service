use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde_json::json;

use crate::app::{
    auth::middleware::Auth,
    models::{dto::medication::AddMedicationPayload, MedicationTrackerEntry},
};

// TODO!: input validation wrt. string length, etc
#[tracing::instrument]
#[axum::debug_handler]
pub async fn add_medication(
    State(db): State<Database>,
    Auth(auth): Auth,
    Json(payload): Json<AddMedicationPayload>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to add a medication"),
        )
            .into_response();
    };

    let id = ObjectId::new();
    let medication = MedicationTrackerEntry::from(
        id,
        auth.id,
        payload.medication_name,
        payload.dose,
        payload.timing,
    );
    let result = db
        .collection::<MedicationTrackerEntry>("medications")
        .insert_one(medication)
        .await;
    match result {
        Ok(..) => (StatusCode::OK, Json(json! ({ "created_id": id }))).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
