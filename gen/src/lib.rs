pub mod markup;
pub mod reference;
pub mod summary;
pub mod r#type;

use crate::{reference::Reference, summary::Summary};
use r#type::Type;
use std::io::{self, Write};

pub struct Documentation {
    pub title: String,
    pub intro: String,
    pub r#type: Type,
}

impl Documentation {
    pub fn write_html(&self, buffer: &mut dyn Write) -> io::Result<()> {
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
