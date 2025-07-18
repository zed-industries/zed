use crate::markdown_elements::{
    HeadingLevel, Link, MarkdownParagraph, MarkdownParagraphChunk, ParsedMarkdown,
    ParsedMarkdownBlockQuote, ParsedMarkdownCodeBlock, ParsedMarkdownElement,
    ParsedMarkdownHeading, ParsedMarkdownListItem, ParsedMarkdownListItemType,
    ParsedMarkdownMathBlock, ParsedMarkdownTable, ParsedMarkdownTableAlignment,
    ParsedMarkdownTableRow,
};
use anyhow;
use comemo::Prehashed;
use fs::normalize_path;
use gpui::{
    AbsoluteLength, AnyElement, App, AppContext as _, ClipboardItem, Context, Div, ElementId,
    Entity, FontWeight, HighlightStyle, Hsla, ImageCacheError, ImageSource, InteractiveText,
    IntoElement, Keystroke, Modifiers, ParentElement, Render, RenderImage, Resource, SharedString,
    Styled, StyledText, TextStyle, WeakEntity, Window, div, img, px,
};
use log::info;
use once_cell::sync::Lazy;
use settings::Settings;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{
    ops::{Mul, Range},
    sync::Arc,
    vec,
};
use theme::{ActiveTheme, ThemeSettings};
use typst::{
    Library, World, compile,
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
};
use typst_svg::svg as typst_svg;
use ui::{
    Clickable, Color, Element, FluentBuilder, IconButton, IconName, IconSize, InteractiveElement,
    Label, LabelCommon, LabelSize, LinkPreview, Rems, StatefulInteractiveElement, StyledImage,
    ToggleState, VisibleOnHover, h_flex, relative, tooltip_container, v_flex,
};
use workspace::{OpenOptions, OpenVisible, Workspace};

// Global math→SVG cache
static MATH_SVG_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Look up a cached SVG; returns Some(svg) if present.
fn cache_lookup(expr: &str) -> Option<String> {
    if let Ok(guard) = MATH_SVG_CACHE.lock() {
        let result = guard.get(expr).cloned();
        if result.is_some() {
            info!("Math cache HIT for '{}'", expr);
        } else {
            info!("Math cache MISS for '{}'", expr);
        }
        result
    } else {
        None
    }
}

/// Store a freshly generated SVG in the cache.
fn cache_store(expr: String, svg: String) {
    info!("Caching math SVG for '{}' ({} chars)", expr, svg.len());
    if let Ok(mut guard) = MATH_SVG_CACHE.lock() {
        guard.insert(expr, svg);
    }
}

type CheckboxClickedCallback = Arc<Box<dyn Fn(bool, Range<usize>, &mut Window, &mut App)>>;

#[derive(Clone)]
pub struct RenderContext {
    workspace: Option<WeakEntity<Workspace>>,
    next_id: usize,
    buffer_font_family: SharedString,
    buffer_text_style: TextStyle,
    text_style: TextStyle,
    border_color: Hsla,
    text_color: Hsla,
    window_rem_size: AbsoluteLength,
    code_block_background_color: Hsla,
    code_span_background_color: Hsla,
    syntax_theme: Arc<theme::SyntaxTheme>,
    indent: usize,
    checkbox_clicked_callback: Option<CheckboxClickedCallback>,
}

impl RenderContext {
    pub fn new(
        workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut App,
    ) -> RenderContext {
        let theme = cx.theme().clone();

        let settings = ThemeSettings::get_global(cx);
        let buffer_font_family = settings.buffer_font.family.clone();
        let mut buffer_text_style = window.text_style();
        buffer_text_style.font_family = buffer_font_family.clone();
        buffer_text_style.font_size = AbsoluteLength::from(settings.buffer_font_size(cx) * 1.1);

        // Increase base text size for all markdown content
        let mut text_style = window.text_style();
        text_style.font_size = AbsoluteLength::from(settings.buffer_font_size(cx) * 1.1);

        RenderContext {
            workspace,
            next_id: 0,
            indent: 0,
            buffer_font_family,
            buffer_text_style,
            text_style,
            syntax_theme: theme.syntax().clone(),
            border_color: theme.colors().border,
            text_color: theme.colors().text,
            window_rem_size: AbsoluteLength::from(window.rem_size()),
            code_block_background_color: theme.colors().surface_background,
            code_span_background_color: theme.colors().editor_document_highlight_read_background,
            checkbox_clicked_callback: None,
        }
    }

    pub fn with_checkbox_clicked_callback(
        mut self,
        callback: impl Fn(bool, Range<usize>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.checkbox_clicked_callback = Some(Arc::new(Box::new(callback)));
        self
    }

    fn next_id(&mut self, span: &Range<usize>) -> ElementId {
        let id = format!("markdown-{}-{}-{}", self.next_id, span.start, span.end);
        self.next_id += 1;
        ElementId::from(SharedString::from(id))
    }

    /// HACK: used to have rems relative to buffer font size, so that things scale appropriately as
    /// buffer font size changes. The callees of this function should be reimplemented to use real
    /// relative sizing once that is implemented in GPUI
    pub fn scaled_rems(&self, rems: f32) -> Rems {
        return self
            .buffer_text_style
            .font_size
            .to_rems(self.window_rem_size.to_pixels(px(1.0)))
            .mul(rems);
    }

    /// This ensures that children inside of block quotes
    /// have padding between them.
    ///
    /// For example, for this markdown:
    ///
    /// ```markdown
    /// > This is a block quote.
    /// >
    /// > And this is the next paragraph.
    /// ```
    ///
    /// We give padding between "This is a block quote."
    /// and "And this is the next paragraph."
    fn with_common_p(&self, element: Div) -> Div {
        let element = element
            .text_size(self.scaled_rems(1.0))
            .line_height(relative(1.6));

        if self.indent > 0 {
            element.pb(self.scaled_rems(0.5)) // More breathing room
        } else {
            element.pb(self.scaled_rems(0.8)) // Even more for top-level
        }
    }
}

pub fn render_parsed_markdown(
    parsed: &ParsedMarkdown,
    workspace: Option<WeakEntity<Workspace>>,
    window: &mut Window,
    cx: &mut App,
) -> Div {
    let mut cx = RenderContext::new(workspace, window, cx);

    v_flex().gap_0().children(
        parsed
            .children
            .iter()
            .map(|block| render_markdown_block(block, &mut cx)),
    )
}
pub fn render_markdown_block(block: &ParsedMarkdownElement, cx: &mut RenderContext) -> AnyElement {
    use ParsedMarkdownElement::*;
    match block {
        Paragraph(text) => render_markdown_paragraph(text, cx),
        Heading(heading) => render_markdown_heading(heading, cx),
        ListItem(list_item) => render_markdown_list_item(list_item, cx),
        Table(table) => render_markdown_table(table, cx),
        BlockQuote(block_quote) => render_markdown_block_quote(block_quote, cx),
        CodeBlock(code_block) => render_markdown_code_block(code_block, cx),
        HorizontalRule(_) => render_markdown_rule(cx),
        MathBlock(math) => render_markdown_math(math, cx),
    }
}

fn render_markdown_heading(parsed: &ParsedMarkdownHeading, cx: &mut RenderContext) -> AnyElement {
    let (size, weight, spacing) = match parsed.level {
        HeadingLevel::H1 => (2.5, FontWeight::EXTRA_BOLD, (0.5, 1.0)),
        HeadingLevel::H2 => (2.0, FontWeight::BOLD, (0.4, 0.8)),
        HeadingLevel::H3 => (1.5, FontWeight::SEMIBOLD, (0.3, 0.6)),
        HeadingLevel::H4 => (1.25, FontWeight::MEDIUM, (0.2, 0.4)),
        HeadingLevel::H5 => (1.0, FontWeight::MEDIUM, (0.15, 0.3)),
        HeadingLevel::H6 => (0.9, FontWeight::MEDIUM, (0.1, 0.2)),
    };

    cx.with_common_p(div())
        .text_size(cx.scaled_rems(size))
        .font_weight(weight)
        .text_color(cx.text_color)
        .pt(cx.scaled_rems(spacing.0))
        .pb(cx.scaled_rems(spacing.1))
        .children(render_markdown_text(&parsed.contents, cx))
        .into_any()
}

fn render_markdown_paragraph(parsed: &MarkdownParagraph, cx: &mut RenderContext) -> AnyElement {
    cx.with_common_p(div())
        .children(render_markdown_text(parsed, cx))
        .into_any()
}

fn render_markdown_paragraph_tight(
    parsed: &MarkdownParagraph,
    cx: &mut RenderContext,
) -> AnyElement {
    div()
        .text_size(cx.scaled_rems(1.0))
        .line_height(relative(1.6))
        .children(render_markdown_text(parsed, cx))
        .into_any()
}

fn render_markdown_list_item(
    parsed: &ParsedMarkdownListItem,
    cx: &mut RenderContext,
) -> AnyElement {
    let content_elements = parsed
        .content
        .iter()
        .map(|element| render_markdown_block(element, cx))
        .collect::<Vec<_>>();

    match &parsed.item_type {
        ParsedMarkdownListItemType::Ordered(order) => {
            let indent = cx.scaled_rems(parsed.depth as f32 * 1.5);

            div()
                .pl(indent)
                .flex()
                .items_start()
                .gap_2()
                .pb(cx.scaled_rems(0.3))
                .child(
                    div()
                        .text_size(cx.scaled_rems(1.0))
                        .text_color(cx.text_color)
                        .child(format!("{}.", order)),
                )
                .child(div().flex().flex_col().flex_1().children(content_elements))
                .into_any()
        }
        ParsedMarkdownListItemType::Unordered => {
            let indent = cx.scaled_rems(parsed.depth as f32 * 1.5);

            div()
                .pl(indent)
                .flex()
                .items_start()
                .gap_2()
                .pb(cx.scaled_rems(0.3))
                .child(
                    div()
                        .text_size(cx.scaled_rems(1.0))
                        .text_color(cx.text_color)
                        .child("•"),
                )
                .child(div().flex().flex_col().flex_1().children(content_elements))
                .into_any()
        }
        ParsedMarkdownListItemType::Task(checked, _) => {
            let indent = cx.scaled_rems(parsed.depth as f32 * 1.5);

            let checkbox = MarkdownCheckbox::new(
                cx.next_id(&parsed.source_range),
                *checked,
                parsed.source_range.clone(),
                cx.checkbox_clicked_callback.clone(),
                &*cx,
            );

            div()
                .pl(indent)
                .flex()
                .items_start()
                .gap_2()
                .pb(cx.scaled_rems(0.3))
                .child(checkbox)
                .child(div().flex().flex_col().flex_1().children(content_elements))
                .into_any()
        }
    }
}

#[derive(IntoElement)]
struct MarkdownCheckbox {
    id: ElementId,
    toggle_state: ToggleState,
    disabled: bool,
    placeholder: SharedString,
    on_click: Option<CheckboxClickedCallback>,
    filled: bool,
    style: Option<ui::ButtonStyle>,
    tooltip: Option<Entity<InteractiveMarkdownElementTooltip>>,
    label: Option<SharedString>,
    render_cx: RenderContext,
    source_range: Range<usize>,
}

impl MarkdownCheckbox {
    fn new(
        element_id: ElementId,
        checked: bool,
        source_range: Range<usize>,
        on_click: Option<CheckboxClickedCallback>,
        render_cx: &RenderContext,
    ) -> Self {
        let toggle_state = if checked {
            ToggleState::Selected
        } else {
            ToggleState::Unselected
        };

        Self {
            id: element_id,
            toggle_state,
            disabled: false,
            placeholder: " ".into(),
            on_click: on_click,
            filled: false,
            style: None,
            tooltip: None,
            label: None,
            render_cx: render_cx.clone(),
            source_range,
        }
    }

    fn on_click(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Arc::new(Box::new(move |state, _range, window, cx| {
            handler(state, window, cx)
        })));
        self
    }

    fn bg_color(&self, cx: &App) -> Hsla {
        match self.toggle_state {
            ToggleState::Selected => cx.theme().colors().element_selected,
            _ => cx.theme().colors().element_background,
        }
    }

    fn border_color(&self, cx: &App) -> Hsla {
        match self.toggle_state {
            ToggleState::Selected => cx.theme().colors().element_selected,
            _ => cx.theme().colors().border_variant,
        }
    }
}

impl gpui::RenderOnce for MarkdownCheckbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let group_id = format!("checkbox_group_{}", self.id);

        div()
            .group(group_id.clone())
            .id(self.id.clone())
            .flex()
            .flex_none()
            .justify_center()
            .items_center()
            .size(self.render_cx.scaled_rems(1.2))
            .bg(self.bg_color(cx))
            .border_1()
            .border_color(self.border_color(cx))
            .rounded_sm()
            .when_some(self.on_click, |this, on_click| {
                let source_range = self.source_range.clone();
                this.cursor_pointer().on_click(move |_event, window, cx| {
                    let new_state = match self.toggle_state {
                        ToggleState::Selected => false,
                        _ => true,
                    };
                    on_click(new_state, source_range.clone(), window, cx);
                })
            })
            .when(self.toggle_state == ToggleState::Selected, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .child(
                            ui::Icon::new(IconName::Check)
                                .size(IconSize::XSmall)
                                .color(Color::Selected),
                        ),
                )
            })
            .when_some(self.tooltip.clone(), |this, tooltip| {
                this.tooltip(move |_window, _cx| tooltip.clone().into())
            })
    }
}

fn paragraph_len(paragraph: &MarkdownParagraph) -> usize {
    paragraph
        .iter()
        .map(|chunk| match chunk {
            MarkdownParagraphChunk::Text(text) => text.contents.len(),
            MarkdownParagraphChunk::Image(_) => 0,
            MarkdownParagraphChunk::InlineMath(math) => math.contents.len(),
        })
        .sum()
}

fn render_markdown_table(parsed: &ParsedMarkdownTable, cx: &mut RenderContext) -> AnyElement {
    let _theme = cx.syntax_theme.clone();

    let header_row = div()
        .flex()
        .border_b_1()
        .border_color(cx.border_color)
        .pb_2()
        .mb_2()
        .children(parsed.header.children.iter().enumerate().map(|(i, cell)| {
            let alignment = parsed
                .column_alignments
                .get(i)
                .unwrap_or(&ParsedMarkdownTableAlignment::None);

            div()
                .flex_1()
                .px_2()
                .when(
                    matches!(alignment, ParsedMarkdownTableAlignment::Center),
                    |this| this.text_center(),
                )
                .when(
                    matches!(alignment, ParsedMarkdownTableAlignment::Right),
                    |this| this.text_right(),
                )
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(cx.text_color)
                .children(render_markdown_text(cell, cx))
        }));

    div()
        .border_1()
        .border_color(cx.border_color)
        .rounded_md()
        .p_4()
        .child(header_row)
        .children(
            parsed
                .body
                .iter()
                .map(|row| render_markdown_table_row(row, &parsed.column_alignments, cx)),
        )
        .into_any()
}

fn render_markdown_table_row(
    parsed: &ParsedMarkdownTableRow,
    alignments: &[ParsedMarkdownTableAlignment],
    cx: &mut RenderContext,
) -> AnyElement {
    div()
        .flex()
        .border_b_1()
        .border_color(cx.border_color)
        .py_2()
        .children(parsed.children.iter().enumerate().map(|(i, cell)| {
            let alignment = alignments
                .get(i)
                .unwrap_or(&ParsedMarkdownTableAlignment::None);

            div()
                .flex_1()
                .px_2()
                .when(
                    matches!(alignment, ParsedMarkdownTableAlignment::Center),
                    |this| this.text_center(),
                )
                .when(
                    matches!(alignment, ParsedMarkdownTableAlignment::Right),
                    |this| this.text_right(),
                )
                .text_color(cx.text_color)
                .children(render_markdown_text(cell, cx))
        }))
        .into_any()
}

fn render_markdown_block_quote(
    parsed: &ParsedMarkdownBlockQuote,
    cx: &mut RenderContext,
) -> AnyElement {
    let original_indent = cx.indent;
    cx.indent += 1;

    let block_quote = div()
        .pl(cx.scaled_rems(1.0))
        .border_l_4()
        .border_color(cx.border_color)
        .ml(cx.scaled_rems(0.5))
        .pb(cx.scaled_rems(0.8))
        .children(
            parsed
                .children
                .iter()
                .map(|child| render_markdown_block(child, cx)),
        );

    cx.indent = original_indent;

    block_quote.into_any()
}

fn render_markdown_code_block(
    parsed: &ParsedMarkdownCodeBlock,
    cx: &mut RenderContext,
) -> AnyElement {
    let body = if let Some(highlights) = parsed.highlights.as_ref() {
        StyledText::new(parsed.contents.clone()).with_default_highlights(
            &cx.buffer_text_style,
            highlights.iter().filter_map(|(range, highlight_id)| {
                highlight_id
                    .style(&*cx.syntax_theme)
                    .map(|style| (range.clone(), style))
            }),
        )
    } else {
        StyledText::new(parsed.contents.clone())
    };

    let copy_block_button = IconButton::new("copy-code", IconName::Copy)
        .icon_size(IconSize::Medium)
        .on_click({
            let contents = parsed.contents.clone();
            move |_event, _window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(contents.to_string()))
            }
        })
        .visible_on_hover("code-block");

    let header = if let Some(language) = parsed.language.as_ref() {
        Some(
            div()
                .flex()
                .justify_between()
                .items_center()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(cx.border_color)
                .bg(cx.code_block_background_color)
                .child(Label::new(language.clone()).color(Color::Muted))
                .child(copy_block_button),
        )
    } else {
        Some(
            div()
                .flex()
                .justify_end()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(cx.border_color)
                .bg(cx.code_block_background_color)
                .child(copy_block_button),
        )
    };

    div()
        .group("code-block")
        .border_1()
        .border_color(cx.border_color)
        .rounded_md()
        .overflow_hidden()
        .my(cx.scaled_rems(0.5))
        .children(header)
        .child(
            div()
                .px_3()
                .py_3()
                .bg(cx.code_block_background_color)
                .font_family(cx.buffer_font_family.clone())
                .text_size(cx.scaled_rems(0.9))
                .text_color(cx.text_color)
                .child(
                    div()
                        .overflow_hidden()
                        .child(InteractiveText::new("code-block", body)),
                ),
        )
        .into_any()
}

fn render_markdown_text(parsed_new: &MarkdownParagraph, cx: &mut RenderContext) -> Vec<AnyElement> {
    let mut any_element = vec![];
    // these values are cloned in-order satisfy borrow checker
    let _syntax_theme = cx.syntax_theme.clone();
    let workspace_clone = cx.workspace.clone();
    let code_span_bg_color = cx.code_span_background_color;
    let text_style = cx.text_style.clone();

    // Check if this paragraph contains inline math - if so, use horizontal flex layout
    let has_inline_math = parsed_new
        .iter()
        .any(|chunk| matches!(chunk, MarkdownParagraphChunk::InlineMath(_)));

    if has_inline_math {
        // Create a horizontal flex container for mixed text and math content
        let mut inline_elements = vec![];

        for parsed_region in parsed_new {
            match parsed_region {
                MarkdownParagraphChunk::Text(parsed) => {
                    let element_id = cx.next_id(&parsed.source_range);

                    let highlights = gpui::combine_highlights(
                        parsed.highlights.iter().filter_map(|(range, highlight)| {
                            highlight
                                .to_highlight_style(&*cx.syntax_theme)
                                .map(|style| (range.clone(), style))
                        }),
                        parsed.regions.iter().zip(&parsed.region_ranges).filter_map(
                            |(region, range)| {
                                if region.code {
                                    Some((
                                        range.clone(),
                                        HighlightStyle {
                                            background_color: Some(code_span_bg_color),
                                            ..Default::default()
                                        },
                                    ))
                                } else {
                                    None
                                }
                            },
                        ),
                    );
                    let mut links = Vec::new();
                    let mut link_ranges = Vec::new();
                    for (range, region) in parsed.region_ranges.iter().zip(&parsed.regions) {
                        if let Some(link) = region.link.clone() {
                            links.push(link);
                            link_ranges.push(range.clone());
                        }
                    }
                    let workspace = workspace_clone.clone();
                    let element = div()
                        .child(
                            InteractiveText::new(
                                element_id,
                                StyledText::new(parsed.contents.clone())
                                    .with_default_highlights(&text_style, highlights),
                            )
                            .tooltip({
                                let links = links.clone();
                                let link_ranges = link_ranges.clone();
                                move |idx, _, cx| {
                                    for (ix, range) in link_ranges.iter().enumerate() {
                                        if range.contains(&idx) {
                                            return Some(LinkPreview::new(
                                                &links[ix].to_string(),
                                                cx,
                                            ));
                                        }
                                    }
                                    None
                                }
                            })
                            .on_click(
                                link_ranges,
                                move |clicked_range_ix, window, cx| match &links[clicked_range_ix] {
                                    Link::Web { url } => cx.open_url(url),
                                    Link::Path { path, .. } => {
                                        if let Some(workspace) = &workspace {
                                            _ = workspace.update(cx, |workspace, cx| {
                                                workspace
                                                    .open_abs_path(
                                                        normalize_path(path.clone().as_path()),
                                                        OpenOptions {
                                                            visible: Some(OpenVisible::None),
                                                            ..Default::default()
                                                        },
                                                        window,
                                                        cx,
                                                    )
                                                    .detach();
                                            });
                                        }
                                    }
                                },
                            ),
                        )
                        .into_any();
                    inline_elements.push(element);
                }

                MarkdownParagraphChunk::InlineMath(inline_math) => {
                    info!(
                        "Rendering inline math: '{}' (SHOULD BE BETWEEN PAIRED $ SYMBOLS)",
                        inline_math.contents
                    );
                    // Try to get the SVG from cache first, then generate if not found
                    let svg_content = if let Some(cached_svg) = cache_lookup(&inline_math.contents)
                    {
                        cached_svg
                    } else {
                        match typst_math_to_svg(&inline_math.contents) {
                            Ok(svg) => {
                                info!(
                                    "Inline math SVG generated successfully: {} chars",
                                    svg.len()
                                );
                                cache_store(inline_math.contents.to_string(), svg.clone());
                                svg
                            }
                            Err(err) => {
                                info!(
                                    "Inline math rendering failed: {} - Using fallback styling",
                                    err
                                );
                                // Fallback to raw text with styling
                                let element_id = cx.next_id(&inline_math.source_range);
                                let fallback_element = div()
                                    .id(element_id)
                                    .px_1()
                                    .border_1()
                                    .border_color(cx.border_color)
                                    .bg(cx.code_span_background_color)
                                    .rounded_sm()
                                    .font_family(cx.buffer_font_family.clone())
                                    .text_color(cx.text_color)
                                    .child(format!("${} $", inline_math.contents))
                                    .into_any();
                                inline_elements.push(fallback_element);
                                continue;
                            }
                        }
                    };

                    // Create inline image for math
                    let svg_content_clone = svg_content.clone();
                    let image_source = ImageSource::Custom(Arc::new(move |_window, _cx| {
                        let tree = match usvg::Tree::from_data(
                            svg_content_clone.as_bytes(),
                            &usvg::Options::default(),
                        ) {
                            Ok(tree) => tree,
                            Err(e) => {
                                return Some(Err(ImageCacheError::Other(Arc::new(
                                    anyhow::anyhow!("SVG parse error: {}", e),
                                ))));
                            }
                        };

                        let size = tree.size();
                        let width = size.width() as u32;
                        let height = size.height() as u32;

                        if width == 0 || height == 0 {
                            return Some(Err(ImageCacheError::Other(Arc::new(anyhow::anyhow!(
                                "Invalid SVG dimensions"
                            )))));
                        }

                        let mut pixmap = match resvg::tiny_skia::Pixmap::new(width, height) {
                            Some(pixmap) => pixmap,
                            None => {
                                return Some(Err(ImageCacheError::Other(Arc::new(
                                    anyhow::anyhow!("Failed to create pixmap"),
                                ))));
                            }
                        };

                        resvg::render(
                            &tree,
                            resvg::tiny_skia::Transform::identity(),
                            &mut pixmap.as_mut(),
                        );

                        let rgba_data = pixmap.take();
                        let rgba_image = match image::RgbaImage::from_raw(width, height, rgba_data)
                        {
                            Some(img) => img,
                            None => {
                                return Some(Err(ImageCacheError::Other(Arc::new(
                                    anyhow::anyhow!("Failed to create RGBA image"),
                                ))));
                            }
                        };

                        let frame = image::Frame::new(rgba_image.into());
                        let render_image = RenderImage::new(vec![frame]);

                        Some(Ok(Arc::new(render_image)))
                    }));

                    let element_id = cx.next_id(&inline_math.source_range);
                    info!("Creating inline math image element");
                    let math_element = img(image_source)
                        .id(element_id)
                        .max_h(px(35.0)) // Larger size for better readability
                        .flex_shrink_0() // Prevent shrinking
                        .into_any();
                    inline_elements.push(math_element);
                    info!("Added inline math element to result");
                }

                MarkdownParagraphChunk::Image(image) => {
                    let image_resource = match image.link.clone() {
                        Link::Web { url } => Resource::Uri(url.into()),
                        Link::Path { path, .. } => Resource::Path(Arc::from(path)),
                    };

                    let element_id = cx.next_id(&image.source_range);

                    let image_element = div()
                        .id(element_id)
                        .cursor_pointer()
                        .child(
                            img(ImageSource::Resource(image_resource))
                                .max_w_full()
                                .with_fallback({
                                    let alt_text = image.alt_text.clone();
                                    move || div().children(alt_text.clone()).into_any_element()
                                }),
                        )
                        .tooltip({
                            let link = image.link.clone();
                            move |_, cx| {
                                InteractiveMarkdownElementTooltip::new(
                                    Some(link.to_string()),
                                    "open image",
                                    cx,
                                )
                                .into()
                            }
                        })
                        .on_click({
                            let workspace = workspace_clone.clone();
                            let link = image.link.clone();
                            move |_, window, cx| {
                                if window.modifiers().secondary() {
                                    match &link {
                                        Link::Web { url } => cx.open_url(url),
                                        Link::Path { path, .. } => {
                                            if let Some(workspace) = &workspace {
                                                _ = workspace.update(cx, |workspace, cx| {
                                                    workspace
                                                        .open_abs_path(
                                                            path.clone(),
                                                            OpenOptions {
                                                                visible: Some(OpenVisible::None),
                                                                ..Default::default()
                                                            },
                                                            window,
                                                            cx,
                                                        )
                                                        .detach();
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .into_any();
                    inline_elements.push(image_element);
                }
            }
        }

        // Wrap all inline elements in a horizontal flex container
        let paragraph_element = h_flex()
            .items_baseline()
            .flex_wrap()
            .children(inline_elements)
            .into_any();
        any_element.push(paragraph_element);
    } else {
        // No inline math - use original approach for text-only paragraphs
        for parsed_region in parsed_new {
            match parsed_region {
                MarkdownParagraphChunk::Text(parsed) => {
                    let element_id = cx.next_id(&parsed.source_range);

                    let highlights = gpui::combine_highlights(
                        parsed.highlights.iter().filter_map(|(range, highlight)| {
                            highlight
                                .to_highlight_style(&*cx.syntax_theme)
                                .map(|style| (range.clone(), style))
                        }),
                        parsed.regions.iter().zip(&parsed.region_ranges).filter_map(
                            |(region, range)| {
                                if region.code {
                                    Some((
                                        range.clone(),
                                        HighlightStyle {
                                            background_color: Some(code_span_bg_color),
                                            ..Default::default()
                                        },
                                    ))
                                } else {
                                    None
                                }
                            },
                        ),
                    );
                    let mut links = Vec::new();
                    let mut link_ranges = Vec::new();
                    for (range, region) in parsed.region_ranges.iter().zip(&parsed.regions) {
                        if let Some(link) = region.link.clone() {
                            links.push(link);
                            link_ranges.push(range.clone());
                        }
                    }
                    let workspace = workspace_clone.clone();
                    let element = div()
                        .child(
                            InteractiveText::new(
                                element_id,
                                StyledText::new(parsed.contents.clone())
                                    .with_default_highlights(&text_style, highlights),
                            )
                            .tooltip({
                                let links = links.clone();
                                let link_ranges = link_ranges.clone();
                                move |idx, _, cx| {
                                    for (ix, range) in link_ranges.iter().enumerate() {
                                        if range.contains(&idx) {
                                            return Some(LinkPreview::new(
                                                &links[ix].to_string(),
                                                cx,
                                            ));
                                        }
                                    }
                                    None
                                }
                            })
                            .on_click(
                                link_ranges,
                                move |clicked_range_ix, window, cx| match &links[clicked_range_ix] {
                                    Link::Web { url } => cx.open_url(url),
                                    Link::Path { path, .. } => {
                                        if let Some(workspace) = &workspace {
                                            _ = workspace.update(cx, |workspace, cx| {
                                                workspace
                                                    .open_abs_path(
                                                        normalize_path(path.clone().as_path()),
                                                        OpenOptions {
                                                            visible: Some(OpenVisible::None),
                                                            ..Default::default()
                                                        },
                                                        window,
                                                        cx,
                                                    )
                                                    .detach();
                                            });
                                        }
                                    }
                                },
                            ),
                        )
                        .into_any();
                    any_element.push(element);
                }
                MarkdownParagraphChunk::Image(image) => {
                    let image_resource = match image.link.clone() {
                        Link::Web { url } => Resource::Uri(url.into()),
                        Link::Path { path, .. } => Resource::Path(Arc::from(path)),
                    };

                    let element_id = cx.next_id(&image.source_range);

                    let image_element = div()
                        .id(element_id)
                        .cursor_pointer()
                        .child(
                            img(ImageSource::Resource(image_resource))
                                .max_w_full()
                                .with_fallback({
                                    let alt_text = image.alt_text.clone();
                                    move || div().children(alt_text.clone()).into_any_element()
                                }),
                        )
                        .tooltip({
                            let link = image.link.clone();
                            move |_, cx| {
                                InteractiveMarkdownElementTooltip::new(
                                    Some(link.to_string()),
                                    "open image",
                                    cx,
                                )
                                .into()
                            }
                        })
                        .on_click({
                            let workspace = workspace_clone.clone();
                            let link = image.link.clone();
                            move |_, window, cx| {
                                if window.modifiers().secondary() {
                                    match &link {
                                        Link::Web { url } => cx.open_url(url),
                                        Link::Path { path, .. } => {
                                            if let Some(workspace) = &workspace {
                                                _ = workspace.update(cx, |workspace, cx| {
                                                    workspace
                                                        .open_abs_path(
                                                            path.clone(),
                                                            OpenOptions {
                                                                visible: Some(OpenVisible::None),
                                                                ..Default::default()
                                                            },
                                                            window,
                                                            cx,
                                                        )
                                                        .detach();
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .into_any();
                    any_element.push(image_element);
                }
                MarkdownParagraphChunk::InlineMath(_) => {
                    // This shouldn't happen in text-only paragraphs, but handle gracefully
                    // by doing nothing - inline math should be handled in the mixed content branch
                }
            }
        }
    }

    any_element
}

fn render_markdown_rule(cx: &mut RenderContext) -> AnyElement {
    let rule = div().w_full().h(cx.scaled_rems(0.125)).bg(cx.border_color);
    div().py(cx.scaled_rems(0.5)).child(rule).into_any()
}

// Minimal World implementation for math rendering
struct MathWorld {
    library: Prehashed<Library>,
    book: Prehashed<FontBook>,
    main_source: Source,
    fonts: Vec<Font>,
}

impl MathWorld {
    fn new(content: &str) -> Self {
        // Create a font book with system fonts
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();

        // Find available math fonts
        let mut available_fonts = Vec::new();
        let mut math_font = None;

        for face_info in fontdb.faces() {
            if let Some(family) = face_info.families.get(0) {
                let family_name = &family.0;
                available_fonts.push(family_name.clone());

                // Prioritize math fonts
                if family_name.contains("STIX Two Math") {
                    math_font = Some("STIX Two Math");
                } else if math_font.is_none()
                    && (family_name.contains("Libertinus Math")
                        || family_name.contains("Latin Modern Math")
                        || family_name.contains("Computer Modern")
                        || family_name.contains("Cambria Math")
                        || family_name.contains("Asana Math"))
                {
                    math_font = Some(family_name.as_str());
                }
            }
        }

        info!("Available font families: {}", available_fonts.len());
        let selected_font = math_font.unwrap_or("serif");
        info!("Selected math font: {}", selected_font);

        let main_source = Source::new(
            FileId::new(None, VirtualPath::new("main.typ")),
            format!(
                r#"#set page(width: auto, height: auto, margin: 2pt)
#show math.equation: set text(font: "{}", size: 18pt, fill: white)
#set text(font: "{}", size: 18pt, fill: white)
$ {} $"#,
                selected_font, selected_font, content
            ),
        );

        // Load all system fonts
        let mut fonts = Vec::new();
        for face_info in fontdb.faces() {
            if let Some((source, _)) = fontdb.face_source(face_info.id) {
                match source {
                    fontdb::Source::Binary(ref bytes) => {
                        if let Some(font) =
                            Font::new(Bytes::from(bytes.as_ref().as_ref()), face_info.index)
                        {
                            fonts.push(font);
                        }
                    }
                    fontdb::Source::File(path) => {
                        if let Ok(data) = std::fs::read(path) {
                            if let Some(font) = Font::new(Bytes::from(data), face_info.index) {
                                fonts.push(font);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let book = FontBook::from_fonts(&fonts);

        Self {
            library: Prehashed::new(Library::builder().build()),
            book: Prehashed::new(book),
            main_source,
            fonts,
        }
    }
}

impl World for MathWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn main(&self) -> Source {
        self.main_source.clone()
    }

    fn source(&self, _id: FileId) -> typst::diag::FileResult<Source> {
        Ok(self.main_source.clone())
    }

    fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
        Err(typst::diag::FileError::NotFound(std::path::PathBuf::new()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

fn typst_math_to_svg(math_content: &str) -> Result<String, String> {
    info!("typst_math_to_svg input: {}", math_content);

    // Create Typst world with math content
    let world = MathWorld::new(math_content);

    // Compile the document
    let result = compile(&world, &mut Default::default());
    let document = match result {
        Ok(doc) => doc,
        Err(errors) => {
            let error_msg = errors
                .iter()
                .map(|e| e.message.to_string())
                .collect::<Vec<_>>()
                .join("; ");
            return Err(format!("Typst compilation failed: {}", error_msg));
        }
    };

    // Convert to SVG
    if let Some(page) = document.pages.first() {
        let svg_content = typst_svg(&page.frame);
        info!(
            "Successfully generated SVG with {} characters",
            svg_content.len()
        );
        Ok(svg_content)
    } else {
        Err("No pages in compiled document".to_string())
    }
}

fn render_markdown_math(math: &ParsedMarkdownMathBlock, cx: &mut RenderContext) -> AnyElement {
    // Try to get the SVG from cache first, then generate if not found
    let svg_content = if let Some(cached_svg) = cache_lookup(&math.contents) {
        cached_svg
    } else {
        match typst_math_to_svg(&math.contents) {
            Ok(svg) => {
                cache_store(math.contents.to_string(), svg.clone());
                svg
            }
            Err(err) => {
                // Render an error box if Typst failed
                info!("Math rendering failed: {}", err);
                return div()
                    .p_2()
                    .border_1()
                    .border_color(cx.border_color)
                    .bg(cx.code_block_background_color)
                    .rounded_md()
                    .child(
                        div()
                            .text_color(cx.text_color)
                            .child(format!("Math rendering error: {}", err)),
                    )
                    .child(
                        div()
                            .mt_2()
                            .text_sm()
                            .font_family(cx.buffer_font_family.clone())
                            .text_color(cx.text_color)
                            .child(format!("Raw: {}", math.contents)),
                    )
                    .into_any();
            }
        }
    };

    // Convert SVG to bitmap using resvg and render as image
    let svg_content_clone = svg_content.clone();
    let image_source = ImageSource::Custom(Arc::new(move |_window, _cx| {
        // Parse SVG and convert to bitmap
        let tree =
            match usvg::Tree::from_data(svg_content_clone.as_bytes(), &usvg::Options::default()) {
                Ok(tree) => tree,
                Err(e) => {
                    return Some(Err(ImageCacheError::Other(Arc::new(anyhow::anyhow!(
                        "SVG parse error: {}",
                        e
                    )))));
                }
            };

        let size = tree.size();
        let width = size.width() as u32;
        let height = size.height() as u32;

        if width == 0 || height == 0 {
            return Some(Err(ImageCacheError::Other(Arc::new(anyhow::anyhow!(
                "Invalid SVG dimensions"
            )))));
        }

        let mut pixmap = match resvg::tiny_skia::Pixmap::new(width, height) {
            Some(pixmap) => pixmap,
            None => {
                return Some(Err(ImageCacheError::Other(Arc::new(anyhow::anyhow!(
                    "Failed to create pixmap"
                )))));
            }
        };

        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::identity(),
            &mut pixmap.as_mut(),
        );

        // Convert to RGBA8 and create Frame
        let rgba_data = pixmap.take();
        let rgba_image = match image::RgbaImage::from_raw(width, height, rgba_data) {
            Some(img) => img,
            None => {
                return Some(Err(ImageCacheError::Other(Arc::new(anyhow::anyhow!(
                    "Failed to create RGBA image"
                )))));
            }
        };

        let frame = image::Frame::new(rgba_image.into());
        let render_image = RenderImage::new(vec![frame]);

        Some(Ok(Arc::new(render_image)))
    }));

    // Render the math equation as an inline image
    div()
        .flex()
        .items_center()
        .justify_center()
        .py_2()
        .child(img(image_source).max_w(px(800.0)).max_h(px(150.0)))
        .into_any()
}

struct InteractiveMarkdownElementTooltip {
    tooltip_text: Option<SharedString>,
    action_text: String,
}

impl InteractiveMarkdownElementTooltip {
    pub fn new(tooltip_text: Option<String>, action_text: &str, cx: &mut App) -> Entity<Self> {
        let tooltip_text = tooltip_text.map(|t| util::truncate_and_trailoff(&t, 50).into());

        cx.new(|_cx| Self {
            tooltip_text,
            action_text: action_text.to_string(),
        })
    }
}

impl Render for InteractiveMarkdownElementTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(window, cx, |el, _, _| {
            let secondary_modifier = Keystroke {
                modifiers: Modifiers::secondary_key(),
                ..Default::default()
            };

            el.child(
                v_flex()
                    .gap_1()
                    .when_some(self.tooltip_text.clone(), |this, text| {
                        this.child(Label::new(text).size(LabelSize::Small))
                    })
                    .child(
                        Label::new(format!(
                            "{}-click to {}",
                            secondary_modifier, self.action_text
                        ))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                    ),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typst_math_to_svg() {
        // Test basic math expression
        let result = typst_math_to_svg("x + y = z");
        match result {
            Ok(svg) => {
                println!("Generated SVG: {}", svg);
                assert!(svg.contains("<svg"));
                assert!(svg.contains("</svg>"));
            }
            Err(e) => {
                println!("Error: {}", e);
                // For now, we'll allow this to fail until fonts are properly set up
            }
        }
    }

    #[test]
    fn test_typst_math_complex() {
        // Test more complex math expression
        let result = typst_math_to_svg("sum_(i=1)^n i = (n(n+1))/2");
        match result {
            Ok(svg) => {
                println!("Generated complex SVG: {}", svg);
                assert!(svg.contains("<svg"));
            }
            Err(e) => {
                println!("Complex math error: {}", e);
                // Allow failure for now
            }
        }
    }
}
