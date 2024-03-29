mod derivation;
mod mk_derivation;

use nix_docs_gen::Documentation;
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn write_survey(buffer: &mut dyn Write) -> io::Result<()> {
    write!(
        buffer,
        r#"<p style="border-bottom: 1px solid lightgray; padding-bottom: 1.5em; text-align: center;">"#
    )?;
    write!(buffer, "Want to see more Nix documentation like this in the future, or have requests/suggestions? Fill out ")?;
    write!(buffer, r#"<a href="https://forms.gle/pHsadGb3mtSeCpuD9">"#)?;
    write!(buffer, "this anonymous Google form")?;
    write!(buffer, "</a>")?;
    write!(buffer, " or submit an issue on ")?;
    write!(
        buffer,
        r#"<a href="https://github.com/LightAndLight/nix-docs">"#
    )?;
    write!(buffer, "GitHub")?;
    write!(buffer, "</a>")?;
    write!(buffer, ".")?;
    writeln!(buffer, "</p>")
}

fn write_document(document: Documentation, buffer: &mut dyn Write) -> io::Result<()> {
    writeln!(buffer, "<!doctype html>")?;
    writeln!(buffer, "<head>")?;
    writeln!(buffer, "<meta charset=\"UTF-8\">")?;
    writeln!(buffer, r#"<meta name="google-site-verification" content="4JBoKbKcgU-sNf4ayd4jvMffHVzbfJn_gFVMp5kgvQ4" />"#)?;
    writeln!(buffer, "<title>{} - nix-docs</title>", document.title)?;
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
    writeln!(
        buffer,
        r#"<div style="display: flex; align-items: center;"><a href="index.html">nix-docs</a><span style="font-size: 1.2em; color: lightgray; font-weight: bold; padding-left: 0.5em; padding-right: 0.5em;">/</span>{}</div>"#,
        document.title
    )?;
    document.write_html(buffer)?;
    writeln!(buffer, "</body>")?;

    writeln!(buffer, "</html>")?;

    Ok(())
}

fn main() -> io::Result<()> {
    {
        let file = File::create("mkDerivation.html")?;
        let mut file = BufWriter::new(file);

        write_document(mk_derivation::docs(), &mut file)
    }?;

    {
        let file = File::create("derivation.html")?;
        let mut file = BufWriter::new(file);

        write_document(derivation::docs(), &mut file)
    }
}
