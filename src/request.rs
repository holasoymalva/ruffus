//! HTTP Request type

use bytes::Bytes;
use http::{HeaderMap, Method, Uri};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Type for storing request extensions
pub type Extensions = http::Extensions;

/// Represents an incoming HTTP request
pub struct Request {
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
    params: HashMap<String, String>,
    query: HashMap<String, String>,
    extensions: Extensions,
}

impl Request {
    /// Create a new Request
    pub fn new(
        method: Method,
        uri: Uri,
        headers: HeaderMap,
        body: Bytes,
    ) -> Self {
        // Parse query parameters from URI
        let query = Self::parse_query_params(&uri);
        
        Self {
            method,
            uri,
            headers,
            body,
            params: HashMap::new(),
            query,
            extensions: Extensions::new(),
        }
    }

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

    /// Get all path parameters
    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Get all query parameters
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query
    }

    /// Set a path parameter (used internally by router)
    pub fn set_param(&mut self, name: String, value: String) {
        self.params.insert(name, value);
    }

    /// Deserialize the request body as JSON
    pub async fn json<T: DeserializeOwned>(&mut self) -> crate::Result<T> {
        let body_bytes = &self.body;
        serde_json::from_slice(body_bytes)
            .map_err(|e| crate::Error::JsonParseError(e))
    }

    /// Get the request body as bytes
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    /// Get mutable access to extensions
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }

    /// Get access to extensions
    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    /// Parse query parameters from URI
    fn parse_query_params(uri: &Uri) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        if let Some(query) = uri.query() {
            for pair in query.split('&') {
                if let Some((key, value)) = pair.split_once('=') {
                    // URL decode both key and value
                    if let (Ok(decoded_key), Ok(decoded_value)) = (
                        urlencoding::decode(key),
                        urlencoding::decode(value),
                    ) {
                        params.insert(decoded_key.into_owned(), decoded_value.into_owned());
                    }
                } else {
                    // Handle keys without values
                    if let Ok(decoded_key) = urlencoding::decode(pair) {
                        params.insert(decoded_key.into_owned(), String::new());
                    }
                }
            }
        }
        
        params
    }
}

/// Convert from hyper::Request
impl<B> TryFrom<hyper::Request<B>> for Request
where
    B: hyper::body::Body + Send + 'static,
    B::Data: Send,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    type Error = crate::Error;

    fn try_from(req: hyper::Request<B>) -> Result<Self, Self::Error> {
        let (parts, body) = req.into_parts();
        
        // We need to collect the body asynchronously, but this is a sync trait
        // This will be handled by the async conversion function below
        Err(crate::Error::InternalServerError(
            "Use from_hyper_async instead".to_string()
        ))
    }
}

impl Request {
    /// Async conversion from hyper::Request
    pub async fn from_hyper<B>(req: hyper::Request<B>) -> crate::Result<Self>
    where
        B: hyper::body::Body + Send + 'static,
        B::Data: Send,
        B::Error: std::error::Error + Send + Sync + 'static,
    {
        let (parts, body) = req.into_parts();
        
        // Collect the body
        let body_bytes = body
            .collect()
            .await
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?
            .to_bytes();
        
        Ok(Request::new(
            parts.method,
            parts.uri,
            parts.headers,
            body_bytes,
        ))
    }
}
