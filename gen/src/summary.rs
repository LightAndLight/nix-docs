use std::io::{self, Write};

use crate::r#type::{RecordField, Type};

pub enum RecordFieldSummary<'a> {
    Item {
        name: &'a str,
        optional: bool,
        r#type: &'a Type,
    },
    Section {
        title: &'a str,
        fields: Vec<RecordFieldSummary<'a>>,
    },
}

impl<'a> RecordFieldSummary<'a> {
    fn write_html<W: Write>(&self, mut buffer: W) -> io::Result<()> {
        fn inner(field: &RecordFieldSummary, buffer: &mut dyn Write) -> io::Result<()> {
            match field {
                RecordFieldSummary::Item {
                    name,
                    optional,
                    r#type,
                } => {
                    write!(
                        buffer,
                        "  <a href=\"#reference-inputs-{}\">{}</a>{}: ",
                        name,
                        name,
                        if *optional { "?" } else { "" }
                    )?;
                    {
                        let buffer = &mut *buffer;
                        r#type.write_name(buffer)?;
                    }
                    writeln!(buffer)
                }
                RecordFieldSummary::Section { title, fields } => {
                    // # $title
                    // fields
                    todo!()
                }
            }
        }

        inner(self, &mut buffer)
    }
}

pub enum Summary<'a> {
    Function {
        input: Box<Summary<'a>>,
        output: Box<Summary<'a>>,
    },
    Record {
        fields: Vec<RecordFieldSummary<'a>>,
    },
    Simple(&'a Type),
}

impl<'a> Summary<'a> {
    pub fn simple(&self) -> bool {
        match self {
            Summary::Function { .. } | Summary::Record { .. } => false,
            Summary::Simple(_) => true,
        }
    }

    pub fn from_type(ty: &'a Type) -> Self {
        match ty {
            Type::String | Type::Derivation | Type::Path | Type::List(_) => Summary::Simple(ty),
            Type::Function { input, output } => {
                let input = Box::new(Self::from_type(input));
                let output = Box::new(Self::from_type(output));
                Summary::Function { input, output }
            }
            Type::Record { fields } => {
                fn record_field_summary(field: &RecordField) -> RecordFieldSummary {
                    match field {
                        RecordField::Item {
                            name,
                            optional,
                            r#type,
                            docs,
                        } => RecordFieldSummary::Item {
                            name,
                            optional: *optional,
                            r#type,
                        },
                        RecordField::Section { title, fields } => RecordFieldSummary::Section {
                            title,
                            fields: fields.iter().map(record_field_summary).collect(),
                        },
                    }
                }

                match fields {
                    Some(fields) if !fields.is_empty() => Summary::Record {
                        fields: fields.iter().map(record_field_summary).collect(),
                    },
                    _ => Summary::Simple(ty),
                }
            }
        }
    }

    pub fn write_html<W: Write>(&self, mut buffer: W) -> io::Result<()> {
        fn inner(value: &Summary, buffer: &mut dyn Write) -> io::Result<()> {
            match value {
                Summary::Function { input, output } => {
                    writeln!(
                        buffer,
                        "<h2>Input{}</h2>",
                        if input.simple() { "" } else { "s" }
                    )?;
                    {
                        let buffer = &mut *buffer;
                        input.write_html(buffer)
                    }?;

                    writeln!(
                        buffer,
                        "<h2>Output{}</h2>",
                        if output.simple() { "" } else { "s" }
                    )?;
                    {
                        let buffer = &mut *buffer;
                        output.write_html(buffer)
                    }?;

                    Ok(())
                }
                Summary::Record { fields } => {
                    write!(buffer, "<pre><code>")?;
                    writeln!(buffer, "{{")?;
                    fields.iter().try_for_each(|field| {
                        let buffer = &mut *buffer;
                        field.write_html(buffer)
                    })?;
                    writeln!(buffer, "}}")?;
                    writeln!(buffer, "</pre></code>")?;
                    Ok(())
                }
                Summary::Simple(ty) => {
                    write!(buffer, "<i><p>")?;
                    {
                        let buffer = &mut *buffer;
                        ty.write_name(buffer)
                    }?;
                    writeln!(buffer, "</i></p>")
                }
            }
        }
        inner(self, &mut buffer)
    }
}
