pub mod engine;
pub mod provider;
pub mod builtin;

use crate::cli::{Framework, ComponentType};
use crate::error::TemplateError;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub content: String,
    pub variables: Vec<TemplateVariable>,
    pub framework: Framework,
    pub component_type: ComponentType,
}

#[derive(Debug, Clone)]
pub struct TemplateVariable {
    pub name: String,
    pub var_type: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
}

impl TemplateVariable {
    pub fn new(name: &str, var_type: &str) -> Self {
        Self {
            name: name.to_string(),
            var_type: var_type.to_string(),
            description: None,
            default_value: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateContext {
    pub component_name: String,
    pub module_name: Option<String>,
    pub framework: String,
    pub timestamp: String,
    pub author: Option<String>,
    pub custom_vars: HashMap<String, String>,
}

impl TemplateContext {
    pub fn new(component_name: String, framework: Framework) -> Self {
        Self {
            component_name,
            module_name: None,
            framework: format!("{:?}", framework),
            timestamp: chrono::Utc::now().to_rfc3339(),
            author: None,
            custom_vars: HashMap::new(),
        }
    }
}

pub trait TemplateProvider {
    fn get_template(&self, component: ComponentType, framework: Framework) -> Result<Template, TemplateError>;
    fn list_templates(&self) -> Vec<TemplateInfo>;
    fn validate_template(&self, template: &Template) -> Result<(), TemplateError>;
}

#[derive(Debug)]
pub struct TemplateInfo {
    pub name: String,
    pub framework: Framework,
    pub component_type: ComponentType,
    pub description: Option<String>,
}