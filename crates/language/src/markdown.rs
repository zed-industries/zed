//! Provides Markdown-related constructs.

use std::sync::Arc;
use std::{ops::Range, path::PathBuf};

use crate::{HighlightId, Language, LanguageRegistry};
use gpui::{px, FontStyle, FontWeight, HighlightStyle, StrikethroughStyle, UnderlineStyle};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

/// Parsed Markdown content.
#[derive(Debug, Clone, Default)]
pub struct ParsedMarkdown {
    /// The Markdown text.
    pub text: String,
    /// The list of highlights contained in the Markdown document.
    pub highlights: Vec<(Range<usize>, MarkdownHighlight)>,
    /// The regions of the various ranges in the Markdown document.
    pub region_ranges: Vec<Range<usize>>,
    /// The regions of the Markdown document.
    pub regions: Vec<ParsedRegion>,
}

/// A run of highlighted Markdown text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownHighlight {
    /// A styled Markdown highlight.
    Style(MarkdownHighlightStyle),
    /// A highlighted code block.
    Code(HighlightId),
}

impl MarkdownHighlight {
    /// Converts this [`MarkdownHighlight`] to a [`HighlightStyle`].
    pub fn to_highlight_style(&self, theme: &theme::SyntaxTheme) -> Option<HighlightStyle> {
        match self {
            MarkdownHighlight::Style(style) => {
                let mut highlight = HighlightStyle::default();

                if style.italic {
                    highlight.font_style = Some(FontStyle::Italic);
                }

                if style.underline {
                    highlight.underline = Some(UnderlineStyle {
                        thickness: px(1.),
                        ..Default::default()
                    });
                }

                if style.strikethrough {
                    highlight.strikethrough = Some(StrikethroughStyle {
                        thickness: px(1.),
                        ..Default::default()
                    });
                }

                if style.weight != FontWeight::default() {
                    highlight.font_weight = Some(style.weight);
                }

                Some(highlight)
            }

            MarkdownHighlight::Code(id) => id.style(theme),
        }
    }
}

/// The style for a Markdown highlight.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MarkdownHighlightStyle {
    /// Whether the text should be italicized.
    pub italic: bool,
    /// Whether the text should be underlined.
    pub underline: bool,
    /// Whether the text should be struck through.
    pub strikethrough: bool,
    /// The weight of the text.
    pub weight: FontWeight,
}

/// A parsed region in a Markdown document.
#[derive(Debug, Clone)]
pub struct ParsedRegion {
    /// Whether the region is a code block.
    pub code: bool,
    /// The link contained in this region, if it has one.
    pub link: Option<Link>,
}

/// A Markdown link.
#[derive(Debug, Clone)]
pub enum Link {
    /// A link to a webpage.
    Web {
        /// The URL of the webpage.
        url: String,
    },
    /// A link to a path on the filesystem.
    Path {
        /// The path to the item.
        path: PathBuf,
    },
}

impl Link {
    fn identify(text: String) -> Option<Link> {
        if text.starts_with("http") {
            return Some(Link::Web { url: text });
        }

        let path = PathBuf::from(text);
        if path.is_absolute() {
            return Some(Link::Path { path });
        }

        None
    }
}

/// Parses a string of Markdown.
pub async fn parse_markdown(
    markdown: &str,
    language_registry: Option<&Arc<LanguageRegistry>>,
    language: Option<Arc<Language>>,
) -> ParsedMarkdown {
    let mut text = String::new();
    let mut highlights = Vec::new();
    let mut region_ranges = Vec::new();
    let mut regions = Vec::new();

    parse_markdown_block(
        markdown,
        language_registry,
        language,
        &mut text,
        &mut highlights,
        &mut region_ranges,
        &mut regions,
    )
    .await;

    ParsedMarkdown {
        text,
        highlights,
        region_ranges,
        regions,
    }
}

/// Parses a Markdown block.
pub async fn parse_markdown_block(
    markdown: &str,
    language_registry: Option<&Arc<LanguageRegistry>>,
    language: Option<Arc<Language>>,
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, MarkdownHighlight)>,
    region_ranges: &mut Vec<Range<usize>>,
    regions: &mut Vec<ParsedRegion>,
) {
    let mut bold_depth = 0;
    let mut italic_depth = 0;
    let mut strikethrough_depth = 0;
    let mut link_url = None;
    let mut current_language = None;
    let mut list_stack = Vec::new();

    let mut options = pulldown_cmark::Options::all();
    options.remove(pulldown_cmark::Options::ENABLE_DEFINITION_LIST);
    options.remove(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    for event in Parser::new_ext(markdown, options) {
        let prev_len = text.len();
        match event {
            Event::Text(t) => {
                if let Some(language) = &current_language {
                    highlight_code(text, highlights, t.as_ref(), language);
                } else {
                    text.push_str(t.as_ref());

                    let mut style = MarkdownHighlightStyle::default();

                    if bold_depth > 0 {
                        style.weight = FontWeight::BOLD;
                    }

                    if italic_depth > 0 {
                        style.italic = true;
                    }

                    if strikethrough_depth > 0 {
                        style.strikethrough = true;
                    }

                    if let Some(link) = link_url.clone().and_then(Link::identify) {
                        region_ranges.push(prev_len..text.len());
                        regions.push(ParsedRegion {
                            code: false,
                            link: Some(link),
                        });
                        style.underline = true;
                    }

                    if style != MarkdownHighlightStyle::default() {
                        let mut new_highlight = true;
                        if let Some((last_range, MarkdownHighlight::Style(last_style))) =
                            highlights.last_mut()
                        {
                            if last_range.end == prev_len && last_style == &style {
                                last_range.end = text.len();
                                new_highlight = false;
                            }
                        }
                        if new_highlight {
                            let range = prev_len..text.len();
                            highlights.push((range, MarkdownHighlight::Style(style)));
                        }
                    }
                }
            }

            Event::Code(t) => {
                text.push_str(t.as_ref());
                region_ranges.push(prev_len..text.len());

                let link = link_url.clone().and_then(Link::identify);
                if link.is_some() {
                    highlights.push((
                        prev_len..text.len(),
                        MarkdownHighlight::Style(MarkdownHighlightStyle {
                            underline: true,
                            ..Default::default()
                        }),
                    ));
                }
                regions.push(ParsedRegion { code: true, link });
            }

            Event::Start(tag) => match tag {
                Tag::Paragraph => new_paragraph(text, &mut list_stack),

                Tag::Heading { .. } => {
                    new_paragraph(text, &mut list_stack);
                    bold_depth += 1;
                }

                Tag::CodeBlock(kind) => {
                    new_paragraph(text, &mut list_stack);
                    current_language = if let CodeBlockKind::Fenced(language) = kind {
                        match language_registry {
                            None => None,
                            Some(language_registry) => language_registry
                                .language_for_name_or_extension(language.as_ref())
                                .await
                                .ok(),
                        }
                    } else {
                        language.clone()
                    }
                }

                Tag::Emphasis => italic_depth += 1,

                Tag::Strong => bold_depth += 1,

                Tag::Strikethrough => strikethrough_depth += 1,

                Tag::Link { dest_url, .. } => link_url = Some(dest_url.to_string()),

                Tag::List(number) => {
                    list_stack.push((number, false));
                }

                Tag::Item => {
                    let len = list_stack.len();
                    if let Some((list_number, has_content)) = list_stack.last_mut() {
                        *has_content = false;
                        if !text.is_empty() && !text.ends_with('\n') {
                            text.push('\n');
                        }
                        for _ in 0..len - 1 {
                            text.push_str("  ");
                        }
                        if let Some(number) = list_number {
                            text.push_str(&format!("{}. ", number));
                            *number += 1;
                            *has_content = false;
                        } else {
                            text.push_str("- ");
                        }
                    }
                }

                _ => {}
            },

            Event::End(tag) => match tag {
                TagEnd::Heading(_) => bold_depth -= 1,
                TagEnd::CodeBlock => current_language = None,
                TagEnd::Emphasis => italic_depth -= 1,
                TagEnd::Strong => bold_depth -= 1,
                TagEnd::Strikethrough => strikethrough_depth -= 1,
                TagEnd::Link => link_url = None,
                TagEnd::List(_) => drop(list_stack.pop()),
                _ => {}
            },

            Event::HardBreak => text.push('\n'),

            Event::SoftBreak => text.push(' '),

            _ => {}
        }
    }
}

/// Appends a highlighted run of text to the provided `text` buffer.
pub fn highlight_code(
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, MarkdownHighlight)>,
    content: &str,
    language: &Arc<Language>,
) {
    let prev_len = text.len();
    text.push_str(content);
    for (range, highlight_id) in language.highlight_text(&content.into(), 0..content.len()) {
        let highlight = MarkdownHighlight::Code(highlight_id);
        highlights.push((prev_len + range.start..prev_len + range.end, highlight));
    }
}

/// Appends a new paragraph to the provided `text` buffer.
pub fn new_paragraph(text: &mut String, list_stack: &mut [(Option<u64>, bool)]) {
    let mut is_subsequent_paragraph_of_list = false;
    if let Some((_, has_content)) = list_stack.last_mut() {
        if *has_content {
            is_subsequent_paragraph_of_list = true;
        } else {
            *has_content = true;
            return;
        }
    }

    if !text.is_empty() {
        if !text.ends_with('\n') {
            text.push('\n');
        }
        text.push('\n');
    }
    for _ in 0..list_stack.len().saturating_sub(1) {
        text.push_str("  ");
    }
    if is_subsequent_paragraph_of_list {
        text.push_str("  ");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dividers() {
        let input = r#"
### instance-method `format`

---
→ `void`
Parameters:
- `const int &`
- `const std::tm &`
- `int & dest`

---
```cpp
// In my_formatter_flag
public: void format(const int &, const std::tm &, int &dest)
```
"#;

        let mut options = pulldown_cmark::Options::all();
        options.remove(pulldown_cmark::Options::ENABLE_DEFINITION_LIST);
        options.remove(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

        let parser = pulldown_cmark::Parser::new_ext(input, options);
        for event in parser.into_iter() {
            println!("{:?}", event);
        }
    }
}
