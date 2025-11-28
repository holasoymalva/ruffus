//! Property-based tests for App

use quickcheck::QuickCheck;
use ruffus::{App, Request, Response};

// **Feature: rust-web-framework, Property 1: Application initialization creates empty state**
// **Validates: Requirements 1.1**
fn prop_app_initialization_creates_empty_state() -> bool {
    // Create a new App instance
    let app = App::new();
    
    // Verify that the router is initialized (has no routes)
    let router = app.router();
    let routes = router.routes();
    
    // Verify that middleware stack is empty
    let middleware = app.middleware();
    
    // Both should be empty for a newly created app
    routes.is_empty() && middleware.is_empty()
}

#[test]
fn test_app_initialization_property() {
    QuickCheck::new()
        .tests(100)
        .quickcheck(prop_app_initialization_creates_empty_state as fn() -> bool);
}

// **Feature: rust-web-framework, Property 4: Handler responses are sent correctly**
// **Validates: Requirements 1.4**
fn prop_handler_responses_sent_correctly(
    status_code: u16,
    header_key: String,
    header_value: String,
    body_text: String,
) -> bool {
    use http::StatusCode;
    use tokio::runtime::Runtime;
    
    // Filter out invalid status codes and header values
    if status_code < 100 || status_code > 599 {
        return true; // Skip invalid status codes
    }
    
    // Filter out invalid header keys/values
    if header_key.is_empty() || header_value.is_empty() {
        return true;
    }
    
    // Filter out non-ASCII header keys/values
    if !header_key.is_ascii() || !header_value.is_ascii() {
        return true;
    }
    
    // Filter out header keys with invalid characters
    if header_key.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
        return true;
    }
    
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        let mut app = App::new();
        
        // Create a response with the given status, header, and body
        let status = StatusCode::from_u16(status_code).unwrap();
        let expected_body = body_text.clone();
        let expected_header_key = header_key.clone();
        let expected_header_value = header_value.clone();
        
        app.get("/test", move |_req| {
            let body = expected_body.clone();
            let key = expected_header_key.clone();
            let val = expected_header_value.clone();
            async move {
                Ok(Response::new()
                    .status(status)
                    .header(&key, &val)
                    .body(body))
            }
        });
        
        // Create a request
        let req = Request::new(
            http::Method::GET,
            "/test".parse().unwrap(),
            http::HeaderMap::new(),
            bytes::Bytes::new(),
        );
        
        // Handle the request
        let response = app.handle_request(req).await.unwrap();
        
        // Verify the response has the correct status code
        let status_matches = response.get_status() == status;
        
        // Verify the response has the correct header
        let header_matches = response.get_headers()
            .get(&header_key)
            .map(|v| v.to_str().unwrap_or("") == header_value)
            .unwrap_or(false);
        
        // Verify the response has the correct body
        let body_matches = response.get_body() == &bytes::Bytes::from(body_text);
        
        status_matches && header_matches && body_matches
    })
}

#[test]
fn test_handler_responses_sent_correctly_property() {
    QuickCheck::new()
        .tests(100)
        .quickcheck(prop_handler_responses_sent_correctly as fn(u16, String, String, String) -> bool);
}
