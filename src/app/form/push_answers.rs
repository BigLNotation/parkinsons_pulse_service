use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::{
    bson::{doc, to_document, DateTime},
    Database,
};

use crate::app::models::{dto::form::FormAnswersPayload, User};

// TODO!: input validation wrt. string length, etc
#[tracing::instrument]
pub async fn push_form_answers(
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
