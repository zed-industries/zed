//! `Editor` — the workhorse entity. It owns the cursor, blink, focus, keyboard
//! handling, and the specialized text-shaping renderer. The *text itself* lives
//! in a shared `Entity<String>` it's handed at construction, so the value is
//! readable/writable from outside while the editing machinery stays in here.
//!
//! This is the piece that proves the point: a text input is genuinely
//! complicated, and `View` lets all of that complexity live in one entity that
//! anything can embed.

use std::ops::Range;
use std::time::Duration;

use gpui::{
    App, Bounds, Context, ElementInputHandler, Entity, EntityInputHandler, FocusHandle, Focusable,
    InteractiveElement, LayoutId, PaintQuad, Pixels, ShapedLine, SharedString, Subscription, Task,
    TextRun, UTF16Selection, Window, fill, hsla, point, prelude::*, px, relative, size,
};
use unicode_segmentation::*;

use crate::{Backspace, Delete, End, Home, Left, Right};

pub struct Editor {
    pub value: Entity<String>,
    pub focus_handle: FocusHandle,
    pub cursor: usize,
    pub cursor_visible: bool,
    _blink_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl Editor {
    /// An editor that owns its own string internally, seeded with `text`.
    /// Nothing to allocate or wire up at the call site.
    pub fn new(text: impl Into<String>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let value = cx.new(|_| text.into());
        Self::over(value, window, cx)
    }

    /// An editor over a string *you* own, so the value is shared in and out.
    pub fn over(value: Entity<String>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let focus_sub = cx.on_focus(&focus_handle, window, |this, _window, cx| {
            this.start_blink(cx);
        });
        let blur_sub = cx.on_blur(&focus_handle, window, |this, _window, cx| {
            this.stop_blink(cx);
        });

        // The value is shared: anything can write it while we hold a cursor into
        // it. Observe it so external writes (a) clamp the cursor back onto a char
        // boundary before the next IME round-trip can slice out of bounds, and
        // (b) notify us, so an `editor.cached(..)` subtree re-renders — the cache
        // is keyed on *our* notify, not the value's.
        let value_sub = cx.observe(&value, |this, value, cx| {
            let content = value.read(cx);
            let mut cursor = this.cursor.min(content.len());
            while cursor > 0 && !content.is_char_boundary(cursor) {
                cursor -= 1;
            }
            this.cursor = cursor;
            cx.notify();
        });

        Self {
            value,
            focus_handle,
            cursor: 0,
            cursor_visible: false,
            _blink_task: Task::ready(()),
            _subscriptions: vec![focus_sub, blur_sub, value_sub],
        }
    }

    /// The current text. Read this from anywhere to get the value out.
    pub fn text(&self, cx: &App) -> String {
        self.value.read(cx).clone()
    }

    fn start_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        self._blink_task = Self::spawn_blink_task(cx);
    }

    fn stop_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = false;
        self._blink_task = Task::ready(());
        cx.notify();
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

    fn reset_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        self._blink_task = Self::spawn_blink_task(cx);
    }

    pub fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        let content = self.text(cx);
        if self.cursor > 0 {
            self.cursor = previous_boundary(&content, self.cursor);
        }
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        let content = self.text(cx);
        if self.cursor < content.len() {
            self.cursor = next_boundary(&content, self.cursor);
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
        self.cursor = self.text(cx).len();
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        let content = self.text(cx);
        if self.cursor > 0 {
            let prev = previous_boundary(&content, self.cursor);
            let cursor = self.cursor;
            self.value.update(cx, |s, cx| {
                s.drain(prev..cursor);
                cx.notify();
            });
            self.cursor = prev;
        }
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        let content = self.text(cx);
        if self.cursor < content.len() {
            let next = next_boundary(&content, self.cursor);
            let cursor = self.cursor;
            self.value.update(cx, |s, cx| {
                s.drain(cursor..next);
                cx.notify();
            });
        }
        self.reset_blink(cx);
        cx.notify();
    }

    pub fn insert_newline(&mut self, cx: &mut Context<Self>) {
        let cursor = self.cursor;
        self.value.update(cx, |s, cx| {
            s.insert(cursor, '\n');
            cx.notify();
        });
        self.cursor += 1;
        self.reset_blink(cx);
        cx.notify();
    }
}

fn previous_boundary(content: &str, offset: usize) -> usize {
    content
        .grapheme_indices(true)
        .rev()
        .find_map(|(idx, _)| (idx < offset).then_some(idx))
        .unwrap_or(0)
}

fn next_boundary(content: &str, offset: usize) -> usize {
    content
        .grapheme_indices(true)
        .find_map(|(idx, _)| (idx > offset).then_some(idx))
        .unwrap_or(content.len())
}

fn offset_from_utf16(content: &str, offset: usize) -> usize {
    let mut utf8_offset = 0;
    let mut utf16_count = 0;
    for ch in content.chars() {
        if utf16_count >= offset {
            break;
        }
        utf16_count += ch.len_utf16();
        utf8_offset += ch.len_utf8();
    }
    utf8_offset
}

fn offset_to_utf16(content: &str, offset: usize) -> usize {
    let mut utf16_offset = 0;
    let mut utf8_count = 0;
    for ch in content.chars() {
        if utf8_count >= offset {
            break;
        }
        utf8_count += ch.len_utf8();
        utf16_offset += ch.len_utf16();
    }
    utf16_offset
}

fn range_to_utf16(content: &str, range: &Range<usize>) -> Range<usize> {
    offset_to_utf16(content, range.start)..offset_to_utf16(content, range.end)
}

fn range_from_utf16(content: &str, range_utf16: &Range<usize>) -> Range<usize> {
    offset_from_utf16(content, range_utf16.start)..offset_from_utf16(content, range_utf16.end)
}

impl Focusable for Editor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for Editor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        let content = self.text(cx);
        let range = range_from_utf16(&content, &range_utf16);
        actual_range.replace(range_to_utf16(&content, &range));
        Some(content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let content = self.text(cx);
        let utf16_cursor = offset_to_utf16(&content, self.cursor);
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
        let content = self.text(cx);
        let range = range_utf16
            .as_ref()
            .map(|r| range_from_utf16(&content, r))
            .unwrap_or(self.cursor..self.cursor);

        let new_content = content[..range.start].to_owned() + new_text + &content[range.end..];
        self.cursor = range.start + new_text.len();
        self.value.update(cx, |s, cx| {
            *s = new_content;
            cx.notify();
        });
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

impl gpui::Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Editor>) -> impl IntoElement {
        EditorText {
            editor: cx.entity(),
        }
    }
}

// ---------------------------------------------------------------------------
// EditorText — the specialized renderer: shapes the text and paints the cursor.
// ---------------------------------------------------------------------------

struct EditorText {
    editor: Entity<Editor>,
}

struct EditorTextPrepaint {
    lines: Vec<ShapedLine>,
    cursor: Option<PaintQuad>,
}

impl IntoElement for EditorText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for EditorText {
    type RequestLayoutState = ();
    type PrepaintState = EditorTextPrepaint;

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
        let editor = self.editor.read(cx);
        let content = editor.value.read(cx);
        let line_count = content.split('\n').count().max(1);
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
        let content = editor.value.read(cx).clone();
        let cursor_offset = editor.cursor;
        let cursor_visible = editor.cursor_visible;
        let is_focused = editor.focus_handle.is_focused(window);

        let style = window.text_style();
        let text_color = style.color;
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();

        let is_placeholder = content.is_empty();

        let lines: Vec<ShapedLine> = if is_placeholder {
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
                        color: text_color,
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
            let (cursor_line, offset_in_line) = cursor_line_and_offset(&content, cursor_offset);
            let cursor_line = cursor_line.min(lines.len().saturating_sub(1));
            let cursor_x = lines[cursor_line].x_for_index(offset_in_line);
            Some(fill(
                Bounds::new(
                    point(
                        bounds.left() + cursor_x,
                        bounds.top() + line_height * cursor_line as f32,
                    ),
                    size(px(1.5), line_height),
                ),
                text_color,
            ))
        } else if is_focused && cursor_visible && is_placeholder {
            Some(fill(
                Bounds::new(
                    point(bounds.left(), bounds.top()),
                    size(px(1.5), line_height),
                ),
                text_color,
            ))
        } else {
            None
        };

        EditorTextPrepaint { lines, cursor }
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

pub fn standard_actions<E: InteractiveElement>(editor: Entity<Editor>) -> impl FnOnce(E) -> E {
    move |element| {
        element
            .on_action({
                let editor = editor.clone();
                move |a: &Left, window, cx| editor.update(cx, |e, cx| e.left(a, window, cx))
            })
            .on_action({
                let editor = editor.clone();
                move |a: &Right, window, cx| editor.update(cx, |e, cx| e.right(a, window, cx))
            })
            .on_action({
                let editor = editor.clone();
                move |a: &Home, window, cx| editor.update(cx, |e, cx| e.home(a, window, cx))
            })
            .on_action({
                let editor = editor.clone();
                move |a: &End, window, cx| editor.update(cx, |e, cx| e.end(a, window, cx))
            })
            .on_action({
                let editor = editor.clone();
                move |a: &Backspace, window, cx| {
                    editor.update(cx, |e, cx| e.backspace(a, window, cx))
                }
            })
            .on_action(move |a: &Delete, window, cx| {
                editor.update(cx, |e, cx| e.delete(a, window, cx))
            })
    }
}
