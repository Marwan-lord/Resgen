use clap::{command, Arg, ArgMatches};

pub struct Cli;

impl Cli {
    pub fn run() -> ArgMatches {
        let parsed = command!()
            .author("Marwan Mohammed <merolokamino@gmail.com>")
            .about("Resgen is a lightning-fast static resume generator built with privacy and ATS optimization in mind")
        .arg(
            Arg::new("filename")
            .short('f')
            .required(true)
            .long("file")
            .help("choose the json file to generate your resume")
        )
        .arg(
            Arg::new("template")
            .short('t')
            .default_value("default")
            .long("temp")
            .help("Your resume's template. options: minimal, clean")
        )
        .arg(
            Arg::new("output")
            .short('o')
            .long("out")
            .help("choose the name of the output file")
            .default_value("cv.pdf")
        )
        .get_matches();

        parsed
    }
}
