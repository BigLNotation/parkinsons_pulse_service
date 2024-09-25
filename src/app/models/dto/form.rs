use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::app::models::{Question, QuestionAndAnswer};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateFormPayload {
    // TODO!: get user id from request header instead once we can do that - don't let people mess with each other's forms
    pub user_id: ObjectId,
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PushAnswersPayload {
    // TODO!: get user id from request header instead once we can do that - don't let people mess with each other's forms
    pub user_id: ObjectId,
    pub form_id: ObjectId,
    pub answers: Vec<QuestionAndAnswer>,
}
