//! Application type - the main entry point for Ruffus

use crate::{Router, Middleware, Result};

/// The main application struct
pub struct App {
    router: Router,
    middleware: Vec<Box<dyn Middleware>>,
}

impl App {
    /// Create a new Application instance
    pub fn new() -> Self {
        Self {
            router: Router::new(""),
            middleware: Vec::new(),
        }
    }

    /// Mount a router at a specific prefix
    pub fn mount(&mut self, prefix: &str, router: Router) -> &mut Self {
        self.router.mount(prefix, router);
        self
    }

    /// Get the internal router (for testing)
    pub fn router(&self) -> &Router {
        &self.router
    }

    /// Get mutable access to the internal router
    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
