use super::{Generator, GenerationResult};
use crate::error::GenerationError;
use crate::cli::Framework;

pub struct ServiceGenerator {
    // TODO: Add template engine and file system manager
}

#[derive(Debug)]
pub struct ServiceGenerationRequest {
    pub name: String,
    pub module: Option<String>,
    pub methods: Vec<String>,
    pub dependencies: Vec<String>,
}

impl Generator for ServiceGenerator {
    type Request = ServiceGenerationRequest;

    async fn generate(&self, _request: Self::Request) -> Result<GenerationResult, GenerationError> {
        // TODO: Implement service generation
        Ok(GenerationResult {
            files_created: vec![],
            files_modified: vec![],
            success: true,
            message: "Service generation not yet implemented".to_string(),
        })
    }

    fn supported_frameworks(&self) -> Vec<Framework> {
        vec![Framework::Axum, Framework::ActixWeb, Framework::Warp, Framework::Rocket]
    }
}