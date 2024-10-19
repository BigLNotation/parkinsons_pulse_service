use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bson::DateTime;
use mongodb::{bson::doc, Collection, Database};
use tracing::info;

use crate::app::{
    auth::middleware::Auth,
    models::{dto::caregiver::CaregiverTokenPath, CaregiverToken, User},
};

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
    info!(
        "Deleted {} expired caregiver tokens",
        delete_result.deleted_count
    );

    Ok(())
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn add_caregiver(
    State(db): State<Database>,
    Auth(auth): Auth,
    Path(path): Path<CaregiverTokenPath>,
) -> Response {
    let Some(auth) = auth else {
        return (
            StatusCode::UNAUTHORIZED,
            String::from("You must be signed in to create a form"),
        )
            .into_response();
    };

    // Delete all of the expired tokens from the database
    match delete_expired_tokens(&db.collection::<CaregiverToken>("caregiver_tokens")).await {
        Err(..) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not verify token"),
            )
                .into_response()
        }
        Ok(..) => {}
    };

    let Ok(Some(found_token)) = db
        .collection::<CaregiverToken>("caregiver_tokens")
        .find_one(doc! {
          "token": path.token.clone()
        })
        .await
    else {
        return (StatusCode::NOT_FOUND, String::from("Could not find token")).into_response();
    };

    match db
        .collection::<CaregiverToken>("caregiver_tokens")
        .delete_one(doc! {
          "token": path.token.clone()
        })
        .await
    {
        Ok(..) => {}
        Err(..) => {}
    }

    let user_id = found_token.user_id;

    if user_id == auth.id {
        return (
            StatusCode::BAD_REQUEST,
            String::from("Cannot add yourself as a caregiver"),
        )
            .into_response();
    }

    match db
        .collection::<User>("users")
        .find_one_and_update(
            doc! {
              "_id": user_id
            },
            doc! {
              "$addToSet": {
                "caregivers": auth.id
              }
            },
        )
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(err) => {
            tracing::error!("{:#?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Could not add you as a caregiver"),
            )
        }
        .into_response(),
    }
}
