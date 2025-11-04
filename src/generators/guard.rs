use super::{Generator, GenerationResult};
use crate::error::GenerationError;
use crate::cli::{Framework, GuardType};

pub struct GuardGenerator {
    // TODO: Add template engine and file system manager
}

#[derive(Debug)]
pub struct GuardGenerationRequest {
    pub name: String,
    pub guard_type: GuardType,
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Debug)]
pub struct ValidationRule {
    pub field: String,
    pub rule_type: String,
    pub parameters: Vec<String>,
}

impl Generator for GuardGenerator {
    type Request = GuardGenerationRequest;

    async fn generate(&self, _request: Self::Request) -> Result<GenerationResult, GenerationError> {
        // TODO: Implement middleware generation
        Ok(GenerationResult {
            files_created: vec![],
            files_modified: vec![],
            success: true,
            message: "Guard generation not yet implemented".to_string(),
        })
    }

    fn supported_frameworks(&self) -> Vec<Framework> {
        vec![Framework::Axum, Framework::ActixWeb, Framework::Warp, Framework::Rocket]
    }
}