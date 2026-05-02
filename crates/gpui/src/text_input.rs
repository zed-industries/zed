//! A built-in single-line text input widget.
//!
//! [`TextInput`] is the canonical reusable input GPUI ships out of the
//! box — most apps shouldn't need to roll their own. It hooks GPUI's
//! [`EntityInputHandler`] / [`PlatformInputHandler`] bridge so the
//! platform's IME (macOS NSTextInput, Linux IBus / IME-kit, Windows
//! TSF, Android `TextInputState`, web `<input>`) drives the field
//! without per-platform widget code.
//!
//! ## Usage
//!
//! ```ignore
//! use gpui::{Context, Entity, ImeKind, ScrollAnchor, ScrollHandle, TextInput, div, prelude::*};
//!
//! struct Form {
//!     name: Entity<TextInput>,
//!     amount: Entity<TextInput>,
//!     scroll: ScrollHandle,
//! }
//!
//! impl Form {
//!     fn new(cx: &mut Context<Self>) -> Self {
//!         let scroll = ScrollHandle::new();
//!         let name_anchor = ScrollAnchor::for_handle(scroll.clone());
//!         let amount_anchor = ScrollAnchor::for_handle(scroll.clone());
//!         let name = cx.new(|cx| {
//!             TextInput::new(cx)
//!                 .placeholder("Your name")
//!                 .scroll_anchor(name_anchor)
//!         });
//!         let amount = cx.new(|cx| {
//!             TextInput::new(cx)
//!                 .placeholder("Amount")
//!                 .ime_kind(ImeKind::Number)
//!                 .scroll_anchor(amount_anchor)
//!         });
//!         cx.observe(&name, |_, _, cx| cx.notify()).detach();
//!         cx.observe(&amount, |_, _, cx| cx.notify()).detach();
//!         Self { name, amount, scroll }
//!     }
//! }
//! ```
//!
//! ## What you get
//!
//! - Tap / click to focus → soft keyboard pops on Android (no extra glue
//!   needed), or hardware keyboard input on desktop.
//! - Composition / preedit support (CJK IMEs, voice input,
//!   autocorrect): handled through [`EntityInputHandler::replace_and_mark_text_in_range`].
//! - Selection, caret, placeholder, theme.
//! - Optional [`ScrollAnchor`] so tapping the field scrolls it into the
//!   parent scrollable's viewport — important on mobile to keep the
//!   field above the soft keyboard.
//! - Optional [`ImeKind`] hint so soft keyboards open with the right
//!   layout (numeric pad, email keyboard, …). On desktop the hint is a
//!   no-op (hardware keyboards have one layout).
//!
//! ## What you *don't* get out of the box
//!
//! Multi-line editing, syntax highlighting, drag-select, undo/redo,
//! keyboard-shortcut-driven cursor movement (cmd-left, etc.) — these
//! belong in higher-level editor widgets. `TextInput` covers the
//! "form field" use case; for an editor surface use `editor::Editor`.

use std::ops::Range;

use crate::{
    App, Bounds, Context, Element, ElementId, ElementInputHandler, Entity, EntityInputHandler,
    FocusHandle, Focusable, GlobalElementId, Hsla, ImeKind, IntoElement, LayoutId, MouseButton,
    MouseDownEvent, PaintQuad, Pixels, Point, Render, ScrollAnchor, ShapedLine, SharedString,
    Style, TextRun, UTF16Selection, Window, div, fill, hsla, point, prelude::*, px, relative, rgb,
    size,
};

/// Visual theme for a [`TextInput`]. All colours are in GPUI's `Hsla`.
/// Use [`TextInputTheme::default`] for the bundled neutral dark theme,
/// or construct one explicitly to integrate with your app's palette.
#[derive(Clone, Copy, Debug)]
pub struct TextInputTheme {
    /// Text colour for entered content.
    pub text: Hsla,
    /// Text colour for the placeholder shown when content is empty.
    pub placeholder: Hsla,
    /// Background fill for the unfocused field.
    pub background: Hsla,
    /// Background fill for the focused field.
    pub background_focused: Hsla,
    /// Border colour for the unfocused field.
    pub border: Hsla,
    /// Border colour for the focused field.
    pub border_focused: Hsla,
    /// Caret colour, painted only while the field is focused.
    pub cursor: Hsla,
    /// Selection-highlight fill for selected text.
    pub selection: Hsla,
}

impl Default for TextInputTheme {
    fn default() -> Self {
        Self {
            text: hsla(0.0, 0.0, 0.95, 1.0),
            placeholder: hsla(0.0, 0.0, 0.6, 1.0),
            background: rgb(0x111418).into(),
            background_focused: rgb(0x1a1f29).into(),
            border: rgb(0x4b5563).into(),
            border_focused: rgb(0x60a5fa).into(),
            cursor: hsla(0.0, 0.0, 0.95, 1.0),
            selection: hsla(214.0 / 360.0, 0.84, 0.6, 0.35),
        }
    }
}

/// Single-line text input that participates in GPUI's IME bridge.
///
/// See the module-level docs for usage. Construct via [`TextInput::new`]
/// and configure with the chained builder methods. Read / write the
/// current value through [`TextInput::content`] /
/// [`TextInput::set_content`].
pub struct TextInput {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    ime_kind: ImeKind,
    theme: TextInputTheme,
    /// Selection range in **UTF-8 byte offsets** (not UTF-16 code units —
    /// IME ranges are translated through `range_from_utf16`).
    selected_range: Range<usize>,
    /// Pending IME composition region, in UTF-8 byte offsets.
    marked_range: Option<Range<usize>>,
    /// Cached layout from the last paint, used by [`bounds_for_range`]
    /// and [`character_index_for_point`].
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    /// Optional [`ScrollAnchor`] so taps auto-scroll the field into the
    /// parent's viewport. Required on mobile to keep the field above
    /// the soft keyboard.
    scroll_anchor: Option<ScrollAnchor>,
}

impl TextInput {
    /// Allocate a new field. Wrap the result in `cx.new(|cx| TextInput::new(cx))`
    /// to obtain an `Entity<TextInput>` and configure with the builder
    /// methods on the resulting binding.
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: SharedString::default(),
            placeholder: SharedString::default(),
            ime_kind: ImeKind::Text,
            theme: TextInputTheme::default(),
            selected_range: 0..0,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            scroll_anchor: None,
        }
    }

    /// Set the placeholder text shown while [`content`](Self::content)
    /// is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the IME kind hint requested when the field gains focus. On
    /// soft-keyboard platforms this drives the keyboard layout; on
    /// desktop it's a no-op. Defaults to [`ImeKind::Text`].
    pub fn ime_kind(mut self, kind: ImeKind) -> Self {
        self.ime_kind = kind;
        self
    }

    /// Override the visual theme. Defaults to [`TextInputTheme::default`].
    pub fn theme(mut self, theme: TextInputTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Bind a [`ScrollAnchor`] so the field auto-scrolls itself into
    /// its parent's viewport whenever it takes focus. The scrollable
    /// container should `track_scroll(&handle)` with the same handle
    /// the anchor was built from.
    ///
    /// Without an anchor wired up, opening the soft keyboard on mobile
    /// can leave the focused field hidden behind the IME — the platform
    /// shrinks the window bounds, but the existing scroll offset of
    /// the parent doesn't auto-adjust.
    pub fn scroll_anchor(mut self, anchor: ScrollAnchor) -> Self {
        self.scroll_anchor = Some(anchor);
        self
    }

    /// Read the field's current content.
    pub fn content(&self) -> &SharedString {
        &self.content
    }

    /// Replace the field's content programmatically. Resets the caret
    /// to the end and clears any in-flight IME composition. Pair with
    /// `cx.notify()` to schedule a redraw.
    pub fn set_content(&mut self, content: impl Into<SharedString>) {
        self.content = content.into();
        let len = self.content.len();
        self.selected_range = len..len;
        self.marked_range = None;
    }

    /// Read the field's IME kind hint.
    pub fn current_ime_kind(&self) -> ImeKind {
        self.ime_kind
    }

    fn cursor_offset(&self) -> usize {
        self.selected_range.end.min(self.content.len())
    }

    fn on_mouse_down(
        &mut self,
        _event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 1. Focus the field — GPUI's IME bridge sees the new focus
        //    and asks the platform to surface the keyboard / wire up
        //    the input handler.
        window.focus(&self.focus_handle, cx);
        // 2. Tell the platform what keyboard layout to request. On
        //    desktop this is a no-op; on Android / iOS / web it picks
        //    the right layout (numeric pad, email keyboard, …).
        window.set_ime_kind(self.ime_kind);
        // 3. Schedule a scroll-into-view if a [`ScrollAnchor`] was
        //    wired. The soft-keyboard slide-up can take ~250ms during
        //    which window bounds shrink, so re-fire across the next
        //    handful of frames — one of them lands after the IME has
        //    settled and the parent's bounds are correct.
        if let Some(anchor) = self.scroll_anchor.clone() {
            for _ in 0..18 {
                anchor.scroll_to(window, cx);
            }
        }
        cx.notify();
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

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    /// Walk down to the nearest UTF-8 char boundary on or before `idx`,
    /// clamping to `[0, content.len()]`. Used to keep stale selection
    /// or marked ranges from panicking when sliced after a content
    /// change (e.g. an IME delta arriving for a position that no
    /// longer exists).
    fn snap_to_boundary(&self, idx: usize) -> usize {
        let len = self.content.len();
        let mut idx = idx.min(len);
        while idx > 0 && !self.content.is_char_boundary(idx) {
            idx -= 1;
        }
        idx
    }

    fn clamp_range(&self, range: Range<usize>) -> Range<usize> {
        let start = self.snap_to_boundary(range.start);
        let end = self.snap_to_boundary(range.end).max(start);
        start..end
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.clamp_range(self.range_from_utf16(&range_utf16));
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let range = self.clamp_range(self.selected_range.clone());
        Some(UTF16Selection {
            range: self.range_to_utf16(&range),
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

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
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| self.selected_range.clone());
        let range = self.clamp_range(range);
        let mut next = String::with_capacity(
            self.content.len() - (range.end - range.start) + new_text.len(),
        );
        next.push_str(&self.content[..range.start]);
        next.push_str(new_text);
        next.push_str(&self.content[range.end..]);
        self.content = next.into();
        let cursor = range.start + new_text.len();
        self.selected_range = cursor..cursor;
        self.marked_range = None;
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|r| self.range_from_utf16(r))
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| self.selected_range.clone());
        let range = self.clamp_range(range);
        let mut next = String::with_capacity(
            self.content.len() - (range.end - range.start) + new_text.len(),
        );
        next.push_str(&self.content[..range.start]);
        next.push_str(new_text);
        next.push_str(&self.content[range.end..]);
        self.content = next.into();
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        let cursor = if let Some(selected) = new_selected_range_utf16 {
            // `new_selected_range_utf16` is the caret position within
            // the inserted text, in UTF-16 code units. Map it to a
            // UTF-8 offset within `new_text`, then anchor at
            // `range.start`.
            let mut utf16 = 0usize;
            let mut utf8 = new_text.len();
            for (i, ch) in new_text.char_indices() {
                if utf16 >= selected.end {
                    utf8 = i;
                    break;
                }
                utf16 += ch.len_utf16();
            }
            range.start + utf8
        } else {
            range.start + new_text.len()
        };
        let cursor = self.snap_to_boundary(cursor);
        self.selected_range = cursor..cursor;
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let layout = self.last_layout.as_ref()?;
        let range = self.clamp_range(self.range_from_utf16(&range_utf16));
        Some(Bounds::from_corners(
            point(bounds.left() + layout.x_for_index(range.start), bounds.top()),
            point(bounds.left() + layout.x_for_index(range.end), bounds.bottom()),
        ))
    }

    fn character_index_for_point(
        &mut self,
        p: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&p)?;
        let layout = self.last_layout.as_ref()?;
        let utf8_index = layout.index_for_x(p.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }
}

/// Inner element that lays out the field's text and paints the caret /
/// selection. Hidden — its sole purpose is to bind the
/// [`ElementInputHandler`] from inside `paint`, which is the only
/// place GPUI lets you do that.
struct TextInputElement {
    input: Entity<TextInput>,
}

struct TextInputPrepaint {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for TextInputElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextInputElement {
    type RequestLayoutState = ();
    type PrepaintState = TextInputPrepaint;

    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&crate::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&crate::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let theme = input.theme;
        let content = input.content.clone();
        let cursor_offset = input.cursor_offset();
        let selected = input.clamp_range(input.selected_range.clone());
        let style = window.text_style();
        let (display, color) = if content.is_empty() {
            (input.placeholder.clone(), theme.placeholder)
        } else {
            (content, theme.text)
        };
        let run = TextRun {
            len: display.len(),
            font: style.font(),
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display, font_size, &[run], None);
        let cursor_x = line.x_for_index(cursor_offset.min(line.text.len()));
        let cursor = fill(
            Bounds::new(
                point(bounds.left() + cursor_x, bounds.top()),
                size(px(2.0), bounds.bottom() - bounds.top()),
            ),
            theme.cursor,
        );
        let selection = if selected.start == selected.end {
            None
        } else {
            let start = line.x_for_index(selected.start.min(line.text.len()));
            let end = line.x_for_index(selected.end.min(line.text.len()));
            Some(fill(
                Bounds::from_corners(
                    point(bounds.left() + start, bounds.top()),
                    point(bounds.left() + end, bounds.bottom()),
                ),
                theme.selection,
            ))
        };
        TextInputPrepaint {
            line: Some(line),
            cursor: Some(cursor),
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&crate::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        // Bind the input handler so the platform IME can route events
        // back into our `EntityInputHandler` impl. Must run every paint
        // — `handle_input` only registers for the *next* frame.
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection);
        }
        let line = prepaint.line.take().expect("prepaint always sets line");
        line.paint(
            bounds.origin,
            window.line_height(),
            crate::TextAlign::Left,
            None,
            window,
            cx,
        )
        .ok();
        if focus_handle.is_focused(window)
            && let Some(cursor) = prepaint.cursor.take()
        {
            window.paint_quad(cursor);
        }
        if focus_handle.is_focused(window) {
            window.invalidate_character_coordinates();
        }
        self.input.update(cx, |input, _| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let focused = focus_handle.is_focused(window);
        let theme = self.theme;
        let border = if focused {
            theme.border_focused
        } else {
            theme.border
        };
        let bg = if focused {
            theme.background_focused
        } else {
            theme.background
        };
        let scroll_anchor = self.scroll_anchor.clone();
        // The root needs an id so `anchor_scroll` (on
        // `StatefulInteractiveElement`) is reachable. Use the entity
        // id — unique and stable for the field's lifetime.
        div()
            .id(ElementId::View(cx.entity().entity_id()))
            .key_context("TextInput")
            .track_focus(&focus_handle)
            .anchor_scroll(scroll_anchor)
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .h(px(44.0))
            .px(px(12.0))
            .py(px(10.0))
            .rounded_md()
            .border_1()
            .border_color(border)
            .bg(bg)
            .child(TextInputElement {
                input: cx.entity(),
            })
    }
}
