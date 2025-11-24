use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, Hsla, LayoutId, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString, Style, TextRun,
    TextStyleRefinement, UTF16Selection, Window, WrappedLine, actions, div, fill, point,
    prelude::*, px, relative, rgba, size,
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
}

#[derive(Clone, Debug)]
struct LineLayout {
    // The range of text in the content string that this logical line represents
    text_range: Range<usize>,
    // The wrapped line for rendering (may contain multiple visual lines)
    wrapped_line: Option<WrappedLine>,
    // Y position of the first visual line
    y_offset: Pixels,
    // Number of visual lines this logical line occupies (1 if not wrapped)
    visual_line_count: usize,
}

impl TextArea {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let style = TextAreaStyle::default();
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
        let before_pos = self.cursor_offset();
        println!(
            "LEFT: Before position: {}, selected_range: {:?}",
            before_pos, self.selected_range
        );

        if self.selected_range.is_empty() {
            let mut new_pos = self.previous_boundary(self.cursor_offset());

            // Check if we landed on a newline and skip it
            // new_pos is already a byte position from previous_boundary
            if new_pos < self.content.len() {
                if let Some(ch) = self.content[new_pos..].chars().next() {
                    if ch == '\n' {
                        println!(
                            "LEFT: Position {} is a newline, skipping to previous",
                            new_pos
                        );
                        new_pos = self.previous_boundary(new_pos);
                    }
                }
            }

            println!("LEFT: Moving to previous boundary: {}", new_pos);
            self.move_to(new_pos, cx);
        } else {
            println!(
                "LEFT: Moving to selection start: {}",
                self.selected_range.start
            );
            self.move_to(self.selected_range.start, cx)
        }

        let after_pos = self.cursor_offset();
        println!("LEFT: After position: {}", after_pos);
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        let before_pos = self.cursor_offset();
        println!(
            "RIGHT: Before position: {}, selected_range: {:?}",
            before_pos, self.selected_range
        );

        if self.selected_range.is_empty() {
            let mut new_pos = self.next_boundary(self.selected_range.end);

            // Check if we landed on a newline and skip it
            // new_pos is already a byte position from next_boundary
            if new_pos < self.content.len() {
                if let Some(ch) = self.content[new_pos..].chars().next() {
                    if ch == '\n' {
                        println!("RIGHT: Position {} is a newline, skipping to next", new_pos);
                        new_pos = self.next_boundary(new_pos);
                    }
                }
            }

            println!("RIGHT: Moving to next boundary: {}", new_pos);
            self.move_to(new_pos, cx);
        } else {
            println!(
                "RIGHT: Moving to selection end: {}",
                self.selected_range.end
            );
            self.move_to(self.selected_range.end, cx)
        }

        let after_pos = self.cursor_offset();
        println!("RIGHT: After position: {}", after_pos);
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        println!(
            "UP: Before position: {}, selected_range: {:?}",
            cursor_pos, self.selected_range
        );
        println!(
            "UP: Attempting to move vertically by -1 from position {}",
            cursor_pos
        );

        if let Some(new_pos) = self.move_vertically(cursor_pos, -1) {
            println!("UP: move_vertically returned new position: {}", new_pos);
            self.move_to(new_pos, cx);
            let after_pos = self.cursor_offset();
            println!("UP: After position: {}", after_pos);
        } else {
            println!("UP: move_vertically returned None, no movement");
        }
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        println!(
            "DOWN: Before position: {}, selected_range: {:?}",
            cursor_pos, self.selected_range
        );
        println!(
            "DOWN: Attempting to move vertically by 1 from position {}",
            cursor_pos
        );

        if let Some(new_pos) = self.move_vertically(cursor_pos, 1) {
            println!("DOWN: move_vertically returned new position: {}", new_pos);
            self.move_to(new_pos, cx);
            let after_pos = self.cursor_offset();
            println!("DOWN: After position: {}", after_pos);
        } else {
            println!("DOWN: move_vertically returned None, no movement");
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        let before_pos = self.cursor_offset();
        let new_pos = self.previous_boundary(before_pos);
        println!(
            "SELECT_LEFT: Before position: {}, moving to: {}, selected_range before: {:?}",
            before_pos, new_pos, self.selected_range
        );
        self.select_to(new_pos, cx);
        println!(
            "SELECT_LEFT: After selected_range: {:?}",
            self.selected_range
        );
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        let before_pos = self.cursor_offset();
        let new_pos = self.next_boundary(before_pos);
        println!(
            "SELECT_RIGHT: Before position: {}, moving to: {}, selected_range before: {:?}",
            before_pos, new_pos, self.selected_range
        );
        self.select_to(new_pos, cx);
        println!(
            "SELECT_RIGHT: After selected_range: {:?}",
            self.selected_range
        );
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        println!(
            "SELECT_UP: Before position: {}, selected_range: {:?}",
            cursor_pos, self.selected_range
        );

        if let Some(new_pos) = self.move_vertically(cursor_pos, -1) {
            println!(
                "SELECT_UP: move_vertically returned new position: {}",
                new_pos
            );
            self.select_to(new_pos, cx);
            println!("SELECT_UP: After selected_range: {:?}", self.selected_range);
        } else {
            println!("SELECT_UP: move_vertically returned None, no selection change");
        }
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        let cursor_pos = self.cursor_offset();
        println!(
            "SELECT_DOWN: Before position: {}, selected_range: {:?}",
            cursor_pos, self.selected_range
        );

        if let Some(new_pos) = self.move_vertically(cursor_pos, 1) {
            println!(
                "SELECT_DOWN: move_vertically returned new position: {}",
                new_pos
            );
            self.select_to(new_pos, cx);
            println!(
                "SELECT_DOWN: After selected_range: {:?}",
                self.selected_range
            );
        } else {
            println!("SELECT_DOWN: move_vertically returned None, no selection change");
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Grab focus when clicked
        window.focus(&self.focus_handle);

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
        // Find the start of the current logical line (after a newline)
        let bytes = self.content.as_bytes();
        for i in (0..offset.min(bytes.len())).rev() {
            if bytes[i] == b'\n' {
                return i + 1;
            }
        }
        0
    }

    fn find_line_end(&self, offset: usize) -> usize {
        // Find the end of the current logical line (before a newline)
        let bytes = self.content.as_bytes();
        for i in offset..bytes.len() {
            if bytes[i] == b'\n' {
                return i;
            }
        }
        self.content.len()
    }

    fn move_vertically(&self, offset: usize, direction: i32) -> Option<usize> {
        // Find current visual line and x position
        let (visual_line_idx, x_pixels) = self.find_visual_line_and_x_offset(offset);
        println!(
            "  move_vertically: current offset: {}, visual_line_idx: {}, x_pixels: {}",
            offset, visual_line_idx, x_pixels
        );

        // Calculate target visual line
        let target_visual_line_idx = (visual_line_idx as i32 + direction).max(0) as usize;
        println!(
            "  move_vertically: target_visual_line_idx: {} (direction: {})",
            target_visual_line_idx, direction
        );

        // Find which LineLayout contains this visual line
        let mut current_visual_line = 0;
        for (idx, layout) in self.line_layouts.iter().enumerate() {
            let visual_lines_in_layout = layout.visual_line_count;

            println!(
                "    Checking layout[{}] range: {:?}, current_visual_line: {}, visual_lines_in_layout: {}",
                idx, layout.text_range, current_visual_line, visual_lines_in_layout
            );
            println!(
                "    target_visual_line_idx ({}) < current_visual_line ({}) + visual_lines_in_layout ({}) = {} < {} = {}",
                target_visual_line_idx,
                current_visual_line,
                visual_lines_in_layout,
                target_visual_line_idx,
                current_visual_line + visual_lines_in_layout,
                target_visual_line_idx < current_visual_line + visual_lines_in_layout
            );

            if target_visual_line_idx < current_visual_line + visual_lines_in_layout {
                // Target is within this layout
                let visual_line_within_layout = target_visual_line_idx - current_visual_line;
                println!(
                    "    FOUND! Target is within this layout, visual_line_within_layout: {}",
                    visual_line_within_layout
                );

                if layout.text_range.is_empty() {
                    println!(
                        "  move_vertically: Found target at layout[{}] with empty text range, returning: {}",
                        idx, layout.text_range.start
                    );
                    return Some(layout.text_range.start);
                }

                if let Some(wrapped) = &layout.wrapped_line {
                    // Calculate y position within the wrapped line (which visual line we're on)
                    let y_within_wrapped =
                        self.style.line_height * visual_line_within_layout as f32;
                    let point = point(px(x_pixels), y_within_wrapped);

                    let closest_idx = wrapped
                        .closest_index_for_position(point, self.style.line_height)
                        .unwrap_or_else(|closest| closest);

                    let result = layout.text_range.start + closest_idx.min(wrapped.text.len());
                    println!(
                        "  move_vertically: Found target at layout[{}] with wrapped line, closest_idx: {}, returning: {}",
                        idx, closest_idx, result
                    );
                    return Some(result);
                }

                println!(
                    "  move_vertically: Found target at layout[{}] with no wrapped line, returning layout start: {}",
                    idx, layout.text_range.start
                );
                return Some(layout.text_range.start);
            }

            current_visual_line += visual_lines_in_layout;
            println!(
                "    Incrementing current_visual_line to: {}",
                current_visual_line
            );
        }

        // Past the end
        if direction > 0 {
            println!(
                "  move_vertically: past end, returning content length: {}",
                self.content.len()
            );
            Some(self.content.len())
        } else {
            println!("  move_vertically: past beginning, returning None");
            None
        }
    }

    fn find_visual_line_and_x_offset(&self, offset: usize) -> (usize, f32) {
        println!(
            "  find_visual_line_and_x_offset: looking for offset {}",
            offset
        );

        // Handle empty content
        if self.line_layouts.is_empty() {
            println!("  find_visual_line_and_x_offset: empty layouts, returning (0, 0.0)");
            return (0, 0.0);
        }

        let mut visual_line_idx = 0;

        for line in &self.line_layouts {
            println!(
                "    checking line with range {:?}, visual_line_idx: {}",
                line.text_range, visual_line_idx
            );
            if line.text_range.is_empty() {
                if offset == line.text_range.start {
                    println!(
                        "    found at empty line, returning ({}, 0.0)",
                        visual_line_idx
                    );
                    return (visual_line_idx, 0.0);
                }
                // Don't increment here - let the increment at the end of the loop handle it
            } else if offset >= line.text_range.start && offset <= line.text_range.end {
                // Found the line containing the offset
                println!("    found line containing offset!");
                if let Some(wrapped) = &line.wrapped_line {
                    let local_offset = (offset - line.text_range.start).min(wrapped.text.len());
                    println!("    has wrapped line, local_offset: {}", local_offset);
                    if let Some(position) =
                        wrapped.position_for_index(local_offset, self.style.line_height)
                    {
                        // The y component tells us which visual line within this wrapped line
                        let visual_line_within =
                            (position.y / self.style.line_height).floor() as usize;
                        let result = (visual_line_idx + visual_line_within, position.x.into());
                        println!(
                            "    wrapped position: {:?}, visual_line_within: {}, returning {:?}",
                            position, visual_line_within, result
                        );
                        return result;
                    }
                }
                println!("    no wrapped line, returning ({}, 0.0)", visual_line_idx);
                return (visual_line_idx, 0.0);
            }
            visual_line_idx += line.visual_line_count;
        }

        // If offset is beyond all lines, return last visual line
        let result = (visual_line_idx.saturating_sub(1), 0.0);
        println!(
            "  find_visual_line_and_x_offset: offset beyond all lines, returning {:?}",
            result
        );
        result
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }

        // Find the line layout and visual line at this y position
        for line in &self.line_layouts {
            let line_height_total = self.style.line_height * line.visual_line_count as f32;

            if position.y >= line.y_offset && position.y < line.y_offset + line_height_total {
                // Handle empty lines
                if line.text_range.is_empty() {
                    return line.text_range.start;
                }

                // Use wrapped line for accurate position
                if let Some(wrapped) = &line.wrapped_line {
                    // Calculate relative position within the wrapped line
                    let relative_y = position.y - line.y_offset;
                    let relative_point = point(position.x, relative_y);

                    let local_idx = wrapped
                        .closest_index_for_position(relative_point, self.style.line_height)
                        .unwrap_or_else(|closest| closest);

                    return line.text_range.start + local_idx.min(wrapped.text.len());
                }
                return line.text_range.start;
            }
        }

        self.content.len()
    }

    fn update_line_layouts(&mut self, width: Pixels, window: &mut Window) {
        self.line_layouts.clear();
        let font_size = self.style.font_size;
        let line_height = self.style.line_height;

        // Handle completely empty content
        if self.content.is_empty() {
            self.line_layouts.push(LineLayout {
                text_range: 0..0,
                wrapped_line: None,
                y_offset: px(0.),
                visual_line_count: 1,
            });
            self.needs_layout = false;
            return;
        }

        self.wrap_width = Some(width);
        let mut y_offset = px(0.);
        let text_style = window.text_style();

        // Split content into logical lines by newlines
        let mut current_pos = 0;

        while current_pos < self.content.len() {
            // Find the end of the current logical line
            let line_end = self.content[current_pos..]
                .find('\n')
                .map(|pos| current_pos + pos)
                .unwrap_or(self.content.len());

            let line_text = &self.content[current_pos..line_end];

            if line_text.is_empty() {
                // Empty line
                let layout = LineLayout {
                    text_range: current_pos..current_pos,
                    wrapped_line: None,
                    y_offset,
                    visual_line_count: 1,
                };
                self.line_layouts.push(layout);
                y_offset += line_height;
            } else {
                // Use shape_text with wrapping for non-empty lines
                let run = TextRun {
                    len: line_text.len(),
                    font: text_style.font(),
                    color: self.style.text_color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };

                // Shape with wrapping
                let wrapped_lines = window
                    .text_system()
                    .shape_text(
                        SharedString::from(line_text.to_string()),
                        font_size,
                        &[run],
                        Some(width),
                        None,
                    )
                    .unwrap_or_default();

                // Process each WrappedLine (should be just one per logical line)
                for (_wrap_idx, wrapped) in wrapped_lines.into_iter().enumerate() {
                    let visual_line_count = wrapped.wrap_boundaries().len() + 1;
                    let line_height_total = line_height * visual_line_count as f32;

                    // Store the single LineLayout with the full WrappedLine
                    let layout = LineLayout {
                        text_range: current_pos..line_end,
                        wrapped_line: Some(wrapped),
                        y_offset,
                        visual_line_count,
                    };

                    self.line_layouts.push(layout);

                    // Advance y_offset by the total height of all visual lines
                    y_offset += line_height_total;
                }
            }

            // Move to next logical line
            current_pos = if line_end < self.content.len() {
                line_end + 1 // Skip the newline
            } else {
                self.content.len()
            };
        }

        // Handle case where content ends with a newline (need empty line at end)
        if !self.content.is_empty() && self.content.ends_with('\n') {
            self.line_layouts.push(LineLayout {
                text_range: self.content.len()..self.content.len(),
                wrapped_line: None,
                y_offset,
                visual_line_count: 1,
            });
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
        // Move by grapheme clusters for proper Unicode support
        let mut indices: Vec<_> = self
            .content
            .grapheme_indices(true)
            .map(|(i, _)| i)
            .collect();
        indices.push(self.content.len()); // Add end position

        // Find the position just before the current offset
        for i in (0..indices.len()).rev() {
            if indices[i] < offset {
                return indices[i];
            }
        }
        0
    }

    fn next_boundary(&self, offset: usize) -> usize {
        // Move by grapheme clusters for proper Unicode support
        let mut indices: Vec<_> = self
            .content
            .grapheme_indices(true)
            .map(|(i, _)| i)
            .collect();
        indices.push(self.content.len()); // Add end position

        // Find the position just after the current offset
        for i in 0..indices.len() {
            if indices[i] > offset {
                return indices[i];
            }
        }
        self.content.len()
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
            // Handle empty lines
            if line.text_range.is_empty() {
                if range.start == line.text_range.start {
                    return Some(Bounds::from_corners(
                        point(bounds.left(), bounds.top() + line.y_offset),
                        point(
                            bounds.left() + px(4.), // Small width for empty line
                            bounds.top() + line.y_offset + self.style.line_height,
                        ),
                    ));
                }
            } else if line.text_range.contains(&range.start) {
                if let Some(wrapped) = &line.wrapped_line {
                    let local_start = range.start - line.text_range.start;
                    let local_end = (range.end - line.text_range.start).min(wrapped.text.len());

                    let start_pos = wrapped
                        .position_for_index(local_start, self.style.line_height)
                        .unwrap_or(point(px(0.), px(0.)));
                    let end_pos = wrapped
                        .position_for_index(local_end, self.style.line_height)
                        .unwrap_or_else(|| {
                            let last_line_y =
                                self.style.line_height * (line.visual_line_count - 1) as f32;
                            point(wrapped.width(), last_line_y)
                        });

                    // Calculate which visual lines are involved
                    let start_visual_line = (start_pos.y / self.style.line_height).floor() as usize;
                    let end_visual_line = (end_pos.y / self.style.line_height).floor() as usize;

                    if start_visual_line == end_visual_line {
                        // Range is within a single visual line
                        return Some(Bounds::from_corners(
                            point(
                                bounds.left() + start_pos.x,
                                bounds.top() + line.y_offset + start_pos.y,
                            ),
                            point(
                                bounds.left() + end_pos.x,
                                bounds.top() + line.y_offset + start_pos.y + self.style.line_height,
                            ),
                        ));
                    } else {
                        // Range spans multiple visual lines - return bounds of first line segment
                        return Some(Bounds::from_corners(
                            point(
                                bounds.left() + start_pos.x,
                                bounds.top() + line.y_offset + start_pos.y,
                            ),
                            point(
                                bounds.left() + wrapped.width(),
                                bounds.top() + line.y_offset + start_pos.y + self.style.line_height,
                            ),
                        ));
                    }
                }
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
        let (focus_handle, content, selected_range, placeholder, line_layouts) = {
            let area_state = self.area.read(cx);
            (
                area_state.focus_handle.clone(),
                area_state.content.clone(),
                area_state.selected_range.clone(),
                area_state.placeholder.clone(),
                area_state.line_layouts.clone(),
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
            println!("Drawing selection: range={:?}", selected_range);
            for (line_idx, line) in line_layouts.iter().enumerate() {
                let line_start = line.text_range.start;
                let line_end = line.text_range.end;

                // Check if selection intersects this line
                if selected_range.end > line_start
                    && (line.text_range.is_empty() || selected_range.start < line_end)
                {
                    println!(
                        "  Selection intersects line {}: line_range={:?}, visual_lines={}",
                        line_idx, line.text_range, line.visual_line_count
                    );
                    // Handle empty lines
                    if line.text_range.is_empty() {
                        // For empty lines, draw selection if it spans across this line
                        if selected_range.start <= line_start && selected_range.end > line_start {
                            window.paint_quad(fill(
                                Bounds::from_corners(
                                    point(bounds.left(), bounds.top() + line.y_offset),
                                    point(
                                        bounds.left() + px(4.), // Small width for empty line selection
                                        bounds.top() + line.y_offset + self.style.line_height,
                                    ),
                                ),
                                rgba(0x3311ff30),
                            ));
                        }
                    } else if let Some(wrapped) = &line.wrapped_line {
                        let sel_start = selected_range.start.max(line_start) - line_start;
                        let sel_end = selected_range.end.min(line_end) - line_start;

                        // Get full position info including which visual line
                        let start_pos = wrapped
                            .position_for_index(sel_start, self.style.line_height)
                            .unwrap_or(point(px(0.), px(0.)));
                        let end_pos = wrapped
                            .position_for_index(sel_end, self.style.line_height)
                            .unwrap_or_else(|| {
                                // If we can't get position, put at end of last visual line
                                let last_line_y =
                                    self.style.line_height * (line.visual_line_count - 1) as f32;
                                point(wrapped.width(), last_line_y)
                            });

                        // Calculate which visual lines are involved
                        let start_visual_line =
                            (start_pos.y / self.style.line_height).floor() as usize;
                        let end_visual_line = (end_pos.y / self.style.line_height).floor() as usize;

                        println!(
                            "    Selection in wrapped line: start_pos={:?}, end_pos={:?}",
                            start_pos, end_pos
                        );
                        println!(
                            "    Visual lines involved: {} to {}",
                            start_visual_line, end_visual_line
                        );

                        if start_visual_line == end_visual_line {
                            // Selection is within a single visual line
                            println!("    Drawing single-line selection");
                            window.paint_quad(fill(
                                Bounds::from_corners(
                                    point(
                                        bounds.left() + start_pos.x,
                                        bounds.top() + line.y_offset + start_pos.y,
                                    ),
                                    point(
                                        bounds.left() + end_pos.x,
                                        bounds.top()
                                            + line.y_offset
                                            + start_pos.y
                                            + self.style.line_height,
                                    ),
                                ),
                                rgba(0x3311ff30),
                            ));
                        } else {
                            // Selection spans multiple visual lines
                            println!("    Drawing multi-line selection");
                            // Draw first line (partial)
                            println!("      First line: from x={:?} to end", start_pos.x);
                            window.paint_quad(fill(
                                Bounds::from_corners(
                                    point(
                                        bounds.left() + start_pos.x,
                                        bounds.top() + line.y_offset + start_pos.y,
                                    ),
                                    point(
                                        bounds.left() + wrapped.width(),
                                        bounds.top()
                                            + line.y_offset
                                            + start_pos.y
                                            + self.style.line_height,
                                    ),
                                ),
                                rgba(0x3311ff30),
                            ));

                            // Draw middle lines (full width)
                            println!(
                                "      Middle lines: {} full-width lines",
                                end_visual_line - start_visual_line - 1
                            );
                            for visual_line in (start_visual_line + 1)..end_visual_line {
                                let y = self.style.line_height * visual_line as f32;
                                window.paint_quad(fill(
                                    Bounds::from_corners(
                                        point(bounds.left(), bounds.top() + line.y_offset + y),
                                        point(
                                            bounds.left() + wrapped.width(),
                                            bounds.top()
                                                + line.y_offset
                                                + y
                                                + self.style.line_height,
                                        ),
                                    ),
                                    rgba(0x3311ff30),
                                ));
                            }

                            // Draw last line (partial)
                            println!("      Last line: from start to x={:?}", end_pos.x);
                            window.paint_quad(fill(
                                Bounds::from_corners(
                                    point(bounds.left(), bounds.top() + line.y_offset + end_pos.y),
                                    point(
                                        bounds.left() + end_pos.x,
                                        bounds.top()
                                            + line.y_offset
                                            + end_pos.y
                                            + self.style.line_height,
                                    ),
                                ),
                                rgba(0x3311ff30),
                            ));
                        }
                    }
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
                    .shape_line(placeholder, font_size, &[run], None);
            line.paint(bounds.origin, self.style.line_height, window, cx)
                .unwrap();
        } else {
            // Draw each line using the pre-wrapped lines
            for line_layout in &line_layouts {
                if let Some(wrapped) = &line_layout.wrapped_line {
                    let paint_pos = point(bounds.left(), bounds.top() + line_layout.y_offset);
                    // WrappedLine::paint handles all wrap boundaries internally
                    wrapped
                        .paint(
                            paint_pos,
                            self.style.line_height,
                            gpui::TextAlign::Left,
                            Some(bounds),
                            window,
                            cx,
                        )
                        .unwrap();
                } else {
                    println!("    Empty line - no paint needed");
                }
            }
        }
        println!("Finished painting text content\n");

        // Draw cursor
        if focus_handle.is_focused(window) && selected_range.is_empty() {
            let cursor_offset = selected_range.start;
            println!("Drawing cursor at offset: {}", cursor_offset);

            // Find the line containing the cursor
            for line in &line_layouts {
                let is_cursor_in_line = if line.text_range.is_empty() {
                    // For empty lines, check if cursor is at this position
                    cursor_offset == line.text_range.start
                } else {
                    line.text_range.contains(&cursor_offset)
                        || (cursor_offset == line.text_range.end && cursor_offset == content.len())
                };

                if is_cursor_in_line {
                    let cursor_position = if let Some(wrapped) = &line.wrapped_line {
                        let local_offset = cursor_offset.saturating_sub(line.text_range.start);
                        wrapped
                            .position_for_index(local_offset, self.style.line_height)
                            .unwrap_or(point(px(0.), px(0.)))
                    } else {
                        // Empty line - cursor at start
                        point(px(0.), px(0.))
                    };

                    window.paint_quad(fill(
                        Bounds::new(
                            point(
                                bounds.left() + cursor_position.x,
                                bounds.top() + line.y_offset + cursor_position.y,
                            ),
                            size(px(2.), self.style.line_height),
                        ),
                        gpui::blue(),
                    ));
                    println!(
                        "  Drew cursor at x={:?}, y={:?}",
                        cursor_position.x,
                        line.y_offset + cursor_position.y
                    );
                    break;
                }
            }
        }
        println!("=== PAINT END ===\n");
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
