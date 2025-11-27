//! HTTP Response type

use bytes::Bytes;
use http::{HeaderMap, StatusCode};

/// Represents an outgoing HTTP response
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl Response {
    /// Create a new empty Response
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    /// Set the status code
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Add a header
    pub fn header(mut self, key: &str, value: &str) -> Self {
        if let (Ok(name), Ok(val)) = (
            http::header::HeaderName::from_bytes(key.as_bytes()),
            http::header::HeaderValue::from_str(value),
        ) {
            self.headers.insert(name, val);
        }
        self
    }

    /// Create a plain text response
    pub fn text(text: String) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::from(text),
        }
    }

    /// Set the body
    pub fn body(mut self, body: String) -> Self {
        self.body = Bytes::from(body);
        self
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}
