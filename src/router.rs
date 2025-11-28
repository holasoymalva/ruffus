//! Router for organizing routes with common prefixes
//!
//! The router module provides types for organizing routes into logical groups
//! with shared prefixes and middleware.
//!
//! # Examples
//!
//! ```no_run
//! use ruffus::{Router, Request, Response};
//!
//! let mut api = Router::new("/api");
//!
//! api.get("/users", |_req: Request| async {
//!     Ok(Response::text("Users list".to_string()))
//! });
//!
//! api.post("/users", |mut req: Request| async move {
//!     Ok(Response::text("User created".to_string()))
//! });
//! ```

use crate::{Method, Middleware, Request, Response, Result};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Represents a segment in a path pattern.
///
/// Path patterns are composed of static segments (literal strings) and
/// dynamic segments (parameters that can match any value).
///
/// # Examples
///
/// - `/users/123` contains two static segments: "users" and "123"
/// - `/users/:id` contains one static segment "users" and one dynamic segment ":id"
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    /// Static path segment (e.g., "users")
    Static(String),
    /// Dynamic path parameter (e.g., ":id")
    Dynamic(String),
}

/// Represents a parsed path pattern with static and dynamic segments.
///
/// Path patterns support dynamic parameters using the `:param` syntax.
///
/// # Examples
///
/// ```
/// use ruffus::PathPattern;
///
/// let pattern = PathPattern::parse("/users/:id/posts/:post_id");
/// // This pattern will match paths like "/users/123/posts/456"
/// ```
#[derive(Debug, Clone)]
pub struct PathPattern {
    segments: Vec<Segment>,
    raw: String,
}

impl PathPattern {
    /// Parses a path pattern string into segments.
    ///
    /// Segments starting with `:` are treated as dynamic parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::PathPattern;
    ///
    /// let pattern = PathPattern::parse("/users/:id");
    /// ```
    pub fn parse(pattern: &str) -> Self {
        let segments = pattern
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|segment| {
                if segment.starts_with(':') {
                    Segment::Dynamic(segment[1..].to_string())
                } else {
                    Segment::Static(segment.to_string())
                }
            })
            .collect();

        Self {
            segments,
            raw: pattern.to_string(),
        }
    }

    /// Checks if a path matches this pattern and extracts parameter values.
    ///
    /// Returns `Some(params)` if the path matches, where `params` contains
    /// the extracted parameter values. Returns `None` if the path doesn't match.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::PathPattern;
    ///
    /// let pattern = PathPattern::parse("/users/:id");
    /// let params = pattern.matches("/users/123").unwrap();
    /// assert_eq!(params.get("id"), Some(&"123".to_string()));
    /// ```
    pub fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        let path_segments: Vec<&str> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Must have same number of segments
        if path_segments.len() != self.segments.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (pattern_seg, path_seg) in self.segments.iter().zip(path_segments.iter()) {
            match pattern_seg {
                Segment::Static(expected) => {
                    if expected != path_seg {
                        return None;
                    }
                }
                Segment::Dynamic(param_name) => {
                    // URL decode the parameter value
                    let decoded = urlencoding::decode(path_seg)
                        .unwrap_or_else(|_| std::borrow::Cow::Borrowed(*path_seg));
                    params.insert(param_name.clone(), decoded.into_owned());
                }
            }
        }

        Some(params)
    }

    /// Returns the raw pattern string.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::PathPattern;
    ///
    /// let pattern = PathPattern::parse("/users/:id");
    /// assert_eq!(pattern.raw(), "/users/:id");
    /// ```
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the parsed segments of the pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::{PathPattern, Segment};
    ///
    /// let pattern = PathPattern::parse("/users/:id");
    /// let segments = pattern.segments();
    /// assert_eq!(segments.len(), 2);
    /// ```
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

/// Type alias for handler functions.
///
/// Handlers are async functions that take a `Request` and return a `Result<Response>`.
pub type HandlerFn = std::sync::Arc<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        + Send
        + Sync
        + 'static,
>;

/// Represents a single route with an HTTP method, path pattern, and handler.
///
/// Routes are typically created through the `App` or `Router` methods
/// (e.g., `get()`, `post()`) rather than directly.
pub struct Route {
    method: Method,
    pattern: PathPattern,
    handler: HandlerFn,
}

impl Route {
    /// Creates a new route with the specified method, pattern, and handler.
    ///
    /// This is typically used internally by the framework.
    pub fn new<F, Fut>(method: Method, pattern: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let handler_fn = std::sync::Arc::new(move |req: Request| {
            Box::pin(handler(req)) as Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        });

        Self {
            method,
            pattern: PathPattern::parse(pattern),
            handler: handler_fn,
        }
    }

    /// Returns the HTTP method for this route.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the path pattern for this route.
    pub fn pattern(&self) -> &PathPattern {
        &self.pattern
    }

    /// Checks if this route matches the given method and path.
    ///
    /// Returns extracted parameters if the route matches, or `None` otherwise.
    pub fn matches(&self, method: &Method, path: &str) -> Option<HashMap<String, String>> {
        if self.method == *method {
            self.pattern.matches(path)
        } else {
            None
        }
    }

    /// Executes the route handler with the given request.
    pub async fn handle(&self, req: Request) -> Result<Response> {
        (self.handler)(req).await
    }
    
    /// Returns a clone of the handler function.
    ///
    /// This is used internally by the framework.
    pub fn handler_fn(&self) -> HandlerFn {
        self.handler.clone()
    }
}

/// Router for grouping routes with a common prefix.
///
/// Routers allow you to organize related routes together and apply
/// middleware to specific groups of routes.
///
/// # Examples
///
/// ```no_run
/// use ruffus::{Router, Request, Response};
///
/// let mut api = Router::new("/api");
///
/// api.get("/users", |_req: Request| async {
///     Ok(Response::text("Users".to_string()))
/// });
///
/// api.get("/posts", |_req: Request| async {
///     Ok(Response::text("Posts".to_string()))
/// });
/// ```
pub struct Router {
    prefix: String,
    routes: Vec<Route>,
    middleware: Vec<std::sync::Arc<dyn Middleware>>,
}

impl Router {
    /// Creates a new Router with the given prefix.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The path prefix for all routes in this router
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::Router;
    ///
    /// let router = Router::new("/api");
    /// ```
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    /// Registers a GET route on this router.
    ///
    /// The route path will be prefixed with the router's prefix.
    pub fn get<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::GET, &full_path, handler));
        self
    }

    /// Registers a POST route on this router.
    ///
    /// The route path will be prefixed with the router's prefix.
    pub fn post<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::POST, &full_path, handler));
        self
    }

    /// Registers a PUT route on this router.
    ///
    /// The route path will be prefixed with the router's prefix.
    pub fn put<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::PUT, &full_path, handler));
        self
    }

    /// Registers a DELETE route on this router.
    ///
    /// The route path will be prefixed with the router's prefix.
    pub fn delete<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::DELETE, &full_path, handler));
        self
    }

    /// Registers a PATCH route on this router.
    ///
    /// The route path will be prefixed with the router's prefix.
    pub fn patch<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::PATCH, &full_path, handler));
        self
    }

    /// Adds middleware to this router.
    ///
    /// The middleware will only apply to routes registered on this router.
    pub fn use_middleware(&mut self, middleware: std::sync::Arc<dyn Middleware>) -> &mut Self {
        self.middleware.push(middleware);
        self
    }

    /// Returns all routes registered on this router.
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    /// Finds a matching route for the given method and path.
    ///
    /// Returns the route and extracted parameters if a match is found.
    pub fn find_route(&self, method: &Method, path: &str) -> Option<(&Route, HashMap<String, String>)> {
        for route in &self.routes {
            if let Some(params) = route.matches(method, path) {
                return Some((route, params));
            }
        }
        None
    }

    /// Checks if any route matches the path (regardless of HTTP method).
    pub fn path_exists(&self, path: &str) -> bool {
        self.routes.iter().any(|route| {
            route.pattern.matches(path).is_some()
        })
    }

    /// Returns the allowed HTTP methods for a given path.
    pub fn allowed_methods(&self, path: &str) -> Vec<Method> {
        self.routes
            .iter()
            .filter(|route| route.pattern.matches(path).is_some())
            .map(|route| *route.method())
            .collect()
    }

    /// Returns the prefix of this router.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Returns the middleware stack for this router.
    pub fn middleware(&self) -> &[std::sync::Arc<dyn Middleware>] {
        &self.middleware
    }

    /// Collects all routes with their full paths.
    ///
    /// This is used internally when mounting routers.
    pub fn collect_routes(self) -> Vec<Route> {
        self.routes
    }

    /// Mounts another router by merging its routes.
    ///
    /// The mounted router's routes will have the mount prefix prepended.
    /// The mounting router's own prefix is also prepended to all routes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{Router, Request, Response};
    /// let mut main_router = Router::new("/api");
    /// let mut sub_router = Router::new("/v1");
    ///
    /// sub_router.get("/users", |_req: Request| async {
    ///     Ok(Response::text("Users".to_string()))
    /// });
    ///
    /// main_router.mount("", sub_router);
    /// // Route is now at /api/v1/users
    /// ```
    pub fn mount(&mut self, mount_prefix: &str, mut router: Router) -> &mut Self {
        // Add each route with both the router's prefix and mount prefix prepended
        for route in router.routes.drain(..) {
            // Combine: self.prefix + mount_prefix + existing route pattern
            let combined_prefix = if self.prefix.is_empty() && mount_prefix.is_empty() {
                String::new()
            } else if self.prefix.is_empty() {
                mount_prefix.to_string()
            } else if mount_prefix.is_empty() {
                self.prefix.clone()
            } else {
                format!("{}{}", self.prefix, mount_prefix)
            };
            
            let new_pattern = if combined_prefix.is_empty() {
                route.pattern.raw().to_string()
            } else {
                format!("{}{}", combined_prefix, route.pattern.raw())
            };
            
            // Create a new route with the updated pattern
            let new_route = Route {
                method: route.method,
                pattern: PathPattern::parse(&new_pattern),
                handler: route.handler,
            };
            
            self.routes.push(new_route);
        }
        
        // Also merge middleware from the mounted router
        for middleware in router.middleware.drain(..) {
            self.middleware.push(middleware);
        }
        
        self
    }
}
