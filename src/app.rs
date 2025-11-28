//! Application type - the main entry point for Ruffus
//!
//! The [`App`] struct is the core of a Ruffus application. It manages routing,
//! middleware, and the HTTP server lifecycle.
//!
//! # Examples
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

use crate::{Error, Method, Middleware, Request, Response, Result, Router};
use std::future::Future;
use std::sync::Arc;

/// The main application struct that manages routing, middleware, and server lifecycle.
///
/// `App` provides methods for:
/// - Registering routes with HTTP methods (GET, POST, PUT, DELETE, PATCH)
/// - Adding global middleware
/// - Mounting routers with prefixes
/// - Starting the HTTP server
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use ruffus::{App, Request, Response};
///
/// #[tokio::main]
/// async fn main() {
///     let mut app = App::new();
///     
///     app.get("/hello", |_req: Request| async {
///         Ok(Response::text("Hello!".to_string()))
///     });
///     
///     app.listen("127.0.0.1:3000").await.unwrap();
/// }
/// ```
pub struct App {
    router: Router,
    middleware: Vec<Arc<dyn Middleware>>,
}

impl App {
    /// Creates a new Application instance with an empty router and middleware stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruffus::App;
    ///
    /// let app = App::new();
    /// ```
    pub fn new() -> Self {
        Self {
            router: Router::new(""),
            middleware: Vec::new(),
        }
    }

    /// Registers a GET route with the specified path and handler.
    ///
    /// # Arguments
    ///
    /// * `path` - The route pattern (e.g., "/users/:id")
    /// * `handler` - An async function that handles the request
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{App, Request, Response};
    /// # let mut app = App::new();
    /// app.get("/users", |_req: Request| async {
    ///     Ok(Response::text("List of users".to_string()))
    /// });
    /// ```
    pub fn get<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.get(path, handler);
        self
    }

    /// Registers a POST route with the specified path and handler.
    ///
    /// # Arguments
    ///
    /// * `path` - The route pattern (e.g., "/users")
    /// * `handler` - An async function that handles the request
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{App, Request, Response};
    /// # let mut app = App::new();
    /// app.post("/users", |mut req: Request| async move {
    ///     // Handle user creation
    ///     Ok(Response::text("User created".to_string()))
    /// });
    /// ```
    pub fn post<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.post(path, handler);
        self
    }

    /// Registers a PUT route with the specified path and handler.
    ///
    /// # Arguments
    ///
    /// * `path` - The route pattern (e.g., "/users/:id")
    /// * `handler` - An async function that handles the request
    pub fn put<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.put(path, handler);
        self
    }

    /// Registers a DELETE route with the specified path and handler.
    ///
    /// # Arguments
    ///
    /// * `path` - The route pattern (e.g., "/users/:id")
    /// * `handler` - An async function that handles the request
    pub fn delete<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.delete(path, handler);
        self
    }

    /// Registers a PATCH route with the specified path and handler.
    ///
    /// # Arguments
    ///
    /// * `path` - The route pattern (e.g., "/users/:id")
    /// * `handler` - An async function that handles the request
    pub fn patch<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.patch(path, handler);
        self
    }

    /// Adds global middleware that will be executed for all requests.
    ///
    /// Middleware is executed in the order it is registered.
    ///
    /// # Arguments
    ///
    /// * `middleware` - An Arc-wrapped middleware implementation
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{App, Middleware, Request, Response, Next};
    /// # use async_trait::async_trait;
    /// # use std::sync::Arc;
    /// #
    /// struct Logger;
    ///
    /// #[async_trait]
    /// impl Middleware for Logger {
    ///     async fn handle(&self, req: Request, next: Next) -> ruffus::Result<Response> {
    ///         println!("{} {}", req.method(), req.uri());
    ///         next.run(req).await
    ///     }
    /// }
    ///
    /// # let mut app = App::new();
    /// app.use_middleware(Arc::new(Logger));
    /// ```
    pub fn use_middleware(&mut self, middleware: Arc<dyn Middleware>) -> &mut Self {
        self.middleware.push(middleware);
        self
    }

    /// Mounts a router at the specified prefix.
    ///
    /// All routes from the mounted router will be prefixed with the given path.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The path prefix for all routes in the router
    /// * `router` - The router to mount
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::{App, Router, Request, Response};
    /// # let mut app = App::new();
    /// let mut api = Router::new("/v1");
    /// api.get("/users", |_req: Request| async {
    ///     Ok(Response::text("Users".to_string()))
    /// });
    ///
    /// app.mount("/api", api);
    /// // Route is now available at /api/v1/users
    /// ```
    pub fn mount(&mut self, prefix: &str, router: Router) -> &mut Self {
        self.router.mount(prefix, router);
        self
    }

    /// Handles an incoming request through the middleware pipeline and routing.
    ///
    /// This method:
    /// 1. Finds a matching route for the request
    /// 2. Extracts path parameters
    /// 3. Executes the middleware stack
    /// 4. Invokes the route handler
    ///
    /// Returns a 404 error if no route matches, or a 405 error if the path exists
    /// but the HTTP method doesn't match.
    pub async fn handle_request(&self, mut req: Request) -> Result<Response> {
        use crate::middleware::{Next};
        
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
                // Create a handler that will execute the route
                // We need to clone the handler function from the route
                let handler_fn = route.handler_fn();
                let handler = Arc::new(move |req: Request| {
                    handler_fn(req)
                });
                
                // Execute middleware stack with the handler
                let next = Next::new(self.middleware.clone(), Some(handler));
                next.run(req).await
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

    /// Starts the HTTP server and listens for incoming connections.
    ///
    /// This method consumes the `App` and runs indefinitely, handling requests
    /// as they arrive. Each connection is handled in a separate Tokio task.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to bind to (e.g., "127.0.0.1:3000")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The address is invalid
    /// - The server fails to bind to the address
    /// - A connection handling error occurs
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ruffus::App;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let app = App::new();
    /// app.listen("127.0.0.1:3000").await.unwrap();
    /// # }
    /// ```
    pub async fn listen(self, addr: &str) -> Result<()> {
        use hyper::server::conn::http1;
        use hyper::service::service_fn;
        use hyper_util::rt::TokioIo;
        use tokio::net::TcpListener;

        // Parse the address
        let addr = addr.parse::<std::net::SocketAddr>()
            .map_err(|e| Error::InternalServerError(format!("Invalid address: {}", e)))?;

        // Bind to the address
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| Error::InternalServerError(format!("Failed to bind: {}", e)))?;

        println!("Ruffus server listening on http://{}", addr);

        // Wrap self in Arc for sharing across connections
        let app = Arc::new(self);

        // Accept connections in a loop
        loop {
            let (stream, _) = listener.accept()
                .await
                .map_err(|e| Error::InternalServerError(format!("Failed to accept connection: {}", e)))?;

            let io = TokioIo::new(stream);
            let app_clone = app.clone();

            // Spawn a task to handle this connection
            tokio::spawn(async move {
                // Create a service function that handles requests
                let service = service_fn(move |hyper_req: hyper::Request<hyper::body::Incoming>| {
                    let app = app_clone.clone();
                    async move {
                        // Convert hyper request to our Request type
                        let req = match Request::from_hyper(hyper_req).await {
                            Ok(req) => req,
                            Err(e) => {
                                // Return error response
                                let response: hyper::Response<http_body_util::Full<bytes::Bytes>> = 
                                    e.into_response().into();
                                return Ok::<_, hyper::Error>(response);
                            }
                        };

                        // Handle the request through our pipeline
                        let response = match app.handle_request(req).await {
                            Ok(resp) => resp,
                            Err(e) => e.into_response(),
                        };

                        // Convert our Response to hyper Response
                        let hyper_response: hyper::Response<http_body_util::Full<bytes::Bytes>> = 
                            response.into();
                        
                        Ok::<_, hyper::Error>(hyper_response)
                    }
                });

                // Serve the connection
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
