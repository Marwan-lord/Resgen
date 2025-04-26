use anyhow::{Context, Result};
use fontconfig::Fontconfig;
use genpdf::{
    fonts::{self, FontData, FontFamily},
    Document, SimplePageDecorator,
};

use std::fs;

use crate::{
    temps::{clean::gen_clean_temp, default::gen_default_temp},
    user::Person,
};

#[derive(Debug)]
enum Template {
    Clean,
    Default,
}

impl Template {
    fn from_str(template: &str) -> Self {
        match template {
            "clean" => Template::Clean,
            _ => Template::Default,
        }
    }
}

pub struct CVGenerator {}

impl CVGenerator {
    pub fn new() -> Self {
        Self {}
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

    fn apply_template(&self, doc: &mut Document, person: &Person, template: Option<&String>) {
        match template.map(|t| Template::from_str(t)) {
            Some(Template::Clean) => gen_clean_temp(doc, person),
            _ => gen_default_temp(doc, person),
        }
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

        self.apply_template(&mut doc, &person, template);

        doc.render_to_file(output_file.expect("Error: Bad Output file name"))
            .context("Failed to render CV to output file")?;

        Ok(())
    }
}
