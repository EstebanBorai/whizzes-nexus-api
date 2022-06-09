use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::pool::Pool;
use sqlx::{FromRow, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::database::Database;
use crate::error::Result;

use super::entity::User;
use super::graphql::users::UsersFilter;
use super::{Gender, Pronoun};

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct UsersTableRow {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub gender: Gender,
    pub pronoun: Pronoun,
    pub custom_gender: Option<String>,
    pub birthdate: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UsersTableRow> for User {
    fn from(dto: UsersTableRow) -> Self {
        User {
            id: dto.id,
            name: dto.name,
            last_name: dto.last_name,
            email: dto.email,
            username: dto.username,
            password_hash: dto.password_hash,
            gender: dto.gender,
            pronoun: dto.pronoun,
            custom_gender: dto.custom_gender,
            birthdate: dto.birthdate,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertUserTableRow {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub birthdate: DateTime<Utc>,
    pub gender: Gender,
    pub pronoun: Pronoun,
    pub custom_gender: Option<String>,
}

pub struct UserRepository {
    database: Arc<Database>,
}

impl UserRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn find_all(&self, filter: Option<UsersFilter>) -> Result<Vec<User>> {
        if let Some(users_filter) = filter {
            let mut query = String::from("SELECT * FROM users WHERE");
            let mut binds = Vec::new();

            if let Some(username) = users_filter.username {
                query.push_str(" users.username = $1");
                binds.push(username);
            }

            let mut query = sqlx::query_as(query.as_str());

            for binding in binds {
                query = query.bind(binding);
            }

            let result: Vec<UsersTableRow> = query.fetch_all(&self.database.conn_pool).await?;
            let users = result.into_iter().map(User::from).collect::<Vec<User>>();

            return Ok(users);
        }

        let result: Vec<UsersTableRow> = sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.database.conn_pool)
            .await?;
        let users = result.into_iter().map(User::from).collect::<Vec<User>>();

        Ok(users)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User> {
        let result: UsersTableRow = sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one::<&Pool<Postgres>>(&self.database.conn_pool)
            .await?;

        Ok(User::from(result))
    }

    pub async fn insert(&self, dto: InsertUserTableRow) -> Result<User> {
        let result: UsersTableRow = sqlx::query_as(
            r#"
            INSERT INTO users (
                name,
                last_name,
                email,
                username,
                password_hash,
                birthdate,
                gender,
                pronoun,
                custom_gender
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7::gender,
                $8::pronoun,
                $9
            ) RETURNING *"#,
        )
        .bind(dto.name)
        .bind(dto.last_name)
        .bind(dto.email)
        .bind(dto.username)
        .bind(dto.password_hash)
        .bind(dto.birthdate)
        .bind(dto.gender)
        .bind(dto.pronoun)
        .bind(dto.custom_gender)
        .fetch_one(&self.database.conn_pool)
        .await?;

        Ok(User::from(result))
    }
}
