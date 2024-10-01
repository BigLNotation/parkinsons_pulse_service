use axum::{
    extract::{Path, State},
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
    models::{
        dto::form::{CreateFormPayload, FindPath},
        Form, User,
    },
};

#[tracing::instrument]
#[axum::debug_handler]
pub async fn find(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<FindPath>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to create a form"),
        )
            .into_response();
    };

    let result = db
        .collection::<Form>("forms")
        .find_one(doc! {
          "_id": path.form_id,
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
