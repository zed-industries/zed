use std::{ops::Range, sync::Arc};

use gpui::FontWeight;
use language::{
    markdown::{MarkdownHighlight, MarkdownHighlightStyle},
    Language,
};
use rpc::proto::{self, documentation};

pub const SIGNATURE_HELP_HIGHLIGHT_CURRENT: MarkdownHighlight =
    MarkdownHighlight::Style(MarkdownHighlightStyle {
        italic: false,
        underline: false,
        strikethrough: false,
        weight: FontWeight::EXTRA_BOLD,
    });

pub const SIGNATURE_HELP_HIGHLIGHT_OVERLOAD: MarkdownHighlight =
    MarkdownHighlight::Style(MarkdownHighlightStyle {
        italic: true,
        underline: false,
        strikethrough: false,
        weight: FontWeight::NORMAL,
    });

#[derive(Debug)]
pub struct SignatureHelp {
    pub markdown: String,
    pub highlights: Vec<(Range<usize>, MarkdownHighlight)>,
    pub(super) original_data: lsp::SignatureHelp,
}

impl SignatureHelp {
    pub fn new(help: lsp::SignatureHelp, language: Option<Arc<Language>>) -> Option<Self> {
        let function_options_count = help.signatures.len();

        let signature_information = help
            .active_signature
            .and_then(|active_signature| help.signatures.get(active_signature as usize))
            .or_else(|| help.signatures.first())?;

        let str_for_join = ", ";
        let parameter_length = signature_information
            .parameters
            .as_ref()
            .map_or(0, |parameters| parameters.len());
        let mut highlight_start = 0;
        let (markdown, mut highlights): (Vec<_>, Vec<_>) = signature_information
            .parameters
            .as_ref()?
            .iter()
            .enumerate()
            .map(|(i, parameter_information)| {
                let label = match parameter_information.label.clone() {
                    lsp::ParameterLabel::Simple(string) => string,
                    lsp::ParameterLabel::LabelOffsets(offset) => signature_information
                        .label
                        .chars()
                        .skip(offset[0] as usize)
                        .take((offset[1] - offset[0]) as usize)
                        .collect::<String>(),
                };
                let label_length = label.len();

                let highlights = help.active_parameter.and_then(|active_parameter| {
                    if i == active_parameter as usize {
                        Some((
                            highlight_start..(highlight_start + label_length),
                            SIGNATURE_HELP_HIGHLIGHT_CURRENT,
                        ))
                    } else {
                        None
                    }
                });

                if i != parameter_length {
                    highlight_start += label_length + str_for_join.len();
                }

                (label, highlights)
            })
            .unzip();

        if markdown.is_empty() {
            None
        } else {
            let markdown = markdown.join(str_for_join);
            let language_name = language
                .map(|n| n.name().to_lowercase())
                .unwrap_or_default();

            let markdown = if function_options_count >= 2 {
                let suffix = format!("(+{} overload)", function_options_count - 1);
                let highlight_start = markdown.len() + 1;
                highlights.push(Some((
                    highlight_start..(highlight_start + suffix.len()),
                    SIGNATURE_HELP_HIGHLIGHT_OVERLOAD,
                )));
                format!("```{language_name}\n{markdown} {suffix}")
            } else {
                format!("```{language_name}\n{markdown}")
            };

            Some(Self {
                markdown,
                highlights: highlights.into_iter().flatten().collect(),
                original_data: help,
            })
        }
    }
}

pub fn lsp_to_proto_signature(lsp_help: lsp::SignatureHelp) -> proto::SignatureHelp {
    proto::SignatureHelp {
        signatures: lsp_help
            .signatures
            .into_iter()
            .map(|signature| proto::SignatureInformation {
                label: signature.label,
                documentation: signature
                    .documentation
                    .map(|documentation| lsp_to_proto_documentation(documentation)),
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
    let documentation = {
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
    };
    documentation
}

#[cfg(test)]
mod tests {
    use crate::lsp_command::signature_help::{
        SignatureHelp, SIGNATURE_HELP_HIGHLIGHT_CURRENT, SIGNATURE_HELP_HIGHLIGHT_OVERLOAD,
    };

    #[test]
    fn test_create_signature_help_markdown_string_1() {
        let signature_help = lsp::SignatureHelp {
            signatures: vec![lsp::SignatureInformation {
                label: "fn test(foo: u8, bar: &str)".to_string(),
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
            }],
            active_signature: Some(0),
            active_parameter: Some(0),
        };
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\nfoo: u8, bar: &str".to_string(),
                vec![(0..7, SIGNATURE_HELP_HIGHLIGHT_CURRENT)]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_2() {
        let signature_help = lsp::SignatureHelp {
            signatures: vec![lsp::SignatureInformation {
                label: "fn test(foo: u8, bar: &str)".to_string(),
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
            }],
            active_signature: Some(0),
            active_parameter: Some(1),
        };
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\nfoo: u8, bar: &str".to_string(),
                vec![(9..18, SIGNATURE_HELP_HIGHLIGHT_CURRENT)]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_3() {
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
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\nfoo: u8, bar: &str (+1 overload)".to_string(),
                vec![
                    (0..7, SIGNATURE_HELP_HIGHLIGHT_CURRENT),
                    (19..32, SIGNATURE_HELP_HIGHLIGHT_OVERLOAD)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_4() {
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
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\nhoge: String, fuga: bool (+1 overload)".to_string(),
                vec![
                    (0..12, SIGNATURE_HELP_HIGHLIGHT_CURRENT),
                    (25..38, SIGNATURE_HELP_HIGHLIGHT_OVERLOAD)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_5() {
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
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\nhoge: String, fuga: bool (+1 overload)".to_string(),
                vec![
                    (14..24, SIGNATURE_HELP_HIGHLIGHT_CURRENT),
                    (25..38, SIGNATURE_HELP_HIGHLIGHT_OVERLOAD)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_6() {
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
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\nhoge: String, fuga: bool (+1 overload)".to_string(),
                vec![(25..38, SIGNATURE_HELP_HIGHLIGHT_OVERLOAD)]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_7() {
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
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\none: usize, two: u32 (+2 overload)".to_string(),
                vec![
                    (12..20, SIGNATURE_HELP_HIGHLIGHT_CURRENT),
                    (21..34, SIGNATURE_HELP_HIGHLIGHT_OVERLOAD)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_8() {
        let signature_help = lsp::SignatureHelp {
            signatures: vec![],
            active_signature: None,
            active_parameter: None,
        };
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_none());
    }

    #[test]
    fn test_create_signature_help_markdown_string_9() {
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
        let maybe_markdown = SignatureHelp::new(signature_help, None);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.markdown, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "```\nfoo: u8, bar: &str".to_string(),
                vec![(0..7, SIGNATURE_HELP_HIGHLIGHT_CURRENT)]
            )
        );
    }
}
