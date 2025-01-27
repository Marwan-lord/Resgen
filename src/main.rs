pub mod templates;
pub mod user;
use std::fs;

use crate::user::Person;

use clap::{command, Arg};
use genpdf::{
    fonts::{self},
    Document, SimplePageDecorator,
};
use templates::{gen_clean_temp, gen_default_temp};

fn main() {
    let parsed = command!()
        .arg(
            Arg::new("filename")
            .short('f')
            .required(true)
            .long("file")
            .long_help("choose the json file to generate your resume")
        )
        .arg(
            Arg::new("template")
            .short('t').
            default_value("default")
            .long("temp")
            .long_help("options: minimal, clean")
        )
        .arg(
            Arg::new("output")
            .short('o')
            .long("out")
            .long_help("choose the name of the output file")
            .default_value("cv.pdf")
        )
        .about("Resgen is a CLI for generating your resume focused on privacy and simplicty as no data is stored")
        .get_matches();

    if let Some(fp) = parsed.get_one::<String>("filename") {
        let data = fs::read_to_string(fp).expect("File not found");
        let p: Person = serde_json::from_str(data.as_str()).expect("Unable to read json from file");

        let font =
            fonts::from_files("./fonts", "LiberationSans", None).expect("Failed to load font");

        let mut doc = Document::new(font);

        doc.set_font_size(12);
        doc.set_title("Demo document");

        let mut deco = SimplePageDecorator::new();
        deco.set_margins(10);
        doc.set_page_decorator(deco);

        if let Some(tmp) = parsed.get_one::<String>("template") {
            match tmp.as_str() {
                "minimal" => todo!(),
                "clean" => gen_clean_temp(&mut doc, &p),
                _ => gen_default_temp(&mut doc, &p),
            }
        }

        if let Some(o) = parsed.get_one::<String>("output") {
            doc.render_to_file(o)
                .expect("Error Rendering file to output");
        }
    }
}
