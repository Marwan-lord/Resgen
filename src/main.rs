use std::path::PathBuf;

use anyhow::{Context, Result};
use setup::CVGenerator;

mod cli;
mod setup;
mod temps;
mod user;

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
