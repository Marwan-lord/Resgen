pub mod cli;
pub mod temps;
pub mod user;

use crate::user::Person;
use std::fs;
use std::path::Path;
use std::process;

use anyhow::Result;
use colored::Colorize;
use genpdf::{
    fonts::{self, FontData, FontFamily},
    Document, SimplePageDecorator,
};

use temps::{clean::gen_clean_temp, default::gen_default_temp};

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
    "/usr/share/fonts/",
    "./fonts/",
    "~/.fonts",
    "~/.local/share/fonts",
    "/run/current-system/sw/share/X11/fonts",
    "%LOCALAPPDATA%\\Microsoft\\Windows\\Fonts",
    "C:\\Windows\\Fonts",
    "C:\\Windows\\Fonts\\Liberation",
    "/Library/Fonts/",
];

const DEFAULT_FONT_NAME: &str = "LiberationSans";

fn main() -> Result<()> {
    let parsed = cli::Cli::run();

    if let Some(fp) = parsed.get_one::<String>("filename") {
        let data = fs::read_to_string(fp)?;
        let person: Person = serde_json::from_str(&data)?;
        let font = load_font(parsed.get_one::<String>("font-path"));

        let mut doc = setup_document(font);

        apply_template(&mut doc, &person, parsed.get_one::<String>("template"));

        if let Some(output) = parsed.get_one::<String>("output") {
            doc.render_to_file(output)
                .expect("Error rendering file to output");
        }
    }

    Ok(())
}

fn load_font(font_path: Option<&String>) -> FontFamily<FontData> {
    if let Some(fp) = font_path {
        fonts::from_files(fp, DEFAULT_FONT_NAME, None).unwrap()
    } else {
        let font_dir = FONT_DIRS.iter()
            .find(|&&path| Path::new(path).exists())
            .unwrap_or_else(|| {
                println!(
                    "{}: Font not found in any font directory, make sure the font {} is on your system or specify the font directory with the -p option",
                    "error".red(),
                    DEFAULT_FONT_NAME
                );
                process::exit(1);
            });
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, None).unwrap()
    }
}

fn setup_document(font: fonts::FontFamily<FontData>) -> Document {
    let mut doc = Document::new(font);
    doc.set_font_size(12);
    doc.set_title("CV");

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc
}

fn apply_template(doc: &mut Document, person: &Person, template: Option<&String>) {
    if let Some(tmp) = template {
        match tmp.as_str() {
            "clean" => gen_clean_temp(doc, person),
            _ => gen_default_temp(doc, person),
        }
    }
}
