use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bson::DateTime;
use chrono::Utc;
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    Collection, Cursor, Database,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::app::{
    auth::{self, middleware::Auth},
    models::{
        dto::{caregiver::CaregiverTokenPath, form::CreateFormPayload},
        CaregiverToken, Form, User,
    },
};

#[derive(Serialize, Deserialize)]
struct CaregiverInfo {
    id: ObjectId,
    first_name: String,
    last_name: String,
    email_address: String,
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn find_caregiver(State(db): State<Database>, Auth(auth): Auth) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to find caregivers"),
        )
            .into_response();
    };

    // Perform the aggregation using $lookup
    let pipeline = vec![
        doc! {
          "$match": {
            "_id": auth.id
          }
        },
        doc! {
            "$lookup": {
                "from": "users",  // The caregivers collection
                "localField": "caregivers",  // The field in users that holds caregiver IDs
                "foreignField": "_id",  // The field in caregivers that matches the user_id
                "as": "caregivers_info"  // The result will be stored in this field
            }
        },
        doc! {
            "$project": {
                "caregivers_info._id": 1,
                "caregivers_info.first_name": 1,  // Only retrieve caregiver ID and name
                "caregivers_info.last_name": 1,  // Only retrieve caregiver ID and name
                "caregivers_info.email_address": 1,  // Only retrieve caregiver ID and name
            }
        },
    ];

    let Ok(mut cursor): Result<Cursor<bson::document::Document>, mongodb::error::Error> =
        db.collection::<User>("users").aggregate(pipeline).await
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("could not find caregivers"),
        )
            .into_response();
    };

    let Some(Ok(caregiver)) = cursor.next().await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("could not find caregiver list"),
        )
            .into_response();
    };

    let Some(caregiver_info) = caregiver.get("caregivers_info") else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("cannot extract caregiver information"),
        )
            .into_response();
    };
    (StatusCode::OK, Json(caregiver_info)).into_response()
}
