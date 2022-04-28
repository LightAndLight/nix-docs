pub mod markup;
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
    pub fn read_cbor(mut buffer: &mut dyn Read) -> Result<Self> {
        fn inner(buffer: &mut dyn Read) -> Result<Documentation> {
            ciborium::de::from_reader(buffer).map_err(Error::ReadCborError)
        }
        inner(&mut buffer)
    }

    pub fn write_cbor(&self, buffer: &mut dyn Write) -> Result<()> {
        ciborium::ser::into_writer(self, buffer).map_err(Error::WriteCborError)
    }

    pub fn write_html(&self, buffer: &mut dyn Write) -> Result<()> {
        writeln!(buffer, "<h1>{}</h1>", self.title)?;

        write!(buffer, "<p><i>")?;
        {
            let buffer = &mut *buffer;
            self.r#type.write_name(buffer)
        }?;
        write!(buffer, "</i></p>")?;

        writeln!(buffer, "<p>{}</p>", self.intro)?;
        {
            let buffer = &mut *buffer;
            Summary::from_type(&self.r#type).write_html(buffer)
        }?;

        writeln!(buffer, "<h2>Reference</h2>")?;
        Reference::from_type(&self.r#type)
            .iter()
            .try_for_each(|reference| {
                let buffer = &mut *buffer;
                reference.write_html(buffer)
            })?;

        Ok(())
    }
}
