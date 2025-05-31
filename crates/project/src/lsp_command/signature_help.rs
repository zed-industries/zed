use std::ops::Range;

use gpui::{FontStyle, FontWeight, HighlightStyle};
use regex::Regex;
use rpc::proto::{self, documentation};

#[derive(Debug)]
pub struct SignatureHelp {
    pub label: String,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
    pub(super) original_data: lsp::SignatureHelp,
}

impl SignatureHelp {
    pub fn new(help: lsp::SignatureHelp) -> Option<Self> {
        let function_options_count = help.signatures.len();

        let signature_information = help
            .active_signature
            .and_then(|active_signature| help.signatures.get(active_signature as usize))
            .or_else(|| help.signatures.first())?;

        let mut highlights = Vec::with_capacity(2);

        if let Some(active_parameter) = help.active_parameter {
            let active_label = &signature_information
                .parameters
                .as_ref()?
                .get(active_parameter as usize)?
                .label;

            let range = match &active_label {
                lsp::ParameterLabel::Simple(label) => {
                    Regex::new(&format!("(\\W|^)(?<label>{})(\\W|$)", regex::escape(label)))
                        .ok()
                        .and_then(|re| re.captures(&signature_information.label))
                        .and_then(|captures| captures.get(2).map(|m| m.start()..m.end()))
                        .unwrap_or(0..0)
                }
                lsp::ParameterLabel::LabelOffsets([start, end]) => {
                    (*start as usize)..(*end as usize)
                }
            };

            highlights.push((range, FontWeight::EXTRA_BOLD.into()));
        }

        let mut label = signature_information.label.clone();
        if function_options_count >= 2 {
            let suffix = format!("(+{} overload)", function_options_count - 1);
            let highlight_start = label.len() + 1;
            let highlight_range = highlight_start..(highlight_start + suffix.len());
            highlights.push((highlight_range, FontStyle::Italic.into()));
            label.push(' ');
            label.push_str(&suffix);
        };

        Some(Self {
            label,
            highlights,
            original_data: help,
        })
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
    use gpui::{FontStyle, FontWeight, HighlightStyle};

    use crate::lsp_command::signature_help::SignatureHelp;

    fn current_parameter() -> HighlightStyle {
        FontWeight::EXTRA_BOLD.into()
    }

    fn overload() -> HighlightStyle {
        FontStyle::Italic.into()
    }

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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test(foo: u8, bar: &str)".to_string(),
                vec![(8..15, current_parameter())]
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test(foo: u8, bar: &str)".to_string(),
                vec![(17..26, current_parameter())]
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test1(foo: u8, bar: &str) (+1 overload)".to_string(),
                vec![(9..16, current_parameter()), (29..42, overload())]
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test2(hoge: String, fuga: bool) (+1 overload)".to_string(),
                vec![(9..21, current_parameter()), (35..48, overload())]
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test2(hoge: String, fuga: bool) (+1 overload)".to_string(),
                vec![(23..33, current_parameter()), (35..48, overload()),]
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test2(hoge: String, fuga: bool) (+1 overload)".to_string(),
                vec![(35..48, overload())]
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test3(one: usize, two: u32) (+2 overload)".to_string(),
                vec![(21..29, current_parameter()), (31..44, overload())]
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
        let maybe_markdown = SignatureHelp::new(signature_help);
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let markdown = (markdown.label, markdown.highlights);
        assert_eq!(
            markdown,
            (
                "fn test(foo: u8, bar: &str)".to_string(),
                vec![(8..15, current_parameter())]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_10() {
        let signature_help = lsp::SignatureHelp {
            signatures: vec![lsp::SignatureInformation {
                label: "foo: u8, bar: &str".to_string(),
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
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let (_, items) = (markdown.label, markdown.highlights);
        assert_eq!(items, vec![(9..18, current_parameter())]);
    }

    #[test]
    fn test_create_signature_help_markdown_string_11() {
        let signature_help = lsp::SignatureHelp {
            signatures: vec![lsp::SignatureInformation {
                label: "(func $func (param $1 i32) (param $2 f32) (result i32))".to_string(),
                documentation: None,
                parameters: Some(vec![
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("(param $1 i32)".to_string()),
                        documentation: None,
                    },
                    lsp::ParameterInformation {
                        label: lsp::ParameterLabel::Simple("(param $2 f32)".to_string()),
                        documentation: None,
                    },
                ]),
                active_parameter: None,
            }],
            active_signature: Some(0),
            active_parameter: Some(1),
        };
        let maybe_markdown = SignatureHelp::new(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        let (_, items) = (markdown.label, markdown.highlights);
        assert_eq!(items, vec![(27..41, current_parameter())]);
    }
}
