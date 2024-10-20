use std::borrow::BorrowMut;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    Cursor, Database,
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

async fn get_users_forms(
    user_id: ObjectId,
    db: &Database,
) -> Result<Vec<Form>, mongodb::error::Error> {
    let result = db
        .collection::<Form>("forms")
        .find(doc! {
          "user_id": user_id
        })
        .await;

    match result {
        Ok(mut data) => {
            let mut forms: Vec<Form> = Vec::new();
            while let Some(Ok(form)) = data.next().await {
                forms.push(form);
            }
            Ok(forms)
        }
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            Err(e)
        }
    }
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn find_all(State(db): State<Database>, Auth(auth): Auth) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to find forms"),
        )
            .into_response();
    };

    let Ok(mut patients): Result<Cursor<User>, mongodb::error::Error> = db
        .clone()
        .collection("users")
        .find(doc! {
          "caregivers": auth.id
        })
        .await
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("could not query database"),
        )
            .into_response();
    };

    let mut forms: Vec<Form> = Vec::new();

    let Ok(own_forms) = get_users_forms(auth.id, &db).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("could not find user forms"),
        )
            .into_response();
    };

    forms.extend(own_forms);

    while let Some(Ok(patient)) = &patients.borrow_mut().next().await {
        let Some(patient_id) = patient.id else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("patient has no id"),
            )
                .into_response();
        };
        let Ok(patient_forms) = get_users_forms(patient_id, &db).await else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("could not find patients forms"),
            )
                .into_response();
        };
        forms.extend(patient_forms);
    }

    (StatusCode::OK, Json(forms)).into_response()
}
