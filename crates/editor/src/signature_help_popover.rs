use crate::Editor;
use gpui::{
    div, AnyElement, FontStyle, FontWeight, HighlightStyle, Hsla, InteractiveElement, IntoElement,
    MouseButton, ParentElement, Pixels, Size, StatefulInteractiveElement, Styled, UnderlineStyle,
    ViewContext,
};
use lsp::SignatureHelp;
use rich_text::{Highlight, RichText};
use std::sync::Arc;
use ui::StyledExt;

pub const SIGNATURE_HELP_HIGHLIGHT_STYLE: HighlightStyle = HighlightStyle {
    color: None,
    font_weight: Some(FontWeight::EXTRA_BOLD),
    font_style: Some(FontStyle::Normal),
    background_color: None,
    underline: Some(UnderlineStyle {
        thickness: Pixels(1.),
        color: None,
        wavy: false,
    }),
    strikethrough: None,
    fade_out: None,
};

pub const SIGNATURE_HELP_OVERLOAD_HIGHLIGHT_STYLE: HighlightStyle = HighlightStyle {
    color: None,
    font_weight: Some(FontWeight::NORMAL),
    font_style: Some(FontStyle::Italic),
    background_color: None,
    underline: None,
    strikethrough: None,
    fade_out: None,
};

#[derive(Clone)]
pub struct SignatureHelpPopover {
    pub text: RichText,
}

pub fn create_signature_help_popover(
    SignatureHelp {
        signatures: signature_information,
        active_signature: maybe_active_signature,
        active_parameter: maybe_active_parameter,
        ..
    }: SignatureHelp,
    background_color: Hsla,
) -> Option<SignatureHelpPopover> {
    let function_options_count = signature_information.len();

    let signature_information = maybe_active_signature
        .and_then(|active_signature| signature_information.get(active_signature as usize))
        .or_else(|| signature_information.first())?;

    let str_for_join = ",  ";
    let parameter_length = signature_information
        .parameters
        .as_ref()
        .map(|parameters| parameters.len())
        .unwrap_or(0);
    let mut highlight_start = 0;
    let (text, mut highlights): (Vec<_>, Vec<_>) = signature_information
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
                    let mut highlight = SIGNATURE_HELP_HIGHLIGHT_STYLE;
                    highlight.background_color = Some(background_color);
                    Some((
                        string,
                        Some((
                            highlight_start..(highlight_start + string_length),
                            Highlight::Highlight(highlight),
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
    let text = text.join(str_for_join);
    let text = if function_options_count >= 2 {
        let suffix = format!("(+{} overload)", function_options_count - 1);
        let highlight_start = text.len() + 1;
        let mut highlight_style = SIGNATURE_HELP_OVERLOAD_HIGHLIGHT_STYLE;
        highlight_style.background_color = Some(background_color);
        highlights.push(Some((
            highlight_start..(highlight_start + suffix.len()),
            Highlight::Highlight(highlight_style),
        )));
        format!("{text} {suffix}")
    } else {
        text
    };

    if text.is_empty() {
        None
    } else {
        let highlights = highlights.into_iter().flatten().collect::<Vec<_>>();
        let text = RichText {
            text: text.into(),
            highlights,
            link_ranges: Vec::new(),
            link_urls: Arc::new([]),
            custom_ranges: Vec::new(),
            custom_ranges_tooltip_fn: None,
        };
        Some(SignatureHelpPopover { text })
    }
}

impl SignatureHelpPopover {
    pub fn render(&mut self, max_size: Size<Pixels>, cx: &mut ViewContext<Editor>) -> AnyElement {
        div()
            .id("signature_help_popover")
            .elevation_2(cx)
            .overflow_y_scroll()
            .max_w(max_size.width)
            .max_h(max_size.height)
            .on_mouse_move(|_, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
            .child(
                div().p_2().child(
                    self.text
                        .element("signature_help_popover_rich_text".into(), cx),
                ),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use crate::signature_help_popover::{
        create_signature_help_popover, SIGNATURE_HELP_HIGHLIGHT_STYLE,
        SIGNATURE_HELP_OVERLOAD_HIGHLIGHT_STYLE,
    };
    use gpui::{HighlightStyle, Hsla};
    use lsp::{SignatureHelp, SignatureInformation};
    use rich_text::Highlight;

    const SIGNATURE_HELP_HIGHLIGHT: Highlight = Highlight::Highlight(HighlightStyle {
        background_color: Some(BACKGROUND_COLOR),
        ..SIGNATURE_HELP_HIGHLIGHT_STYLE
    });
    const SIGNATURE_HELP_OVERLOAD_HIGHLIGHT: Highlight = Highlight::Highlight(HighlightStyle {
        background_color: Some(BACKGROUND_COLOR),
        ..SIGNATURE_HELP_OVERLOAD_HIGHLIGHT_STYLE
    });
    const BACKGROUND_COLOR: Hsla = Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    };

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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let signature_help_popover = maybe_markdown.unwrap();
        assert_eq!(
            (
                signature_help_popover.text.text,
                signature_help_popover.text.highlights
            ),
            (
                "foo: u8,  bar: &str".to_string().into(),
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            (markdown.text.text, markdown.text.highlights),
            (
                "foo: u8,  bar: &str".to_string().into(),
                vec![(10..19, SIGNATURE_HELP_HIGHLIGHT)]
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            (markdown.text.text, markdown.text.highlights),
            (
                "foo: u8,  bar: &str (+1 overload)".to_string().into(),
                vec![
                    (0..7, SIGNATURE_HELP_HIGHLIGHT),
                    (20..33, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            (markdown.text.text, markdown.text.highlights),
            (
                "hoge: String,  fuga: bool (+1 overload)".to_string().into(),
                vec![
                    (0..12, SIGNATURE_HELP_HIGHLIGHT),
                    (26..39, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            (markdown.text.text, markdown.text.highlights),
            (
                "hoge: String,  fuga: bool (+1 overload)".to_string().into(),
                vec![
                    (15..25, SIGNATURE_HELP_HIGHLIGHT),
                    (26..39, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            (markdown.text.text, markdown.text.highlights),
            (
                "hoge: String,  fuga: bool (+1 overload)".to_string().into(),
                vec![(26..39, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)]
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            (markdown.text.text, markdown.text.highlights),
            (
                "one: usize,  two: u32 (+2 overload)".to_string().into(),
                vec![
                    (13..21, SIGNATURE_HELP_HIGHLIGHT),
                    (22..35, SIGNATURE_HELP_OVERLOAD_HIGHLIGHT)
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
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
        let maybe_markdown = create_signature_help_popover(signature_help, BACKGROUND_COLOR);
        assert!(maybe_markdown.is_some());

        let markdown = maybe_markdown.unwrap();
        assert_eq!(
            (markdown.text.text, markdown.text.highlights),
            (
                "foo: u8,  bar: &str".to_string().into(),
                vec![(0..7, SIGNATURE_HELP_HIGHLIGHT)]
            )
        );
    }
}
