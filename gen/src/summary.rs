use std::io::{self, Write};

use crate::r#type::{RecordField, RecordFieldItem, RecordFields, Type};

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
                    writeln!(buffer, "<tr>")?;
                    writeln!(
                        buffer,
                        "<td><a href=\"#reference-inputs-{}\">{}</a>{}:</td>",
                        name,
                        name,
                        if *optional { "?" } else { "" }
                    )?;
                    write!(buffer, "<td>")?;
                    {
                        let buffer = &mut *buffer;
                        r#type.write_name(buffer)
                    }?;
                    writeln!(buffer, "</td>")?;
                    writeln!(buffer, "</tr>")
                }
                RecordFieldSummary::Section { title, fields } => {
                    writeln!(buffer, "<tr><td>")?;
                    writeln!(buffer, "  <span class=\"comment\"># {}</span>", title)?;
                    writeln!(buffer, "</td></tr>")?;
                    fields.iter().try_for_each(|field| {
                        let buffer = &mut *buffer;
                        field.write_html(buffer)
                    })?;
                    Ok(())
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
        rest: Option<&'a str>,
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
            Type::String | Type::Derivation | Type::Path | Type::List(_) | Type::Union(_, _) => {
                Summary::Simple(ty)
            }
            Type::Function { input, output } => {
                let input = Box::new(Self::from_type(input));
                let output = Box::new(Self::from_type(output));
                Summary::Function { input, output }
            }
            Type::Record { fields } => {
                fn from_record_field_item(field: &RecordFieldItem) -> RecordFieldSummary {
                    RecordFieldSummary::Item {
                        name: &field.name,
                        optional: field.optional,
                        r#type: &field.r#type,
                    }
                }

                match fields {
                    Some(RecordFields { fields, rest }) if !fields.is_empty() => Summary::Record {
                        fields: fields
                            .iter()
                            .map(|field| match field {
                                RecordField::Item(item) => from_record_field_item(item),
                                RecordField::Section { title, fields } => {
                                    RecordFieldSummary::Section {
                                        title,
                                        fields: fields.iter().map(from_record_field_item).collect(),
                                    }
                                }
                            })
                            .collect(),
                        rest: rest.as_ref().map(|rest| rest.name.as_str()),
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
                Summary::Record { fields, rest } => {
                    writeln!(buffer, "<div class=\"code\">")?;
                    write!(buffer, "<code>")?;
                    writeln!(
                        buffer,
                        "<p style=\"margin-top: 0.5rem; margin-bottom: 0.5rem;\">{{</p>"
                    )?;

                    writeln!(buffer, "<table style=\"padding-left: 1rem;\">")?;
                    writeln!(buffer, "<tbody>")?;

                    enum RecordFieldType {
                        Item,
                        Section,
                    }
                    let mut previous_field: Option<RecordFieldType> = None;
                    fields.iter().try_for_each(|field| {
                        match previous_field {
                            Some(RecordFieldType::Section) => {
                                writeln!(buffer, "<tr><td>&nbsp;</td></tr>")
                            }
                            _ => Ok(()),
                        }?;

                        previous_field = Some(match field {
                            RecordFieldSummary::Item { .. } => RecordFieldType::Item,
                            RecordFieldSummary::Section { .. } => RecordFieldType::Section,
                        });

                        let buffer = &mut *buffer;
                        field.write_html(buffer)
                    })?;

                    rest.iter().try_for_each(|rest| {
                        match previous_field {
                            Some(RecordFieldType::Section) => {
                                writeln!(buffer, "<tr><td>&nbsp;</td></tr>")
                            }
                            _ => Ok(()),
                        }?;

                        writeln!(
                            buffer,
                            "<tr><td><a href=\"#reference-inputs-{}\">{}</a>...</td></tr>",
                            urlencoding::encode(rest),
                            rest,
                        )
                    })?;

                    writeln!(buffer, "</tbody>")?;
                    writeln!(buffer, "</table>")?;

                    writeln!(
                        buffer,
                        "<p style=\"margin-top: 0.5rem; margin-bottom: 0.5rem\">}}</p>"
                    )?;
                    writeln!(buffer, "</code>")?;
                    writeln!(buffer, "</div>")?;
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
