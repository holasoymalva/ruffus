use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    Configuration(#[from] ConfigError),
    
    #[error("Template error: {0}")]
    Template(#[from] TemplateError),
    
    #[error("File system error: {0}")]
    FileSystem(#[from] FileSystemError),
    
    #[error("Generation error: {0}")]
    Generation(#[from] GenerationError),
    
    #[error("Framework detection error: {0}")]
    Detection(#[from] DetectionError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Missing configuration file: {0}")]
    MissingConfig(String),
}

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Template not found for framework {framework} and component {component}")]
    TemplateNotFound { framework: String, component: String },
    
    #[error("Failed to render template: {0}")]
    RenderError(String),
    
    #[error("Template validation failed: {0}")]
    ValidationError(String),
    
    #[error("Failed to register template: {0}")]
    RegistrationError(String),
    
    #[error("Invalid template syntax: {0}")]
    SyntaxError(String),
}

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("File already exists: {0}")]
    FileExists(String),
    
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Path traversal detected: {0}")]
    PathTraversal(String),
}

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Component '{0}' already exists")]
    ComponentExists(String),
    
    #[error("Invalid component name: {0}")]
    InvalidName(String),
    
    #[error("Template not found for framework {framework} and component {component}")]
    TemplateNotFound { framework: String, component: String },
    
    #[error("Failed to render template: {0}")]
    TemplateRender(String),
    
    #[error("Failed to create file: {0}")]
    FileCreation(String),
    
    #[error("Failed to update module file: {0}")]
    ModuleUpdate(String),
    
    #[error("Invalid module structure: {0}")]
    InvalidModuleStructure(String),
}

#[derive(Debug, Error)]
pub enum DetectionError {
    #[error("No framework detected in project")]
    NoFrameworkDetected,
    
    #[error("Multiple frameworks detected: {0:?}")]
    MultipleFrameworks(Vec<String>),
    
    #[error("Unsupported framework: {0}")]
    UnsupportedFramework(String),
    
    #[error("Failed to read Cargo.toml: {0}")]
    CargoTomlError(String),
    
    #[error("Invalid project structure: {0}")]
    InvalidProjectStructure(String),
}