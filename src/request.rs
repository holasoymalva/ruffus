//! HTTP Request type
//!
//! This module provides the [`Request`] type which represents an incoming HTTP request.

use bytes::Bytes;
use http::{HeaderMap, Method, Uri};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Type for storing request extensions.
///
/// Extensions allow you to store arbitrary data associated with a request.
pub type Extensions = http::Extensions;

/// Represents an incoming HTTP request.
///
/// The `Request` type provides access to:
/// - HTTP method and URI
/// - Headers
/// - Request body
/// - Path parameters (extracted from the route pattern)
/// - Query parameters (from the URL query string)
/// - Extensions (for storing custom data)
///
/// # Examples
///
/// ```no_run
/// use ruffus::{Request, Response};
///
/// async fn handler(req: Request) -> ruffus::Result<Response> {
///     let method = req.method();
///     let uri = req.uri();
///     let id = req.param("id");
///     
///     Ok(Response::text(format!("Method: {}, URI: {}", method, uri)))
/// }
/// ```
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
    /// Creates a new Request.
    ///
    /// This is typically used internally by the framework. Most users will
    /// receive `Request` objects as handler parameters.
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

    /// Returns the HTTP method of the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::Request;
    /// # async fn example(req: Request) {
    /// let method = req.method();
    /// println!("Method: {}", method);
    /// # }
    /// ```
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the request URI.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::Request;
    /// # async fn example(req: Request) {
    /// let uri = req.uri();
    /// println!("Path: {}", uri.path());
    /// # }
    /// ```
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Returns the request headers.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::Request;
    /// # async fn example(req: Request) {
    /// if let Some(content_type) = req.headers().get("content-type") {
    ///     println!("Content-Type: {:?}", content_type);
    /// }
    /// # }
    /// ```
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns a path parameter by name.
    ///
    /// Path parameters are extracted from the route pattern (e.g., `/users/:id`).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{App, Request, Response};
    /// # let mut app = App::new();
    /// app.get("/users/:id", |req: Request| async move {
    ///     let id = req.param("id").unwrap_or("unknown");
    ///     Ok(Response::text(format!("User ID: {}", id)))
    /// });
    /// ```
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(|s| s.as_str())
    }

    /// Returns a query parameter by name.
    ///
    /// Query parameters are extracted from the URL query string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{App, Request, Response};
    /// # let mut app = App::new();
    /// // For URL: /search?q=rust&limit=10
    /// app.get("/search", |req: Request| async move {
    ///     let query = req.query("q").unwrap_or("");
    ///     let limit = req.query("limit").unwrap_or("20");
    ///     Ok(Response::text(format!("Search: {}, Limit: {}", query, limit)))
    /// });
    /// ```
    pub fn query(&self, name: &str) -> Option<&str> {
        self.query.get(name).map(|s| s.as_str())
    }

    /// Returns all path parameters as a HashMap.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::Request;
    /// # async fn example(req: Request) {
    /// for (key, value) in req.params() {
    ///     println!("{}: {}", key, value);
    /// }
    /// # }
    /// ```
    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Returns all query parameters as a HashMap.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::Request;
    /// # async fn example(req: Request) {
    /// for (key, value) in req.query_params() {
    ///     println!("{}: {}", key, value);
    /// }
    /// # }
    /// ```
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query
    }

    /// Sets a path parameter (used internally by the router).
    ///
    /// This method is typically not called by user code.
    pub fn set_param(&mut self, name: String, value: String) {
        self.params.insert(name, value);
    }

    /// Deserializes the request body as JSON.
    ///
    /// # Errors
    ///
    /// Returns an error if the body is not valid JSON or cannot be deserialized
    /// into the target type.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{App, Request, Response};
    /// # use serde::Deserialize;
    /// #
    /// #[derive(Deserialize)]
    /// struct User {
    ///     name: String,
    ///     email: String,
    /// }
    ///
    /// # let mut app = App::new();
    /// app.post("/users", |mut req: Request| async move {
    ///     let user: User = req.json().await?;
    ///     Ok(Response::text(format!("Created user: {}", user.name)))
    /// });
    /// ```
    pub async fn json<T: DeserializeOwned>(&mut self) -> crate::Result<T> {
        let body_bytes = &self.body;
        serde_json::from_slice(body_bytes)
            .map_err(|e| crate::Error::JsonParseError(e))
    }

    /// Returns the request body as bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::Request;
    /// # async fn example(req: Request) {
    /// let body_bytes = req.body();
    /// println!("Body size: {} bytes", body_bytes.len());
    /// # }
    /// ```
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    /// Returns mutable access to request extensions.
    ///
    /// Extensions allow you to store arbitrary data with the request.
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }

    /// Returns access to request extensions.
    ///
    /// Extensions allow you to retrieve arbitrary data stored with the request.
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
