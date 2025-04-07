use anyhow::{bail, Context, Result};
use genpdf::{
    fonts::{self, FontData, FontFamily},
    Document, SimplePageDecorator,
};

use std::env;
use std::fs;
use std::path::PathBuf;

use crate::{
    temps::{clean::gen_clean_temp, default::gen_default_temp},
    user::Person,
};

/// Contains the font discovery logic for the document setup
struct FontDiscovery {
    // all paths regardless of the system
    system_font_paths: Vec<PathBuf>,
}

impl FontDiscovery {
    fn new() -> Self {
        let mut paths = vec![
            "/usr/share/fonts".into(),
            "/usr/local/share/fonts".into(),
            "/run/current-system/sw/share/X11/fonts".into(),
            PathBuf::from(env::var("HOME").unwrap_or_default()).join(".fonts"),
            "/Library/Fonts".into(),
            "/System/Library/Fonts".into(),
        ];

        if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
            paths.push(PathBuf::from(xdg_data_home).join("fonts"));
        }

        Self {
            system_font_paths: paths,
        }
    }

    /// find the font liberation sans returning the found path
    fn find_liberation_sans(&self) -> Result<PathBuf> {
        let font_names = [
            "LiberationSans-Regular.ttf",
            "LiberationSans.ttf",
            "Liberation Sans Regular.ttf",
        ];

        for base_path in self.system_font_paths.iter() {
            for font_name in &font_names {
                let potential_path = base_path.join(font_name);
                if potential_path.exists() {
                    return Ok(potential_path);
                }
            }
        }

        bail!("Could not find Liberation Sans font")
    }
}

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

/// Main struct containing the document setup logic
pub struct CVGenerator {
    font_discovery: FontDiscovery,
}

impl CVGenerator {
    pub fn new() -> Self {
        Self {
            font_discovery: FontDiscovery::new(),
        }
    }

    fn load_font(&self, explicit_path: Option<&String>) -> Result<FontFamily<FontData>> {
        // Chose if there is an explicit path over the system paths if provided
        if let Some(path) = explicit_path {
            let path = PathBuf::from(path);
            if path.exists() {
                return fonts::from_files(&path, "LiberationSans", None)
                    .context("Failed to load font from explicit path");
            }
        }

        // find the font in system paths
        let font_path = self.font_discovery.find_liberation_sans()?;

        fonts::from_files(font_path.parent().unwrap(), "LiberationSans", None)
            .context("Failed to load Liberation Sans font")
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
        font_path: Option<&String>,
        template: Option<&String>,
    ) -> Result<()> {
        let data = fs::read_to_string(input_file).context("Failed to read input data file")?;

        let person = toml::from_str(&data).context("Invalid toml file format in input file")?;

        let font = self.load_font(font_path)?;

        let mut doc = self.setup_document(font);

        self.apply_template(&mut doc, &person, template);

        doc.render_to_file(output_file.expect("Error: Bad Output file name"))
            .context("Failed to render CV to output file")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_template_name_variations() {
        assert!(matches!(Template::from_str("clean"), Template::Clean));
        assert!(matches!(Template::from_str("default"), Template::Default));
        assert!(matches!(
            Template::from_str("nonexistent"),
            Template::Default
        ));
        assert!(matches!(Template::from_str(""), Template::Default));
        assert!(matches!(Template::from_str("fancy"), Template::Default));
    }

    #[test]
    fn test_generate_cv_with_missing_input_file() {
        let mut cv_generator = CVGenerator::new();
        let temp_dir = TempDir::new().unwrap();
        let missing_input_file = temp_dir.path().join("missing_person.toml");
        let output_file = temp_dir.path().join("output.pdf");

        let result = cv_generator.generate_cv(
            missing_input_file.to_str().unwrap(),
            Some(&output_file.to_str().unwrap().to_string()),
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_font_discovery_with_empty_paths() {
        let font_discovery = FontDiscovery {
            system_font_paths: Vec::new(),
        };

        let result = font_discovery.find_liberation_sans();
        assert!(result.is_err());
    }

    #[test]
    fn test_load_font_with_directory_path() {
        let cv_generator = CVGenerator::new();
        let temp_dir = TempDir::new().unwrap();
        let directory_path = temp_dir.path().to_str().unwrap().to_string();

        let result = cv_generator.load_font(Some(&directory_path));
        assert!(result.is_err());
    }
}
