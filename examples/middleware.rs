//! Example demonstrating middleware usage in Ruffus

use async_trait::async_trait;
use ruffus::{App, Middleware, Next, Request, Response, Result, Router};
use std::sync::Arc;
use std::time::Instant;
use http::StatusCode;

/// Logger middleware that logs request method and path
struct Logger;

#[async_trait]
impl Middleware for Logger {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        
        println!("[Logger] {} {}", method, path);
        
        // Continue to next middleware or handler
        next.run(req).await
    }
}

/// Timer middleware that measures request processing time
struct Timer;

#[async_trait]
impl Middleware for Timer {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let start = Instant::now();
        
        // Process the request
        let response = next.run(req).await;
        
        let duration = start.elapsed();
        println!("[Timer] Request processed in {:?}", duration);
        
        response
    }
}

/// Authentication middleware that checks for an API key
struct Auth {
    api_key: String,
}

impl Auth {
    fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl Middleware for Auth {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        // Check for Authorization header
        let auth_header = req.headers()
            .get("authorization")
            .and_then(|v| v.to_str().ok());
        
        match auth_header {
            Some(header) if header == format!("Bearer {}", self.api_key) => {
                println!("[Auth] Authentication successful");
                next.run(req).await
            }
            Some(_) => {
                println!("[Auth] Invalid API key");
                use serde_json::json;
                Response::json(&json!({
                    "error": "Invalid API key"
                })).map(|r| {
                    r.status(StatusCode::UNAUTHORIZED)
                })
            }
            None => {
                println!("[Auth] Missing Authorization header");
                use serde_json::json;
                Response::json(&json!({
                    "error": "Missing Authorization header"
                })).map(|r| {
                    r.status(StatusCode::UNAUTHORIZED)
                })
            }
        }
    }
}

/// CORS middleware that adds CORS headers to responses
struct Cors;

#[async_trait]
impl Middleware for Cors {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        // Process the request
        let response = next.run(req).await?;
        
        // Add CORS headers to the response
        let response = response
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type, Authorization");
        
        println!("[CORS] Added CORS headers");
        
        Ok(response)
    }
}

/// Request ID middleware that adds a unique ID to each request
struct RequestId;

#[async_trait]
impl Middleware for RequestId {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        // Generate a simple request ID (in production, use UUID)
        let request_id = format!("req-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis());
        
        println!("[RequestId] Generated ID: {}", request_id);
        
        // Process the request
        let response = next.run(req).await?;
        
        // Add request ID to response headers
        let response = response.header("X-Request-Id", &request_id);
        
        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    // Add global middleware (applies to all routes)
    // Middleware executes in the order they are registered
    app.use_middleware(Arc::new(Logger));
    app.use_middleware(Arc::new(Timer));
    app.use_middleware(Arc::new(RequestId));
    app.use_middleware(Arc::new(Cors));

    // Public route (no authentication required)
    app.get("/", |_req: Request| async {
        Ok(Response::text("Welcome to Ruffus! This is a public endpoint.".to_string()))
    });

    // Public API info route
    app.get("/api", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "name": "Ruffus API",
            "version": "0.1.0",
            "endpoints": {
                "public": ["/", "/api"],
                "protected": ["/protected", "/admin"]
            }
        }))
    });

    // Create a protected router with authentication
    let mut protected_router = Router::new("");
    
    // Add authentication middleware only to protected routes
    protected_router.use_middleware(Arc::new(Auth::new("secret-key-123".to_string())));

    protected_router.get("/protected", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "message": "You have accessed a protected resource!",
            "data": "Secret information"
        }))
    });

    protected_router.get("/admin", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "message": "Admin panel",
            "users": ["alice", "bob", "charlie"]
        }))
    });

    // Mount the protected routes
    app.mount("", protected_router);

    println!("Starting Ruffus server with middleware...");
    println!("\nMiddleware chain:");
    println!("  1. Logger    - Logs all requests");
    println!("  2. Timer     - Measures request duration");
    println!("  3. RequestId - Adds unique request ID");
    println!("  4. CORS      - Adds CORS headers");
    println!("  5. Auth      - Protects /protected and /admin routes");
    println!("\nTry these endpoints:");
    println!("  GET http://localhost:3000/");
    println!("  GET http://localhost:3000/api");
    println!("  GET http://localhost:3000/protected");
    println!("      (requires: Authorization: Bearer secret-key-123)");
    println!("  GET http://localhost:3000/admin");
    println!("      (requires: Authorization: Bearer secret-key-123)");
    
    app.listen("127.0.0.1:3000").await.unwrap();
}
