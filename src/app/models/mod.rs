use std::collections::HashMap;

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
    form_filled: Vec<Form>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Form {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    created_by: ObjectId,
    created_at: DateTime,
    questions: Vec<Question>,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    FormSubmitted(FormSubmitted),
    QuestionEdited(QuestionEdited),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuestionEdited {
    question_id: ObjectId,
    former_question: Question,
    new_question: Question,
    edited_by: ObjectId,
    edited_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormSubmitted {
    answers: Vec<QuestionAndAnswer>,
    submitted_by: ObjectId,
    submitted_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Question {
    Multichoice(MultichoiceQuestion),
    Slider(SliderQuestion),
    FreeForm(FreeFormQuestion),
}

pub type MultichoiceAnswer = ObjectId;
pub type SliderAnswer = f64;
pub type FreeFormAnswer = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QuestionAndAnswer {
    Multichoice(MultichoiceQuestion, MultichoiceAnswer),
    Slider(SliderQuestion, SliderAnswer),
    FreeForm(FreeFormQuestion, FreeFormAnswer),
}

// In the template, the answer will be set to the default
// it is when the form is opened
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FreeFormQuestion {
    id: ObjectId,
    title: String,
    max_length: u64,
    min_length: u64,
}

// In the template, the answer will store the default value
// for when the form is opened
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SliderQuestion {
    id: ObjectId,
    title: String,
    units: Option<String>,
    low: f64,
    high: f64,
    step: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestion {
    id: ObjectId,
    title: String,
    options: Vec<MultichoiceQuestionOption>,
    min_selected: u64,
    max_selected: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MultichoiceQuestionOption {
    name: String,
    id: ObjectId,
}

fn main() {
    let form: Form = Form {
        id: Some(ObjectId::new()),
        title: String::from("Tremors"),
        created_by: ObjectId::new(),
        created_at: DateTime::now(),
        questions: vec![
            Question::Multichoice(MultichoiceQuestion {
                id: ObjectId::new(),
                title: String::from("How many times have you experienced this in the last week?"),
                options: vec![MultichoiceQuestionOption {
                    name: String::from("Once"),
                    id: ObjectId::new(),
                }],
                min_selected: 1,
                max_selected: 2,
            }),
            Question::FreeForm(FreeFormQuestion {
                id: ObjectId::new(),
                title: String::from("Is there anything else you would like to add?"),
                max_length: 200,
                min_length: 0,
            }),
        ],
        events: vec![
            Event::QuestionEdited(QuestionEdited {
                question_id: ObjectId::new(),
                former_question: Question::FreeForm(FreeFormQuestion {
                    id: ObjectId::new(),
                    title: String::from("How are you feeling this week?"),
                    max_length: 100,
                    min_length: 10,
                }),
                new_question: Question::FreeForm(FreeFormQuestion {
                    id: ObjectId::new(),
                    title: String::from("Is there anything else you would like to add?"),
                    max_length: 200,
                    min_length: 0,
                }),
                edited_at: DateTime::now(),
                edited_by: ObjectId::new(),
            }),
            Event::FormSubmitted(FormSubmitted {
                answers: vec![
                    QuestionAndAnswer::Multichoice(
                        MultichoiceQuestion {
                            id: ObjectId::new(),
                            title: String::from(
                                "How many times have you experienced this in the last week?",
                            ),
                            options: vec![MultichoiceQuestionOption {
                                name: String::from("Once"),
                                id: ObjectId::new(),
                            }],
                            min_selected: 1,
                            max_selected: 2,
                        },
                        ObjectId::new(),
                    ),
                    QuestionAndAnswer::FreeForm(
                        FreeFormQuestion {
                            id: ObjectId::new(),
                            title: String::from("Is there anything else you would like to add?"),
                            max_length: 200,
                            min_length: 0,
                        },
                        String::from("I wasn't able to press the elevator buttons this morning"),
                    ),
                ],
                submitted_at: DateTime::now(),
                submitted_by: ObjectId::new(),
            }),
        ],
    };
}
