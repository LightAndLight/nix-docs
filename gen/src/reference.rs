use std::io::{self, Write};

use crate::r#type::{RecordField, Type};

pub enum RecordFieldReference<'a> {
    Item {
        name: &'a str,
        r#type: &'a Type,
        optional: bool,
        docs: &'a str,
    },
    Section {
        title: &'a str,
        fields: Vec<RecordFieldReference<'a>>,
    },
}

impl<'a> RecordFieldReference<'a> {
    fn write_html<W: Write>(&self, mut buffer: W) -> io::Result<()> {
        fn inner(value: &RecordFieldReference, buffer: &mut dyn Write) -> io::Result<()> {
            match value {
                RecordFieldReference::Item {
                    name,
                    r#type,
                    optional,
                    docs,
                } => {
                    writeln!(buffer, "<h4 id=\"reference-inputs-{}\">{}</h4>", name, name)?;

                    write!(buffer, "<p><i>")?;
                    {
                        let buffer = &mut *buffer;
                        r#type.write_name(buffer)
                    }?;

                    write!(buffer, ", ")?;
                    if *optional {
                        write!(buffer, "optional")
                    } else {
                        write!(buffer, "required")
                    }?;
                    write!(buffer, "</i></p>")?;

                    writeln!(buffer, "<p>{}</p>", docs)?;

                    Ok(())
                }
                RecordFieldReference::Section { title, fields } => todo!(),
            }
        }
        inner(self, &mut buffer)
    }
}

pub enum Reference<'a> {
    Function {
        input: Option<Box<Reference<'a>>>,
        output: Option<Box<Reference<'a>>>,
    },
    Record {
        fields: Vec<RecordFieldReference<'a>>,
    },
}

impl<'a> Reference<'a> {
    pub fn from_type(ty: &'a Type) -> Option<Self> {
        match ty {
            Type::String | Type::Derivation | Type::Path | Type::List(_) => None,
            Type::Function { input, output } => {
                match (
                    Self::from_type(input).map(Box::new),
                    Self::from_type(output).map(Box::new),
                ) {
                    (None, None) => None,
                    (input, output) => Some(Reference::Function { input, output }),
                }
            }
            Type::Record { fields } => fields.as_ref().and_then(|fields| {
                if fields.is_empty() {
                    None
                } else {
                    Some(Reference::Record {
                        fields: fields
                            .iter()
                            .map(|field| match field {
                                RecordField::Item {
                                    name,
                                    optional,
                                    r#type,
                                    docs,
                                } => RecordFieldReference::Item {
                                    name,
                                    r#type,
                                    optional: *optional,
                                    docs,
                                },
                                RecordField::Section { title, fields } => todo!(),
                            })
                            .collect(),
                    })
                }
            }),
        }
    }

    pub fn write_html<W: Write>(&self, mut buffer: W) -> io::Result<()> {
        fn inner(value: &Reference, buffer: &mut dyn Write) -> io::Result<()> {
            match value {
                Reference::Function { input, output } => {
                    if let Some(input) = input {
                        writeln!(buffer, "<h3>Inputs</h3>")?;
                        inner(input, buffer)?;
                    }

                    if let Some(output) = output {
                        writeln!(buffer, "<h3>Outputs</h3>")?;
                        inner(output, buffer)?;
                    }

                    Ok(())
                }
                Reference::Record { fields } => fields.iter().try_for_each(|field| {
                    let buffer = &mut *buffer;
                    field.write_html(buffer)
                }),
            }
        }

        inner(self, &mut buffer)
    }
}