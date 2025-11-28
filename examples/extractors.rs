//! Example demonstrating extractor patterns in Ruffus

use ruffus::{App, FromRequest, Json, Path, Query, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserPath {
    id: u32,
}

#[derive(Deserialize)]
struct Pagination {
    page: u32,
    limit: u32,
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    // Using Path extractor for path parameters
    app.get("/users/:id", |mut req: ruffus::Request| async move {
        let Path(params): Path<UserPath> = Path::from_request(&mut req).await?;
        
        let user = User {
            id: params.id,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };
        
        Response::json(&user)
    });

    // Using Query extractor for query parameters
    app.get("/users", |mut req: ruffus::Request| async move {
        let Query(params): Query<Pagination> = Query::from_request(&mut req).await?;
        
        let users = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
        ];
        
        // In a real app, you'd use page and limit to paginate
        println!("Fetching page {} with limit {}", params.page, params.limit);
        
        Response::json(&users)
    });

    // Using Json extractor for request body
    app.post("/users", |mut req: ruffus::Request| async move {
        let Json(body): Json<CreateUser> = Json::from_request(&mut req).await?;
        
        let user = User {
            id: 123,
            name: body.name,
            email: body.email,
        };
        
        Response::json(&user)
    });

    println!("Starting Ruffus server with extractors...");
    println!("Try:");
    println!("  GET  http://localhost:3000/users/42");
    println!("  GET  http://localhost:3000/users?page=1&limit=10");
    println!("  POST http://localhost:3000/users (with JSON body)");
    
    app.listen("127.0.0.1:3000").await.unwrap();
}
