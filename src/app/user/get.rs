use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use crate::app::models::User;

#[tracing::instrument]
pub async fn get_user(Path(id): Path<ObjectId>, State(db): State<Database>) -> Response {
    let document = db
        .collection::<User>("users")
        .find_one(doc! { "_id": id })
        .await;
    match document {
        Ok(document) => (StatusCode::OK, Json::from(document)).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
