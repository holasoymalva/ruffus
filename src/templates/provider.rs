use super::{Template, TemplateInfo, TemplateProvider};
use crate::cli::{Framework, ComponentType};
use crate::error::TemplateError;

pub struct BuiltInTemplateProvider;

impl TemplateProvider for BuiltInTemplateProvider {
    fn get_template(&self, component: ComponentType, framework: Framework) -> Result<Template, TemplateError> {
        // TODO: Implement template retrieval from built-in templates
        Err(TemplateError::TemplateNotFound {
            framework: format!("{:?}", framework),
            component: format!("{:?}", component),
        })
    }

    fn list_templates(&self) -> Vec<TemplateInfo> {
        // TODO: Return list of built-in templates
        vec![]
    }

    fn validate_template(&self, _template: &Template) -> Result<(), TemplateError> {
        // TODO: Implement template validation
        Ok(())
    }
}

pub struct CustomTemplateProvider {
    template_paths: Vec<std::path::PathBuf>,
}

impl CustomTemplateProvider {
    pub fn new(template_paths: Vec<std::path::PathBuf>) -> Self {
        Self { template_paths }
    }
}

impl TemplateProvider for CustomTemplateProvider {
    fn get_template(&self, component: ComponentType, framework: Framework) -> Result<Template, TemplateError> {
        // TODO: Implement template retrieval from custom paths
        Err(TemplateError::TemplateNotFound {
            framework: format!("{:?}", framework),
            component: format!("{:?}", component),
        })
    }

    fn list_templates(&self) -> Vec<TemplateInfo> {
        // TODO: Return list of custom templates
        vec![]
    }

    fn validate_template(&self, _template: &Template) -> Result<(), TemplateError> {
        // TODO: Implement template validation
        Ok(())
    }
}