use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
use clap::{Parser, Subcommand};
use nix_docs_gen::{Documentation, r#type::{Type, RecordField}};

#[derive(Subcommand)]
enum Command {
    /**
    Read a documentation file.

    Pretty-prints the documentation file to stdout.
    */
    Read {
        /// The file to read
        file: PathBuf,
    },

    /**
    Write a documentation file.

    Takes pretty-printed documentation on stdin and writes it to <FILE>.
    */
    Write {
        /// The file to write
        file: PathBuf,
    },

    /**
    Translate a documentation file to HTML.
    */
    Gen {
        /// The file to translate
        file: PathBuf,
    },

    /**
    Generate a test documentation file.
    */
    Test {
        /// The file to generate
        file: PathBuf,
    },
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

fn read_file(path: &Path) -> nix_docs_gen::Result<()> {
    let documentation = {
        let file = File::open(path).map_err(nix_docs_gen::Error::IoError)?;
        let mut file = BufReader::new(file);
        Documentation::read_cbor(&mut file)
    }?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    documentation.write_syntax(&mut stdout)
}

fn write_file(path: &Path) -> nix_docs_gen::Result<()> {
    let documentation = {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        Documentation::read_cbor(&mut stdin)
    }?;

    let file = File::create(path).map_err(nix_docs_gen::Error::IoError)?;
    let mut file = BufWriter::new(file);
    documentation.write_cbor(&mut file)
}

fn generate(path: &Path) -> nix_docs_gen::Result<()> {
    let documentation = {
        let file = File::open(path).map_err(nix_docs_gen::Error::IoError)?;
        let mut file = BufReader::new(file);
        Documentation::read_cbor(&mut file)
    }?;

    let file = File::create(path.with_extension("html")).map_err(nix_docs_gen::Error::IoError)?;
    let mut file = BufWriter::new(file);
    documentation.write_html(&mut file)
}

fn test(path: &Path) -> nix_docs_gen::Result<()> {
    let documentation = Documentation {
        title: String::from("mkDerivation"),
        intro: String::from("mkDerivation is the main function used to build packages with the standard environment."),
        r#type: Type::Function{
            input: Box::new(Type::Record{fields: Some(vec![
                RecordField::Section{
                    title: String::from("Core Attributes"),
                    fields: vec![
                        RecordField::Item{ 
                            name: String::from("name"), 
                            optional: false,
                            r#type: Type::String, 
                            docs: String::from("The derivation name.") 
                        },
                        RecordField::Item{ 
                            name: String::from("pname"), 
                            optional: true,
                            r#type: Type::String, 
                            docs: String::from("The package name.") 
                        },
                        RecordField::Item{ 
                            name: String::from("version"), 
                            optional: true,
                            r#type: Type::String, 
                            docs: String::from("The package version.") 
                        },
                        RecordField::Item{ 
                            name: String::from("src"), 
                            optional: false,
                            r#type: Type::Path, 
                            docs: String::from("The package source directory.") 
                        },
                    ]
                },
                RecordField::Section{
                    title: String::from("Building"),
                    fields: vec![
                        RecordField::Item{ 
                            name: String::from("buildInputs"), 
                            optional: true,
                            r#type: Type::List(Box::new(Type::Derivation)), 
                            docs: String::from("The package's dependencies.") 
                        },
                        RecordField::Item{ 
                            name: String::from("buildPhase"), 
                            optional: true,
                            r#type: Type::String, 
                            docs: String::from("A shell script to run during the build phase. If omitted, the build phase will run make.") 
                        },
                        RecordField::Item{ 
                            name: String::from("installPhase"), 
                            optional: true,
                            r#type: Type::String, 
                            docs: String::from("A shell script to run during the install phase. If omitted, the install phase will run make install.") 
                        },
                        RecordField::Item{ 
                            name: String::from("builder"), 
                            optional: true,
                            r#type: Type::Path,
                            docs: String::from("The path to a shell script that describes how to build the package. If omitted, the build runs using stdenv's generic builder.") 
                        },
                    ]
                },
                RecordField::Item{ 
                    name: String::from("shellHook"), 
                    optional: true,
                    r#type: Type::String,
                    docs: String::from("A shell script to run after entering a nix-shell.") 
                },
            ])}),
            output: Box::new(Type::Derivation),
        }
    };

    let file = File::create(path).map_err(nix_docs_gen::Error::IoError)?;
    let mut file = BufWriter::new(file);
    documentation.write_cbor(&mut file)
}

fn main() -> nix_docs_gen::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Read { file } => read_file(&file),
        Command::Write { file } => write_file(&file),
        Command::Gen { file } => generate(&file),
        Command::Test { file } => test(&file),
    }
}
