# Resgen - JSON to PDF Resume Generator

Resgen is a lightweight, command-line tool that converts structured JSON data into polished PDF resumes. Built with Rust, it prioritizes speed, simplicity, and customization while avoiding bloated dependencies. Define your resume once in JSON, then generate consistent PDF versions tailored to different opportunities, it prioritizes privacy as no data is collected unlike cloud alterinatives.  


## Examples
- resgen -f resume.json -o mycv.pdf -t clean  
- resgen -f cv.json

## Result PDF
The default template: [here](https://github.com/Marwan-lord/Resgen/blob/main/default_cv.pdf)  
The clean option: [here](https://github.com/Marwan-lord/Resgen/blob/main/clean_cv.pdf)  

## Features

- **Clean JSON Structure**: Define your resume content in a human-readable JSON format
- **CLI Simplicity**: Generate PDFs with a single terminal command
- **Zero Runtime Dependencies**: Compiled binary works out-of-the-box
- **Cross-Platform**: Runs seamlessly on macOS, and Linux.
- **Privacy-First**: Your data stays local—no cloud dependencies or tracking.

## Installation

### From Source
Requires [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

```bash
cargo install --git https://github.com/Marwan-lord/Resgen
