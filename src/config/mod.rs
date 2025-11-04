use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::cli::{Framework, ComponentType, HttpMethod, GuardType};
use crate::error::ConfigError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub framework: Framework,
    pub project_name: String,
    pub author: Option<String>,
    pub template_directory: Option<PathBuf>,
    pub custom_variables: HashMap<String, String>,
    pub module_structure: ModuleStructure,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub default_author: Option<String>,
    pub preferred_framework: Option<Framework>,
    pub custom_template_paths: Vec<PathBuf>,
    pub editor_integration: EditorConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleStructure {
    pub services_dir: String,
    pub routes_dir: String,
    pub guards_dir: String,
    pub models_dir: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorConfig {
    pub auto_format: bool,
    pub auto_import: bool,
}

impl Default for ModuleStructure {
    fn default() -> Self {
        Self {
            services_dir: "services".to_string(),
            routes_dir: "routes".to_string(),
            guards_dir: "guards".to_string(),
            models_dir: "models".to_string(),
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            auto_format: true,
            auto_import: true,
        }
    }
}

pub struct ConfigurationManager {
    project_config: Option<ProjectConfig>,
    user_config: Option<UserConfig>,
}

impl ConfigurationManager {
    pub fn new() -> Self {
        Self {
            project_config: None,
            user_config: None,
        }
    }

    pub async fn load_project_config(&mut self, project_path: &std::path::Path) -> Result<(), ConfigError> {
        let config_path = project_path.join(".ruffus.toml");
        if config_path.exists() {
            let content = tokio::fs::read_to_string(&config_path).await
                .map_err(|e| ConfigError::IoError(e.to_string()))?;
            let config: ProjectConfig = toml::from_str(&content)
                .map_err(|e| ConfigError::ParseError(e.to_string()))?;
            self.project_config = Some(config);
        }
        Ok(())
    }

    pub async fn load_user_config(&mut self) -> Result<(), ConfigError> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_path = home_dir.join(".ruffus").join("config.toml");
            if config_path.exists() {
                let content = tokio::fs::read_to_string(&config_path).await
                    .map_err(|e| ConfigError::IoError(e.to_string()))?;
                let config: UserConfig = toml::from_str(&content)
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?;
                self.user_config = Some(config);
            }
        }
        Ok(())
    }

    pub fn get_project_config(&self) -> Option<&ProjectConfig> {
        self.project_config.as_ref()
    }

    pub fn get_user_config(&self) -> Option<&UserConfig> {
        self.user_config.as_ref()
    }
}

impl Default for ConfigurationManager {
    fn default() -> Self {
        Self::new()
    }
}

// Generation Request Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGenerationRequest {
    pub name: String,
    pub module: Option<String>,
    pub methods: Vec<String>,
    pub dependencies: Vec<String>,
    pub crud: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteGenerationRequest {
    pub name: String,
    pub path: String,
    pub methods: Vec<HttpMethod>,
    pub middleware: Vec<String>,
    pub service_dependency: Option<String>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardGenerationRequest {
    pub name: String,
    pub guard_type: GuardType,
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleGenerationRequest {
    pub name: String,
    pub components: Vec<ComponentRequest>,
    pub dependencies: Vec<String>,
    pub with_auth: bool,
    pub with_crud: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRequest {
    pub component_type: ComponentType,
    pub name: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub field: String,
    pub rule_type: ValidationRuleType,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Email,
    Numeric,
    Custom(String),
}