mod derivation;
mod mk_derivation;

use nix_docs_gen::Documentation;
use std::fs::File;
use std::io::{BufWriter, Write};

fn write_document<W: Write>(document: Documentation, mut buffer: W) -> nix_docs_gen::Result<()> {
    writeln!(buffer, "<!doctype html>")?;
    writeln!(buffer, "<head>")?;
    writeln!(buffer, "<meta charset=\"UTF-8\">")?;
    writeln!(buffer, "<title>{}</title>", document.title)?;
    writeln!(buffer, "<link rel=\"stylesheet\" href=\"style.css\">")?;
    writeln!(
        buffer,
        "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">"
    )?;
    writeln!(
        buffer,
        "<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>"
    )?;
    writeln!(buffer, "<link href=\"https://fonts.googleapis.com/css2?family=Overpass:wght@100;200;300;400;500;600;700;800;900&family=Roboto:wght@100;300;400;500;700;900&display=swap\" rel=\"stylesheet\">")?;
    writeln!(buffer, "</head>")?;

    writeln!(buffer, "<body>")?;
    document.write_html(&mut buffer)?;
    writeln!(buffer, "</body>")?;

    writeln!(buffer, "</html>")?;

    Ok(())
}

fn main() -> nix_docs_gen::Result<()> {
    {
        let file =
            File::create("stdenv-mkDerivation.html").map_err(nix_docs_gen::Error::IoError)?;
        let mut file = BufWriter::new(file);

        write_document(mk_derivation::docs(), &mut file)
    }?;

    {
        let file = File::create("derivation.html").map_err(nix_docs_gen::Error::IoError)?;
        let mut file = BufWriter::new(file);

        write_document(derivation::docs(), &mut file)
    }
}
