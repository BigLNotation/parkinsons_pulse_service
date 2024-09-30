pub mod dto;

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    // national_health_identifer: String,
    pub email_address: String,
    pub hashed_password: String,
    pub is_patient: bool,
    pub caregivers: Vec<ObjectId>,
    pub forms: Vec<Form>,
}

impl User {
    #[must_use]
    pub fn from(
        first_name: String,
        last_name: String,
        email_address: String,
        hashed_password: String,
        is_patient: bool,
    ) -> User {
        User {
            id: None,
            first_name,
            last_name,
            email_address,
            hashed_password,
            is_patient,
            caregivers: vec![],
            forms: vec![],
        }
    }
}

/// A form that clients fill in is represented here
///
///
/// # Examples
/// ```
/// use parkinsons_pulse_service::app::models::*;
/// use mongodb::bson::{oid::ObjectId, DateTime};
///
/// Form {
///     id: Some(ObjectId::new()),
///     title: String::from("Tremors"),
///     created_by: ObjectId::new(),
///     created_at: DateTime::now(),
///     responses: vec![
///         QuestionAndAnswer::Multichoice(MultichoiceQuestion {
///             title: String::from("How many times have you experienced this in the last week?"),
///             options: vec![MultichoiceQuestionOption {
///                 name: String::from("Once"),
///             }],
///             min_selected: 1,
///             max_selected: 2,
///         }, String::from("Once")),
///         QuestionAndAnswer::FreeForm(FreeFormQuestion {
///             title: String::from("Is there anything else you would like to add?"),
///             max_length: 200,
///             min_length: 0,
///         }, String::from("Example answer")),
///     ],
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Form {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// Symptom or name of form
    pub title: String,
    pub created_by: ObjectId,
    pub created_at: DateTime,
    /// List of questions in the form
    pub responses: Vec<QuestionAndAnswer>,
}

impl Form {
    #[must_use]
    pub fn from(
        id: ObjectId,
        title: String,
        created_by: ObjectId,
        responses: Vec<QuestionAndAnswer>,
    ) -> Self {
        Self {
            id: Some(id),
            title,
            created_by,
            created_at: DateTime::now(),
            responses,
        }
    }
}

/// This represents a form question for clients to answer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Question {
    /// This is a list of multiple choices for a question
    Multichoice(MultichoiceQuestion),
    /// This is a numeric slider
    Slider(SliderQuestion),
    /// This is for free form questions where the client may type whatever
    FreeForm(FreeFormQuestion),
}

/// Name of choice in the questions that is selected
pub type MultichoiceAnswer = String;
/// Numerical value that the user selects
pub type SliderAnswer = f64;
/// String for the answer that the client types
pub type FreeFormAnswer = String;

/// Combination of both the question and answer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QuestionAndAnswer {
    Multichoice(MultichoiceQuestion, MultichoiceAnswer),
    Slider(SliderQuestion, SliderAnswer),
    FreeForm(FreeFormQuestion, FreeFormAnswer),
}

/// Free form question with some validation rules you could apply
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FreeFormQuestion {
    pub title: String,
    pub max_length: u64,
    pub min_length: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SliderQuestion {
    pub title: String,
    pub units: Option<String>,
    pub low: f64,
    pub high: f64,
    pub step: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestion {
    pub title: String,
    pub options: Vec<MultichoiceQuestionOption>,
    pub min_selected: u64,
    pub max_selected: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestionOption {
    pub name: String,
}
