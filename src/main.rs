use clap::Parser;
use std::process;

mod cli;
mod generators;
mod templates;
mod config;
mod error;
mod filesystem;
mod detector;

use cli::{Commands, GenerateComponent, ConfigAction, ComponentType, GuardType};
use error::{CliError, GenerationError};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let commands = Commands::parse();

    if let Err(e) = run(commands).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run(commands: Commands) -> Result<(), CliError> {
    match commands {
        Commands::Init { framework, name } => {
            handle_init_command(framework, name).await
        }
        Commands::Generate { component } => {
            handle_generate_command(component).await
        }
        Commands::Config { action } => {
            handle_config_command(action).await
        }
    }
}

/// Handle the init command for initializing a new project
async fn handle_init_command(
    framework: Option<cli::Framework>,
    name: String,
) -> Result<(), CliError> {
    // Validate project name
    validate_component_name(&name)?;
    
    println!("Initializing project '{}' with framework {:?}", name, framework);
    
    // TODO: Implement actual project initialization logic
    // This will be implemented in a future task
    
    Ok(())
}

/// Handle the generate command for creating components
async fn handle_generate_command(component: GenerateComponent) -> Result<(), CliError> {
    match component {
        GenerateComponent::Service { name, module, methods, dependencies } => {
            handle_generate_service(name, module, methods, dependencies).await
        }
        GenerateComponent::Route { name, methods, path, middleware, service_dependency } => {
            handle_generate_route(name, methods, path, middleware, service_dependency).await
        }
        GenerateComponent::Guard { name, guard_type, validation_rules } => {
            handle_generate_guard(name, guard_type, validation_rules).await
        }
        GenerateComponent::Module { name, components, dependencies } => {
            handle_generate_module(name, components, dependencies).await
        }
    }
}

/// Handle service generation
async fn handle_generate_service(
    name: String,
    module: Option<String>,
    methods: Vec<String>,
    dependencies: Vec<String>,
) -> Result<(), CliError> {
    // Validate service name
    validate_component_name(&name)?;
    
    // Validate module name if provided
    if let Some(ref module_name) = module {
        validate_component_name(module_name)?;
    }
    
    // Validate method names
    for method in &methods {
        validate_method_name(method)?;
    }
    
    println!("Generating service '{}' in module {:?}", name, module);
    println!("Methods: {:?}", methods);
    println!("Dependencies: {:?}", dependencies);
    
    // TODO: Implement actual service generation logic
    // This will be implemented in a future task
    
    Ok(())
}

/// Handle route generation
async fn handle_generate_route(
    name: String,
    methods: Vec<cli::HttpMethod>,
    path: String,
    middleware: Vec<String>,
    service_dependency: Option<String>,
) -> Result<(), CliError> {
    // Validate route name
    validate_component_name(&name)?;
    
    // Validate path
    validate_route_path(&path)?;
    
    // Validate service dependency if provided
    if let Some(ref service_name) = service_dependency {
        validate_component_name(service_name)?;
    }
    
    // Validate middleware names
    for middleware_name in &middleware {
        validate_component_name(middleware_name)?;
    }
    
    println!("Generating route '{}' with path '{}'", name, path);
    println!("HTTP methods: {:?}", methods);
    println!("Middleware: {:?}", middleware);
    println!("Service dependency: {:?}", service_dependency);
    
    // TODO: Implement actual route generation logic
    // This will be implemented in a future task
    
    Ok(())
}

/// Handle guard generation
async fn handle_generate_guard(
    name: String,
    guard_type: GuardType,
    validation_rules: Vec<String>,
) -> Result<(), CliError> {
    // Validate guard name
    validate_component_name(&name)?;
    
    println!("Generating guard '{}' of type {:?}", name, guard_type);
    println!("Validation rules: {:?}", validation_rules);
    
    // TODO: Implement actual guard generation logic
    // This will be implemented in a future task
    
    Ok(())
}

/// Handle module generation
async fn handle_generate_module(
    name: String,
    components: Vec<ComponentType>,
    dependencies: Vec<String>,
) -> Result<(), CliError> {
    // Validate module name
    validate_component_name(&name)?;
    
    // Validate dependency names
    for dependency in &dependencies {
        validate_component_name(dependency)?;
    }
    
    println!("Generating module '{}' with components {:?}", name, components);
    println!("Dependencies: {:?}", dependencies);
    
    // TODO: Implement actual module generation logic
    // This will be implemented in a future task
    
    Ok(())
}

/// Handle config command
async fn handle_config_command(action: ConfigAction) -> Result<(), CliError> {
    match action {
        ConfigAction::Set { key, value } => {
            // Validate config key
            validate_config_key(&key)?;
            
            println!("Setting config '{}' to '{}'", key, value);
            
            // TODO: Implement actual config setting logic
            // This will be implemented in a future task
        }
        ConfigAction::Get { key } => {
            // Validate config key
            validate_config_key(&key)?;
            
            println!("Getting config '{}'", key);
            
            // TODO: Implement actual config getting logic
            // This will be implemented in a future task
        }
        ConfigAction::List => {
            println!("Listing all configuration values");
            
            // TODO: Implement actual config listing logic
            // This will be implemented in a future task
        }
    }
    
    Ok(())
}

/// Validate component names (services, routes, guards, modules)
fn validate_component_name(name: &str) -> Result<(), GenerationError> {
    if name.is_empty() {
        return Err(GenerationError::InvalidName("Component name cannot be empty".to_string()));
    }
    
    // Check for valid Rust identifier
    if !name.chars().next().unwrap_or('0').is_ascii_alphabetic() && name.chars().next() != Some('_') {
        return Err(GenerationError::InvalidName(
            "Component name must start with a letter or underscore".to_string()
        ));
    }
    
    // Check that all characters are valid for Rust identifiers
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(GenerationError::InvalidName(
            "Component name can only contain letters, numbers, and underscores".to_string()
        ));
    }
    
    // Check for reserved Rust keywords
    const RUST_KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
        "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
        "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
        "async", "await", "dyn", "abstract", "become", "box", "do", "final",
        "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try"
    ];
    
    if RUST_KEYWORDS.contains(&name) {
        return Err(GenerationError::InvalidName(
            format!("'{}' is a reserved Rust keyword", name)
        ));
    }
    
    Ok(())
}

/// Validate method names for services
fn validate_method_name(name: &str) -> Result<(), GenerationError> {
    if name.is_empty() {
        return Err(GenerationError::InvalidName("Method name cannot be empty".to_string()));
    }
    
    // Use the same validation as component names
    validate_component_name(name)
}

/// Validate route paths
fn validate_route_path(path: &str) -> Result<(), GenerationError> {
    if path.is_empty() {
        return Err(GenerationError::InvalidName("Route path cannot be empty".to_string()));
    }
    
    if !path.starts_with('/') {
        return Err(GenerationError::InvalidName("Route path must start with '/'".to_string()));
    }
    
    // Check for valid path characters
    if !path.chars().all(|c| c.is_ascii_alphanumeric() || "/-_:{}".contains(c)) {
        return Err(GenerationError::InvalidName(
            "Route path contains invalid characters. Only alphanumeric, '/', '-', '_', ':', '{', '}' are allowed".to_string()
        ));
    }
    
    Ok(())
}

/// Validate configuration keys
fn validate_config_key(key: &str) -> Result<(), GenerationError> {
    if key.is_empty() {
        return Err(GenerationError::InvalidName("Config key cannot be empty".to_string()));
    }
    
    // Config keys can contain dots for nested configuration
    if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-') {
        return Err(GenerationError::InvalidName(
            "Config key can only contain letters, numbers, underscores, dots, and hyphens".to_string()
        ));
    }
    
    Ok(())
}