use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::app::models::QuestionAndAnswer;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmitFormPayload {
    // TODO!: get user id from request header instead once we can do that - don't let people mess with each other's forms
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FindPath {
    pub form_id: ObjectId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmitPath {
    pub form_id: ObjectId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmitPayload {
    pub answers: Vec<QuestionAndAnswer>,
}
