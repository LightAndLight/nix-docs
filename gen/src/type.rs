use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
pub enum RecordField {
    Item {
        name: String,
        optional: bool,
        r#type: Type,
        docs: String,
    },
    Section {
        title: String,
        fields: Vec<RecordField>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    String,
    Derivation,
    Path,
    List(Box<Type>),
    Function { input: Box<Type>, output: Box<Type> },
    Record { fields: Option<Vec<RecordField>> },
}

impl Type {
    pub fn write_name<W: Write>(&self, mut buffer: W) -> io::Result<()> {
        fn inner(value: &Type, buffer: &mut dyn Write) -> io::Result<()> {
            match value {
                Type::String => write!(buffer, "string"),
                Type::Derivation => write!(buffer, "derivation"),
                Type::Path => write!(buffer, "path"),
                Type::Function { .. } => write!(buffer, "function"),
                Type::Record { .. } => write!(buffer, "record"),
                Type::List(value) => {
                    write!(buffer, "list[")?;
                    inner(value, buffer)?;
                    write!(buffer, "]")?;
                    Ok(())
                }
            }
        }
        inner(self, &mut buffer)
    }
}
