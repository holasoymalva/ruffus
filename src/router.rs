//! Router for organizing routes with common prefixes

use crate::{Method, Middleware, Request, Response, Result};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Represents a segment in a path pattern
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    /// Static path segment (e.g., "users")
    Static(String),
    /// Dynamic path parameter (e.g., ":id")
    Dynamic(String),
}

/// Represents a parsed path pattern
#[derive(Debug, Clone)]
pub struct PathPattern {
    segments: Vec<Segment>,
    raw: String,
}

impl PathPattern {
    /// Parse a path pattern string into segments
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

    /// Check if a path matches this pattern and extract parameters
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

    /// Get the raw pattern string
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Get the segments
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

/// Type alias for handler functions
pub type HandlerFn = Box<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        + Send
        + Sync
        + 'static,
>;

/// Represents a single route with method, pattern, and handler
pub struct Route {
    method: Method,
    pattern: PathPattern,
    handler: HandlerFn,
}

impl Route {
    /// Create a new route
    pub fn new<F, Fut>(method: Method, pattern: &str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let handler_fn = Box::new(move |req: Request| {
            Box::pin(handler(req)) as Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        });

        Self {
            method,
            pattern: PathPattern::parse(pattern),
            handler: handler_fn,
        }
    }

    /// Get the method
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get the pattern
    pub fn pattern(&self) -> &PathPattern {
        &self.pattern
    }

    /// Check if this route matches the request
    pub fn matches(&self, method: &Method, path: &str) -> Option<HashMap<String, String>> {
        if self.method == *method {
            self.pattern.matches(path)
        } else {
            None
        }
    }

    /// Execute the handler with the given request
    pub async fn handle(&self, req: Request) -> Result<Response> {
        (self.handler)(req).await
    }
}

/// Router for grouping routes with a common prefix
pub struct Router {
    prefix: String,
    routes: Vec<Route>,
    middleware: Vec<std::sync::Arc<dyn Middleware>>,
}

impl Router {
    /// Create a new Router with the given prefix
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    /// Register a GET route
    pub fn get<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::GET, &full_path, handler));
        self
    }

    /// Register a POST route
    pub fn post<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::POST, &full_path, handler));
        self
    }

    /// Register a PUT route
    pub fn put<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::PUT, &full_path, handler));
        self
    }

    /// Register a DELETE route
    pub fn delete<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::DELETE, &full_path, handler));
        self
    }

    /// Register a PATCH route
    pub fn patch<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        self.routes.push(Route::new(Method::PATCH, &full_path, handler));
        self
    }

    /// Add middleware to this router
    pub fn use_middleware(&mut self, middleware: std::sync::Arc<dyn Middleware>) -> &mut Self {
        self.middleware.push(middleware);
        self
    }

    /// Get all routes
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    /// Find a matching route for the given method and path
    pub fn find_route(&self, method: &Method, path: &str) -> Option<(&Route, HashMap<String, String>)> {
        for route in &self.routes {
            if let Some(params) = route.matches(method, path) {
                return Some((route, params));
            }
        }
        None
    }

    /// Check if any route matches the path (regardless of method)
    pub fn path_exists(&self, path: &str) -> bool {
        self.routes.iter().any(|route| {
            route.pattern.matches(path).is_some()
        })
    }

    /// Get allowed methods for a path
    pub fn allowed_methods(&self, path: &str) -> Vec<Method> {
        self.routes
            .iter()
            .filter(|route| route.pattern.matches(path).is_some())
            .map(|route| *route.method())
            .collect()
    }

    /// Get the prefix of this router
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get the middleware stack
    pub fn middleware(&self) -> &[std::sync::Arc<dyn Middleware>] {
        &self.middleware
    }

    /// Collect all routes with their full paths (for mounting)
    pub fn collect_routes(self) -> Vec<Route> {
        self.routes
    }

    /// Mount another router by merging its routes
    /// The mounted router's routes will have the mount prefix prepended
    /// The mounting router's own prefix is also prepended to all routes
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
