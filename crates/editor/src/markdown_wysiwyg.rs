use crate::display_map::HighlightKey;
use crate::Editor;
use gpui::{Context, FontStyle, FontWeight, HighlightStyle, Hsla, Task};
use multi_buffer::{MultiBufferOffset, MultiBufferSnapshot};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::ops::Range;
use std::time::Duration;

const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub struct MarkdownWysiwygState {
    pub active: bool,
    reparse_task: Task<()>,
}

impl MarkdownWysiwygState {
    pub fn new() -> Self {
        Self {
            active: false,
            reparse_task: Task::ready(()),
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
}

struct MarkdownDecorations {
    inline_decorations: Vec<InlineDecoration>,
    headings: Vec<HeadingDecoration>,
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

    let mut bold_start: Option<usize> = None;
    let mut italic_start: Option<usize> = None;
    let mut strikethrough_start: Option<usize> = None;
    let mut heading_start: Option<(u8, usize)> = None;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Strong) => {
                bold_start = Some(range.start);
            }
            Event::End(TagEnd::Strong) => {
                if let Some(start) = bold_start.take() {
                    inline_decorations.push(InlineDecoration {
                        content_range: start..range.end,
                        kind: DecorationKind::Bold,
                    });
                }
            }
            Event::Start(Tag::Emphasis) => {
                italic_start = Some(range.start);
            }
            Event::End(TagEnd::Emphasis) => {
                if let Some(start) = italic_start.take() {
                    inline_decorations.push(InlineDecoration {
                        content_range: start..range.end,
                        kind: DecorationKind::Italic,
                    });
                }
            }
            Event::Start(Tag::Strikethrough) => {
                strikethrough_start = Some(range.start);
            }
            Event::End(TagEnd::Strikethrough) => {
                if let Some(start) = strikethrough_start.take() {
                    inline_decorations.push(InlineDecoration {
                        content_range: start..range.end,
                        kind: DecorationKind::Strikethrough,
                    });
                }
            }
            Event::Code(_) => {
                inline_decorations.push(InlineDecoration {
                    content_range: range.start..range.end,
                    kind: DecorationKind::InlineCode,
                });
            }
            Event::Start(Tag::Heading { level, .. }) => {
                heading_start = Some((level as u8, range.start));
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((_lvl, start)) = heading_start.take() {
                    headings.push(HeadingDecoration {
                        line_range: start..range.end,
                    });
                }
            }
            _ => {}
        }
    }

    MarkdownDecorations {
        inline_decorations,
        headings,
    }
}

fn is_markdown_file(buffer: &language::Buffer) -> bool {
    if let Some(language) = buffer.language() {
        let name = language.name();
        return name.as_ref() == "Markdown";
    }
    if let Some(file) = buffer.file() {
        if let Some(ext) = file.path().extension() {
            return ext == "md" || ext == "markdown" || ext == "mdx";
        }
    }
    false
}

pub fn check_and_activate_wysiwyg(editor: &mut Editor, cx: &mut Context<Editor>) {
    let should_activate = editor
        .buffer()
        .read(cx)
        .as_singleton()
        .map(|buffer| is_markdown_file(&buffer.read(cx)))
        .unwrap_or(false);

    if should_activate && !editor.markdown_wysiwyg_state.active {
        editor.markdown_wysiwyg_state.active = true;
        refresh_wysiwyg_decorations(editor, cx);
    } else if !should_activate && editor.markdown_wysiwyg_state.active {
        clear_wysiwyg_decorations(editor, cx);
        editor.markdown_wysiwyg_state.active = false;
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

fn clear_wysiwyg_decorations(editor: &mut Editor, cx: &mut Context<Editor>) {
    editor.clear_highlights(HighlightKey::MarkdownWysiwygBold, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygItalic, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygStrikethrough, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygCode, cx);
    editor.clear_highlights(HighlightKey::MarkdownWysiwygHeading, cx);
}

pub fn on_selection_changed(editor: &mut Editor, cx: &mut Context<Editor>) {
    if !editor.markdown_wysiwyg_state.active {
        return;
    }
    refresh_wysiwyg_decorations(editor, cx);
}
