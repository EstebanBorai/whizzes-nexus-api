use argon2::{self, Config};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::error::Result;
use crate::modules::user::graphql::account_register::AccountRegisterInput;

use super::{InsertUserTableRow, User, UserRepository};

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

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        self.repository.find_by_username(username).await
    }

    pub async fn create(&self, payload: AccountRegisterInput) -> Result<User> {
        let password_hash = self.hash_password(&payload.password)?;
        let inserted = self
            .repository
            .insert(InsertUserTableRow {
                name: payload.name,
                last_name: payload.last_name,
                email: payload.email,
                username: payload.username,
                gender: payload.gender,
                pronoun: payload.pronoun,
                custom_gender: payload.custom_gender,
                password_hash,
                birthdate: payload.birthdate,
            })
            .await?;

        Ok(inserted)
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
