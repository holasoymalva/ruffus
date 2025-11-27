//! Middleware trait and types

use crate::{Request, Response, Result};
use async_trait::async_trait;

/// Trait for middleware that can process requests
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    /// Handle a request and optionally pass it to the next middleware
    async fn handle(&self, req: Request, next: Next) -> Result<Response>;
}

/// Represents the next middleware in the chain
pub struct Next {
    // Internal state will be added in later tasks
}

impl Next {
    /// Continue to the next middleware or handler
    pub async fn run(self, req: Request) -> Result<Response> {
        // Implementation will be added in later tasks
        Ok(Response::new())
    }
}
