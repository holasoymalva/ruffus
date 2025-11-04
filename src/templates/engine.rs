use handlebars::Handlebars;
use crate::error::TemplateError;
use crate::templates::{Template, TemplateContext};


pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self, TemplateError> {
        let mut handlebars = Handlebars::new();
        
        // Register custom helpers
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
        
        Ok(Self { handlebars })
    }

    pub fn render(&self, template: &Template, context: &TemplateContext) -> Result<String, TemplateError> {
        self.handlebars
            .render_template(&template.content, context)
            .map_err(|e| TemplateError::RenderError(e.to_string()))
    }

    pub fn register_template(&mut self, name: &str, template: &str) -> Result<(), TemplateError> {
        self.handlebars
            .register_template_string(name, template)
            .map_err(|e| TemplateError::RegistrationError(e.to_string()))
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default template engine")
    }
}

// Helper functions for case conversion
fn snake_case_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    let snake_case = to_snake_case(param);
    out.write(&snake_case)?;
    Ok(())
}

fn pascal_case_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    let pascal_case = to_pascal_case(param);
    out.write(&pascal_case)?;
    Ok(())
}

fn kebab_case_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    let kebab_case = to_kebab_case(param);
    out.write(&kebab_case)?;
    Ok(())
}

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
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}