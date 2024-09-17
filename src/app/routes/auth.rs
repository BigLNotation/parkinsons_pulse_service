use argon2::{self, Config};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};
use mongodb::{
    bson::{self, doc, to_bson, Bson},
    error::{ErrorKind, WriteError},
};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_cookies::Cookies;
use tracing::info;

use crate::{
    app::middleware::auth::Auth,
    app::models::User,
    app::utils::auth::{generate_jwt, AuthCookieBuilder},
    app::AppState,
};

fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
}

fn verify_password(password: &[u8], hash: &String) -> argon2::Result<bool> {
    argon2::verify_encoded(hash, password)
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create))
        .route("/login", post(login))
        .route("/logout", patch(logout))
        .route("/info", get(info))
}

pub async fn info(Auth(user): Auth) -> Response {
    match user {
        Some(info) => (StatusCode::OK, serde_json::to_string(&info).unwrap()).into_response(),
        None => (StatusCode::BAD_REQUEST).into_response(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserBody {
    first_name: String,
    last_name: String,
    password: String,
    email_address: String,
    is_patient: bool,
}

pub async fn create(
    State(state): State<AppState>,
    Json(create_user_body): Json<CreateUserBody>,
) -> Response {
    let Ok(hashed_password) = hash_password(&create_user_body.password) else {
        return (
            StatusCode::BAD_REQUEST,
            String::from("failed to hash password"),
        )
            .into_response();
    };

    let new_user = User {
        id: None,
        email_address: create_user_body.email_address,
        first_name: create_user_body.first_name,
        last_name: create_user_body.last_name,
        hashed_password,
        is_patient: create_user_body.is_patient,
        caregivers: vec![],
        form_templates: vec![],
    };

    let Ok(serialized_user) = to_bson(&new_user) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Could not serialize user"),
        )
            .into_response();
    };
    let Bson::Document(document) = serialized_user else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Could not serialize user"),
        )
            .into_response();
    };
    match state.db.collection("users").insert_one(document).await {
        Ok(..) => StatusCode::OK.into_response(),
        Err(err) => match err.kind.as_ref() {
            ErrorKind::Write(mongodb::error::WriteFailure::WriteError(err)) => {
                if err.code == 11000 {
                    return (
                        StatusCode::BAD_REQUEST,
                        String::from("Email Address in use"),
                    )
                        .into_response();
                }
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserBody {
    email_address: String,
    password: String,
}

pub async fn login(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(body): Json<LoginUserBody>,
) -> Response {
    let query = doc! {
      "email_address": &body.email_address,
    };

    let Ok(Some(user)) = state.db.collection::<User>("users").find_one(query).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let Ok(true) = verify_password(&body.password.as_bytes(), &user.hashed_password) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let Ok(user_without_password) = user.try_into() else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    info!(
        "generated user without password: {:#?}",
        user_without_password
    );

    let Ok(cookie_value) = generate_jwt(&user_without_password) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("couldn't generate a cookie"),
        )
            .into_response();
    };

    let auth_cookie = AuthCookieBuilder::new(cookie_value)
        .max_age(tower_cookies::cookie::time::Duration::weeks(2))
        .build();

    info!("{:#?}", auth_cookie);
    cookies.add(auth_cookie);

    StatusCode::OK.into_response()
}

async fn logout(mut cookies: Cookies) -> Response {
    let mut auth_cookie = AuthCookieBuilder::new("".to_string()).build();
    auth_cookie.make_removal();
    cookies.add(auth_cookie);
    (StatusCode::OK).into_response()
}
