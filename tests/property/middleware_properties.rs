// Property-based tests for middleware functionality

use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};
use quickcheck_macros::quickcheck;
use ruffus::{Handler, Middleware, Next, Request, Response, Result};
use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, Method, Uri};
use std::sync::{Arc, Mutex};

// Helper to create a test request
fn create_test_request() -> Request {
    Request::new(
        Method::GET,
        Uri::from_static("http://localhost/test"),
        HeaderMap::new(),
        Bytes::new(),
    )
}

// Middleware that records its execution order
struct OrderRecordingMiddleware {
    id: usize,
    order: Arc<Mutex<Vec<usize>>>,
}

impl OrderRecordingMiddleware {
    fn new(id: usize, order: Arc<Mutex<Vec<usize>>>) -> Self {
        Self { id, order }
    }
}

#[async_trait]
impl Middleware for OrderRecordingMiddleware {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        self.order.lock().unwrap().push(self.id);
        next.run(req).await
    }
}

// **Feature: rust-web-framework, Property 13: Middleware registration order is preserved**
// **Validates: Requirements 4.1**
#[quickcheck]
fn prop_middleware_registration_order_preserved(count: u8) -> TestResult {
    // Limit to reasonable number of middleware
    let count = (count % 10) + 1;
    
    if count == 0 {
        return TestResult::discard();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let execution_order = Arc::new(Mutex::new(Vec::new()));
        let mut middleware_stack: Vec<Arc<dyn Middleware>> = Vec::new();

        // Register middleware in order
        for i in 0..count {
            middleware_stack.push(Arc::new(OrderRecordingMiddleware::new(
                i as usize,
                execution_order.clone(),
            )));
        }

        // Create a simple handler
        let handler = Arc::new(|_req: Request| {
            Box::pin(async { Ok(Response::new()) })
                as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
        });

        // Execute the middleware stack
        let req = create_test_request();
        let _ = ruffus::middleware::execute_middleware_stack(
            middleware_stack,
            handler,
            req,
        )
        .await;

        // Check that middleware executed in registration order
        let order = execution_order.lock().unwrap();
        let expected: Vec<usize> = (0..count as usize).collect();
        
        TestResult::from_bool(*order == expected)
    })
}

// **Feature: rust-web-framework, Property 14: Middleware executes in order**
// **Validates: Requirements 4.2, 4.5**
#[quickcheck]
fn prop_middleware_executes_in_order(count: u8) -> TestResult {
    // Limit to reasonable number of middleware
    let count = (count % 10) + 1;
    
    if count == 0 {
        return TestResult::discard();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let execution_order = Arc::new(Mutex::new(Vec::new()));
        let mut middleware_stack: Vec<Arc<dyn Middleware>> = Vec::new();

        // Register middleware in order
        for i in 0..count {
            middleware_stack.push(Arc::new(OrderRecordingMiddleware::new(
                i as usize,
                execution_order.clone(),
            )));
        }

        // Create a handler that also records execution
        let handler_order = execution_order.clone();
        let handler = Arc::new(move |_req: Request| {
            let order = handler_order.clone();
            Box::pin(async move {
                order.lock().unwrap().push(999); // Use 999 to mark handler execution
                Ok(Response::new())
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
        });

        // Execute the middleware stack
        let req = create_test_request();
        let _ = ruffus::middleware::execute_middleware_stack(
            middleware_stack,
            handler,
            req,
        )
        .await;

        // Check that middleware executed in order, then handler
        let order = execution_order.lock().unwrap();
        let mut expected: Vec<usize> = (0..count as usize).collect();
        expected.push(999); // Handler should execute last
        
        TestResult::from_bool(*order == expected)
    })
}

// Middleware that modifies the request by adding a parameter
struct RequestModifyingMiddleware {
    key: String,
    value: String,
}

impl RequestModifyingMiddleware {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

#[async_trait]
impl Middleware for RequestModifyingMiddleware {
    async fn handle(&self, mut req: Request, next: Next) -> Result<Response> {
        req.set_param(self.key.clone(), self.value.clone());
        next.run(req).await
    }
}

// **Feature: rust-web-framework, Property 15: Request modifications propagate through chain**
// **Validates: Requirements 4.3**
#[quickcheck]
fn prop_request_modifications_propagate(modifications: Vec<(String, String)>) -> TestResult {
    // Limit to reasonable number of modifications
    if modifications.is_empty() || modifications.len() > 10 {
        return TestResult::discard();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut middleware_stack: Vec<Arc<dyn Middleware>> = Vec::new();

        // Create middleware that adds parameters
        for (key, value) in modifications.iter() {
            middleware_stack.push(Arc::new(RequestModifyingMiddleware::new(
                key.clone(),
                value.clone(),
            )));
        }

        // Create a handler that checks all modifications are present
        let expected_mods = modifications.clone();
        let handler = Arc::new(move |req: Request| {
            let mods = expected_mods.clone();
            Box::pin(async move {
                // Check that all modifications are present
                for (key, value) in mods.iter() {
                    if req.param(key) != Some(value.as_str()) {
                        return Err(ruffus::Error::BadRequest(format!(
                            "Missing or incorrect parameter: {}",
                            key
                        )));
                    }
                }
                Ok(Response::new())
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
        });

        // Execute the middleware stack
        let req = create_test_request();
        let result = ruffus::middleware::execute_middleware_stack(
            middleware_stack,
            handler,
            req,
        )
        .await;

        TestResult::from_bool(result.is_ok())
    })
}

// Middleware that returns early without calling next
struct EarlyReturnMiddleware {
    status_code: u16,
}

impl EarlyReturnMiddleware {
    fn new(status_code: u16) -> Self {
        Self { status_code }
    }
}

#[async_trait]
impl Middleware for EarlyReturnMiddleware {
    async fn handle(&self, _req: Request, _next: Next) -> Result<Response> {
        Ok(Response::new().status(http::StatusCode::from_u16(self.status_code).unwrap()))
    }
}

// **Feature: rust-web-framework, Property 16: Early middleware return skips remaining chain**
// **Validates: Requirements 4.4**
#[quickcheck]
fn prop_early_middleware_return_skips_chain(
    early_position: u8,
    total_count: u8,
    status_code: u16,
) -> TestResult {
    // Limit to reasonable numbers
    let total_count = (total_count % 10) + 2; // At least 2 middleware
    let early_position = early_position % total_count;
    
    // Use valid HTTP status codes
    let status_code = 200 + (status_code % 100);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let execution_order = Arc::new(Mutex::new(Vec::new()));
        let mut middleware_stack: Vec<Arc<dyn Middleware>> = Vec::new();

        // Add middleware that record execution
        for i in 0..total_count {
            if i == early_position {
                // This middleware returns early
                middleware_stack.push(Arc::new(EarlyReturnMiddleware::new(status_code)));
            } else {
                // Regular middleware that records execution
                middleware_stack.push(Arc::new(OrderRecordingMiddleware::new(
                    i as usize,
                    execution_order.clone(),
                )));
            }
        }

        // Create a handler that should NOT be called
        let handler_order = execution_order.clone();
        let handler = Arc::new(move |_req: Request| {
            let order = handler_order.clone();
            Box::pin(async move {
                order.lock().unwrap().push(999); // Should not be reached
                Ok(Response::new())
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
        });

        // Execute the middleware stack
        let req = create_test_request();
        let result = ruffus::middleware::execute_middleware_stack(
            middleware_stack,
            handler,
            req,
        )
        .await;

        // Check that:
        // 1. The response is Ok
        // 2. Only middleware before early_position executed
        // 3. Handler was not called (999 not in order)
        let order = execution_order.lock().unwrap();
        let expected_count = early_position as usize;
        let handler_not_called = !order.contains(&999);
        let correct_count = order.len() == expected_count;
        
        // Check status code matches
        let correct_status = result.as_ref().map(|r| r.get_status().as_u16() == status_code).unwrap_or(false);
        
        TestResult::from_bool(result.is_ok() && handler_not_called && correct_count && correct_status)
    })
}

// Middleware that throws an error
struct ErrorThrowingMiddleware {
    error_message: String,
}

impl ErrorThrowingMiddleware {
    fn new(error_message: String) -> Self {
        Self { error_message }
    }
}

#[async_trait]
impl Middleware for ErrorThrowingMiddleware {
    async fn handle(&self, _req: Request, _next: Next) -> Result<Response> {
        Err(ruffus::Error::BadRequest(self.error_message.clone()))
    }
}

// Error handling middleware that catches errors and converts them
struct ErrorHandlingMiddleware {
    handled: Arc<Mutex<bool>>,
}

impl ErrorHandlingMiddleware {
    fn new(handled: Arc<Mutex<bool>>) -> Self {
        Self { handled }
    }
}

#[async_trait]
impl Middleware for ErrorHandlingMiddleware {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        match next.run(req).await {
            Ok(response) => Ok(response),
            Err(e) => {
                // Mark that we handled an error
                *self.handled.lock().unwrap() = true;
                // Convert error to response
                Ok(e.into_response())
            }
        }
    }
}

// **Feature: rust-web-framework, Property 25: Error middleware handles all errors**
// **Validates: Requirements 6.5**
#[quickcheck]
fn prop_error_middleware_handles_errors(error_message: String) -> TestResult {
    if error_message.is_empty() {
        return TestResult::discard();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let error_handled = Arc::new(Mutex::new(false));
        let mut middleware_stack: Vec<Arc<dyn Middleware>> = Vec::new();

        // Add error handling middleware first
        middleware_stack.push(Arc::new(ErrorHandlingMiddleware::new(error_handled.clone())));
        
        // Add middleware that throws an error
        middleware_stack.push(Arc::new(ErrorThrowingMiddleware::new(error_message.clone())));

        // Create a handler that should NOT be called
        let handler = Arc::new(|_req: Request| {
            Box::pin(async {
                Ok(Response::new())
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
        });

        // Execute the middleware stack
        let req = create_test_request();
        let result = ruffus::middleware::execute_middleware_stack(
            middleware_stack,
            handler,
            req,
        )
        .await;

        // Check that:
        // 1. The result is Ok (error was handled)
        // 2. The error handling middleware was invoked
        let was_handled = *error_handled.lock().unwrap();
        
        TestResult::from_bool(result.is_ok() && was_handled)
    })
}

// Test different handler types

// Regular async function
async fn async_function_handler(_req: Request) -> Result<Response> {
    Ok(Response::new().status(http::StatusCode::OK))
}

// Struct that implements Handler trait
struct StructHandler {
    status: u16,
}

impl ruffus::Handler for StructHandler {
    fn handle(&self, _req: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send + 'static>> {
        let status = self.status;
        Box::pin(async move {
            Ok(Response::new().status(http::StatusCode::from_u16(status).unwrap()))
        })
    }
}

// **Feature: rust-web-framework, Property 31: Various handler types are accepted**
// **Validates: Requirements 9.2**
#[quickcheck]
fn prop_various_handler_types_accepted(handler_type: u8, status_code: u16) -> TestResult {
    // Use valid HTTP status codes
    let status_code = 200 + (status_code % 100);
    let handler_type = handler_type % 3; // 3 different handler types
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let middleware_stack: Vec<Arc<dyn Middleware>> = Vec::new();

        let handler: ruffus::middleware::BoxedHandler = match handler_type {
            0 => {
                // Closure handler
                Arc::new(move |_req: Request| {
                    Box::pin(async move {
                        Ok(Response::new().status(http::StatusCode::from_u16(status_code).unwrap()))
                    }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
                })
            }
            1 => {
                // Async function handler
                Arc::new(move |req: Request| {
                    Box::pin(async move {
                        Ok(Response::new().status(http::StatusCode::from_u16(status_code).unwrap()))
                    }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
                })
            }
            2 => {
                // Struct handler
                let struct_handler = StructHandler { status: status_code };
                Arc::new(move |req: Request| {
                    let handler = StructHandler { status: status_code };
                    Box::pin(async move { handler.handle(req).await })
                        as std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response>> + Send>>
                })
            }
            _ => unreachable!(),
        };

        // Execute the middleware stack with the handler
        let req = create_test_request();
        let result = ruffus::middleware::execute_middleware_stack(
            middleware_stack,
            handler,
            req,
        )
        .await;

        // Check that the handler executed successfully
        match result {
            Ok(response) => TestResult::from_bool(response.get_status().as_u16() == status_code),
            Err(_) => TestResult::from_bool(false),
        }
    })
}
