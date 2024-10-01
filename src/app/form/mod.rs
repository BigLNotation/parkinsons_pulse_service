mod create;
mod find;
mod submit;

use axum::{
    routing::{get, post},
    Router,
};
use create::create_form;
use find::find;
use submit::submit;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_form))
        .route("/find/:form_id", get(find))
        .route("/submit/:form_id", post(submit))
}
