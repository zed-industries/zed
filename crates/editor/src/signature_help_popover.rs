use crate::{Editor, EditorStyle};
use gpui::{
    div, AnyElement, FontWeight, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Pixels, Size, StatefulInteractiveElement, Styled, ViewContext, WeakView,
};
use language::markdown::{MarkdownHighlight, MarkdownHighlightStyle};
use language::ParsedMarkdown;
use lsp::SignatureHelp;
use std::ops::Range;
use ui::StyledExt;
use workspace::Workspace;

const SIGNATURE_HELP_HIGHLIGHT: MarkdownHighlight =
    MarkdownHighlight::Style(MarkdownHighlightStyle {
        italic: false,
        underline: true,
        strikethrough: false,
        weight: FontWeight::EXTRA_BOLD,
    });

const SIGNATURE_HELP_OVERLOAD_HIGHLIGHT: MarkdownHighlight =
    MarkdownHighlight::Style(MarkdownHighlightStyle {
        italic: true,
        underline: false,
        strikethrough: false,
        weight: FontWeight::NORMAL,
    });

#[derive(Clone, Debug)]
pub struct SignatureHelpPopover {
    pub parsed_content: ParsedMarkdown,
}

/// create_signature_help_markdown_string generates the markdown text that is displayed in the `SignatureHelp` window.
pub fn create_signature_help_markdown_string(
    signature_help: SignatureHelp,
) -> Option<(String, Vec<(Range<usize>, MarkdownHighlight)>)> {
    let (signature_information, maybe_active_signature, maybe_active_parameter) = (
        signature_help.signatures,
        signature_help.active_signature,
        signature_help.active_parameter,
    );

    let function_options_count = signature_information.len();

    let signature_information = maybe_active_signature
        .and_then(|active_signature| signature_information.get(active_signature as usize))?;

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

            let result = if let Some(active_parameter) = maybe_active_parameter {
                if i == active_parameter as usize {
                    Some((
                        string,
                        Some((
                            highlight_start..(highlight_start + string_length),
                            SIGNATURE_HELP_HIGHLIGHT,
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
    let markdown = markdown.join(str_for_join);
    let markdown = if function_options_count >= 2 {
        let suffix = format!("(+{} overload)", function_options_count - 1);
        let highlight_start = markdown.len() + 1;
        highlights.push(Some((
            highlight_start..(highlight_start + suffix.len()),
            SIGNATURE_HELP_OVERLOAD_HIGHLIGHT,
        )));
        let markdown = format!("{} {}", markdown, suffix);
        markdown
    } else {
        markdown
    };

    let highlights = highlights.into_iter().flatten().collect::<Vec<_>>();

    if markdown.is_empty() {
        None
    } else {
        Some((markdown, highlights))
    }
}

impl SignatureHelpPopover {
    pub fn render(
        &mut self,
        style: &EditorStyle,
        max_size: Size<Pixels>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Editor>,
    ) -> AnyElement {
        div()
            .id("signature_help_popover")
            .elevation_2(cx)
            .overflow_y_scroll()
            .max_w(max_size.width)
            .max_h(max_size.height)
            .on_mouse_move(|_, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
            .child(div().p_2().child(crate::render_parsed_markdown(
                "content",
                &self.parsed_content,
                style,
                workspace,
                cx,
            )))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use crate::signature_help_popover::{
        create_signature_help_markdown_string, SIGNATURE_HELP_HIGHLIGHT,
        SIGNATURE_HELP_OVERLOAD_HIGHLIGHT,
    };
    use lsp::{SignatureHelp, SignatureInformation};

    #[test]
    fn test_create_signature_help_markdown_string_1() {
        let signature_help = SignatureHelp {
            signatures: vec![SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "foo: u8, bar: &str".to_string(),
                vec![(0..7, SIGNATURE_HELP_HIGHLIGHT)]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_2() {
        let signature_help = SignatureHelp {
            signatures: vec![SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "foo: u8, bar: &str".to_string(),
                vec![(9..18, SIGNATURE_HELP_HIGHLIGHT)]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_3() {
        let signature_help = SignatureHelp {
            signatures: vec![
                SignatureInformation {
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
                SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "foo: u8, bar: &str (+1 overload)".to_string(),
                vec![
                    (0..7, SIGNATURE_HELP_HIGHLIGHT),
                    (19..32, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_4() {
        let signature_help = SignatureHelp {
            signatures: vec![
                SignatureInformation {
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
                SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "hoge: String, fuga: bool (+1 overload)".to_string(),
                vec![
                    (0..12, SIGNATURE_HELP_HIGHLIGHT),
                    (25..38, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_5() {
        let signature_help = SignatureHelp {
            signatures: vec![
                SignatureInformation {
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
                SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "hoge: String, fuga: bool (+1 overload)".to_string(),
                vec![
                    (14..24, SIGNATURE_HELP_HIGHLIGHT),
                    (25..38, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_6() {
        let signature_help = SignatureHelp {
            signatures: vec![
                SignatureInformation {
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
                SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "hoge: String, fuga: bool (+1 overload)".to_string(),
                vec![(25..38, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_7() {
        let signature_help = SignatureHelp {
            signatures: vec![
                SignatureInformation {
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
                SignatureInformation {
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
                SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "one: usize, two: u32 (+2 overload)".to_string(),
                vec![
                    (12..20, SIGNATURE_HELP_HIGHLIGHT),
                    (21..34, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
                ]
            )
        );
    }

    #[test]
    fn test_create_signature_help_markdown_string_8() {
        let signature_help = SignatureHelp {
            signatures: vec![],
            active_signature: None,
            active_parameter: None,
        };
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_none());
    }

    #[test]
    fn test_create_signature_help_markdown_string_9() {
        let signature_help = SignatureHelp {
            signatures: vec![SignatureInformation {
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
        let maybe_markdown = create_signature_help_markdown_string(signature_help);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            markdown,
            (
                "foo: u8, bar: &str".to_string(),
                vec![(0..7, SIGNATURE_HELP_HIGHLIGHT)]
            )
        );
    }
}
