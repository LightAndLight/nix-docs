use clap::{Parser, Subcommand};
use nix_docs_gen::Documentation;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

#[derive(Subcommand)]
enum Command {
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
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
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

fn main() -> nix_docs_gen::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Write { file } => write_file(&file),
        Command::Gen { file } => generate(&file),
    }
}
