use crate::display_map::{
    BlockContext, BlockPlacement, BlockProperties, BlockStyle, Crease, CustomBlockId,
    FoldPlaceholder, HighlightKey, RenderBlock,
};
use crate::{Editor, ToggleMarkdownWysiwyg};
use gpui::{
    Context, ElementId, FontStyle, FontWeight, HighlightStyle, Hsla, InteractiveElement,
    IntoElement, ParentElement, SharedString, StatefulInteractiveElement, Styled, Task,
    TextStyleRefinement, Window,
};
use multi_buffer::{Anchor, MultiBufferOffset, MultiBufferSnapshot, ToOffset};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use theme::ActiveTheme;
use collections::HashSet;
use std::any::TypeId;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

struct WysiwygFoldTag;

const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);
const READABLE_LINE_LENGTH: u32 = 60;

pub struct MarkdownWysiwygState {
    pub active: bool,
    reparse_task: Task<()>,
    references_task: Task<()>,
    block_ids: Vec<CustomBlockId>,
    references_block_ids: Vec<CustomBlockId>,
    pub cached_references: Vec<String>,
    previous_show_gutter: Option<bool>,
    previous_show_line_numbers: Option<Option<bool>>,
    previous_soft_wrap_override: Option<Option<language::language_settings::SoftWrap>>,
}

impl MarkdownWysiwygState {
    pub fn new() -> Self {
        Self {
            active: false,
            reparse_task: Task::ready(()),
            references_task: Task::ready(()),
            block_ids: Vec::new(),
            references_block_ids: Vec::new(),
            cached_references: Vec::new(),
            previous_show_gutter: None,
            previous_show_line_numbers: None,
            previous_soft_wrap_override: None,
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
    width: Option<u32>,
    height: Option<u32>,
}

struct SyntaxMarker {
    range: Range<usize>,
}

struct WikilinkDecoration {
    range: Range<usize>,
    display_text: String,
}

struct ListItemDecoration {
    marker_range: Range<usize>,
}

struct MarkdownDecorations {
    inline_decorations: Vec<InlineDecoration>,
    headings: Vec<HeadingDecoration>,
    tables: Vec<TableDecoration>,
    images: Vec<ImageDecoration>,
    syntax_markers: Vec<SyntaxMarker>,
    wikilinks: Vec<WikilinkDecoration>,
    list_items: Vec<ListItemDecoration>,
}

fn parse_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

fn parse_image_dimensions(title: &str) -> (Option<u32>, Option<u32>) {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return (None, None);
    }
    if let Some(inner) = trimmed.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
        let parts: Vec<&str> = inner.split(',').collect();
        let width = parts.first().and_then(|s| s.trim().parse::<u32>().ok());
        let height = parts.get(1).and_then(|s| s.trim().parse::<u32>().ok());
        return (width, height);
    }
    if let Ok(width) = trimmed.parse::<u32>() {
        return (Some(width), None);
    }
    (None, None)
}

fn parse_wikilink_images(text: &str) -> Vec<ImageDecoration> {
    let mut results = Vec::new();
    let mut search_start = 0;
    while let Some(start) = text[search_start..].find("![[") {
        let absolute_start = search_start + start;
        let after_prefix = absolute_start + 3;
        if let Some(end_offset) = text[after_prefix..].find("]]") {
            let inner = &text[after_prefix..after_prefix + end_offset];
            let absolute_end = after_prefix + end_offset + 2;
            let (url, width) = if let Some(pipe_pos) = inner.find('|') {
                let path = inner[..pipe_pos].trim().to_string();
                let dimension = inner[pipe_pos + 1..].trim().parse::<u32>().ok();
                (path, dimension)
            } else {
                (inner.trim().to_string(), None)
            };
            results.push(ImageDecoration {
                range: absolute_start..absolute_end,
                url,
                width,
                height: None,
            });
            search_start = absolute_end;
        } else {
            break;
        }
    }
    results
}

fn parse_wikilinks(text: &str) -> Vec<WikilinkDecoration> {
    let mut results = Vec::new();
    let mut search_start = 0;
    while let Some(start) = text[search_start..].find("[[") {
        let absolute_start = search_start + start;
        if absolute_start > 0 && text.as_bytes().get(absolute_start.wrapping_sub(1)) == Some(&b'!') {
            search_start = absolute_start + 2;
            continue;
        }
        let after_prefix = absolute_start + 2;
        if let Some(end_offset) = text[after_prefix..].find("]]") {
            let inner = &text[after_prefix..after_prefix + end_offset];
            let absolute_end = after_prefix + end_offset + 2;
            let display_text = if let Some(pipe_pos) = inner.find('|') {
                inner[pipe_pos + 1..].trim().to_string()
            } else {
                inner.trim().to_string()
            };
            results.push(WikilinkDecoration {
                range: absolute_start..absolute_end,
                display_text,
            });
            search_start = absolute_end;
        } else {
            break;
        }
    }
    results
}

fn parse_list_items(text: &str) -> Vec<ListItemDecoration> {
    let mut results = Vec::new();
    for (line_start, line) in text.match_indices('\n') {
        let content_start = line_start + 1;
        let rest = &text[content_start..];
        let trimmed = rest.trim_start();
        let indent = rest.len() - trimmed.len();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let marker_start = content_start + indent;
            let marker_end = marker_start + 2;
            results.push(ListItemDecoration {
                marker_range: marker_start..marker_end,
            });
        }
    }
    if text.starts_with("- ") || text.starts_with("* ") {
        results.push(ListItemDecoration {
            marker_range: 0..2,
        });
    } else {
        let trimmed = text.trim_start();
        let indent = text.len() - trimmed.len();
        if indent > 0 && (trimmed.starts_with("- ") || trimmed.starts_with("* ")) {
            results.push(ListItemDecoration {
                marker_range: indent..indent + 2,
            });
        }
    }
    results
}

fn parse_markdown_decorations(text: &str) -> MarkdownDecorations {
    let parser = Parser::new_ext(text, parse_options());

    let mut inline_decorations = Vec::new();
    let mut headings = Vec::new();
    let mut tables = Vec::new();
    let mut images: Vec<ImageDecoration> = parse_wikilink_images(text);
    let mut syntax_markers = Vec::new();
    let wikilinks = parse_wikilinks(text);
    let list_items = parse_list_items(text);

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
                        content_range: start + 2..range.end - 2,
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
                        content_range: start + 1..range.end - 1,
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
                        content_range: start + 2..range.end - 2,
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
                    inline_decorations.push(InlineDecoration {
                        content_range: full_start + backtick_len..full_end - backtick_len,
                        kind: DecorationKind::InlineCode,
                    });
                } else {
                    inline_decorations.push(InlineDecoration {
                        content_range: full_start..full_end,
                        kind: DecorationKind::InlineCode,
                    });
                }
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
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                let url = dest_url.to_string();
                let mut end = range.end;
                if let Some(pos) = text[range.start..].find(')') {
                    end = range.start + pos + 1;
                }
                let (width, height) = parse_image_dimensions(&title);
                images.push(ImageDecoration {
                    range: range.start..end,
                    url,
                    width,
                    height,
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

    for wikilink in &wikilinks {
        syntax_markers.push(SyntaxMarker {
            range: wikilink.range.start..wikilink.range.start + 2,
        });
        syntax_markers.push(SyntaxMarker {
            range: wikilink.range.end - 2..wikilink.range.end,
        });
        if let Some(pipe_pos) = text[wikilink.range.start + 2..wikilink.range.end - 2].find('|') {
            let link_end = wikilink.range.start + 2 + pipe_pos + 1;
            syntax_markers.push(SyntaxMarker {
                range: wikilink.range.start + 2..link_end,
            });
        }
    }


    MarkdownDecorations {
        inline_decorations,
        headings,
        tables,
        images,
        syntax_markers,
        wikilinks,
        list_items,
    }
}


fn cursor_offset(editor: &Editor, cx: &mut Context<Editor>) -> usize {
    let anchor = editor.selections.newest_anchor();
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let offset: MultiBufferOffset = anchor.head().to_offset(&snapshot);
    offset.0
}

/// Returns the full range of lines covered by the newest selection.
/// For a collapsed cursor this is just the cursor line; for a multi-line
/// selection it spans from the first selected line start to the last selected line end.
fn selection_line_range(editor: &Editor, text: &str, cx: &mut Context<Editor>) -> Range<usize> {
    let anchor = editor.selections.newest_anchor();
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let head_offset: usize = anchor.head().to_offset(&snapshot).0;
    let tail_offset: usize = anchor.tail().to_offset(&snapshot).0;
    let selection_start = head_offset.min(tail_offset);
    let selection_end = head_offset.max(tail_offset);

    let start_line = cursor_line_range(text, selection_start);
    let end_line = cursor_line_range(text, selection_end);
    start_line.start..end_line.end
}

fn range_contains_cursor(range: &Range<usize>, cursor: usize) -> bool {
    cursor >= range.start && cursor <= range.end
}

fn cursor_line_range(text: &str, cursor: usize) -> Range<usize> {
    let cursor = cursor.min(text.len());
    let line_start = text[..cursor].rfind('\n').map_or(0, |pos| pos + 1);
    let line_end = text[cursor..].find('\n').map_or(text.len(), |pos| cursor + pos);
    line_start..line_end
}

fn range_on_cursor_line(range: &Range<usize>, cursor_line: &Range<usize>) -> bool {
    range.start < cursor_line.end && range.end > cursor_line.start
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
            if let Some(show_gutter) = self.markdown_wysiwyg_state.previous_show_gutter {
                self.set_show_gutter(show_gutter, cx);
            }
            if let Some(show_line_numbers) = self.markdown_wysiwyg_state.previous_show_line_numbers
            {
                self.show_line_numbers = show_line_numbers;
            }
            if let Some(soft_wrap) = self.markdown_wysiwyg_state.previous_soft_wrap_override.take()
            {
                self.soft_wrap_mode_override = soft_wrap;
            }
            self.set_text_style_refinement(TextStyleRefinement::default());
            self.style = None;
            self.markdown_wysiwyg_state.active = false;
            cx.notify();
        } else {
            self.markdown_wysiwyg_state.previous_show_gutter = Some(self.show_gutter);
            self.markdown_wysiwyg_state.previous_show_line_numbers = Some(self.show_line_numbers);
            self.markdown_wysiwyg_state.previous_soft_wrap_override =
                Some(self.soft_wrap_mode_override);

            self.set_text_style_refinement(TextStyleRefinement {
                font_family: Some(SharedString::from("Liberation Serif")),
                ..Default::default()
            });
            self.style = None;

            self.set_show_gutter(false, cx);
            self.soft_wrap_mode_override =
                Some(language::language_settings::SoftWrap::PreferredLineLength);
            self.markdown_wysiwyg_state.active = true;
            refresh_wysiwyg_decorations(self, cx);
            fetch_references(self, cx);
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
    let cursor = cursor_offset(editor, cx);
    // Use the full selection range (covers all lines in a multi-line selection)
    let active_line_range = selection_line_range(editor, &text, cx);

    apply_highlights(editor, &snapshot, &decorations, cursor, &active_line_range, cx);
    remove_stale_folds(editor, cx);
    apply_marker_folds(editor, &snapshot, &decorations, cursor, &active_line_range, cx);
    apply_blocks(editor, &snapshot, &decorations, &active_line_range, cx);
}

fn apply_highlights(
    editor: &mut Editor,
    snapshot: &MultiBufferSnapshot,
    decorations: &MarkdownDecorations,
    cursor: usize,
    cursor_line: &Range<usize>,
    cx: &mut Context<Editor>,
) {
    let foreground = cx.theme().colors().editor_foreground;
    let block_ranges = collect_block_replaced_ranges(decorations, cursor_line);

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
            DecorationKind::InlineCode => {
                if !marker_overlaps_block(&decoration.content_range, &block_ranges) {
                    code_ranges.push(range);
                }
            }
        }
    }

    let mut heading_ranges = Vec::new();
    for heading in &decorations.headings {
        let start = snapshot.anchor_before(MultiBufferOffset(heading.line_range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(heading.line_range.end));
        heading_ranges.push(start..end);
    }

    let marker_ranges: Vec<Range<multi_buffer::Anchor>> = Vec::new();

    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygBold,
        bold_ranges,
        HighlightStyle {
            color: Some(foreground),
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
            color: Some(foreground),
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
            color: Some(foreground),
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
            color: Some(foreground),
            background_color: Some(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.2,
                a: 0.08,
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
            color: Some(foreground),
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        },
        cx,
    );

    // Collect diagnostics for the full buffer to detect unresolved links
    let all_diagnostics: Vec<(Range<usize>, String)> = snapshot
        .diagnostics_in_range::<MultiBufferOffset>(MultiBufferOffset(0)..MultiBufferOffset(snapshot.len().0))
        .map(|entry| (entry.range.start.0..entry.range.end.0, entry.diagnostic.message.clone()))
        .collect();

    let mut wikilink_ranges = Vec::new();
    let mut unresolved_wikilink_ranges = Vec::new();
    for wikilink in &decorations.wikilinks {
        if !marker_overlaps_block(&wikilink.range, &block_ranges)
            && !range_on_cursor_line(&wikilink.range, cursor_line)
        {
            let content_start = wikilink.range.start + 2;
            let content_end = wikilink.range.end - 2;
            let inner = &snapshot.text()[content_start..content_end];
            let display_start = if let Some(pipe_pos) = inner.find('|') {
                content_start + pipe_pos + 1
            } else {
                content_start
            };
            let start = snapshot.anchor_before(MultiBufferOffset(display_start));
            let end = snapshot.anchor_after(MultiBufferOffset(content_end));

            // Check if this wikilink has a diagnostic (unresolved link)
            let has_diagnostic = all_diagnostics.iter().any(|(diag_range, _msg)| {
                diag_range.start < wikilink.range.end && diag_range.end > wikilink.range.start
            });

            if has_diagnostic {
                unresolved_wikilink_ranges.push(start..end);
            } else {
                wikilink_ranges.push(start..end);
            }
        }
    }

    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygWikilink,
        wikilink_ranges,
        HighlightStyle {
            color: Some(Hsla {
                h: 0.6,
                s: 0.5,
                l: 0.6,
                a: 1.0,
            }),
            background_color: Some(Hsla {
                h: 0.6,
                s: 0.2,
                l: 0.3,
                a: 0.15,
            }),
            font_weight: Some(FontWeight::BOLD),
            font_style: Some(gpui::FontStyle::Normal),
            underline: Some(gpui::UnderlineStyle {
                thickness: gpui::px(1.0),
                color: Some(Hsla {
                    h: 0.6,
                    s: 0.5,
                    l: 0.6,
                    a: 0.6,
                }),
                wavy: false,
            }),
            ..Default::default()
        },
        cx,
    );

    // Unresolved wikilinks get a pastel blue color
    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygUnresolvedLink,
        unresolved_wikilink_ranges,
        HighlightStyle {
            color: Some(Hsla {
                h: 0.58,
                s: 0.4,
                l: 0.65,
                a: 0.7,
            }),
            background_color: Some(Hsla {
                h: 0.58,
                s: 0.2,
                l: 0.3,
                a: 0.1,
            }),
            font_weight: Some(FontWeight::BOLD),
            font_style: Some(gpui::FontStyle::Normal),
            underline: Some(gpui::UnderlineStyle {
                thickness: gpui::px(1.0),
                color: Some(Hsla {
                    h: 0.58,
                    s: 0.4,
                    l: 0.65,
                    a: 0.5,
                }),
                wavy: true,
            }),
            ..Default::default()
        },
        cx,
    );

    let background = cx.theme().colors().editor_background;
    set_or_clear_highlight(
        editor,
        HighlightKey::MarkdownWysiwygMarker,
        marker_ranges,
        HighlightStyle {
            color: Some(background),
            background_color: Some(background),
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

fn collect_block_replaced_ranges(
    decorations: &MarkdownDecorations,
    active_range: &Range<usize>,
) -> Vec<Range<usize>> {
    let mut block_ranges: Vec<Range<usize>> = Vec::new();
    for heading in &decorations.headings {
        if !range_on_cursor_line(&heading.line_range, active_range) {
            block_ranges.push(heading.line_range.clone());
        }
    }
    for table in &decorations.tables {
        if !range_on_cursor_line(&table.range, active_range) {
            block_ranges.push(table.range.clone());
        }
    }
    for image in &decorations.images {
        if !range_on_cursor_line(&image.range, active_range) {
            block_ranges.push(image.range.clone());
        }
    }
    block_ranges.sort_by_key(|range| range.start);
    block_ranges
}

fn marker_overlaps_block(marker: &Range<usize>, block_ranges: &[Range<usize>]) -> bool {
    block_ranges.iter().any(|block| {
        marker.start < block.end && marker.end > block.start
    })
}

fn apply_marker_folds(
    editor: &mut Editor,
    snapshot: &MultiBufferSnapshot,
    decorations: &MarkdownDecorations,
    _cursor: usize,
    active_range: &Range<usize>,
    cx: &mut Context<Editor>,
) {
    let block_ranges = collect_block_replaced_ranges(decorations, active_range);
    let placeholder = FoldPlaceholder {
        render: Arc::new(|_fold_id, _range, _cx| gpui::Empty.into_any_element()),
        constrain_width: false,
        merge_adjacent: false,
        type_tag: Some(TypeId::of::<WysiwygFoldTag>()),
        collapsed_text: Some("".into()),
    };

    let bullet_placeholder = FoldPlaceholder {
        render: Arc::new(|_fold_id, _range, _cx| {
            gpui::div()
                .child(SharedString::from("• "))
                .into_any_element()
        }),
        constrain_width: true,
        merge_adjacent: false,
        type_tag: Some(TypeId::of::<WysiwygFoldTag>()),
        collapsed_text: Some("• ".into()),
    };

    let mut creases = Vec::new();
    for marker in &decorations.syntax_markers {
        if marker.range.start < marker.range.end
            && !marker_overlaps_block(&marker.range, &block_ranges)
            && !range_on_cursor_line(&marker.range, active_range)
        {
            let start = MultiBufferOffset(marker.range.start);
            let end = MultiBufferOffset(marker.range.end);
            creases.push(Crease::simple(start..end, placeholder.clone()));
        }
    }

    for list_item in &decorations.list_items {
        if !marker_overlaps_block(&list_item.marker_range, &block_ranges)
            && !range_on_cursor_line(&list_item.marker_range, active_range)
        {
            let start = MultiBufferOffset(list_item.marker_range.start);
            let end = MultiBufferOffset(list_item.marker_range.end);
            creases.push(Crease::simple(start..end, bullet_placeholder.clone()));
        }
    }

    if !creases.is_empty() {
        editor.display_map.update(cx, |map, cx| map.fold(creases, cx));
        cx.notify();
    }
}

fn remove_stale_folds(editor: &mut Editor, cx: &mut Context<Editor>) {
    let type_id = TypeId::of::<WysiwygFoldTag>();
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let buffer_len = snapshot.len();
    let full_range = vec![MultiBufferOffset(0)..buffer_len];
    editor.remove_folds_with_type(&full_range, type_id, false, cx);
}

fn apply_blocks(
    editor: &mut Editor,
    snapshot: &MultiBufferSnapshot,
    decorations: &MarkdownDecorations,
    active_range: &Range<usize>,
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
        if range_on_cursor_line(&heading.line_range, active_range) {
            continue;
        }

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
            let clamped_level = level.min(4);
            let font_size_multiplier = match clamped_level {
                1 => 2.6_f32,
                2 => 2.08,
                3 => 1.69,
                _ => 1.43,
            };
            let base_size = block_context.em_width;
            let scaled_size = base_size * font_size_multiplier;
            let left_margin = block_context.anchor_x;

            gpui::div()
                .pl(left_margin)
                .text_size(scaled_size)
                .font_weight(FontWeight::BOLD)
                .line_height(scaled_size * 1.0)
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

    for (table_index, table) in decorations.tables.iter().enumerate() {
        if range_on_cursor_line(&table.range, active_range) {
            continue;
        }

        let start = snapshot.anchor_before(MultiBufferOffset(table.range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(table.range.end));

        let headers = table.headers.clone();
        let rows = table.rows.clone();
        let row_count = rows.len() as u32 + 1;

        let max_col_width_px: f32 = 50.0;
        let padding_px: f32 = 20.0;

        let column_count = headers.len();
        let mut column_widths: Vec<f32> = Vec::with_capacity(column_count);
        for (col_index, header) in headers.iter().enumerate() {
            let mut max_len = header.len();
            for row in &rows {
                if let Some(cell) = row.get(col_index) {
                    max_len = max_len.max(cell.len());
                }
            }
            let text_based_width = (max_len as f32) * 7.0 + padding_px;
            let clamped_width = text_based_width.min(max_col_width_px * 7.0);
            column_widths.push(clamped_width);
        }

        let render: RenderBlock = Arc::new(move |block_context: &mut BlockContext| {
            let border_color = Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.5,
                a: 0.3,
            };
            let left_margin = block_context.anchor_x;

            let mut inner_table = gpui::div()
                .flex()
                .flex_col()
                .border_1()
                .border_color(border_color);

            let mut header_row = gpui::div()
                .flex()
                .flex_row()
                .flex_shrink_0()
                .border_b_1()
                .border_color(border_color)
                .bg(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.3,
                    a: 0.1,
                });

            for (col_index, header) in headers.iter().enumerate() {
                let col_width = column_widths.get(col_index).copied().unwrap_or(100.0);
                header_row = header_row.child(
                    gpui::div()
                        .w(gpui::px(col_width))
                        .flex_shrink_0()
                        .px_2()
                        .py_1()
                        .font_weight(FontWeight::BOLD)
                        .border_r_1()
                        .border_color(border_color)
                        .child(SharedString::from(header.clone())),
                );
            }
            inner_table = inner_table.child(header_row);

            for row in &rows {
                let mut row_element = gpui::div()
                    .flex()
                    .flex_row()
                    .flex_shrink_0()
                    .border_b_1()
                    .border_color(border_color);

                for (col_index, cell) in row.iter().enumerate() {
                    let col_width = column_widths.get(col_index).copied().unwrap_or(100.0);
                    row_element = row_element.child(
                        gpui::div()
                            .w(gpui::px(col_width))
                            .flex_shrink_0()
                            .px_2()
                            .py_1()
                            .border_r_1()
                            .border_color(border_color)
                            .child(SharedString::from(cell.clone())),
                    );
                }
                inner_table = inner_table.child(row_element);
            }

            let table_total_width: f32 = column_widths.iter().sum();
            let scrollable_wrapper = gpui::div()
                .id(ElementId::Name(
                    SharedString::from(format!("wysiwyg-table-{}", table_index)),
                ))
                .overflow_x_scroll()
                .max_w(block_context.max_width * 0.7)
                .child(inner_table.w(gpui::px(table_total_width)));

            gpui::div()
                .pl(left_margin)
                .child(scrollable_wrapper)
                .into_any_element()
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
        if range_on_cursor_line(&image.range, active_range) {
            continue;
        }

        let start = snapshot.anchor_before(MultiBufferOffset(image.range.start));
        let end = snapshot.anchor_after(MultiBufferOffset(image.range.end));

        let url = image.url.clone();
        let is_local = !url.starts_with("http://") && !url.starts_with("https://");
        let resolved_path = if is_local {
            let path = PathBuf::from(&url);
            if path.exists() { Some(path) } else { None }
        } else {
            None
        };
        let image_width = image.width;
        let image_height = image.height;

        let render: RenderBlock = Arc::new(move |block_context: &mut BlockContext| {
            let left_margin = block_context.anchor_x;
            let mut image_element = if let Some(path) = &resolved_path {
                gpui::img(path.clone())
            } else {
                gpui::img(SharedString::from(url.clone()))
            };

            if let Some(width) = image_width {
                image_element = image_element.w(gpui::px(width as f32));
            }
            if let Some(height) = image_height {
                image_element = image_element.h(gpui::px(height as f32));
            }
            if image_width.is_none() && image_height.is_none() {
                image_element = image_element.max_w(block_context.max_width * 0.8);
            }

            gpui::div()
                .pl(left_margin)
                .py_1()
                .child(image_element)
                .into_any_element()
        });

        let height = 10;

        block_properties.push(BlockProperties {
            placement: BlockPlacement::Replace(start..=end),
            height: Some(height),
            style: BlockStyle::Flex,
            render,
            priority: 0,
        });
    }

    if !block_properties.is_empty() {
        let new_ids = editor.insert_blocks(block_properties, None, cx);
        editor.markdown_wysiwyg_state.block_ids = new_ids;
    }

    // Render references section if we have cached references
    apply_references_block(editor, snapshot, cx);
}

fn apply_references_block(
    editor: &mut Editor,
    snapshot: &MultiBufferSnapshot,
    cx: &mut Context<Editor>,
) {
    // Remove old references blocks
    let old_ref_ids: HashSet<CustomBlockId> = editor
        .markdown_wysiwyg_state
        .references_block_ids
        .drain(..)
        .collect();
    if !old_ref_ids.is_empty() {
        editor.remove_blocks(old_ref_ids, None, cx);
    }

    let references = editor.markdown_wysiwyg_state.cached_references.clone();
    eprintln!("[WYSIWYG-REF] apply_references_block called with {} references: {:?}", references.len(), references);
    if references.is_empty() {
        eprintln!("[WYSIWYG-REF] No references, returning early");
        return;
    }

    // Place the references block after the last line of the document
    let buffer_end = snapshot.len();
    let end_anchor = snapshot.anchor_after(buffer_end);

    let ref_count = references.len();
    let height = (ref_count as u32) + 2; // +2 for header and divider

    let render: RenderBlock = Arc::new(move |block_context: &mut BlockContext| {
        let left_margin = block_context.anchor_x;
        let border_color = Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.2,
        };

        let mut container = gpui::div()
            .pl(left_margin)
            .pt_4()
            .flex()
            .flex_col()
            .gap_1();

        // Divider line
        container = container.child(
            gpui::div()
                .h(gpui::px(1.0))
                .bg(border_color)
                .mb_2(),
        );

        // Header
        container = container.child(
            gpui::div()
                .text_size(block_context.em_width * 1.1)
                .font_weight(FontWeight::BOLD)
                .text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.6,
                    a: 1.0,
                })
                .mb_1()
                .child(SharedString::from("Linked Mentions")),
        );

        // Reference entries
        for ref_name in &references {
            container = container.child(
                gpui::div()
                    .flex()
                    .flex_row()
                    .gap_1()
                    .child(
                        gpui::div()
                            .text_color(Hsla {
                                h: 0.6,
                                s: 0.5,
                                l: 0.6,
                                a: 1.0,
                            })
                            .child(SharedString::from(format!("  {}", ref_name))),
                    ),
            );
        }

        container.into_any_element()
    });

    let block_properties = vec![BlockProperties {
        placement: BlockPlacement::Below(end_anchor),
        height: Some(height),
        style: BlockStyle::Flex,
        render,
        priority: 0,
    }];

    let new_ids = editor.insert_blocks(block_properties, None, cx);
    eprintln!("[WYSIWYG-REF] Inserted {} reference blocks", new_ids.len());
    editor.markdown_wysiwyg_state.references_block_ids = new_ids;
}

/// Fetches references for the current document from the LSP and caches them.
/// This is called when WYSIWYG mode is toggled on or when the document changes.
pub fn fetch_references(editor: &mut Editor, cx: &mut Context<Editor>) {
    if !editor.markdown_wysiwyg_state.active {
        return;
    }

    // Get the current file's absolute path and stem name
    let multi_buffer = editor.buffer().read(cx);
    let buffers = multi_buffer.all_buffers();
    let mut current_file_stem: Option<String> = None;
    let mut workspace_dir: Option<PathBuf> = None;

    for buffer in buffers {
        let buffer_read = buffer.read(cx);
        if let Some(file) = buffer_read.file() {
            current_file_stem = file.path().file_stem().map(|s| s.to_string());
            if let Some(local_file) = file.as_local() {
                let abs = local_file.abs_path(cx);
                workspace_dir = abs.parent().map(|p| p.to_path_buf());
            }
            break;
        }
    }

    let file_stem = match current_file_stem {
        Some(s) => s,
        None => {
            eprintln!("[WYSIWYG-REF] No current file stem found");
            return;
        }
    };
    let dir = match workspace_dir {
        Some(d) => d,
        None => {
            eprintln!("[WYSIWYG-REF] No workspace dir found");
            return;
        }
    };
    eprintln!("[WYSIWYG-REF] Scanning dir {:?} for backlinks to {:?}", dir, file_stem);

    // Scan all .md files in the workspace directory for wikilinks to this file
    editor.markdown_wysiwyg_state.references_task = cx.spawn(async move |editor, cx| {
        let mut backlink_files: Vec<String> = Vec::new();

        // Read all .md files in the directory
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("md") {
                    continue;
                }
                // Skip the current file
                let entry_stem = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                if entry_stem == file_stem {
                    continue;
                }

                // Read the file and check for wikilinks to the current file
                if let Ok(content) = std::fs::read_to_string(&path) {
                    // Look for [[file_stem]] or [[file_stem|display]] or [[file_stem#section]]
                    let pattern_simple = format!("[[{}]]", file_stem);
                    let pattern_pipe = format!("[[{}|", file_stem);
                    let pattern_hash = format!("[[{}#", file_stem);

                    if content.contains(&pattern_simple)
                        || content.contains(&pattern_pipe)
                        || content.contains(&pattern_hash)
                    {
                        backlink_files.push(entry_stem);
                    }
                }
            }
        }

        backlink_files.sort();
        eprintln!("[WYSIWYG-REF] Found {} backlink files: {:?}", backlink_files.len(), backlink_files);

        editor.update(cx, |editor, cx| {
            editor.markdown_wysiwyg_state.cached_references = backlink_files;
            if editor.markdown_wysiwyg_state.active {
                refresh_wysiwyg_decorations(editor, cx);
            }
        }).ok();
    });
}

fn clear_wysiwyg_decorations(editor: &mut Editor, cx: &mut Context<Editor>) {
    editor.clear_highlights(HighlightKey::MarkdownWysiwygBold, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygItalic, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygStrikethrough, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygCode, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygHeading, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygMarker, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygWikilink, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygUnresolvedLink, cx);

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

    let old_ref_ids: HashSet<CustomBlockId> = editor
        .markdown_wysiwyg_state
        .references_block_ids
        .drain(..)
        .collect();
    if !old_ref_ids.is_empty() {
        editor.remove_blocks(old_ref_ids, None, cx);
    }
    editor.markdown_wysiwyg_state.cached_references.clear();
}

pub fn on_selection_changed(editor: &mut Editor, cx: &mut Context<Editor>) {
    if !editor.markdown_wysiwyg_state.active {
        return;
    }
    refresh_wysiwyg_decorations(editor, cx);
}

/// Checks if the cursor is on a wikilink in WYSIWYG mode, and if so,
/// triggers go-to-definition. Returns true if a wikilink click was handled.
pub fn try_handle_wikilink_click(
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    if !editor.markdown_wysiwyg_state.active {
        return false;
    }

    let snapshot = editor.buffer().read(cx).snapshot(cx);
    let text = snapshot.text();
    let cursor = cursor_offset(editor, cx);

    // Check if cursor is inside a wikilink
    let wikilinks = parse_wikilinks(&text);
    for wikilink in &wikilinks {
        if cursor >= wikilink.range.start && cursor <= wikilink.range.end {
            // Cursor is inside a wikilink — trigger go-to-definition
            // Position cursor at the link content (inside [[ ]])
            let content_start = wikilink.range.start + 2;
            let link_end = wikilink.range.end - 2;
            let inner = &text[content_start..link_end];
            // If there's a pipe, position at the link part (before pipe)
            let link_target_end = if let Some(pipe_pos) = inner.find('|') {
                content_start + pipe_pos
            } else {
                link_end
            };
            // Move cursor to middle of the link target for go-to-definition
            let target_pos = (content_start + link_target_end) / 2;
            let anchor = snapshot.anchor_before(MultiBufferOffset(target_pos));

            // Set the cursor position to be inside the wikilink content
            use crate::SelectionEffects;
            editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
                selections.select_anchor_ranges([anchor..anchor]);
            });

            // Now trigger go-to-definition
            use crate::GoToDefinition;
            let _ = editor.go_to_definition(&GoToDefinition, window, cx);
            return true;
        }
    }

    false
}

/// Attempts to handle an image paste when WYSIWYG mode is active.
/// Returns true if an image was handled, false otherwise (so normal paste continues).
pub fn try_handle_image_paste(
    editor: &mut Editor,
    image: &gpui::Image,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    if !editor.markdown_wysiwyg_state.active {
        return false;
    }

    let extension = match image.format {
        gpui::ImageFormat::Png => "png",
        gpui::ImageFormat::Jpeg => "jpg",
        gpui::ImageFormat::Webp => "webp",
        gpui::ImageFormat::Gif => "gif",
        _ => "png",
    };

    // Get the workspace root directory from the buffer's file
    let workspace_root = {
        let multi_buffer = editor.buffer().read(cx);
        let buffers = multi_buffer.all_buffers();
        let mut root_path: Option<PathBuf> = None;
        for buffer in buffers {
            let buffer_read = buffer.read(cx);
            if let Some(file) = buffer_read.file() {
                if let Some(local_file) = file.as_local() {
                    let abs = local_file.abs_path(cx);
                    root_path = abs.parent().map(|p| p.to_path_buf());
                    break;
                }
            }
        }
        root_path
    };

    let save_dir = match workspace_root {
        Some(dir) => dir,
        None => return false,
    };

    // Generate a unique filename using a timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let filename = format!("pasted-image-{}.{}", timestamp, extension);
    let save_path = save_dir.join(&filename);

    // Write the image bytes to disk
    if std::fs::write(&save_path, &image.bytes).is_err() {
        return false;
    }

    // Insert a wikilink image reference at the cursor
    let wikilink_text = format!("![[{}]]", filename);
    editor.insert(&wikilink_text, window, cx);

    // Trigger a WYSIWYG refresh so the image renders immediately
    schedule_wysiwyg_refresh(editor, cx);

    true
}
