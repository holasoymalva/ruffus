//! Complete REST API example with Ruffus
//! 
//! This example demonstrates a full-featured REST API with:
//! - CRUD operations
//! - Middleware (logging, auth, CORS)
//! - Router organization
//! - Error handling
//! - JSON request/response handling
//! - Path and query parameters

use async_trait::async_trait;
use ruffus::{App, Middleware, Next, Request, Response, Result, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use http::StatusCode;

// ============================================================================
// Data Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTaskRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }
}

impl ApiResponse<()> {
    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

// ============================================================================
// In-Memory Database
// ============================================================================

type Database = Arc<Mutex<HashMap<u32, Task>>>;

fn create_database() -> Database {
    let mut db = HashMap::new();
    
    // Seed with some initial data
    db.insert(1, Task {
        id: 1,
        title: "Learn Rust".to_string(),
        description: "Study the Rust programming language".to_string(),
        completed: false,
        created_at: "2024-01-01T10:00:00Z".to_string(),
    });
    
    db.insert(2, Task {
        id: 2,
        title: "Build a web framework".to_string(),
        description: "Create Ruffus, a minimalist web framework".to_string(),
        completed: true,
        created_at: "2024-01-02T11:00:00Z".to_string(),
    });
    
    db.insert(3, Task {
        id: 3,
        title: "Write documentation".to_string(),
        description: "Document all the features and examples".to_string(),
        completed: false,
        created_at: "2024-01-03T12:00:00Z".to_string(),
    });
    
    Arc::new(Mutex::new(db))
}

// ============================================================================
// Middleware
// ============================================================================

/// Logger middleware
struct Logger;

#[async_trait]
impl Middleware for Logger {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        
        println!("[{}] {} {}", timestamp, method, path);
        
        next.run(req).await
    }
}

/// Timer middleware
struct Timer;

#[async_trait]
impl Middleware for Timer {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let start = Instant::now();
        let response = next.run(req).await;
        let duration = start.elapsed();
        
        println!("  ⏱️  Request completed in {:?}", duration);
        
        response
    }
}

/// CORS middleware
struct Cors;

#[async_trait]
impl Middleware for Cors {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let response = next.run(req).await?;
        
        let response = response
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type, Authorization");
        
        Ok(response)
    }
}

/// Simple API key authentication
struct ApiKeyAuth {
    api_key: String,
}

impl ApiKeyAuth {
    fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl Middleware for ApiKeyAuth {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        let api_key = req.headers()
            .get("x-api-key")
            .and_then(|v| v.to_str().ok());
        
        match api_key {
            Some(key) if key == self.api_key => {
                next.run(req).await
            }
            _ => {
                Response::json(&ApiResponse::<()>::error(
                    "Invalid or missing API key".to_string()
                )).map(|r| {
                    r.status(StatusCode::UNAUTHORIZED)
                })
            }
        }
    }
}

// ============================================================================
// Route Handlers
// ============================================================================

fn create_task_router(db: Database) -> Router {
    let mut router = Router::new("/tasks");

    // GET /tasks - List all tasks with optional filtering
    let db_clone = db.clone();
    router.get("", move |req: Request| {
        let db = db_clone.clone();
        async move {
            let db = db.lock().unwrap();
            
            // Check for completed filter in query params
            let completed_filter = req.query("completed");
            
            let tasks: Vec<Task> = db.values()
                .filter(|task| {
                    if let Some(filter) = completed_filter {
                        if filter == "true" {
                            task.completed
                        } else if filter == "false" {
                            !task.completed
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
            
            Response::json(&ApiResponse::success(tasks))
        }
    });

    // GET /tasks/:id - Get a specific task
    let db_clone = db.clone();
    router.get("/:id", move |req: Request| {
        let db = db_clone.clone();
        async move {
            let id = req.param("id").unwrap_or("0");
            let task_id: u32 = match id.parse() {
                Ok(id) => id,
                Err(_) => {
                    return Response::json(&ApiResponse::<()>::error(
                        "Invalid task ID".to_string()
                    )).map(|r| {
                        r.status(StatusCode::BAD_REQUEST)
                    });
                }
            };
            
            let db = db.lock().unwrap();
            
            match db.get(&task_id) {
                Some(task) => Response::json(&ApiResponse::success(task.clone())),
                None => Response::json(&ApiResponse::<()>::error(
                    "Task not found".to_string()
                )).map(|r| {
                    r.status(StatusCode::NOT_FOUND)
                }),
            }
        }
    });

    // POST /tasks - Create a new task
    let db_clone = db.clone();
    router.post("", move |mut req: Request| {
        let db = db_clone.clone();
        async move {
            let body: CreateTaskRequest = match req.json().await {
                Ok(body) => body,
                Err(e) => {
                    return Response::json(&ApiResponse::<()>::error(
                        format!("Invalid JSON: {}", e)
                    )).map(|r| {
                        r.status(StatusCode::BAD_REQUEST)
                    });
                }
            };
            
            // Validate
            if body.title.trim().is_empty() {
                return Response::json(&ApiResponse::<()>::error(
                    "Title cannot be empty".to_string()
                )).map(|r| {
                    r.status(StatusCode::BAD_REQUEST)
                });
            }
            
            let mut db = db.lock().unwrap();
            
            // Generate new ID
            let new_id = db.keys().max().unwrap_or(&0) + 1;
            
            let task = Task {
                id: new_id,
                title: body.title,
                description: body.description,
                completed: false,
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            
            db.insert(new_id, task.clone());
            
            Response::json(&ApiResponse::success(task)).map(|r| {
                r.status(StatusCode::CREATED)
            })
        }
    });

    // PUT /tasks/:id - Update a task
    let db_clone = db.clone();
    router.put("/:id", move |mut req: Request| {
        let db = db_clone.clone();
        async move {
            let id = req.param("id").unwrap_or("0");
            let task_id: u32 = match id.parse() {
                Ok(id) => id,
                Err(_) => {
                    return Response::json(&ApiResponse::<()>::error(
                        "Invalid task ID".to_string()
                    )).map(|r| {
                        r.status(StatusCode::BAD_REQUEST)
                    });
                }
            };
            
            let body: UpdateTaskRequest = match req.json().await {
                Ok(body) => body,
                Err(e) => {
                    return Response::json(&ApiResponse::<()>::error(
                        format!("Invalid JSON: {}", e)
                    )).map(|r| {
                        r.status(StatusCode::BAD_REQUEST)
                    });
                }
            };
            
            let mut db = db.lock().unwrap();
            
            match db.get_mut(&task_id) {
                Some(task) => {
                    if let Some(title) = body.title {
                        if !title.trim().is_empty() {
                            task.title = title;
                        }
                    }
                    if let Some(description) = body.description {
                        task.description = description;
                    }
                    if let Some(completed) = body.completed {
                        task.completed = completed;
                    }
                    
                    Response::json(&ApiResponse::success(task.clone()))
                }
                None => Response::json(&ApiResponse::<()>::error(
                    "Task not found".to_string()
                )).map(|r| {
                    r.status(StatusCode::NOT_FOUND)
                }),
            }
        }
    });

    // DELETE /tasks/:id - Delete a task
    let db_clone = db.clone();
    router.delete("/:id", move |req: Request| {
        let db = db_clone.clone();
        async move {
            let id = req.param("id").unwrap_or("0");
            let task_id: u32 = match id.parse() {
                Ok(id) => id,
                Err(_) => {
                    return Response::json(&ApiResponse::<()>::error(
                        "Invalid task ID".to_string()
                    )).map(|r| {
                        r.status(StatusCode::BAD_REQUEST)
                    });
                }
            };
            
            let mut db = db.lock().unwrap();
            
            match db.remove(&task_id) {
                Some(_) => {
                    use serde_json::json;
                    Response::json(&json!({
                        "success": true,
                        "message": format!("Task {} deleted successfully", task_id)
                    }))
                }
                None => Response::json(&ApiResponse::<()>::error(
                    "Task not found".to_string()
                )).map(|r| {
                    r.status(StatusCode::NOT_FOUND)
                }),
            }
        }
    });

    router
}

// ============================================================================
// Main Application
// ============================================================================

#[tokio::main]
async fn main() {
    let mut app = App::new();
    
    // Initialize database
    let db = create_database();

    // Add global middleware
    app.use_middleware(Arc::new(Logger));
    app.use_middleware(Arc::new(Timer));
    app.use_middleware(Arc::new(Cors));

    // Root endpoint
    app.get("/", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "name": "Ruffus Task API",
            "version": "1.0.0",
            "description": "A complete REST API example built with Ruffus",
            "endpoints": {
                "health": "GET /health",
                "tasks": {
                    "list": "GET /api/tasks?completed=true|false",
                    "get": "GET /api/tasks/:id",
                    "create": "POST /api/tasks",
                    "update": "PUT /api/tasks/:id",
                    "delete": "DELETE /api/tasks/:id"
                },
                "admin": {
                    "stats": "GET /api/admin/stats (requires X-API-Key header)"
                }
            }
        }))
    });

    // Health check endpoint
    app.get("/health", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    });

    // Create API router
    let mut api_router = Router::new("/api");
    
    // Mount task router
    api_router.mount("", create_task_router(db.clone()));
    
    // Admin endpoints with authentication
    let mut admin_router = Router::new("/admin");
    admin_router.use_middleware(Arc::new(ApiKeyAuth::new("secret-admin-key".to_string())));
    
    let db_clone = db.clone();
    admin_router.get("/stats", move |_req: Request| {
        let db = db_clone.clone();
        async move {
            let db = db.lock().unwrap();
            let total = db.len();
            let completed = db.values().filter(|t| t.completed).count();
            let pending = total - completed;
            
            use serde_json::json;
            Response::json(&json!({
                "total_tasks": total,
                "completed_tasks": completed,
                "pending_tasks": pending,
                "completion_rate": if total > 0 { 
                    (completed as f64 / total as f64) * 100.0 
                } else { 
                    0.0 
                }
            }))
        }
    });
    
    api_router.mount("", admin_router);
    
    // Mount API router
    app.mount("", api_router);

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         Ruffus Full REST API Example                      ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("Server running at: http://localhost:3000");
    println!();
    println!("📋 Available Endpoints:");
    println!("  GET    /                    - API information");
    println!("  GET    /health              - Health check");
    println!();
    println!("📝 Task Management:");
    println!("  GET    /api/tasks           - List all tasks");
    println!("  GET    /api/tasks?completed=true  - Filter completed tasks");
    println!("  GET    /api/tasks/:id       - Get specific task");
    println!("  POST   /api/tasks           - Create new task");
    println!("  PUT    /api/tasks/:id       - Update task");
    println!("  DELETE /api/tasks/:id       - Delete task");
    println!();
    println!("🔐 Admin (requires X-API-Key: secret-admin-key):");
    println!("  GET    /api/admin/stats     - Get statistics");
    println!();
    println!("💡 Example requests:");
    println!("  curl http://localhost:3000/api/tasks");
    println!("  curl -X POST http://localhost:3000/api/tasks \\");
    println!("       -H 'Content-Type: application/json' \\");
    println!("       -d '{{\"title\":\"New Task\",\"description\":\"Do something\"}}'");
    println!("  curl http://localhost:3000/api/admin/stats \\");
    println!("       -H 'X-API-Key: secret-admin-key'");
    println!();
    
    app.listen("127.0.0.1:3000").await.unwrap();
}
