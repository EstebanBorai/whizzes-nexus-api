use chrono::{DateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::types::{FromSql, ToSql};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::database::Database;
use crate::error::Result;
use crate::schema::users;

use super::entity::{Gender, Pronoun, User};

/// Refer: https://github.com/diesel-rs/diesel/blob/d5ce11bdd58cbb39f5c93a50cf7342d7851217b0/diesel_tests/tests/custom_types.rs#L18
#[derive(Debug, SqlType)]
#[postgres(type_name = "gender")]
pub struct PgGender;

impl ToSql<PgGender, Pg> for Gender {
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Pg>,
    ) -> diesel::serialize::Result {
        let gender = format!("{:?}", self);

        ToSql::<Text, Pg>::to_sql(&gender, out)
    }
}

impl FromSql<PgGender, Pg> for Gender {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        FromSql::<Text, Pg>::from_sql(bytes).map(|v: String| Gender::from(v.as_str()))
    }
}

/// Refer: https://github.com/diesel-rs/diesel/blob/d5ce11bdd58cbb39f5c93a50cf7342d7851217b0/diesel_tests/tests/custom_types.rs#L18
#[derive(Debug, SqlType)]
#[postgres(type_name = "pronoun")]
pub struct PgPronoun;

impl ToSql<PgPronoun, Pg> for Pronoun {
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Pg>,
    ) -> diesel::serialize::Result {
        let pronoun = format!("{:?}", self);

        ToSql::<Text, Pg>::to_sql(&pronoun, out)
    }
}

impl FromSql<PgPronoun, Pg> for Pronoun {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        FromSql::<Text, Pg>::from_sql(bytes).map(|v: String| Pronoun::from(v.as_str()))
    }
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct UsersTableRow {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub gender: Option<Gender>,
    pub pronoun: Option<Pronoun>,
    pub gender_name: Option<String>,
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
            gender_name: dto.gender_name,
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
    pub gender: Option<Gender>,
    pub pronoun: Option<Pronoun>,
    pub gender_name: Option<String>,
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
