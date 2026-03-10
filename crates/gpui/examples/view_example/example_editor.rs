//! The `ExampleEditor` entity — owns the truth about text content, cursor position,
//! blink state, and keyboard handling.
//!
//! Also contains `ExampleEditorText`, the low-level custom `Element` that shapes text
//! and paints the cursor, and `ExampleEditorView`, a cached `View` wrapper that
//! automatically pairs an `ExampleEditor` entity with its `ExampleEditorText` element.

use std::hash::Hash;
use std::ops::Range;
use std::time::Duration;

use gpui::{
    App, Bounds, Context, ElementInputHandler, Entity, EntityInputHandler, FocusHandle, Focusable,
    Hsla, IntoViewElement, LayoutId, PaintQuad, Pixels, ShapedLine, SharedString, Task, TextRun,
    UTF16Selection, Window, fill, hsla, point, prelude::*, px, relative, size,
};
use unicode_segmentation::*;

use crate::{Backspace, Delete, End, Home, Left, Right};

pub struct ExampleEditor {
    pub focus_handle: FocusHandle,
    pub content: String,
    pub cursor: usize,
    pub cursor_visible: bool,
    _blink_task: Task<()>,
}

impl ExampleEditor {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let blink_task = Self::spawn_blink_task(cx);

        Self {
            focus_handle: cx.focus_handle(),
            content: String::new(),
            cursor: 0,
            cursor_visible: true,
            _blink_task: blink_task,
        }
    }

    fn spawn_blink_task(cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(500))
                    .await;
                let result = this.update(cx, |editor, cx| {
                    editor.cursor_visible = !editor.cursor_visible;
                    cx.notify();
                });
                if result.is_err() {
                    break;
                }
            }
        })
    }

    pub fn reset_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        self._blink_task = Self::spawn_blink_task(cx);
    }

    pub fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor > 0 {
            self.cursor = self.previous_boundary(self.cursor);
        }
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor < self.content.len() {
            self.cursor = self.next_boundary(self.cursor);
        }
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor = 0;
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.cursor = self.content.len();
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor > 0 {
            let prev = self.previous_boundary(self.cursor);
            self.content.drain(prev..self.cursor);
            self.cursor = prev;
        }
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.cursor < self.content.len() {
            let next = self.next_boundary(self.cursor);
            self.content.drain(self.cursor..next);
        }
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn insert_newline(&mut self, cx: &mut Context<Self>) {
        self.content.insert(self.cursor, '\n');
        self.cursor += 1;
        self.reset_blink(cx);
        cx.notify();
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }
}

impl Focusable for ExampleEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for ExampleEditor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let utf16_cursor = self.offset_to_utf16(self.cursor);
        Some(UTF16Selection {
            range: utf16_cursor..utf16_cursor,
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|r| self.range_from_utf16(r))
            .unwrap_or(self.cursor..self.cursor);

        self.content =
            self.content[..range.start].to_owned() + new_text + &self.content[range.end..];
        self.cursor = range.start + new_text.len();
        self.reset_blink(cx);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.replace_text_in_range(range_utf16, new_text, window, cx);
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        _point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        None
    }
}

// ---------------------------------------------------------------------------
// ExampleEditorText — custom Element that shapes text & paints the cursor
// ---------------------------------------------------------------------------

struct ExampleEditorText {
    editor: Entity<ExampleEditor>,
    text_color: Hsla,
}

struct ExampleEditorTextPrepaintState {
    lines: Vec<ShapedLine>,
    cursor: Option<PaintQuad>,
}

impl ExampleEditorText {
    pub fn new(editor: Entity<ExampleEditor>, text_color: Hsla) -> Self {
        Self { editor, text_color }
    }
}

impl IntoElement for ExampleEditorText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ExampleEditorText {
    type RequestLayoutState = ();
    type PrepaintState = ExampleEditorTextPrepaintState;

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let line_count = self.editor.read(cx).content.split('\n').count().max(1);
        let line_height = window.line_height();
        let mut style = gpui::Style::default();
        style.size.width = relative(1.).into();
        style.size.height = (line_height * line_count as f32).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let editor = self.editor.read(cx);
        let content = &editor.content;
        let cursor_offset = editor.cursor;
        let cursor_visible = editor.cursor_visible;
        let is_focused = editor.focus_handle.is_focused(window);

        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();

        let is_placeholder = content.is_empty();

        let shaped_lines: Vec<ShapedLine> = if is_placeholder {
            let placeholder: SharedString = "Type here...".into();
            let run = TextRun {
                len: placeholder.len(),
                font: style.font(),
                color: hsla(0., 0., 0.5, 0.5),
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            vec![
                window
                    .text_system()
                    .shape_line(placeholder, font_size, &[run], None),
            ]
        } else {
            content
                .split('\n')
                .map(|line_str| {
                    let text: SharedString = SharedString::from(line_str.to_string());
                    let run = TextRun {
                        len: text.len(),
                        font: style.font(),
                        color: self.text_color,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    };
                    window
                        .text_system()
                        .shape_line(text, font_size, &[run], None)
                })
                .collect()
        };

        let cursor = if is_focused && cursor_visible && !is_placeholder {
            let (cursor_line, offset_in_line) = cursor_line_and_offset(content, cursor_offset);
            let cursor_line = cursor_line.min(shaped_lines.len().saturating_sub(1));
            let cursor_x = shaped_lines[cursor_line].x_for_index(offset_in_line);

            Some(fill(
                Bounds::new(
                    point(
                        bounds.left() + cursor_x,
                        bounds.top() + line_height * cursor_line as f32,
                    ),
                    size(px(1.5), line_height),
                ),
                self.text_color,
            ))
        } else if is_focused && cursor_visible && is_placeholder {
            Some(fill(
                Bounds::new(
                    point(bounds.left(), bounds.top()),
                    size(px(1.5), line_height),
                ),
                self.text_color,
            ))
        } else {
            None
        };

        ExampleEditorTextPrepaintState {
            lines: shaped_lines,
            cursor,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.editor.read(cx).focus_handle.clone();

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );

        let line_height = window.line_height();
        for (i, line) in prepaint.lines.iter().enumerate() {
            let origin = point(bounds.left(), bounds.top() + line_height * i as f32);
            line.paint(origin, line_height, gpui::TextAlign::Left, None, window, cx)
                .unwrap();
        }

        if let Some(cursor) = prepaint.cursor.take() {
            window.paint_quad(cursor);
        }
    }
}

fn cursor_line_and_offset(content: &str, cursor: usize) -> (usize, usize) {
    let mut line_index = 0;
    let mut line_start = 0;
    for (i, ch) in content.char_indices() {
        if i >= cursor {
            break;
        }
        if ch == '\n' {
            line_index += 1;
            line_start = i + 1;
        }
    }
    (line_index, cursor - line_start)
}

// ---------------------------------------------------------------------------
// ExampleEditorView — a cached View that pairs an ExampleEditor entity with ExampleEditorText
// ---------------------------------------------------------------------------

/// A simple cached view that renders an `ExampleEditor` entity via the `ExampleEditorText`
/// custom element. Use this when you want a bare editor display with automatic
/// caching and no extra chrome.
#[derive(IntoViewElement, Hash)]
pub struct ExampleEditorView {
    editor: Entity<ExampleEditor>,
    text_color: Hsla,
}

impl ExampleEditorView {
    pub fn new(editor: Entity<ExampleEditor>) -> Self {
        Self {
            editor,
            text_color: hsla(0., 0., 0.1, 1.),
        }
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = color;
        self
    }
}

impl gpui::View for ExampleEditorView {
    type Entity = ExampleEditor;

    fn entity(&self) -> Option<Entity<ExampleEditor>> {
        Some(self.editor.clone())
    }

    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ExampleEditorText::new(self.editor, self.text_color)
    }
}
