pub mod service;
pub mod route;
pub mod guard;
pub mod module;

use crate::error::GenerationError;
use crate::cli::Framework;

#[derive(Debug)]
pub struct GenerationResult {
    pub files_created: Vec<String>,
    pub files_modified: Vec<String>,
    pub success: bool,
    pub message: String,
}

pub trait Generator {
    type Request;
    
    async fn generate(&self, request: Self::Request) -> Result<GenerationResult, GenerationError>;
    fn supported_frameworks(&self) -> Vec<Framework>;
}