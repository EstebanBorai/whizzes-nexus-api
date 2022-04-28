use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use diesel::AsExpression;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Copy, Clone, DbEnum, Debug, Deserialize, Enum, Eq, PartialEq, Serialize,
)]
pub enum Gender {
    Female,
    Male,
    Custom,
}

#[derive(
    Copy, Clone, DbEnum, Debug, Deserialize, Enum, Eq, PartialEq, Serialize,
)]
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
    pub gender: Option<Gender>,
    pub pronoun: Option<Pronoun>,
    pub gender_name: Option<String>,
    pub birthdate: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
