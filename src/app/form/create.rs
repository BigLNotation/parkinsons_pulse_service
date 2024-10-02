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

use crate::app::{
    auth::{self, middleware::Auth},
    models::{dto::form::CreateFormPayload, Form, User},
};

#[tracing::instrument]
#[axum::debug_handler]
pub async fn create_form(
    State(db): State<Database>,
    Auth(auth): Auth,
    Json(payload): Json<CreateFormPayload>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to create a form"),
        )
            .into_response();
    };

    let id = ObjectId::new();
    let form = Form::from(id, payload.title, auth.id, auth.id, payload.questions);
    let result = db.collection::<Form>("forms").insert_one(form).await;
    match result {
        Ok(..) => (StatusCode::OK, Json(json! ({ "created_id": id }))).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
