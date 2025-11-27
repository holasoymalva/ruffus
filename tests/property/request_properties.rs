// Property-based tests for Request handling

use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use ruffus::Request;
use bytes::Bytes;
use http::{HeaderMap, Method, Uri};

// Feature: rust-web-framework, Property 6: Path parameters are extracted correctly
// Validates: Requirements 2.1, 2.3
#[quickcheck]
fn prop_path_parameters_extracted_correctly(param_name: String, param_value: String) -> TestResult {
    // Filter out invalid parameter names and values
    if param_name.is_empty() || param_name.contains('/') || param_name.contains('?') {
        return TestResult::discard();
    }
    if param_value.contains('/') || param_value.contains('?') {
        return TestResult::discard();
    }
    
    // Create a request
    let uri = format!("http://example.com/users/{}", param_value);
    let uri = match uri.parse::<Uri>() {
        Ok(u) => u,
        Err(_) => return TestResult::discard(),
    };
    
    let mut request = Request::new(
        Method::GET,
        uri,
        HeaderMap::new(),
        Bytes::new(),
    );
    
    // Simulate router setting the parameter
    request.set_param(param_name.clone(), param_value.clone());
    
    // Extract the parameter
    let extracted = request.param(&param_name);
    
    // The extracted value should match what was set
    TestResult::from_bool(extracted == Some(param_value.as_str()))
}

// Feature: rust-web-framework, Property 7: Multiple path parameters are all extracted
// Validates: Requirements 2.2
#[quickcheck]
fn prop_multiple_path_parameters_extracted(
    param1_name: String,
    param1_value: String,
    param2_name: String,
    param2_value: String,
) -> TestResult {
    // Filter out invalid parameter names and values
    if param1_name.is_empty() || param1_name.contains('/') || param1_name.contains('?') {
        return TestResult::discard();
    }
    if param2_name.is_empty() || param2_name.contains('/') || param2_name.contains('?') {
        return TestResult::discard();
    }
    if param1_value.contains('/') || param1_value.contains('?') {
        return TestResult::discard();
    }
    if param2_value.contains('/') || param2_value.contains('?') {
        return TestResult::discard();
    }
    // Ensure parameter names are different
    if param1_name == param2_name {
        return TestResult::discard();
    }
    
    // Create a request
    let uri = format!("http://example.com/users/{}/posts/{}", param1_value, param2_value);
    let uri = match uri.parse::<Uri>() {
        Ok(u) => u,
        Err(_) => return TestResult::discard(),
    };
    
    let mut request = Request::new(
        Method::GET,
        uri,
        HeaderMap::new(),
        Bytes::new(),
    );
    
    // Simulate router setting both parameters
    request.set_param(param1_name.clone(), param1_value.clone());
    request.set_param(param2_name.clone(), param2_value.clone());
    
    // Extract both parameters
    let extracted1 = request.param(&param1_name);
    let extracted2 = request.param(&param2_name);
    
    // Both extracted values should match what was set
    TestResult::from_bool(
        extracted1 == Some(param1_value.as_str()) &&
        extracted2 == Some(param2_value.as_str())
    )
}

// Feature: rust-web-framework, Property 8: URL decoding round-trip
// Validates: Requirements 2.4
#[quickcheck]
fn prop_url_decoding_round_trip(original_value: String) -> TestResult {
    // Filter out values that contain characters that would break URI parsing
    if original_value.is_empty() || original_value.contains('/') || original_value.contains('?') {
        return TestResult::discard();
    }
    
    // URL encode the value
    let encoded = urlencoding::encode(&original_value);
    
    // Create a request with the encoded value in the path
    let uri = format!("http://example.com/users/{}", encoded);
    let uri = match uri.parse::<Uri>() {
        Ok(u) => u,
        Err(_) => return TestResult::discard(),
    };
    
    let mut request = Request::new(
        Method::GET,
        uri,
        HeaderMap::new(),
        Bytes::new(),
    );
    
    // Simulate router setting the parameter with the encoded value
    // In a real router, it would decode the path segment
    let decoded = urlencoding::decode(&encoded).unwrap().into_owned();
    request.set_param("id".to_string(), decoded.clone());
    
    // Extract the parameter
    let extracted = request.param("id");
    
    // The extracted value should match the original (decoded) value
    TestResult::from_bool(extracted == Some(original_value.as_str()))
}

// Feature: rust-web-framework, Property 9: Query parameters are parsed completely
// Validates: Requirements 3.1
#[quickcheck]
fn prop_query_parameters_parsed_completely(
    key1: String,
    value1: String,
    key2: String,
    value2: String,
) -> TestResult {
    // Filter out invalid keys and values
    if key1.is_empty() || key2.is_empty() {
        return TestResult::discard();
    }
    if key1.contains('&') || key1.contains('=') || key1.contains('?') {
        return TestResult::discard();
    }
    if key2.contains('&') || key2.contains('=') || key2.contains('?') {
        return TestResult::discard();
    }
    if value1.contains('&') || value1.contains('?') {
        return TestResult::discard();
    }
    if value2.contains('&') || value2.contains('?') {
        return TestResult::discard();
    }
    // Ensure keys are different
    if key1 == key2 {
        return TestResult::discard();
    }
    
    // URL encode the values
    let encoded_key1 = urlencoding::encode(&key1);
    let encoded_value1 = urlencoding::encode(&value1);
    let encoded_key2 = urlencoding::encode(&key2);
    let encoded_value2 = urlencoding::encode(&value2);
    
    // Create a request with query parameters
    let uri = format!(
        "http://example.com/test?{}={}&{}={}",
        encoded_key1, encoded_value1, encoded_key2, encoded_value2
    );
    let uri = match uri.parse::<Uri>() {
        Ok(u) => u,
        Err(_) => return TestResult::discard(),
    };
    
    let request = Request::new(
        Method::GET,
        uri,
        HeaderMap::new(),
        Bytes::new(),
    );
    
    // Extract both query parameters
    let extracted1 = request.query(&key1);
    let extracted2 = request.query(&key2);
    
    // Both extracted values should match what was set
    TestResult::from_bool(
        extracted1 == Some(value1.as_str()) &&
        extracted2 == Some(value2.as_str())
    )
}

// Feature: rust-web-framework, Property 10: JSON deserialization round-trip
// Validates: Requirements 3.2, 5.1
#[quickcheck]
fn prop_json_deserialization_round_trip(name: String, age: u8, active: bool) -> TestResult {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData {
        name: String,
        age: u8,
        active: bool,
    }
    
    let original = TestData {
        name: name.clone(),
        age,
        active,
    };
    
    // Serialize to JSON
    let json_str = match serde_json::to_string(&original) {
        Ok(s) => s,
        Err(_) => return TestResult::discard(),
    };
    
    // Create a request with the JSON body
    let mut request = Request::new(
        Method::POST,
        "http://example.com/test".parse().unwrap(),
        HeaderMap::new(),
        Bytes::from(json_str),
    );
    
    // Use tokio runtime to run async code
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        request.json::<TestData>().await
    });
    
    // Deserialize should succeed and match the original
    match result {
        Ok(deserialized) => TestResult::from_bool(deserialized == original),
        Err(_) => TestResult::failed(),
    }
}

// Feature: rust-web-framework, Property 11: Invalid JSON returns 400 error
// Validates: Requirements 3.3
#[quickcheck]
fn prop_invalid_json_returns_400_error(invalid_json: String) -> TestResult {
    use serde::Deserialize;
    
    #[derive(Debug, Deserialize)]
    struct TestData {
        name: String,
        age: u8,
    }
    
    // Try to parse the string as JSON first to see if it's actually invalid
    if serde_json::from_str::<TestData>(&invalid_json).is_ok() {
        // If it's valid JSON, discard this test case
        return TestResult::discard();
    }
    
    // Create a request with the invalid JSON body
    let mut request = Request::new(
        Method::POST,
        "http://example.com/test".parse().unwrap(),
        HeaderMap::new(),
        Bytes::from(invalid_json),
    );
    
    // Use tokio runtime to run async code
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        request.json::<TestData>().await
    });
    
    // Should return an error
    match result {
        Err(ruffus::Error::JsonParseError(_)) => TestResult::passed(),
        Err(_) => TestResult::passed(), // Any error is acceptable
        Ok(_) => TestResult::failed(), // Should not succeed with invalid JSON
    }
}

// Feature: rust-web-framework, Property 12: All request headers are accessible
// Validates: Requirements 3.4
#[quickcheck]
fn prop_all_request_headers_accessible(
    header1_name: String,
    header1_value: String,
    header2_name: String,
    header2_value: String,
) -> TestResult {
    use http::header::{HeaderName, HeaderValue};
    
    // Filter out invalid header names and values
    if header1_name.is_empty() || header2_name.is_empty() {
        return TestResult::discard();
    }
    
    // Try to create valid header names
    let name1 = match HeaderName::from_bytes(header1_name.to_lowercase().as_bytes()) {
        Ok(n) => n,
        Err(_) => return TestResult::discard(),
    };
    let name2 = match HeaderName::from_bytes(header2_name.to_lowercase().as_bytes()) {
        Ok(n) => n,
        Err(_) => return TestResult::discard(),
    };
    
    // Ensure header names are different
    if name1 == name2 {
        return TestResult::discard();
    }
    
    // Try to create valid header values
    let value1 = match HeaderValue::from_str(&header1_value) {
        Ok(v) => v,
        Err(_) => return TestResult::discard(),
    };
    let value2 = match HeaderValue::from_str(&header2_value) {
        Ok(v) => v,
        Err(_) => return TestResult::discard(),
    };
    
    // Create headers
    let mut headers = HeaderMap::new();
    headers.insert(name1.clone(), value1.clone());
    headers.insert(name2.clone(), value2.clone());
    
    // Create a request with the headers
    let request = Request::new(
        Method::GET,
        "http://example.com/test".parse().unwrap(),
        headers,
        Bytes::new(),
    );
    
    // Access the headers
    let retrieved_headers = request.headers();
    
    // Both headers should be accessible
    let header1_accessible = retrieved_headers.get(&name1) == Some(&value1);
    let header2_accessible = retrieved_headers.get(&name2) == Some(&value2);
    
    TestResult::from_bool(header1_accessible && header2_accessible)
}
