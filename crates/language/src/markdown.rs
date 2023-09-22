use std::ops::Range;
use std::sync::Arc;

use crate::{Language, LanguageRegistry};
use futures::FutureExt;
use gpui::{
    elements::Text,
    fonts::{HighlightStyle, Underline, Weight},
    platform::{CursorStyle, MouseButton},
    CursorRegion, MouseRegion, ViewContext,
};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

#[derive(Debug, Clone)]
pub struct RenderedMarkdown {
    text: String,
    highlights: Vec<(Range<usize>, HighlightStyle)>,
    region_ranges: Vec<Range<usize>>,
    regions: Vec<RenderedRegion>,
}

// impl RenderedMarkdown {
//     pub fn render(&self, style: &theme::Editor, cx: &mut ViewContext<Editor>) -> Text {
//         let code_span_background_color = style.document_highlight_read_background;
//         let view_id = cx.view_id();
//         let mut region_id = 0;
//         Text::new(text, style.text.clone())
//             .with_highlights(highlights)
//             .with_custom_runs(region_ranges, move |ix, bounds, scene, _| {
//                 region_id += 1;
//                 let region = regions[ix].clone();
//                 if let Some(url) = region.link_url {
//                     scene.push_cursor_region(CursorRegion {
//                         bounds,
//                         style: CursorStyle::PointingHand,
//                     });
//                     scene.push_mouse_region(
//                         MouseRegion::new::<Editor>(view_id, region_id, bounds)
//                             .on_click::<Editor, _>(MouseButton::Left, move |_, _, cx| {
//                                 cx.platform().open_url(&url)
//                             }),
//                     );
//                 }
//                 if region.code {
//                     scene.push_quad(gpui::Quad {
//                         bounds,
//                         background: Some(code_span_background_color),
//                         border: Default::default(),
//                         corner_radii: (2.0).into(),
//                     });
//                 }
//             })
//             .with_soft_wrap(true)
//     }
// }

#[derive(Debug, Clone)]
pub struct RenderedRegion {
    pub code: bool,
    pub link_url: Option<String>,
}

pub fn render_markdown(
    markdown: &str,
    language_registry: &Arc<LanguageRegistry>,
    language: &Option<Arc<Language>>,
    style: &theme::Editor,
) -> RenderedMarkdown {
    let mut text = String::new();
    let mut highlights = Vec::new();
    let mut region_ranges = Vec::new();
    let mut regions = Vec::new();

    render_markdown_block(
        markdown,
        language_registry,
        language,
        style,
        &mut text,
        &mut highlights,
        &mut region_ranges,
        &mut regions,
    );

    RenderedMarkdown {
        text,
        highlights,
        region_ranges,
        regions,
    }
}

pub fn render_markdown_block(
    markdown: &str,
    language_registry: &Arc<LanguageRegistry>,
    language: &Option<Arc<Language>>,
    style: &theme::Editor,
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, HighlightStyle)>,
    region_ranges: &mut Vec<Range<usize>>,
    regions: &mut Vec<RenderedRegion>,
) {
    let mut bold_depth = 0;
    let mut italic_depth = 0;
    let mut link_url = None;
    let mut current_language = None;
    let mut list_stack = Vec::new();

    for event in Parser::new_ext(&markdown, Options::all()) {
        let prev_len = text.len();
        match event {
            Event::Text(t) => {
                if let Some(language) = &current_language {
                    render_code(text, highlights, t.as_ref(), language, style);
                } else {
                    text.push_str(t.as_ref());

                    let mut style = HighlightStyle::default();
                    if bold_depth > 0 {
                        style.weight = Some(Weight::BOLD);
                    }
                    if italic_depth > 0 {
                        style.italic = Some(true);
                    }
                    if let Some(link_url) = link_url.clone() {
                        region_ranges.push(prev_len..text.len());
                        regions.push(RenderedRegion {
                            link_url: Some(link_url),
                            code: false,
                        });
                        style.underline = Some(Underline {
                            thickness: 1.0.into(),
                            ..Default::default()
                        });
                    }

                    if style != HighlightStyle::default() {
                        let mut new_highlight = true;
                        if let Some((last_range, last_style)) = highlights.last_mut() {
                            if last_range.end == prev_len && last_style == &style {
                                last_range.end = text.len();
                                new_highlight = false;
                            }
                        }
                        if new_highlight {
                            highlights.push((prev_len..text.len(), style));
                        }
                    }
                }
            }

            Event::Code(t) => {
                text.push_str(t.as_ref());
                region_ranges.push(prev_len..text.len());
                if link_url.is_some() {
                    highlights.push((
                        prev_len..text.len(),
                        HighlightStyle {
                            underline: Some(Underline {
                                thickness: 1.0.into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                    ));
                }
                regions.push(RenderedRegion {
                    code: true,
                    link_url: link_url.clone(),
                });
            }

            Event::Start(tag) => match tag {
                Tag::Paragraph => new_paragraph(text, &mut list_stack),

                Tag::Heading(_, _, _) => {
                    new_paragraph(text, &mut list_stack);
                    bold_depth += 1;
                }

                Tag::CodeBlock(kind) => {
                    new_paragraph(text, &mut list_stack);
                    current_language = if let CodeBlockKind::Fenced(language) = kind {
                        language_registry
                            .language_for_name(language.as_ref())
                            .now_or_never()
                            .and_then(Result::ok)
                    } else {
                        language.clone()
                    }
                }

                Tag::Emphasis => italic_depth += 1,

                Tag::Strong => bold_depth += 1,

                Tag::Link(_, url, _) => link_url = Some(url.to_string()),

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
                Tag::Heading(_, _, _) => bold_depth -= 1,
                Tag::CodeBlock(_) => current_language = None,
                Tag::Emphasis => italic_depth -= 1,
                Tag::Strong => bold_depth -= 1,
                Tag::Link(_, _, _) => link_url = None,
                Tag::List(_) => drop(list_stack.pop()),
                _ => {}
            },

            Event::HardBreak => text.push('\n'),

            Event::SoftBreak => text.push(' '),

            _ => {}
        }
    }
}

pub fn render_code(
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, HighlightStyle)>,
    content: &str,
    language: &Arc<Language>,
    style: &theme::Editor,
) {
    let prev_len = text.len();
    text.push_str(content);
    for (range, highlight_id) in language.highlight_text(&content.into(), 0..content.len()) {
        if let Some(style) = highlight_id.style(&style.syntax) {
            highlights.push((prev_len + range.start..prev_len + range.end, style));
        }
    }
}

pub fn new_paragraph(text: &mut String, list_stack: &mut Vec<(Option<u64>, bool)>) {
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
