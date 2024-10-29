pub mod add;
pub mod find;
pub mod generate;
pub mod remove;

use axum::{
    routing::{delete, get, post},
    Router,
};
use bson::{doc, DateTime};
use mongodb::Collection;

use crate::app::auth::info;

use super::{models::CaregiverToken, AppState};

async fn delete_expired_tokens(
    collection: &Collection<CaregiverToken>,
) -> mongodb::error::Result<()> {
    let now = DateTime::now();

    let filter = doc! {
      "expired_by": {
        "$lt": now
      }
    };

    let delete_result = collection.delete_many(filter).await?;
    tracing::info!(
        "Deleted {} expired caregiver tokens",
        delete_result.deleted_count
    );

    Ok(())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/list", get(find::find_caregiver))
        .route("/generate", get(generate::generate))
        .route("/add/:token", post(add::add_caregiver))
        .route("/remove/:caregiver_id", delete(remove::remove_caregiver))
}
