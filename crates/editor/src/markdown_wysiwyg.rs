use crate::display_map::{
    BlockContext, BlockPlacement, BlockProperties, BlockStyle, Crease, CustomBlockId,
    FoldPlaceholder, HighlightKey, RenderBlock,
};
use crate::{Editor, ToggleMarkdownWysiwyg};
use gpui::{
    Context, FontStyle, FontWeight, HighlightStyle, Hsla, IntoElement, ParentElement, SharedString,
    Styled, Task, Window,
};
use multi_buffer::{Anchor, MultiBufferOffset, MultiBufferSnapshot};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::any::TypeId;
use collections::HashSet;
use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

struct WysiwygFoldTag;

const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub struct MarkdownWysiwygState {
    pub active: bool,
    reparse_task: Task<()>,
    block_ids: Vec<CustomBlockId>,
}

impl MarkdownWysiwygState {
    pub fn new() -> Self {
        Self {
            active: false,
            reparse_task: Task::ready(()),
            block_ids: Vec::new(),
        }
    }
}

struct InlineDecoration {
    content_range: Range<usize>,
    kind: DecorationKind,
}

enum DecorationKind {
    Bold,
    Italic,
    Strikethrough,
    InlineCode,
}

struct HeadingDecoration {
    line_range: Range<usize>,
    level: u8,
}

struct TableDecoration {
    range: Range<usize>,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

struct ImageDecoration {
    range: Range<usize>,
    url: String,
}

struct SyntaxMarker {
    range: Range<usize>,
}

struct MarkdownDecorations {
    inline_decorations: Vec<InlineDecoration>,
    headings: Vec<HeadingDecoration>,
    tables: Vec<TableDecoration>,
    images: Vec<ImageDecoration>,
    syntax_markers: Vec<SyntaxMarker>,
}

fn parse_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

fn parse_markdown_decorations(text: &str) -> MarkdownDecorations {
    let parser = Parser::new_ext(text, parse_options());

    let mut inline_decorations = Vec::new();
    let mut headings = Vec::new();
    let mut tables = Vec::new();
    let mut images = Vec::new();
    let mut syntax_markers = Vec::new();

    let mut bold_start: Option<usize> = None;
    let mut italic_start: Option<usize> = None;
    let mut strikethrough_start: Option<usize> = None;
    let mut heading_start: Option<(u8, usize)> = None;

    let mut table_start: Option<usize> = None;
    let mut table_headers: Vec<String> = Vec::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();
    let mut in_table_head = false;
    let mut in_table_cell = false;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Strong) => {
                bold_start = Some(range.start);
                syntax_markers.push(SyntaxMarker {
                    range: range.start..range.start + 2,
                });
            }
            Event::End(TagEnd::Strong) => {
                if let Some(start) = bold_start.take() {
                    syntax_markers.push(SyntaxMarker {
                        range: range.end - 2..range.end,
                    });
                    inline_decorations.push(InlineDecoration {
                        content_range: start..range.end,
                        kind: DecorationKind::Bold,
                    });
                }
            }
            Event::Start(Tag::Emphasis) => {
                italic_start = Some(range.start);
                syntax_markers.push(SyntaxMarker {
                    range: range.start..range.start + 1,
                });
            }
            Event::End(TagEnd::Emphasis) => {
                if let Some(start) = italic_start.take() {
                    syntax_markers.push(SyntaxMarker {
                        range: range.end - 1..range.end,
                    });
                    inline_decorations.push(InlineDecoration {
                        content_range: start..range.end,
                        kind: DecorationKind::Italic,
                    });
                }
            }
            Event::Start(Tag::Strikethrough) => {
                strikethrough_start = Some(range.start);
                syntax_markers.push(SyntaxMarker {
                    range: range.start..range.start + 2,
                });
            }
            Event::End(TagEnd::Strikethrough) => {
                if let Some(start) = strikethrough_start.take() {
                    syntax_markers.push(SyntaxMarker {
                        range: range.end - 2..range.end,
                    });
                    inline_decorations.push(InlineDecoration {
                        content_range: start..range.end,
                        kind: DecorationKind::Strikethrough,
                    });
                }
            }
            Event::Code(code_text) => {
                let code_str = code_text.as_ref();
                let full_start = range.start;
                let full_end = range.end;
                if full_end > full_start + code_str.len() {
                    let backtick_len = (full_end - full_start - code_str.len()) / 2;
                    syntax_markers.push(SyntaxMarker {
                        range: full_start..full_start + backtick_len,
                    });
                    syntax_markers.push(SyntaxMarker {
                        range: full_end - backtick_len..full_end,
                    });
                }
                inline_decorations.push(InlineDecoration {
                    content_range: full_start..full_end,
                    kind: DecorationKind::InlineCode,
                });
            }
            Event::Start(Tag::Heading { level, .. }) => {
                heading_start = Some((level as u8, range.start));
                let hash_count = level as usize;
                let marker_end = range.start + hash_count;
                let space_end = if text.as_bytes().get(marker_end) == Some(&b' ') {
                    marker_end + 1
                } else {
                    marker_end
                };
                syntax_markers.push(SyntaxMarker {
                    range: range.start..space_end,
                });
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, start)) = heading_start.take() {
                    headings.push(HeadingDecoration {
                        line_range: start..range.end,
                        level,
                    });
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                let url = dest_url.to_string();
                let mut end = range.end;
                if let Some(pos) = text[range.start..].find(')') {
                    end = range.start + pos + 1;
                }
                images.push(ImageDecoration {
                    range: range.start..end,
                    url,
                });
            }
            Event::Start(Tag::Table(_)) => {
                table_start = Some(range.start);
                table_headers.clear();
                table_rows.clear();
            }
            Event::End(TagEnd::Table) => {
                if let Some(start) = table_start.take() {
                    tables.push(TableDecoration {
                        range: start..range.end,
                        headers: table_headers.clone(),
                        rows: table_rows.clone(),
                    });
                }
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
                current_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
                table_headers = current_row.clone();
                current_row.clear();
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                if !in_table_head {
                    table_rows.push(current_row.clone());
                }
                current_row.clear();
            }
            Event::Start(Tag::TableCell) => {
                in_table_cell = true;
                current_cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                in_table_cell = false;
                current_row.push(current_cell.clone());
                current_cell.clear();
            }
            Event::Text(text_content) => {
                if in_table_cell {
                    current_cell.push_str(text_content.as_ref());
                }
            }
            _ => {}
        }
    }

    MarkdownDecorations {
        inline_decorations,
        headings,
        tables,
        images,
        syntax_markers,
    }
}


impl Editor {
    pub fn toggle_markdown_wysiwyg(
        &mut self,
        _: &ToggleMarkdownWysiwyg,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.markdown_wysiwyg_state.active {
            clear_wysiwyg_decorations(self, cx);
            self.markdown_wysiwyg_state.active = false;
        } else {
            self.markdown_wysiwyg_state.active = true;
            refresh_wysiwyg_decorations(self, cx);
        }
    }
}

pub fn schedule_wysiwyg_refresh(editor: &mut Editor, cx: &mut Context<Editor>) {
    if !editor.markdown_wysiwyg_state.active {
        return;
    }

    editor.markdown_wysiwyg_state.reparse_task = cx.spawn(async move |editor, cx| {
        cx.background_executor()
            .timer(REPARSE_DEBOUNCE)
            .await;
        editor
            .update(cx, |editor, cx| {
                refresh_wysiwyg_decorations(editor, cx);
            })
            .ok();
    });
}

fn refresh_wysiwyg_decorations(editor: &mut Editor, cx: &mut Context<Editor>) {
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let text = snapshot.text();
    let decorations = parse_markdown_decorations(&text);

    apply_highlights(editor, &snapshot, &decorations, cx);
    apply_folds(editor, &snapshot, &decorations, cx);
    apply_blocks(editor, &snapshot, &decorations, cx);
}

fn apply_highlights(
    editor: &mut Editor,
    snapshot: &MultiBufferSnapshot,
    decorations: &MarkdownDecorations,
    cx: &mut Context<Editor>,
) {
    let mut bold_ranges = Vec::new();
    let mut italic_ranges = Vec::new();
    let mut strikethrough_ranges = Vec::new();
    let mut code_ranges = Vec::new();

    for decoration in &decorations.inline_decorations {
        let start = snapshot.anchor_before(MultiBufferOffset(decoration.content_range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(decoration.content_range.end));
        let range = start..end;

        match decoration.kind {
            DecorationKind::Bold => bold_ranges.push(range),
            DecorationKind::Italic => italic_ranges.push(range),
            DecorationKind::Strikethrough => strikethrough_ranges.push(range),
            DecorationKind::InlineCode => code_ranges.push(range),
        }
    }

    let mut heading_ranges = Vec::new();
    for heading in &decorations.headings {
        let start = snapshot.anchor_before(MultiBufferOffset(heading.line_range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(heading.line_range.end));
        heading_ranges.push(start..end);
    }

    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygBold,
        bold_ranges,
        HighlightStyle {
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        },
        cx,
    );

    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygItalic,
        italic_ranges,
        HighlightStyle {
            font_style: Some(FontStyle::Italic),
            ..Default::default()
        },
        cx,
    );

    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygStrikethrough,
        strikethrough_ranges,
        HighlightStyle {
            strikethrough: Some(gpui::StrikethroughStyle {
                thickness: gpui::px(1.0),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    );

    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygCode,
        code_ranges,
        HighlightStyle {
            background_color: Some(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.3,
                a: 0.15,
            }),
            ..Default::default()
        },
        cx,
    );

    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygHeading,
        heading_ranges,
        HighlightStyle {
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        },
        cx,
    );
}

fn set_or_clear_highlight(
    editor: &mut Editor,
    key: HighlightKey,
    ranges: Vec<Range<multi_buffer::Anchor>>,
    style: HighlightStyle,
    cx: &mut Context<Editor>,
) {
    if ranges.is_empty() {
        editor.clear_highlights(key, cx);
    } else {
        editor.highlight_text(key, ranges, style, cx);
    }
}

fn collect_block_replaced_ranges(decorations: &MarkdownDecorations) -> Vec<Range<usize>> {
    let mut block_ranges: Vec<Range<usize>> = Vec::new();
    for heading in &decorations.headings {
        block_ranges.push(heading.line_range.clone());
    }
    for table in &decorations.tables {
        block_ranges.push(table.range.clone());
    }
    for image in &decorations.images {
        block_ranges.push(image.range.clone());
    }
    block_ranges.sort_by_key(|range| range.start);
    block_ranges
}

fn marker_overlaps_block(marker: &Range<usize>, block_ranges: &[Range<usize>]) -> bool {
    block_ranges.iter().any(|block| {
        marker.start < block.end && marker.end > block.start
    })
}

fn apply_folds(
    editor: &mut Editor,
    snapshot: &MultiBufferSnapshot,
    decorations: &MarkdownDecorations,
    cx: &mut Context<Editor>,
) {
    let type_id = TypeId::of::<WysiwygFoldTag>();

    let block_ranges = collect_block_replaced_ranges(decorations);

    let valid_markers: Vec<&SyntaxMarker> = decorations
        .syntax_markers
        .iter()
        .filter(|marker| {
            marker.range.start < marker.range.end
                && !marker_overlaps_block(&marker.range, &block_ranges)
        })
        .collect();

    if valid_markers.is_empty() {
        return;
    }

    let placeholder = FoldPlaceholder {
        render: Arc::new(|_, _, _| gpui::Empty.into_any_element()),
        constrain_width: true,
        merge_adjacent: false,
        type_tag: Some(type_id),
        collapsed_text: Some(SharedString::from("\u{200B}")),
    };

    let mut sorted_markers = valid_markers;
    sorted_markers.sort_by_key(|marker| marker.range.start);

    let mut merged_ranges: Vec<Range<usize>> = Vec::new();
    for marker in &sorted_markers {
        if let Some(last) = merged_ranges.last_mut() {
            if marker.range.start <= last.end {
                last.end = last.end.max(marker.range.end);
                continue;
            }
        }
        merged_ranges.push(marker.range.clone());
    }

    let creases: Vec<Crease<MultiBufferOffset>> = merged_ranges
        .into_iter()
        .map(|range| {
            Crease::simple(
                MultiBufferOffset(range.start)..MultiBufferOffset(range.end),
                placeholder.clone(),
            )
        })
        .collect();

    editor.display_map.update(cx, |display_map, cx| {
        display_map.fold(creases, cx);
    });
    cx.notify();
}

fn apply_blocks(
    editor: &mut Editor,
    snapshot: &MultiBufferSnapshot,
    decorations: &MarkdownDecorations,
    cx: &mut Context<Editor>,
) {
    let old_block_ids: HashSet<CustomBlockId> = editor
        .markdown_wysiwyg_state
        .block_ids
        .drain(..)
        .collect();
    if !old_block_ids.is_empty() {
        editor.remove_blocks(old_block_ids, None, cx);
    }

    let mut block_properties: Vec<BlockProperties<Anchor>> = Vec::new();

    for heading in &decorations.headings {
        let start = snapshot.anchor_before(MultiBufferOffset(heading.line_range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(heading.line_range.end));

        let level = heading.level;
        let buffer_text = snapshot.text();
        let display_text: String = buffer_text
            .get(heading.line_range.clone())
            .unwrap_or_default()
            .trim_start_matches('#')
            .trim_start()
            .to_string();

        let render: RenderBlock = Arc::new(move |block_context: &mut BlockContext| {
            let font_size_multiplier = match level {
                1 => 2.0_f32,
                2 => 1.6,
                3 => 1.3,
                4 => 1.1,
                5 => 1.0,
                _ => 0.9,
            };
            let base_size = block_context.em_width;
            let scaled_size = base_size * font_size_multiplier;

            gpui::div()
                .text_size(scaled_size)
                .font_weight(FontWeight::BOLD)
                .line_height(scaled_size * 1.4)
                .child(SharedString::from(display_text.clone()))
                .into_any_element()
        });

        block_properties.push(BlockProperties {
            placement: BlockPlacement::Replace(start..=end),
            height: Some(1),
            style: BlockStyle::Flex,
            render,
            priority: 0,
        });
    }

    for table in &decorations.tables {
        let start = snapshot.anchor_before(MultiBufferOffset(table.range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(table.range.end));

        let headers = table.headers.clone();
        let rows = table.rows.clone();
        let row_count = rows.len() as u32 + 1;

        let render: RenderBlock = Arc::new(move |_block_context: &mut BlockContext| {
            let border_color = Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.5,
                a: 0.3,
            };

            let mut table_element = gpui::div()
                .w_full()
                .border_1()
                .border_color(border_color);

            let mut header_row = gpui::div()
                .flex()
                .flex_row()
                .w_full()
                .border_b_1()
                .border_color(border_color)
                .bg(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.3,
                    a: 0.1,
                });

            for header in &headers {
                header_row = header_row.child(
                    gpui::div()
                        .flex_1()
                        .px_2()
                        .py_1()
                        .font_weight(FontWeight::BOLD)
                        .border_r_1()
                        .border_color(border_color)
                        .child(SharedString::from(header.clone())),
                );
            }
            table_element = table_element.child(header_row);

            for row in &rows {
                let mut row_element = gpui::div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .border_b_1()
                    .border_color(border_color);

                for cell in row {
                    row_element = row_element.child(
                        gpui::div()
                            .flex_1()
                            .px_2()
                            .py_1()
                            .border_r_1()
                            .border_color(border_color)
                            .child(SharedString::from(cell.clone())),
                    );
                }
                table_element = table_element.child(row_element);
            }

            table_element.into_any_element()
        });

        block_properties.push(BlockProperties {
            placement: BlockPlacement::Replace(start..=end),
            height: Some(row_count + 1),
            style: BlockStyle::Flex,
            render,
            priority: 0,
        });
    }

    for image in &decorations.images {
        let start = snapshot.anchor_before(MultiBufferOffset(image.range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(image.range.end));

        let url = image.url.clone();

        let render: RenderBlock = Arc::new(move |_block_context: &mut BlockContext| {
            gpui::div()
                .py_1()
                .child(
                    gpui::div()
                        .px_2()
                        .py_1()
                        .bg(Hsla {
                            h: 0.6,
                            s: 0.2,
                            l: 0.3,
                            a: 0.2,
                        })
                        .rounded_sm()
                        .child(SharedString::from(format!("[Image: {}]", &url))),
                )
                .into_any_element()
        });

        block_properties.push(BlockProperties {
            placement: BlockPlacement::Replace(start..=end),
            height: Some(2),
            style: BlockStyle::Flex,
            render,
            priority: 0,
        });
    }

    if !block_properties.is_empty() {
        let new_ids = editor.insert_blocks(block_properties, None, cx);
        editor.markdown_wysiwyg_state.block_ids = new_ids;
    }
}

fn clear_wysiwyg_decorations(editor: &mut Editor, cx: &mut Context<Editor>) {
    editor.clear_highlights(HighlightKey::MarkdownWysiwygBold, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygItalic, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygStrikethrough, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygCode, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygHeading, cx);

    let type_id = TypeId::of::<WysiwygFoldTag>();
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let buffer_len = snapshot.len();
    let full_range = vec![
        MultiBufferOffset(0)..buffer_len,
    ];
    editor.remove_folds_with_type(&full_range, type_id, false, cx);

    let old_block_ids: HashSet<CustomBlockId> = editor
        .markdown_wysiwyg_state
        .block_ids
        .drain(..)
        .collect();
    if !old_block_ids.is_empty() {
        editor.remove_blocks(old_block_ids, None, cx);
    }
}

pub fn on_selection_changed(editor: &mut Editor, cx: &mut Context<Editor>) {
    if !editor.markdown_wysiwyg_state.active {
        return;
    }
    refresh_wysiwyg_decorations(editor, cx);
}
