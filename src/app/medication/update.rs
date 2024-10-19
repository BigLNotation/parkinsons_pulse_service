use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::{bson::doc, Database};

use crate::app::{
    auth::middleware::Auth,
    models::{
        dto::medication::{AddMedicationPayload, UpdatePath},
        MedicationTrackerEntry,
    },
};

// TODO!: input validation wrt. string length, etc
#[tracing::instrument]
#[axum::debug_handler]
pub async fn update_medication(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<UpdatePath>,
    Json(payload): Json<AddMedicationPayload>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to update a medication"),
        )
            .into_response();
    };

    let result = db.collection::<MedicationTrackerEntry>("medications")
		.update_one(doc! {"_id": path.medication_id, "user_id": auth.id },
					doc! { "medication_name": payload.medication_name, "dose": payload.dose, "timing": payload.timing } )
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
