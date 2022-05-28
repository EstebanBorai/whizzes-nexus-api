use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, ErrorCode};
use crate::modules::post::Scope;
use crate::modules::user::User;

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct Post {
    pub id: Uuid,
    pub content: String,
    pub user: User,
    pub scope: Scope,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
