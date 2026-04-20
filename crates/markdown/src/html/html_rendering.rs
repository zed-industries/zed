use std::ops::Range;

use gpui::{App, FontStyle, FontWeight, StrikethroughStyle, TextStyleRefinement, UnderlineStyle};
use pulldown_cmark::Alignment;
use ui::prelude::*;

use crate::html::html_parser::{
    HtmlHighlightStyle, HtmlImage, HtmlParagraph, HtmlParagraphChunk, ParsedHtmlBlock,
    ParsedHtmlElement, ParsedHtmlList, ParsedHtmlListItemType, ParsedHtmlTable, ParsedHtmlTableRow,
    ParsedHtmlText,
};
use crate::{MarkdownElement, MarkdownElementBuilder};

pub(crate) struct HtmlSourceAllocator {
    source_range: Range<usize>,
    next_source_index: usize,
}

impl HtmlSourceAllocator {
    pub(crate) fn new(source_range: Range<usize>) -> Self {
        Self {
            next_source_index: source_range.start,
            source_range,
        }
    }

    pub(crate) fn allocate(&mut self, requested_len: usize) -> Range<usize> {
        let remaining = self.source_range.end.saturating_sub(self.next_source_index);
        let len = requested_len.min(remaining);
        let start = self.next_source_index;
        let end = start + len;
        self.next_source_index = end;
        start..end
    }
}

impl MarkdownElement {
    pub(crate) fn render_html_block(
        &self,
        block: &ParsedHtmlBlock,
        builder: &mut MarkdownElementBuilder,
        markdown_end: usize,
        cx: &mut App,
    ) {
        let mut source_allocator = HtmlSourceAllocator::new(block.source_range.clone());
        self.render_html_elements(
            &block.children,
            &mut source_allocator,
            builder,
            markdown_end,
            cx,
        );
    }

    fn render_html_elements(
        &self,
        elements: &[ParsedHtmlElement],
        source_allocator: &mut HtmlSourceAllocator,
        builder: &mut MarkdownElementBuilder,
        markdown_end: usize,
        cx: &mut App,
    ) {
        for element in elements {
            self.render_html_element(element, source_allocator, builder, markdown_end, cx);
        }
    }

    fn render_html_element(
        &self,
        element: &ParsedHtmlElement,
        source_allocator: &mut HtmlSourceAllocator,
        builder: &mut MarkdownElementBuilder,
        markdown_end: usize,
        cx: &mut App,
    ) {
        let Some(source_range) = element.source_range() else {
            return;
        };

        match element {
            ParsedHtmlElement::Paragraph(paragraph) => {
                self.push_markdown_paragraph(
                    builder,
                    &source_range,
                    markdown_end,
                    paragraph.text_align,
                );
                self.render_html_paragraph(
                    &paragraph.contents,
                    source_allocator,
                    builder,
                    cx,
                    markdown_end,
                );
                self.pop_markdown_paragraph(builder);
            }
            ParsedHtmlElement::Heading(heading) => {
                self.push_markdown_heading(
                    builder,
                    heading.level,
                    &heading.source_range,
                    markdown_end,
                    heading.text_align,
                );
                self.render_html_paragraph(
                    &heading.contents,
                    source_allocator,
                    builder,
                    cx,
                    markdown_end,
                );
                self.pop_markdown_heading(builder);
            }
            ParsedHtmlElement::List(list) => {
                self.render_html_list(list, source_allocator, builder, markdown_end, cx);
            }
            ParsedHtmlElement::BlockQuote(block_quote) => {
                self.push_markdown_block_quote(builder, &block_quote.source_range, markdown_end);
                self.render_html_elements(
                    &block_quote.children,
                    source_allocator,
                    builder,
                    markdown_end,
                    cx,
                );
                self.pop_markdown_block_quote(builder);
            }
            ParsedHtmlElement::Table(table) => {
                self.render_html_table(table, source_allocator, builder, markdown_end, cx);
            }
            ParsedHtmlElement::Image(image) => {
                self.render_html_image(image, builder);
            }
        }
    }

    fn render_html_list(
        &self,
        list: &ParsedHtmlList,
        source_allocator: &mut HtmlSourceAllocator,
        builder: &mut MarkdownElementBuilder,
        markdown_end: usize,
        cx: &mut App,
    ) {
        builder.push_div(div().pl_2p5(), &list.source_range, markdown_end);

        for list_item in &list.items {
            let bullet = match list_item.item_type {
                ParsedHtmlListItemType::Ordered(order) => html_list_item_prefix(
                    order as usize,
                    list.ordered,
                    list.depth.saturating_sub(1) as usize,
                ),
                ParsedHtmlListItemType::Unordered => {
                    html_list_item_prefix(1, false, list.depth.saturating_sub(1) as usize)
                }
            };

            self.push_markdown_list_item(
                builder,
                div().child(bullet).into_any_element(),
                &list_item.source_range,
                markdown_end,
            );
            self.render_html_elements(
                &list_item.content,
                source_allocator,
                builder,
                markdown_end,
                cx,
            );
            self.pop_markdown_list_item(builder);
        }

        builder.pop_div();
    }

    fn render_html_table(
        &self,
        table: &ParsedHtmlTable,
        source_allocator: &mut HtmlSourceAllocator,
        builder: &mut MarkdownElementBuilder,
        markdown_end: usize,
        cx: &mut App,
    ) {
        if let Some(caption) = &table.caption {
            builder.push_div(
                div().when(!self.style.height_is_multiple_of_line_height, |el| {
                    el.mb_2().line_height(rems(1.3))
                }),
                &table.source_range,
                markdown_end,
            );
            self.render_html_paragraph(caption, source_allocator, builder, cx, markdown_end);
            builder.pop_div();
        }

        let actual_header_column_count = html_table_columns_count(&table.header);
        let actual_body_column_count = html_table_columns_count(&table.body);
        let max_column_count = actual_header_column_count.max(actual_body_column_count);

        if max_column_count == 0 {
            return;
        }

        let total_rows = table.header.len() + table.body.len();
        let mut grid_occupied = vec![vec![false; max_column_count]; total_rows];

        builder.push_div(
            div()
                .id(("html-table", table.source_range.start))
                .grid()
                .grid_cols(max_column_count as u16)
                .when(self.style.table_columns_min_size, |this| {
                    this.grid_cols_min_content(max_column_count as u16)
                })
                .when(!self.style.table_columns_min_size, |this| {
                    this.grid_cols(max_column_count as u16)
                })
                .w_full()
                .mb_2()
                .border(px(1.5))
                .border_color(cx.theme().colors().border)
                .rounded_sm()
                .overflow_hidden(),
            &table.source_range,
            markdown_end,
        );

        for (row_index, row) in table.header.iter().chain(table.body.iter()).enumerate() {
            let mut column_index = 0;

            for cell in &row.columns {
                while column_index < max_column_count && grid_occupied[row_index][column_index] {
                    column_index += 1;
                }

                if column_index >= max_column_count {
                    break;
                }

                let max_span = max_column_count.saturating_sub(column_index);
                let mut cell_div = div()
                    .col_span(cell.col_span.min(max_span) as u16)
                    .row_span(cell.row_span.min(total_rows - row_index) as u16)
                    .when(column_index > 0, |this| this.border_l_1())
                    .when(row_index > 0, |this| this.border_t_1())
                    .border_color(cx.theme().colors().border)
                    .px_2()
                    .py_1()
                    .when(cell.is_header, |this| {
                        this.bg(cx.theme().colors().title_bar_background)
                    })
                    .when(!cell.is_header && row_index % 2 == 1, |this| {
                        this.bg(cx.theme().colors().panel_background)
                    });

                cell_div = match cell.alignment {
                    Alignment::Center => cell_div.items_center(),
                    Alignment::Right => cell_div.items_end(),
                    _ => cell_div,
                };

                builder.push_div(cell_div, &table.source_range, markdown_end);
                self.render_html_paragraph(
                    &cell.children,
                    source_allocator,
                    builder,
                    cx,
                    markdown_end,
                );
                builder.pop_div();

                for row_offset in 0..cell.row_span {
                    for column_offset in 0..cell.col_span {
                        if row_index + row_offset < total_rows
                            && column_index + column_offset < max_column_count
                        {
                            grid_occupied[row_index + row_offset][column_index + column_offset] =
                                true;
                        }
                    }
                }

                column_index += cell.col_span;
            }

            while column_index < max_column_count {
                if grid_occupied[row_index][column_index] {
                    column_index += 1;
                    continue;
                }

                builder.push_div(
                    div()
                        .when(column_index > 0, |this| this.border_l_1())
                        .when(row_index > 0, |this| this.border_t_1())
                        .border_color(cx.theme().colors().border)
                        .when(row_index % 2 == 1, |this| {
                            this.bg(cx.theme().colors().panel_background)
                        }),
                    &table.source_range,
                    markdown_end,
                );
                builder.pop_div();
                column_index += 1;
            }
        }

        builder.pop_div();
    }

    fn render_html_paragraph(
        &self,
        paragraph: &HtmlParagraph,
        source_allocator: &mut HtmlSourceAllocator,
        builder: &mut MarkdownElementBuilder,
        cx: &mut App,
        _markdown_end: usize,
    ) {
        for chunk in paragraph {
            match chunk {
                HtmlParagraphChunk::Text(text) => {
                    self.render_html_text(text, source_allocator, builder, cx);
                }
                HtmlParagraphChunk::Image(image) => {
                    self.render_html_image(image, builder);
                }
            }
        }
    }

    fn render_html_text(
        &self,
        text: &ParsedHtmlText,
        source_allocator: &mut HtmlSourceAllocator,
        builder: &mut MarkdownElementBuilder,
        cx: &mut App,
    ) {
        let text_contents = text.contents.as_ref();
        if text_contents.is_empty() {
            return;
        }

        let allocated_range = source_allocator.allocate(text_contents.len());
        let allocated_len = allocated_range.end.saturating_sub(allocated_range.start);

        let mut boundaries = vec![0, text_contents.len()];
        for (range, _) in &text.highlights {
            boundaries.push(range.start);
            boundaries.push(range.end);
        }
        for (range, _) in &text.links {
            boundaries.push(range.start);
            boundaries.push(range.end);
        }
        boundaries.sort_unstable();
        boundaries.dedup();

        for segment in boundaries.windows(2) {
            let start = segment[0];
            let end = segment[1];
            if start >= end {
                continue;
            }

            let source_start = allocated_range.start + start.min(allocated_len);
            let source_end = allocated_range.start + end.min(allocated_len);
            if source_start >= source_end {
                continue;
            }

            let mut refinement = TextStyleRefinement::default();
            let mut has_refinement = false;

            for (highlight_range, style) in &text.highlights {
                if highlight_range.start < end && highlight_range.end > start {
                    apply_html_highlight_style(&mut refinement, style);
                    has_refinement = true;
                }
            }

            let link = text.links.iter().find_map(|(link_range, link)| {
                if link_range.start < end && link_range.end > start {
                    Some(link.clone())
                } else {
                    None
                }
            });

            if let Some(link) = link.as_ref() {
                builder.push_link(link.clone(), source_start..source_end);
                let link_style = self
                    .style
                    .link_callback
                    .as_ref()
                    .and_then(|callback| callback(link.as_ref(), cx))
                    .unwrap_or_else(|| self.style.link.clone());
                builder.push_text_style(link_style);
            }

            if has_refinement {
                builder.push_text_style(refinement);
            }

            builder.push_text(&text_contents[start..end], source_start..source_end);

            if has_refinement {
                builder.pop_text_style();
            }

            if link.is_some() {
                builder.pop_text_style();
            }
        }
    }

    fn render_html_image(&self, image: &HtmlImage, builder: &mut MarkdownElementBuilder) {
        let Some(source) = self
            .image_resolver
            .as_ref()
            .and_then(|resolve| resolve(image.dest_url.as_ref()))
        else {
            return;
        };

        self.push_markdown_image(
            builder,
            &image.source_range,
            source,
            image.width,
            image.height,
        );
    }
}

fn apply_html_highlight_style(refinement: &mut TextStyleRefinement, style: &HtmlHighlightStyle) {
    if style.weight != FontWeight::default() {
        refinement.font_weight = Some(style.weight);
    }

    if style.oblique {
        refinement.font_style = Some(FontStyle::Oblique);
    } else if style.italic {
        refinement.font_style = Some(FontStyle::Italic);
    }

    if style.underline {
        refinement.underline = Some(UnderlineStyle {
            thickness: px(1.),
            color: None,
            ..Default::default()
        });
    }

    if style.strikethrough {
        refinement.strikethrough = Some(StrikethroughStyle {
            thickness: px(1.),
            color: None,
        });
    }
}

fn html_list_item_prefix(order: usize, ordered: bool, depth: usize) -> String {
    let index = order.saturating_sub(1);
    const NUMBERED_PREFIXES_1: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERED_PREFIXES_2: &str = "abcdefghijklmnopqrstuvwxyz";
    const BULLETS: [&str; 5] = ["•", "◦", "▪", "‣", "⁃"];

    if ordered {
        match depth {
            0 => format!("{}. ", order),
            1 => format!(
                "{}. ",
                NUMBERED_PREFIXES_1
                    .chars()
                    .nth(index % NUMBERED_PREFIXES_1.len())
                    .unwrap()
            ),
            _ => format!(
                "{}. ",
                NUMBERED_PREFIXES_2
                    .chars()
                    .nth(index % NUMBERED_PREFIXES_2.len())
                    .unwrap()
            ),
        }
    } else {
        let depth = depth.min(BULLETS.len() - 1);
        format!("{} ", BULLETS[depth])
    }
}

fn html_table_columns_count(rows: &[ParsedHtmlTableRow]) -> usize {
    let mut actual_column_count = 0;
    for row in rows {
        actual_column_count = actual_column_count.max(
            row.columns
                .iter()
                .map(|column| column.col_span)
                .sum::<usize>(),
        );
    }
    actual_column_count
}

#[cfg(test)]
mod tests {
    use gpui::{TestAppContext, size};
    use ui::prelude::*;

    use crate::{
        CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownOptions,
        MarkdownStyle,
    };

    fn ensure_theme_initialized(cx: &mut TestAppContext) {
        cx.update(|cx| {
            if !cx.has_global::<settings::SettingsStore>() {
                settings::init(cx);
            }
            if !cx.has_global::<theme::GlobalTheme>() {
                theme_settings::init(theme::LoadThemes::JustBase, cx);
            }
        });
    }

    fn render_markdown_text(markdown: &str, cx: &mut TestAppContext) -> crate::RenderedText {
        struct TestWindow;

        impl Render for TestWindow {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
            }
        }

        ensure_theme_initialized(cx);

        let (_, cx) = cx.add_window_view(|_, _| TestWindow);
        let markdown = cx.new(|cx| Markdown::new(markdown.to_string().into(), None, None, cx));
        cx.run_until_parked();
        let (rendered, _) = cx.draw(
            Default::default(),
            size(px(600.0), px(600.0)),
            |_window, _cx| {
                MarkdownElement::new(markdown, MarkdownStyle::default()).code_block_renderer(
                    CodeBlockRenderer::Default {
                        copy_button_visibility: CopyButtonVisibility::Hidden,
                        border: false,
                    },
                )
            },
        );
        rendered.text
    }

    #[gpui::test]
    fn test_html_block_rendering_smoke(cx: &mut TestAppContext) {
        let rendered = render_markdown_text(
            "<h1>Hello</h1><blockquote><p>world</p></blockquote><ul><li>item</li></ul>",
            cx,
        );

        let rendered_lines = rendered
            .lines
            .iter()
            .map(|line| line.layout.wrapped_text())
            .collect::<Vec<_>>();

        assert_eq!(
            rendered_lines.concat().replace('\n', ""),
            "<h1>Hello</h1><blockquote><p>world</p></blockquote><ul><li>item</li></ul>"
        );
    }

    #[gpui::test]
    fn test_html_block_rendering_can_be_enabled(cx: &mut TestAppContext) {
        struct TestWindow;

        impl Render for TestWindow {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
            }
        }

        ensure_theme_initialized(cx);

        let (_, cx) = cx.add_window_view(|_, _| TestWindow);
        let markdown = cx.new(|cx| {
            Markdown::new_with_options(
                "<h1>Hello</h1><blockquote><p>world</p></blockquote><ul><li>item</li></ul>".into(),
                None,
                None,
                MarkdownOptions {
                    parse_html: true,
                    ..Default::default()
                },
                cx,
            )
        });
        cx.run_until_parked();
        let (rendered, _) = cx.draw(
            Default::default(),
            size(px(600.0), px(600.0)),
            |_window, _cx| {
                MarkdownElement::new(markdown, MarkdownStyle::default()).code_block_renderer(
                    CodeBlockRenderer::Default {
                        copy_button_visibility: CopyButtonVisibility::Hidden,
                        border: false,
                    },
                )
            },
        );

        let rendered_lines = rendered
            .text
            .lines
            .iter()
            .map(|line| line.layout.wrapped_text())
            .collect::<Vec<_>>();

        assert_eq!(rendered_lines[0], "Hello");
        assert_eq!(rendered_lines[1], "world");
        assert!(rendered_lines.iter().any(|line| line.contains("item")));
    }
}
