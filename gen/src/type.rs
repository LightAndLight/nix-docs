use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordFieldItem {
    pub name: String,
    pub optional: bool,
    pub r#type: Type,
    pub docs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RecordField {
    Item(RecordFieldItem),
    Section {
        title: String,
        fields: Vec<RecordFieldItem>,
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
