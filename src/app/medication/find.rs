use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::StreamExt;
use mongodb::{bson::doc, Database};

use crate::app::{
    auth::middleware::Auth,
    models::{dto::medication::FindPath, MedicationTrackerEntry},
};

#[tracing::instrument]
#[axum::debug_handler]
pub async fn find_medication(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<FindPath>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to access medication tracker entries"),
        )
            .into_response();
    };

    let result = db
        .collection::<MedicationTrackerEntry>("medications")
        .find_one(doc! {
          "_id": path.medication_id,
          "user_id": auth.id
        })
        .await;
    match result {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn find_all_medications(State(db): State<Database>, Auth(auth): Auth) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to access medication tracker entries"),
        )
            .into_response();
    };

    let result = db
        .collection::<MedicationTrackerEntry>("medications")
        .find(doc! {
          "user_id": auth.id
        })
        .await;

    match result {
        Ok(mut data) => {
            let mut medications: Vec<MedicationTrackerEntry> = Vec::new();
            while let Some(Ok(medication)) = data.next().await {
                medications.push(medication);
            }
            (StatusCode::OK, Json(medications)).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
