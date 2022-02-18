use argon2::{self, Config};
use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::Result;

use super::{InsertUserTableRow, User, UserRepository};

#[derive(Deserialize, Serialize, InputObject)]
#[graphql(input_name = "UserCreateInput")]
pub struct UserCreateDto {
    name: String,
    last_name: String,
    email: String,
    username: String,
    password: String,
    birthdate: DateTime<Utc>,
}

pub struct UserService {
    repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        let users = self.repository.find_all().await?;

        Ok(users)
    }

    pub async fn create(&self, payload: UserCreateDto) -> Result<User> {
        let password_hash = self.hash_password(&payload.password)?;

        self.repository
            .insert(InsertUserTableRow {
                name: payload.name,
                last_name: payload.last_name,
                email: payload.email,
                username: payload.username,
                password_hash,
                birthdate: payload.birthdate,
            })
            .await
    }

    fn hash_password(&self, raw: &str) -> Result<String> {
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let hash =
            argon2::hash_encoded(raw.as_bytes(), salt.as_bytes(), &Config::default()).unwrap();

        Ok(hash)
    }
}
