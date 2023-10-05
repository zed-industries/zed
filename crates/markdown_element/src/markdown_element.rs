use std::{ops::Range, sync::Arc};

use futures::FutureExt;
use gpui::{
    color::Color,
    elements::Text,
    fonts::{HighlightStyle, TextStyle, Underline, Weight},
    platform::{CursorStyle, MouseButton},
    AnyElement, CursorRegion, Element, MouseRegion,
};
use language::{HighlightId, Language, LanguageRegistry};
use theme::SyntaxTheme;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Highlight {
    Id(HighlightId),
    Highlight(HighlightStyle),
}

#[derive(Debug, Clone)]
pub struct MarkdownData {
    text: String,
    highlights: Vec<(Range<usize>, Highlight)>,
    region_ranges: Vec<Range<usize>>,
    regions: Vec<RenderedRegion>,
}

#[derive(Debug, Clone)]
struct RenderedRegion {
    code: bool,
    link_url: Option<String>,
}

pub struct MarkdownElement {
    data: Arc<MarkdownData>,
    syntax: Arc<SyntaxTheme>,
    style: TextStyle,
    code_span_background_color: Color,
}

impl MarkdownElement {
    pub fn new(
        data: Arc<MarkdownData>,
        style: TextStyle,
        syntax: Arc<SyntaxTheme>,
        code_span_background_color: Color,
    ) -> Self {
        Self {
            data,
            style,
            syntax,
            code_span_background_color,
        }
    }
}

impl<V: 'static> Element<V> for MarkdownElement {
    type LayoutState = AnyElement<V>;

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        cx: &mut gpui::ViewContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        let mut region_id = 0;
        let view_id = cx.view_id();

        let code_span_background_color = self.code_span_background_color;
        let data = self.data.clone();
        let mut element = Text::new(self.data.text.clone(), self.style.clone())
            .with_highlights(
                self.data
                    .highlights
                    .iter()
                    .filter_map(|(range, highlight)| {
                        let style = match highlight {
                            Highlight::Id(id) => id.style(&self.syntax)?,
                            Highlight::Highlight(style) => style.clone(),
                        };
                        Some((range.clone(), style))
                    })
                    .collect::<Vec<_>>(),
            )
            .with_custom_runs(self.data.region_ranges.clone(), move |ix, bounds, cx| {
                region_id += 1;
                let region = data.regions[ix].clone();
                if let Some(url) = region.link_url {
                    cx.scene().push_cursor_region(CursorRegion {
                        bounds,
                        style: CursorStyle::PointingHand,
                    });
                    cx.scene().push_mouse_region(
                        MouseRegion::new::<Self>(view_id, region_id, bounds)
                            .on_click::<V, _>(MouseButton::Left, move |_, _, cx| {
                                cx.platform().open_url(&url)
                            }),
                    );
                }
                if region.code {
                    cx.scene().push_quad(gpui::Quad {
                        bounds,
                        background: Some(code_span_background_color),
                        border: Default::default(),
                        corner_radii: (2.0).into(),
                    });
                }
            })
            .with_soft_wrap(true)
            .into_any();

        let constraint = element.layout(constraint, view, cx);

        (constraint, element)
    }

    fn paint(
        &mut self,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut gpui::ViewContext<V>,
    ) -> Self::PaintState {
        layout.paint(bounds.origin(), visible_bounds, view, cx);
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _: gpui::geometry::rect::RectF,
        _: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> Option<gpui::geometry::rect::RectF> {
        layout.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> gpui::serde_json::Value {
        layout.debug(view, cx)
    }
}

pub fn render_markdown(block: String, language_registry: &Arc<LanguageRegistry>) -> MarkdownData {
    let mut text = String::new();
    let mut highlights = Vec::new();
    let mut region_ranges = Vec::new();
    let mut regions = Vec::new();

    use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

    let mut bold_depth = 0;
    let mut italic_depth = 0;
    let mut link_url = None;
    let mut current_language = None;
    let mut list_stack = Vec::new();

    for event in Parser::new_ext(&block, Options::all()) {
        let prev_len = text.len();
        match event {
            Event::Text(t) => {
                if let Some(language) = &current_language {
                    render_code(&mut text, &mut highlights, t.as_ref(), language);
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
                            if last_range.end == prev_len
                                && last_style == &Highlight::Highlight(style)
                            {
                                last_range.end = text.len();
                                new_highlight = false;
                            }
                        }
                        if new_highlight {
                            highlights.push((prev_len..text.len(), Highlight::Highlight(style)));
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
                        Highlight::Highlight(HighlightStyle {
                            underline: Some(Underline {
                                thickness: 1.0.into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ));
                }
                regions.push(RenderedRegion {
                    code: true,
                    link_url: link_url.clone(),
                });
            }
            Event::Start(tag) => match tag {
                Tag::Paragraph => new_paragraph(&mut text, &mut list_stack),
                Tag::Heading(_, _, _) => {
                    new_paragraph(&mut text, &mut list_stack);
                    bold_depth += 1;
                }
                Tag::CodeBlock(kind) => {
                    new_paragraph(&mut text, &mut list_stack);
                    current_language = if let CodeBlockKind::Fenced(language) = kind {
                        language_registry
                            .language_for_name(language.as_ref())
                            .now_or_never()
                            .and_then(Result::ok)
                    } else {
                        None
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

    MarkdownData {
        text: text.trim().to_string(),
        highlights,
        region_ranges,
        regions,
    }
}

fn render_code(
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

fn new_paragraph(text: &mut String, list_stack: &mut Vec<(Option<u64>, bool)>) {
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
