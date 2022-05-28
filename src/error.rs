use async_graphql::{Enum, ErrorExtensions};
use serde::{Deserialize, Serialize};

use crate::graphql::relay::Base64CursorError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, thiserror::Error, PartialEq, Serialize)]
pub enum ErrorCode {
    #[error("BASE64_CURSOR_ERROR")]
    Base64CursorError,
    #[error("SERVER_ERROR")]
    ServerError,
    #[error("INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[error("INVALID_JWT")]
    InvalidJsonWebToken,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("UNIQUE")]
    Unique,
    #[error("UNHANDLED")]
    Unhandled,
}

#[derive(Clone, Debug, Serialize)]
pub struct Error {
    pub field: Option<String>,
    pub message: Option<String>,
    pub code: ErrorCode,
}

impl Error {
    pub fn new(field: &str, message: &str, code: ErrorCode) -> Self {
        Self {
            field: Some(field.to_string()),
            message: Some(message.to_string()),
            code,
        }
    }

    pub fn code(code: ErrorCode) -> Self {
        Self {
            field: None,
            message: None,
            code,
        }
    }

    pub fn server_error() -> Self {
        Self {
            field: None,
            message: None,
            code: ErrorCode::ServerError,
        }
    }

    pub fn unique(field: &str, value: Option<&str>) -> Self {
        if let Some(value) = value {
            return Self {
                field: Some(field.to_string()),
                message: Some(format!("A {field} with {value} already exists")),
                code: ErrorCode::Unique,
            };
        }

        Self {
            field: Some(field.to_string()),
            message: Some(format!("The {field} already exists")),
            code: ErrorCode::Unique,
        }
    }

    pub fn unhandled(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self {
            field: Some(String::from("An unhandled error ocurred")),
            message: Some(err.to_string()),
            code: ErrorCode::Unhandled,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            field: None,
            message: Some(String::from("Missing access token")),
            code: ErrorCode::Unauthorized,
        }
    }
}

impl From<Error> for async_graphql::Error {
    fn from(err: Error) -> Self {
        let gql_error = async_graphql::Error::new("An error occurred");

        gql_error.extend_with(|_, e| {
            if let Some(message) = &err.message {
                e.set("message", message.to_string());
            }

            if let Some(field) = &err.field {
                e.set("field", field.to_string());
            }

            e.set("code", err.code.to_string());
        })
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(err: sqlx::error::Error) -> Self {
        Self::unhandled(Box::new(err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;

        match err.kind() {
            ErrorKind::InvalidToken => Error::code(ErrorCode::InvalidJsonWebToken),
            _ => Error::unhandled(Box::new(err)),
        }
    }
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Error::unhandled(Box::new(err))
    }
}

impl From<Base64CursorError> for Error {
    fn from(_err: Base64CursorError) -> Self {
        Error::code(ErrorCode::Base64CursorError)
    }
}

impl From<async_graphql::Error> for Error {
    fn from(err: async_graphql::Error) -> Self {
        println!("{:?}", err);
        Error::server_error()
    }
}
