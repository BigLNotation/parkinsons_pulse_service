mod create;
mod push_answers;

use axum::{routing::put, Router};
use create::create_form;
use push_answers::push_form_answers;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", put(create_form))
        .route("/push_answers", put(push_form_answers))
}
