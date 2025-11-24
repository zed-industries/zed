use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, Hsla, LayoutId, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString, Style, TextRun,
    TextStyleRefinement, UTF16Selection, Window, actions, div, fill, point, prelude::*, px,
    relative, rgba, size,
};
use unicode_segmentation::*;

actions!(
    text_area,
    [
        Backspace,
        Delete,
        Left,
        Right,
        Up,
        Down,
        SelectLeft,
        SelectRight,
        SelectUp,
        SelectDown,
        SelectAll,
        Home,
        End,
        PageUp,
        PageDown,
        Paste,
        Cut,
        Copy,
        Enter,
    ]
);

pub struct TextArea {
    style: TextAreaStyle,
    focus_handle: FocusHandle,
    content: String,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,

    // Layout cache for each line
    line_layouts: Vec<LineLayout>,
    wrap_width: Option<Pixels>,
    needs_layout: bool,

    // Interaction state
    is_selecting: bool,

    // Approximate character width (calculated from font size)
    char_width: Pixels,
}

#[derive(Clone, Debug)]
struct LineLayout {
    // The range of text in the content string that this line represents
    text_range: Range<usize>,
    // The shaped line for rendering
    shaped_line: Option<gpui::ShapedLine>,
    // Y position of this line
    y_offset: Pixels,
    // Whether this is a wrapped line (continuation of previous)
    is_wrapped: bool,
}

impl TextArea {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let style = TextAreaStyle::default();
        let char_width = style.font_size * 0.6; // Approximate character width
        Self {
            style,
            focus_handle: cx.focus_handle(),
            content: String::new(),
            placeholder: "Type here...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            line_layouts: Vec::new(),
            wrap_width: None,
            needs_layout: true,
            is_selecting: false,
            char_width,
        }
    }

    pub fn set_content(&mut self, content: &str, cx: &mut Context<Self>) {
        self.content = content.to_string();
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.needs_layout = true;
        cx.notify();
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        if let Some(new_pos) = self.move_vertically(cursor_pos, -1) {
            self.move_to(new_pos, cx);
        }
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        if let Some(new_pos) = self.move_vertically(cursor_pos, 1) {
            self.move_to(new_pos, cx);
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        if let Some(new_pos) = self.move_vertically(cursor_pos, -1) {
            self.select_to(new_pos, cx);
        }
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        if let Some(new_pos) = self.move_vertically(cursor_pos, 1) {
            self.select_to(new_pos, cx);
        }
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        let cursor = self.cursor_offset();
        let line_start = self.find_line_start(cursor);
        self.move_to(line_start, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        let cursor = self.cursor_offset();
        let line_end = self.find_line_end(cursor);
        self.move_to(line_end, cx);
    }

    fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_text_in_range(None, "\n", window, cx)
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;

        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn find_line_start(&self, offset: usize) -> usize {
        self.content[..offset]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0)
    }

    fn find_line_end(&self, offset: usize) -> usize {
        self.content[offset..]
            .find('\n')
            .map(|i| offset + i)
            .unwrap_or(self.content.len())
    }

    fn move_vertically(&self, offset: usize, direction: i32) -> Option<usize> {
        let (line_idx, x_in_line) = self.find_line_and_x_offset(offset);
        let target_line_idx = (line_idx as i32 + direction).max(0) as usize;

        if target_line_idx >= self.line_layouts.len() {
            return None;
        }

        let target_line = &self.line_layouts[target_line_idx];
        let line_text = &self.content[target_line.text_range.clone()];

        // Find the closest character position in the target line
        let mut best_offset = target_line.text_range.start;
        let mut best_distance = f32::MAX;
        let mut current_x = 0.0;

        for (idx, _) in line_text.grapheme_indices(true) {
            let distance = (current_x - x_in_line).abs();
            if distance < best_distance {
                best_distance = distance;
                best_offset = target_line.text_range.start + idx;
            }
            current_x += f32::from(self.char_width); // Use calculated character width
        }

        Some(best_offset)
    }

    fn find_line_and_x_offset(&self, offset: usize) -> (usize, f32) {
        for (idx, line) in self.line_layouts.iter().enumerate() {
            if line.text_range.contains(&offset) {
                let x_offset = (offset - line.text_range.start) as f32 * f32::from(self.char_width);
                return (idx, x_offset);
            }
        }
        (0, 0.0)
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }

        // Find the line at this y position
        for line in &self.line_layouts {
            if position.y >= line.y_offset && position.y < line.y_offset + self.style.line_height {
                // Approximate character position within the line
                let char_index =
                    ((position.x / self.char_width).floor() as usize).min(line.text_range.len());
                return line.text_range.start + char_index;
            }
        }

        self.content.len()
    }

    fn update_line_layouts(&mut self, width: Pixels, _window: &mut Window) {
        self.line_layouts.clear();
        let _font_size = self.style.font_size;
        let line_height = self.style.line_height;

        if self.content.is_empty() {
            return;
        }

        let mut y_offset = px(0.);
        let mut current_pos = 0;

        // Split content by newlines while preserving byte positions
        while current_pos <= self.content.len() {
            let line_end = if current_pos < self.content.len() {
                // Find next newline or end of string
                self.content[current_pos..]
                    .find('\n')
                    .map(|i| current_pos + i)
                    .unwrap_or(self.content.len())
            } else {
                // Handle empty line at end after newline
                current_pos
            };

            // Only process non-empty ranges or if we're at a newline boundary
            if current_pos < self.content.len()
                || (current_pos == self.content.len()
                    && current_pos > 0
                    && self.content.chars().last() == Some('\n'))
            {
                // For now, simple wrapping - count grapheme clusters for better UTF-8 handling
                let line_text = &self.content[current_pos..line_end];
                let grapheme_count = line_text.graphemes(true).count();
                let chars_per_line = (width / self.char_width).floor() as usize;

                if chars_per_line > 0 && grapheme_count > chars_per_line {
                    // Split into wrapped lines using grapheme boundaries
                    let mut graphemes = line_text.grapheme_indices(true);
                    let mut chunk_start = 0;
                    let mut grapheme_counter = 0;

                    for (byte_idx, _) in graphemes.by_ref() {
                        grapheme_counter += 1;
                        if grapheme_counter >= chars_per_line {
                            self.line_layouts.push(LineLayout {
                                text_range: (current_pos + chunk_start)..(current_pos + byte_idx),
                                shaped_line: None,
                                y_offset,
                                is_wrapped: chunk_start > 0,
                            });
                            y_offset += line_height;
                            chunk_start = byte_idx;
                            grapheme_counter = 0;
                        }
                    }
                    // Add remaining chunk if any
                    if chunk_start < line_text.len() {
                        self.line_layouts.push(LineLayout {
                            text_range: (current_pos + chunk_start)..line_end,
                            shaped_line: None,
                            y_offset,
                            is_wrapped: chunk_start > 0,
                        });
                        y_offset += line_height;
                    }
                } else if current_pos < line_end || current_pos == 0 {
                    // Regular line or empty first line
                    self.line_layouts.push(LineLayout {
                        text_range: current_pos..line_end,
                        shaped_line: None,
                        y_offset,
                        is_wrapped: false,
                    });
                    y_offset += line_height;
                }
            }

            // Move past the newline if there is one
            if line_end < self.content.len() {
                current_pos = line_end + 1;
            } else {
                break;
            }
        }

        self.needs_layout = false;
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
}

impl EntityInputHandler for TextArea {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        adjusted_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
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
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            self.content[0..range.start].to_owned() + new_text + &self.content[range.end..];
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        self.needs_layout = true;
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
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            self.content[0..range.start].to_owned() + new_text + &self.content[range.end..];
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        self.needs_layout = true;
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let range = self.range_from_utf16(&range_utf16);

        // Find the line containing the range start
        for line in &self.line_layouts {
            if line.text_range.contains(&range.start) {
                let x_start =
                    (range.start - line.text_range.start) as f32 * f32::from(self.char_width);
                let x_end = ((range.end - line.text_range.start).min(line.text_range.len())) as f32
                    * f32::from(self.char_width);

                return Some(Bounds::from_corners(
                    point(bounds.left() + px(x_start), bounds.top() + line.y_offset),
                    point(
                        bounds.left() + px(x_end),
                        bounds.top() + line.y_offset + self.style.line_height,
                    ),
                ));
            }
        }
        None
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let index = self.index_for_mouse_position(point);
        Some(self.offset_to_utf16(index))
    }
}

pub struct TextAreaStyle {
    font_size: Pixels,
    line_height: Pixels,
    text_color: Hsla,
}

impl Default for TextAreaStyle {
    fn default() -> Self {
        Self {
            font_size: px(12.),
            line_height: px(14.),
            text_color: gpui::black(),
        }
    }
}

struct TextAreaElement {
    style: TextAreaStyle,
    area: Entity<TextArea>,
}

impl TextAreaElement {
    pub fn new(area: Entity<TextArea>) -> Self {
        Self {
            style: TextAreaStyle::default(),
            area,
        }
    }
}

impl IntoElement for TextAreaElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextAreaElement {
    type RequestLayoutState = ();
    type PrepaintState = Bounds<Pixels>;

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

        let mut text_style = TextStyleRefinement::default();

        text_style.font_size = Some(self.style.font_size.into());
        text_style.line_height = Some(self.style.line_height.into());
        text_style.color = Some(self.style.text_color);

        style.text.refine(&text_style);

        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.area.update(cx, |area, _| {
            if area.needs_layout {
                area.update_line_layouts(bounds.size.width, window);
            }
        });
        bounds
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (focus_handle, content, selected_range, placeholder, line_layouts, char_width) = {
            let area_state = self.area.read(cx);
            (
                area_state.focus_handle.clone(),
                area_state.content.clone(),
                area_state.selected_range.clone(),
                area_state.placeholder.clone(),
                area_state.line_layouts.clone(),
                area_state.char_width,
            )
        };

        // Handle input
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.area.clone()),
            cx,
        );

        // Draw selection
        if !selected_range.is_empty() {
            for line in &line_layouts {
                let line_start = line.text_range.start;
                let line_end = line.text_range.end;

                if selected_range.end > line_start && selected_range.start < line_end {
                    let sel_start = selected_range.start.max(line_start) - line_start;
                    let sel_end = selected_range.end.min(line_end) - line_start;

                    let x_start = sel_start as f32 * f32::from(char_width);
                    let x_end = sel_end as f32 * f32::from(char_width);

                    window.paint_quad(fill(
                        Bounds::from_corners(
                            point(bounds.left() + px(x_start), bounds.top() + line.y_offset),
                            point(
                                bounds.left() + px(x_end),
                                bounds.top() + line.y_offset + self.style.line_height,
                            ),
                        ),
                        rgba(0x3311ff30),
                    ));
                }
            }
        }

        // Draw text or placeholder
        let style = window.text_style();
        let font_size = self.style.font_size;

        if content.is_empty() {
            let run = TextRun {
                len: placeholder.len(),
                font: style.font(),
                color: rgba(0x00000033).into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let line =
                window
                    .text_system()
                    .shape_line(placeholder.clone(), font_size, &[run], None);
            line.paint(bounds.origin, self.style.line_height, window, cx)
                .unwrap();
        } else {
            // Draw each line
            for line_layout in &line_layouts {
                if line_layout.text_range.start <= content.len()
                    && line_layout.text_range.end <= content.len()
                {
                    let line_text = &content[line_layout.text_range.clone()];
                    if !line_text.is_empty() {
                        let run = TextRun {
                            len: line_text.len(),
                            font: style.font(),
                            color: style.color,
                            background_color: None,
                            underline: None,
                            strikethrough: None,
                        };
                        let shaped = window.text_system().shape_line(
                            SharedString::from(line_text.to_string()),
                            font_size,
                            &[run],
                            None,
                        );
                        shaped
                            .paint(
                                point(bounds.left(), bounds.top() + line_layout.y_offset),
                                self.style.line_height,
                                window,
                                cx,
                            )
                            .unwrap();
                    }
                }
            }
        }

        // Draw cursor
        if focus_handle.is_focused(window) && selected_range.is_empty() {
            let cursor_offset = selected_range.start;

            // Find the line containing the cursor
            for line in &line_layouts {
                if line.text_range.contains(&cursor_offset)
                    || (cursor_offset == line.text_range.end && cursor_offset == content.len())
                {
                    let x_offset =
                        (cursor_offset - line.text_range.start) as f32 * f32::from(char_width);

                    window.paint_quad(fill(
                        Bounds::new(
                            point(bounds.left() + px(x_offset), bounds.top() + line.y_offset),
                            size(px(2.), self.style.line_height),
                        ),
                        gpui::blue(),
                    ));
                    break;
                }
            }
        }
    }
}

impl Render for TextArea {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .key_context("TextArea")
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_up))
            .on_action(cx.listener(Self::select_down))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .size_full()
            .child(TextAreaElement::new(cx.entity()))
    }
}

impl Focusable for TextArea {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
