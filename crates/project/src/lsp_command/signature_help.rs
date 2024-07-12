use std::{ops::Range, sync::Arc};

use gpui::FontWeight;
use language::{
    markdown::{MarkdownHighlight, MarkdownHighlightStyle},
    Language,
};

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
}

impl SignatureHelp {
    pub fn new(
        lsp::SignatureHelp {
            signatures,
            active_signature,
            active_parameter,
            ..
        }: lsp::SignatureHelp,
        language: Option<Arc<Language>>,
    ) -> Option<Self> {
        let function_options_count = signatures.len();

        let signature_information = active_signature
            .and_then(|active_signature| signatures.get(active_signature as usize))
            .or_else(|| signatures.first())?;

        let str_for_join = ", ";
        let parameter_length = signature_information
            .parameters
            .as_ref()
            .map(|parameters| parameters.len())
            .unwrap_or(0);
        let mut highlight_start = 0;
        let (markdown, mut highlights): (Vec<_>, Vec<_>) = signature_information
            .parameters
            .as_ref()?
            .iter()
            .enumerate()
            .filter_map(|(i, parameter_information)| {
                let string = match parameter_information.label.clone() {
                    lsp::ParameterLabel::Simple(string) => string,
                    lsp::ParameterLabel::LabelOffsets(offset) => signature_information
                        .label
                        .chars()
                        .skip(offset[0] as usize)
                        .take((offset[1] - offset[0]) as usize)
                        .collect::<String>(),
                };
                let string_length = string.len();

                let result = if let Some(active_parameter) = active_parameter {
                    if i == active_parameter as usize {
                        Some((
                            string,
                            Some((
                                highlight_start..(highlight_start + string_length),
                                SIGNATURE_HELP_HIGHLIGHT_CURRENT,
                            )),
                        ))
                    } else {
                        Some((string, None))
                    }
                } else {
                    Some((string, None))
                };

                if i != parameter_length {
                    highlight_start += string_length + str_for_join.len();
                }

                result
            })
            .unzip();

        let result = if markdown.is_empty() {
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

            Some((markdown, highlights.into_iter().flatten().collect()))
        };

        result.map(|(markdown, highlights)| Self {
            markdown,
            highlights,
        })
    }
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
