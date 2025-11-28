// Property-based tests for Response type

use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use ruffus::Response;
use serde::{Deserialize, Serialize};

// Feature: rust-web-framework, Property 17: JSON responses include correct Content-Type
// Validates: Requirements 5.2
#[quickcheck]
fn prop_json_responses_include_content_type(
    string_field: String,
    number_field: i32,
    bool_field: bool,
) -> TestResult {
    // Create a simple JSON-serializable struct
    #[derive(Serialize)]
    struct TestData {
        string_field: String,
        number_field: i32,
        bool_field: bool,
    }

    let data = TestData {
        string_field,
        number_field,
        bool_field,
    };

    let response = match Response::json(&data) {
        Ok(r) => r,
        Err(_) => return TestResult::discard(),
    };

    let headers = response.get_headers();
    let content_type = headers.get("content-type");

    TestResult::from_bool(
        content_type.is_some()
            && content_type.unwrap().to_str().unwrap() == "application/json",
    )
}

// Feature: rust-web-framework, Property 18: Custom status codes are preserved
// Validates: Requirements 5.3
#[quickcheck]
fn prop_custom_status_codes_are_preserved(status_code: u16) -> TestResult {
    // Only test valid HTTP status codes (100-599)
    if !(100..=599).contains(&status_code) {
        return TestResult::discard();
    }

    let status = match http::StatusCode::from_u16(status_code) {
        Ok(s) => s,
        Err(_) => return TestResult::discard(),
    };

    let response = Response::new().status(status);

    TestResult::from_bool(response.get_status() == status)
}

// Feature: rust-web-framework, Property 19: Custom headers are included in response
// Validates: Requirements 5.4
#[quickcheck]
fn prop_custom_headers_are_included(header_name: String, header_value: String) -> TestResult {
    // Filter out invalid header names and values
    if header_name.is_empty() || header_value.is_empty() {
        return TestResult::discard();
    }

    // Only test with valid ASCII header names (alphanumeric and hyphens)
    if !header_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return TestResult::discard();
    }

    // Only test with valid ASCII header values (printable ASCII)
    if !header_value.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return TestResult::discard();
    }

    let response = Response::new().header(&header_name, &header_value);

    let headers = response.get_headers();
    let retrieved_value = headers.get(&header_name);

    TestResult::from_bool(
        retrieved_value.is_some()
            && retrieved_value.unwrap().to_str().unwrap() == header_value,
    )
}

// Feature: rust-web-framework, Property 20: Serialization failures return 500
// Validates: Requirements 5.5
#[test]
fn prop_serialization_failures_return_500() {
    use std::fmt;

    // Create a type that always fails to serialize
    struct FailingSerialize;

    impl serde::Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("intentional serialization failure"))
        }
    }

    // Attempt to create a JSON response with the failing type
    let result = Response::json(&FailingSerialize);

    // Should return an error
    assert!(result.is_err());

    // The error should be a JsonSerializeError
    match result {
        Err(ruffus::Error::JsonSerializeError(_)) => {
            // Verify that this error maps to status 500
            let error = ruffus::Error::JsonSerializeError(
                serde_json::to_string(&FailingSerialize)
                    .unwrap_err()
            );
            assert_eq!(error.status_code(), http::StatusCode::INTERNAL_SERVER_ERROR);
        }
        _ => panic!("Expected JsonSerializeError"),
    }
}

// Feature: rust-web-framework, Property 33: Response builder methods work correctly
// Validates: Requirements 9.4
#[quickcheck]
fn prop_response_builder_methods_work_correctly(
    status_code: u16,
    header_name: String,
    header_value: String,
    body_text: String,
) -> TestResult {
    // Only test valid HTTP status codes (100-599)
    if !(100..=599).contains(&status_code) {
        return TestResult::discard();
    }

    let status = match http::StatusCode::from_u16(status_code) {
        Ok(s) => s,
        Err(_) => return TestResult::discard(),
    };

    // Filter out invalid header names and values
    if header_name.is_empty() || header_value.is_empty() {
        return TestResult::discard();
    }

    // Only test with valid ASCII header names (alphanumeric and hyphens)
    if !header_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return TestResult::discard();
    }

    // Only test with valid ASCII header values (printable ASCII)
    if !header_value.chars().all(|c| c.is_ascii() && !c.is_control()) {
        return TestResult::discard();
    }

    // Build a response using method chaining
    let response = Response::new()
        .status(status)
        .header(&header_name, &header_value)
        .body(body_text.clone());

    // Verify all modifications are reflected
    let status_matches = response.get_status() == status;
    
    let headers = response.get_headers();
    let header_matches = headers.get(&header_name).is_some()
        && headers.get(&header_name).unwrap().to_str().unwrap() == header_value;
    
    let body_matches = response.get_body().as_ref() == body_text.as_bytes();

    TestResult::from_bool(status_matches && header_matches && body_matches)
}

// Additional test for convenience methods
#[test]
fn prop_convenience_methods_work() {
    // Test html()
    let html_response = Response::html("<h1>Hello</h1>".to_string());
    assert_eq!(html_response.get_status(), http::StatusCode::OK);
    assert_eq!(
        html_response.get_headers().get("content-type").unwrap().to_str().unwrap(),
        "text/html; charset=utf-8"
    );
    assert_eq!(html_response.get_body().as_ref(), b"<h1>Hello</h1>");

    // Test not_found()
    let not_found = Response::not_found();
    assert_eq!(not_found.get_status(), http::StatusCode::NOT_FOUND);

    // Test bad_request()
    let bad_req = Response::bad_request("Invalid input".to_string());
    assert_eq!(bad_req.get_status(), http::StatusCode::BAD_REQUEST);
    assert_eq!(bad_req.get_body().as_ref(), b"Invalid input");

    // Test internal_error()
    let error = Response::internal_error("Server error".to_string());
    assert_eq!(error.get_status(), http::StatusCode::INTERNAL_SERVER_ERROR);

    // Test redirect()
    let redirect = Response::redirect("/new-location");
    assert_eq!(redirect.get_status(), http::StatusCode::FOUND);
    assert_eq!(
        redirect.get_headers().get("location").unwrap().to_str().unwrap(),
        "/new-location"
    );

    // Test no_content()
    let no_content = Response::no_content();
    assert_eq!(no_content.get_status(), http::StatusCode::NO_CONTENT);
    assert!(no_content.get_body().is_empty());

    // Test method chaining with convenience methods
    let chained = Response::html("<p>Test</p>".to_string())
        .status(http::StatusCode::CREATED)
        .header("X-Custom", "value");
    assert_eq!(chained.get_status(), http::StatusCode::CREATED);
    assert_eq!(
        chained.get_headers().get("x-custom").unwrap().to_str().unwrap(),
        "value"
    );
}
