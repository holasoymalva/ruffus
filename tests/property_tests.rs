//! Property-based tests for Ruffus

#[path = "property/app_properties.rs"]
mod app_properties;

#[path = "property/error_properties.rs"]
mod error_properties;

#[path = "property/request_properties.rs"]
mod request_properties;

#[path = "property/response_properties.rs"]
mod response_properties;

#[path = "property/router_properties.rs"]
mod router_properties;

#[path = "property/middleware_properties.rs"]
mod middleware_properties;
