//! Error types for Ruffus

use http::StatusCode;
use std::fmt;

/// Main error type for Ruffus
#[derive(Debug)]
pub enum Error {
    /// Route not found (404)
    RouteNotFound,
    /// Method not allowed (405)
    MethodNotAllowed(Vec<http::Method>),
    /// Bad request (400)
    BadRequest(String),
    /// Internal server error (500)
    InternalServerError(String),
    /// JSON parsing error
    JsonParseError(serde_json::Error),
    /// JSON serialization error
    JsonSerializeError(serde_json::Error),
    /// Custom error with status and message
    Custom {
        status: StatusCode,
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RouteNotFound => write!(f, "Route not found"),
            Error::MethodNotAllowed(methods) => {
                write!(f, "Method not allowed. Allowed methods: {:?}", methods)
            }
            Error::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Error::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            Error::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
            Error::JsonSerializeError(e) => write!(f, "JSON serialize error: {}", e),
            Error::Custom { status, message } => write!(f, "{}: {}", status, message),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::RouteNotFound => StatusCode::NOT_FOUND,
            Error::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::JsonParseError(_) => StatusCode::BAD_REQUEST,
            Error::JsonSerializeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Custom { status, .. } => *status,
        }
    }
}
