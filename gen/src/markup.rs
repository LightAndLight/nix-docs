use std::io::{self, Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Markup(pub Vec<Block>);

impl Markup {
    pub fn paragraph(text: &str) -> Self {
        Markup(vec![Block::Paragraph(vec![Text::Plain(String::from(
            text,
        ))])])
    }

    pub fn write_html(&self, buffer: &mut dyn Write) -> io::Result<()> {
        self.0.iter().try_for_each(|block| block.write_html(buffer))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Text {
    Plain(String),
    Code(String),
    Bold(String),
    Link { destination: String, text: String },
}

impl Text {
    pub fn plain(text: &str) -> Self {
        Text::Plain(String::from(text))
    }

    pub fn code(text: &str) -> Self {
        Text::Code(String::from(text))
    }

    pub fn bold(text: &str) -> Self {
        Text::Bold(String::from(text))
    }

    pub fn write_html(&self, buffer: &mut dyn Write) -> io::Result<()> {
        match self {
            Text::Plain(text) => buffer.write_all(text.as_bytes()),
            Text::Code(text) => {
                write!(buffer, "<code>")?;
                buffer.write_all(text.as_bytes())?;
                write!(buffer, "</code>")?;
                Ok(())
            }
            Text::Bold(text) => {
                write!(buffer, "<b>")?;
                buffer.write_all(text.as_bytes())?;
                write!(buffer, "</b>")?;
                Ok(())
            }
            Text::Link { destination, text } => {
                write!(buffer, "<a href={}>", destination)?;
                buffer.write_all(text.as_bytes())?;
                write!(buffer, "</a>")?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListOrdering {
    Ordered,
    Unordered,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem(pub Vec<Block>);

impl ListItem {
    pub fn block(block: Block) -> Self {
        ListItem(vec![block])
    }

    pub fn paragraph(text: Vec<Text>) -> Self {
        Self::block(Block::Paragraph(text))
    }

    pub fn write_html(&self, buffer: &mut dyn Write) -> io::Result<()> {
        writeln!(buffer, "<li>")?;
        self.0
            .iter()
            .try_for_each(|block| block.write_html(buffer))?;
        writeln!(buffer, "</li>")?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Block {
    Paragraph(Vec<Text>),
    Code(String),
    List {
        ordering: ListOrdering,
        items: Vec<ListItem>,
    },
}

impl Block {
    pub fn paragraph(text: &str) -> Self {
        Block::Paragraph(vec![Text::Plain(String::from(text))])
    }

    pub fn code(code: &str) -> Self {
        Block::Code(String::from(code))
    }

    pub fn write_html(&self, buffer: &mut dyn Write) -> io::Result<()> {
        match self {
            Block::Paragraph(paragraph) => {
                writeln!(buffer, "<p>")?;
                paragraph
                    .iter()
                    .try_for_each(|text| text.write_html(buffer))?;
                writeln!(buffer, "</p>")?;
                Ok(())
            }
            Block::Code(code) => {
                write!(buffer, "<pre><code>")?;
                buffer.write_all(code.as_bytes())?;
                writeln!(buffer, "</code></pre>")?;
                Ok(())
            }
            Block::List { ordering, items } => {
                match ordering {
                    ListOrdering::Ordered => writeln!(buffer, "<ol>"),
                    ListOrdering::Unordered => writeln!(buffer, "<ul>"),
                }?;
                items.iter().try_for_each(|item| item.write_html(buffer))?;
                match ordering {
                    ListOrdering::Ordered => writeln!(buffer, "</ol>"),
                    ListOrdering::Unordered => writeln!(buffer, "</ul>"),
                }?;
                Ok(())
            }
        }
    }
}
