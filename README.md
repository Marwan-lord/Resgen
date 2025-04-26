# Resgen - TOML to PDF Resume Generator

Resgen is a lightweight, command-line tool that converts structured TOML data into polished PDF resumes.
Built with Rust, it prioritizes speed, simplicity,and customization 
while avoiding bloated dependencies.
Define your resume once ,then generate consistent PDF versions tailored to different opportunities.


## Prerequisites
1. Having LiberationSans font installed

## Getting Started
1. Make a file ending with .toml then copy the file temp.toml found in this repo to the file
2. fill the the required fields and remove the optional fields (eg. Work Expreince, Projects, all skills fields are optional but you must have one at least)
3. save and exit then execute

### Testing on your machine 
```bash
git clone https://github.com/Marwan-lord/Resgen.git
cd Resgen
cargo build --release
./target/release/resgen -f it.toml # the output is going to be cv.pdf
```

### Simple Troubleshooting
If it says that the font isn't found try installing the font and putting it in ~/.fonts/ then rerun the program


### Producing the Output resume
``` bash
 resgen -f resume.toml -o mycv.pdf -t clean -p /path/to/dir # note that it's the directory not the font itself (LiberationSans)
```

``` bash
 resgen -f cv.toml
```
## Results
![default](https://github.com/Marwan-lord/Resgen/blob/main/assets/default_cv.png)
![clean](https://github.com/Marwan-lord/Resgen/blob/main/assets/clean_cv.png)

## Resulted PDFs
The default template: [here](https://github.com/Marwan-lord/Resgen/blob/main/default_cv.pdf)  
The clean option: [here](https://github.com/Marwan-lord/Resgen/blob/main/clean_cv.pdf)  

## Features

- **Clean TOML Structure**: Define your resume content in a human-readable TOML format
- **CLI Simplicity**: Generate PDFs with a single terminal command
- **Zero Runtime Dependencies**: Compiled binary works out-of-the-box
- **Cross-Platform**: Runs seamlessly on macOS, and Linux.
- **Privacy-First**: Your data stays local as no cloud dependencies or tracking.

## Installation

### From Source
Requires [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

```bash
cargo install --git https://github.com/Marwan-lord/Resgen
```

### On NixOS
```bash 
git clone https://github.com/Marwan-lord/Resgen.git
cd Resgen
nix-build package.nix
nix-env -f package.nix -i
resgen --help
```
This installs the program on your system. 

## Nix Shell

You can also use the provided shell.nix file, just type
```bash
nix-shell
```

## Flakes
Flakes are the more modern approach to nix dev environments, just run
```
nix develop 
cargo build 
```

to setup the dev environment and then build the project
