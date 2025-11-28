//! Middleware trait and types

use crate::{Request, Response, Result};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for boxed handler functions
pub type BoxedHandler = Arc<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        + Send
        + Sync
        + 'static,
>;

/// Trait for request handlers
pub trait Handler: Send + Sync + 'static {
    /// Handle a request and return a response
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

/// Trait for middleware that can process requests
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    /// Handle a request and optionally pass it to the next middleware
    async fn handle(&self, req: Request, next: Next) -> Result<Response>;
}

/// Represents the next middleware or handler in the chain
pub struct Next {
    middleware_stack: Vec<Arc<dyn Middleware>>,
    handler: Option<BoxedHandler>,
    index: usize,
}

impl Next {
    /// Create a new Next with a middleware stack and final handler
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

    /// Continue to the next middleware or handler
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

/// Execute a middleware stack with a final handler
pub async fn execute_middleware_stack(
    middleware: Vec<Arc<dyn Middleware>>,
    handler: BoxedHandler,
    req: Request,
) -> Result<Response> {
    let next = Next::new(middleware, Some(handler));
    next.run(req).await
}
