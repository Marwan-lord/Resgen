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
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::TempDir;

    use super::*;

    fn create_test_font(dir: &Path, name: &str) -> Result<PathBuf> {
        let font_path = dir.join(name);
        let mut file = File::create(&font_path)?;
        file.write_all(b"This is a dummy font file for testing")?;
        Ok(font_path)
    }

    fn create_test_json(dir: &Path) -> Result<PathBuf> {
        let json_path = dir.join("test_person.json");
        // NOTE NOTE NOTE LGTM
        let json_content = include_str!("../it.json");

        let mut file = File::create(&json_path)?;
        file.write_all(json_content.as_bytes())?;
        Ok(json_path)
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

        // Test adding multiple paths
        let another_path = PathBuf::from("/another/custom/path");
        discovery.add_custom_path(another_path.clone());
        assert!(discovery.custom_paths.contains(&another_path));
        assert_eq!(discovery.custom_paths.len(), 2);
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

    // NOTE NOTE NOTE NOTE LGTM
    #[test]
    fn test_find_liberation_sans_with_alternative_names() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Test with different font name variants
        let font_path = create_test_font(temp_dir.path(), "LiberationSans.ttf")?;

        let mut discovery = FontDiscovery::new();
        discovery.add_custom_path(temp_dir.path().to_path_buf());

        let found_path = discovery.find_liberation_sans()?;

        assert_eq!(found_path, font_path);

        let temp_dir2 = TempDir::new()?;
        let font_path2 = create_test_font(temp_dir2.path(), "Liberation Sans Regular.ttf")?;

        let mut discovery2 = FontDiscovery::new();
        discovery2.add_custom_path(temp_dir2.path().to_path_buf());

        let found_path2 = discovery2.find_liberation_sans()?;
        assert_eq!(found_path2, font_path2);

        Ok(())
    }

    // NOTE NOTE NOTE NOTE NOTE LGTM
    #[test]
    fn test_find_liberation_sans_system_path_priority() -> Result<()> {
        let system_dir = TempDir::new()?;
        let custom_dir = TempDir::new()?;

        let system_font = create_test_font(system_dir.path(), "LiberationSans-Regular.ttf")?;
        let _custom_font = create_test_font(custom_dir.path(), "LiberationSans-Regular.ttf")?;

        let mut discovery = FontDiscovery::new();
        discovery.system_font_paths.clear();
        discovery
            .system_font_paths
            .push(system_dir.path().to_path_buf());
        discovery.add_custom_path(custom_dir.path().to_path_buf());

        let found_path = discovery.find_liberation_sans()?;

        // Custom paths should take priority over system paths
        assert_ne!(found_path, system_font);
        Ok(())
    }

    // NOTE NOTE NOTE NOTE LGTM
    #[test]
    fn test_find_liberation_sans_not_found() {
        let mut discovery = FontDiscovery::new();

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

        assert!(!generator.font_discovery.system_font_paths.is_empty());
    }

    #[test]
    fn test_add_font_path_to_generator() {
        let mut generator = CVGenerator::new();
        let custom_path = PathBuf::from("/test/font/path");

        generator.add_font_path(custom_path.clone());

        assert!(generator.font_discovery.custom_paths.contains(&custom_path));
        assert_eq!(generator.font_discovery.custom_paths.len(), 1);
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

    // NOTE NOTE NOTE LGTM
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

        match Template::from_str("default") {
            Template::Default => {}
            _ => panic!("Expected Template::Default for 'default'"),
        }

        match Template::from_str("") {
            Template::Default => {}
            _ => panic!("Expected Template::Default for empty string"),
        }
    }

    // NOTE NOTE NOTE Kinda
    #[test]
    fn test_generate_cv_explicit_output() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let json_path = create_test_json(temp_dir.path())?;
        let output_path = temp_dir
            .path()
            .join("explicit_output.pdf")
            .to_string_lossy()
            .to_string();

        let mut generator = CVGenerator::new();

        let result =
            generator.generate_cv(json_path.to_str().unwrap(), Some(&output_path), None, None);

        assert!(result.is_ok());
        Ok(())
    }

    // NOTE NOTE NOTE Kinda
    #[test]
    fn test_generate_cv_with_clean_template() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let json_path = create_test_json(temp_dir.path())?;
        let template = "clean".to_string();

        let mut generator = CVGenerator::new();

        let result =
            generator.generate_cv(json_path.to_str().unwrap(), None, None, Some(&template));

        assert!(result.is_ok());

        Ok(())
    }

    // NOTE NOTE NOTE LGTM
    #[test]
    fn test_generate_cv_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_json_path = temp_dir.path().join("invalid.json");
        fs::write(&invalid_json_path, "{jakjdkfj json}").unwrap();

        let mut generator = CVGenerator::new();

        let result = generator.generate_cv(invalid_json_path.to_str().unwrap(), None, None, None);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                e.to_string().contains("Invalid JSON format"),
                "Unexpected error message: {}",
                e
            );
        }
    }

    // NOTE NOTE NOTE LGTM
    #[test]
    fn test_generate_cv_nonexistent_file() {
        let nonexistent_path = "/tmp/nonexistent_file_for_testing.json";

        let mut generator = CVGenerator::new();

        let result = generator.generate_cv(nonexistent_path, None, None, None);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                e.to_string().contains("Failed to read input JSON file"),
                "Unexpected error message: {}",
                e
            );
        }
    }
}
