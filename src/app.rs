//! Application type - the main entry point for Ruffus

use crate::{Error, Method, Middleware, Request, Response, Result, Router};
use std::future::Future;
use std::sync::Arc;

/// The main application struct
pub struct App {
    router: Router,
    middleware: Vec<Arc<dyn Middleware>>,
}

impl App {
    /// Create a new Application instance
    pub fn new() -> Self {
        Self {
            router: Router::new(""),
            middleware: Vec::new(),
        }
    }

    /// Register a GET route
    pub fn get<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.get(path, handler);
        self
    }

    /// Register a POST route
    pub fn post<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.post(path, handler);
        self
    }

    /// Register a PUT route
    pub fn put<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.put(path, handler);
        self
    }

    /// Register a DELETE route
    pub fn delete<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.delete(path, handler);
        self
    }

    /// Register a PATCH route
    pub fn patch<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.patch(path, handler);
        self
    }

    /// Add global middleware
    pub fn use_middleware(&mut self, middleware: Arc<dyn Middleware>) -> &mut Self {
        self.middleware.push(middleware);
        self
    }

    /// Mount a router at a specific prefix
    pub fn mount(&mut self, prefix: &str, router: Router) -> &mut Self {
        self.router.mount(prefix, router);
        self
    }

    /// Handle an incoming request through the middleware pipeline and routing
    pub async fn handle_request(&self, mut req: Request) -> Result<Response> {
        let method = Method::from(req.method().clone());
        let path = req.uri().path().to_string();

        // Try to find a matching route
        if let Some((route, params)) = self.router.find_route(&method, &path) {
            // Set path parameters in the request
            for (key, value) in params {
                req.set_param(key, value);
            }

            // Execute middleware stack with the route handler
            if self.middleware.is_empty() {
                // No middleware, execute handler directly
                route.handle(req).await
            } else {
                // Create handler for this route
                let response = route.handle(req).await;
                
                // For now, we'll execute middleware without the route handler
                // This is a simplified implementation - full middleware integration
                // will be done in task 10
                response
            }
        } else {
            // Check if path exists with different method
            if self.router.path_exists(&path) {
                let allowed = self.router.allowed_methods(&path);
                let allowed_http: Vec<http::Method> = allowed.into_iter().map(|m| m.into()).collect();
                Err(Error::MethodNotAllowed(allowed_http))
            } else {
                Err(Error::RouteNotFound)
            }
        }
    }

    /// Get the internal router (for testing)
    pub fn router(&self) -> &Router {
        &self.router
    }

    /// Get mutable access to the internal router
    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }

    /// Get the middleware stack (for testing)
    pub fn middleware(&self) -> &[Arc<dyn Middleware>] {
        &self.middleware
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
