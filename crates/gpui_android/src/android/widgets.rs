//! Reusable Android-aware GPUI widgets.
//!
//! Apps targeting Android often need UI primitives that hand back a
//! [`PlatformInputHandler`] so the soft keyboard pops on tap. The existing
//! GPUI examples ship per-example text fields, but each one duplicates the
//! same UTF-8 / UTF-16 conversion logic, IME marking handling and focus
//! plumbing. This module collapses that into a single
//! [`TextField`] consumers can drop into any view.
//!
//! ```ignore
//! use gpui_android::widgets::{FieldKind, TextField};
//!
//! struct Form {
//!     name: Entity<TextField>,
//!     amount: Entity<TextField>,
//! }
//!
//! impl Form {
//!     fn new(cx: &mut Context<Self>) -> Self {
//!         let name = cx.new(|cx| TextField::new(cx, "your name", FieldKind::Text));
//!         let amount = cx.new(|cx| TextField::new(cx, "0.00", FieldKind::Number));
//!         cx.observe(&name, |_, _, cx| cx.notify()).detach();
//!         cx.observe(&amount, |_, _, cx| cx.notify()).detach();
//!         Self { name, amount }
//!     }
//! }
//! ```
//!
//! ## Why widget owns the focus handle
//!
//! GPUI's IME bridge keys off `FocusHandle`s — see [`Window::handle_input`].
//! If the parent view owned the handle and merely passed it to the widget on
//! render, the platform layer would still bind input to the parent (because
//! that's the handle that's "focused"), and `EntityInputHandler` would never
//! be called on the field. Owning the handle inside [`TextField`] keeps the
//! IME → field path straight.
//!
//! [`PlatformInputHandler`]: gpui::PlatformInputHandler
//! [`Window::handle_input`]: gpui::Window::handle_input

use std::ops::Range;

use gpui::{
    App, Bounds, Context, Element, ElementId, ElementInputHandler, Entity, EntityInputHandler,
    FocusHandle, Focusable, GlobalElementId, Hsla, IntoElement, LayoutId, MouseButton,
    MouseDownEvent, PaintQuad, Pixels, Render, ShapedLine, SharedString, Style, TextRun,
    UTF16Selection, Window, div, fill, hsla, point, prelude::*, px, relative, rgb, size,
};

/// Variant hint for the field. `Number` filters the IME-delivered text down
/// to digit / sign / decimal characters at write time.
///
/// The soft keyboard *layout* itself stays the default text layout: switching
/// the keyboard to a numeric pad requires per-focus `InputType`, which the
/// `gpui_android` IME bridge doesn't yet plumb through. Applying the filter
/// at write time at least keeps the captured string parseable as a number.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldKind {
    /// Free-form text. No filter; whatever the IME inserts is kept.
    Text,
    /// Decimal numbers. Drops everything except ASCII digits, `-` and `.` at
    /// IME write time.
    Number,
}

/// Single-line text field that participates in GPUI's IME bridge.
///
/// Tapping the field focuses its [`FocusHandle`] which, on Android, causes
/// `gpui_android` to push a `PlatformInputHandler` to the platform on the
/// next paint and pop the soft keyboard.
///
/// `content` and `placeholder` are exposed as public fields so callers can
/// read the current value or push an initial value before the user types.
/// External edits should pair with `cx.notify()` so any observers re-render.
pub struct TextField {
    /// Focus owned by the field. See module docs for why the field — not
    /// the parent view — owns this.
    pub focus_handle: FocusHandle,
    /// Current content. Empty `SharedString` renders the placeholder.
    pub content: SharedString,
    /// Placeholder shown when `content` is empty.
    pub placeholder: SharedString,
    /// Whether the field filters IME input as a decimal number.
    pub kind: FieldKind,
    /// Selection range in **UTF-8 byte offsets** (not UTF-16 code units —
    /// callers from the IME side go through `range_from_utf16`).
    selected_range: Range<usize>,
    /// Pending IME composition region, in UTF-8 byte offsets.
    marked_range: Option<Range<usize>>,
    /// Cached layout from the last paint, used by `bounds_for_range` and
    /// `character_index_for_point`.
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    /// Theme colours applied to the field's chrome. `None` falls back to a
    /// neutral dark theme.
    theme: Option<TextFieldTheme>,
}

/// Visual theme for a [`TextField`]. All colours are in GPUI's `Hsla`. Use
/// `TextFieldTheme::default()` for the bundled neutral dark theme, or
/// construct one explicitly to integrate with your app's palette.
#[derive(Clone, Copy, Debug)]
pub struct TextFieldTheme {
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
    /// Cursor colour, painted only while the field is focused.
    pub cursor: Hsla,
    /// Selection-highlight fill for selected text.
    pub selection: Hsla,
}

impl Default for TextFieldTheme {
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

impl TextField {
    /// Allocate a new field with the given placeholder + filter kind. The
    /// returned `TextField` is meant to be wrapped in `cx.new(...)` to
    /// yield an `Entity<TextField>`.
    pub fn new(
        cx: &mut Context<Self>,
        placeholder: impl Into<SharedString>,
        kind: FieldKind,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: SharedString::default(),
            placeholder: placeholder.into(),
            kind,
            selected_range: 0..0,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            theme: None,
        }
    }

    /// Override the visual theme. Call before mounting the field if you need
    /// colours different from the default neutral dark scheme.
    pub fn with_theme(mut self, theme: TextFieldTheme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Replace the field's content programmatically. Resets selection to the
    /// end and clears any in-flight IME composition. Call `cx.notify()` after
    /// to schedule a redraw.
    pub fn set_content(&mut self, content: impl Into<SharedString>) {
        self.content = content.into();
        let len = self.content.len();
        self.selected_range = len..len;
        self.marked_range = None;
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
        // Tapping the field focuses it; focus triggers
        // `PlatformWindow::set_input_handler` on the next paint, which on
        // Android pops the soft keyboard.
        window.focus(&self.focus_handle, cx);
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
    /// clamping to `[0, content.len()]`. Used to keep stale selection /
    /// marked ranges from panicking when sliced after a content change.
    fn snap_to_boundary(&self, idx: usize) -> usize {
        let len = self.content.len();
        let mut idx = idx.min(len);
        while idx > 0 && !self.content.is_char_boundary(idx) {
            idx -= 1;
        }
        idx
    }

    /// Clamp a byte range against `content`, snapping each endpoint to the
    /// nearest valid char boundary and ensuring `start <= end`. Returning a
    /// well-formed range here is what keeps the slicing in
    /// [`Self::replace_text_in_range`] from panicking when the range was
    /// derived from a stale `selected_range` or `marked_range`.
    fn clamp_range(&self, range: Range<usize>) -> Range<usize> {
        let start = self.snap_to_boundary(range.start);
        let end = self.snap_to_boundary(range.end).max(start);
        start..end
    }

    fn theme(&self) -> TextFieldTheme {
        self.theme.unwrap_or_default()
    }
}

impl Focusable for TextField {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for TextField {
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
        let filtered_storage;
        let new_text = if matches!(self.kind, FieldKind::Number) {
            filtered_storage = new_text
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '-' || *c == '.')
                .collect::<String>();
            filtered_storage.as_str()
        } else {
            new_text
        };

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

        // `new_selected_range_utf16`, when present, is documented as the
        // selection *within the inserted text*, expressed in UTF-16 code
        // units. Translate it to absolute UTF-8 byte offsets within the
        // updated content (anchored at `range.start`), then snap to char
        // boundaries — without snapping, downstream slicing can panic for
        // multi-byte UTF-8 inserts.
        self.selected_range = if let Some(selected) = new_selected_range_utf16 {
            let inserted_utf8: Vec<usize> =
                std::iter::once(0)
                    .chain(new_text.char_indices().skip(1).map(|(i, _)| i))
                    .chain(std::iter::once(new_text.len()))
                    .collect();
            let inserted_utf16: Vec<usize> = {
                let mut acc = 0usize;
                let mut v = vec![0usize];
                for ch in new_text.chars() {
                    acc += ch.len_utf16();
                    v.push(acc);
                }
                v
            };
            let map = |units: usize| -> usize {
                inserted_utf16
                    .iter()
                    .position(|u| *u >= units)
                    .map(|idx| inserted_utf8[idx.min(inserted_utf8.len() - 1)])
                    .unwrap_or(new_text.len())
            };
            let start = range.start + map(selected.start);
            let end = range.start + map(selected.end);
            let start = self.snap_to_boundary(start);
            let end = self.snap_to_boundary(end).max(start);
            start..end
        } else {
            let cursor = range.start + new_text.len();
            cursor..cursor
        };
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
        p: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&p)?;
        let layout = self.last_layout.as_ref()?;
        let utf8_index = layout.index_for_x(p.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }
}

/// Custom element that lays out the field's text and paints a cursor at the
/// current offset. The element is the only place `window.handle_input(...)`
/// can be called (must run during paint), which is what plumbs the platform
/// IME → `EntityInputHandler` chain.
struct TextLineElement {
    input: Entity<TextField>,
}

struct TextLinePrepaint {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for TextLineElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextLineElement {
    type RequestLayoutState = ();
    type PrepaintState = TextLinePrepaint;

    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
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
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let theme = input.theme();
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
        TextLinePrepaint {
            line: Some(line),
            cursor: Some(cursor),
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        // Bind `EntityInputHandler` so platform IME events reach us. Has to
        // be called every paint: `handle_input` only registers for *the next
        // frame*.
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
            gpui::TextAlign::Left,
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
        // When the field is focused, ask GPUI to push the cursor's
        // candidate-window position to the IME via
        // `PlatformWindow::update_ime_position`. The Android backend uses
        // the bounds it receives to remember where the focused field sits
        // so the run loop can scroll it into view if the keyboard would
        // cover it. This is a no-op when the field isn't focused.
        if focus_handle.is_focused(window) {
            window.invalidate_character_coordinates();
        }
        self.input.update(cx, |input, _| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

impl Render for TextField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let focused = focus_handle.is_focused(window);
        let theme = self.theme();
        let border = if focused { theme.border_focused } else { theme.border };
        let bg = if focused { theme.background_focused } else { theme.background };
        div()
            .key_context("AndroidTextField")
            .track_focus(&focus_handle)
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .h(px(44.0))
            .px(px(12.0))
            .py(px(10.0))
            .rounded_md()
            .border_1()
            .border_color(border)
            .bg(bg)
            .child(TextLineElement { input: cx.entity() })
    }
}
