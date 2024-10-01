mod submit;

use axum::{routing::put, Router};
use submit::submit_form;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/submit", put(submit_form))
}
