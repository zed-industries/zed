use gpui::{FontWeight, HighlightStyle, SharedString, TestAppContext};
use lsp::{Documentation, MarkupContent, MarkupKind};

use project::lsp_command::signature_help::SignatureHelp;

fn current_parameter() -> HighlightStyle {
    HighlightStyle {
        font_weight: Some(FontWeight::EXTRA_BOLD),
        ..Default::default()
    }
}

#[gpui::test]
fn test_create_signature_help_markdown_string_1(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn test(foo: u8, bar: &str)".to_string(),
            documentation: Some(Documentation::String(
                "This is a test documentation".to_string(),
            )),
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test(foo: u8, bar: &str)"),
            vec![(8..15, current_parameter())]
        )
    );
    assert_eq!(
        signature
            .documentation
            .unwrap()
            .update(cx, |documentation, _| documentation.source().to_owned()),
        "This is a test documentation",
    )
}

#[gpui::test]
fn test_create_signature_help_markdown_string_2(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn test(foo: u8, bar: &str)".to_string(),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "This is a test documentation".to_string(),
            })),
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(1),
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test(foo: u8, bar: &str)"),
            vec![(17..26, current_parameter())]
        )
    );
    assert_eq!(
        signature
            .documentation
            .unwrap()
            .update(cx, |documentation, _| documentation.source().to_owned()),
        "This is a test documentation",
    )
}

#[gpui::test]
fn test_create_signature_help_markdown_string_3(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![
            lsp::SignatureInformation {
                label: "fn test1(foo: u8, bar: &str)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
            lsp::SignatureInformation {
                label: "fn test2(hoge: String, fuga: bool)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("hoge: String".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("fuga: bool".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
        ],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test1(foo: u8, bar: &str)"),
            vec![(9..16, current_parameter())]
        )
    );
}

#[gpui::test]
fn test_create_signature_help_markdown_string_4(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![
            lsp::SignatureInformation {
                label: "fn test1(foo: u8, bar: &str)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
            lsp::SignatureInformation {
                label: "fn test2(hoge: String, fuga: bool)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("hoge: String".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("fuga: bool".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
        ],
        active_signature: Some(1),
        active_parameter: Some(0),
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test2(hoge: String, fuga: bool)"),
            vec![(9..21, current_parameter())]
        )
    );
}

#[gpui::test]
fn test_create_signature_help_markdown_string_5(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![
            lsp::SignatureInformation {
                label: "fn test1(foo: u8, bar: &str)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
            lsp::SignatureInformation {
                label: "fn test2(hoge: String, fuga: bool)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("hoge: String".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("fuga: bool".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
        ],
        active_signature: Some(1),
        active_parameter: Some(1),
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test2(hoge: String, fuga: bool)"),
            vec![(23..33, current_parameter())]
        )
    );
}

#[gpui::test]
fn test_create_signature_help_markdown_string_6(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![
            lsp::SignatureInformation {
                label: "fn test1(foo: u8, bar: &str)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
            lsp::SignatureInformation {
                label: "fn test2(hoge: String, fuga: bool)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("hoge: String".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("fuga: bool".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
        ],
        active_signature: Some(1),
        active_parameter: None,
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test2(hoge: String, fuga: bool)"),
            vec![(9..21, current_parameter())]
        )
    );
}

#[gpui::test]
fn test_create_signature_help_markdown_string_7(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![
            lsp::SignatureInformation {
                label: "fn test1(foo: u8, bar: &str)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
            lsp::SignatureInformation {
                label: "fn test2(hoge: String, fuga: bool)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("hoge: String".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("fuga: bool".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
            lsp::SignatureInformation {
                label: "fn test3(one: usize, two: u32)".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("one: usize".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("two: u32".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            },
        ],
        active_signature: Some(2),
        active_parameter: Some(1),
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test3(one: usize, two: u32)"),
            vec![(21..29, current_parameter())]
        )
    );
}

#[gpui::test]
fn test_create_signature_help_markdown_string_8(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![],
        active_signature: None,
        active_parameter: None,
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_none());
}

#[gpui::test]
fn test_create_signature_help_markdown_string_9(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn test(foo: u8, bar: &str)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::LabelOffsets([8, 15]),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::LabelOffsets([17, 26]),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    let maybe_markdown = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_markdown.is_some());

    let markdown = maybe_markdown.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test(foo: u8, bar: &str)"),
            vec![(8..15, current_parameter())]
        )
    );
}

#[gpui::test]
fn test_parameter_documentation(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn test(foo: u8, bar: &str)".to_string(),
            documentation: Some(Documentation::String(
                "This is a test documentation".to_string(),
            )),
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                    documentation: Some(Documentation::String("The foo parameter".to_string())),
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("bar: &str".to_string()),
                    documentation: Some(Documentation::String("The bar parameter".to_string())),
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    let maybe_signature_help = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(maybe_signature_help.is_some());

    let signature_help = maybe_signature_help.unwrap();
    let signature = &signature_help.signatures[signature_help.active_signature];

    // Check that parameter documentation is extracted
    assert_eq!(signature.parameters.len(), 2);
    assert_eq!(
        signature.parameters[0]
            .documentation
            .as_ref()
            .unwrap()
            .update(cx, |documentation, _| documentation.source().to_owned()),
        "The foo parameter",
    );
    assert_eq!(
        signature.parameters[1]
            .documentation
            .as_ref()
            .unwrap()
            .update(cx, |documentation, _| documentation.source().to_owned()),
        "The bar parameter",
    );

    // Check that the active parameter is correct
    assert_eq!(signature.active_parameter, Some(0));
}

#[gpui::test]
fn test_create_signature_help_implements_utf16_spec(cx: &mut TestAppContext) {
    let signature_help = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn test(🦀: u8, 🦀: &str)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::LabelOffsets([8, 10]),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::LabelOffsets([16, 18]),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    let signature_help = cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
    assert!(signature_help.is_some());

    let markdown = signature_help.unwrap();
    let signature = markdown.signatures[markdown.active_signature].clone();
    let markdown = (signature.label, signature.highlights);
    assert_eq!(
        markdown,
        (
            SharedString::new_static("fn test(🦀: u8, 🦀: &str)"),
            vec![(8..12, current_parameter())]
        )
    );
}
