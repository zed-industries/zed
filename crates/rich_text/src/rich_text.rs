use futures::FutureExt;
use gpui::{
    AnyElement, AnyView, App, ElementId, FontStyle, FontWeight, HighlightStyle, InteractiveText,
    IntoElement, SharedString, StrikethroughStyle, StyledText, UnderlineStyle, Window,
};
use language::{HighlightId, Language, LanguageRegistry};
use std::{ops::Range, sync::Arc};
use theme::ActiveTheme;
use ui::LinkPreview;
use util::RangeExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Highlight {
    Code,
    Id(HighlightId),
    InlineCode(bool),
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

#[derive(Clone, Default)]
pub struct RichText {
    pub text: SharedString,
    pub highlights: Vec<(Range<usize>, Highlight)>,
    pub link_ranges: Vec<Range<usize>>,
    pub link_urls: Arc<[String]>,

    pub custom_ranges: Vec<Range<usize>>,
    custom_ranges_tooltip_fn:
        Option<Arc<dyn Fn(usize, Range<usize>, &mut Window, &mut App) -> Option<AnyView>>>,
}

/// Allows one to specify extra links to the rendered markdown, which can be used
/// for e.g. mentions.
#[derive(Debug)]
pub struct Mention {
    pub range: Range<usize>,
    pub is_self_mention: bool,
}

impl RichText {
    pub fn new(
        block: String,
        mentions: &[Mention],
        language_registry: &Arc<LanguageRegistry>,
    ) -> Self {
        let mut text = String::new();
        let mut highlights = Vec::new();
        let mut link_ranges = Vec::new();
        let mut link_urls = Vec::new();
        render_markdown_mut(
            &block,
            mentions,
            language_registry,
            None,
            &mut text,
            &mut highlights,
            &mut link_ranges,
            &mut link_urls,
        );
        text.truncate(text.trim_end().len());

        RichText {
            text: SharedString::from(text),
            link_urls: link_urls.into(),
            link_ranges,
            highlights,
            custom_ranges: Vec::new(),
            custom_ranges_tooltip_fn: None,
        }
    }

    pub fn set_tooltip_builder_for_custom_ranges(
        &mut self,
        f: impl Fn(usize, Range<usize>, &mut Window, &mut App) -> Option<AnyView> + 'static,
    ) {
        self.custom_ranges_tooltip_fn = Some(Arc::new(f));
    }

    pub fn element(&self, id: ElementId, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.theme();
        let code_background = theme.colors().surface_background;

        InteractiveText::new(
            id,
            StyledText::new(self.text.clone()).with_default_highlights(
                &window.text_style(),
                self.highlights.iter().map(|(range, highlight)| {
                    (
                        range.clone(),
                        match highlight {
                            Highlight::Code => HighlightStyle {
                                background_color: Some(code_background),
                                ..Default::default()
                            },
                            Highlight::Id(id) => HighlightStyle {
                                background_color: Some(code_background),
                                ..id.style(theme.syntax()).unwrap_or_default()
                            },
                            Highlight::InlineCode(link) => {
                                if *link {
                                    HighlightStyle {
                                        background_color: Some(code_background),
                                        underline: Some(UnderlineStyle {
                                            thickness: 1.0.into(),
                                            ..Default::default()
                                        }),
                                        ..Default::default()
                                    }
                                } else {
                                    HighlightStyle {
                                        background_color: Some(code_background),
                                        ..Default::default()
                                    }
                                }
                            }
                            Highlight::Highlight(highlight) => *highlight,
                            Highlight::Mention => HighlightStyle {
                                font_weight: Some(FontWeight::BOLD),
                                ..Default::default()
                            },
                            Highlight::SelfMention => HighlightStyle {
                                font_weight: Some(FontWeight::BOLD),
                                ..Default::default()
                            },
                        },
                    )
                }),
            ),
        )
        .on_click(self.link_ranges.clone(), {
            let link_urls = self.link_urls.clone();
            move |ix, _, cx| {
                let url = &link_urls[ix];
                if url.starts_with("http") {
                    cx.open_url(url);
                }
            }
        })
        .tooltip({
            let link_ranges = self.link_ranges.clone();
            let link_urls = self.link_urls.clone();
            let custom_tooltip_ranges = self.custom_ranges.clone();
            let custom_tooltip_fn = self.custom_ranges_tooltip_fn.clone();
            move |idx, window, cx| {
                for (ix, range) in link_ranges.iter().enumerate() {
                    if range.contains(&idx) {
                        return Some(LinkPreview::new(&link_urls[ix], cx));
                    }
                }
                for range in &custom_tooltip_ranges {
                    if range.contains(&idx)
                        && let Some(f) = &custom_tooltip_fn
                    {
                        return f(idx, range.clone(), window, cx);
                    }
                }
                None
            }
        })
        .into_any_element()
    }
}

pub fn render_markdown_mut(
    block: &str,
    mut mentions: &[Mention],
    language_registry: &Arc<LanguageRegistry>,
    language: Option<&Arc<Language>>,
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, Highlight)>,
    link_ranges: &mut Vec<Range<usize>>,
    link_urls: &mut Vec<String>,
) {
    use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

    let mut bold_depth = 0;
    let mut italic_depth = 0;
    let mut strikethrough_depth = 0;
    let mut link_url = None;
    let mut current_language = None;
    let mut list_stack = Vec::new();

    let mut options = Options::all();
    options.remove(pulldown_cmark::Options::ENABLE_DEFINITION_LIST);

    for (event, source_range) in Parser::new_ext(block, options).into_offset_iter() {
        let prev_len = text.len();
        match event {
            Event::Text(t) => {
                if let Some(language) = &current_language {
                    render_code(text, highlights, t.as_ref(), language);
                } else {
                    while let Some(mention) = mentions.first() {
                        if !source_range.contains_inclusive(&mention.range) {
                            break;
                        }
                        mentions = &mentions[1..];
                        let range = (prev_len + mention.range.start - source_range.start)
                            ..(prev_len + mention.range.end - source_range.start);
                        highlights.push((
                            range.clone(),
                            if mention.is_self_mention {
                                Highlight::SelfMention
                            } else {
                                Highlight::Mention
                            },
                        ));
                    }

                    text.push_str(t.as_ref());
                    let mut style = HighlightStyle::default();
                    if bold_depth > 0 {
                        style.font_weight = Some(FontWeight::BOLD);
                    }
                    if italic_depth > 0 {
                        style.font_style = Some(FontStyle::Italic);
                    }
                    if strikethrough_depth > 0 {
                        style.strikethrough = Some(StrikethroughStyle {
                            thickness: 1.0.into(),
                            ..Default::default()
                        });
                    }
                    let last_run_len = if let Some(link_url) = link_url.clone() {
                        link_ranges.push(prev_len..text.len());
                        link_urls.push(link_url);
                        style.underline = Some(UnderlineStyle {
                            thickness: 1.0.into(),
                            ..Default::default()
                        });
                        prev_len
                    } else {
                        // Manually scan for links
                        let mut finder = linkify::LinkFinder::new();
                        finder.kinds(&[linkify::LinkKind::Url]);
                        let mut last_link_len = prev_len;
                        for link in finder.links(&t) {
                            let start = link.start();
                            let end = link.end();
                            let range = (prev_len + start)..(prev_len + end);
                            link_ranges.push(range.clone());
                            link_urls.push(link.as_str().to_string());

                            // If there is a style before we match a link, we have to add this to the highlighted ranges
                            if style != HighlightStyle::default() && last_link_len < link.start() {
                                highlights.push((
                                    last_link_len..link.start(),
                                    Highlight::Highlight(style),
                                ));
                            }

                            highlights.push((
                                range,
                                Highlight::Highlight(HighlightStyle {
                                    underline: Some(UnderlineStyle {
                                        thickness: 1.0.into(),
                                        ..Default::default()
                                    }),
                                    ..style
                                }),
                            ));

                            last_link_len = end;
                        }
                        last_link_len
                    };

                    if style != HighlightStyle::default() && last_run_len < text.len() {
                        let mut new_highlight = true;
                        if let Some((last_range, last_style)) = highlights.last_mut()
                            && last_range.end == last_run_len
                            && last_style == &Highlight::Highlight(style)
                        {
                            last_range.end = text.len();
                            new_highlight = false;
                        }
                        if new_highlight {
                            highlights
                                .push((last_run_len..text.len(), Highlight::Highlight(style)));
                        }
                    }
                }
            }
            Event::Code(t) => {
                text.push_str(t.as_ref());
                let is_link = link_url.is_some();

                if let Some(link_url) = link_url.clone() {
                    link_ranges.push(prev_len..text.len());
                    link_urls.push(link_url);
                }

                highlights.push((prev_len..text.len(), Highlight::InlineCode(is_link)))
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
            Event::SoftBreak => text.push('\n'),
            _ => {}
        }
    }
}

pub fn render_code(
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, Highlight)>,
    content: &str,
    language: &Arc<Language>,
) {
    let prev_len = text.len();
    text.push_str(content);
    let mut offset = 0;
    for (range, highlight_id) in language.highlight_text(&content.into(), 0..content.len()) {
        if range.start > offset {
            highlights.push((prev_len + offset..prev_len + range.start, Highlight::Code));
        }
        highlights.push((
            prev_len + range.start..prev_len + range.end,
            Highlight::Id(highlight_id),
        ));
        offset = range.end;
    }
    if offset < content.len() {
        highlights.push((prev_len + offset..prev_len + content.len(), Highlight::Code));
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
