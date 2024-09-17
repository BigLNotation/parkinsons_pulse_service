use crate::app::{middleware::auth::UserClaims, models::User};
use anyhow::anyhow;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::{
    cookie::{time::OffsetDateTime, CookieBuilder},
    Cookie,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user: UserClaims,
    pub exp: usize,
}

pub struct AuthCookieBuilder<'a> {
    cookie: CookieBuilder<'a>,
}

impl<'a> AuthCookieBuilder<'a> {
    pub fn new(value: String) -> Self {
        Self {
            cookie: CookieBuilder::new("auth_token", value)
                .domain(std::env::var("DOMAIN").expect("DOMAIN environment variable must be set"))
                .path("/")
                .http_only(true)
                .same_site(tower_cookies::cookie::SameSite::Lax)
                .secure(false),
        }
    }
    pub fn max_age(self, age: tower_cookies::cookie::time::Duration) -> Self {
        let mut exp_offset = OffsetDateTime::now_utc();
        exp_offset += age;
        Self {
            cookie: self.cookie.max_age(age).expires(exp_offset),
        }
    }
    pub fn build(self) -> Cookie<'a> {
        self.cookie.build()
    }
}

pub fn generate_jwt(user: &UserClaims) -> anyhow::Result<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::weeks(3))
        .ok_or(anyhow!("couldn't add signed"))?
        .timestamp();
    let claims = Claims {
        user: user.clone(),
        exp: exp as usize,
    };
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?)
}

pub fn decode_jwt(token: &str) -> anyhow::Result<Claims> {
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )?;

    Ok(token_data.claims)
}
