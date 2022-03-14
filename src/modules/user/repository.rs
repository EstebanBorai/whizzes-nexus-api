use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::database::Database;
use crate::schema::users;
use crate::Result;

use super::entity::User;

#[derive(Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct UsersTableRow {
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

impl From<UsersTableRow> for User {
    fn from(dto: UsersTableRow) -> Self {
        User {
            id: dto.id,
            name: dto.name,
            last_name: dto.last_name,
            email: dto.email,
            username: dto.username,
            password_hash: dto.password_hash,
            birthdate: dto.birthdate,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "users"]
pub struct InsertUserTableRow {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub birthdate: DateTime<Utc>,
}

pub struct UserRepository {
    database: Arc<Database>,
}

impl UserRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        let conn = self.database.conn_pool.get()?;
        let users = users::table.load::<UsersTableRow>(&conn)?;
        let users = users.into_iter().map(User::from).collect::<Vec<User>>();

        Ok(users)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User> {
        let conn = self.database.conn_pool.get()?;
        let row = users::table
            .filter(users::username.eq(username))
            .limit(1)
            .first::<UsersTableRow>(&conn)?;

        Ok(User::from(row))
    }

    pub async fn insert(&self, dto: InsertUserTableRow) -> Result<User> {
        let conn = self.database.conn_pool.get()?;
        let row = diesel::insert_into(users::table)
            .values(dto)
            .get_result::<UsersTableRow>(&conn)?;
        let user = User::from(row);

        Ok(user)
    }
}
