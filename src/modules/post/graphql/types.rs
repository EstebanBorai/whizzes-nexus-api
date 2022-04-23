use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorCode};

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct PostError {
    field: Option<String>,
    message: Option<String>,
    code: PostErrorCode,
}

#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum PostErrorCode {
    Unauthorized,
}

impl PostError {
    pub fn unathorized() -> Self {
        PostError {
            field: None,
            message: Some(String::from("Token is either missing or invalid.")),
            code: PostErrorCode::Unauthorized,
        }
    }
}

impl TryFrom<Error> for PostError {
    type Error = Error;

    fn try_from(value: Error) -> std::result::Result<Self, Self::Error> {
        match value.code {
            ErrorCode::InvalidJsonWebToken => Ok(PostError {
                field: None,
                message: None,
                code: PostErrorCode::Unauthorized,
            }),
            _ => Err(value),
        }
    }
}
