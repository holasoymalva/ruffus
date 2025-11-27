//! Router for organizing routes with common prefixes

use crate::Middleware;

/// Router for grouping routes with a common prefix
pub struct Router {
    prefix: String,
    middleware: Vec<Box<dyn Middleware>>,
}

impl Router {
    /// Create a new Router with the given prefix
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            middleware: Vec::new(),
        }
    }
}
