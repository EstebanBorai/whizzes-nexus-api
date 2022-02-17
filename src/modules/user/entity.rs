use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub birthdate: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct UserTableRow {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub birthdate: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserTableRow> for User {
    fn from(dto: UserTableRow) -> Self {
        User {
            id: dto.id,
            name: dto.name,
            last_name: dto.last_name,
            email: dto.email,
            username: dto.username,
            birthdate: dto.birthdate,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}
