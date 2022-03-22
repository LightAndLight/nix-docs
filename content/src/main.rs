mod mk_derivation;

use std::fs::File;
use std::io::BufWriter;

fn main() -> nix_docs_gen::Result<()> {
    let file = File::create("stdenv-mkDerivation.html").map_err(nix_docs_gen::Error::IoError)?;
    let mut file = BufWriter::new(file);
    mk_derivation::docs().write_html(&mut file)
}
