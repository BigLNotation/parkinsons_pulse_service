use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use mongodb::{bson::doc, Database};

use crate::app::{
    auth::middleware::Auth,
    models::{dto::medication::RemovePath, MedicationTrackerEntry},
};

// TODO!: input validation wrt. string length, etc
#[tracing::instrument]
#[axum::debug_handler]
pub async fn remove_medication(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<RemovePath>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to remove a medication"),
        )
            .into_response();
    };

    let result = db
        .collection::<MedicationTrackerEntry>("medications")
        .delete_one(doc! {"_id": path.medication_id, "user_id": auth.id })
        .await;
    match result {
        Ok(result) => {
            if result.deleted_count == 0 {
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
