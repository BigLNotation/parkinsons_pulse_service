use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::StreamExt;
use mongodb::{
    bson::doc,
    Database,
};

use crate::app::{
    auth::{middleware::Auth},
    models::{
        dto::form::FindPath,
        Form,
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

#[tracing::instrument]
#[axum::debug_handler]
pub async fn find_all(State(db): State<Database>, Auth(auth): Auth) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to create a form"),
        )
            .into_response();
    };

    let result = db
        .collection::<Form>("forms")
        .find(doc! {
          "user_id": auth.id
        })
        .await;

    match result {
        Ok(mut data) => {
            let mut forms: Vec<Form> = Vec::new();
            while let Some(Ok(form)) = data.next().await {
                forms.push(form);
            }
            (StatusCode::OK, Json(forms)).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
