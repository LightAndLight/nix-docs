use std::io::{self, Write};

use crate::{
    markup::Markup,
    r#type::{RecordField, RecordFieldItem, RecordFields, Type},
};

pub enum RecordFieldReference<'a> {
    Item {
        name: &'a str,
        r#type: &'a Type,
        optional: bool,
        docs: &'a Markup,
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
                    writeln!(
                        buffer,
                        "<h5 id=\"reference-inputs-{}\" style=\"font-weight: 500;\"><code>{}</code></h5>",
                        name, name
                    )?;

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

                    {
                        let buffer = &mut *buffer;
                        docs.write_html(buffer)
                    }?;

                    Ok(())
                }
                RecordFieldReference::Section { title, fields } => {
                    writeln!(buffer, "<h4>{}</h4>", title)?;
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

pub struct RecordRest<'a> {
    pub name: &'a str,
    pub docs: &'a Markup
}

pub enum Reference<'a> {
    Function {
        input: Option<Box<Reference<'a>>>,
        output: Option<Box<Reference<'a>>>,
    },
    Record {
        fields: Vec<RecordFieldReference<'a>>,
        rest: Option<RecordRest<'a>>,
    },
}

impl<'a> Reference<'a> {
    pub fn from_type(ty: &'a Type) -> Option<Self> {
        match ty {
            Type::String | Type::Derivation | Type::Path | Type::List(_) | Type::Union(_, _) => {
                None
            }
            Type::Function { input, output } => {
                match (
                    Self::from_type(input).map(Box::new),
                    Self::from_type(output).map(Box::new),
                ) {
                    (None, None) => None,
                    (input, output) => Some(Reference::Function { input, output }),
                }
            }
            Type::Record { fields } => fields.as_ref().and_then(|RecordFields { fields, rest }| {
                fn from_record_field_item(field: &RecordFieldItem) -> RecordFieldReference {
                    RecordFieldReference::Item {
                        name: &field.name,
                        r#type: &field.r#type,
                        optional: field.optional,
                        docs: &field.docs,
                    }
                }

                if fields.is_empty() {
                    None
                } else {
                    Some(Reference::Record {
                        fields: fields
                            .iter()
                            .map(|field| match field {
                                RecordField::Item(item) => from_record_field_item(item),
                                RecordField::Section { title, fields } => {
                                    RecordFieldReference::Section {
                                        title,
                                        fields: fields.iter().map(from_record_field_item).collect(),
                                    }
                                }
                            })
                            .collect(),
                        rest: rest.as_ref().map(|rest| RecordRest{name: rest.name.as_str(), docs: &rest.docs}),
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
                Reference::Record { fields, rest } => {
                    fields.iter().try_for_each(|field| {
                        let buffer = &mut *buffer;
                        field.write_html(buffer)
                    })?;

                    rest.iter().try_for_each(|rest| {
                        writeln!(
                            buffer,
                            "<h5 id=\"reference-inputs-{}\" style=\"font-weight: 500;\"><code>{}</code></h5>",
                            urlencoding::encode(rest.name), 
                            rest.name,
                        )?;
                        rest.docs.write_html(buffer)?;
                        Ok(())
                    })
                }
            }
        }

        inner(self, &mut buffer)
    }
}
