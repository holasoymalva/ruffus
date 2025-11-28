# Implementation Plan: Ruffus

This implementation plan outlines the tasks to build **Ruffus**, a fast, minimalist web framework for Rust inspired by Express.js.

- [x] 1. Set up project structure and dependencies
  - Create Cargo.toml with all required dependencies (tokio, hyper, serde, async-trait, bytes, http)
  - Add dev dependencies (quickcheck, quickcheck_macros)
  - Create module structure: lib.rs, app.rs, router.rs, request.rs, response.rs, middleware.rs, error.rs
  - Set up test directories: tests/unit/ and tests/property/
  - _Requirements: 1.1_

- [x] 2. Implement core error types
  - Define Error enum with all error variants (RouteNotFound, MethodNotAllowed, BadRequest, etc.)
  - Implement Display and std::error::Error traits for Error type
  - Implement status_code() method to map errors to HTTP status codes
  - Implement into_response() method to convert errors to HTTP responses
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [x] 2.1 Write property test for error conversion
  - **Property 21: Handler errors convert to HTTP responses**
  - **Validates: Requirements 6.1**

- [x] 2.2 Write property test for unhandled errors
  - **Property 22: Unhandled errors return 500**
  - **Validates: Requirements 6.2**

- [x] 3. Implement HTTP Method enum
  - Define Method enum with variants (GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD)
  - Implement conversion from hyper::Method
  - Implement Display and comparison traits
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [x] 4. Implement Request type
  - Create Request struct with fields: method, uri, headers, body, params, query, extensions
  - Implement method(), uri(), headers() accessor methods
  - Implement param() method to get path parameters
  - Implement query() method to get query parameters
  - Implement async json() method for JSON deserialization
  - Implement conversion from hyper::Request
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_

- [x] 4.1 Write property test for path parameter extraction
  - **Property 6: Path parameters are extracted correctly**
  - **Validates: Requirements 2.1, 2.3**

- [x] 4.2 Write property test for multiple path parameters
  - **Property 7: Multiple path parameters are all extracted**
  - **Validates: Requirements 2.2**

- [x] 4.3 Write property test for URL decoding
  - **Property 8: URL decoding round-trip**
  - **Validates: Requirements 2.4**

- [x] 4.4 Write property test for query parameter parsing
  - **Property 9: Query parameters are parsed completely**
  - **Validates: Requirements 3.1**

- [x] 4.5 Write property test for JSON deserialization
  - **Property 10: JSON deserialization round-trip**
  - **Validates: Requirements 3.2, 5.1**

- [x] 4.6 Write property test for invalid JSON handling
  - **Property 11: Invalid JSON returns 400 error**
  - **Validates: Requirements 3.3**

- [x] 4.7 Write property test for header access
  - **Property 12: All request headers are accessible**
  - **Validates: Requirements 3.4**

- [x] 5. Implement Response type
  - Create Response struct with fields: status, headers, body
  - Implement new(), status(), header() builder methods
  - Implement json() method for automatic JSON serialization
  - Implement text() method for plain text responses
  - Implement conversion to hyper::Response
  - _Requirements: 1.4, 5.1, 5.2, 5.3, 5.4, 5.5_

- [x] 5.1 Write property test for JSON Content-Type header
  - **Property 17: JSON responses include correct Content-Type**
  - **Validates: Requirements 5.2**

- [x] 5.2 Write property test for custom status codes
  - **Property 18: Custom status codes are preserved**
  - **Validates: Requirements 5.3**

- [x] 5.3 Write property test for custom headers
  - **Property 19: Custom headers are included in response**
  - **Validates: Requirements 5.4**

- [x] 5.4 Write property test for serialization failures
  - **Property 20: Serialization failures return 500**
  - **Validates: Requirements 5.5**

- [x] 6. Implement routing system
  - Create PathPattern struct to represent route patterns with static and dynamic segments
  - Implement pattern parsing to extract parameter names from `:param` syntax
  - Implement pattern matching algorithm to match URLs against patterns
  - Create Route struct with method, pattern, and handler
  - Implement route matching logic that extracts path parameters
  - _Requirements: 1.2, 1.3, 2.1, 2.2, 2.3, 2.4_

- [x] 6.1 Write property test for route registration
  - **Property 2: Route registration is preserved**
  - **Validates: Requirements 1.2**

- [x] 6.2 Write property test for route matching
  - **Property 3: Matching requests invoke handlers**
  - **Validates: Requirements 1.3**

- [x] 6.3 Write property test for HTTP method matching
  - **Property 30: HTTP method matching is exclusive**
  - **Validates: Requirements 8.1, 8.2, 8.3, 8.4, 8.5**

- [x] 6.4 Write property test for 404 handling
  - **Property 23: Non-existent routes return 404**
  - **Validates: Requirements 6.3**

- [x] 6.5 Write property test for 405 handling
  - **Property 24: Wrong method returns 405**
  - **Validates: Requirements 6.4**

- [x] 7. Implement Handler trait and middleware system
  - Define Handler trait with async handle method
  - Implement Handler for async closures and functions
  - Define Middleware trait with async handle method
  - Create Next struct for middleware chain continuation
  - Implement middleware stack execution logic
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 9.2_

- [x] 7.1 Write property test for middleware order preservation
  - **Property 13: Middleware registration order is preserved**
  - **Validates: Requirements 4.1**

- [x] 7.2 Write property test for middleware execution order
  - **Property 14: Middleware executes in order**
  - **Validates: Requirements 4.2, 4.5**

- [x] 7.3 Write property test for request modification propagation
  - **Property 15: Request modifications propagate through chain**
  - **Validates: Requirements 4.3**

- [x] 7.4 Write property test for early middleware return
  - **Property 16: Early middleware return skips remaining chain**
  - **Validates: Requirements 4.4**

- [x] 7.5 Write property test for error middleware
  - **Property 25: Error middleware handles all errors**
  - **Validates: Requirements 6.5**

- [x] 7.6 Write property test for handler types
  - **Property 31: Various handler types are accepted**
  - **Validates: Requirements 9.2**

- [ ] 8. Implement Router with prefix support
  - Create Router struct with prefix, routes, and middleware fields
  - Implement get(), post(), put(), delete(), patch() methods for route registration
  - Implement use_middleware() method for router-scoped middleware
  - Implement prefix prepending logic for all registered routes
  - Implement route collection method to get all routes with full paths
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

- [ ] 8.1 Write property test for router prefix prepending
  - **Property 26: Router prefix prepends to all routes**
  - **Validates: Requirements 7.1**

- [ ] 8.2 Write property test for router mounting
  - **Property 27: Mounted router routes are registered**
  - **Validates: Requirements 7.2**

- [ ] 8.3 Write property test for nested routers
  - **Property 28: Nested router prefixes combine correctly**
  - **Validates: Requirements 7.3**

- [ ] 8.4 Write property test for router middleware scoping
  - **Property 29: Router middleware scopes correctly**
  - **Validates: Requirements 7.4**

- [ ] 9. Implement App (Application) type
  - Create App struct with router and middleware stack
  - Implement new() constructor
  - Implement get(), post(), put(), delete(), patch() methods that delegate to internal router
  - Implement use_middleware() method for global middleware
  - Implement mount() method to mount routers with prefixes
  - Implement request handling pipeline that executes middleware then routes to handlers
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 7.2_

- [ ] 9.1 Write property test for app initialization
  - **Property 1: Application initialization creates empty state**
  - **Validates: Requirements 1.1**

- [ ] 9.2 Write property test for handler response sending
  - **Property 4: Handler responses are sent correctly**
  - **Validates: Requirements 1.4**

- [ ] 10. Implement server listening and async runtime integration
  - Implement listen() method that binds to address and starts accepting connections
  - Integrate with Hyper's server builder
  - Set up Tokio runtime for async execution
  - Implement connection handling loop
  - Wire up request pipeline: receive request → execute middleware → route to handler → send response
  - _Requirements: 1.5, 10.1, 10.2, 10.3_

- [ ] 10.1 Write property test for server binding
  - **Property 5: Server binds to specified port**
  - **Validates: Requirements 1.5**

- [ ] 10.2 Write property test for async handler execution
  - **Property 34: Async handlers execute asynchronously**
  - **Validates: Requirements 10.1**

- [ ] 10.3 Write property test for concurrent request handling
  - **Property 35: Concurrent requests are handled concurrently**
  - **Validates: Requirements 10.2**

- [ ] 10.4 Write property test for async middleware completion
  - **Property 36: Async middleware completes before proceeding**
  - **Validates: Requirements 10.3**

- [ ] 11. Implement extractor patterns
  - Create Path<T> extractor for path parameters
  - Create Json<T> extractor for JSON body
  - Create Query<T> extractor for query parameters
  - Define FromRequest trait for extractors
  - Implement FromRequest for Path, Json, and Query extractors
  - _Requirements: 9.3_

- [ ] 11.1 Write property test for extractors
  - **Property 32: Extractors work with various types**
  - **Validates: Requirements 9.3**

- [ ] 12. Implement response builder pattern
  - Enhance Response with fluent builder methods
  - Implement method chaining for status(), header(), and body()
  - Add convenience methods for common response types
  - _Requirements: 9.4_

- [ ] 12.1 Write property test for response builder
  - **Property 33: Response builder methods work correctly**
  - **Validates: Requirements 9.4**

- [ ] 13. Create comprehensive examples
  - Create examples/basic.rs with simple hello world server
  - Create examples/json_api.rs with JSON request/response handling
  - Create examples/middleware.rs demonstrating middleware usage
  - Create examples/router.rs showing router organization
  - Create examples/full_api.rs with a complete REST API example
  - _Requirements: All_

- [ ] 14. Write documentation
  - Add module-level documentation for all public modules
  - Add doc comments for all public types, traits, and methods
  - Include usage examples in doc comments
  - Create README.md with quick start guide and feature overview
  - _Requirements: All_

- [ ] 15. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
