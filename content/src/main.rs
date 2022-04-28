mod derivation;
mod mk_derivation;

use nix_docs_gen::Documentation;
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn write_survey(buffer: &mut dyn Write) -> io::Result<()> {
    write!(buffer, "<p>")?;
    write!(buffer, "If you want to see more Nix documentation like this in the future, or have any requests or suggestions, then please fill out ")?;
    write!(buffer, r#"<a href="https://forms.gle/pHsadGb3mtSeCpuD9">"#)?;
    write!(buffer, "this anonymous Google form")?;
    write!(buffer, "</a>")?;
    write!(buffer, ".")?;
    writeln!(buffer, "</p>")
}

fn write_document(document: Documentation, buffer: &mut dyn Write) -> nix_docs_gen::Result<()> {
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
    write_survey(buffer)?;
    document.write_html(buffer)?;
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
