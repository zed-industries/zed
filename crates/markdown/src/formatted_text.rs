use std::{ops::Range, sync::Arc};

use gpui::{FontStyle, FontWeight, HighlightStyle, StyledText};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use ui::prelude::*;

// Parsing runs synchronously. Bound both the input and the text produced by
// expanding many references to the same long link destination.
const MAX_FORMATTED_BYTES: usize = 64 * 1024;

/// Markdown presented as styled text, for descriptions that must not open links,
/// resolve images or files, or embed controls. Link destinations are visible text.
#[derive(Clone, Debug, IntoElement)]
pub struct FormattedText {
    text: SharedString,
    spans: Arc<[(Range<usize>, SpanStyle)]>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SpanStyle {
    bold: bool,
    italic: bool,
    code: bool,
}

#[derive(Default)]
struct TextBuilder {
    text: String,
    spans: Vec<(Range<usize>, SpanStyle)>,
    pending_breaks: usize,
}

impl TextBuilder {
    fn push(&mut self, text: &str, style: SpanStyle) {
        if text.is_empty() {
            return;
        }
        if !self.text.is_empty() && self.pending_breaks != 0 {
            let trailing_breaks = self
                .text
                .bytes()
                .rev()
                .take(self.pending_breaks)
                .take_while(|&byte| byte == b'\n')
                .count();
            for _ in trailing_breaks..self.pending_breaks {
                self.text.push('\n');
            }
        }
        self.pending_breaks = 0;
        let start = self.text.len();
        self.text.push_str(text);
        if style != SpanStyle::default() {
            if let Some((last, previous_style)) = self.spans.last_mut()
                && *previous_style == style
                && last.end == start
            {
                last.end = self.text.len();
            } else {
                self.spans.push((start..self.text.len(), style));
            }
        }
    }
}

impl FormattedText {
    pub fn plain(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            spans: Arc::default(),
        }
    }

    pub fn from_markdown(source: &str) -> Self {
        if source.len() > MAX_FORMATTED_BYTES {
            return Self::plain(source);
        }
        let mut output = TextBuilder::default();
        let mut style = SpanStyle::default();
        let mut styles = Vec::new();
        let mut lists = Vec::new();
        let mut links = Vec::new();
        for event in Parser::new(source) {
            match event {
                Event::Start(tag) => {
                    styles.push(style);
                    match tag {
                        Tag::Emphasis => style.italic = true,
                        Tag::Strong | Tag::Heading { .. } => style.bold = true,
                        Tag::CodeBlock(_) => style.code = true,
                        Tag::List(start) => lists.push(start),
                        Tag::Item => {
                            output.pending_breaks = output.pending_breaks.max(1);
                            for _ in 1..lists.len() {
                                output.push("  ", style);
                            }
                            if let Some(Some(number)) = lists.last_mut() {
                                output.push(&format!("{number}. "), style);
                                *number = number.saturating_add(1);
                            } else {
                                output.push("• ", style);
                            }
                        }
                        Tag::Link { dest_url, .. } => links.push((output.text.len(), dest_url)),
                        // An image contributes only its alt text, never its URI.
                        _ => {}
                    }
                }
                Event::End(tag) => {
                    style = styles.pop().unwrap_or_default();
                    match tag {
                        TagEnd::Link => {
                            if let Some((start, target)) = links.pop() {
                                let label = output.text[start..].trim();
                                if label.is_empty() {
                                    output.push(&target, style);
                                } else if !target.is_empty() && label != target.as_ref() {
                                    output.push(" (", style);
                                    output.push(&target, style);
                                    output.push(")", style);
                                }
                            }
                        }
                        TagEnd::List(_) => {
                            lists.pop();
                            output.pending_breaks = if lists.is_empty() { 2 } else { 1 };
                        }
                        TagEnd::Paragraph
                        | TagEnd::HtmlBlock
                        | TagEnd::Heading(_)
                        | TagEnd::CodeBlock
                        | TagEnd::BlockQuote(_) => {
                            output.pending_breaks = if lists.is_empty() { 2 } else { 1 };
                        }
                        _ => {}
                    }
                }
                Event::Text(text) | Event::Html(text) | Event::InlineHtml(text) => {
                    output.push(&text, style)
                }
                Event::Code(text) => output.push(
                    &text,
                    SpanStyle {
                        code: true,
                        ..style
                    },
                ),
                Event::SoftBreak => output.push(" ", style),
                Event::HardBreak => output.push("\n", style),
                _ => {}
            }
            if output.text.len() > MAX_FORMATTED_BYTES {
                return Self::plain(source);
            }
        }
        Self {
            text: output.text.into(),
            spans: output.spans.into(),
        }
    }

    pub fn text(&self) -> &SharedString {
        &self.text
    }
}

impl RenderOnce for FormattedText {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = StyledText::new(self.text);
        if self.spans.is_empty() {
            return text;
        }
        let highlights = self.spans.iter().map(|(range, style)| {
            (
                range.clone(),
                HighlightStyle {
                    font_weight: style.bold.then_some(FontWeight::BOLD),
                    font_style: style.italic.then_some(FontStyle::Italic),
                    background_color: style.code.then(|| cx.theme().colors().element_background),
                    ..Default::default()
                },
            )
        });
        let code_fonts = self
            .spans
            .iter()
            .filter(|(_, style)| style.code)
            .map(|(range, _)| {
                (
                    range.clone(),
                    theme::theme_settings(cx).buffer_font(cx).family.clone(),
                )
            });
        text.with_highlights(highlights)
            .with_font_family_overrides(code_fonts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_and_list_markers_survive_without_interactive_content() {
        let text = FormattedText::from_markdown(
            "**bold *é* `code`**\n\n3. [Docs](https://example.org)\n4. ![Logo](file:///unused)\n   - <https://example.org>",
        );
        assert_eq!(
            text.text(),
            "bold é code\n\n3. Docs (https://example.org)\n4. Logo\n  • https://example.org"
        );
        let styles = text
            .spans
            .iter()
            .map(|(range, style)| {
                (
                    &text.text[range.clone()],
                    style.bold,
                    style.italic,
                    style.code,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            styles,
            [
                ("bold ", true, false, false),
                ("é", true, true, false),
                (" ", true, false, false),
                ("code", true, false, true)
            ]
        );
    }

    #[test]
    fn markup_in_code_html_and_link_targets_is_never_reinterpreted() {
        let text = FormattedText::from_markdown(
            "`[x](a)` <img src='photo'> [**details**](custom:open) ![preview](data:image/png;base64,AAAA)\n\n```mermaid\ngraph TD\n```",
        );
        assert_eq!(
            text.text(),
            "[x](a) <img src='photo'> details (custom:open) preview\n\ngraph TD\n"
        );
        let literal = FormattedText::plain("**name** [link](target)");
        assert_eq!(literal.text(), "**name** [link](target)");
        assert!(literal.spans.is_empty());
    }

    #[test]
    fn large_reference_expansions_fall_back_to_literal_text() {
        let source = format!(
            "[link][docs] [link][docs]\n\n[docs]: https://example.org/{}",
            "a".repeat(MAX_FORMATTED_BYTES / 2)
        );
        assert!(source.len() < MAX_FORMATTED_BYTES);
        assert_eq!(
            FormattedText::from_markdown(&source).text(),
            source.as_str()
        );
    }
}
