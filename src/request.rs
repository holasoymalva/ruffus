//! HTTP Request type

use bytes::Bytes;
use http::{HeaderMap, Method, Uri};
use std::collections::HashMap;

/// Represents an incoming HTTP request
pub struct Request {
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
    params: HashMap<String, String>,
    query: HashMap<String, String>,
}

impl Request {
    /// Get the HTTP method
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get the request URI
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Get the request headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get a path parameter by name
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(|s| s.as_str())
    }

    /// Get a query parameter by name
    pub fn query(&self, name: &str) -> Option<&str> {
        self.query.get(name).map(|s| s.as_str())
    }
}
