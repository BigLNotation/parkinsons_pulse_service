pub mod dto;

use chrono::{Duration, Utc};
use mongodb::bson::{oid::ObjectId, DateTime};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
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
}

impl User {
    #[must_use]
    pub fn from(
        first_name: String,
        last_name: String,
        email_address: String,
        password: String,
        is_patient: bool,
    ) -> User {
        User {
            id: None,
            first_name,
            last_name,
            email_address,
            hashed_password: password,
            is_patient,
            caregivers: vec![],
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
///     user_id: Some(ObjectId::new()),
///     title: String::from("Tremors"),
///     created_by: ObjectId::new(),
///     created_at: DateTime::now(),
///     questions: vec![
///         Question::Multichoice(MultichoiceQuestion {
///             id: Some(ObjectId::new()),
///             title: String::from("How many times have you experienced this in the last week?"),
///             options: vec![MultichoiceQuestionOption {
///                 name: String::from("Once"),
///                 id: Some(ObjectId::new()),
///             }],
///             min_selected: 1,
///             max_selected: 2,
///         }),
///         Question::FreeForm(FreeFormQuestion {
///             id: Some(ObjectId::new()),
///             title: String::from("Is there anything else you would like to add?"),
///             max_length: 200,
///             min_length: 0,
///         }),
///     ],
///     events: vec![
///         Event::QuestionEdited(QuestionEdited {
///             question_id: ObjectId::new(),
///             former_question: Question::FreeForm(FreeFormQuestion {
///                 id: Some(ObjectId::new()),
///                 title: String::from("How are you feeling this week?"),
///                 max_length: 100,
///                 min_length: 10,
///             }),
///             new_question: Question::FreeForm(FreeFormQuestion {
///                 id: Some(ObjectId::new()),
///                 title: String::from("Is there anything else you would like to add?"),
///                 max_length: 200,
///                 min_length: 0,
///             }),
///             edited_at: DateTime::now(),
///             edited_by: ObjectId::new(),
///         }),
///         Event::FormSubmitted(FormSubmitted {
///             answers: vec![
///                 QuestionAndAnswer::Multichoice(
///                     ObjectId::new(),
///                     ObjectId::new(),
///                 ),
///                 QuestionAndAnswer::FreeForm(
///                     ObjectId::new(),
///                     String::from("I wasn't able to press the elevator buttons this morning"),
///                 ),
///             ],
///             submitted_at: DateTime::now(),
///             submitted_by: ObjectId::new(),
///         }),
///     ],
/// };
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Form {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ObjectId>,
    /// Title of the form for clients
    pub title: String,
    pub created_by: ObjectId,
    pub created_at: DateTime,
    /// List of questions in the form
    pub questions: Vec<Question>,
    /// List of events such as a user filling in a form or a moderator updating the form
    pub events: Vec<Event>,
}

impl Form {
    #[must_use]
    pub fn from(
        id: ObjectId,
        title: String,
        created_by: ObjectId,
        user_id: ObjectId,
        mut questions: Vec<Question>,
    ) -> Self {
        for question in &mut questions {
            match question {
                Question::Multichoice(ref mut question) => {
                    question.id = Some(ObjectId::new());
                    for option in &mut question.options {
                        option.id = Some(ObjectId::new());
                    }
                }

                Question::Slider(ref mut question) => {
                    question.id = Some(ObjectId::new());
                }
                Question::FreeForm(ref mut question) => {
                    question.id = Some(ObjectId::new());
                }
            }
        }
        Self {
            id: Some(id),
            title,
            created_by,
            user_id: Some(user_id),
            created_at: DateTime::now(),
            questions,
            events: Vec::new(),
        }
    }
}

/// This represents a form event, either filling in the form and submitting it, or changing a question
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    FormSubmitted(FormSubmitted),
    QuestionEdited(QuestionEdited),
}

/// This represents how a question may change
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuestionEdited {
    /// This is the ID of the question in the form being changed
    pub question_id: ObjectId,
    pub former_question: Question,
    pub new_question: Question,
    pub edited_by: ObjectId,
    pub edited_at: DateTime,
}

/// This is how we represent a form being filled
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormSubmitted {
    /// This is a list of all of the questions and the answers that were selected or entered
    pub answers: Vec<QuestionAndAnswer>,
    /// This is the ID of the user that submitted the form
    pub submitted_by: ObjectId,
    /// This is the time that they submitted it
    pub submitted_at: DateTime,
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

/// ID of choice in the questions that is selected
pub type MultichoiceAnswer = ObjectId;
/// Numerical value that the user selects
pub type SliderAnswer = f64;
/// String for the answer that the client types
pub type FreeFormAnswer = String;

/// Combination of both the question and answer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QuestionAndAnswer {
    Multichoice(ObjectId, MultichoiceAnswer),
    Slider(ObjectId, SliderAnswer),
    FreeForm(ObjectId, FreeFormAnswer),
}

/// Free form question with some validation rules you could apply
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FreeFormQuestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub max_length: u64,
    pub min_length: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SliderQuestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub units: Option<String>,
    pub low: f64,
    pub high: f64,
    pub step: f64,
    pub highest_message: Option<String>,
    pub middle_message: Option<String>,
    pub lowest_message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub options: Vec<MultichoiceQuestionOption>,
    pub min_selected: u64,
    pub max_selected: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestionOption {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CaregiverToken {
    pub token: String,
    pub user_id: ObjectId,
    created_at: mongodb::bson::DateTime,
    expired_by: mongodb::bson::DateTime,
}
fn generate_random_string(length: usize) -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
        .to_ascii_lowercase()
}

impl CaregiverToken {
    pub fn new(user_id: ObjectId) -> CaregiverToken {
        let expired_by = Utc::now() + Duration::days(3);
        CaregiverToken {
            token: generate_random_string(10),
            expired_by: mongodb::bson::DateTime::from_millis(expired_by.timestamp_millis()),
            user_id,
            created_at: mongodb::bson::DateTime::now(),
        }
    }
}
