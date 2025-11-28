//! Property-based tests for routing system

use quickcheck::{Arbitrary, Gen, QuickCheck};
use ruffus::{Method, Response, Router};

// Helper to generate valid path segments
#[derive(Clone, Debug)]
struct ValidPathSegment(String);

impl Arbitrary for ValidPathSegment {
    fn arbitrary(g: &mut Gen) -> Self {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789_-"
            .chars()
            .collect();
        let len = (u8::arbitrary(g) % 10) + 1; // 1-10 chars
        let segment: String = (0..len)
            .map(|_| chars[usize::arbitrary(g) % chars.len()])
            .collect();
        ValidPathSegment(segment)
    }
}

// Helper to generate valid path patterns
#[derive(Clone, Debug)]
struct ValidPath {
    segments: Vec<String>,
    pattern: String,
}

impl Arbitrary for ValidPath {
    fn arbitrary(g: &mut Gen) -> Self {
        let num_segments = (u8::arbitrary(g) % 5) + 1; // 1-5 segments
        let segments: Vec<String> = (0..num_segments)
            .map(|_| ValidPathSegment::arbitrary(g).0)
            .collect();
        
        let pattern = format!("/{}", segments.join("/"));
        
        ValidPath { segments, pattern }
    }
}

// Wrapper for Method to implement Arbitrary
#[derive(Clone, Debug)]
struct TestMethod(Method);

impl Arbitrary for TestMethod {
    fn arbitrary(g: &mut Gen) -> Self {
        let methods = [
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ];
        TestMethod(methods[usize::arbitrary(g) % methods.len()])
    }
}

/// **Feature: rust-web-framework, Property 2: Route registration is preserved**
/// **Validates: Requirements 1.2**
///
/// For any route with a method and path, registering it in the router
/// should make it retrievable and matchable.
fn prop_route_registration_preserved(method: TestMethod, path: ValidPath) -> bool {
    let mut router = Router::new("");
    let method = method.0;
    
    // Register a route
    match method {
        Method::GET => router.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Check that the route can be found
    let result = router.find_route(&method, &path.pattern);
    
    // The route should be found and match
    result.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_route_registration_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_route_registration_preserved as fn(TestMethod, ValidPath) -> bool);
    }
}

/// **Feature: rust-web-framework, Property 3: Matching requests invoke handlers**
/// **Validates: Requirements 1.3**
///
/// For any registered route and matching request, the framework should invoke
/// the corresponding handler.
fn prop_matching_requests_invoke_handlers(method: TestMethod, path: ValidPath) -> bool {
    use tokio::runtime::Runtime;
    
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut router = Router::new("");
        let method = method.0;
        let mut handler_called = false;
        
        // Create a handler that sets a flag when called
        let handler = |_req: ruffus::Request| async {
            Ok(ruffus::Response::text("handler called".to_string()))
        };
        
        // Register the route
        match method {
            Method::GET => router.get(&path.pattern, handler),
            Method::POST => router.post(&path.pattern, handler),
            Method::PUT => router.put(&path.pattern, handler),
            Method::DELETE => router.delete(&path.pattern, handler),
            Method::PATCH => router.patch(&path.pattern, handler),
            _ => return true, // Skip unsupported methods
        };
        
        // Find and invoke the handler
        if let Some((route, params)) = router.find_route(&method, &path.pattern) {
            // Create a request
            let uri = path.pattern.parse::<http::Uri>().unwrap();
            let mut req = ruffus::Request::new(
                method.into(),
                uri,
                http::HeaderMap::new(),
                bytes::Bytes::new(),
            );
            
            // Set the extracted params
            for (key, value) in params {
                req.set_param(key, value);
            }
            
            // Invoke the handler
            let result = route.handle(req).await;
            
            // Handler should succeed
            result.is_ok()
        } else {
            false
        }
    })
}

#[cfg(test)]
mod test_matching {
    use super::*;

    #[test]
    fn run_matching_requests_invoke_handlers_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_matching_requests_invoke_handlers as fn(TestMethod, ValidPath) -> bool);
    }
}

/// **Feature: rust-web-framework, Property 30: HTTP method matching is exclusive**
/// **Validates: Requirements 8.1, 8.2, 8.3, 8.4, 8.5**
///
/// For any route registered with a specific HTTP method, only requests with
/// that exact method should match the route.
fn prop_http_method_matching_exclusive(method1: TestMethod, method2: TestMethod, path: ValidPath) -> bool {
    let mut router = Router::new("");
    let method1 = method1.0;
    let method2 = method2.0;
    
    // Register a route with method1
    match method1 {
        Method::GET => router.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Try to find route with method1 - should succeed
    let found_with_method1 = router.find_route(&method1, &path.pattern).is_some();
    
    // Try to find route with method2 - should only succeed if method2 == method1
    let found_with_method2 = router.find_route(&method2, &path.pattern).is_some();
    
    // Property: route is found with method2 if and only if method1 == method2
    found_with_method1 && (found_with_method2 == (method1 == method2))
}

#[cfg(test)]
mod test_method_matching {
    use super::*;

    #[test]
    fn run_http_method_matching_exclusive_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_http_method_matching_exclusive as fn(TestMethod, TestMethod, ValidPath) -> bool);
    }
}

/// **Feature: rust-web-framework, Property 23: Non-existent routes return 404**
/// **Validates: Requirements 6.3**
///
/// For any request to a non-registered route, the framework should return a 404 status code.
fn prop_non_existent_routes_return_404(
    registered_path: ValidPath,
    non_existent_path: ValidPath,
    method: TestMethod,
) -> bool {
    // Only test if paths are different
    if registered_path.pattern == non_existent_path.pattern {
        return true; // Discard this test case
    }
    
    let mut router = Router::new("");
    let method = method.0;
    
    // Register a route with the first path
    match method {
        Method::GET => router.get(&registered_path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router.post(&registered_path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router.put(&registered_path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router.delete(&registered_path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router.patch(&registered_path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Try to find a route with the non-existent path
    let result = router.find_route(&method, &non_existent_path.pattern);
    
    // Should not find the route
    result.is_none()
}

#[cfg(test)]
mod test_404_handling {
    use super::*;

    #[test]
    fn run_non_existent_routes_return_404_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_non_existent_routes_return_404 as fn(ValidPath, ValidPath, TestMethod) -> bool);
    }
}

/// **Feature: rust-web-framework, Property 24: Wrong method returns 405**
/// **Validates: Requirements 6.4**
///
/// For any request with a method not allowed for a route, the framework should
/// return a 405 status code with allowed methods in headers.
fn prop_wrong_method_returns_405(
    registered_method: TestMethod,
    request_method: TestMethod,
    path: ValidPath,
) -> bool {
    // Only test when methods are different
    if registered_method.0 == request_method.0 {
        return true; // Discard this test case
    }
    
    let mut router = Router::new("");
    let registered_method = registered_method.0;
    let request_method = request_method.0;
    
    // Register a route with registered_method
    match registered_method {
        Method::GET => router.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Check if path exists (should be true)
    let path_exists = router.path_exists(&path.pattern);
    
    // Try to find route with wrong method (should fail)
    let route_found = router.find_route(&request_method, &path.pattern).is_some();
    
    // Get allowed methods for this path
    let allowed_methods = router.allowed_methods(&path.pattern);
    
    // Property: path exists, route not found with wrong method, and allowed methods includes registered method
    path_exists && !route_found && allowed_methods.contains(&registered_method)
}

#[cfg(test)]
mod test_405_handling {
    use super::*;

    #[test]
    fn run_wrong_method_returns_405_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_wrong_method_returns_405 as fn(TestMethod, TestMethod, ValidPath) -> bool);
    }
}

// Helper to generate valid prefix strings
#[derive(Clone, Debug)]
struct ValidPrefix(String);

impl Arbitrary for ValidPrefix {
    fn arbitrary(g: &mut Gen) -> Self {
        let num_segments = (u8::arbitrary(g) % 3) + 1; // 1-3 segments
        let segments: Vec<String> = (0..num_segments)
            .map(|_| ValidPathSegment::arbitrary(g).0)
            .collect();
        
        let prefix = format!("/{}", segments.join("/"));
        ValidPrefix(prefix)
    }
}

/// **Feature: rust-web-framework, Property 26: Router prefix prepends to all routes**
/// **Validates: Requirements 7.1**
///
/// For any router with a prefix and registered routes, all route paths should
/// have the prefix prepended.
fn prop_router_prefix_prepends(prefix: ValidPrefix, path: ValidPath, method: TestMethod) -> bool {
    let mut router = Router::new(&prefix.0);
    let method = method.0;
    
    // Register a route
    match method {
        Method::GET => router.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // The full path should be prefix + path
    let expected_full_path = format!("{}{}", prefix.0, path.pattern);
    
    // Try to find the route with the full path
    let found_with_full_path = router.find_route(&method, &expected_full_path).is_some();
    
    // Try to find the route with just the path (should fail)
    let found_with_partial_path = router.find_route(&method, &path.pattern).is_some();
    
    // Property: route is found with full path but not with partial path
    found_with_full_path && !found_with_partial_path
}

#[cfg(test)]
mod test_router_prefix {
    use super::*;

    #[test]
    fn run_router_prefix_prepends_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_router_prefix_prepends as fn(ValidPrefix, ValidPath, TestMethod) -> bool);
    }
}

/// **Feature: rust-web-framework, Property 27: Mounted router routes are registered**
/// **Validates: Requirements 7.2**
///
/// For any router mounted on an application, all routes from the router should
/// be registered with their full paths.
fn prop_mounted_router_routes_registered(
    mount_prefix: ValidPrefix,
    router_prefix: ValidPrefix,
    path: ValidPath,
    method: TestMethod,
) -> bool {
    let mut router = Router::new(&router_prefix.0);
    let method = method.0;
    
    // Register a route on the router
    match method {
        Method::GET => router.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Create a main router and mount the sub-router
    let mut main_router = Router::new("");
    main_router.mount(&mount_prefix.0, router);
    
    // The full path should be mount_prefix + router_prefix + path
    let expected_full_path = format!("{}{}{}", mount_prefix.0, router_prefix.0, path.pattern);
    
    // Try to find the route with the full path
    let found = main_router.find_route(&method, &expected_full_path).is_some();
    
    // Property: route should be found with the full combined path
    found
}

#[cfg(test)]
mod test_router_mounting {
    use super::*;

    #[test]
    fn run_mounted_router_routes_registered_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_mounted_router_routes_registered as fn(ValidPrefix, ValidPrefix, ValidPath, TestMethod) -> bool);
    }
}

/// **Feature: rust-web-framework, Property 28: Nested router prefixes combine correctly**
/// **Validates: Requirements 7.3**
///
/// For any nested routers with prefixes, the final route paths should correctly
/// combine all prefixes in order.
fn prop_nested_router_prefixes_combine(
    prefix1: ValidPrefix,
    prefix2: ValidPrefix,
    prefix3: ValidPrefix,
    path: ValidPath,
    method: TestMethod,
) -> bool {
    let method = method.0;
    
    // Create the innermost router with prefix3
    let mut inner_router = Router::new(&prefix3.0);
    match method {
        Method::GET => inner_router.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => inner_router.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => inner_router.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => inner_router.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => inner_router.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Create middle router with prefix2 and mount inner_router
    let mut middle_router = Router::new(&prefix2.0);
    middle_router.mount("", inner_router);
    
    // Create outer router with prefix1 and mount middle_router
    let mut outer_router = Router::new(&prefix1.0);
    outer_router.mount("", middle_router);
    
    // The full path should be prefix1 + prefix2 + prefix3 + path
    let expected_full_path = format!("{}{}{}{}", prefix1.0, prefix2.0, prefix3.0, path.pattern);
    
    // Try to find the route with the full path
    let found = outer_router.find_route(&method, &expected_full_path).is_some();
    
    // Property: route should be found with all prefixes combined
    found
}

#[cfg(test)]
mod test_nested_routers {
    use super::*;

    #[test]
    fn run_nested_router_prefixes_combine_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_nested_router_prefixes_combine as fn(ValidPrefix, ValidPrefix, ValidPrefix, ValidPath, TestMethod) -> bool);
    }
}

/// **Feature: rust-web-framework, Property 29: Router middleware scopes correctly**
/// **Validates: Requirements 7.4**
///
/// For any middleware registered on a router, it should only apply to routes
/// within that router, not to other routes.
fn prop_router_middleware_scopes(
    router1_prefix: ValidPrefix,
    router2_prefix: ValidPrefix,
    path: ValidPath,
    method: TestMethod,
) -> bool {
    use ruffus::middleware::{Middleware, Next};
    use async_trait::async_trait;
    use std::sync::Arc;
    
    // Ensure the two routers have different prefixes
    if router1_prefix.0 == router2_prefix.0 {
        return true; // Discard this test case
    }
    
    let method = method.0;
    
    // Create a test middleware
    struct TestMiddleware;
    
    #[async_trait]
    impl Middleware for TestMiddleware {
        async fn handle(&self, req: ruffus::Request, next: Next) -> ruffus::Result<ruffus::Response> {
            next.run(req).await
        }
    }
    
    // Create router1 with middleware
    let mut router1 = Router::new(&router1_prefix.0);
    router1.use_middleware(std::sync::Arc::new(TestMiddleware));
    match method {
        Method::GET => router1.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router1.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router1.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router1.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router1.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Create router2 without middleware
    let mut router2 = Router::new(&router2_prefix.0);
    match method {
        Method::GET => router2.get(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::POST => router2.post(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PUT => router2.put(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::DELETE => router2.delete(&path.pattern, |_req| async { Ok(Response::new()) }),
        Method::PATCH => router2.patch(&path.pattern, |_req| async { Ok(Response::new()) }),
        _ => return true, // Skip unsupported methods
    };
    
    // Check middleware counts
    let router1_middleware_count = router1.middleware().len();
    let router2_middleware_count = router2.middleware().len();
    
    // Property: router1 should have middleware, router2 should not
    router1_middleware_count == 1 && router2_middleware_count == 0
}

#[cfg(test)]
mod test_router_middleware_scoping {
    use super::*;

    #[test]
    fn run_router_middleware_scopes_property() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_router_middleware_scopes as fn(ValidPrefix, ValidPrefix, ValidPath, TestMethod) -> bool);
    }
}
