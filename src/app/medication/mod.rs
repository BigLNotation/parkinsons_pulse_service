mod add;
mod find;
mod remove;
mod update;

use add::add_medication;
use axum::{
    routing::{get, post},
    Router,
};
use find::{find_all_medications, find_medication};
use remove::remove_medication;
use update::update_medication;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/find/:medication_id", get(find_medication))
        .route("/find", get(find_all_medications))
        .route("/add", post(add_medication))
        .route("/update/:medication_id", post(update_medication))
        .route("/remove/:medication_id", post(remove_medication))
}
