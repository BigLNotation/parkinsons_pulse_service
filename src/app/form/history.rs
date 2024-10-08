use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::{FutureExt, StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    Database,
};
use serde_json::json;

use crate::app::{
    auth::{self, middleware::Auth},
    models::{
        dto::form::{CreateFormPayload, FindPath},
        Event, Form, FormSubmitted, User,
    },
};

#[tracing::instrument]
#[axum::debug_handler]
pub async fn history(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<FindPath>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to view a form"),
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
        Ok(data) => (
            StatusCode::OK,
            Json(match data.try_collect::<Vec<Form>>().await {
                Err(..) => Vec::new(),
                Ok(forms) => forms
                    .iter()
                    .map(|form| {
                        form.events.iter().filter_map(|event: &Event| match event {
                            Event::FormSubmitted(form) => Some(form.clone()),
                            _ => None,
                        })
                    })
                    .flatten()
                    .collect::<Vec<FormSubmitted>>(),
            }),
        )
            .into_response(),
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
