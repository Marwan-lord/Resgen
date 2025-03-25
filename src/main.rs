use anyhow::{bail, Context, Result};
use genpdf::{
    fonts::{self, FontData, FontFamily},
    Document, SimplePageDecorator,
};
use serde_json;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod cli;
mod temps;
mod user;

use temps::{clean::gen_clean_temp, default::gen_default_temp};
use user::Person;

struct FontDiscovery {
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

struct CVGenerator {
    font_discovery: FontDiscovery,
}

impl CVGenerator {
    fn new() -> Self {
        Self {
            font_discovery: FontDiscovery::new(),
        }
    }

    fn add_font_path(&mut self, path: PathBuf) {
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

    fn generate_cv(
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

fn main() -> Result<()> {
    let parsed = cli::Cli::run();

    let input_file = parsed
        .get_one::<String>("filename")
        .context("No input file specified")?;

    let mut generator = CVGenerator::new();

    if let Some(custom_path) = parsed.get_one::<String>("font-path") {
        generator.add_font_path(PathBuf::from(custom_path));
    }

    generator.generate_cv(
        input_file,
        parsed.get_one::<String>("output"),
        parsed.get_one::<String>("font-path"),
        parsed.get_one::<String>("template"),
    )
}
