use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::cli::Framework;
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