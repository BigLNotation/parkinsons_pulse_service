use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    first_name: String,
    last_name: String,
    email_address: String,
    hashed_password: String,
    is_patient: bool,
    caregivers: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormTemplate {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    questions: Vec<QuestionTemplate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QuestionTemplate {
    Multichoice(MultichoiceQuestion),
    Slider(SliderQuestion),
    FreeForm(FreeFormQuestion),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FreeFormQuestion {
    title: String,
    max_length: u64,
    min_length: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SliderQuestion {
    title: String,
    units: Option<String>,
    low: i64,
    high: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MultichoiceQuestion {
    title: String,
    options: Vec<String>,
}
