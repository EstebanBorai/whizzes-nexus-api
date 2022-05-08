use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Deserialize, Enum, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Gender {
    Female,
    Male,
    Custom,
}

#[derive(Copy, Clone, Debug, Deserialize, Enum, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Pronoun {
    He,
    She,
    They,
}

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub gender: Gender,
    pub pronoun: Pronoun,
    pub custom_gender: Option<String>,
    pub birthdate: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
