use crate::markdown_elements::{
    HeadingLevel, Image, Link, MarkdownParagraph, MarkdownParagraphChunk, ParsedMarkdown,
    ParsedMarkdownBlockQuote, ParsedMarkdownCodeBlock, ParsedMarkdownElement,
    ParsedMarkdownHeading, ParsedMarkdownListItem, ParsedMarkdownListItemType, ParsedMarkdownTable,
    ParsedMarkdownTableAlignment, ParsedMarkdownTableRow, ParsedMarkdownText,
};
use gpui::{
    div, img, px, rems, AbsoluteLength, AnyElement, ClipboardItem, DefiniteLength, Div, Element,
    ElementId, HighlightStyle, Hsla, ImageSource, InteractiveText, IntoElement, Keystroke, Length,
    Modifiers, ParentElement, Resource, SharedString, Styled, StyledText, TextStyle, WeakView,
    WindowContext,
};
use settings::Settings;
use std::{
    ops::{Mul, Range},
    path::Path,
    sync::Arc,
    vec,
};
use theme::{ActiveTheme, SyntaxTheme, ThemeSettings};
use ui::{
    h_flex, relative, v_flex, Checkbox, Clickable, FluentBuilder, IconButton, IconName, IconSize,
    InteractiveElement, LinkPreview, Selection, StatefulInteractiveElement, StyledExt, StyledImage,
    Tooltip, VisibleOnHover,
};
use workspace::Workspace;

type CheckboxClickedCallback = Arc<Box<dyn Fn(bool, Range<usize>, &mut WindowContext)>>;

#[derive(Clone)]
pub struct RenderContext {
    workspace: Option<WeakView<Workspace>>,
    next_id: usize,
    buffer_font_family: SharedString,
    buffer_text_style: TextStyle,
    text_style: TextStyle,
    border_color: Hsla,
    text_color: Hsla,
    text_muted_color: Hsla,
    code_block_background_color: Hsla,
    code_span_background_color: Hsla,
    syntax_theme: Arc<SyntaxTheme>,
    indent: usize,
    checkbox_clicked_callback: Option<CheckboxClickedCallback>,
}

impl RenderContext {
    pub fn new(workspace: Option<WeakView<Workspace>>, cx: &WindowContext) -> RenderContext {
        let theme = cx.theme().clone();

        let settings = ThemeSettings::get_global(cx);
        let buffer_font_family = settings.buffer_font.family.clone();
        let mut buffer_text_style = cx.text_style();
        buffer_text_style.font_family = buffer_font_family.clone();

        RenderContext {
            workspace,
            next_id: 0,
            indent: 0,
            buffer_font_family,
            buffer_text_style,
            text_style: cx.text_style(),
            syntax_theme: theme.syntax().clone(),
            border_color: theme.colors().border,
            text_color: theme.colors().text,
            text_muted_color: theme.colors().text_muted,
            code_block_background_color: theme.colors().surface_background,
            code_span_background_color: theme.colors().editor_document_highlight_read_background,
            checkbox_clicked_callback: None,
        }
    }

    pub fn with_checkbox_clicked_callback(
        mut self,
        callback: impl Fn(bool, Range<usize>, &mut WindowContext) + 'static,
    ) -> Self {
        self.checkbox_clicked_callback = Some(Arc::new(Box::new(callback)));
        self
    }

    fn next_id(&mut self, span: &Range<usize>) -> ElementId {
        let id = format!("markdown-{}-{}-{}", self.next_id, span.start, span.end);
        self.next_id += 1;
        ElementId::from(SharedString::from(id))
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
        if self.indent > 0 {
            element.pb_3()
        } else {
            element
        }
    }
}

pub fn render_parsed_markdown(
    parsed: &ParsedMarkdown,
    workspace: Option<WeakView<Workspace>>,
    cx: &WindowContext,
) -> Vec<AnyElement> {
    let mut cx = RenderContext::new(workspace, cx);
    let mut elements = Vec::new();

    for child in &parsed.children {
        elements.push(render_markdown_block(child, &mut cx));
    }

    elements
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
    }
}

fn render_markdown_heading(parsed: &ParsedMarkdownHeading, cx: &mut RenderContext) -> AnyElement {
    let size = match parsed.level {
        HeadingLevel::H1 => rems(2.),
        HeadingLevel::H2 => rems(1.5),
        HeadingLevel::H3 => rems(1.25),
        HeadingLevel::H4 => rems(1.),
        HeadingLevel::H5 => rems(0.875),
        HeadingLevel::H6 => rems(0.85),
    };

    let color = match parsed.level {
        HeadingLevel::H6 => cx.text_muted_color,
        _ => cx.text_color,
    };

    let line_height = DefiniteLength::from(size.mul(1.25));

    div()
        .line_height(line_height)
        .text_size(size)
        .text_color(color)
        .pt(rems(0.15))
        .pb_1()
        .children(render_markdown_text(&parsed.contents, cx))
        .whitespace_normal()
        .into_any()
}

fn render_markdown_list_item(
    parsed: &ParsedMarkdownListItem,
    cx: &mut RenderContext,
) -> AnyElement {
    use ParsedMarkdownListItemType::*;

    let padding = rems((parsed.depth - 1) as f32);

    let bullet = match &parsed.item_type {
        Ordered(order) => format!("{}.", order).into_any_element(),
        Unordered => "•".into_any_element(),
        Task(checked, range) => div()
            .id(cx.next_id(range))
            .mt(px(3.))
            .child(
                Checkbox::new(
                    "checkbox",
                    if *checked {
                        Selection::Selected
                    } else {
                        Selection::Unselected
                    },
                )
                .when_some(
                    cx.checkbox_clicked_callback.clone(),
                    |this, callback| {
                        this.on_click({
                            let range = range.clone();
                            move |selection, cx| {
                                let checked = match selection {
                                    Selection::Selected => true,
                                    Selection::Unselected => false,
                                    _ => return,
                                };

                                if cx.modifiers().secondary() {
                                    callback(checked, range.clone(), cx);
                                }
                            }
                        })
                    },
                ),
            )
            .hover(|s| s.cursor_pointer())
            .tooltip(|cx| {
                let secondary_modifier = Keystroke {
                    key: "".to_string(),
                    modifiers: Modifiers::secondary_key(),
                    key_char: None,
                };
                Tooltip::text(
                    format!("{}-click to toggle the checkbox", secondary_modifier),
                    cx,
                )
            })
            .into_any_element(),
    };
    let bullet = div().mr_2().child(bullet);

    let contents: Vec<AnyElement> = parsed
        .content
        .iter()
        .map(|c| render_markdown_block(c, cx))
        .collect();

    let item = h_flex()
        .pl(DefiniteLength::Absolute(AbsoluteLength::Rems(padding)))
        .items_start()
        .children(vec![bullet, div().children(contents).pr_4().w_full()]);

    cx.with_common_p(item).into_any()
}

fn paragraph_len(paragraphs: &MarkdownParagraph) -> usize {
    paragraphs
        .iter()
        .map(|paragraph| match paragraph {
            MarkdownParagraphChunk::Text(text) => text.contents.len(),
            // TODO: Scale column width based on image size
            MarkdownParagraphChunk::Image(_) => 1,
        })
        .sum()
}

fn render_markdown_table(parsed: &ParsedMarkdownTable, cx: &mut RenderContext) -> AnyElement {
    let mut max_lengths: Vec<usize> = vec![0; parsed.header.children.len()];

    for (index, cell) in parsed.header.children.iter().enumerate() {
        let length = paragraph_len(&cell);
        max_lengths[index] = length;
    }

    for row in &parsed.body {
        for (index, cell) in row.children.iter().enumerate() {
            let length = paragraph_len(&cell);

            if length > max_lengths[index] {
                max_lengths[index] = length;
            }
        }
    }

    let total_max_length: usize = max_lengths.iter().sum();
    let max_column_widths: Vec<f32> = max_lengths
        .iter()
        .map(|&length| length as f32 / total_max_length as f32)
        .collect();

    let header = render_markdown_table_row(
        &parsed.header,
        &parsed.column_alignments,
        &max_column_widths,
        true,
        cx,
    );

    let body: Vec<AnyElement> = parsed
        .body
        .iter()
        .map(|row| {
            render_markdown_table_row(
                row,
                &parsed.column_alignments,
                &max_column_widths,
                false,
                cx,
            )
        })
        .collect();

    cx.with_common_p(v_flex())
        .w_full()
        .child(header)
        .children(body)
        .into_any()
}

fn render_markdown_table_row(
    parsed: &ParsedMarkdownTableRow,
    alignments: &Vec<ParsedMarkdownTableAlignment>,
    max_column_widths: &Vec<f32>,
    is_header: bool,
    cx: &mut RenderContext,
) -> AnyElement {
    let mut items = vec![];

    for (index, cell) in parsed.children.iter().enumerate() {
        let alignment = alignments
            .get(index)
            .copied()
            .unwrap_or(ParsedMarkdownTableAlignment::None);

        let contents = render_markdown_text(cell, cx);

        let container = match alignment {
            ParsedMarkdownTableAlignment::Left | ParsedMarkdownTableAlignment::None => div(),
            ParsedMarkdownTableAlignment::Center => v_flex().items_center(),
            ParsedMarkdownTableAlignment::Right => v_flex().items_end(),
        };

        let max_width = max_column_widths.get(index).unwrap_or(&0.0);
        let mut cell = container
            .w(Length::Definite(relative(*max_width)))
            .h_full()
            .children(contents)
            .px_2()
            .py_1()
            .border_color(cx.border_color);

        if is_header {
            cell = cell.border_2()
        } else {
            cell = cell.border_1()
        }

        items.push(cell);
    }

    h_flex().children(items).into_any_element()
}

fn render_markdown_block_quote(
    parsed: &ParsedMarkdownBlockQuote,
    cx: &mut RenderContext,
) -> AnyElement {
    cx.indent += 1;

    let children: Vec<AnyElement> = parsed
        .children
        .iter()
        .map(|child| render_markdown_block(child, cx))
        .collect();

    cx.indent -= 1;

    cx.with_common_p(div())
        .child(
            div()
                .border_l_4()
                .border_color(cx.border_color)
                .pl_3()
                .children(children),
        )
        .into_any()
}

fn render_markdown_code_block(
    parsed: &ParsedMarkdownCodeBlock,
    cx: &mut RenderContext,
) -> AnyElement {
    let body = if let Some(highlights) = parsed.highlights.as_ref() {
        StyledText::new(parsed.contents.clone()).with_highlights(
            &cx.buffer_text_style,
            highlights.iter().filter_map(|(range, highlight_id)| {
                highlight_id
                    .style(cx.syntax_theme.as_ref())
                    .map(|style| (range.clone(), style))
            }),
        )
    } else {
        StyledText::new(parsed.contents.clone())
    };

    let copy_block_button = IconButton::new("copy-code", IconName::Copy)
        .icon_size(IconSize::Small)
        .on_click({
            let contents = parsed.contents.clone();
            move |_, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(contents.to_string()));
            }
        })
        .visible_on_hover("markdown-block");

    cx.with_common_p(div())
        .font_family(cx.buffer_font_family.clone())
        .px_3()
        .py_3()
        .bg(cx.code_block_background_color)
        .rounded_md()
        .child(body)
        .child(
            div()
                .h_flex()
                .absolute()
                .right_1()
                .top_1()
                .child(copy_block_button),
        )
        .into_any()
}

fn render_markdown_paragraph(parsed: &MarkdownParagraph, cx: &mut RenderContext) -> AnyElement {
    cx.with_common_p(div())
        .children(render_markdown_text(parsed, cx))
        .flex()
        .into_any_element()
}

fn render_markdown_text(parsed_new: &MarkdownParagraph, cx: &mut RenderContext) -> Vec<AnyElement> {
    let mut any_element = vec![];
    // these values are cloned in-order satisfy borrow checker
    let syntax_theme = cx.syntax_theme.clone();
    let workspace_clone = cx.workspace.clone();
    let code_span_bg_color = cx.code_span_background_color;
    let text_style = cx.text_style.clone();

    for parsed_region in parsed_new {
        match parsed_region {
            MarkdownParagraphChunk::Text(parsed) => {
                let element_id = cx.next_id(&parsed.source_range);

                let highlights = gpui::combine_highlights(
                    parsed.highlights.iter().filter_map(|(range, highlight)| {
                        highlight
                            .to_highlight_style(&syntax_theme)
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
                                .with_highlights(&text_style, highlights),
                        )
                        .tooltip({
                            let links = links.clone();
                            let link_ranges = link_ranges.clone();
                            move |idx, cx| {
                                for (ix, range) in link_ranges.iter().enumerate() {
                                    if range.contains(&idx) {
                                        return Some(LinkPreview::new(&links[ix].to_string(), cx));
                                    }
                                }
                                None
                            }
                        })
                        .on_click(
                            link_ranges,
                            move |clicked_range_ix, window_cx| match &links[clicked_range_ix] {
                                Link::Web { url } => window_cx.open_url(url),
                                Link::Path { path, .. } => {
                                    if let Some(workspace) = &workspace {
                                        _ = workspace.update(window_cx, |workspace, cx| {
                                            workspace
                                                .open_abs_path(path.clone(), false, cx)
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
                let (link, source_range, image_source, alt_text) = match image {
                    Image::Web {
                        link,
                        source_range,
                        url,
                        alt_text,
                    } => (
                        link,
                        source_range,
                        Resource::Uri(url.clone().into()),
                        alt_text,
                    ),
                    Image::Path {
                        link,
                        source_range,
                        path,
                        alt_text,
                        ..
                    } => {
                        let image_path = Path::new(path.to_str().unwrap());
                        (
                            link,
                            source_range,
                            Resource::Path(Arc::from(image_path)),
                            alt_text,
                        )
                    }
                };

                let element_id = cx.next_id(source_range);

                match link {
                    None => {
                        let fallback_workspace = workspace_clone.clone();
                        let fallback_syntax_theme = syntax_theme.clone();
                        let fallback_text_style = text_style.clone();
                        let fallback_alt_text = alt_text.clone();
                        let element_id_new = element_id.clone();
                        let element = div()
                            .child(img(ImageSource::Resource(image_source)).with_fallback({
                                move || {
                                    fallback_text(
                                        fallback_alt_text.clone().unwrap(),
                                        element_id.clone(),
                                        &fallback_syntax_theme,
                                        code_span_bg_color,
                                        fallback_workspace.clone(),
                                        &fallback_text_style,
                                    )
                                }
                            }))
                            .id(element_id_new)
                            .into_any();
                        any_element.push(element);
                    }
                    Some(link) => {
                        let link_click = link.clone();
                        let link_tooltip = link.clone();
                        let fallback_workspace = workspace_clone.clone();
                        let fallback_syntax_theme = syntax_theme.clone();
                        let fallback_text_style = text_style.clone();
                        let fallback_alt_text = alt_text.clone();
                        let element_id_new = element_id.clone();
                        let image_element = div()
                            .child(img(ImageSource::Resource(image_source)).with_fallback({
                                move || {
                                    fallback_text(
                                        fallback_alt_text.clone().unwrap(),
                                        element_id.clone(),
                                        &fallback_syntax_theme,
                                        code_span_bg_color,
                                        fallback_workspace.clone(),
                                        &fallback_text_style,
                                    )
                                }
                            }))
                            .id(element_id_new)
                            .tooltip(move |cx| LinkPreview::new(&link_tooltip.to_string(), cx))
                            .on_click({
                                let workspace = workspace_clone.clone();
                                move |_event, window_cx| match &link_click {
                                    Link::Web { url } => window_cx.open_url(url),
                                    Link::Path { path, .. } => {
                                        if let Some(workspace) = &workspace {
                                            _ = workspace.update(window_cx, |workspace, cx| {
                                                workspace
                                                    .open_abs_path(path.clone(), false, cx)
                                                    .detach();
                                            });
                                        }
                                    }
                                }
                            })
                            .into_any();
                        any_element.push(image_element);
                    }
                }
            }
        }
    }

    any_element
}

fn render_markdown_rule(cx: &mut RenderContext) -> AnyElement {
    let rule = div().w_full().h(px(2.)).bg(cx.border_color);
    div().pt_3().pb_3().child(rule).into_any()
}

fn fallback_text(
    parsed: ParsedMarkdownText,
    source_range: ElementId,
    syntax_theme: &theme::SyntaxTheme,
    code_span_bg_color: Hsla,
    workspace: Option<WeakView<Workspace>>,
    text_style: &TextStyle,
) -> AnyElement {
    let element_id = source_range;

    let highlights = gpui::combine_highlights(
        parsed.highlights.iter().filter_map(|(range, highlight)| {
            let highlight = highlight.to_highlight_style(syntax_theme)?;
            Some((range.clone(), highlight))
        }),
        parsed
            .regions
            .iter()
            .zip(&parsed.region_ranges)
            .filter_map(|(region, range)| {
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
            }),
    );
    let mut links = Vec::new();
    let mut link_ranges = Vec::new();
    for (range, region) in parsed.region_ranges.iter().zip(&parsed.regions) {
        if let Some(link) = region.link.clone() {
            links.push(link);
            link_ranges.push(range.clone());
        }
    }
    let element = div()
        .child(
            InteractiveText::new(
                element_id,
                StyledText::new(parsed.contents.clone()).with_highlights(text_style, highlights),
            )
            .tooltip({
                let links = links.clone();
                let link_ranges = link_ranges.clone();
                move |idx, cx| {
                    for (ix, range) in link_ranges.iter().enumerate() {
                        if range.contains(&idx) {
                            return Some(LinkPreview::new(&links[ix].to_string(), cx));
                        }
                    }
                    None
                }
            })
            .on_click(
                link_ranges,
                move |clicked_range_ix, window_cx| match &links[clicked_range_ix] {
                    Link::Web { url } => window_cx.open_url(url),
                    Link::Path { path, .. } => {
                        if let Some(workspace) = &workspace {
                            _ = workspace.update(window_cx, |workspace, cx| {
                                workspace.open_abs_path(path.clone(), false, cx).detach();
                            });
                        }
                    }
                },
            ),
        )
        .into_any();
    return element;
}
