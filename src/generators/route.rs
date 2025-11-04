use super::{Generator, GenerationResult};
use crate::error::GenerationError;
use crate::cli::{Framework, HttpMethod};

pub struct RouteGenerator {
    // TODO: Add template engine and file system manager
}

#[derive(Debug)]
pub struct RouteGenerationRequest {
    pub name: String,
    pub path: String,
    pub methods: Vec<HttpMethod>,
    pub middleware: Vec<String>,
    pub service_dependency: Option<String>,
}

impl Generator for RouteGenerator {
    type Request = RouteGenerationRequest;

    async fn generate(&self, _request: Self::Request) -> Result<GenerationResult, GenerationError> {
        // TODO: Implement route generation
        Ok(GenerationResult {
            files_created: vec![],
            files_modified: vec![],
            success: true,
            message: "Route generation not yet implemented".to_string(),
        })
    }

    fn supported_frameworks(&self) -> Vec<Framework> {
        vec![Framework::Axum, Framework::ActixWeb, Framework::Warp, Framework::Rocket]
    }
}