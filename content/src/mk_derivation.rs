use nix_docs_gen::{
    markup::{Block, Markup, Text},
    r#type::{RecordField, RecordFieldItem, Type},
    Documentation,
};

pub fn docs() -> Documentation {
    Documentation {
        title: String::from("mkDerivation"),
        intro: String::from("mkDerivation is the main function used to build packages with the standard environment."),
        r#type: Type::Function{
            input: Box::new(Type::Record{fields: Some(vec![
                RecordField::Section{
                    title: String::from("Core Attributes"),
                    fields: vec![
                        RecordFieldItem{
                            name: String::from("name"),
                            optional: false,
                            r#type: Type::String,
                            docs: Markup(vec![
                                Block::Paragraph(vec![
                                    Text::Plain(String::from("The derivation name.")),
                                ]),
                                Block::Paragraph(vec![
                                    Text::Plain(String::from("Can be omitted if ")),
                                    Text::Link{destination: String::from("#reference-inputs-pname"), text: String::from("pname")},
                                    Text::Plain(String::from(" and ")),
                                    Text::Link{destination: String::from("#reference-inputs-version"), text: String::from("version")},
                                    Text::Plain(String::from(" are set, in which case it is automatically set to ")),
                                    Text::Code(String::from("${pname}-${version}")),
                                    Text::Plain(String::from(".")),
                                ]),
                                Block::Paragraph(vec![Text::Bold(String::from("Examples"))]),
                                Block::Code(String::from("stdenv.mkDerivation {\n  name = \"libfoo-1.2.3\";\n  src = ./src;\n}\n")),
                                Block::Code(String::from("stdenv.mkDerivation {\n  pname = \"libfoo\";\n  version = \"1.2.3\";\n  src = ./src;\n}")),
                            ])
                        },
                        RecordFieldItem{
                            name: String::from("pname"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup::paragraph("The package name.")
                        },
                        RecordFieldItem{
                            name: String::from("version"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup::paragraph("The package version.")
                        },
                        RecordFieldItem{
                            name: String::from("src"),
                            optional: false,
                            r#type: Type::Path,
                            docs: Markup(vec![
                                Block::paragraph("The package source directory."),
                                Block::Paragraph(vec![Text::bold("Examples")]),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo-1.2.3\";\n  src = ./src;\n}"),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo-1.2.3\";\n  src = fetchurl {\n    url = http://example.org/libfoo-1.2.3.tar.bz2;\n    sha256 = \"0x2g1jqygyr5wiwg4ma1nd7w4ydpy82z9gkcv8vh2v8dn3y58v5m\";\n  };\n}"),
                            ])
                        },
                    ]
                },
                RecordField::Section{
                    title: String::from("Building"),
                    fields: vec![
                        RecordFieldItem{
                            name: String::from("buildInputs"),
                            optional: true,
                            r#type: Type::List(Box::new(Type::Derivation)),
                            docs: Markup(vec![
                                Block::paragraph("The package's build-time dependencies."),
                                Block::Paragraph(vec![
                                    Text::plain("This attribute ensures that the outputs of the dependencies (e.g. "),
                                    Text::code("bin"),
                                    Text::plain(", "),
                                    Text::code("includes"),
                                    Text::plain(") are in scope during the package build."),
                                ]),
                                Block::Paragraph(vec![Text::bold("Example")]),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo\";\n  src = ./src;\n  buildInputs = [libbar perl ncurses];\n}")
                            ])
                        },
                        RecordFieldItem{
                            name: String::from("buildPhase"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup(vec![
                                Block::paragraph("A shell script to run during the build phase."),
                                Block::Paragraph(vec![
                                    Text::plain("If omitted, the build phase will run "),
                                    Text::code("make"),
                                    Text::plain("."),
                                ]),
                                Block::Paragraph(vec![Text::bold("Example")]),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo\";\n  src = ./src;\n  buildPhase = ''\n    gcc foo.c -o foo\n  '';\n}")
                            ])
                        },
                        RecordFieldItem{
                            name: String::from("installPhase"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup(vec![
                                Block::paragraph("A shell script to run during the install phase."),
                                Block::Paragraph(vec![
                                    Text::plain("If omitted, the install phase will run "),
                                    Text::code("make install"),
                                    Text::plain("."),
                                ]),
                                Block::Paragraph(vec![Text::bold("Example")]),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo\";\n  src = ./src;\n  buildPhase = ''\n    gcc foo.c -o foo\n  '';\n  installPhase = ''\n    mkdir -p $out/bin\n    cp foo $out/bin\n  '';\n}")
                            ])
                        },
                        RecordFieldItem{
                            name: String::from("builder"),
                            optional: true,
                            r#type: Type::Path,
                            docs: Markup(vec![
                                Block::paragraph("A path to a shell script that describes how to build the package."),
                                Block::paragraph("If omitted, the build runs using stdenv's generic builder."),
                                Block::Paragraph(vec![Text::bold("Example")]),
                                Block::code("# builder.sh\nsource $stdenv/setup\n\nbuildPhase() {\n  echo \"... this is my custom build phase ...\"\n  gcc foo.c -o foo\n}\n\ninstallPhase() {\n  mkdir -p $out/bin\n  cp foo $out/bin\n}\n\ngenericBuild"),
                                Block::code("stdenv.mkDerivation {\n  name = \"libfoo\";\n  src = ./src;\n  builder = ./builder.sh;\n}"),
                            ])
                        },
                    ]
                },
                RecordField::Section{
                    title: String::from("Nix shell"),
                    fields: vec![
                        RecordFieldItem{
                            name: String::from("shellHook"),
                            optional: true,
                            r#type: Type::String,
                            docs: Markup(vec![
                                Block::paragraph("A script to run after entering a Nix shell."),
                                Block::Paragraph(vec![
                                    Text::plain("If "),
                                    Text::code("shellHook"),
                                    Text::plain(" is defined, it will be run in the "),
                                    Text::code("nix-shell"),
                                    Text::plain(" after "),
                                    Text::code("$stdenv/setup"),
                                    Text::plain(" has been sourced by the package's "),
                                    Text::Link{destination: String::from("#reference-inputs-builder"), text: String::from("builder")},
                                    Text::plain(".")
                                ]),
                                Block::Paragraph(vec![Text::bold("Example")]),
                                Block::code("stdenv.mkDerivation {\n  name = \"my-package\";\n  src = ./src;\n  shellHook = ''\n    echo \"Hello shell!\"\n  '';\n}")
                            ])
                        }
                    ]
                },
            ])}),
            output: Box::new(Type::Derivation),
        }
    }
}
