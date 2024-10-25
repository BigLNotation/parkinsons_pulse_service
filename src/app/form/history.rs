use std::cmp::Ordering;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::app::{
    auth::middleware::Auth,
    models::{Event, Form, Question, QuestionAndAnswer},
};
#[derive(Serialize, Deserialize)]
struct FormSubmittedWithForm {
    pub id: Option<ObjectId>,
    pub user_id: Option<ObjectId>,
    pub title: String,
    pub created_by: ObjectId,
    pub created_at: DateTime,
    pub questions: Vec<Question>,
    pub answers: Vec<QuestionAndAnswer>,
    pub submitted_at: DateTime,
    pub submitted_by: ObjectId,
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn history(State(db): State<Database>, Auth(auth): Auth) -> Response {
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
        Ok(data) => {
            let mut res = match data.try_collect::<Vec<Form>>().await {
                Err(..) => Vec::new(),
                Ok(forms) => forms
                    .iter()
                    .map(|form| {
                        form.events.iter().filter_map(|event: &Event| match event {
                            Event::FormSubmitted(form_submitted) => Some(FormSubmittedWithForm {
                                id: form.id,
                                user_id: form.user_id,
                                title: form.title.clone(),
                                created_by: form.created_by,
                                created_at: form.created_at,
                                questions: form.questions.clone(),
                                answers: form_submitted.answers.clone(),
                                submitted_at: form_submitted.submitted_at,
                                submitted_by: form_submitted.submitted_by,
                            }),
                            _ => None,
                        })
                    })
                    .flatten()
                    .collect::<Vec<FormSubmittedWithForm>>(),
            };
            res.sort_by(|a, b| {
                if a.submitted_at.to_chrono().timestamp() > b.submitted_at.to_chrono().timestamp() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            (StatusCode::OK, Json(res)).into_response()
        }

        Err(e) => {
            tracing::error!(error = %e, "Error occurred while querying database");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
