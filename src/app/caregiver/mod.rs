use axum::{
    routing::{get, post},
    Router,
};
use create::create_form;
use find::{find, find_all};
use history::history;
use submit::submit;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/add", post(create_form))
}
