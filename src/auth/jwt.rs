use chrono::{Duration, Local};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::constants::messages;
use crate::models::{response::Response, usuario::Usuario};

#[derive(Eq, PartialEq, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserClaim {
    pub sub: String,
    pub exp: i64,
    pub role: String,
    pub tenant: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserClaim {
    type Error = status::Custom<Json<Response>>;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Success(UserClaim::default()),
            Some(auth) => {
                let auth_str = auth.to_string();
                if auth_str.starts_with("Bearer") {
                    let token = auth_str[6..auth_str.len()].trim();
                    if let Ok(token_data) = decode_token(token.to_string()) {
                        return Outcome::Success(token_data.claims);
                    }
                }
                return Outcome::Failure((
                    Status::Unauthorized,
                    status::Custom(
                        Status::Unauthorized,
                        Json(Response {
                            message: String::from(messages::MESSAGE_INVALID_TOKEN),
                            data: serde_json::to_value("").unwrap(),
                        }),
                    ),
                ));
            }
        }
    }
}

lazy_static! {
    static ref JWT_SECRET_KEY: String =
        std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");
}

pub fn generate_token(user: &Usuario) -> Result<String, Error> {
    let exp_time = Local::now() + Duration::days(1);
    let payload = UserClaim {
        exp: exp_time.timestamp(),
        sub: user.email.to_owned(),
        role: user.role.to_uppercase(),
        tenant: "".to_owned(),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
    )
}

pub fn decode_token(token: String) -> Result<TokenData<UserClaim>, Error> {
    jsonwebtoken::decode::<UserClaim>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
        &Validation::default(),
    )
}
