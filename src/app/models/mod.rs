use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    first_name: String,
    last_name: String,
    national_health_identifer: String,
    email_address: String,
    hashed_password: String,
    is_patient: bool,
    caregivers: Vec<ObjectId>,
    form_templates: Vec<Form>,
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
///     questions: vec![
///         Question::Multichoice(MultichoiceQuestion {
///             _id: ObjectId::new(),
///             title: String::from("How many times have you experienced this in the last week?"),
///             options: vec![MultichoiceQuestionOption {
///                 name: String::from("Once"),
///                 _id: ObjectId::new(),
///             }],
///             min_selected: 1,
///             max_selected: 2,
///         }),
///         Question::FreeForm(FreeFormQuestion {
///             _id: ObjectId::new(),
///             title: String::from("Is there anything else you would like to add?"),
///             max_length: 200,
///             min_length: 0,
///         }),
///     ],
///     events: vec![
///         Event::QuestionEdited(QuestionEdited {
///             question_id: ObjectId::new(),
///             former_question: Question::FreeForm(FreeFormQuestion {
///                 _id: ObjectId::new(),
///                 title: String::from("How are you feeling this week?"),
///                 max_length: 100,
///                 min_length: 10,
///             }),
///             new_question: Question::FreeForm(FreeFormQuestion {
///                 _id: ObjectId::new(),
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
///                     MultichoiceQuestion {
///                         _id: ObjectId::new(),
///                         title: String::from(
///                             "How many times have you experienced this in the last week?",
///                         ),
///                         options: vec![MultichoiceQuestionOption {
///                             name: String::from("Once"),
///                             _id: ObjectId::new(),
///                         }],
///                         min_selected: 1,
///                         max_selected: 2,
///                     },
///                     ObjectId::new(),
///                 ),
///                 QuestionAndAnswer::FreeForm(
///                     FreeFormQuestion {
///                         _id: ObjectId::new(),
///                         title: String::from("Is there anything else you would like to add?"),
///                         max_length: 200,
///                         min_length: 0,
///                     },
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
    /// Title of the form for clients
    pub title: String,
    pub created_by: ObjectId,
    pub created_at: DateTime,
    /// List of questions in the form
    pub questions: Vec<Question>,
    /// List of events such as a user filling in a form or a moderator updating the form
    pub events: Vec<Event>,
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
    Multichoice(MultichoiceQuestion, MultichoiceAnswer),
    Slider(SliderQuestion, SliderAnswer),
    FreeForm(FreeFormQuestion, FreeFormAnswer),
}

/// Free form question with some validation rules you could apply
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FreeFormQuestion {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub max_length: u64,
    pub min_length: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SliderQuestion {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: ObjectId,
    pub title: String,
    pub units: Option<String>,
    pub low: f64,
    pub high: f64,
    pub step: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestion {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: ObjectId,
    pub title: String,
    pub options: Vec<MultichoiceQuestionOption>,
    pub min_selected: u64,
    pub max_selected: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestionOption {
    pub name: String,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: ObjectId,
}
