use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    Database,
};
use serde_json::json;

use crate::app::models::{dto::form::SubmitFormPayload, Form, User};

#[tracing::instrument]
pub async fn submit_form(
    State(db): State<Database>,
    Json(payload): Json<SubmitFormPayload>,
) -> Response {
    let id = ObjectId::new();
    let form = Form::from(id, payload.title, payload.user_id, payload.responses);
    let form_document = match to_document(&form) {
        Ok(doc) => doc,
        Err(e) => {
            tracing::error!(error = %e, "Failed to convert Form to BSON document");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };
    let result = db
        .collection::<User>("users")
        .update_one(
            doc! { "_id": payload.user_id },
            doc! { "$push": { "forms": form_document } },
        )
        .await;
    match result {
        Ok(result) => {
            if result.modified_count == 0 {
                // TODO this shouldn't be relevant for this endpoint once user ID comes from auth header instead
                return StatusCode::BAD_REQUEST.into_response();
            }
            (StatusCode::OK, Json(json! ({ "created_id": id }))).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
