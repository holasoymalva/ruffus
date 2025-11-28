//! Example demonstrating router organization in Ruffus

use ruffus::{App, Request, Response, Router};
use serde::{Deserialize, Serialize};
use http::StatusCode;

#[derive(Serialize)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
}

#[derive(Serialize)]
struct Order {
    id: u32,
    user_id: u32,
    product_ids: Vec<u32>,
    total: f64,
    status: String,
}

#[derive(Deserialize, Serialize)]
struct BlogPost {
    id: Option<u32>,
    title: String,
    content: String,
    author: String,
}

/// Create a router for user-related endpoints
fn create_user_router() -> Router {
    let mut router = Router::new("/users");

    router.get("", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "users": [
                {"id": 1, "name": "Alice", "email": "alice@example.com"},
                {"id": 2, "name": "Bob", "email": "bob@example.com"}
            ]
        }))
    });

    router.get("/:id", |req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        use serde_json::json;
        Response::json(&json!({
            "id": id,
            "name": "Alice",
            "email": "alice@example.com"
        }))
    });

    router.post("", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "id": 3,
            "name": "Charlie",
            "email": "charlie@example.com",
            "message": "User created successfully"
        })).map(|r| {
            r.status(StatusCode::CREATED)
        })
    });

    router.delete("/:id", |req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        use serde_json::json;
        Response::json(&json!({
            "message": format!("User {} deleted", id)
        }))
    });

    router
}

/// Create a router for product-related endpoints
fn create_product_router() -> Router {
    let mut router = Router::new("/products");

    router.get("", |_req: Request| async {
        let products = vec![
            Product {
                id: 1,
                name: "Laptop".to_string(),
                price: 999.99,
                category: "Electronics".to_string(),
            },
            Product {
                id: 2,
                name: "Mouse".to_string(),
                price: 29.99,
                category: "Electronics".to_string(),
            },
            Product {
                id: 3,
                name: "Desk".to_string(),
                price: 299.99,
                category: "Furniture".to_string(),
            },
        ];
        Response::json(&products)
    });

    router.get("/:id", |req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        let product_id: u32 = id.parse().unwrap_or(0);
        
        let product = Product {
            id: product_id,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
        };
        Response::json(&product)
    });

    router.get("/category/:category", |req: Request| async move {
        let category = req.param("category").unwrap_or("all");
        
        let products = vec![
            Product {
                id: 1,
                name: "Laptop".to_string(),
                price: 999.99,
                category: category.to_string(),
            },
        ];
        Response::json(&products)
    });

    router
}

/// Create a router for order-related endpoints
fn create_order_router() -> Router {
    let mut router = Router::new("/orders");

    router.get("", |_req: Request| async {
        let orders = vec![
            Order {
                id: 1,
                user_id: 1,
                product_ids: vec![1, 2],
                total: 1029.98,
                status: "completed".to_string(),
            },
            Order {
                id: 2,
                user_id: 2,
                product_ids: vec![3],
                total: 299.99,
                status: "pending".to_string(),
            },
        ];
        Response::json(&orders)
    });

    router.get("/:id", |req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        let order_id: u32 = id.parse().unwrap_or(0);
        
        let order = Order {
            id: order_id,
            user_id: 1,
            product_ids: vec![1, 2],
            total: 1029.98,
            status: "completed".to_string(),
        };
        Response::json(&order)
    });

    router.post("", |_req: Request| async {
        let order = Order {
            id: 3,
            user_id: 1,
            product_ids: vec![1],
            total: 999.99,
            status: "pending".to_string(),
        };
        Response::json(&order).map(|r| {
            r.status(StatusCode::CREATED)
        })
    });

    router
}

/// Create a router for blog-related endpoints
fn create_blog_router() -> Router {
    let mut router = Router::new("/blog");

    router.get("/posts", |_req: Request| async {
        let posts = vec![
            BlogPost {
                id: Some(1),
                title: "Getting Started with Ruffus".to_string(),
                content: "Ruffus is a fast, minimalist web framework...".to_string(),
                author: "Alice".to_string(),
            },
            BlogPost {
                id: Some(2),
                title: "Building REST APIs".to_string(),
                content: "Learn how to build REST APIs with Ruffus...".to_string(),
                author: "Bob".to_string(),
            },
        ];
        Response::json(&posts)
    });

    router.get("/posts/:id", |req: Request| async move {
        let id = req.param("id").unwrap_or("0");
        let post_id: u32 = id.parse().unwrap_or(0);
        
        let post = BlogPost {
            id: Some(post_id),
            title: "Getting Started with Ruffus".to_string(),
            content: "Ruffus is a fast, minimalist web framework for Rust...".to_string(),
            author: "Alice".to_string(),
        };
        Response::json(&post)
    });

    router.post("/posts", |_req: Request| async {
        let post = BlogPost {
            id: Some(3),
            title: "New Post".to_string(),
            content: "This is a new blog post...".to_string(),
            author: "Charlie".to_string(),
        };
        Response::json(&post).map(|r| {
            r.status(StatusCode::CREATED)
        })
    });

    router
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    // Root endpoint
    app.get("/", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "name": "Ruffus Router Example",
            "version": "0.1.0",
            "endpoints": {
                "api": "/api/*",
                "blog": "/blog/*"
            }
        }))
    });

    // Create an API router that groups all API endpoints
    let mut api_router = Router::new("/api");
    
    // Mount sub-routers onto the API router
    // This creates nested routes like /api/users, /api/products, etc.
    api_router.mount("", create_user_router());
    api_router.mount("", create_product_router());
    api_router.mount("", create_order_router());

    // Mount the API router onto the main app
    app.mount("", api_router);

    // Mount the blog router directly onto the main app
    app.mount("", create_blog_router());

    // Health check endpoint
    app.get("/health", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    });

    println!("Starting Ruffus server with organized routers...");
    println!("\nRouter structure:");
    println!("  /");
    println!("  /health");
    println!("  /api");
    println!("    /api/users");
    println!("      GET    /api/users");
    println!("      GET    /api/users/:id");
    println!("      POST   /api/users");
    println!("      DELETE /api/users/:id");
    println!("    /api/products");
    println!("      GET    /api/products");
    println!("      GET    /api/products/:id");
    println!("      GET    /api/products/category/:category");
    println!("    /api/orders");
    println!("      GET    /api/orders");
    println!("      GET    /api/orders/:id");
    println!("      POST   /api/orders");
    println!("  /blog");
    println!("    GET    /blog/posts");
    println!("    GET    /blog/posts/:id");
    println!("    POST   /blog/posts");
    
    app.listen("127.0.0.1:3000").await.unwrap();
}
