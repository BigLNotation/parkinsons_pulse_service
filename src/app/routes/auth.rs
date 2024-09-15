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
    middleware::auth::Auth,
    utils::auth::{generate_jwt, AuthCookieBuilder},
    AppState,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct SignInInformation {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub first_name: String,
    pub last_name: String,
}

impl TryFrom<User> for SignInInformation {
    type Error = anyhow::Error;
    fn try_from(value: User) -> anyhow::Result<Self> {
        Ok(SignInInformation {
            id: value.id.ok_or(anyhow::anyhow!("missing user id"))?,
            first_name: value.first_name,
            last_name: value.last_name,
            phone_number: value.phone_number,
        })
    }
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

#[derive(Serialize, Deserialize, Clone)]
pub struct PhoneNumber {
    pub country_code: i64,
    pub national_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserBody {
    first_name: String,
    last_name: String,
    phone_number: PhoneNumber,
    password: String,
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
        first_name: create_user_body.first_name,
        last_name: create_user_body.last_name,
        phone_number: create_user_body.phone_number,
        hashed_password,
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
    match state
        .db
        .collection("users")
        .insert_one(document, None)
        .await
    {
        Ok(..) => StatusCode::OK.into_response(),
        Err(err) => match err.kind.as_ref() {
            ErrorKind::Write(mongodb::error::WriteFailure::WriteError(err)) => {
                if err.code == 11000 {
                    return (StatusCode::BAD_REQUEST, String::from("Phone Number in use"))
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
    phone_number: PhoneNumber,
    password: String,
}

pub async fn login(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(body): Json<LoginUserBody>,
) -> Response {
    let query = doc! {
      "phone_number.country_code": &body.phone_number.country_code,
      "phone_number.national_number": &body.phone_number.national_number,
    };

    let Ok(Some(user)) = state
        .db
        .collection::<User>("users")
        .find_one(query, None)
        .await
    else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let Ok(true) = verify_password(&body.password.as_bytes(), &user.hashed_password) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let Ok(user_without_password) = user.try_into() else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

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
