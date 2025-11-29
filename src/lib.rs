//! Ruffus - Fast, minimalist web framework for Rust
//!
//! Ruffus is a web framework inspired by Express.js that provides a simple,
//! ergonomic API for building web applications in Rust.
//!
//! # Features
//!
//! - **Express-like API**: Familiar, intuitive routing and middleware system
//! - **Type-safe extractors**: Extract path parameters, query strings, and JSON bodies with compile-time safety
//! - **Async/await support**: Built on Tokio and Hyper for high-performance async I/O
//! - **Flexible middleware**: Composable middleware for cross-cutting concerns
//! - **Router mounting**: Organize routes with prefixes and nested routers
//!
//! # Quick Start
//!
//! ```no_run
//! use ruffus::{App, Request, Response};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut app = App::new();
//!     
//!     app.get("/", |_req: Request| async {
//!         Ok(Response::text("Hello, World!".to_string()))
//!     });
//!     
//!     app.listen("127.0.0.1:3000").await.unwrap();
//! }
//! ```
//!
//! # Path Parameters
//!
//! Extract dynamic segments from URLs:
//!
//! ```no_run
//! # use ruffus::{App, Request, Response};
//! # #[tokio::main]
//! # async fn main() {
//! # let mut app = App::new();
//! app.get("/users/:id", |req: Request| async move {
//!     let id = req.param("id").unwrap();
//!     Ok(Response::text(format!("User ID: {}", id)))
//! });
//! # }
//! ```
//!
//! # JSON Handling
//!
//! Automatically serialize and deserialize JSON:
//!
//! ```no_run
//! # use ruffus::{App, Request, Response};
//! # use serde::{Deserialize, Serialize};
//! #
//! #[derive(Deserialize)]
//! struct CreateUser {
//!     name: String,
//!     email: String,
//! }
//!
//! #[derive(Serialize)]
//! struct User {
//!     id: u64,
//!     name: String,
//!     email: String,
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let mut app = App::new();
//! app.post("/users", |mut req: Request| async move {
//!     let body: CreateUser = req.json().await?;
//!     let user = User {
//!         id: 1,
//!         name: body.name,
//!         email: body.email,
//!     };
//!     Response::json(&user)
//! });
//! # }
//! ```
//!
//! # Middleware
//!
//! Add cross-cutting functionality with middleware:
//!
//! ```no_run
//! # use ruffus::{App, Request, Response, Middleware, Next};
//! # use async_trait::async_trait;
//! # use std::sync::Arc;
//! #
//! struct Logger;
//!
//! #[async_trait]
//! impl Middleware for Logger {
//!     async fn handle(&self, req: Request, next: Next) -> ruffus::Result<Response> {
//!         println!("{} {}", req.method(), req.uri());
//!         next.run(req).await
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let mut app = App::new();
//! app.use_middleware(Arc::new(Logger));
//! # }
//! ```
//!
//! # Routers
//!
//! Organize routes with common prefixes:
//!
//! ```no_run
//! # use ruffus::{App, Router, Request, Response};
//! # #[tokio::main]
//! # async fn main() {
//! let mut api = Router::new("/api");
//!
//! api.get("/users", |_req: Request| async {
//!     Ok(Response::json(&serde_json::json!({"users": []}))?)
//! });
//!
//! api.post("/users", |mut req: Request| async move {
//!     // Handle user creation
//!     Ok(Response::json(&serde_json::json!({"status": "created"}))?)
//! });
//!
//! let mut app = App::new();
//! app.mount("/", api);
//! // Routes are now available at /api/users
//! # }
//! ```

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
