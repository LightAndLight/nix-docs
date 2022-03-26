use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::markup::Markup;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordFieldItem {
    pub name: String,
    pub optional: bool,
    pub r#type: Type,
    pub docs: Markup,
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
pub struct RecordRest {
    pub name: String,
    pub docs: Markup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordFields {
    pub fields: Vec<RecordField>,
    pub rest: Option<RecordRest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    String,
    Derivation,
    Path,
    List(Box<Type>),
    Union(Box<Type>, Box<Type>),
    Function { input: Box<Type>, output: Box<Type> },
    Record { fields: Option<RecordFields> },
}

impl Type {
    pub fn write_name<W: Write>(&self, mut buffer: W) -> io::Result<()> {
        fn inner(value: &Type, buffer: &mut dyn Write) -> io::Result<()> {
            match value {
                Type::String => write!(buffer, "string"),
                Type::Derivation => write!(buffer, "derivation"),
                Type::Path => write!(buffer, "path"),
                Type::Union(a, b) => {
                    inner(a, buffer)?;
                    write!(buffer, " | ")?;
                    inner(b, buffer)
                }
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
