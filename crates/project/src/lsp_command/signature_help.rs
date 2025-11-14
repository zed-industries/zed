use std::{ops::Range, sync::Arc};

use gpui::{App, AppContext, Entity, FontWeight, HighlightStyle, SharedString};
use language::LanguageRegistry;
use lsp::LanguageServerId;
use markdown::Markdown;
use rpc::proto::{self, documentation};
use util::maybe;

#[derive(Debug)]
pub struct SignatureHelp {
    pub active_signature: usize,
    pub signatures: Vec<SignatureHelpData>,
    pub(super) original_data: lsp::SignatureHelp,
}

#[derive(Debug, Clone)]
pub struct SignatureHelpData {
    pub label: SharedString,
    pub documentation: Option<Entity<Markdown>>,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
    pub active_parameter: Option<usize>,
    pub parameters: Vec<ParameterInfo>,
}

#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub label_range: Option<Range<usize>>,
    pub documentation: Option<Entity<Markdown>>,
}

impl SignatureHelp {
    pub fn new(
        help: lsp::SignatureHelp,
        language_registry: Option<Arc<LanguageRegistry>>,
        lang_server_id: Option<LanguageServerId>,
        cx: &mut App,
    ) -> Option<Self> {
        if help.signatures.is_empty() {
            return None;
        }
        let active_signature = help.active_signature.unwrap_or(0) as usize;
        let mut signatures = Vec::<SignatureHelpData>::with_capacity(help.signatures.capacity());
        for signature in &help.signatures {
            let label = SharedString::from(signature.label.clone());
            let active_parameter = signature
                .active_parameter
                .unwrap_or_else(|| help.active_parameter.unwrap_or(0))
                as usize;
            let mut highlights = Vec::new();
            let mut parameter_infos = Vec::new();

            if let Some(parameters) = &signature.parameters {
                for (index, parameter) in parameters.iter().enumerate() {
                    let label_range = match &parameter.label {
                        &lsp::ParameterLabel::LabelOffsets([offset1, offset2]) => {
                            maybe!({
                                let offset1 = offset1 as usize;
                                let offset2 = offset2 as usize;
                                if offset1 < offset2 {
                                    let mut indices = label.char_indices().scan(
                                        0,
                                        |utf16_offset_acc, (offset, c)| {
                                            let utf16_offset = *utf16_offset_acc;
                                            *utf16_offset_acc += c.len_utf16();
                                            Some((utf16_offset, offset))
                                        },
                                    );
                                    let (_, offset1) = indices
                                        .find(|(utf16_offset, _)| *utf16_offset == offset1)?;
                                    let (_, offset2) = indices
                                        .find(|(utf16_offset, _)| *utf16_offset == offset2)?;
                                    Some(offset1..offset2)
                                } else {
                                    log::warn!(
                                        "language server {lang_server_id:?} produced invalid parameter label range: {offset1:?}..{offset2:?}",
                                    );
                                    None
                                }
                            })
                        }
                        lsp::ParameterLabel::Simple(parameter_label) => {
                            if let Some(start) = signature.label.find(parameter_label) {
                                Some(start..start + parameter_label.len())
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(label_range) = &label_range
                        && index == active_parameter
                    {
                        highlights.push((
                            label_range.clone(),
                            HighlightStyle {
                                font_weight: Some(FontWeight::EXTRA_BOLD),
                                ..HighlightStyle::default()
                            },
                        ));
                    }

                    let documentation = parameter
                        .documentation
                        .as_ref()
                        .map(|doc| documentation_to_markdown(doc, language_registry.clone(), cx));

                    parameter_infos.push(ParameterInfo {
                        label_range,
                        documentation,
                    });
                }
            }

            let documentation = signature
                .documentation
                .as_ref()
                .map(|doc| documentation_to_markdown(doc, language_registry.clone(), cx));

            signatures.push(SignatureHelpData {
                label,
                documentation,
                highlights,
                active_parameter: Some(active_parameter),
                parameters: parameter_infos,
            });
        }
        Some(Self {
            signatures,
            active_signature,
            original_data: help,
        })
    }
}

fn documentation_to_markdown(
    documentation: &lsp::Documentation,
    language_registry: Option<Arc<LanguageRegistry>>,
    cx: &mut App,
) -> Entity<Markdown> {
    match documentation {
        lsp::Documentation::String(string) => {
            cx.new(|cx| Markdown::new_text(SharedString::from(string), cx))
        }
        lsp::Documentation::MarkupContent(markup) => match markup.kind {
            lsp::MarkupKind::PlainText => {
                cx.new(|cx| Markdown::new_text(SharedString::from(&markup.value), cx))
            }
            lsp::MarkupKind::Markdown => cx.new(|cx| {
                Markdown::new(
                    SharedString::from(&markup.value),
                    language_registry,
                    None,
                    cx,
                )
            }),
        },
    }
}

pub fn lsp_to_proto_signature(lsp_help: lsp::SignatureHelp) -> proto::SignatureHelp {
    proto::SignatureHelp {
        signatures: lsp_help
            .signatures
            .into_iter()
            .map(|signature| proto::SignatureInformation {
                label: signature.label,
                documentation: signature.documentation.map(lsp_to_proto_documentation),
                parameters: signature
                    .parameters
                    .unwrap_or_default()
                    .into_iter()
                    .map(|parameter_info| proto::ParameterInformation {
                        label: Some(match parameter_info.label {
                            lsp::ParameterLabel::Simple(label) => {
                                proto::parameter_information::Label::Simple(label)
                            }
                            lsp::ParameterLabel::LabelOffsets(offsets) => {
                                proto::parameter_information::Label::LabelOffsets(
                                    proto::LabelOffsets {
                                        start: offsets[0],
                                        end: offsets[1],
                                    },
                                )
                            }
                        }),
                        documentation: parameter_info.documentation.map(lsp_to_proto_documentation),
                    })
                    .collect(),
                active_parameter: signature.active_parameter,
            })
            .collect(),
        active_signature: lsp_help.active_signature,
        active_parameter: lsp_help.active_parameter,
    }
}

fn lsp_to_proto_documentation(documentation: lsp::Documentation) -> proto::Documentation {
    proto::Documentation {
        content: Some(match documentation {
            lsp::Documentation::String(string) => proto::documentation::Content::Value(string),
            lsp::Documentation::MarkupContent(content) => {
                proto::documentation::Content::MarkupContent(proto::MarkupContent {
                    is_markdown: matches!(content.kind, lsp::MarkupKind::Markdown),
                    value: content.value,
                })
            }
        }),
    }
}

pub fn proto_to_lsp_signature(proto_help: proto::SignatureHelp) -> lsp::SignatureHelp {
    lsp::SignatureHelp {
        signatures: proto_help
            .signatures
            .into_iter()
            .map(|signature| lsp::SignatureInformation {
                label: signature.label,
                documentation: signature.documentation.and_then(proto_to_lsp_documentation),
                parameters: Some(
                    signature
                        .parameters
                        .into_iter()
                        .filter_map(|parameter_info| {
                            Some(lsp::ParameterInformation {
                                label: match parameter_info.label? {
                                    proto::parameter_information::Label::Simple(string) => {
                                        lsp::ParameterLabel::Simple(string)
                                    }
                                    proto::parameter_information::Label::LabelOffsets(offsets) => {
                                        lsp::ParameterLabel::LabelOffsets([
                                            offsets.start,
                                            offsets.end,
                                        ])
                                    }
                                },
                                documentation: parameter_info
                                    .documentation
                                    .and_then(proto_to_lsp_documentation),
                            })
                        })
                        .collect(),
                ),
                active_parameter: signature.active_parameter,
            })
            .collect(),
        active_signature: proto_help.active_signature,
        active_parameter: proto_help.active_parameter,
    }
}

fn proto_to_lsp_documentation(documentation: proto::Documentation) -> Option<lsp::Documentation> {
    {
        Some(match documentation.content? {
            documentation::Content::Value(string) => lsp::Documentation::String(string),
            documentation::Content::MarkupContent(markup) => {
                lsp::Documentation::MarkupContent(if markup.is_markdown {
                    lsp::MarkupContent {
                        kind: lsp::MarkupKind::Markdown,
                        value: markup.value,
                    }
                } else {
                    lsp::MarkupContent {
                        kind: lsp::MarkupKind::PlainText,
                        value: markup.value,
                    }
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use gpui::{FontWeight, HighlightStyle, SharedString, TestAppContext};
    use lsp::{Documentation, MarkupContent, MarkupKind};

    use crate::lsp_command::signature_help::SignatureHelp;

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
                SharedString::new("fn test(foo: u8, bar: &str)"),
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
                SharedString::new("fn test(foo: u8, bar: &str)"),
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
                SharedString::new("fn test1(foo: u8, bar: &str)"),
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
                SharedString::new("fn test2(hoge: String, fuga: bool)"),
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
                SharedString::new("fn test2(hoge: String, fuga: bool)"),
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
                SharedString::new("fn test2(hoge: String, fuga: bool)"),
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
                SharedString::new("fn test3(one: usize, two: u32)"),
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
                SharedString::new("fn test(foo: u8, bar: &str)"),
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
        let maybe_signature_help =
            cx.update(|cx| SignatureHelp::new(signature_help, None, None, cx));
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
                SharedString::new("fn test(🦀: u8, 🦀: &str)"),
                vec![(8..12, current_parameter())]
            )
        );
    }
}
