use async_graphql::{Context, Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::{Error, ErrorCode, Result};
use crate::modules::auth::Tokens;
use crate::services::Services;

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct TokenCreate {
    tokens: Option<Tokens>,
    error: Option<TokenCreateError>,
}

#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct TokenCreateError {
    field: Option<String>,
    message: Option<String>,
    code: TokenCreateErrorCode,
}

impl TryFrom<Error> for TokenCreateError {
    type Error = Error;

    fn try_from(value: Error) -> std::result::Result<Self, Self::Error> {
        match value.code {
            ErrorCode::InvalidCredentials => Ok(TokenCreateError {
                field: None,
                message: None,
                code: TokenCreateErrorCode::InvalidCredentials,
            }),
            _ => Err(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum TokenCreateErrorCode {
    InvalidCredentials,
}

impl TokenCreate {
    pub async fn exec(
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<TokenCreate> {
        let services = ctx.data::<Arc<Services>>().unwrap();
        let result = services.auth.create_token(username, password).await;

        match result {
            Ok(tokens) => Ok(TokenCreate {
                tokens: Some(tokens),
                error: None,
            }),
            Err(err) => {
                let token_create_error = TokenCreateError::try_from(err)?;

                Ok(TokenCreate {
                    tokens: None,
                    error: Some(token_create_error),
                })
            }
        }
    }
}
