use crate::{Editor, EditorStyle};
use gpui::{
    div, AnyElement, InteractiveElement, IntoElement, MouseButton, ParentElement, Pixels, Size,
    StatefulInteractiveElement, Styled, ViewContext, WeakView,
};
use itertools::Itertools;
use language::ParsedMarkdown;
use lsp::SignatureHelp;
use ui::StyledExt;
use workspace::Workspace;

#[derive(Clone, Debug)]
pub struct SignatureHelpPopover {
    pub parsed_content: ParsedMarkdown,
}

/// create_signature_help_markdown_string generates the markdown text that is displayed in the `SignatureHelp` window.
pub fn create_signature_help_markdown_string(signature_help: SignatureHelp) -> Option<String> {
    let (signature_information, maybe_active_signature, maybe_active_parameter) = (
        signature_help.signatures,
        signature_help.active_signature,
        signature_help.active_parameter,
    );

    let function_options_count = signature_information.len();

    let signature_information = maybe_active_signature
        .and_then(|active_signature| signature_information.get(active_signature as usize))?;

    let parameter_information = signature_information
        .parameters
        .as_ref()?
        .iter()
        .enumerate()
        .filter_map(
            |(i, parameter_information)| match parameter_information.label.clone() {
                lsp::ParameterLabel::Simple(string) => {
                    if let Some(active_parameter) = maybe_active_parameter {
                        if i == active_parameter as usize {
                            Some(format!("**{}**", string))
                        } else {
                            Some(string)
                        }
                    } else {
                        Some(string)
                    }
                }
                _ => None,
            },
        )
        .join(", ");

    let parameter_information = if parameter_information.is_empty() {
        None
    } else {
        Some(parameter_information)
    }?;

    let markdown = if function_options_count >= 2 {
        format!(
            "{} (+{} overload)",
            parameter_information,
            function_options_count - 1
        )
    } else {
        parameter_information
    };
    Some(markdown)
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
    use crate::signature_help_popover::create_signature_help_markdown_string;
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
        assert_eq!(markdown, "**foo: u8**, bar: &str");
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
        assert_eq!(markdown, "foo: u8, **bar: &str**");
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
        assert_eq!(markdown, "**foo: u8**, bar: &str (+1 overload)");
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
        assert_eq!(markdown, "**hoge: String**, fuga: bool (+1 overload)");
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
        assert_eq!(markdown, "hoge: String, **fuga: bool** (+1 overload)");
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
        assert_eq!(markdown, "hoge: String, fuga: bool (+1 overload)");
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
        assert_eq!(markdown, "one: usize, **two: u32** (+2 overload)");
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
}
