use async_graphql::{Context, Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::{Error, ErrorCode, Result};
use crate::modules::user::{User, Gender, Pronoun};
use crate::services::Services;

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccountRegister {
    user: Option<User>,
    error: Option<AccountRegisterError>,
}

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccountRegisterError {
    field: String,
    message: String,
    code: AccountRegisterErrorCode,
}

#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum AccountRegisterErrorCode {
    UsernameTaken,
}

impl TryFrom<Error> for AccountRegister {
    type Error = Error;

    fn try_from(value: Error) -> std::result::Result<Self, Self::Error> {
        match value.code {
            ErrorCode::Unique => Ok(AccountRegister {
                user: None,
                error: Some(AccountRegisterError {
                    field: String::from("username"),
                    message: String::from("Username is already taken"),
                    code: AccountRegisterErrorCode::UsernameTaken,
                }),
            }),
            _ => Err(value),
        }
    }
}

#[derive(Deserialize, Serialize, InputObject)]
#[graphql(input_name = "AccountRegisterInput")]
pub struct AccountRegisterInput {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub gender: Option<Gender>,
    pub pronoun: Option<Pronoun>,
    pub gender_name: Option<String>,
    pub birthdate: DateTime<Utc>,
}

pub async fn exec(ctx: &Context<'_>, input: AccountRegisterInput) -> Result<AccountRegister> {
    let services = ctx.data::<Arc<Services>>().unwrap();
    let result = services.user.create(input).await;

    match result {
        Ok(user) => Ok(AccountRegister {
            user: Some(user),
            error: None,
        }),
        Err(err) => {
            let account_register = AccountRegister::try_from(err)?;

            Ok(account_register)
        }
    }
}
