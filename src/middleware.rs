//! Middleware trait and types
//!
//! Middleware allows you to add cross-cutting functionality to your application,
//! such as logging, authentication, or request/response modification.
//!
//! # Examples
//!
//! ```no_run
//! use ruffus::{Middleware, Request, Response, Next};
//! use async_trait::async_trait;
//!
//! struct Logger;
//!
//! #[async_trait]
//! impl Middleware for Logger {
//!     async fn handle(&self, req: Request, next: Next) -> ruffus::Result<Response> {
//!         println!("{} {}", req.method(), req.uri());
//!         next.run(req).await
//!     }
//! }
//! ```

use crate::{Request, Response, Result};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for boxed handler functions.
pub type BoxedHandler = Arc<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        + Send
        + Sync
        + 'static,
>;

/// Trait for request handlers.
///
/// Handlers are async functions that process requests and return responses.
/// This trait is automatically implemented for async closures and functions.
pub trait Handler: Send + Sync + 'static {
    /// Handles a request and returns a response.
    ///
    /// This method is automatically implemented for async closures and functions.
    fn handle(&self, req: Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send + 'static>>;
}

/// Implement Handler for async closures and functions
impl<F, Fut> Handler for F
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response>> + Send + 'static,
{
    fn handle(&self, req: Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send + 'static>> {
        Box::pin(self(req))
    }
}

/// Trait for middleware that can process requests.
///
/// Middleware can:
/// - Inspect and modify requests before they reach handlers
/// - Inspect and modify responses before they're sent to clients
/// - Short-circuit the request pipeline by returning early
/// - Pass control to the next middleware or handler using `next.run()`
///
/// # Examples
///
/// ```no_run
/// use ruffus::{Middleware, Request, Response, Next};
/// use async_trait::async_trait;
///
/// struct AuthMiddleware;
///
/// #[async_trait]
/// impl Middleware for AuthMiddleware {
///     async fn handle(&self, req: Request, next: Next) -> ruffus::Result<Response> {
///         // Check authentication
///         if req.headers().get("authorization").is_none() {
///             return Ok(Response::new()
///                 .status(http::StatusCode::UNAUTHORIZED)
///                 .body("Unauthorized".to_string()));
///         }
///         
///         // Continue to next middleware or handler
///         next.run(req).await
///     }
/// }
/// ```
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    /// Handles a request and optionally passes it to the next middleware.
    ///
    /// Call `next.run(req)` to continue to the next middleware or handler.
    async fn handle(&self, req: Request, next: Next) -> Result<Response>;
}

/// Represents the next middleware or handler in the chain.
///
/// Call `next.run(req)` to pass control to the next middleware or handler.
pub struct Next {
    middleware_stack: Vec<Arc<dyn Middleware>>,
    handler: Option<BoxedHandler>,
    index: usize,
}

impl Next {
    /// Creates a new Next with a middleware stack and final handler.
    ///
    /// This is used internally by the framework.
    pub fn new(
        middleware_stack: Vec<Arc<dyn Middleware>>,
        handler: Option<BoxedHandler>,
    ) -> Self {
        Self {
            middleware_stack,
            handler,
            index: 0,
        }
    }

    /// Create a Next at a specific index in the middleware stack
    fn at_index(
        middleware_stack: Vec<Arc<dyn Middleware>>,
        handler: Option<BoxedHandler>,
        index: usize,
    ) -> Self {
        Self {
            middleware_stack,
            handler,
            index,
        }
    }

    /// Continues execution to the next middleware or handler in the chain.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{Middleware, Request, Response, Next};
    /// # use async_trait::async_trait;
    /// #
    /// # struct MyMiddleware;
    /// #
    /// # #[async_trait]
    /// # impl Middleware for MyMiddleware {
    /// async fn handle(&self, req: Request, next: Next) -> ruffus::Result<Response> {
    ///     // Do something before
    ///     let response = next.run(req).await?;
    ///     // Do something after
    ///     Ok(response)
    /// }
    /// # }
    /// ```
    pub async fn run(self, req: Request) -> Result<Response> {
        if self.index < self.middleware_stack.len() {
            // Execute the next middleware
            let middleware = self.middleware_stack[self.index].clone();
            let next = Next::at_index(
                self.middleware_stack,
                self.handler,
                self.index + 1,
            );
            middleware.handle(req, next).await
        } else if let Some(handler) = self.handler {
            // Execute the final handler
            handler(req).await
        } else {
            // No handler available
            Err(crate::Error::RouteNotFound)
        }
    }
}

/// Executes a middleware stack with a final handler.
///
/// This is used internally by the framework to process requests through
/// the middleware pipeline.
pub async fn execute_middleware_stack(
    middleware: Vec<Arc<dyn Middleware>>,
    handler: BoxedHandler,
    req: Request,
) -> Result<Response> {
    let next = Next::new(middleware, Some(handler));
    next.run(req).await
}
