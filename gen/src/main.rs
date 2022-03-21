use clap::{Parser, Subcommand};
use nix_docs_gen::{
    markup::{Block, Markup, Text},
    r#type::{RecordField, RecordFieldItem, Type},
    Documentation,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

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
                        RecordFieldItem{
                            name: String::from("name"),
                            optional: false,
                            r#type: Type::String,
                            docs: Markup(vec![
                                Block::Paragraph(vec![
                                    Text::Plain(String::from("The derivation name.")),
                                ]),
                                Block::Paragraph(vec![
                                    Text::Plain(String::from("Can be omitted if ")),
                                    Text::Link{destination: String::from("#reference-inputs-pname"), text: String::from("pname")},
                                    Text::Plain(String::from(" and ")),
                                    Text::Link{destination: String::from("#reference-inputs-version"), text: String::from("version")},
                                    Text::Plain(String::from(" are set, in which case it is automatically set to ")),
                                    Text::Code(String::from("${pname}-${version}")),
                                    Text::Plain(String::from(".")),
                                ]),
                                Block::Paragraph(vec![Text::Bold(String::from("Examples"))]),
                                Block::Code(String::from("stdenv.mkDerivation {\n  name = \"libfoo-1.2.3\";\n  src = ./src;\n}\n")),
                                Block::Code(String::from("stdenv.mkDerivation {\n  pname = \"libfoo\";\n  version = \"1.2.3\";\n  src = ./src;\n}")),
                            ])
                        },
                        RecordFieldItem{
                            name: String::from("pname"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup::paragraph("The package name.")
                        },
                        RecordFieldItem{
                            name: String::from("version"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup::paragraph("The package version.")
                        },
                        RecordFieldItem{
                            name: String::from("src"),
                            optional: false,
                            r#type: Type::Path,
                            docs: Markup(vec![
                                Block::paragraph("The package source directory."),
                                Block::Paragraph(vec![Text::bold("Examples")]),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo-1.2.3\";\n  src = ./src;\n}"),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo-1.2.3\";\n  src = fetchurl {\n    url = http://example.org/libfoo-1.2.3.tar.bz2;\n    sha256 = \"0x2g1jqygyr5wiwg4ma1nd7w4ydpy82z9gkcv8vh2v8dn3y58v5m\";\n  };\n}"),
                            ])
                        },
                    ]
                },
                RecordField::Section{
                    title: String::from("Building"),
                    fields: vec![
                        RecordFieldItem{
                            name: String::from("buildInputs"),
                            optional: true,
                            r#type: Type::List(Box::new(Type::Derivation)),
                            docs: Markup::paragraph("The package's dependencies.")
                        },
                        RecordFieldItem{
                            name: String::from("buildPhase"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup::paragraph("A shell script to run during the build phase. If omitted, the build phase will run make.")
                        },
                        RecordFieldItem{
                            name: String::from("installPhase"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup::paragraph("A shell script to run during the install phase. If omitted, the install phase will run make install.")
                        },
                        RecordFieldItem{
                            name: String::from("builder"),
                            optional: true,
                            r#type: Type::Path,
                            docs: Markup::paragraph("The path to a shell script that describes how to build the package. If omitted, the build runs using stdenv's generic builder.")
                        },
                    ]
                },
                RecordField::Section{
                    title: String::from("Nix shell"),
                    fields: vec![
                        RecordFieldItem{
                            name: String::from("shellHook"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup::paragraph("A shell script to run after entering a nix-shell.")
                        }
                    ]
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
