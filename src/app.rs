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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
