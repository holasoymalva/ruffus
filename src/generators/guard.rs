use super::{Generator, GenerationResult};
use crate::error::GenerationError;
use crate::cli::{Framework, MiddlewareType};

pub struct MiddlewareGenerator {
    // TODO: Add template engine and file system manager
}

#[derive(Debug)]
pub struct MiddlewareGenerationRequest {
    pub name: String,
    pub middleware_type: MiddlewareType,
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Debug)]
pub struct ValidationRule {
    pub field: String,
    pub rule_type: String,
    pub parameters: Vec<String>,
}

impl Generator for MiddlewareGenerator {
    type Request = MiddlewareGenerationRequest;

    async fn generate(&self, _request: Self::Request) -> Result<GenerationResult, GenerationError> {
        // TODO: Implement middleware generation
        Ok(GenerationResult {
            files_created: vec![],
            files_modified: vec![],
            success: true,
            message: "Middleware generation not yet implemented".to_string(),
        })
    }

    fn supported_frameworks(&self) -> Vec<Framework> {
        vec![Framework::Axum, Framework::ActixWeb, Framework::Warp, Framework::Rocket]
    }
}