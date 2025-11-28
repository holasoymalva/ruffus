//! Ruffus - Fast, minimalist web framework for Rust
//!
//! Ruffus is a web framework inspired by Express.js that provides a simple,
//! ergonomic API for building web applications in Rust.

pub mod app;
pub mod error;
pub mod extractors;
pub mod method;
pub mod middleware;
pub mod request;
pub mod response;
pub mod router;

// Re-export main types for convenience
pub use app::App;
pub use error::Error;
pub use extractors::{FromRequest, Json, Path, Query};
pub use method::Method;
pub use middleware::{Handler, Middleware, Next};
pub use request::Request;
pub use response::Response;
pub use router::{PathPattern, Route, Router, Segment};

pub type Result<T> = std::result::Result<T, Error>;
