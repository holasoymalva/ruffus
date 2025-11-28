//! Basic example of a Ruffus server

use ruffus::{App, Request, Response};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    // Simple GET route
    app.get("/", |_req: Request| async {
        Ok(Response::text("Hello, Ruffus!".to_string()))
    });

    // Route with path parameter
    app.get("/hello/:name", |req: Request| async move {
        let name = req.param("name").unwrap_or("World");
        Ok(Response::text(format!("Hello, {}!", name)))
    });

    // JSON response
    app.get("/json", |_req: Request| async {
        use serde_json::json;
        Response::json(&json!({
            "message": "Hello from Ruffus!",
            "version": "0.1.0"
        }))
    });

    println!("Starting Ruffus server...");
    app.listen("127.0.0.1:3000").await.unwrap();
}
