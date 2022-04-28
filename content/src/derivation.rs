use nix_docs_gen::{
    markup::{Block, Markup, Text, ListOrdering, ListItem},
    r#type::{RecordField, RecordFieldItem, Type, RecordFields, RecordRest},
    Documentation,
};

pub fn docs() -> Documentation {
    Documentation {
        title: String::from("derivation"),
        intro: String::from("The built-in function for creating derivations."),
        r#type: Type::Function {
            input: Box::new(Type::Record {
                fields: Some(RecordFields{
                    fields: vec![
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
                        }),
                        RecordField::Item(RecordFieldItem {
                            name: String::from("builder"),
                            optional: false,
                            r#type: Type::Union(Box::new(Type::Path), Box::new(Type::Derivation)),
                            docs: Markup(vec![
                                Block::paragraph("An executable file used build the derivation."),
                                Block::paragraph("When the derivation is built, the builder is executed in an isolated temporary directory."),
                                Block::Paragraph(vec![Text::bold("Arguments")]),
                                Block::Paragraph(vec![
                                    Text::plain("The builder is passed any command-line arguments from "),
                                    Text::Link{destination: String::from("#reference-inputs-args"), text: String::from("args")},
                                    Text::plain("."),
                                ]),
                                Block::Paragraph(vec![Text::bold("Environment Variables")]),
                                Block::paragraph("Every input attribute is available to the builder as an environment variable. Each attribute value is translated to a string before it is stored."),
                                Block::paragraph("Values are translated as follows:"),
                                Block::List{
                                    ordering: ListOrdering::Unordered,
                                    items: vec![
                                        ListItem::paragraph(vec![
                                            Text::code("string"),
                                            Text::plain(", "),
                                            Text::code("number"),
                                            Text::plain(": copied verbatim."),
                                        ]),
                                        ListItem::paragraph(vec![
                                            Text::code("path"),
                                            Text::plain(": the referenced file is copied to the store, and the environment variable is set to the resulting store location."),
                                        ]),
                                        ListItem::paragraph(vec![
                                            Text::code("derivation"),
                                            Text::plain(": the derivation is built and the environment variable is set to the derivation's default output path."),
                                        ]),
                                        ListItem::paragraph(vec![
                                            Text::code("list[string | number | path | derivation]"),
                                            Text::plain(": each item is translated individually, and the environment variable is set to the space-separated concatenation of the results."),
                                        ]),
                                        ListItem::paragraph(vec![
                                            Text::code("bool"),
                                            Text::plain(": "),
                                            Text::code("true"),
                                            Text::plain(" is translated to 1 and "),
                                            Text::code("false"),
                                            Text::plain(": is translated to the empty string."),
                                        ]),
                                        ListItem::paragraph(vec![
                                            Text::code("null"),
                                            Text::plain(": translated to the empty string."),
                                        ]),
                                        ListItem::paragraph(vec![
                                            Text::plain("For each item of "),
                                            Text::Link{destination: String::from("#reference-inputs-outputs"), text:String::from("outputs")}, 
                                            Text::plain(" there is an environment variable of the same name that is set to the store path for that output."),
                                        ]),
                                    ]
                                },
                                Block::Paragraph(vec![Text::bold("Outputs")]),
                                Block::Paragraph(vec![
                                    Text::plain("The builder must create a file or directory for each item in "),
                                    Text::Link{destination: String::from("#reference-inputs-outputs"), text: String::from("outputs")},
                                    Text::plain(". If "),
                                    Text::Link{destination: String::from("#reference-inputs-outputs"), text: String::from("outputs")},
                                    Text::plain(" is omitted then it will default to "),
                                    Text::code("[\"out\"]"),
                                    Text::plain("."),
                                ]),
                                Block::Paragraph(vec![Text::bold("Examples")]),
                                Block::code("# builder.sh\n#! /bin/sh\necho \"hello\" > $out"),
                                Block::code("derivation {\n  name = \"hello\";\n  system = builtins.currentSystem;\n  builder = ./builder.sh;\n}"),
                                Block::paragraph("&nbsp;"),
                                Block::code("# builder.sh\n#! /bin/sh\necho \"hello\" > $one\necho \"goodbye\" > $two"),
                                Block::code("derivation {\nname = \"hello-goodbye\";\n  system = builtins.currentSystem;\n  builder = ./builder.sh;\n  outputs = [\"one\" \"two\"];\n}"),
                            ]),

                        }),
                        RecordField::Item(RecordFieldItem {
                            name: String::from("args"),
                            optional: true,
                            r#type: Type::List(Box::new(Type::String)),
                            docs: Markup(vec![
                                Block::paragraph("A list of command line arguments to be passed to the builder."),
                                Block::Paragraph(vec![
                                    Text::plain("The list items can be any value that is convertible to a string. See "),
                                    Text::Link{destination: String::from("#reference-inputs-builder"), text: String::from("builder")},
                                    Text::plain(" for how these values are translated.")
                                ]),
                                Block::Paragraph(vec![Text::bold("Example")]),
                                Block::code("# builder.sh\n#! /bin/sh\necho $1\necho $2"),
                                Block::code("derivation {\n  name = \"say-hello-world\";\n  system = builtins.currentSystem;\n  builder = ./builder.sh;\n  args = [\"hello\" \"world\"];\n}"),
                            ]),
                        }),
                        RecordField::Item(RecordFieldItem {
                            name: String::from("outputs"),
                            optional: true,
                            r#type: Type::List(Box::new(Type::String)),
                            docs: Markup(vec![
                                Block::Paragraph(vec![
                                    Text::plain("A list of outputs from the derivation. Defaults to "),
                                    Text::code("[\"out\"]"),
                                    Text::plain(".")
                                ]),
                                Block::Paragraph(vec![
                                    Text::plain("The derivation will produce one store path per output."),
                                    Text::plain("This allows the outputs to be downloaded and garbage collected separately."),
                                ]),
                                Block::Paragraph(vec![Text::bold("Example")]),
                                Block::code("# builder.sh\n#! /bin/sh\necho \"hello\" > $one\necho \"world\" > $two"),
                                Block::code("derivation {\n  name = \"write-hello-world\";\n  system = builtins.currentSystem;\n  builder = ./builder.sh;\n  args = [\"one\" \"two\"];\n}"),
                            ]),
                        }),
                    ],
                    rest: Some(RecordRest{
                        name: String::from("user-defined attributes"),
                        docs: Markup(vec![
                            Block::paragraph("The user may define any number of additional attributes."),
                            Block::Paragraph(vec![
                                Text::plain("Each user-defined attribute is converted to a string and made available to the builder through an environment variable. See "),
                                Text::Link{destination: String::from("#reference-inputs-builder"), text: String::from("builder")},
                                Text::plain(" for how these values are translated."),
                            ]),
                            Block::Paragraph(vec![Text::bold("Example")]),
                            Block::code("# builder.sh\n#! /bin/sh\necho \"$result\" > $out\necho \"$count\"\necho \"$word\""),
                            Block::code("derivation {\n  name = \"42-pelican\";\n  system = builtins.currentSystem;\n  builder = ./builder.sh;\n  result = [ \"one\" \"two\" 3 ];\n  count = 42;\n  word = \"pelican\";\n}"),
                        ])
                    }),
                }),
            }),
            output: Box::new(Type::Derivation),
        },
    }
}
