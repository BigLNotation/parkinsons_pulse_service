use anyhow::anyhow;
use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, response::Response,
    RequestPartsExt,
};
use mongodb::bson::oid::ObjectId;
use tower_cookies::Cookies;
use tracing::info;

use crate::app::{auth::utils::decode_jwt, models::User};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserClaims {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    // national_health_identifer: String,
    pub email_address: String,
    pub is_patient: bool,
}

impl TryInto<UserClaims> for User {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<UserClaims, Self::Error> {
        Ok(UserClaims {
            id: self.id.clone().ok_or(anyhow!("user is missing id"))?,
            first_name: self.first_name,
            last_name: self.last_name,
            email_address: self.email_address,
            is_patient: self.is_patient,
        })
    }
}

pub struct Auth(pub Option<UserClaims>);

#[async_trait]
impl<B> FromRequestParts<B> for Auth
where
    B: Send,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let Ok(cookies) = parts.extract::<Cookies>().await else {
            info!("Could not extract cookies from request");
            return Ok(Auth(None));
        };
        let Some(auth_cookie) = cookies.get("auth_token") else {
            info!("Could not get cookie :(");
            return Ok(Auth(None));
        };
        match decode_jwt(auth_cookie.value()) {
            Ok(value) => Ok(Auth(Some(value.user))),
            Err(err) => {
                info!(
                    "{}: Decoding JWT Token Failed, {}",
                    err,
                    auth_cookie.value()
                );
                Ok(Auth(None))
            }
        }
    }
}
