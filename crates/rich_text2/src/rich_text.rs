use std::{ops::Range, sync::Arc};

use anyhow::bail;
use futures::FutureExt;
use gpui::{AnyElement, FontStyle, FontWeight, HighlightStyle, UnderlineStyle, WindowContext};
use language::{HighlightId, Language, LanguageRegistry};
use util::RangeExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Highlight {
    Id(HighlightId),
    Highlight(HighlightStyle),
    Mention,
    SelfMention,
}

impl From<HighlightStyle> for Highlight {
    fn from(style: HighlightStyle) -> Self {
        Self::Highlight(style)
    }
}

impl From<HighlightId> for Highlight {
    fn from(style: HighlightId) -> Self {
        Self::Id(style)
    }
}

#[derive(Debug, Clone)]
pub struct RichText {
    pub text: String,
    pub highlights: Vec<(Range<usize>, Highlight)>,
    pub region_ranges: Vec<Range<usize>>,
    pub regions: Vec<RenderedRegion>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundKind {
    Code,
    /// A mention background for non-self user.
    Mention,
    SelfMention,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedRegion {
    pub background_kind: Option<BackgroundKind>,
    pub link_url: Option<String>,
}

/// Allows one to specify extra links to the rendered markdown, which can be used
/// for e.g. mentions.
pub struct Mention {
    pub range: Range<usize>,
    pub is_self_mention: bool,
}

impl RichText {
    pub fn element(&self, _cx: &mut WindowContext) -> AnyElement {
        todo!();

        // let mut region_id = 0;
        // let view_id = cx.view_id();

        // let regions = self.regions.clone();

        // enum Markdown {}
        // Text::new(self.text.clone(), style.text.clone())
        //     .with_highlights(
        //         self.highlights
        //             .iter()
        //             .filter_map(|(range, highlight)| {
        //                 let style = match highlight {
        //                     Highlight::Id(id) => id.style(&syntax)?,
        //                     Highlight::Highlight(style) => style.clone(),
        //                     Highlight::Mention => style.mention_highlight,
        //                     Highlight::SelfMention => style.self_mention_highlight,
        //                 };
        //                 Some((range.clone(), style))
        //             })
        //             .collect::<Vec<_>>(),
        //     )
        //     .with_custom_runs(self.region_ranges.clone(), move |ix, bounds, cx| {
        //         region_id += 1;
        //         let region = regions[ix].clone();
        //         if let Some(url) = region.link_url {
        //             cx.scene().push_cursor_region(CursorRegion {
        //                 bounds,
        //                 style: CursorStyle::PointingHand,
        //             });
        //             cx.scene().push_mouse_region(
        //                 MouseRegion::new::<Markdown>(view_id, region_id, bounds)
        //                     .on_click::<V, _>(MouseButton::Left, move |_, _, cx| {
        //                         cx.platform().open_url(&url)
        //                     }),
        //             );
        //         }
        //         if let Some(region_kind) = &region.background_kind {
        //             let background = match region_kind {
        //                 BackgroundKind::Code => style.code_background,
        //                 BackgroundKind::Mention => style.mention_background,
        //                 BackgroundKind::SelfMention => style.self_mention_background,
        //             };
        //             if background.is_some() {
        //                 cx.scene().push_quad(gpui::Quad {
        //                     bounds,
        //                     background,
        //                     border: Default::default(),
        //                     corner_radii: (2.0).into(),
        //                 });
        //             }
        //         }
        //     })
        //     .with_soft_wrap(true)
        //     .into_any()
    }

    pub fn add_mention(
        &mut self,
        range: Range<usize>,
        is_current_user: bool,
        mention_style: HighlightStyle,
    ) -> anyhow::Result<()> {
        if range.end > self.text.len() {
            bail!(
                "Mention in range {range:?} is outside of bounds for a message of length {}",
                self.text.len()
            );
        }

        if is_current_user {
            self.region_ranges.push(range.clone());
            self.regions.push(RenderedRegion {
                background_kind: Some(BackgroundKind::Mention),
                link_url: None,
            });
        }
        self.highlights
            .push((range, Highlight::Highlight(mention_style)));
        Ok(())
    }
}

pub fn render_markdown_mut(
    block: &str,
    mut mentions: &[Mention],
    language_registry: &Arc<LanguageRegistry>,
    language: Option<&Arc<Language>>,
    data: &mut RichText,
) {
    use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

    let mut bold_depth = 0;
    let mut italic_depth = 0;
    let mut link_url = None;
    let mut current_language = None;
    let mut list_stack = Vec::new();

    let options = Options::all();
    for (event, source_range) in Parser::new_ext(&block, options).into_offset_iter() {
        let prev_len = data.text.len();
        match event {
            Event::Text(t) => {
                if let Some(language) = &current_language {
                    render_code(&mut data.text, &mut data.highlights, t.as_ref(), language);
                } else {
                    if let Some(mention) = mentions.first() {
                        if source_range.contains_inclusive(&mention.range) {
                            mentions = &mentions[1..];
                            let range = (prev_len + mention.range.start - source_range.start)
                                ..(prev_len + mention.range.end - source_range.start);
                            data.highlights.push((
                                range.clone(),
                                if mention.is_self_mention {
                                    Highlight::SelfMention
                                } else {
                                    Highlight::Mention
                                },
                            ));
                            data.region_ranges.push(range);
                            data.regions.push(RenderedRegion {
                                background_kind: Some(if mention.is_self_mention {
                                    BackgroundKind::SelfMention
                                } else {
                                    BackgroundKind::Mention
                                }),
                                link_url: None,
                            });
                        }
                    }

                    data.text.push_str(t.as_ref());
                    let mut style = HighlightStyle::default();
                    if bold_depth > 0 {
                        style.font_weight = Some(FontWeight::BOLD);
                    }
                    if italic_depth > 0 {
                        style.font_style = Some(FontStyle::Italic);
                    }
                    if let Some(link_url) = link_url.clone() {
                        data.region_ranges.push(prev_len..data.text.len());
                        data.regions.push(RenderedRegion {
                            link_url: Some(link_url),
                            background_kind: None,
                        });
                        style.underline = Some(UnderlineStyle {
                            thickness: 1.0.into(),
                            ..Default::default()
                        });
                    }

                    if style != HighlightStyle::default() {
                        let mut new_highlight = true;
                        if let Some((last_range, last_style)) = data.highlights.last_mut() {
                            if last_range.end == prev_len
                                && last_style == &Highlight::Highlight(style)
                            {
                                last_range.end = data.text.len();
                                new_highlight = false;
                            }
                        }
                        if new_highlight {
                            data.highlights
                                .push((prev_len..data.text.len(), Highlight::Highlight(style)));
                        }
                    }
                }
            }
            Event::Code(t) => {
                data.text.push_str(t.as_ref());
                data.region_ranges.push(prev_len..data.text.len());
                if link_url.is_some() {
                    data.highlights.push((
                        prev_len..data.text.len(),
                        Highlight::Highlight(HighlightStyle {
                            underline: Some(UnderlineStyle {
                                thickness: 1.0.into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ));
                }
                data.regions.push(RenderedRegion {
                    background_kind: Some(BackgroundKind::Code),
                    link_url: link_url.clone(),
                });
            }
            Event::Start(tag) => match tag {
                Tag::Paragraph => new_paragraph(&mut data.text, &mut list_stack),
                Tag::Heading(_, _, _) => {
                    new_paragraph(&mut data.text, &mut list_stack);
                    bold_depth += 1;
                }
                Tag::CodeBlock(kind) => {
                    new_paragraph(&mut data.text, &mut list_stack);
                    current_language = if let CodeBlockKind::Fenced(language) = kind {
                        language_registry
                            .language_for_name(language.as_ref())
                            .now_or_never()
                            .and_then(Result::ok)
                    } else {
                        language.cloned()
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
                        if !data.text.is_empty() && !data.text.ends_with('\n') {
                            data.text.push('\n');
                        }
                        for _ in 0..len - 1 {
                            data.text.push_str("  ");
                        }
                        if let Some(number) = list_number {
                            data.text.push_str(&format!("{}. ", number));
                            *number += 1;
                            *has_content = false;
                        } else {
                            data.text.push_str("- ");
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
            Event::HardBreak => data.text.push('\n'),
            Event::SoftBreak => data.text.push(' '),
            _ => {}
        }
    }
}

pub fn render_markdown(
    block: String,
    mentions: &[Mention],
    language_registry: &Arc<LanguageRegistry>,
    language: Option<&Arc<Language>>,
) -> RichText {
    let mut data = RichText {
        text: Default::default(),
        highlights: Default::default(),
        region_ranges: Default::default(),
        regions: Default::default(),
    };

    render_markdown_mut(&block, mentions, language_registry, language, &mut data);

    data.text = data.text.trim().to_string();

    data
}

pub fn render_code(
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, Highlight)>,
    content: &str,
    language: &Arc<Language>,
) {
    let prev_len = text.len();
    text.push_str(content);
    for (range, highlight_id) in language.highlight_text(&content.into(), 0..content.len()) {
        highlights.push((
            prev_len + range.start..prev_len + range.end,
            Highlight::Id(highlight_id),
        ));
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
