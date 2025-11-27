//! HTTP Response type

use bytes::Bytes;
use http::{HeaderMap, StatusCode};
use serde::Serialize;

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

    /// Create a JSON response from a serializable value
    pub fn json<T: Serialize>(value: &T) -> crate::Result<Self> {
        let json_string = serde_json::to_string(value)
            .map_err(crate::Error::JsonSerializeError)?;
        
        Ok(Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::from(json_string),
        }
        .header("Content-Type", "application/json"))
    }

    /// Set the body
    pub fn body(mut self, body: String) -> Self {
        self.body = Bytes::from(body);
        self
    }

    /// Get the status code
    pub fn get_status(&self) -> StatusCode {
        self.status
    }

    /// Get the headers
    pub fn get_headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get the body
    pub fn get_body(&self) -> &Bytes {
        &self.body
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Response> for hyper::Response<http_body_util::Full<Bytes>> {
    fn from(response: Response) -> Self {
        let mut builder = hyper::Response::builder()
            .status(response.status);

        // Add all headers
        for (key, value) in response.headers.iter() {
            builder = builder.header(key, value);
        }

        builder
            .body(http_body_util::Full::new(response.body))
            .expect("Failed to build hyper response")
    }
}
