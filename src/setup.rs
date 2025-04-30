use crate::temps::TemplateRegistry;
use anyhow::{Context, Result};
use fontconfig::Fontconfig;
use genpdf::{
    fonts::{self, FontData, FontFamily},
    Document, SimplePageDecorator,
};
use std::fs;

pub struct CVGenerator {
    template_registry: TemplateRegistry,
}

impl CVGenerator {
    pub fn new() -> Self {
        Self {
            template_registry: TemplateRegistry::new(),
        }
    }

    fn setup_document(&self, font: FontFamily<FontData>) -> Document {
        let mut doc = Document::new(font);
        doc.set_font_size(11);
        doc.set_title("Professional CV");
        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins(12);
        doc.set_page_decorator(decorator);
        doc
    }

    pub fn generate_cv(
        &mut self,
        input_file: &str,
        output_file: Option<&String>,
        template: Option<&String>,
    ) -> Result<()> {
        let data = fs::read_to_string(input_file).context("Failed to read input data file")?;
        let person = toml::from_str(&data).context("Invalid toml file format in input file")?;

        let fc = Fontconfig::new().unwrap();
        let ff = fc.find("LiberationSans", None).unwrap();
        let path = ff.path.parent().unwrap().to_str().unwrap();
        let font = fonts::from_files(&path, "LiberationSans", None).unwrap();

        let mut doc = self.setup_document(font);

        let bind = "default".to_string();
        let template_name = template.unwrap_or(&bind);
        self.template_registry
            .generate(template_name, &mut doc, &person);

        doc.render_to_file(output_file.expect("Error: Bad Output file name"))
            .context("Failed to render CV to output file")?;

        Ok(())
    }
}
