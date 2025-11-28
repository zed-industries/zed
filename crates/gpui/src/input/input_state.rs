use std::ops::Range;

use crate::{
    App, Bounds, ClipboardItem, Context, EntityInputHandler, FocusHandle, Focusable, Pixels, Point,
    SharedString, TextRun, UTF16Selection, Window, WrappedLine, actions, point, px,
};
use unicode_segmentation::UnicodeSegmentation;

use super::bidi::{TextDirection, detect_base_direction};

actions!(
    input,
    [
        /// Delete the character before the cursor.
        Backspace,
        /// Delete the character after the cursor.
        Delete,
        /// Insert a tab character at the cursor position.
        Tab,
        /// Move the cursor one character to the left.
        Left,
        /// Move the cursor one character to the right.
        Right,
        /// Move the cursor up one visual line.
        Up,
        /// Move the cursor down one visual line.
        Down,
        /// Extend selection one character to the left.
        SelectLeft,
        /// Extend selection one character to the right.
        SelectRight,
        /// Extend selection up one visual line.
        SelectUp,
        /// Extend selection down one visual line.
        SelectDown,
        /// Select all text content.
        SelectAll,
        /// Move cursor to the start of the current line.
        Home,
        /// Move cursor to the end of the current line.
        End,
        /// Extend selection to the beginning of the content.
        SelectToBeginning,
        /// Extend selection to the end of the content.
        SelectToEnd,
        /// Move cursor to the beginning of the content.
        MoveToBeginning,
        /// Move cursor to the end of the content.
        MoveToEnd,
        /// Paste from clipboard at the cursor position.
        Paste,
        /// Cut selected text to clipboard.
        Cut,
        /// Copy selected text to clipboard.
        Copy,
        /// Insert a newline at the cursor position.
        Enter,
        /// Move cursor one word to the left.
        WordLeft,
        /// Move cursor one word to the right.
        WordRight,
        /// Extend selection one word to the left.
        SelectWordLeft,
        /// Extend selection one word to the right.
        SelectWordRight,
    ]
);

/// `Input` is the state model for text input components. It handles:
/// - Text content storage and manipulation
/// - Selection and cursor management
/// - Keyboard navigation and editing actions
/// - IME (Input Method Editor) support via `EntityInputHandler`
/// ```
pub struct InputState {
    focus_handle: FocusHandle,
    content: String,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    pub(crate) line_height: Pixels,
    pub(crate) line_layouts: Vec<InputLineLayout>,
    pub(crate) wrap_width: Option<Pixels>,
    pub(crate) needs_layout: bool,
    is_selecting: bool,
    last_click_position: Option<Point<Pixels>>,
    click_count: usize,
    /// Scroll offset - vertical for multiline, horizontal for single-line
    pub(crate) scroll_offset: Pixels,
    pub(crate) available_height: Pixels,
    pub(crate) available_width: Pixels,
    multiline: bool,
}

/// Layout information for a single logical line of text in an input.
///
/// A logical line corresponds to content between newlines in the input text.
/// When text wrapping is enabled, a logical line may span multiple visual lines.
#[derive(Clone, Debug)]
pub struct InputLineLayout {
    /// The byte range in the content string that this line covers.
    pub text_range: Range<usize>,
    /// The shaped and wrapped text for this line, if available.
    pub wrapped_line: Option<WrappedLine>,
    /// The vertical offset from the top of the text area in pixels.
    pub y_offset: Pixels,
    /// The number of visual lines this logical line spans (due to wrapping).
    pub visual_line_count: usize,
    /// The base text direction for this line (LTR or RTL).
    pub direction: TextDirection,
}

impl InputState {
    /// Creates a new multiline `Input` with empty content.
    pub fn new_multiline(cx: &mut Context<Self>) -> Self {
        Self::new(cx).multiline(true)
    }

    /// Creates a new singleline `Input` with empty content.
    pub fn new_singleline(cx: &mut Context<Self>) -> Self {
        Self::new(cx).multiline(false)
    }

    /// Creates a new `Input` with the specified multiline setting.
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: String::new(),
            placeholder: SharedString::default(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            line_height: px(18.),
            line_layouts: Vec::new(),
            wrap_width: None,
            needs_layout: true,
            is_selecting: false,
            last_click_position: None,
            click_count: 0,
            scroll_offset: px(0.),
            available_height: px(0.),
            available_width: px(0.),
            multiline: false,
        }
    }

    /// Sets whether this input allows multiple lines.
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// Returns whether this input allows multiple lines.
    pub fn is_multiline(&self) -> bool {
        self.multiline
    }

    /// Returns the current text content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Sets the text content, resetting selection to the beginning.
    pub fn set_content(&mut self, content: impl Into<String>, cx: &mut Context<Self>) {
        let content = content.into();
        self.content = if self.multiline {
            content
        } else {
            // Strip newlines for single-line input
            content.replace('\n', " ").replace('\r', "")
        };
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.needs_layout = true;
        cx.notify();
    }

    /// Returns the placeholder text shown when content is empty.
    pub fn placeholder(&self) -> &SharedString {
        &self.placeholder
    }

    /// Sets the placeholder text.
    pub fn set_placeholder(
        &mut self,
        placeholder: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.placeholder = placeholder.into();
        cx.notify();
    }

    /// Returns the current selection range.
    pub fn selected_range(&self) -> &Range<usize> {
        &self.selected_range
    }

    /// Returns true if the selection is reversed (cursor at start).
    pub fn selection_reversed(&self) -> bool {
        self.selection_reversed
    }

    /// Returns the current cursor offset.
    pub fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    /// Returns the marked text range (for IME composition).
    pub fn marked_range(&self) -> Option<&Range<usize>> {
        self.marked_range.as_ref()
    }

    /// Selects all text.
    pub fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.content.len();
        self.selection_reversed = false;
        cx.notify();
    }

    pub(crate) fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let new_pos = self.previous_boundary(self.cursor_offset());
            self.move_to(new_pos, cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    pub(crate) fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let new_pos = self.next_boundary(self.cursor_offset());
            self.move_to(new_pos, cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    pub(crate) fn up(&mut self, _: &Up, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.multiline {
            // In single-line mode, up moves to start
            self.selected_range = 0..0;
            self.selection_reversed = false;
            self.scroll_to_cursor();
            cx.notify();
            return;
        }
        if let Some(new_offset) = self.move_vertically(self.cursor_offset(), -1) {
            self.selected_range = new_offset..new_offset;
            self.selection_reversed = false;
            self.scroll_to_cursor();
            cx.notify();
        }
    }

    pub(crate) fn down(&mut self, _: &Down, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.multiline {
            // In single-line mode, down moves to end
            let end = self.content.len();
            self.selected_range = end..end;
            self.selection_reversed = false;
            self.scroll_to_cursor();
            cx.notify();
            return;
        }
        if let Some(new_offset) = self.move_vertically(self.cursor_offset(), 1) {
            self.selected_range = new_offset..new_offset;
            self.selection_reversed = false;
            self.scroll_to_cursor();
            cx.notify();
        }
    }

    pub(crate) fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    pub(crate) fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    pub(crate) fn select_up(&mut self, _: &SelectUp, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.multiline {
            // In single-line mode, select_up selects to start
            self.select_to(0, cx);
            return;
        }
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

    pub(crate) fn select_down(
        &mut self,
        _: &SelectDown,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.multiline {
            // In single-line mode, select_down selects to end
            self.select_to(self.content.len(), cx);
            return;
        }
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

    pub(crate) fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        let line_start = self.find_line_start(self.cursor_offset());
        self.move_to(line_start, cx);
    }

    pub(crate) fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        let line_end = self.find_line_end(self.cursor_offset());
        self.move_to(line_end, cx);
    }

    pub(crate) fn move_to_beginning(
        &mut self,
        _: &MoveToBeginning,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(0, cx);
    }

    pub(crate) fn move_to_end(&mut self, _: &MoveToEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    pub(crate) fn select_to_beginning(
        &mut self,
        _: &SelectToBeginning,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(0, cx);
    }

    pub(crate) fn select_to_end(
        &mut self,
        _: &SelectToEnd,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(self.content.len(), cx);
    }

    pub(crate) fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        let new_pos = self.previous_word_boundary(self.cursor_offset());
        self.move_to(new_pos, cx);
    }

    pub(crate) fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        let new_pos = self.next_word_boundary(self.cursor_offset());
        self.move_to(new_pos, cx);
    }

    pub(crate) fn select_word_left(
        &mut self,
        _: &SelectWordLeft,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_pos = self.previous_word_boundary(self.cursor_offset());
        self.select_to(new_pos, cx);
    }

    pub(crate) fn select_word_right(
        &mut self,
        _: &SelectWordRight,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_pos = self.next_word_boundary(self.cursor_offset());
        self.select_to(new_pos, cx);
    }

    pub(crate) fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            self.replace_text_in_range(None, "\n", window, cx);
        }
    }

    pub(crate) fn tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_text_in_range(None, "\t", window, cx);
    }

    pub(crate) fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    pub(crate) fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    pub(crate) fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            if self.multiline {
                self.replace_text_in_range(None, &text, window, cx);
            } else {
                // Strip newlines for single-line input
                let text = text.replace('\n', " ").replace('\r', "");
                self.replace_text_in_range(None, &text, window, cx);
            }
        }
    }

    pub(crate) fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    pub(crate) fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx);
        }
    }

    pub(crate) fn on_mouse_down(
        &mut self,
        position: Point<Pixels>,
        click_count: usize,
        shift: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle);
        self.is_selecting = true;

        let is_same_position = self
            .last_click_position
            .map(|last| {
                let threshold = px(4.);
                (position.x - last.x).abs() < threshold && (position.y - last.y).abs() < threshold
            })
            .unwrap_or(false);

        if is_same_position && click_count > 1 {
            self.click_count = click_count;
        } else {
            self.click_count = 1;
        }
        self.last_click_position = Some(position);

        let clicked_offset = self.index_for_position(position);

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
                if shift {
                    self.select_to(clicked_offset, cx);
                } else {
                    self.move_to(clicked_offset, cx);
                }
            }
        }
    }

    pub(crate) fn on_mouse_up(&mut self, _cx: &mut Context<Self>) {
        self.is_selecting = false;
    }

    pub(crate) fn on_mouse_move(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if self.is_selecting && self.click_count == 1 {
            self.select_to(self.index_for_position(position), cx);
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = offset.min(self.content.len());
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        self.scroll_to_cursor();
        cx.notify();
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

    pub(crate) fn find_line_start(&self, offset: usize) -> usize {
        self.content[..offset.min(self.content.len())]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0)
    }

    pub(crate) fn find_line_end(&self, offset: usize) -> usize {
        self.content[offset.min(self.content.len())..]
            .find('\n')
            .map(|pos| offset + pos)
            .unwrap_or(self.content.len())
    }

    /// Returns the text direction for a specific line layout by index.
    pub fn line_direction(&self, line_idx: usize) -> TextDirection {
        self.line_layouts
            .get(line_idx)
            .map(|layout| layout.direction)
            .unwrap_or_default()
    }

    /// Returns the text direction at a given byte offset in the content.
    pub fn direction_at_offset(&self, offset: usize) -> TextDirection {
        let offset = offset.min(self.content.len());
        for layout in &self.line_layouts {
            if offset >= layout.text_range.start && offset <= layout.text_range.end {
                return layout.direction;
            }
        }
        TextDirection::default()
    }

    fn move_vertically(&self, offset: usize, direction: i32) -> Option<usize> {
        let (visual_line_idx, x_pixels) = self.find_visual_line_and_x_offset(offset);
        let target_visual_line_idx = (visual_line_idx as i32 + direction).max(0) as usize;

        eprintln!(
            "[move_vertically] offset={}, direction={}, visual_line_idx={}, x_pixels={}, target_visual_line_idx={}",
            offset, direction, visual_line_idx, x_pixels, target_visual_line_idx
        );

        let mut current_visual_line = 0;
        for (layout_idx, layout) in self.line_layouts.iter().enumerate() {
            let visual_lines_in_layout = layout.visual_line_count;

            eprintln!(
                "[move_vertically] layout {}: current_visual_line={}, visual_lines_in_layout={}, text_range={:?}",
                layout_idx, current_visual_line, visual_lines_in_layout, layout.text_range
            );

            if target_visual_line_idx < current_visual_line + visual_lines_in_layout {
                let visual_line_within_layout = target_visual_line_idx - current_visual_line;
                eprintln!(
                    "[move_vertically] -> target is in this layout, visual_line_within_layout={}",
                    visual_line_within_layout
                );

                if layout.text_range.is_empty() {
                    eprintln!(
                        "[move_vertically] -> empty line, returning {}",
                        layout.text_range.start
                    );
                    return Some(layout.text_range.start);
                }

                if let Some(wrapped) = &layout.wrapped_line {
                    let y_within_wrapped = self.line_height * visual_line_within_layout as f32;
                    let target_point = point(px(x_pixels), y_within_wrapped);

                    let closest_result =
                        wrapped.closest_index_for_position(target_point, self.line_height);
                    eprintln!(
                        "[move_vertically] -> closest_index_for_position({:?}) = {:?}",
                        target_point, closest_result
                    );

                    let closest_idx = closest_result.unwrap_or_else(|closest| closest);
                    let clamped = closest_idx.min(wrapped.text.len());
                    let result = layout.text_range.start + clamped;

                    eprintln!(
                        "[move_vertically] -> closest_idx={}, wrapped.text.len()={}, clamped={}, result={}",
                        closest_idx,
                        wrapped.text.len(),
                        clamped,
                        result
                    );

                    return Some(result);
                }

                eprintln!(
                    "[move_vertically] -> no wrapped line, returning {}",
                    layout.text_range.start
                );
                return Some(layout.text_range.start);
            }

            current_visual_line += visual_lines_in_layout;
        }

        if direction > 0 {
            eprintln!(
                "[move_vertically] -> past end, returning content.len()={}",
                self.content.len()
            );
            Some(self.content.len())
        } else {
            eprintln!("[move_vertically] -> before start, returning None");
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
                        wrapped.position_for_index(local_offset, self.line_height)
                    {
                        let visual_line_within = (position.y / self.line_height).floor() as usize;
                        return (visual_line_idx + visual_line_within, position.x.into());
                    }
                }
                return (visual_line_idx, 0.0);
            }
            visual_line_idx += line.visual_line_count;
        }

        (visual_line_idx.saturating_sub(1), 0.0)
    }

    pub(crate) fn index_for_position(&self, position: Point<Pixels>) -> usize {
        eprintln!(
            "[index_for_position] position: {:?}, content.len: {}",
            position,
            self.content.len()
        );

        if self.content.is_empty() {
            eprintln!("[index_for_position] empty content, returning 0");
            return 0;
        }

        for (line_idx, line) in self.line_layouts.iter().enumerate() {
            let line_height_total = self.line_height * line.visual_line_count as f32;

            eprintln!(
                "[index_for_position] line {}: y_offset={:?}, height_total={:?}, text_range={:?}",
                line_idx, line.y_offset, line_height_total, line.text_range
            );

            if position.y >= line.y_offset && position.y < line.y_offset + line_height_total {
                eprintln!("[index_for_position] -> position.y in this line's bounds");

                if line.text_range.is_empty() {
                    eprintln!(
                        "[index_for_position] -> empty line, returning {}",
                        line.text_range.start
                    );
                    return line.text_range.start;
                }

                if let Some(wrapped) = &line.wrapped_line {
                    let relative_y = position.y - line.y_offset;
                    let relative_point = point(position.x, relative_y);

                    let closest_result =
                        wrapped.closest_index_for_position(relative_point, self.line_height);
                    eprintln!(
                        "[index_for_position] -> closest_index_for_position({:?}) = {:?}",
                        relative_point, closest_result
                    );

                    let local_idx = closest_result.unwrap_or_else(|closest| closest);
                    let clamped = local_idx.min(wrapped.text.len());
                    let result = line.text_range.start + clamped;

                    eprintln!(
                        "[index_for_position] -> local_idx={}, wrapped.text.len()={}, clamped={}, result={}",
                        local_idx,
                        wrapped.text.len(),
                        clamped,
                        result
                    );
                    eprintln!(
                        "[index_for_position] -> wrapped.text={:?}",
                        wrapped.text.as_ref()
                    );

                    return result;
                }
                eprintln!(
                    "[index_for_position] -> no wrapped line, returning {}",
                    line.text_range.start
                );
                return line.text_range.start;
            }
        }

        eprintln!(
            "[index_for_position] -> no line matched, returning content.len()={}",
            self.content.len()
        );
        self.content.len()
    }

    pub(crate) fn scroll_to_cursor(&mut self) {
        if self.line_layouts.is_empty() {
            return;
        }

        let cursor_offset = if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        };

        if self.multiline {
            self.scroll_to_cursor_vertical(cursor_offset);
        } else {
            self.scroll_to_cursor_horizontal(cursor_offset);
        }
    }

    fn scroll_to_cursor_vertical(&mut self, cursor_offset: usize) {
        if self.available_height <= px(0.) {
            return;
        }

        let line_height = self.line_height;

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
                        wrapped.position_for_index(local_offset, self.line_height)
                    {
                        line.y_offset + position.y
                    } else {
                        line.y_offset
                    }
                } else {
                    line.y_offset
                };

                let visible_top = self.scroll_offset;
                let visible_bottom = self.scroll_offset + self.available_height;

                if cursor_visual_y < visible_top {
                    self.scroll_offset = cursor_visual_y;
                } else if cursor_visual_y + line_height > visible_bottom {
                    self.scroll_offset = (cursor_visual_y + line_height) - self.available_height;
                }

                self.scroll_offset = self.scroll_offset.max(px(0.));
                break;
            }
        }
    }

    fn scroll_to_cursor_horizontal(&mut self, cursor_offset: usize) {
        if self.available_width <= px(0.) {
            return;
        }

        // For single-line input, get cursor x position from the first (only) line
        let Some(line) = self.line_layouts.first() else {
            return;
        };

        let cursor_x = if let Some(wrapped) = &line.wrapped_line {
            let local_offset = cursor_offset.saturating_sub(line.text_range.start);
            wrapped
                .position_for_index(local_offset, self.line_height)
                .map(|p| p.x)
                .unwrap_or(px(0.))
        } else {
            px(0.)
        };

        let visible_left = self.scroll_offset;
        let visible_right = self.scroll_offset + self.available_width;

        // Add some padding so cursor isn't right at the edge
        let padding = px(2.0);

        if cursor_x < visible_left + padding {
            self.scroll_offset = (cursor_x - padding).max(px(0.));
        } else if cursor_x > visible_right - padding {
            self.scroll_offset = cursor_x - self.available_width + padding;
        }

        self.scroll_offset = self.scroll_offset.max(px(0.));
    }

    pub(crate) fn update_line_layouts(
        &mut self,
        width: Pixels,
        line_height: Pixels,
        text_color: impl Into<crate::Hsla>,
        window: &mut Window,
    ) {
        self.line_height = line_height;

        if !self.needs_layout && self.wrap_width == Some(width) {
            return;
        }

        self.line_layouts.clear();
        self.wrap_width = Some(width);

        let text_color = text_color.into();
        let text_style = window.text_style();
        let font_size = text_style.font_size.to_pixels(window.rem_size());

        if self.content.is_empty() {
            self.line_layouts.push(InputLineLayout {
                text_range: 0..0,
                wrapped_line: None,
                y_offset: px(0.),
                visual_line_count: 1,
                direction: TextDirection::default(),
            });
            self.needs_layout = false;
            return;
        }

        let mut last_direction = TextDirection::default();

        let mut y_offset = px(0.);
        let mut current_pos = 0;

        while current_pos < self.content.len() {
            let line_end = self.content[current_pos..]
                .find('\n')
                .map(|pos| current_pos + pos)
                .unwrap_or(self.content.len());

            let line_text = &self.content[current_pos..line_end];

            if line_text.is_empty() {
                self.line_layouts.push(InputLineLayout {
                    text_range: current_pos..current_pos,
                    wrapped_line: None,
                    y_offset,
                    visual_line_count: 1,
                    direction: last_direction,
                });
                y_offset += line_height;
            } else {
                let direction = detect_base_direction(line_text);
                last_direction = direction;
                let run = TextRun {
                    len: line_text.len(),
                    font: text_style.font(),
                    color: text_color,
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

                    self.line_layouts.push(InputLineLayout {
                        text_range: current_pos..line_end,
                        wrapped_line: Some(wrapped),
                        y_offset,
                        visual_line_count,
                        direction,
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
            self.line_layouts.push(InputLineLayout {
                text_range: self.content.len()..self.content.len(),
                wrapped_line: None,
                y_offset,
                visual_line_count: 1,
                direction: last_direction,
            });
        }

        self.needs_layout = false;
        self.scroll_to_cursor();
    }

    pub(crate) fn total_content_height(&self) -> Pixels {
        self.line_layouts
            .last()
            .map(|last| last.y_offset + self.line_height * last.visual_line_count as f32)
            .unwrap_or(px(0.))
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
            .next_back()
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

impl EntityInputHandler for InputState {
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

        // Strip newlines for single-line input
        let sanitized_text;
        let text_to_insert = if self.multiline {
            new_text
        } else {
            sanitized_text = new_text.replace('\n', " ").replace('\r', "");
            &sanitized_text
        };

        self.content =
            self.content[0..range.start].to_owned() + text_to_insert + &self.content[range.end..];
        self.selected_range =
            range.start + text_to_insert.len()..range.start + text_to_insert.len();
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

        // Strip newlines for single-line input
        let sanitized_text;
        let text_to_insert = if self.multiline {
            new_text
        } else {
            sanitized_text = new_text.replace('\n', " ").replace('\r', "");
            &sanitized_text
        };

        self.content =
            self.content[0..range.start].to_owned() + text_to_insert + &self.content[range.end..];

        if !text_to_insert.is_empty() {
            self.marked_range = Some(range.start..range.start + text_to_insert.len());
        } else {
            self.marked_range = None;
        }

        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.start)
            .unwrap_or_else(|| {
                range.start + text_to_insert.len()..range.start + text_to_insert.len()
            });

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
                            bounds.top() + line.y_offset + self.line_height,
                        ),
                    ));
                }
            } else if line.text_range.contains(&range.start) {
                if let Some(wrapped) = &line.wrapped_line {
                    let local_start = range.start - line.text_range.start;
                    let local_end = (range.end - line.text_range.start).min(wrapped.text.len());

                    let start_pos = wrapped
                        .position_for_index(local_start, self.line_height)
                        .unwrap_or(point(px(0.), px(0.)));
                    let end_pos = wrapped
                        .position_for_index(local_end, self.line_height)
                        .unwrap_or_else(|| {
                            let last_line_y =
                                self.line_height * (line.visual_line_count - 1) as f32;
                            point(wrapped.width(), last_line_y)
                        });

                    let start_visual_line = (start_pos.y / self.line_height).floor() as usize;
                    let end_visual_line = (end_pos.y / self.line_height).floor() as usize;

                    if start_visual_line == end_visual_line {
                        return Some(Bounds::from_corners(
                            point(
                                bounds.left() + start_pos.x,
                                bounds.top() + line.y_offset + start_pos.y,
                            ),
                            point(
                                bounds.left() + end_pos.x,
                                bounds.top() + line.y_offset + start_pos.y + self.line_height,
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
                                bounds.top() + line.y_offset + start_pos.y + self.line_height,
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
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let index = self.index_for_position(point);
        Some(self.offset_to_utf16(index))
    }
}

impl Focusable for InputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AppContext, Entity, IntoElement, Render, TestAppContext, div};

    struct TestView {
        input: Entity<InputState>,
    }

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
        }
    }

    fn create_test_input(
        cx: &mut TestAppContext,
        content: &str,
        range: std::ops::Range<usize>,
    ) -> crate::WindowHandle<TestView> {
        cx.add_window(|_window, cx| {
            let input = cx.new(|cx| {
                let mut input = InputState::new_multiline(cx);
                input.content = content.to_string();
                input.selected_range = range;
                input
            });
            TestView { input }
        })
    }

    fn create_test_input_with_layout(
        cx: &mut TestAppContext,
        content: &str,
        range: std::ops::Range<usize>,
    ) -> crate::WindowHandle<TestView> {
        let view = cx.add_window(|window, cx| {
            let input = cx.new(|cx| {
                let mut input = InputState::new_multiline(cx);
                input.content = content.to_string();
                input.selected_range = range;
                input.update_line_layouts(px(500.), px(20.), crate::black(), window);
                input
            });
            TestView { input }
        });
        view
    }

    // ============================================================
    // BASIC MOVEMENT
    // ============================================================

    #[crate::test]
    fn test_left_at_start_of_content(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_left_moves_by_grapheme(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 3..3);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range, 2..2);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_left_collapses_selection_to_start(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 1..4);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range, 1..1);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_left_stops_at_end_of_line(cx: &mut TestAppContext) {
        // "ab\ncd" - cursor at position 3 (start of "cd", after newline)
        // Pressing left should move to position 2 (end of "ab", before newline)
        let view = create_test_input(cx, "ab\ncd", 3..3);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range, 2..2); // cursor at end of line 1
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_right_at_end_of_content(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 5..5);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_right_moves_by_grapheme(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 2..2);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 3..3);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_right_collapses_selection_to_end(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 1..4);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 4..4);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_right_stops_at_end_of_line(cx: &mut TestAppContext) {
        // "ab\ncd" - cursor at position 1 (after 'a')
        // Pressing right should move to position 2 (end of "ab", before newline)
        let view = create_test_input(cx, "ab\ncd", 1..1);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 2..2); // cursor at end of line 1
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_right_crosses_newline(cx: &mut TestAppContext) {
        // "ab\ncd" - cursor at position 2 (end of "ab", before newline)
        // Pressing right should move to position 3 (after newline, start of "cd")
        let view = create_test_input(cx, "ab\ncd", 2..2);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 3..3); // cursor at start of line 2
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_left_crosses_newline(cx: &mut TestAppContext) {
        // "ab\ncd" - cursor at position 2 (end of "ab", before newline)
        // Pressing left should move to position 1 (after 'a')
        let view = create_test_input(cx, "ab\ncd", 2..2);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range, 1..1);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_home_moves_to_line_start(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "first\nsecond", 9..9);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.home(&Home, window, cx);
                assert_eq!(input.selected_range, 6..6);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_end_moves_to_line_end(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "first\nsecond", 8..8);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.end(&End, window, cx);
                assert_eq!(input.selected_range, 12..12);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_move_to_beginning(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "first\nsecond\nthird", 9..9);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.move_to_beginning(&MoveToBeginning, window, cx);
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_move_to_end(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "first\nsecond\nthird", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.move_to_end(&MoveToEnd, window, cx);
                assert_eq!(input.selected_range, 18..18);
            });
        })
        .unwrap();
    }

    // ============================================================
    // WORD MOVEMENT
    // ============================================================

    #[crate::test]
    fn test_word_left_at_start(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.word_left(&WordLeft, window, cx);
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_word_left_stops_at_boundary(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world test", 11..11);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.word_left(&WordLeft, window, cx);
                assert_eq!(input.selected_range, 6..6);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_word_right_at_end(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 11..11);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.word_right(&WordRight, window, cx);
                assert_eq!(input.selected_range, 11..11);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_word_right_stops_at_boundary(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world test", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.word_right(&WordRight, window, cx);
                assert_eq!(input.selected_range, 5..5);
            });
        })
        .unwrap();
    }

    // ============================================================
    // SELECTION
    // ============================================================

    #[crate::test]
    fn test_select_left_extends_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 3..3);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_left(&SelectLeft, window, cx);
                assert_eq!(input.selected_range, 2..3);
                assert!(input.selection_reversed);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_select_right_extends_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 2..2);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_right(&SelectRight, window, cx);
                assert_eq!(input.selected_range, 2..3);
                assert!(!input.selection_reversed);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_select_all(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello\nworld", 3..3);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_all(&SelectAll, window, cx);
                assert_eq!(input.selected_range, 0..11);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_select_to_beginning(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 6..6);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_to_beginning(&SelectToBeginning, window, cx);
                assert_eq!(input.selected_range, 0..6);
                assert!(input.selection_reversed);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_select_to_end(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 6..6);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_to_end(&SelectToEnd, window, cx);
                assert_eq!(input.selected_range, 6..11);
                assert!(!input.selection_reversed);
            });
        })
        .unwrap();
    }

    // ============================================================
    // EDITING - BACKSPACE
    // ============================================================

    #[crate::test]
    fn test_backspace_deletes_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 6..11);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.backspace(&Backspace, window, cx);
                assert_eq!(input.content(), "hello ");
                assert_eq!(input.selected_range, 6..6);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_backspace_deletes_previous_grapheme(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.backspace(&Backspace, window, cx);
                assert_eq!(input.content(), "hell");
                assert_eq!(input.selected_range, 4..4);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_backspace_at_start_does_nothing(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.backspace(&Backspace, window, cx);
                assert_eq!(input.content(), "hello");
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_backspace_deletes_entire_emoji(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "Hi 👋", 7..7);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.backspace(&Backspace, window, cx);
                assert_eq!(input.content(), "Hi ");
                assert_eq!(input.selected_range, 3..3);
            });
        })
        .unwrap();
    }

    // ============================================================
    // EDITING - DELETE
    // ============================================================

    #[crate::test]
    fn test_delete_deletes_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 0..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.delete(&Delete, window, cx);
                assert_eq!(input.content(), " world");
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_delete_deletes_next_grapheme(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.delete(&Delete, window, cx);
                assert_eq!(input.content(), "ello");
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_delete_at_end_does_nothing(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.delete(&Delete, window, cx);
                assert_eq!(input.content(), "hello");
                assert_eq!(input.selected_range, 5..5);
            });
        })
        .unwrap();
    }

    // ============================================================
    // EDITING - ENTER
    // ============================================================

    #[crate::test]
    fn test_enter_inserts_newline(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.enter(&Enter, window, cx);
                assert_eq!(input.content(), "hello\n world");
                assert_eq!(input.selected_range, 6..6);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_enter_replaces_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 5..6);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.enter(&Enter, window, cx);
                assert_eq!(input.content(), "hello\nworld");
                assert_eq!(input.selected_range, 6..6);
            });
        })
        .unwrap();
    }

    // ============================================================
    // CLIPBOARD
    // ============================================================

    #[crate::test]
    fn test_copy_with_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 6..11);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.copy(&Copy, window, cx);
            });
        })
        .unwrap();

        let clipboard = cx.read_from_clipboard();
        assert!(clipboard.is_some());
        assert_eq!(clipboard.unwrap().text().as_deref(), Some("world"));
    }

    #[crate::test]
    fn test_cut_with_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 0..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.cut(&Cut, window, cx);
                assert_eq!(input.content(), " world");
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();

        let clipboard = cx.read_from_clipboard();
        assert_eq!(clipboard.unwrap().text().as_deref(), Some("hello"));
    }

    #[crate::test]
    fn test_paste_inserts_text(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 5..5);
        cx.write_to_clipboard(ClipboardItem::new_string(" there".to_string()));
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.paste(&Paste, window, cx);
                assert_eq!(input.content(), "hello there world");
                assert_eq!(input.selected_range, 11..11);
            });
        })
        .unwrap();
    }

    // ============================================================
    // UNICODE / GRAPHEME HANDLING
    // ============================================================

    #[crate::test]
    fn test_movement_with_multibyte_utf8(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "café", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 1..1);
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 2..2);
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 3..3);
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 5..5);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_movement_with_emoji(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "a👋b", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 1..1);
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 5..5);
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 6..6);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_selection_with_multibyte_characters(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "日本語", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_right(&SelectRight, window, cx);
                assert_eq!(input.selected_range, 0..3);
                input.select_right(&SelectRight, window, cx);
                assert_eq!(input.selected_range, 0..6);
                input.select_right(&SelectRight, window, cx);
                assert_eq!(input.selected_range, 0..9);
            });
        })
        .unwrap();
    }

    // ============================================================
    // NEWLINE HANDLING
    // ============================================================

    #[crate::test]
    fn test_find_line_start_and_end(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "first\nsecond\nthird", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert_eq!(input.find_line_start(0), 0);
                assert_eq!(input.find_line_start(3), 0);
                assert_eq!(input.find_line_start(6), 6);
                assert_eq!(input.find_line_start(13), 13);

                assert_eq!(input.find_line_end(0), 5);
                assert_eq!(input.find_line_end(6), 12);
                assert_eq!(input.find_line_end(13), 18);
            });
        })
        .unwrap();
    }

    // ============================================================
    // EDGE CASES
    // ============================================================

    #[crate::test]
    fn test_operations_on_empty_content(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range, 0..0);

                input.right(&Right, window, cx);
                assert_eq!(input.selected_range, 0..0);

                input.backspace(&Backspace, window, cx);
                assert_eq!(input.content(), "");

                input.delete(&Delete, window, cx);
                assert_eq!(input.content(), "");

                input.select_all(&SelectAll, window, cx);
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_set_content_resets_selection(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 3..8);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, cx| {
                input.selection_reversed = true;
                input.marked_range = Some(5..7);
                input.set_content("new content", cx);
                assert_eq!(input.content(), "new content");
                assert_eq!(input.selected_range, 0..0);
                assert!(!input.selection_reversed);
                assert_eq!(input.marked_range, None);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_cursor_clamped_to_content_length(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 100..100);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, cx| {
                input.move_to(1000, cx);
                assert_eq!(input.selected_range, 5..5);

                input.selected_range = 0..0;
                input.select_to(1000, cx);
                assert_eq!(input.selected_range, 0..5);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_previous_boundary_at_start(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert_eq!(input.previous_boundary(0), 0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_next_boundary_at_end(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert_eq!(input.next_boundary(5), 5);
                assert_eq!(input.next_boundary(100), 5);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_word_range_at_boundary(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "hello world", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                let (start, end) = input.word_range_at(5);
                assert_eq!(start, 0);
                assert_eq!(end, 5);

                let (start, end) = input.word_range_at(8);
                assert_eq!(start, 6);
                assert_eq!(end, 11);
            });
        })
        .unwrap();
    }

    // ============================================================
    // EMOJI & GRAPHEME CLUSTERS
    // ============================================================

    #[crate::test]
    fn test_simple_emoji_navigation(cx: &mut TestAppContext) {
        // 😀 is 4 bytes in UTF-8
        let view = create_test_input(cx, "a😀b", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                // Move right through: a -> 😀 -> b
                input.right(&Right, window, cx);
                assert_eq!(input.selected_range.start, 1); // after 'a'

                input.right(&Right, window, cx);
                assert_eq!(input.selected_range.start, 5); // after 😀 (1 + 4 bytes)

                input.right(&Right, window, cx);
                assert_eq!(input.selected_range.start, 6); // after 'b'

                // Move left back
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range.start, 5); // before 'b'

                input.left(&Left, window, cx);
                assert_eq!(input.selected_range.start, 1); // before 😀
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_emoji_with_skin_tone_modifier(cx: &mut TestAppContext) {
        // 👋🏽 = 👋 (U+1F44B, 4 bytes) + 🏽 (U+1F3FD, 4 bytes) = 8 bytes total
        let emoji = "👋🏽";
        assert_eq!(emoji.len(), 8);

        let view = create_test_input(cx, &format!("a{}b", emoji), 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'a'
                assert_eq!(input.selected_range.start, 1);

                input.right(&Right, window, cx); // past entire emoji with modifier
                assert_eq!(input.selected_range.start, 9); // 1 + 8

                input.left(&Left, window, cx); // back before emoji
                assert_eq!(input.selected_range.start, 1);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_zwj_family_emoji(cx: &mut TestAppContext) {
        // 👨‍👩‍👧 = man + ZWJ + woman + ZWJ + girl
        // Each person emoji is 4 bytes, ZWJ is 3 bytes
        // Total: 4 + 3 + 4 + 3 + 4 = 18 bytes
        let family = "👨‍👩‍👧";
        assert_eq!(family.len(), 18);

        let view = create_test_input(cx, &format!("x{}y", family), 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'x'
                assert_eq!(input.selected_range.start, 1);

                input.right(&Right, window, cx); // past entire ZWJ sequence
                assert_eq!(input.selected_range.start, 19); // 1 + 18

                input.right(&Right, window, cx); // past 'y'
                assert_eq!(input.selected_range.start, 20);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_backspace_deletes_emoji_between_ascii(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "a😀b", 5..5); // cursor after emoji
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.backspace(&Backspace, window, cx);
                assert_eq!(input.content(), "ab");
                assert_eq!(input.selected_range.start, 1);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_backspace_deletes_zwj_sequence(cx: &mut TestAppContext) {
        let family = "👨‍👩‍👧";
        let content = format!("a{}b", family);
        let cursor_pos = 1 + family.len(); // after the family emoji

        let view = create_test_input(cx, &content, cursor_pos..cursor_pos);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.backspace(&Backspace, window, cx);
                assert_eq!(input.content(), "ab");
                assert_eq!(input.selected_range.start, 1);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_delete_removes_entire_emoji(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "a😀b", 1..1); // cursor before emoji
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.delete(&Delete, window, cx);
                assert_eq!(input.content(), "ab");
                assert_eq!(input.selected_range.start, 1);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_flag_emoji_navigation(cx: &mut TestAppContext) {
        // 🇯🇵 = Regional Indicator J (4 bytes) + Regional Indicator P (4 bytes)
        let flag = "🇯🇵";
        assert_eq!(flag.len(), 8);

        let view = create_test_input(cx, &format!("x{}y", flag), 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'x'
                input.right(&Right, window, cx); // past flag (should be single grapheme)
                assert_eq!(input.selected_range.start, 9); // 1 + 8
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_combining_diacritical_marks(cx: &mut TestAppContext) {
        // é as e + combining acute accent (U+0301)
        let combining = "e\u{0301}"; // 1 + 2 = 3 bytes
        assert_eq!(combining.len(), 3);

        let view = create_test_input(cx, &format!("a{}b", combining), 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'a'
                assert_eq!(input.selected_range.start, 1);

                input.right(&Right, window, cx); // past e + combining mark (single grapheme)
                assert_eq!(input.selected_range.start, 4); // 1 + 3

                input.left(&Left, window, cx);
                assert_eq!(input.selected_range.start, 1);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_multiple_combining_marks(cx: &mut TestAppContext) {
        // ë́ = e + combining diaeresis (U+0308) + combining acute (U+0301)
        let multi_combining = "e\u{0308}\u{0301}"; // 1 + 2 + 2 = 5 bytes
        assert_eq!(multi_combining.len(), 5);

        let view = create_test_input(cx, &format!("x{}y", multi_combining), 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'x'
                input.right(&Right, window, cx); // past entire combined character
                assert_eq!(input.selected_range.start, 6); // 1 + 5
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_select_emoji_with_shift(cx: &mut TestAppContext) {
        let view = create_test_input(cx, "a😀b", 1..1); // cursor before emoji
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_right(&SelectRight, window, cx);
                assert_eq!(input.selected_range, 1..5); // selected the entire emoji
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_cjk_characters(cx: &mut TestAppContext) {
        // 你好 - each character is 3 bytes in UTF-8
        let view = create_test_input(cx, "a你好b", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'a'
                assert_eq!(input.selected_range.start, 1);

                input.right(&Right, window, cx); // past 你
                assert_eq!(input.selected_range.start, 4); // 1 + 3

                input.right(&Right, window, cx); // past 好
                assert_eq!(input.selected_range.start, 7); // 4 + 3

                input.right(&Right, window, cx); // past 'b'
                assert_eq!(input.selected_range.start, 8);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_mixed_script_text(cx: &mut TestAppContext) {
        // Mix of ASCII, CJK, and emoji
        let view = create_test_input(cx, "Hi你😀", 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'H'
                assert_eq!(input.selected_range.start, 1);

                input.right(&Right, window, cx); // past 'i'
                assert_eq!(input.selected_range.start, 2);

                input.right(&Right, window, cx); // past 你 (3 bytes)
                assert_eq!(input.selected_range.start, 5);

                input.right(&Right, window, cx); // past 😀 (4 bytes)
                assert_eq!(input.selected_range.start, 9);

                // Now go back
                input.left(&Left, window, cx);
                assert_eq!(input.selected_range.start, 5);

                input.left(&Left, window, cx);
                assert_eq!(input.selected_range.start, 2);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_variation_selector_emoji(cx: &mut TestAppContext) {
        // ☺️ = ☺ (U+263A, 3 bytes) + variation selector-16 (U+FE0F, 3 bytes)
        let emoji_presentation = "☺\u{FE0F}";
        assert_eq!(emoji_presentation.len(), 6);

        let view = create_test_input(cx, &format!("a{}b", emoji_presentation), 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'a'
                input.right(&Right, window, cx); // past emoji with variation selector
                assert_eq!(input.selected_range.start, 7); // 1 + 6
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_keycap_emoji(cx: &mut TestAppContext) {
        // 1️⃣ = 1 + variation selector + combining enclosing keycap
        let keycap = "1\u{FE0F}\u{20E3}";

        let view = create_test_input(cx, &format!("x{}y", keycap), 0..0);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.right(&Right, window, cx); // past 'x'
                input.right(&Right, window, cx); // past keycap sequence
                let expected_pos = 1 + keycap.len();
                assert_eq!(input.selected_range.start, expected_pos);
            });
        })
        .unwrap();
    }

    // Single-line input tests

    fn create_single_line_input(
        cx: &mut TestAppContext,
        content: &str,
        selected_range: Range<usize>,
    ) -> crate::WindowHandle<TestView> {
        cx.add_window(|_window, cx| {
            let input = cx.new(|cx| {
                let mut input = InputState::new_singleline(cx);
                input.content = content.to_string();
                input.selected_range = selected_range;
                input
            });
            TestView { input }
        })
    }

    #[crate::test]
    fn test_single_line_enter_does_nothing(cx: &mut TestAppContext) {
        let view = create_single_line_input(cx, "hello", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.enter(&Enter, window, cx);
                assert_eq!(input.content(), "hello");
                assert_eq!(input.selected_range, 5..5);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_single_line_set_content_strips_newlines(cx: &mut TestAppContext) {
        let view = create_single_line_input(cx, "", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, cx| {
                input.set_content("hello\nworld\r\nfoo", cx);
                assert_eq!(input.content(), "hello world foo");
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_single_line_up_moves_to_start(cx: &mut TestAppContext) {
        let view = create_single_line_input(cx, "hello world", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.up(&Up, window, cx);
                assert_eq!(input.selected_range, 0..0);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_single_line_down_moves_to_end(cx: &mut TestAppContext) {
        let view = create_single_line_input(cx, "hello world", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.down(&Down, window, cx);
                assert_eq!(input.selected_range, 11..11); // "hello world".len() == 11
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_single_line_select_up_selects_to_start(cx: &mut TestAppContext) {
        let view = create_single_line_input(cx, "hello world", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_up(&SelectUp, window, cx);
                assert_eq!(input.selected_range, 0..5);
                assert!(input.selection_reversed);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_single_line_select_down_selects_to_end(cx: &mut TestAppContext) {
        let view = create_single_line_input(cx, "hello world", 5..5);
        view.update(cx, |view, window, cx| {
            view.input.update(cx, |input, cx| {
                input.select_down(&SelectDown, window, cx);
                assert_eq!(input.selected_range, 5..11); // "hello world".len() == 11
                assert!(!input.selection_reversed);
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_single_line_multiline_getter(cx: &mut TestAppContext) {
        let view = create_single_line_input(cx, "hello", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(!input.is_multiline());
            });
        })
        .unwrap();

        let multiline_view = create_test_input(cx, "hello", 0..0);
        multiline_view
            .update(cx, |view, _window, cx| {
                view.input.update(cx, |input, _cx| {
                    assert!(input.is_multiline());
                });
            })
            .unwrap();
    }

    #[crate::test]
    fn test_line_direction_ltr(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "Hello world", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.line_direction(0).is_ltr());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_line_direction_rtl_arabic(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "مرحبا بالعالم", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.line_direction(0).is_rtl());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_line_direction_rtl_hebrew(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "שלום עולם", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.line_direction(0).is_rtl());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_line_direction_mixed_lines(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "Hello\nمرحبا\nWorld", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.line_direction(0).is_ltr());
                assert!(input.line_direction(1).is_rtl());
                assert!(input.line_direction(2).is_ltr());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_direction_at_offset_ltr(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "Hello world", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.direction_at_offset(0).is_ltr());
                assert!(input.direction_at_offset(5).is_ltr());
                assert!(input.direction_at_offset(11).is_ltr());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_direction_at_offset_rtl(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "مرحبا", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.direction_at_offset(0).is_rtl());
                assert!(input.direction_at_offset(5).is_rtl());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_direction_at_offset_multiline(cx: &mut TestAppContext) {
        let content = "Hello\nמילים";
        let view = create_test_input_with_layout(cx, content, 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.direction_at_offset(0).is_ltr());
                assert!(input.direction_at_offset(5).is_ltr());
                assert!(input.direction_at_offset(6).is_rtl());
                assert!(input.direction_at_offset(content.len()).is_rtl());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_empty_line_inherits_direction(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "مرحبا\n\nWorld", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.line_direction(0).is_rtl());
                assert!(input.line_direction(1).is_rtl());
                assert!(input.line_direction(2).is_ltr());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_numbers_before_rtl(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "123 مرحبا", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.line_direction(0).is_rtl());
            });
        })
        .unwrap();
    }

    #[crate::test]
    fn test_line_direction_out_of_bounds(cx: &mut TestAppContext) {
        let view = create_test_input_with_layout(cx, "Hello", 0..0);
        view.update(cx, |view, _window, cx| {
            view.input.update(cx, |input, _cx| {
                assert!(input.line_direction(100).is_ltr());
            });
        })
        .unwrap();
    }
}
