use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorCode};

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct UserError {
    field: Option<String>,
    message: Option<String>,
    code: UserErrorCode,
}

#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    Unauthorized,
}

impl TryFrom<Error> for UserError {
    type Error = Error;

    fn try_from(value: Error) -> std::result::Result<Self, Self::Error> {
        match value.code {
            ErrorCode::InvalidJsonWebToken => Ok(UserError {
                field: None,
                message: None,
                code: UserErrorCode::Unauthorized,
            }),
            _ => Err(value),
        }
    }
}
