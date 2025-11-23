use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, ContentMask, Context, CursorStyle, DispatchPhase, Edges, ElementId,
    ElementInputHandler, Entity, EntityInputHandler, FocusHandle, Focusable, GlobalElementId,
    Hitbox, HitboxBehavior, Hsla, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, Point, ScrollWheelEvent, SharedString, Style, TextRun,
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
        SelectToBeginning,
        SelectToEnd,
        MoveToBeginning,
        MoveToEnd,
        Paste,
        Cut,
        Copy,
        Enter,
        WordLeft,
        WordRight,
        SelectWordLeft,
        SelectWordRight,
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
    line_layouts: Vec<LineLayout>,
    wrap_width: Option<Pixels>,
    needs_layout: bool,
    is_selecting: bool,
    last_click_position: Option<Point<Pixels>>,
    click_count: usize,
    scroll_offset: Pixels,
    available_height: Pixels,
}

#[derive(Clone, Debug)]
struct LineLayout {
    text_range: Range<usize>,
    wrapped_line: Option<WrappedLine>,
    y_offset: Pixels,
    visual_line_count: usize,
}

#[derive(Clone, Debug)]
pub struct TextAreaStyle {
    pub font_size: Pixels,
    pub line_height: Pixels,
    pub text_color: Hsla,
    pub padding: Edges<Pixels>,
}

impl Default for TextAreaStyle {
    fn default() -> Self {
        Self {
            font_size: px(12.),
            line_height: px(18.),
            text_color: gpui::black(),
            padding: Edges {
                top: px(4.),
                right: px(4.),
                bottom: px(4.),
                left: px(4.),
            },
        }
    }
}

impl TextArea {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            style: TextAreaStyle::default(),
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
            last_click_position: None,
            click_count: 0,
            scroll_offset: px(0.),
            available_height: px(0.),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, content: &str, cx: &mut Context<Self>) {
        self.content = content.to_string();
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.needs_layout = true;
        cx.notify();
    }

    // pub fn style(&self) -> &TextAreaStyle {
    //     &self.style
    // }

    // pub fn set_style(&mut self, style: TextAreaStyle, cx: &mut Context<Self>) {
    //     self.style = style;
    //     self.needs_layout = true;
    //     cx.notify();
    // }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let mut new_pos = self.previous_boundary(self.cursor_offset());

            // Skip over newline characters - cursor shouldn't land on them
            if new_pos < self.content.len() {
                if let Some(ch) = self.content[new_pos..].chars().next() {
                    if ch == '\n' {
                        new_pos = self.previous_boundary(new_pos);
                    }
                }
            }

            self.move_to(new_pos, cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let mut new_pos = self.next_boundary(self.selected_range.end);

            // Skip over newline characters - cursor shouldn't land on them
            if new_pos < self.content.len() {
                if let Some(ch) = self.content[new_pos..].chars().next() {
                    if ch == '\n' {
                        new_pos = self.next_boundary(new_pos);
                    }
                }
            }

            self.move_to(new_pos, cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    fn up(&mut self, _: &Up, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_offset) = self.move_vertically(self.cursor_offset(), -1) {
            self.selected_range = new_offset..new_offset;
            self.selection_reversed = false;
            self.scroll_to_cursor();
            cx.notify();
        }
    }

    fn down(&mut self, _: &Down, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_offset) = self.move_vertically(self.cursor_offset(), 1) {
            self.selected_range = new_offset..new_offset;
            self.selection_reversed = false;
            self.scroll_to_cursor();
            cx.notify();
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_up(&mut self, _: &SelectUp, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_offset) = self.move_vertically(self.cursor_offset(), -1) {
            if self.selection_reversed {
                self.selected_range.start = new_offset;
            } else {
                self.selected_range.end = new_offset;
            }
            if self.selected_range.end < self.selected_range.start {
                self.selection_reversed = !self.selection_reversed;
                self.selected_range = self.selected_range.end..self.selected_range.start;
            }
            self.scroll_to_cursor();
            cx.notify();
        }
    }

    fn select_down(&mut self, _: &SelectDown, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_offset) = self.move_vertically(self.cursor_offset(), 1) {
            if self.selection_reversed {
                self.selected_range.start = new_offset;
            } else {
                self.selected_range.end = new_offset;
            }
            if self.selected_range.end < self.selected_range.start {
                self.selection_reversed = !self.selection_reversed;
                self.selected_range = self.selected_range.end..self.selected_range.start;
            }
            self.scroll_to_cursor();
            cx.notify();
        }
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.content.len();
        self.selection_reversed = false;
        cx.notify();
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        let line_start = self.find_line_start(self.cursor_offset());
        self.move_to(line_start, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        let line_end = self.find_line_end(self.cursor_offset());
        self.move_to(line_end, cx);
    }

    fn move_to_beginning(&mut self, _: &MoveToBeginning, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn move_to_end(&mut self, _: &MoveToEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    fn select_to_beginning(
        &mut self,
        _: &SelectToBeginning,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(0, cx);
    }

    fn select_to_end(&mut self, _: &SelectToEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.content.len(), cx);
    }

    fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        let new_pos = self.previous_word_boundary(self.cursor_offset());
        self.move_to(new_pos, cx);
    }

    fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        let new_pos = self.next_word_boundary(self.cursor_offset());
        self.move_to(new_pos, cx);
    }

    fn select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        let new_pos = self.previous_word_boundary(self.cursor_offset());
        self.select_to(new_pos, cx);
    }

    fn select_word_right(&mut self, _: &SelectWordRight, _: &mut Window, cx: &mut Context<Self>) {
        let new_pos = self.next_word_boundary(self.cursor_offset());
        self.select_to(new_pos, cx);
    }

    fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_text_in_range(None, "\n", window, cx);
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle);
        self.is_selecting = true;

        let is_same_position = self
            .last_click_position
            .map(|last| {
                let threshold = px(4.);
                (event.position.x - last.x).abs() < threshold
                    && (event.position.y - last.y).abs() < threshold
            })
            .unwrap_or(false);

        if is_same_position && event.click_count > 1 {
            self.click_count = event.click_count;
        } else {
            self.click_count = 1;
        }
        self.last_click_position = Some(event.position);

        let adjusted_position = self.adjust_mouse_position_for_scroll(event.position);
        let clicked_offset = self.index_for_mouse_position(adjusted_position);

        match self.click_count {
            2 => {
                let (word_start, word_end) = self.word_range_at(clicked_offset);
                self.selected_range = word_start..word_end;
                self.selection_reversed = false;
                cx.notify();
            }
            3 => {
                let line_start = self.find_line_start(clicked_offset);
                let line_end = self.find_line_end(clicked_offset);
                let line_end_with_newline = if line_end < self.content.len() {
                    line_end + 1
                } else {
                    line_end
                };
                self.selected_range = line_start..line_end_with_newline;
                self.selection_reversed = false;
                cx.notify();
            }
            _ => {
                if event.modifiers.shift {
                    self.select_to(clicked_offset, cx);
                } else {
                    self.move_to(clicked_offset, cx);
                }
            }
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting && self.click_count == 1 {
            let adjusted_position = self.adjust_mouse_position_for_scroll(event.position);
            self.select_to(self.index_for_mouse_position(adjusted_position), cx);
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
            self.replace_text_in_range(None, "", window, cx);
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = offset.min(self.content.len());
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        self.scroll_to_cursor();
        cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = offset.min(self.content.len());
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        self.scroll_to_cursor();
        cx.notify();
    }

    fn find_line_start(&self, offset: usize) -> usize {
        self.content[..offset.min(self.content.len())]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0)
    }

    fn find_line_end(&self, offset: usize) -> usize {
        self.content[offset.min(self.content.len())..]
            .find('\n')
            .map(|pos| offset + pos)
            .unwrap_or(self.content.len())
    }

    fn move_vertically(&self, offset: usize, direction: i32) -> Option<usize> {
        let (visual_line_idx, x_pixels) = self.find_visual_line_and_x_offset(offset);
        let target_visual_line_idx = (visual_line_idx as i32 + direction).max(0) as usize;

        let mut current_visual_line = 0;
        for layout in &self.line_layouts {
            let visual_lines_in_layout = layout.visual_line_count;

            if target_visual_line_idx < current_visual_line + visual_lines_in_layout {
                let visual_line_within_layout = target_visual_line_idx - current_visual_line;

                if layout.text_range.is_empty() {
                    return Some(layout.text_range.start);
                }

                if let Some(wrapped) = &layout.wrapped_line {
                    let y_within_wrapped =
                        self.style.line_height * visual_line_within_layout as f32;
                    let target_point = point(px(x_pixels), y_within_wrapped);

                    let closest_idx = wrapped
                        .closest_index_for_position(target_point, self.style.line_height)
                        .unwrap_or_else(|closest| closest);

                    return Some(layout.text_range.start + closest_idx.min(wrapped.text.len()));
                }

                return Some(layout.text_range.start);
            }

            current_visual_line += visual_lines_in_layout;
        }

        if direction > 0 {
            Some(self.content.len())
        } else {
            None
        }
    }

    fn find_visual_line_and_x_offset(&self, offset: usize) -> (usize, f32) {
        if self.line_layouts.is_empty() {
            return (0, 0.0);
        }

        let mut visual_line_idx = 0;

        for line in &self.line_layouts {
            if line.text_range.is_empty() {
                if offset == line.text_range.start {
                    return (visual_line_idx, 0.0);
                }
            } else if offset >= line.text_range.start && offset <= line.text_range.end {
                if let Some(wrapped) = &line.wrapped_line {
                    let local_offset = (offset - line.text_range.start).min(wrapped.text.len());
                    if let Some(position) =
                        wrapped.position_for_index(local_offset, self.style.line_height)
                    {
                        let visual_line_within =
                            (position.y / self.style.line_height).floor() as usize;
                        return (visual_line_idx + visual_line_within, position.x.into());
                    }
                }
                return (visual_line_idx, 0.0);
            }
            visual_line_idx += line.visual_line_count;
        }

        (visual_line_idx.saturating_sub(1), 0.0)
    }

    fn adjust_mouse_position_for_scroll(&self, position: Point<Pixels>) -> Point<Pixels> {
        let padding = &self.style.padding;
        point(
            position.x - padding.left,
            position.y - padding.top + self.scroll_offset,
        )
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }

        for line in &self.line_layouts {
            let line_height_total = self.style.line_height * line.visual_line_count as f32;

            if position.y >= line.y_offset && position.y < line.y_offset + line_height_total {
                if line.text_range.is_empty() {
                    return line.text_range.start;
                }

                if let Some(wrapped) = &line.wrapped_line {
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

    fn scroll_to_cursor(&mut self) {
        if self.line_layouts.is_empty() || self.available_height <= px(0.) {
            return;
        }

        let cursor_offset = self.selected_range.start;
        let padding = self.style.padding.clone();
        let line_height = self.style.line_height;

        // Find the line containing the cursor
        for line in &self.line_layouts {
            let is_cursor_in_line = if line.text_range.is_empty() {
                cursor_offset == line.text_range.start
            } else {
                line.text_range.contains(&cursor_offset)
                    || (cursor_offset == line.text_range.end && cursor_offset == self.content.len())
            };

            if is_cursor_in_line {
                let cursor_visual_y = if let Some(wrapped) = &line.wrapped_line {
                    let local_offset = cursor_offset.saturating_sub(line.text_range.start);
                    if let Some(position) =
                        wrapped.position_for_index(local_offset, self.style.line_height)
                    {
                        line.y_offset + position.y
                    } else {
                        line.y_offset
                    }
                } else {
                    line.y_offset
                };

                // Ensure cursor is visible within the scrollable area
                let visible_top = self.scroll_offset;
                let visible_bottom =
                    self.scroll_offset + self.available_height - padding.top - padding.bottom;

                if cursor_visual_y < visible_top {
                    // Cursor is above visible area, scroll up
                    self.scroll_offset = cursor_visual_y;
                } else if cursor_visual_y + line_height > visible_bottom {
                    // Cursor is below visible area, scroll down
                    self.scroll_offset = (cursor_visual_y + line_height)
                        - (self.available_height - padding.top - padding.bottom);
                }

                // Clamp scroll offset
                self.scroll_offset = self.scroll_offset.max(px(0.));
                break;
            }
        }
    }

    fn update_line_layouts(&mut self, width: Pixels, window: &mut Window) {
        if !self.needs_layout && self.wrap_width == Some(width) {
            return;
        }

        self.line_layouts.clear();
        self.wrap_width = Some(width);

        let font_size = self.style.font_size;
        let line_height = self.style.line_height;

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

        let mut y_offset = px(0.);
        let text_style = window.text_style();
        let mut current_pos = 0;

        while current_pos < self.content.len() {
            let line_end = self.content[current_pos..]
                .find('\n')
                .map(|pos| current_pos + pos)
                .unwrap_or(self.content.len());

            let line_text = &self.content[current_pos..line_end];

            if line_text.is_empty() {
                self.line_layouts.push(LineLayout {
                    text_range: current_pos..current_pos,
                    wrapped_line: None,
                    y_offset,
                    visual_line_count: 1,
                });
                y_offset += line_height;
            } else {
                let run = TextRun {
                    len: line_text.len(),
                    font: text_style.font(),
                    color: self.style.text_color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };

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

                for wrapped in wrapped_lines {
                    let visual_line_count = wrapped.wrap_boundaries().len() + 1;
                    let line_height_total = line_height * visual_line_count as f32;

                    self.line_layouts.push(LineLayout {
                        text_range: current_pos..line_end,
                        wrapped_line: Some(wrapped),
                        y_offset,
                        visual_line_count,
                    });

                    y_offset += line_height_total;
                }
            }

            current_pos = if line_end < self.content.len() {
                line_end + 1
            } else {
                self.content.len()
            };
        }

        if self.content.ends_with('\n') {
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

        for character in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += character.len_utf16();
            utf8_offset += character.len_utf8();
        }

        utf8_offset.min(self.content.len())
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for character in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += character.len_utf8();
            utf16_offset += character.len_utf16();
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
        if offset == 0 {
            return 0;
        }

        let text_before = &self.content[..offset.min(self.content.len())];
        text_before
            .grapheme_indices(true)
            .map(|(i, _)| i)
            .last()
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        if offset >= self.content.len() {
            return self.content.len();
        }

        let text_after = &self.content[offset..];
        text_after
            .grapheme_indices(true)
            .nth(1)
            .map(|(i, _)| offset + i)
            .unwrap_or(self.content.len())
    }

    fn previous_word_boundary(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }

        let text_before = &self.content[..offset.min(self.content.len())];

        let mut last_word_start = 0;
        for (idx, _) in text_before.unicode_word_indices() {
            if idx < offset {
                last_word_start = idx;
            }
        }

        if last_word_start == 0 && offset > 0 {
            let trimmed = text_before.trim_end();
            if trimmed.is_empty() {
                return 0;
            }
            for (idx, _) in trimmed.unicode_word_indices() {
                last_word_start = idx;
            }
        }

        last_word_start
    }

    fn next_word_boundary(&self, offset: usize) -> usize {
        if offset >= self.content.len() {
            return self.content.len();
        }

        let text_after = &self.content[offset..];

        for (idx, word) in text_after.unicode_word_indices() {
            let word_end = offset + idx + word.len();
            if word_end > offset {
                return word_end;
            }
        }

        self.content.len()
    }

    fn word_range_at(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.content.len());

        for (idx, word) in self.content.unicode_word_indices() {
            let word_end = idx + word.len();
            if offset >= idx && offset <= word_end {
                return (idx, word_end);
            }
        }

        (offset, offset)
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
        let clamped_range = range.start.min(self.content.len())..range.end.min(self.content.len());
        adjusted_range.replace(self.range_to_utf16(&clamped_range));
        Some(self.content[clamped_range].to_string())
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

        let range = range.start.min(self.content.len())..range.end.min(self.content.len());

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

        let range = range.start.min(self.content.len())..range.end.min(self.content.len());

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
            .map(|new_range| new_range.start + range.start..new_range.end + range.start)
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

        for line in &self.line_layouts {
            if line.text_range.is_empty() {
                if range.start == line.text_range.start {
                    return Some(Bounds::from_corners(
                        point(bounds.left(), bounds.top() + line.y_offset),
                        point(
                            bounds.left() + px(4.),
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

                    let start_visual_line = (start_pos.y / self.style.line_height).floor() as usize;
                    let end_visual_line = (end_pos.y / self.style.line_height).floor() as usize;

                    if start_visual_line == end_visual_line {
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

struct TextAreaElement {
    area: Entity<TextArea>,
}

impl TextAreaElement {
    pub fn new(area: Entity<TextArea>) -> Self {
        Self { area }
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
    type PrepaintState = (Bounds<Pixels>, Option<Hitbox>);

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
        let style_info = self.area.read(cx).style.clone();

        let mut style = Style::default();
        let mut text_style = TextStyleRefinement::default();

        text_style.font_size = Some(style_info.font_size.into());
        text_style.line_height = Some(style_info.line_height.into());
        text_style.color = Some(style_info.text_color);

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
        let (total_content_height, padding) = self.area.update(cx, |area, _| {
            let padding = area.style.padding.clone();
            let content_width = bounds.size.width - padding.left - padding.right;
            area.available_height = bounds.size.height;
            area.update_line_layouts(content_width, window);

            let total_height = area
                .line_layouts
                .last()
                .map(|last| last.y_offset + area.style.line_height * last.visual_line_count as f32)
                .unwrap_or(px(0.));

            (total_height, padding)
        });

        let content_height = bounds.size.height - padding.top - padding.bottom;
        let hitbox = if total_content_height > content_height {
            Some(window.insert_hitbox(bounds, HitboxBehavior::Normal))
        } else {
            None
        };

        (bounds, hitbox)
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
        let (
            focus_handle,
            content,
            selected_range,
            placeholder,
            line_layouts,
            style,
            scroll_offset,
            entity,
        ) = {
            let area_state = self.area.read(cx);
            (
                area_state.focus_handle.clone(),
                area_state.content.clone(),
                area_state.selected_range.clone(),
                area_state.placeholder.clone(),
                area_state.line_layouts.clone(),
                area_state.style.clone(),
                area_state.scroll_offset,
                self.area.clone(),
            )
        };

        let padding = style.padding.clone();
        let content_bounds = Bounds {
            origin: point(
                bounds.origin.x + padding.left,
                bounds.origin.y + padding.top,
            ),
            size: size(
                bounds.size.width - padding.left - padding.right,
                bounds.size.height - padding.top - padding.bottom,
            ),
        };

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(content_bounds, self.area.clone()),
            cx,
        );

        // Handle scroll wheel events if hitbox was created in prepaint
        if let Some(hitbox) = _prepaint.1.clone() {
            let total_content_height = line_layouts
                .last()
                .map(|last| last.y_offset + style.line_height * last.visual_line_count as f32)
                .unwrap_or(px(0.));

            let area_for_scroll = entity.clone();
            let max_scroll = (total_content_height - content_bounds.size.height).max(px(0.));

            window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble && hitbox.is_hovered(window) {
                    let pixel_delta = event.delta.pixel_delta(px(20.));
                    let _ = area_for_scroll.update(cx, |area, cx| {
                        area.scroll_offset =
                            (area.scroll_offset - pixel_delta.y).clamp(px(0.), max_scroll);
                        cx.notify();
                    });
                }
            });
        }

        // Wrap all drawing in content mask for proper clipping
        window.with_content_mask(
            Some(ContentMask {
                bounds: content_bounds,
            }),
            |window| {
                // Draw selection
                if !selected_range.is_empty() {
                    for line in &line_layouts {
                        let line_start = line.text_range.start;
                        let line_end = line.text_range.end;

                        let line_y = line.y_offset - scroll_offset;

                        // Skip lines outside visible area
                        if (line_y + style.line_height * line.visual_line_count as f32) < px(0.)
                            || line_y > content_bounds.size.height
                        {
                            continue;
                        }

                        if selected_range.end > line_start
                            && (line.text_range.is_empty() || selected_range.start < line_end)
                        {
                            if line.text_range.is_empty() {
                                if selected_range.start <= line_start
                                    && selected_range.end > line_start
                                {
                                    window.paint_quad(fill(
                                        Bounds::from_corners(
                                            point(
                                                content_bounds.left(),
                                                content_bounds.top() + line_y,
                                            ),
                                            point(
                                                content_bounds.left() + px(4.),
                                                content_bounds.top() + line_y + style.line_height,
                                            ),
                                        ),
                                        rgba(0x3311ff30),
                                    ));
                                }
                            } else if let Some(wrapped) = &line.wrapped_line {
                                let sel_start = selected_range.start.max(line_start) - line_start;
                                let sel_end = selected_range.end.min(line_end) - line_start;

                                let start_pos = wrapped
                                    .position_for_index(sel_start, style.line_height)
                                    .unwrap_or(point(px(0.), px(0.)));
                                let end_pos = wrapped
                                    .position_for_index(sel_end, style.line_height)
                                    .unwrap_or_else(|| {
                                        let last_line_y =
                                            style.line_height * (line.visual_line_count - 1) as f32;
                                        point(wrapped.width(), last_line_y)
                                    });

                                let start_visual_line =
                                    (start_pos.y / style.line_height).floor() as usize;
                                let end_visual_line =
                                    (end_pos.y / style.line_height).floor() as usize;

                                if start_visual_line == end_visual_line {
                                    window.paint_quad(fill(
                                        Bounds::from_corners(
                                            point(
                                                content_bounds.left() + start_pos.x,
                                                content_bounds.top() + line_y + start_pos.y,
                                            ),
                                            point(
                                                content_bounds.left() + end_pos.x,
                                                content_bounds.top()
                                                    + line_y
                                                    + start_pos.y
                                                    + style.line_height,
                                            ),
                                        ),
                                        rgba(0x3311ff30),
                                    ));
                                } else {
                                    // First line (partial)
                                    window.paint_quad(fill(
                                        Bounds::from_corners(
                                            point(
                                                content_bounds.left() + start_pos.x,
                                                content_bounds.top() + line_y + start_pos.y,
                                            ),
                                            point(
                                                content_bounds.left() + wrapped.width(),
                                                content_bounds.top()
                                                    + line_y
                                                    + start_pos.y
                                                    + style.line_height,
                                            ),
                                        ),
                                        rgba(0x3311ff30),
                                    ));

                                    // Middle lines (full width)
                                    for visual_line in (start_visual_line + 1)..end_visual_line {
                                        let y = style.line_height * visual_line as f32;
                                        window.paint_quad(fill(
                                            Bounds::from_corners(
                                                point(
                                                    content_bounds.left(),
                                                    content_bounds.top() + line_y + y,
                                                ),
                                                point(
                                                    content_bounds.left() + wrapped.width(),
                                                    content_bounds.top()
                                                        + line_y
                                                        + y
                                                        + style.line_height,
                                                ),
                                            ),
                                            rgba(0x3311ff30),
                                        ));
                                    }

                                    // Last line (partial)
                                    window.paint_quad(fill(
                                        Bounds::from_corners(
                                            point(
                                                content_bounds.left(),
                                                content_bounds.top() + line_y + end_pos.y,
                                            ),
                                            point(
                                                content_bounds.left() + end_pos.x,
                                                content_bounds.top()
                                                    + line_y
                                                    + end_pos.y
                                                    + style.line_height,
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
                let text_style_ref = window.text_style();

                if content.is_empty() {
                    let run = TextRun {
                        len: placeholder.len(),
                        font: text_style_ref.font(),
                        color: rgba(0x00000033).into(),
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    };
                    let shaped_line =
                        window
                            .text_system()
                            .shape_line(placeholder, style.font_size, &[run], None);
                    shaped_line
                        .paint(content_bounds.origin, style.line_height, window, cx)
                        .ok();
                } else {
                    for line_layout in &line_layouts {
                        let line_y = line_layout.y_offset - scroll_offset;

                        // Skip lines outside visible area
                        if (line_y + style.line_height * line_layout.visual_line_count as f32)
                            < px(0.)
                            || line_y > content_bounds.size.height
                        {
                            continue;
                        }

                        if let Some(wrapped) = &line_layout.wrapped_line {
                            let paint_pos =
                                point(content_bounds.left(), content_bounds.top() + line_y);
                            wrapped
                                .paint(
                                    paint_pos,
                                    style.line_height,
                                    gpui::TextAlign::Left,
                                    Some(content_bounds),
                                    window,
                                    cx,
                                )
                                .ok();
                        }
                    }
                }

                // Draw cursor
                if focus_handle.is_focused(window) && selected_range.is_empty() {
                    let cursor_offset = selected_range.start;

                    for line in &line_layouts {
                        let line_y = line.y_offset - scroll_offset;

                        // Skip lines outside visible area
                        if (line_y + style.line_height * line.visual_line_count as f32) < px(0.)
                            || line_y > content_bounds.size.height
                        {
                            continue;
                        }

                        let is_cursor_in_line = if line.text_range.is_empty() {
                            cursor_offset == line.text_range.start
                        } else {
                            line.text_range.contains(&cursor_offset)
                                || (cursor_offset == line.text_range.end
                                    && cursor_offset == content.len())
                        };

                        if is_cursor_in_line {
                            let cursor_position = if let Some(wrapped) = &line.wrapped_line {
                                let local_offset =
                                    cursor_offset.saturating_sub(line.text_range.start);
                                wrapped
                                    .position_for_index(local_offset, style.line_height)
                                    .unwrap_or(point(px(0.), px(0.)))
                            } else {
                                point(px(0.), px(0.))
                            };

                            window.paint_quad(fill(
                                Bounds::new(
                                    point(
                                        content_bounds.left() + cursor_position.x,
                                        content_bounds.top() + line_y + cursor_position.y,
                                    ),
                                    size(px(2.), style.line_height),
                                ),
                                gpui::blue(),
                            ));
                            break;
                        }
                    }
                }
            },
        );
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
            .on_action(cx.listener(Self::move_to_beginning))
            .on_action(cx.listener(Self::move_to_end))
            .on_action(cx.listener(Self::select_to_beginning))
            .on_action(cx.listener(Self::select_to_end))
            .on_action(cx.listener(Self::word_left))
            .on_action(cx.listener(Self::word_right))
            .on_action(cx.listener(Self::select_word_left))
            .on_action(cx.listener(Self::select_word_right))
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    // ============================================================
    // BASIC MOVEMENT
    // ============================================================

    #[gpui::test]
    fn test_left_at_start_of_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.left(&Left, window, cx);
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_left_moves_by_grapheme(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at position 3: "hel|lo"
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.left(&Left, window, cx);
            // cursor now at position 2: "he|llo"
            assert_eq!(area.selected_range, 2..2);
        });
    }

    #[gpui::test]
    fn test_left_collapses_selection_to_start(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // selection: "h[ell]o" (positions 1..4)
            area.selected_range = 1..4;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.left(&Left, window, cx);
            // cursor should collapse to start of selection: "h|ello"
            assert_eq!(area.selected_range, 1..1);
        });
    }

    #[gpui::test]
    fn test_left_skips_newline_character(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("ab\ncd", cx);
            // cursor at start of second line: "ab\n|cd" (position 3)
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.left(&Left, window, cx);
            // should skip the newline and land on 'b': "a|b\ncd" (position 1)
            assert_eq!(area.selected_range, 1..1);
        });
    }

    #[gpui::test]
    fn test_right_at_end_of_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at end: "hello|"
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx);
            assert_eq!(area.selected_range, 5..5);
        });
    }

    #[gpui::test]
    fn test_right_moves_by_grapheme(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at position 2: "he|llo"
            area.selected_range = 2..2;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx);
            // cursor now at position 3: "hel|lo"
            assert_eq!(area.selected_range, 3..3);
        });
    }

    #[gpui::test]
    fn test_right_collapses_selection_to_end(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // selection: "h[ell]o" (positions 1..4)
            area.selected_range = 1..4;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx);
            // cursor should collapse to end of selection: "hell|o"
            assert_eq!(area.selected_range, 4..4);
        });
    }

    #[gpui::test]
    fn test_right_skips_newline_character(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("ab\ncd", cx);
            // cursor on 'b': "a|b\ncd" (position 1)
            area.selected_range = 1..1;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx);
            // moves to position 2 (the \n), then skips it to position 3 (c)
            assert_eq!(area.selected_range, 3..3);
        });
    }

    #[gpui::test]
    fn test_up_from_first_line_stays_put(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor on first line: "fir|st\nsecond"
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.up(&Up, window, cx);
            // should stay at position 3 since we're on the first line
            assert_eq!(area.selected_range, 3..3);
        });
    }

    #[gpui::test]
    fn test_up_moves_to_previous_line(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor on second line: "first\nsec|ond" (position 9)
            area.selected_range = 9..9;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.up(&Up, window, cx);
            // should move to first line at similar x position: "fir|st\nsecond"
            assert_eq!(area.selected_range, 3..3);
        });
    }

    #[gpui::test]
    fn test_up_maintains_column_position(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // equal length lines to verify column is maintained
            area.set_content("abcdef\nghijkl", cx);
            // cursor on second line: "abcdef\nghij|kl" (position 11)
            area.selected_range = 11..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.up(&Up, window, cx);
            // should move to same column on first line: "abcd|ef\nghijkl" (position 4)
            assert_eq!(area.selected_range, 4..4);
        });
    }

    #[gpui::test]
    fn test_down_from_last_line_stays_put(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nlast", cx);
            // cursor on last line: "first\nla|st" (position 8)
            area.selected_range = 8..8;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.down(&Down, window, cx);
            // should move to end of content since we're on last line
            assert_eq!(area.selected_range, 10..10);
        });
    }

    #[gpui::test]
    fn test_down_moves_to_next_line(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor on first line: "fir|st\nsecond" (position 3)
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.down(&Down, window, cx);
            // should move to second line at similar x position: "first\nsec|ond"
            assert_eq!(area.selected_range, 9..9);
        });
    }

    #[gpui::test]
    fn test_down_maintains_column_position(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // equal length lines to verify column is maintained
            area.set_content("abcdef\nghijkl", cx);
            // cursor on first line: "abcd|ef\nghijkl" (position 4)
            area.selected_range = 4..4;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.down(&Down, window, cx);
            // should move to same column on second line: "abcdef\nghij|kl" (position 11)
            assert_eq!(area.selected_range, 11..11);
        });
    }

    #[gpui::test]
    fn test_vertical_movement_clamps_to_short_lines(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // middle line is shorter - tests column clamping behavior
            area.set_content("abcdef\nxy\nabcdef", cx);
            // cursor on first line: "abcd|ef\nxy\nabcdef" (position 4)
            area.selected_range = 4..4;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.down(&Down, window, cx);
            // column 4 exceeds "xy" length, clamps to end: "abcdef\nxy|\nabcdef" (position 9)
            assert_eq!(area.selected_range, 9..9);

            area.down(&Down, window, cx);
            // from end of short line, moves to column based on current x position
            // (no "sticky column" memory - this is current behavior)
            assert_eq!(area.selected_range, 12..12);
        });
    }

    #[gpui::test]
    fn test_home_moves_to_line_start(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor in middle of second line: "first\nsec|ond" (position 9)
            area.selected_range = 9..9;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.home(&Home, window, cx);
            // should move to start of second line: "first\n|second" (position 6)
            assert_eq!(area.selected_range, 6..6);
        });
    }

    #[gpui::test]
    fn test_end_moves_to_line_end(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor in middle of second line: "first\nse|cond" (position 8)
            area.selected_range = 8..8;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.end(&End, window, cx);
            // should move to end of second line: "first\nsecond|" (position 12)
            assert_eq!(area.selected_range, 12..12);
        });
    }

    #[gpui::test]
    fn test_move_to_beginning_moves_to_document_start(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond\nthird", cx);
            // cursor somewhere in the middle: "first\nsec|ond\nthird" (position 9)
            area.selected_range = 9..9;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.move_to_beginning(&MoveToBeginning, window, cx);
            // should move to position 0
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_move_to_end_moves_to_document_end(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond\nthird", cx);
            // cursor at start
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.move_to_end(&MoveToEnd, window, cx);
            // should move to end of content (position 18)
            assert_eq!(area.selected_range, 18..18);
        });
    }

    // ============================================================
    // WORD MOVEMENT
    // ============================================================
    #[gpui::test]
    fn test_word_left_at_start_of_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.word_left(&WordLeft, window, cx);
            // should stay at 0
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_word_left_skips_whitespace(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello   world", cx);
            // cursor at 'w': "hello   |world" (position 8)
            area.selected_range = 8..8;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.word_left(&WordLeft, window, cx);
            // should skip whitespace and move to start of "hello": "|hello   world"
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_word_left_stops_at_word_boundary(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world test", cx);
            // cursor at end of "world": "hello world| test" (position 11)
            area.selected_range = 11..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.word_left(&WordLeft, window, cx);
            // should move to start of "world": "hello |world test" (position 6)
            assert_eq!(area.selected_range, 6..6);
        });
    }

    #[gpui::test]
    fn test_word_right_at_end_of_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // cursor at end
            area.selected_range = 11..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.word_right(&WordRight, window, cx);
            // should stay at end
            assert_eq!(area.selected_range, 11..11);
        });
    }

    #[gpui::test]
    fn test_word_right_skips_whitespace(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello   world", cx);
            // cursor at end of "hello": "hello|   world" (position 5)
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.word_right(&WordRight, window, cx);
            // should skip whitespace and move to end of "world": "hello   world|"
            assert_eq!(area.selected_range, 13..13);
        });
    }

    #[gpui::test]
    fn test_word_right_stops_at_word_boundary(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world test", cx);
            // cursor at start: "|hello world test" (position 0)
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.word_right(&WordRight, window, cx);
            // should move to end of "hello": "hello| world test" (position 5)
            assert_eq!(area.selected_range, 5..5);
        });
    }

    #[gpui::test]
    fn test_word_boundary_with_punctuation(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello, world", cx);
            // cursor at start
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.word_right(&WordRight, window, cx);
            // should stop at "hello" (comma is not part of word): "hello|, world"
            assert_eq!(area.selected_range, 5..5);

            area.word_right(&WordRight, window, cx);
            // should move to end of "world": "hello, world|"
            assert_eq!(area.selected_range, 12..12);
        });
    }

    // ============================================================
    // SELECTION
    // ============================================================

    #[gpui::test]
    fn test_select_left_extends_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at position 3: "hel|lo"
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_left(&SelectLeft, window, cx);
            // should extend selection left: "he[l]lo" (2..3)
            assert_eq!(area.selected_range, 2..3);
            assert!(area.selection_reversed);
        });
    }

    #[gpui::test]
    fn test_select_left_reverses_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // existing selection forward: "he[l]lo" (2..3)
            area.selected_range = 2..3;
            area.selection_reversed = false;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_left(&SelectLeft, window, cx);
            // cursor at end moves left, making selection: "he[ll]o" then reversing to "he[]lo"
            // actually: end moves from 3 to 2, so selection becomes 2..2
            assert_eq!(area.selected_range, 2..2);
        });
    }

    #[gpui::test]
    fn test_select_right_extends_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at position 2: "he|llo"
            area.selected_range = 2..2;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_right(&SelectRight, window, cx);
            // should extend selection right: "he[l]lo" (2..3)
            assert_eq!(area.selected_range, 2..3);
            assert!(!area.selection_reversed);
        });
    }

    #[gpui::test]
    fn test_select_right_reverses_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // existing reversed selection: "he[l]lo" where cursor is at start
            area.selected_range = 2..3;
            area.selection_reversed = true;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_right(&SelectRight, window, cx);
            // cursor at start (2) moves right to 3, collapsing selection
            assert_eq!(area.selected_range, 3..3);
        });
    }

    #[gpui::test]
    fn test_select_up_extends_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor on second line: "first\nsec|ond" (position 9)
            area.selected_range = 9..9;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.select_up(&SelectUp, window, cx);
            // should select from position 9 up to position 3 on first line
            assert_eq!(area.selected_range, 3..9);
            assert!(area.selection_reversed);
        });
    }

    #[gpui::test]
    fn test_select_down_extends_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor on first line: "fir|st\nsecond" (position 3)
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            area.select_down(&SelectDown, window, cx);
            // should select from position 3 down to position 9 on second line
            assert_eq!(area.selected_range, 3..9);
            assert!(!area.selection_reversed);
        });
    }

    #[gpui::test]
    fn test_select_all_selects_entire_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello\nworld", cx);
            // cursor somewhere in the middle
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_all(&SelectAll, window, cx);
            // should select entire content: "[hello\nworld]" (0..11)
            assert_eq!(area.selected_range, 0..11);
        });
    }

    #[gpui::test]
    fn test_select_to_beginning_selects_to_start(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // cursor at position 6: "hello |world"
            area.selected_range = 6..6;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_to_beginning(&SelectToBeginning, window, cx);
            // should select from cursor to start: "[hello ]world" (0..6)
            assert_eq!(area.selected_range, 0..6);
            assert!(area.selection_reversed);
        });
    }

    #[gpui::test]
    fn test_select_to_end_selects_to_end(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // cursor at position 6: "hello |world"
            area.selected_range = 6..6;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_to_end(&SelectToEnd, window, cx);
            // should select from cursor to end: "hello [world]" (6..11)
            assert_eq!(area.selected_range, 6..11);
            assert!(!area.selection_reversed);
        });
    }

    #[gpui::test]
    fn test_select_word_left_extends_by_word(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // cursor at end of "world": "hello world|" (position 11)
            area.selected_range = 11..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_word_left(&SelectWordLeft, window, cx);
            // should select "world": "hello [world]" (6..11)
            assert_eq!(area.selected_range, 6..11);
            assert!(area.selection_reversed);
        });
    }
    #[gpui::test]
    fn test_select_word_right_extends_by_word(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // cursor at start: "|hello world" (position 0)
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_word_right(&SelectWordRight, window, cx);
            // should select "hello": "[hello] world" (0..5)
            assert_eq!(area.selected_range, 0..5);
            assert!(!area.selection_reversed);
        });
    }

    #[gpui::test]
    fn test_selection_reversed_flag(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor in middle: "hel|lo" (position 3)
            area.selected_range = 3..3;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            // select right - cursor at end, not reversed
            area.select_right(&SelectRight, window, cx);
            assert_eq!(area.selected_range, 3..4);
            assert!(!area.selection_reversed);

            // select left past start - should reverse
            area.select_left(&SelectLeft, window, cx);
            area.select_left(&SelectLeft, window, cx);
            // selection is now 2..3 with cursor at start (reversed)
            assert_eq!(area.selected_range, 2..3);
            assert!(area.selection_reversed);
        });
    }

    // ============================================================
    // EDITING - BACKSPACE
    // ============================================================

    #[gpui::test]
    fn test_backspace_deletes_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // selection: "hello [world]" (6..11)
            area.selected_range = 6..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.backspace(&Backspace, window, cx);
            assert_eq!(area.content(), "hello ");
            assert_eq!(area.selected_range, 6..6);
        });
    }

    #[gpui::test]
    fn test_backspace_deletes_previous_grapheme(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at end: "hello|" (position 5)
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.backspace(&Backspace, window, cx);
            assert_eq!(area.content(), "hell");
            assert_eq!(area.selected_range, 4..4);
        });
    }

    #[gpui::test]
    fn test_backspace_at_start_does_nothing(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at start: "|hello" (position 0)
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.backspace(&Backspace, window, cx);
            assert_eq!(area.content(), "hello");
            assert_eq!(area.selected_range, 0..0);
        });
    }
    #[gpui::test]
    fn test_backspace_deletes_entire_emoji(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("Hi 👋", cx);
            // cursor after emoji: "Hi 👋|" (👋 is 4 bytes, so position 7)
            area.selected_range = 7..7;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.backspace(&Backspace, window, cx);
            assert_eq!(area.content(), "Hi ");
            assert_eq!(area.selected_range, 3..3);
        });
    }

    #[gpui::test]
    fn test_backspace_deletes_multi_codepoint_grapheme(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // Family emoji is a ZWJ sequence: 👨‍👩‍👧 (multiple codepoints joined)
            area.set_content("X👨‍👩‍👧Y", cx);
            let emoji_end = "X👨‍👩‍👧".len();
            area.selected_range = emoji_end..emoji_end;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.backspace(&Backspace, window, cx);
            assert_eq!(area.content(), "XY");
            assert_eq!(area.selected_range, 1..1);
        });
    }

    #[gpui::test]
    fn test_backspace_joins_lines(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor at start of second line: "first\n|second" (position 6)
            area.selected_range = 6..6;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.backspace(&Backspace, window, cx);
            assert_eq!(area.content(), "firstsecond");
            assert_eq!(area.selected_range, 5..5);
        });
    }

    // ============================================================
    // EDITING - DELETE
    // ============================================================

    #[gpui::test]
    fn test_delete_deletes_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // selection: "[hello] world" (0..5)
            area.selected_range = 0..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.delete(&Delete, window, cx);
            assert_eq!(area.content(), " world");
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_delete_deletes_next_grapheme(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at start: "|hello" (position 0)
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.delete(&Delete, window, cx);
            assert_eq!(area.content(), "ello");
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_delete_at_end_does_nothing(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at end: "hello|" (position 5)
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.delete(&Delete, window, cx);
            assert_eq!(area.content(), "hello");
            assert_eq!(area.selected_range, 5..5);
        });
    }

    #[gpui::test]
    fn test_delete_deletes_entire_emoji(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("👋 Hi", cx);
            // cursor at start: "|👋 Hi" (position 0)
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.delete(&Delete, window, cx);
            assert_eq!(area.content(), " Hi");
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_delete_deletes_newline(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond", cx);
            // cursor at end of first line: "first|\nsecond" (position 5)
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.delete(&Delete, window, cx);
            assert_eq!(area.content(), "firstsecond");
            assert_eq!(area.selected_range, 5..5);
        });
    }

    // ============================================================
    // EDITING - ENTER
    // ============================================================

    #[gpui::test]
    fn test_enter_inserts_newline(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // cursor in middle: "hello| world" (position 5)
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.enter(&Enter, window, cx);
            assert_eq!(area.content(), "hello\n world");
            assert_eq!(area.selected_range, 6..6);
        });
    }

    #[gpui::test]
    fn test_enter_replaces_selection_with_newline(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // selection: "hello[ ]world" (5..6)
            area.selected_range = 5..6;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.enter(&Enter, window, cx);
            assert_eq!(area.content(), "hello\nworld");
            assert_eq!(area.selected_range, 6..6);
        });
    }

    #[gpui::test]
    fn test_enter_at_end_of_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // cursor at end: "hello|"
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.enter(&Enter, window, cx);
            assert_eq!(area.content(), "hello\n");
            assert_eq!(area.selected_range, 6..6);
        });
    }

    // ============================================================
    // CLIPBOARD
    // ============================================================

    #[gpui::test]
    fn test_copy_with_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // selection: "hello [world]" (6..11)
            area.selected_range = 6..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.copy(&Copy, window, cx);
        });

        let clipboard = cx.read_from_clipboard();
        assert!(clipboard.is_some());
        assert_eq!(clipboard.unwrap().text().as_deref(), Some("world"));
    }

    #[gpui::test]
    fn test_copy_without_selection_does_nothing(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // no selection, just cursor
            area.selected_range = 3..3;
            area
        });

        // Clear clipboard first
        cx.write_to_clipboard(gpui::ClipboardItem::new_string("initial".to_string()));

        _ = area.update(cx, |area, window, cx| {
            area.copy(&Copy, window, cx);
        });

        // Clipboard should be unchanged
        let clipboard = cx.read_from_clipboard();
        assert_eq!(clipboard.unwrap().text().as_deref(), Some("initial"));
    }

    #[gpui::test]
    fn test_cut_with_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // selection: "[hello] world" (0..5)
            area.selected_range = 0..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.cut(&Cut, window, cx);
            assert_eq!(area.content(), " world");
            assert_eq!(area.selected_range, 0..0);
        });

        let clipboard = cx.read_from_clipboard();
        assert_eq!(clipboard.unwrap().text().as_deref(), Some("hello"));
    }

    #[gpui::test]
    fn test_cut_without_selection_does_nothing(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // no selection
            area.selected_range = 3..3;
            area
        });

        cx.write_to_clipboard(gpui::ClipboardItem::new_string("initial".to_string()));

        _ = area.update(cx, |area, window, cx| {
            area.cut(&Cut, window, cx);
            // content unchanged
            assert_eq!(area.content(), "hello");
        });

        // Clipboard unchanged
        let clipboard = cx.read_from_clipboard();
        assert_eq!(clipboard.unwrap().text().as_deref(), Some("initial"));
    }

    #[gpui::test]
    fn test_paste_inserts_text(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // cursor in middle: "hello| world" (position 5)
            area.selected_range = 5..5;
            area
        });

        cx.write_to_clipboard(gpui::ClipboardItem::new_string(" there".to_string()));

        _ = area.update(cx, |area, window, cx| {
            area.paste(&Paste, window, cx);
            assert_eq!(area.content(), "hello there world");
            assert_eq!(area.selected_range, 11..11);
        });
    }

    #[gpui::test]
    fn test_paste_replaces_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // selection: "[hello] world" (0..5)
            area.selected_range = 0..5;
            area
        });

        cx.write_to_clipboard(gpui::ClipboardItem::new_string("goodbye".to_string()));

        _ = area.update(cx, |area, window, cx| {
            area.paste(&Paste, window, cx);
            assert_eq!(area.content(), "goodbye world");
            assert_eq!(area.selected_range, 7..7);
        });
    }

    // ============================================================
    // UNICODE / GRAPHEME HANDLING
    // ============================================================

    #[gpui::test]
    fn test_movement_with_multibyte_utf8(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // "café" - é is 2 bytes in UTF-8
            area.set_content("café", cx);
            // cursor at start
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx); // c
            assert_eq!(area.selected_range, 1..1);
            area.right(&Right, window, cx); // a
            assert_eq!(area.selected_range, 2..2);
            area.right(&Right, window, cx); // f
            assert_eq!(area.selected_range, 3..3);
            area.right(&Right, window, cx); // é (2 bytes)
            assert_eq!(area.selected_range, 5..5); // "café".len() == 5
        });
    }

    #[gpui::test]
    fn test_movement_with_emoji(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // 👋 is 4 bytes
            area.set_content("a👋b", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx); // past 'a'
            assert_eq!(area.selected_range, 1..1);
            area.right(&Right, window, cx); // past 👋 (4 bytes)
            assert_eq!(area.selected_range, 5..5);
            area.right(&Right, window, cx); // past 'b'
            assert_eq!(area.selected_range, 6..6);
        });
    }

    #[gpui::test]
    fn test_movement_with_combining_characters(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // "é" as e + combining acute accent (2 codepoints, 1 grapheme)
            area.set_content("ae\u{0301}b", cx); // a + e + combining accent + b
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx); // past 'a'
            assert_eq!(area.selected_range, 1..1);
            area.right(&Right, window, cx); // past 'é' (e + combining = 1 grapheme)
            // e is 1 byte, combining acute is 2 bytes = 3 bytes total
            assert_eq!(area.selected_range, 4..4);
            area.right(&Right, window, cx); // past 'b'
            assert_eq!(area.selected_range, 5..5);
        });
    }

    #[gpui::test]
    fn test_movement_with_zwj_emoji_sequence(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // 👨‍👩‍👧 family emoji (ZWJ sequence - multiple codepoints, 1 grapheme)
            area.set_content("X👨‍👩‍👧Y", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx); // past 'X'
            assert_eq!(area.selected_range, 1..1);
            area.right(&Right, window, cx); // past entire family emoji as one grapheme
            let emoji_end = "X👨‍👩‍👧".len();
            assert_eq!(area.selected_range, emoji_end..emoji_end);
            area.right(&Right, window, cx); // past 'Y'
            assert_eq!(area.selected_range, emoji_end + 1..emoji_end + 1);
        });
    }
    #[gpui::test]
    fn test_selection_with_multibyte_characters(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // "日本語" - each character is 3 bytes
            area.set_content("日本語", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.select_right(&SelectRight, window, cx);
            // "日" is 3 bytes
            assert_eq!(area.selected_range, 0..3);

            area.select_right(&SelectRight, window, cx);
            // "日本" is 6 bytes
            assert_eq!(area.selected_range, 0..6);

            area.select_right(&SelectRight, window, cx);
            // "日本語" is 9 bytes
            assert_eq!(area.selected_range, 0..9);
        });
    }

    #[gpui::test]
    fn test_utf16_offset_conversion(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // "café" - é is 2 UTF-8 bytes but 1 UTF-16 code unit
            area.set_content("café", cx);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            // UTF-8 offset 5 (end of string) should be UTF-16 offset 4
            assert_eq!(area.offset_to_utf16(5), 4);
            // UTF-16 offset 4 should be UTF-8 offset 5
            assert_eq!(area.offset_from_utf16(4), 5);

            // UTF-8 offset 3 (before é) should be UTF-16 offset 3
            assert_eq!(area.offset_to_utf16(3), 3);
        });
    }

    #[gpui::test]
    fn test_utf16_offset_conversion_with_surrogate_pairs(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // 👋 is 4 UTF-8 bytes and 2 UTF-16 code units (surrogate pair)
            area.set_content("a👋b", cx);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            // 'a' at UTF-8 offset 0, UTF-16 offset 0
            assert_eq!(area.offset_to_utf16(0), 0);
            // After 'a': UTF-8 offset 1, UTF-16 offset 1
            assert_eq!(area.offset_to_utf16(1), 1);
            // After 👋: UTF-8 offset 5, UTF-16 offset 3 (1 + 2 for surrogate pair)
            assert_eq!(area.offset_to_utf16(5), 3);
            // After 'b': UTF-8 offset 6, UTF-16 offset 4
            assert_eq!(area.offset_to_utf16(6), 4);

            // Reverse conversion
            assert_eq!(area.offset_from_utf16(3), 5);
            assert_eq!(area.offset_from_utf16(4), 6);
        });
    }

    // ============================================================
    // NEWLINE HANDLING
    // ============================================================

    #[gpui::test]
    fn test_cursor_position_after_newline(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("ab\ncd", cx);
            // cursor at 'b': "a|b\ncd" (position 1)
            area.selected_range = 1..1;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.right(&Right, window, cx);
            // Due to newline skipping, cursor should jump from 'b' to 'c': "ab\n|cd"
            assert_eq!(area.selected_range, 3..3);
        });
    }

    #[gpui::test]
    fn test_content_ending_with_newline(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello\n", cx);
            // cursor at end
            area.selected_range = 6..6;
            area
        });

        _ = area.update(cx, |area, window, _cx| {
            area.update_line_layouts(px(200.), window);
            // Should have 2 line layouts: "hello" and empty line after newline
            assert_eq!(area.line_layouts.len(), 2);
            // Last line should be empty
            assert!(area.line_layouts[1].text_range.is_empty());
        });
    }

    #[gpui::test]
    fn test_multiple_consecutive_newlines(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("a\n\n\nb", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);
            // Should have 4 line layouts: "a", "", "", "b"
            assert_eq!(area.line_layouts.len(), 4);

            // Navigate through empty lines
            area.selected_range = 2..2; // first empty line
            area.down(&Down, window, cx);
            assert_eq!(area.selected_range, 3..3); // second empty line
            area.down(&Down, window, cx);
            assert_eq!(area.selected_range, 4..4); // 'b'
        });
    }

    #[gpui::test]
    fn test_find_line_start_and_end(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("first\nsecond\nthird", cx);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            // Test find_line_start
            assert_eq!(area.find_line_start(0), 0); // start of first line
            assert_eq!(area.find_line_start(3), 0); // middle of first line
            assert_eq!(area.find_line_start(5), 0); // end of first line
            assert_eq!(area.find_line_start(6), 6); // start of second line
            assert_eq!(area.find_line_start(9), 6); // middle of second line
            assert_eq!(area.find_line_start(13), 13); // start of third line

            // Test find_line_end
            assert_eq!(area.find_line_end(0), 5); // from start of first line
            assert_eq!(area.find_line_end(3), 5); // from middle of first line
            assert_eq!(area.find_line_end(6), 12); // from start of second line
            assert_eq!(area.find_line_end(13), 18); // from start of third line (to end)
        });
    }

    // ============================================================
    // VERTICAL MOVEMENT EDGE CASES
    // ============================================================

    #[gpui::test]
    fn test_up_down_with_lines_of_different_lengths(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // Short line, long line, short line
            area.set_content("ab\nabcdefgh\nxy", cx);
            // cursor at end of long line: "ab\nabcdefgh|\nxy" (position 11)
            area.selected_range = 11..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);

            area.up(&Up, window, cx);
            // First line is shorter, should clamp to end: "ab|\nabcdefgh\nxy"
            assert_eq!(area.selected_range, 2..2);

            area.selected_range = 11..11; // back to end of long line
            area.down(&Down, window, cx);
            // Third line is shorter, should clamp: "ab\nabcdefgh\nxy|"
            assert_eq!(area.selected_range, 14..14);
        });
    }

    #[gpui::test]
    fn test_up_down_through_empty_lines(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("abc\n\ndef", cx);
            // cursor on first line: "ab|c\n\ndef" (position 2)
            area.selected_range = 2..2;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);

            area.down(&Down, window, cx);
            // empty line at position 4
            assert_eq!(area.selected_range, 4..4);

            area.down(&Down, window, cx);
            // third line "def", position should be at column based on x
            // empty line has x=0, so should land at start of "def"
            assert_eq!(area.selected_range, 5..5);
        });
    }

    #[gpui::test]
    fn test_up_down_with_wrapped_lines(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // Long line that will wrap at narrow width
            area.set_content("abcdefghijklmnop\nxy", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            // Use narrow width to force wrapping
            area.update_line_layouts(px(50.), window);

            // Check that first line wraps (visual_line_count > 1)
            assert!(area.line_layouts[0].visual_line_count > 1);

            // Down from first visual line should move within the wrapped line
            area.down(&Down, window, cx);
            // Should still be within the first logical line
            assert!(area.selected_range.start < 17); // before newline
        });
    }

    // ============================================================
    // TEXT INPUT (EntityInputHandler)
    // ============================================================

    #[gpui::test]
    fn test_replace_text_in_range(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area.selected_range = 0..0;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            // Replace "world" (positions 6..11) with "there"
            let range_utf16 = Some(6..11);
            area.replace_text_in_range(range_utf16, "there", window, cx);
            assert_eq!(area.content(), "hello there");
            // Cursor should be at end of inserted text
            assert_eq!(area.selected_range, 11..11);
        });
    }

    #[gpui::test]
    fn test_replace_text_at_cursor(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            // Select "world"
            area.selected_range = 6..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            // Replace selection with "there" (None uses selected_range)
            area.replace_text_in_range(None, "there", window, cx);
            assert_eq!(area.content(), "hello there");
            assert_eq!(area.selected_range, 11..11);
        });
    }

    #[gpui::test]
    fn test_replace_and_mark_text(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            area.selected_range = 5..5; // at end
            area
        });

        _ = area.update(cx, |area, window, cx| {
            // Simulate IME composition: insert marked text
            area.replace_and_mark_text_in_range(None, " world", None, window, cx);
            assert_eq!(area.content(), "hello world");
            // Text should be marked
            assert_eq!(area.marked_range, Some(5..11));
            // Cursor at end of marked text
            assert_eq!(area.selected_range, 11..11);
        });
    }

    #[gpui::test]
    fn test_unmark_text(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area.marked_range = Some(6..11);
            area.selected_range = 11..11;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.unmark_text(window, cx);
            assert_eq!(area.marked_range, None);
            // Content and selection unchanged
            assert_eq!(area.content(), "hello world");
            assert_eq!(area.selected_range, 11..11);
        });
    }

    #[gpui::test]
    fn test_text_for_range(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area
        });

        _ = area.update(cx, |area, window, cx| {
            let mut adjusted = None;
            let text = area.text_for_range(0..5, &mut adjusted, window, cx);
            assert_eq!(text, Some("hello".to_string()));
            assert_eq!(adjusted, Some(0..5));
        });
    }

    #[gpui::test]
    fn test_selected_text_range(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area.selected_range = 3..8;
            area.selection_reversed = true;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            let selection = area.selected_text_range(false, window, cx);
            assert!(selection.is_some());
            let sel = selection.unwrap();
            assert_eq!(sel.range, 3..8);
            assert!(sel.reversed);
        });
    }

    #[gpui::test]
    fn test_text_for_range_with_unicode(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // "café" has multi-byte 'é' (2 bytes in UTF-8)
            area.set_content("café", cx);
            area
        });

        _ = area.update(cx, |area, window, cx| {
            let mut adjusted = None;
            // UTF-16 range for "café" is 0..4 (é is one UTF-16 unit)
            let text = area.text_for_range(0..4, &mut adjusted, window, cx);
            assert_eq!(text, Some("café".to_string()));
        });
    }

    #[gpui::test]
    fn test_replace_text_with_emoji(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            area.selected_range = 5..5;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            // Insert emoji at end
            area.replace_text_in_range(None, " 👋", window, cx);
            assert_eq!(area.content(), "hello 👋");
        });
    }

    // ============================================================
    // MOUSE INTERACTION
    // ============================================================
    // Note: Visual mouse interaction tests (click positioning, drag selection)
    // require VisualTestContext with known pixel positions to be deterministic.
    // The tests below cover the deterministic helper functions.

    #[gpui::test]
    fn test_index_for_mouse_position_empty_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("", cx);
            area
        });

        _ = area.update(cx, |area, window, _cx| {
            area.update_line_layouts(px(200.), window);

            // Any position on empty content should return 0
            let idx = area.index_for_mouse_position(point(px(50.), px(10.)));
            assert_eq!(idx, 0);
        });
    }

    #[gpui::test]
    fn test_index_for_mouse_position_multiline(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("line1\nline2\nline3", cx);
            area
        });

        _ = area.update(cx, |area, window, _cx| {
            area.update_line_layouts(px(200.), window);

            // Position at very beginning should give 0
            let idx = area.index_for_mouse_position(point(px(0.), px(0.)));
            assert_eq!(idx, 0);

            // Position below all lines should give content length
            let idx = area.index_for_mouse_position(point(px(0.), px(1000.)));
            assert_eq!(idx, area.content.len());
        });
    }

    // ============================================================
    // SCROLLING
    // ============================================================

    #[gpui::test]
    fn test_scroll_to_cursor_on_movement(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            // Create content with many lines
            let content = (0..20)
                .map(|i| format!("line {}", i))
                .collect::<Vec<_>>()
                .join("\n");
            area.set_content(&content, cx);
            area.selected_range = 0..0;
            area.available_height = px(100.); // Small height to enable scrolling
            area
        });

        _ = area.update(cx, |area, window, cx| {
            area.update_line_layouts(px(200.), window);

            // Move to end of content
            area.move_to_end(&MoveToEnd, window, cx);

            // scroll_to_cursor should have been called, scroll_offset may have changed
            // Just verify it doesn't panic
            assert_eq!(area.selected_range.start, area.content.len());
        });
    }

    #[gpui::test]
    fn test_scroll_offset_clamped(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("short", cx);
            area.available_height = px(100.);
            area.scroll_offset = px(-50.); // Invalid negative
            area
        });

        _ = area.update(cx, |area, window, _cx| {
            area.update_line_layouts(px(200.), window);
            area.scroll_to_cursor();

            // Scroll offset should be clamped to 0 minimum
            assert!(area.scroll_offset >= px(0.));
        });
    }

    #[gpui::test]
    fn test_adjust_mouse_position_for_scroll(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            area.scroll_offset = px(20.);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            let original = point(px(50.), px(30.));
            let adjusted = area.adjust_mouse_position_for_scroll(original);

            // Should subtract padding.left from x and add scroll_offset to y
            let padding = &area.style.padding;
            assert_eq!(adjusted.x, original.x - padding.left);
            assert_eq!(adjusted.y, original.y - padding.top + area.scroll_offset);
        });
    }

    // ============================================================
    // EMPTY / EDGE CASES
    // ============================================================

    #[gpui::test]
    fn test_operations_on_empty_content(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("", cx);
            area
        });

        _ = area.update(cx, |area, window, cx| {
            // Left on empty should be safe
            area.left(&Left, window, cx);
            assert_eq!(area.selected_range, 0..0);

            // Right on empty should be safe
            area.right(&Right, window, cx);
            assert_eq!(area.selected_range, 0..0);

            // Backspace on empty should be safe
            area.backspace(&Backspace, window, cx);
            assert_eq!(area.content(), "");

            // Delete on empty should be safe
            area.delete(&Delete, window, cx);
            assert_eq!(area.content(), "");

            // Select all on empty
            area.select_all(&SelectAll, window, cx);
            assert_eq!(area.selected_range, 0..0);
        });
    }

    #[gpui::test]
    fn test_set_content_resets_selection(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area.selected_range = 3..8;
            area.selection_reversed = true;
            area.marked_range = Some(5..7);
            area
        });

        _ = area.update(cx, |area, _window, cx| {
            // Set new content should reset everything
            area.set_content("new content", cx);
            assert_eq!(area.content(), "new content");
            assert_eq!(area.selected_range, 0..0);
            assert_eq!(area.selection_reversed, false);
            assert_eq!(area.marked_range, None);
        });
    }

    #[gpui::test]
    fn test_cursor_clamped_to_content_length(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            // Artificially set cursor beyond content length
            area.selected_range = 100..100;
            area
        });

        _ = area.update(cx, |area, _window, cx| {
            // move_to should clamp to content length
            area.move_to(1000, cx);
            assert_eq!(area.selected_range, 5..5);

            // select_to should also clamp
            area.selected_range = 0..0;
            area.select_to(1000, cx);
            assert_eq!(area.selected_range, 0..5);
        });
    }

    #[gpui::test]
    fn test_previous_boundary_at_start(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            // previous_boundary at 0 should return 0
            assert_eq!(area.previous_boundary(0), 0);
        });
    }

    #[gpui::test]
    fn test_next_boundary_at_end(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello", cx);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            // next_boundary at end should return end
            assert_eq!(area.next_boundary(5), 5);
            // next_boundary beyond end should return end
            assert_eq!(area.next_boundary(100), 5);
        });
    }

    #[gpui::test]
    fn test_word_range_at_boundary(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            // At end of "hello" (position 5) - still considered in "hello"
            let (start, end) = area.word_range_at(5);
            assert_eq!(start, 0);
            assert_eq!(end, 5);

            // Within "world"
            let (start, end) = area.word_range_at(8);
            assert_eq!(start, 6);
            assert_eq!(end, 11);

            // At space position (6 is 'w', so use a position that's truly in whitespace)
            // Actually in "hello world", positions are:
            // h=0, e=1, l=2, l=3, o=4, space=5, w=6...
            // Position 5 is the space, but word_range_at checks offset <= word_end
            // So position 5 matches "hello" (0..5) since 5 <= 5
            // This is intended for double-click behavior
        });
    }

    #[gpui::test]
    fn test_find_line_boundaries_edge_cases(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("line1\nline2\nline3", cx);
            area
        });

        _ = area.update(cx, |area, _window, _cx| {
            // At very start
            assert_eq!(area.find_line_start(0), 0);
            assert_eq!(area.find_line_end(0), 5);

            // At newline character
            assert_eq!(area.find_line_start(5), 0);
            assert_eq!(area.find_line_end(5), 5);

            // Just after newline
            assert_eq!(area.find_line_start(6), 6);
            assert_eq!(area.find_line_end(6), 11);

            // At very end
            assert_eq!(area.find_line_start(17), 12);
            assert_eq!(area.find_line_end(17), 17);
        });
    }

    #[gpui::test]
    fn test_marked_text_range(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area.marked_range = Some(6..11);
            area
        });

        _ = area.update(cx, |area, window, cx| {
            let range = area.marked_text_range(window, cx);
            assert_eq!(range, Some(6..11));
        });
    }

    #[gpui::test]
    fn test_marked_text_range_none(cx: &mut TestAppContext) {
        let area = cx.add_window(|_window, cx| {
            let mut area = TextArea::new(cx);
            area.set_content("hello world", cx);
            area.marked_range = None;
            area
        });

        _ = area.update(cx, |area, window, cx| {
            let range = area.marked_text_range(window, cx);
            assert_eq!(range, None);
        });
    }
}
