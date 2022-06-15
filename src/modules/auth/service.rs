use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::Config;
use crate::error::{Error, ErrorCode, Result};
use crate::modules::user::{User, UserService};

use super::Tokens;

pub struct AuthService {
    jwt_secret: Vec<u8>,
    user_service: Arc<UserService>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    uid: String,
}

impl AuthService {
    pub fn new(config: &Config, user_service: Arc<UserService>) -> Self {
        Self {
            jwt_secret: config.jwt_secret.clone().into_bytes(),
            user_service,
        }
    }

    /// Validate provided `username` and `password`. If valid, fetches the
    /// corresponding user and signs a JSON Web Token.
    pub async fn create_token(&self, username: String, password: String) -> Result<Tokens> {
        let find_user_by_username = self.user_service.find_by_username(&username).await?;

        if let Some(user) = find_user_by_username {
            let is_valid_password =
                argon2::verify_encoded(&user.password_hash, password.as_bytes())?;

            if is_valid_password {
                let iat = Utc::now().timestamp() as usize;
                let exp = Utc::now()
                    .checked_add_signed(Duration::days(30))
                    .unwrap()
                    .timestamp() as usize;

                let claims = Claims {
                    sub: String::from("nexus"),
                    iat,
                    exp,
                    uid: username,
                };

                let access_token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(&self.jwt_secret),
                )?;

                return Ok(Tokens { access_token });
            }
        }

        Err(Error::code(ErrorCode::InvalidCredentials))
    }

    /// Retrieves the User Data for the provided token
    pub async fn whoami(&self, token: String) -> Result<User> {
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&self.jwt_secret),
            &Validation::default(),
        )?;

        let username = token.claims.uid;
        let find_user_by_username = self.user_service.find_by_username(&username).await?;

        if let Some(user) = find_user_by_username {
            return Ok(user);
        }

        Err(Error::code(ErrorCode::InvalidCredentials))
    }
}
