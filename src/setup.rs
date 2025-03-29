use anyhow::{bail, Context, Result};
use genpdf::{
    fonts::{self, FontData, FontFamily},
    Document, SimplePageDecorator,
};

use serde_json;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    temps::{clean::gen_clean_temp, default::gen_default_temp},
    user::Person,
};

pub struct FontDiscovery {
    system_font_paths: Vec<PathBuf>,
    custom_paths: Vec<PathBuf>,
}

impl FontDiscovery {
    fn new() -> Self {
        let mut paths = vec![
            "/usr/share/fonts".into(),
            "/usr/local/share/fonts".into(),
            "/run/current-system/sw/share/X11/fonts".into(),
            PathBuf::from(env::var("HOME").unwrap_or_default()).join(".fonts"),
            PathBuf::from(env::var("WINDIR").unwrap_or_default()).join("Fonts"),
            "/Library/Fonts".into(),
            "/System/Library/Fonts".into(),
        ];

        if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
            paths.push(PathBuf::from(xdg_data_home).join("fonts"));
        }

        Self {
            system_font_paths: paths,
            custom_paths: Vec::new(),
        }
    }

    fn add_custom_path(&mut self, path: PathBuf) {
        self.custom_paths.push(path);
    }

    fn find_liberation_sans(&self) -> Result<PathBuf> {
        let font_names = [
            "LiberationSans-Regular.ttf",
            "LiberationSans.ttf",
            "Liberation Sans Regular.ttf",
        ];

        let search_paths = self
            .custom_paths
            .iter()
            .chain(self.system_font_paths.iter());

        for base_path in search_paths {
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

pub struct CVGenerator {
    font_discovery: FontDiscovery,
}

impl CVGenerator {
    pub fn new() -> Self {
        Self {
            font_discovery: FontDiscovery::new(),
        }
    }

    pub fn add_font_path(&mut self, path: PathBuf) {
        self.font_discovery.add_custom_path(path);
    }

    fn load_font(&self, explicit_path: Option<&String>) -> Result<FontFamily<FontData>> {
        // If an explicit path is provided, try that first
        if let Some(path) = explicit_path {
            let path = PathBuf::from(path);
            if path.exists() {
                return fonts::from_files(&path, "LiberationSans", None)
                    .context("Failed to load font from explicit path");
            }
        }

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
        let data = fs::read_to_string(input_file).context("Failed to read input JSON file")?;

        let person: Person =
            serde_json::from_str(&data).context("Invalid JSON format in input file")?;

        let font = self.load_font(font_path)?;

        let mut doc = self.setup_document(font);

        self.apply_template(&mut doc, &person, template);

        if let Some(output) = output_file {
            doc.render_to_file(output)
                .context("Failed to render CV to output file")?;
        } else {
            let default_output = format!(
                "{}_cv.pdf",
                Path::new(input_file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output")
            );
            doc.render_to_file(&default_output)
                .context("Failed to render CV to default output file")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::TempDir;

    use fs::File;

    use super::*;

    fn create_test_font(dir: &Path, name: &str) -> Result<PathBuf> {
        let font_path = dir.join(name);
        let mut file = File::create(&font_path)?;
        file.write_all(b"This is a dummy font file for testing")?;
        Ok(font_path)
    }

    #[test]
    fn test_font_discovery_initialization() {
        let discovery = FontDiscovery::new();

        assert!(discovery
            .system_font_paths
            .contains(&PathBuf::from("/usr/share/fonts")));
        assert!(discovery
            .system_font_paths
            .contains(&PathBuf::from("/Library/Fonts")));

        if let Ok(home) = env::var("HOME") {
            assert!(discovery
                .system_font_paths
                .contains(&PathBuf::from(home).join(".fonts")));
        }

        assert!(discovery.custom_paths.is_empty());
    }

    #[test]
    fn test_add_custom_path() {
        let mut discovery = FontDiscovery::new();
        let custom_path = PathBuf::from("/custom/fonts/path");

        discovery.add_custom_path(custom_path.clone());

        assert!(discovery.custom_paths.contains(&custom_path));
        assert_eq!(discovery.custom_paths.len(), 1);
    }

    #[test]
    fn test_find_liberation_sans_with_custom_path() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let font_path = create_test_font(temp_dir.path(), "LiberationSans-Regular.ttf")?;

        let mut discovery = FontDiscovery::new();
        discovery.add_custom_path(temp_dir.path().to_path_buf());

        let found_path = discovery.find_liberation_sans()?;

        assert_eq!(found_path, font_path);
        Ok(())
    }

    #[test]
    fn test_find_liberation_sans_with_alternative_names() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let font_path = create_test_font(temp_dir.path(), "LiberationSans.ttf")?;

        let mut discovery = FontDiscovery::new();
        discovery.add_custom_path(temp_dir.path().to_path_buf());

        let found_path = discovery.find_liberation_sans()?;

        assert_eq!(found_path, font_path);
        Ok(())
    }

    #[test]
    fn test_find_liberation_sans_not_found() {
        let mut discovery = FontDiscovery::new();

        // Create a temp directory path that doesn't actually exist
        let temp_dir = PathBuf::from("/tmp/nonexistent_dir_for_testing");
        discovery.system_font_paths.clear();
        discovery.add_custom_path(temp_dir);

        let result = discovery.find_liberation_sans();

        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Could not find Liberation Sans font");
        }
    }

    #[test]
    fn test_cv_generator_creation() {
        let generator = CVGenerator::new();

        // Verify it has a font discovery instance
        assert!(!generator.font_discovery.system_font_paths.is_empty());
    }

    #[test]
    fn test_load_font_with_explicit_path() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path().to_path_buf();

        let ttf_dir = dir_path.join("fonts");
        fs::create_dir_all(&ttf_dir)?;

        let font_path = ttf_dir.join("LiberationSans-Regular.ttf");
        File::create(&font_path)?;

        let generator = CVGenerator::new();
        let explicit_path = font_path.to_string_lossy().to_string();

        let result = generator.load_font(Some(&explicit_path));

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Failed to load font"));
        }

        Ok(())
    }

    #[test]
    fn test_template_from_str() {
        match Template::from_str("clean") {
            Template::Clean => {}
            _ => panic!("Expected Template::Clean"),
        }

        match Template::from_str("unknown") {
            Template::Default => {}
            _ => panic!("Expected Template::Default for unknown template"),
        }
    }
}
