//! Application type - the main entry point for Ruffus

use crate::{Error, Method, Middleware, Request, Response, Result, Router};
use std::future::Future;
use std::sync::Arc;

/// The main application struct
pub struct App {
    router: Router,
    middleware: Vec<Arc<dyn Middleware>>,
}

impl App {
    /// Create a new Application instance
    pub fn new() -> Self {
        Self {
            router: Router::new(""),
            middleware: Vec::new(),
        }
    }

    /// Register a GET route
    pub fn get<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.get(path, handler);
        self
    }

    /// Register a POST route
    pub fn post<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.post(path, handler);
        self
    }

    /// Register a PUT route
    pub fn put<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.put(path, handler);
        self
    }

    /// Register a DELETE route
    pub fn delete<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.delete(path, handler);
        self
    }

    /// Register a PATCH route
    pub fn patch<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        self.router.patch(path, handler);
        self
    }

    /// Add global middleware
    pub fn use_middleware(&mut self, middleware: Arc<dyn Middleware>) -> &mut Self {
        self.middleware.push(middleware);
        self
    }

    /// Mount a router at a specific prefix
    pub fn mount(&mut self, prefix: &str, router: Router) -> &mut Self {
        self.router.mount(prefix, router);
        self
    }

    /// Handle an incoming request through the middleware pipeline and routing
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

    /// Start the server and listen for incoming connections
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
