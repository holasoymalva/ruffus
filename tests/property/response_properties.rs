// Property-based tests for Response type

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
