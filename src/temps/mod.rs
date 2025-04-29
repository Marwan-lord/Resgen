pub mod clean;
pub mod default;
pub mod line;
pub mod template;

// Template registry
use crate::user::Person;
use genpdf::Document;
use std::collections::HashMap;
use template::CVTemplate;

pub struct TemplateRegistry {
    templates: HashMap<String, Box<dyn CVTemplate>>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };

        // Register available templates
        registry.register(Box::new(default::DefaultTemplate::new()));
        registry.register(Box::new(clean::CleanTemplate::new()));

        registry
    }

    pub fn register(&mut self, template: Box<dyn CVTemplate>) {
        self.templates.insert(template.name().to_string(), template);
    }

    pub fn get(&self, template_name: &str) -> Option<&Box<dyn CVTemplate>> {
        self.templates.get(template_name)
    }

    pub fn generate(&self, template_name: &str, doc: &mut Document, person: &Person) {
        let template = self.get(template_name).unwrap_or_else(|| {
            // Default to the "default" template if the requested one isn't found
            self.templates
                .get("default")
                .expect("Default template should always be available")
        });

        template.generate(doc, person);
    }
}
