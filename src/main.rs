pub mod cli;
pub mod templates;
pub mod user;
use std::process::{self};
use std::fs;

use crate::user::Person;

use colored::Colorize;
use genpdf::{
    fonts::{self},
    Document, SimplePageDecorator,
};
use templates::{gen_clean_temp, gen_default_temp};

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
];

const DEFAULT_FONT_NAME: &'static str = "LiberationSans";

fn main() -> anyhow::Result<()>{
    let parsed = cli::Cli::run();

    if let Some(fp) = parsed.get_one::<String>("filename") {
        let data = fs::read_to_string(fp)?;
        let p: Person = serde_json::from_str(data.as_str())?;

        // Lookup fonts
        let font_dir = FONT_DIRS
            .iter()
            .find(|path| std::path::Path::new(path).exists())
            .unwrap_or_else(|| {
                println!("{}: Font not found in any font directory, make sure the font {} is on your system","error".red(), DEFAULT_FONT_NAME);
                process::exit(1);
            });

        let font =
            fonts::from_files(font_dir, DEFAULT_FONT_NAME, None)?;

        let mut doc = Document::new(font);

        doc.set_font_size(12);
        doc.set_title("Resume");

        let mut deco = SimplePageDecorator::new();
        deco.set_margins(10);
        doc.set_page_decorator(deco);

        if let Some(tmp) = parsed.get_one::<String>("template") {
            match tmp.as_str() {
                "clean" => gen_clean_temp(&mut doc, &p),
                _ => gen_default_temp(&mut doc, &p),
            }
        }

        if let Some(o) = parsed.get_one::<String>("output") {
            doc.render_to_file(o)
                .expect("Error Rendering file to output");
        }
    }

    Ok(())
}
