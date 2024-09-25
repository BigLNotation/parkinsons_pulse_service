use crate::{app::auth::middleware::UserClaims, config};
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
    #[must_use]
    pub fn new(value: String) -> Self {
        Self {
            cookie: CookieBuilder::new("auth_token", value)
                .domain(config::get_domain())
                .path("/")
                .http_only(true)
                .same_site(tower_cookies::cookie::SameSite::Lax)
                .secure(false),
        }
    }
    #[must_use]
    pub fn max_age(self, age: tower_cookies::cookie::time::Duration) -> Self {
        let mut exp_offset = OffsetDateTime::now_utc();
        exp_offset += age;
        Self {
            cookie: self.cookie.max_age(age).expires(exp_offset),
        }
    }
    #[must_use]
    pub fn build(self) -> Cookie<'a> {
        self.cookie.build()
    }
}

///
/// # Errors
/// This function errors if it couldn't add a signed and dated expiry or couldn't convert exp to a usize
///
///
///
///
pub fn generate_jwt(user: &UserClaims) -> anyhow::Result<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::weeks(3))
        .ok_or(anyhow!("couldn't add signed"))?
        .timestamp();
    let claims = Claims {
        user: user.clone(),
        exp: usize::try_from(exp)?,
    };
    let jwt_secret = config::get_jwt_secret();
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?)
}

///
///
/// # Errors
/// This function errors if the JWT token is invalid
pub fn decode_jwt(token: &str) -> anyhow::Result<Claims> {
    let jwt_secret = config::get_jwt_secret();
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}
