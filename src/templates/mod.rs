pub mod engine;
pub mod provider;
pub mod builtin;

use crate::cli::{Framework, ComponentType};
use crate::error::TemplateError;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Template struct with metadata for code generation
#[derive(Debug, Clone)]
pub struct Template {
    /// Unique template name
    pub name: String,
    /// Template content with Handlebars syntax
    pub content: String,
    /// List of variables used in the template
    pub variables: Vec<TemplateVariable>,
    /// Target framework for this template
    pub framework: Framework,
    /// Component type this template generates
    pub component_type: ComponentType,
    /// Optional description of the template
    pub description: Option<String>,
    /// Template version for compatibility tracking
    pub version: String,
    /// Template author information
    pub author: Option<String>,
    /// Tags for template categorization
    pub tags: Vec<String>,
}

impl Template {
    /// Create a new template with basic information
    pub fn new(
        name: String,
        content: String,
        framework: Framework,
        component_type: ComponentType,
    ) -> Self {
        Self {
            name,
            content,
            variables: Vec::new(),
            framework,
            component_type,
            description: None,
            version: "1.0.0".to_string(),
            author: None,
            tags: Vec::new(),
        }
    }

    /// Add a variable to the template
    pub fn add_variable(&mut self, variable: TemplateVariable) {
        self.variables.push(variable);
    }

    /// Set template description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set template version
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// Set template author
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Add tags to the template
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Validate template syntax and variables
    pub fn validate(&self) -> Result<(), TemplateError> {
        // Check if template content is not empty
        if self.content.trim().is_empty() {
            return Err(TemplateError::ValidationError(
                "Template content cannot be empty".to_string(),
            ));
        }

        // Check if template name is valid
        if self.name.trim().is_empty() {
            return Err(TemplateError::ValidationError(
                "Template name cannot be empty".to_string(),
            ));
        }

        // Validate each template variable
        for variable in &self.variables {
            variable.validate()?;
        }

        Ok(())
    }
}

/// Template variable definition for validation and documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable name as used in template
    pub name: String,
    /// Variable type (String, Vec<String>, bool, etc.)
    pub var_type: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Default value if not provided
    pub default_value: Option<String>,
    /// Whether this variable is required
    pub required: bool,
    /// Validation pattern (regex) for the variable
    pub validation_pattern: Option<String>,
    /// Example values for documentation
    pub examples: Vec<String>,
}

impl TemplateVariable {
    /// Create a new required template variable
    pub fn new(name: &str, var_type: &str) -> Self {
        Self {
            name: name.to_string(),
            var_type: var_type.to_string(),
            description: None,
            default_value: None,
            required: true,
            validation_pattern: None,
            examples: Vec::new(),
        }
    }

    /// Create an optional template variable with default value
    pub fn optional(name: &str, var_type: &str, default_value: &str) -> Self {
        Self {
            name: name.to_string(),
            var_type: var_type.to_string(),
            description: None,
            default_value: Some(default_value.to_string()),
            required: false,
            validation_pattern: None,
            examples: Vec::new(),
        }
    }

    /// Set variable description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set validation pattern
    pub fn with_validation(mut self, pattern: String) -> Self {
        self.validation_pattern = Some(pattern);
        self
    }

    /// Add example values
    pub fn with_examples(mut self, examples: Vec<String>) -> Self {
        self.examples = examples;
        self
    }

    /// Validate the template variable definition
    pub fn validate(&self) -> Result<(), TemplateError> {
        if self.name.trim().is_empty() {
            return Err(TemplateError::ValidationError(
                "Template variable name cannot be empty".to_string(),
            ));
        }

        if self.var_type.trim().is_empty() {
            return Err(TemplateError::ValidationError(
                format!("Template variable '{}' must have a type", self.name),
            ));
        }

        // Validate regex pattern if provided
        if let Some(pattern) = &self.validation_pattern {
            if let Err(_) = regex::Regex::new(pattern) {
                return Err(TemplateError::ValidationError(
                    format!("Invalid validation pattern for variable '{}': {}", self.name, pattern),
                ));
            }
        }

        Ok(())
    }

    /// Check if a value matches this variable's validation pattern
    pub fn validate_value(&self, value: &str) -> Result<(), TemplateError> {
        if let Some(pattern) = &self.validation_pattern {
            let regex = regex::Regex::new(pattern).map_err(|e| {
                TemplateError::ValidationError(format!("Invalid regex pattern: {}", e))
            })?;
            
            if !regex.is_match(value) {
                return Err(TemplateError::ValidationError(
                    format!("Value '{}' does not match pattern '{}' for variable '{}'", 
                           value, pattern, self.name),
                ));
            }
        }
        Ok(())
    }
}

/// Template context with dynamic variables for rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContext {
    /// Name of the component being generated
    pub component_name: String,
    /// Optional module name for organization
    pub module_name: Option<String>,
    /// Target framework
    pub framework: String,
    /// Generation timestamp in RFC3339 format
    pub timestamp: String,
    /// Author information
    pub author: Option<String>,
    /// Custom variables provided by user
    pub custom_vars: HashMap<String, String>,
    /// Computed helper variables
    pub helpers: TemplateHelpers,
}

/// Helper variables computed from the context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateHelpers {
    /// Component name in snake_case
    pub snake_case_name: String,
    /// Component name in PascalCase
    pub pascal_case_name: String,
    /// Component name in camelCase
    pub camel_case_name: String,
    /// Component name in kebab-case
    pub kebab_case_name: String,
    /// Component name in SCREAMING_SNAKE_CASE
    pub screaming_snake_case_name: String,
    /// Module name in snake_case (if provided)
    pub snake_case_module: Option<String>,
    /// Module name in PascalCase (if provided)
    pub pascal_case_module: Option<String>,
}

impl TemplateContext {
    /// Create a new template context
    pub fn new(component_name: String, framework: Framework) -> Self {
        let helpers = TemplateHelpers::from_component_name(&component_name, None);
        
        Self {
            component_name,
            module_name: None,
            framework: format!("{:?}", framework),
            timestamp: chrono::Utc::now().to_rfc3339(),
            author: None,
            custom_vars: HashMap::new(),
            helpers,
        }
    }

    /// Set the module name and update helpers
    pub fn with_module(mut self, module_name: String) -> Self {
        self.helpers = TemplateHelpers::from_component_name(&self.component_name, Some(&module_name));
        self.module_name = Some(module_name);
        self
    }

    /// Set the author
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Add a custom variable
    pub fn add_variable(&mut self, key: String, value: String) {
        self.custom_vars.insert(key, value);
    }

    /// Add multiple custom variables
    pub fn with_variables(mut self, variables: HashMap<String, String>) -> Self {
        self.custom_vars.extend(variables);
        self
    }

    /// Validate the context against template requirements
    pub fn validate_against_template(&self, template: &Template) -> Result<(), TemplateError> {
        for variable in &template.variables {
            if variable.required {
                let value = self.custom_vars.get(&variable.name);
                if value.is_none() && variable.default_value.is_none() {
                    return Err(TemplateError::ValidationError(
                        format!("Required variable '{}' is missing", variable.name),
                    ));
                }
                
                // Validate the value if provided
                if let Some(value) = value {
                    variable.validate_value(value)?;
                }
            }
        }
        Ok(())
    }

    /// Get a variable value with fallback to default
    pub fn get_variable_value(&self, variable: &TemplateVariable) -> Option<String> {
        self.custom_vars
            .get(&variable.name)
            .cloned()
            .or_else(|| variable.default_value.clone())
    }
}

impl TemplateHelpers {
    /// Create helpers from component name and optional module name
    pub fn from_component_name(component_name: &str, module_name: Option<&str>) -> Self {
        Self {
            snake_case_name: to_snake_case(component_name),
            pascal_case_name: to_pascal_case(component_name),
            camel_case_name: to_camel_case(component_name),
            kebab_case_name: to_kebab_case(component_name),
            screaming_snake_case_name: to_screaming_snake_case(component_name),
            snake_case_module: module_name.map(to_snake_case),
            pascal_case_module: module_name.map(to_pascal_case),
        }
    }
}

// Helper functions for case conversion
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() && !result.is_empty() {
            result.push('_');
        }
        result.push(ch.to_lowercase().next().unwrap_or(ch));
    }
    
    result
}

fn to_pascal_case(s: &str) -> String {
    s.split(&['_', '-', ' '][..])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}

fn to_screaming_snake_case(s: &str) -> String {
    to_snake_case(s).to_uppercase()
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