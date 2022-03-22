use nix_docs_gen::{
    markup::{Block, Markup, Text, ListOrdering, ListItem},
    r#type::{RecordField, RecordFieldItem, Type},
    Documentation,
};

pub fn docs() -> Documentation {
    Documentation {
        title: String::from("derivation"),
        intro: String::from("The built-in function for creating derivations."),
        r#type: Type::Function {
            input: Box::new(Type::Record {
                fields: Some(vec![
                    RecordField::Item(RecordFieldItem {
                        name: String::from("name"),
                        optional: false,
                        r#type: Type::String,
                        docs: Markup(vec![
                            Block::paragraph("The derivation name."),
                            Block::paragraph("A name must be a non-empty string consisting only of alphanumeric characters and the symbols +-._?=. Names must not begin with a period."),
                        ]),
                    }),
                    RecordField::Item(RecordFieldItem {
                        name: String::from("system"),
                        optional: false,
                        r#type: Type::String,
                        docs: Markup(vec![
                            Block::paragraph("A platform identifier."),
                            Block::Paragraph(vec![
                                Text::plain("The resulting derivation can only be built on a machine with a matching platform identifier. Platform identifiers should be specified in "),
                                Text::Link{destination: String::from("https://clang.llvm.org/docs/CrossCompilation.html"), text: String::from("Clang target-triple format")},
                                Text::plain(".")
                            ]),
                            Block::paragraph("Valid platform identifiers include:"),
                            Block::List{
                                ordering: ListOrdering::Unordered, 
                                items: vec![
                                    ListItem::block(Block::Paragraph(vec![Text::code("i386-linux")])),
                                    ListItem::block(Block::Paragraph(vec![Text::code("x86_64-darwin")])),
                                    ListItem::block(Block::Paragraph(vec![Text::code("armv7-linux-androideabi")])),
                                ]
                            },
                            Block::Paragraph(vec![
                                Text::plain("The current platform identifier can be retrieved using "),
                                Text::Link{destination: String::from("https://nixos.org/nix/manual/#ssec-builtins"), text: String::from("builtins.currentSystem")},
                                Text::plain(".")
                            ]),
                            Block::Paragraph(vec![Text::bold("Example")]),
                            Block::code("derivation {\n  name = \"hello\";\n  system = builtins.currentSystem;\n  builder = ./builder.sh;\n}"),
                        ]),
                    })
                ]),
            }),
            output: Box::new(Type::Derivation),
        },
    }
}
