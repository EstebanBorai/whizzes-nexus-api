use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use diesel::{AsExpression, FromSqlRow, Identifiable};
use diesel::pg::Pg;
use diesel::sql_types::Text;
use diesel::types::{ToSql, FromSql};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(AsExpression, Copy, Clone, Debug, Deserialize, Enum, Eq, FromSqlRow, PartialEq, Serialize, SqlType)]
#[postgres(type_name = "gender")]
#[sql_type = "Text"]
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
            _ => panic!("{}", &format!("The value: {s} doesn't corresponds to a `Gender` variant."))
        }
    }
}

impl ToSql<Text, Pg> for Gender {
    fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, Pg>) -> diesel::serialize::Result {
        let gender = format!("{:?}", self);

        ToSql::<Text, Pg>::to_sql(&gender, out)
    }
}

impl FromSql<Text, Pg> for Gender {
    fn from_sql(bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>) -> diesel::deserialize::Result<Self> {
        FromSql::<Text, Pg>::from_sql(bytes)
            .map(|&s| Gender::from)
    }
}

#[derive(AsExpression, Copy, Clone, Debug, Deserialize, Enum, Eq, FromSqlRow, PartialEq, Serialize, SqlType)]
#[postgres(type_name = "pronoun")]
#[sql_type = "Text"]
pub enum Pronoun {
    He,
    She,
    They,
}

impl ToSql<Text, Pg> for Pronoun {
    fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, Pg>) -> diesel::serialize::Result {
        let pronoun = format!("{:?}", self);

        ToSql::<Text, Pg>::to_sql(&pronoun, out)
    }
}

impl FromSql<Text, Pg> for Pronoun {
    fn from_sql(bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>) -> diesel::deserialize::Result<Self> {
        FromSql::<Text, Pg>::from_sql(bytes)
            .map(|&s| Pronoun::from)
    }
}

impl From<&str> for Pronoun {
    fn from(s: &str) -> Self {
        match s {
            "he" => Self::He,
            "she" => Self::She,
            "they" => Self::They,
            _ => panic!("{}", &format!("The value: {s} doesn't corresponds to a `Pronoun` variant."))
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
