use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::app::models::QuestionAndAnswer;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmitFormPayload {
    // TODO!: get user id from request header instead once we can do that - don't let people mess with each other's forms
    pub user_id: ObjectId,
    pub title: String,
    pub responses: Vec<QuestionAndAnswer>,
}
