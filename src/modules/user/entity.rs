use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    AsExpression, Copy, Clone, Debug, Deserialize, Enum, Eq, FromSqlRow, PartialEq, Serialize,
)]
#[sql_type = "crate::modules::user::repository::PgGender"]
pub enum Gender {
    Female,
    Male,
    Custom,
}

impl From<&str> for Gender {
    fn from(s: &str) -> Self {
        match s {
            "female" => Self::Female,
            "male" => Self::Male,
            "custom" => Self::Custom,
            _ => panic!(
                "{}",
                &format!("The value: {s} doesn't corresponds to a `Gender` variant.")
            ),
        }
    }
}

#[derive(
    AsExpression, Copy, Clone, Debug, Deserialize, Enum, Eq, FromSqlRow, PartialEq, Serialize,
)]
#[sql_type = "crate::modules::user::repository::PgPronoun"]
pub enum Pronoun {
    He,
    She,
    They,
}

impl From<&str> for Pronoun {
    fn from(s: &str) -> Self {
        match s {
            "he" => Self::He,
            "she" => Self::She,
            "they" => Self::They,
            _ => panic!(
                "{}",
                &format!("The value: {s} doesn't corresponds to a `Pronoun` variant.")
            ),
        }
    }
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
