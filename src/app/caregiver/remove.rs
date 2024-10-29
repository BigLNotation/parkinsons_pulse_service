use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bson::DateTime;
use chrono::Utc;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    Collection, Database,
};
use serde_json::json;
use tracing::info;

use crate::app::{
    auth::{self, middleware::Auth},
    models::{
        dto::{
            caregiver::{CaregiverTokenPath, RemoveCaregiverPath},
            form::CreateFormPayload,
        },
        CaregiverToken, Form, User,
    },
};

#[tracing::instrument]
#[axum::debug_handler]
pub async fn remove_caregiver(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<RemoveCaregiverPath>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to create a form"),
        )
            .into_response();
    };

    

    match db
        .collection::<User>("users")
        .find_one_and_update(
            doc! {
              "_id": auth.id
            },
            doc! {
              "$pull": {
                "caregivers": path.caregiver_id
              }
            },
        )
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(err) => {
            tracing::error!("{:#?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not remove caregiver"),
            )
        }
        .into_response(),
    }
}
