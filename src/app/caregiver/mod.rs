pub mod add;
pub mod generate;

use axum::{
    routing::post,
    Router,
};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate", post(generate::generate))
        .route("/add/:token", post(add::add_caregiver))
}
