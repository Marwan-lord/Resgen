# Resgen - JSON to PDF Resume Generator

Resgen is a lightweight, command-line tool that converts structured JSON data into polished PDF resumes.
Built with Rust, it prioritizes speed, simplicity,and customization 
while avoiding bloated dependencies.
Define your resume once in JSON,then generate consistent PDF versions tailored to different opportunities.

>[!WARNING]
> The app might not work due to problems with reading fonts from font directories if you encounter such problem please submit an issue

## Prerequisites
1. Having LiberationSans font installed

## Getting Started
1. Make a file ending with .json then copy the file temp.json found in this repo to the file
2. fill the the required fields and remove the optional fields (eg. Work Expreince, Projects, all skills fields are optional but you must have one at least)
3. save and exit then execute


### Producing the Output resume
``` bash
 resgen -f resume.json -o mycv.pdf -t clean  
```

``` bash
 resgen -f cv.json
```

## Resulted PDFs
The default template: [here](https://github.com/Marwan-lord/Resgen/blob/main/default_cv.pdf)  
The clean option: [here](https://github.com/Marwan-lord/Resgen/blob/main/clean_cv.pdf)  

## Features

- **Clean JSON Structure**: Define your resume content in a human-readable JSON format
- **CLI Simplicity**: Generate PDFs with a single terminal command
- **Zero Runtime Dependencies**: Compiled binary works out-of-the-box
- **Cross-Platform**: Runs seamlessly on macOS, and Linux.
- **Privacy-First**: Your data stays local as no cloud dependencies or tracking.

## Installation

### From Source
Requires [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

```bash
cargo install --git https://github.com/Marwan-lord/Resgen
