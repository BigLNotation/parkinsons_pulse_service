mod create;
mod get;

use axum::{
    routing::{get, put},
    Router,
};
use create::create_user;
use get::get_user;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/get/:id", get(get_user))
        .route("/create", put(create_user))
}
