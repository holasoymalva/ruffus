//! Property-based tests for Ruffus

mod property {
    include!("property/error_properties.rs");
    include!("property/request_properties.rs");
}
