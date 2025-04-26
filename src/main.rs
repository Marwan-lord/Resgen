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

    generator.generate_cv(
        input_file,
        parsed.get_one::<String>("output"),
        parsed.get_one::<String>("template"),
    )
}
