use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ruffus")]
#[command(about = "Rust Web CLI Generator")]
#[command(version = "0.1.0")]
pub enum Commands {
    /// Initialize a new project
    Init {
        #[arg(short, long)]
        framework: Option<Framework>,
        #[arg(short, long)]
        name: String,
    },
    /// Generate a component
    Generate {
        #[command(subcommand)]
        component: GenerateComponent,
    },
    /// Configure templates and settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum GenerateComponent {
    /// Generate a service
    Service { 
        name: String, 
        #[arg(short, long)]
        module: Option<String>,
        #[arg(long)]
        methods: Vec<String>,
        #[arg(long)]
        dependencies: Vec<String>,
    },
    /// Generate REST API routes
    Route { 
        name: String, 
        #[arg(short, long)]
        methods: Vec<HttpMethod>, 
        #[arg(short, long)]
        path: String,
        #[arg(long)]
        middleware: Vec<String>,
        #[arg(long)]
        service_dependency: Option<String>,
    },
    /// Generate middleware/guard
    Guard { 
        name: String, 
        #[arg(short, long)]
        guard_type: GuardType,
        #[arg(long)]
        validation_rules: Vec<String>,
    },
    /// Generate a complete web service module
    Module { 
        name: String, 
        #[arg(short, long)]
        components: Vec<ComponentType>,
        #[arg(long)]
        dependencies: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Set a configuration value
    Set { key: String, value: String },
    /// Get a configuration value
    Get { key: String },
    /// List all configuration values
    List,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Framework {
    Axum,
    ActixWeb,
    Warp,
    Rocket,
    Custom(String),
}

impl std::str::FromStr for Framework {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "axum" => Ok(Framework::Axum),
            "actix" | "actix-web" => Ok(Framework::ActixWeb),
            "warp" => Ok(Framework::Warp),
            "rocket" => Ok(Framework::Rocket),
            _ => Ok(Framework::Custom(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl std::str::FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(format!("Invalid HTTP method: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GuardType {
    Auth,
    Jwt,
    Validation,
    RateLimit,
    Cors,
    Logging,
    Custom(String),
}

impl std::str::FromStr for GuardType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auth" => Ok(GuardType::Auth),
            "jwt" => Ok(GuardType::Jwt),
            "validation" => Ok(GuardType::Validation),
            "ratelimit" | "rate-limit" => Ok(GuardType::RateLimit),
            "cors" => Ok(GuardType::Cors),
            "logging" => Ok(GuardType::Logging),
            _ => Ok(GuardType::Custom(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ComponentType {
    Service,
    Route,
    Guard,
    Model,
}

impl std::str::FromStr for ComponentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "service" => Ok(ComponentType::Service),
            "route" => Ok(ComponentType::Route),
            "guard" => Ok(ComponentType::Guard),
            "model" => Ok(ComponentType::Model),
            _ => Err(format!("Invalid component type: {}", s)),
        }
    }
}