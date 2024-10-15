use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::{
    bson::{doc, to_document, DateTime},
    Database,
};

use crate::app::{
    auth::middleware::Auth,
    models::{
        dto::form::{SubmitPath, SubmitPayload},
        Form,
    },
};

// TODO!: input validation wrt. string length, etc
#[tracing::instrument]
#[axum::debug_handler]
pub async fn submit(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<SubmitPath>,
    Json(payload): Json<SubmitPayload>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to add answers to the form"),
        )
            .into_response();
    };

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
        .collection::<Form>("forms")
        .update_one(
            doc! { "user_id": auth.id, "_id": path.form_id },
            doc! { "$push": { "events": { "FormSubmitted": {
                "answers": answers_document,
                "submitted_by": auth.id,
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
