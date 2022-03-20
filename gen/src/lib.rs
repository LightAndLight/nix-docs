pub mod reference;
pub mod summary;
pub mod r#type;

use r#type::Type;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

use crate::{reference::Reference, summary::Summary};

#[derive(Debug)]
pub enum Error {
    ReadCborError(ciborium::de::Error<io::Error>),
    WriteCborError(ciborium::ser::Error<io::Error>),
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Serialize, Deserialize)]
pub struct Documentation {
    pub title: String,
    pub intro: String,
    pub r#type: Type,
}

impl Documentation {
    pub fn read_cbor<R: Read>(mut buffer: R) -> Result<Self> {
        fn inner(buffer: &mut dyn Read) -> Result<Documentation> {
            ciborium::de::from_reader(buffer).map_err(Error::ReadCborError)
        }
        inner(&mut buffer)
    }

    pub fn write_cbor<W: Write>(&self, buffer: &mut W) -> Result<()> {
        fn inner(value: &Documentation, buffer: &mut dyn Write) -> Result<()> {
            ciborium::ser::into_writer(value, buffer).map_err(Error::WriteCborError)
        }
        inner(self, buffer)
    }

    pub fn write_html<W: Write>(&self, mut buffer: W) -> Result<()> {
        fn inner(value: &Documentation, buffer: &mut dyn Write) -> Result<()> {
            writeln!(buffer, "<!doctype html>")?;
            writeln!(buffer, "<head>")?;
            writeln!(buffer, "<meta charset=\"UTF-8\">")?;
            writeln!(buffer, "<title>{}</title>", value.title)?;
            writeln!(buffer, "</head>")?;

            writeln!(buffer, "<body>")?;
            writeln!(buffer, "<h1>{}</h1>", value.title)?;

            write!(buffer, "<p><i>")?;
            {
                let buffer = &mut *buffer;
                value.r#type.write_name(buffer)
            }?;
            write!(buffer, "</i></p>")?;

            writeln!(buffer, "<p>{}</p>", value.intro)?;
            {
                let buffer = &mut *buffer;
                Summary::from_type(&value.r#type).write_html(buffer)
            }?;

            writeln!(buffer, "<h2>Reference</h2>")?;
            Reference::from_type(&value.r#type)
                .iter()
                .try_for_each(|reference| {
                    let buffer = &mut *buffer;
                    reference.write_html(buffer)
                })?;

            writeln!(buffer, "</body>")?;
            Ok(())
        }
        inner(self, &mut buffer)
    }

    pub fn write_syntax<W: Write>(&self, buffer: &mut W) -> Result<()> {
        fn inner(value: &Documentation, buffer: &mut dyn Write) -> Result<()> {
            todo!()
        }
        inner(self, buffer)
    }
}
