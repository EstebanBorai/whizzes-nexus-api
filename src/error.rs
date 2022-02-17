use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use std::io::Cursor;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    status_code: u16,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    pub fn new(status_code: u16, message: &str) -> Self {
        ApiError {
            message: String::from(message),
            status_code,
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let status = Status::from_code(self.status_code).unwrap();
        let body = serde_json::to_string(&self).unwrap();

        Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}

impl From<Box<dyn std::error::Error>> for ApiError {
    fn from(_err: Box<dyn std::error::Error>) -> Self {
        ApiError::new(500, "An unhandled error ocurred")
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::DatabaseError(_, err) => ApiError::new(409, err.message()),
            diesel::result::Error::NotFound => ApiError::new(404, "Record not found"),
            _ => ApiError::new(500, "An unexpected error ocurred"),
        }
    }
}

impl From<r2d2::Error> for ApiError {
    fn from(error: r2d2::Error) -> Self {
        ApiError::new(500, &error.to_string())
    }
}
