//! Example demonstrating JSON request/response handling in Ruffus

use ruffus::{App, Request, Response};
use serde::{Deserialize, Serialize};
use http::StatusCode;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
    age: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
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

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    // GET /users - List all users (returns JSON array)
    app.get("/users", |_req: Request| async {
        let users = vec![
            User {
                id: Some(1),
                name: "Alice Johnson".to_string(),
                email: "alice@example.com".to_string(),
                age: Some(28),
            },
            User {
                id: Some(2),
                name: "Bob Smith".to_string(),
                email: "bob@example.com".to_string(),
                age: Some(35),
            },
            User {
                id: Some(3),
                name: "Charlie Brown".to_string(),
                email: "charlie@example.com".to_string(),
                age: None,
            },
        ];

        Response::json(&ApiResponse::success(users))
    });

    // GET /users/:id - Get a specific user by ID
    app.get("/users/:id", |req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        
        // Parse the ID
        let user_id: u32 = match id.parse() {
            Ok(id) => id,
            Err(_) => {
                return Response::json(&ApiResponse::<()>::error(
                    "Invalid user ID".to_string()
                )).map(|r| {
                    r.status(StatusCode::BAD_REQUEST)
                });
            }
        };

        // Simulate finding a user
        let user = User {
            id: Some(user_id),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: Some(28),
        };

        Response::json(&ApiResponse::success(user))
    });

    // POST /users - Create a new user (accepts JSON body)
    app.post("/users", |mut req: Request| async move {
        // Parse JSON body
        let body: CreateUserRequest = match req.json().await {
            Ok(body) => body,
            Err(e) => {
                return Response::json(&ApiResponse::<()>::error(
                    format!("Invalid JSON: {}", e)
                )).map(|r| {
                    r.status(StatusCode::BAD_REQUEST)
                });
            }
        };

        // Validate the request
        if body.name.is_empty() {
            return Response::json(&ApiResponse::<()>::error(
                "Name cannot be empty".to_string()
            )).map(|r| {
                r.status(StatusCode::BAD_REQUEST)
            });
        }

        if !body.email.contains('@') {
            return Response::json(&ApiResponse::<()>::error(
                "Invalid email address".to_string()
            )).map(|r| {
                r.status(StatusCode::BAD_REQUEST)
            });
        }

        // Create the user (simulate with ID 123)
        let user = User {
            id: Some(123),
            name: body.name,
            email: body.email,
            age: body.age,
        };

        Response::json(&ApiResponse::success(user)).map(|r| {
            r.status(StatusCode::CREATED)
        })
    });

    // PUT /users/:id - Update a user (accepts JSON body)
    app.put("/users/:id", |mut req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        
        // Parse the ID
        let user_id: u32 = match id.parse() {
            Ok(id) => id,
            Err(_) => {
                return Response::json(&ApiResponse::<()>::error(
                    "Invalid user ID".to_string()
                )).map(|r| {
                    r.status(StatusCode::BAD_REQUEST)
                });
            }
        };

        // Parse JSON body
        let body: UpdateUserRequest = match req.json().await {
            Ok(body) => body,
            Err(e) => {
                return Response::json(&ApiResponse::<()>::error(
                    format!("Invalid JSON: {}", e)
                )).map(|r| {
                    r.status(StatusCode::BAD_REQUEST)
                });
            }
        };

        // Update the user (simulate)
        let user = User {
            id: Some(user_id),
            name: body.name.unwrap_or_else(|| "Alice Johnson".to_string()),
            email: body.email.unwrap_or_else(|| "alice@example.com".to_string()),
            age: body.age,
        };

        Response::json(&ApiResponse::success(user))
    });

    // DELETE /users/:id - Delete a user
    app.delete("/users/:id", |req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        
        // Parse the ID
        let user_id: u32 = match id.parse() {
            Ok(id) => id,
            Err(_) => {
                return Response::json(&ApiResponse::<()>::error(
                    "Invalid user ID".to_string()
                )).map(|r| {
                    r.status(StatusCode::BAD_REQUEST)
                });
            }
        };

        use serde_json::json;
        Response::json(&json!({
            "success": true,
            "message": format!("User {} deleted successfully", user_id)
        }))
    });

    println!("Starting Ruffus JSON API server...");
    println!("Try these endpoints:");
    println!("  GET    http://localhost:3000/users");
    println!("  GET    http://localhost:3000/users/1");
    println!("  POST   http://localhost:3000/users");
    println!("         Body: {{\"name\": \"John\", \"email\": \"john@example.com\", \"age\": 30}}");
    println!("  PUT    http://localhost:3000/users/1");
    println!("         Body: {{\"name\": \"Jane\"}}");
    println!("  DELETE http://localhost:3000/users/1");
    
    app.listen("127.0.0.1:3000").await.unwrap();
}
