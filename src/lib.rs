//! Ruffus - Fast, minimalist web framework for Rust
//!
//! Ruffus is a web framework inspired by Express.js that provides a simple,
//! ergonomic API for building web applications in Rust.

pub mod app;
pub mod error;
pub mod middleware;
pub mod request;
pub mod response;
pub mod router;

// Re-export main types for convenience
pub use app::App;
pub use error::Error;
pub use middleware::{Middleware, Next};
pub use request::Request;
pub use response::Response;
pub use router::Router;

pub type Result<T> = std::result::Result<T, Error>;
