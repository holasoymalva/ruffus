//! HTTP Response type
//!
//! This module provides the [`Response`] type for building HTTP responses.

use bytes::Bytes;
use http::{HeaderMap, StatusCode};
use serde::Serialize;

/// Represents an outgoing HTTP response.
///
/// The `Response` type provides a builder-style API for constructing HTTP responses
/// with status codes, headers, and bodies.
///
/// # Examples
///
/// ```
/// use ruffus::Response;
///
/// // Plain text response
/// let response = Response::text("Hello, World!".to_string());
///
/// // JSON response
/// let json_response = Response::json(&serde_json::json!({
///     "message": "Success"
/// })).unwrap();
///
/// // Custom response with status and headers
/// let custom = Response::new()
///     .status(http::StatusCode::CREATED)
///     .header("X-Custom-Header", "value")
///     .body("Created".to_string());
/// ```
pub struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl Response {
    /// Creates a new empty Response with status 200 OK.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::new();
    /// ```
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    /// Sets the HTTP status code.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    /// use http::StatusCode;
    ///
    /// let response = Response::new()
    ///     .status(StatusCode::CREATED)
    ///     .body("Created".to_string());
    /// ```
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Adds a header to the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::new()
    ///     .header("Content-Type", "text/plain")
    ///     .header("X-Custom", "value");
    /// ```
    pub fn header(mut self, key: &str, value: &str) -> Self {
        if let (Ok(name), Ok(val)) = (
            http::header::HeaderName::from_bytes(key.as_bytes()),
            http::header::HeaderValue::from_str(value),
        ) {
            self.headers.insert(name, val);
        }
        self
    }

    /// Creates a plain text response with status 200 OK.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::text("Hello, World!".to_string());
    /// ```
    pub fn text(text: String) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::from(text),
        }
    }

    /// Creates a JSON response from a serializable value.
    ///
    /// Automatically sets the `Content-Type` header to `application/json`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value cannot be serialized to JSON.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     id: u64,
    ///     name: String,
    /// }
    ///
    /// let user = User { id: 1, name: "Alice".to_string() };
    /// let response = Response::json(&user).unwrap();
    /// ```
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

    /// Sets the response body from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::new()
    ///     .body("Hello, World!".to_string());
    /// ```
    pub fn body(mut self, body: String) -> Self {
        self.body = Bytes::from(body);
        self
    }

    /// Sets the response body from bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    /// use bytes::Bytes;
    ///
    /// let response = Response::new()
    ///     .body_bytes(Bytes::from("Hello"));
    /// ```
    pub fn body_bytes(mut self, body: Bytes) -> Self {
        self.body = body;
        self
    }

    /// Creates an HTML response with the appropriate Content-Type header.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::html("<h1>Hello</h1>".to_string());
    /// ```
    pub fn html(html: String) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::from(html),
        }
        .header("Content-Type", "text/html; charset=utf-8")
    }

    /// Creates a 404 Not Found response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::not_found();
    /// ```
    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            headers: HeaderMap::new(),
            body: Bytes::from("Not Found"),
        }
    }

    /// Creates a 400 Bad Request response with a custom message.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::bad_request("Invalid input".to_string());
    /// ```
    pub fn bad_request(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            headers: HeaderMap::new(),
            body: Bytes::from(message),
        }
    }

    /// Creates a 500 Internal Server Error response with a custom message.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::internal_error("Database error".to_string());
    /// ```
    pub fn internal_error(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers: HeaderMap::new(),
            body: Bytes::from(message),
        }
    }

    /// Creates a 302 redirect response to the specified location.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::redirect("/login");
    /// ```
    pub fn redirect(location: &str) -> Self {
        Self {
            status: StatusCode::FOUND,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
        .header("Location", location)
    }

    /// Creates a 204 No Content response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::no_content();
    /// ```
    pub fn no_content() -> Self {
        Self {
            status: StatusCode::NO_CONTENT,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    /// Returns the HTTP status code of the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    /// use http::StatusCode;
    ///
    /// let response = Response::new().status(StatusCode::CREATED);
    /// assert_eq!(response.get_status(), StatusCode::CREATED);
    /// ```
    pub fn get_status(&self) -> StatusCode {
        self.status
    }

    /// Returns the response headers.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::new().header("X-Custom", "value");
    /// let headers = response.get_headers();
    /// ```
    pub fn get_headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns the response body as bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Response;
    ///
    /// let response = Response::text("Hello".to_string());
    /// let body = response.get_body();
    /// ```
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
