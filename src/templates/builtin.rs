// Built-in templates will be stored here as string constants
// This module will contain the default templates for each framework and component type

pub const AXUM_SERVICE_TEMPLATE: &str = r#"
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct {{pascal_case component_name}}Request {
    // TODO: Define request structure
}

#[derive(Debug, Serialize, Deserialize)]
pub struct {{pascal_case component_name}}Response {
    // TODO: Define response structure
}

pub struct {{pascal_case component_name}}Service {
    // TODO: Add service dependencies
}

impl {{pascal_case component_name}}Service {
    pub fn new() -> Self {
        Self {
            // TODO: Initialize dependencies
        }
    }

    pub async fn handle(&self, request: {{pascal_case component_name}}Request) -> Result<{{pascal_case component_name}}Response, ServiceError> {
        // TODO: Implement service logic
        todo!("Implement service logic")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Internal server error")]
    Internal,
    // TODO: Add specific error types
}
"#;

pub const AXUM_ROUTE_TEMPLATE: &str = r#"
use axum::{extract::State, http::StatusCode, Json, Router, routing::{{http_method}}};
use std::sync::Arc;

use crate::services::{{snake_case component_name}}_service::{{pascal_case component_name}}Service;

pub async fn {{snake_case component_name}}_handler(
    State(service): State<Arc<{{pascal_case component_name}}Service>>,
    Json(request): Json<{{pascal_case component_name}}Request>,
) -> Result<Json<{{pascal_case component_name}}Response>, StatusCode> {
    match service.handle(request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn {{snake_case component_name}}_routes() -> Router<Arc<{{pascal_case component_name}}Service>> {
    Router::new()
        .route("{{route_path}}", {{http_method}}({{snake_case component_name}}_handler))
}
"#;

// TODO: Add templates for other frameworks (Actix-web, Warp, Rocket)
// TODO: Add templates for guards/middleware
// TODO: Add templates for modules