// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::cmp;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    render_decoration, Action, Attrs, AttrsList, BorrowedWithFontSystem, BufferLine, BufferRef,
    Change, ChangeItem, Color, Cursor, Edit, FontSystem, LayoutRun, LineEnding, LineIter, Renderer,
    Selection, Shaping,
};

/// A wrapper of [`Buffer`] for easy editing
#[derive(Debug, Clone)]
pub struct Editor<'buffer> {
    buffer_ref: BufferRef<'buffer>,
    cursor: Cursor,
    cursor_x_opt: Option<i32>,
    selection: Selection,
    cursor_moved: bool,
    auto_indent: bool,
    change: Option<Change>,
}

fn cursor_position(cursor: &Cursor, run: &LayoutRun) -> Option<(i32, i32)> {
    let x = run.cursor_position(cursor)?;
    Some((x as i32, run.line_top as i32))
}

impl<'buffer> Editor<'buffer> {
    /// Create a new [`Editor`] with the provided [`Buffer`]
    pub fn new(buffer: impl Into<BufferRef<'buffer>>) -> Self {
        Self {
            buffer_ref: buffer.into(),
            cursor: Cursor::default(),
            cursor_x_opt: None,
            selection: Selection::None,
            cursor_moved: false,
            auto_indent: false,
            change: None,
        }
    }

    /// Draw the editor
    ///
    /// Automatically resolves any pending dirty state before drawing.
    #[cfg(feature = "swash")]
    #[allow(clippy::too_many_arguments)]
    pub fn draw<F>(
        &mut self,
        font_system: &mut FontSystem,
        cache: &mut crate::SwashCache,
        text_color: Color,
        cursor_color: Color,
        selection_color: Color,
        selected_text_color: Color,
        callback: F,
    ) where
        F: FnMut(i32, i32, u32, u32, Color),
    {
        self.with_buffer_mut(|buffer| buffer.shape_until_scroll(font_system, false));
        let mut renderer = crate::LegacyRenderer {
            font_system,
            cache,
            callback,
        };
        self.render(
            &mut renderer,
            text_color,
            cursor_color,
            selection_color,
            selected_text_color,
        );
    }

    /// Render the editor using the provided renderer.
    ///
    /// The caller is responsible for calling [`Edit::shape_as_needed`] first
    /// to ensure layout is up to date.
    pub fn render<R: Renderer>(
        &self,
        renderer: &mut R,
        text_color: Color,
        cursor_color: Color,
        selection_color: Color,
        selected_text_color: Color,
    ) {
        let selection_bounds = self.selection_bounds();
        self.with_buffer(|buffer| {
            for run in buffer.layout_runs() {
                let line_i = run.line_i;
                let line_y = run.line_y;
                let line_top = run.line_top;
                let line_height = run.line_height;

                // Highlight selection
                if let Some((start, end)) = selection_bounds {
                    if line_i >= start.line && line_i <= end.line {
                        let highlights: Vec<(f32, f32)> = run.highlight(start, end).collect();

                        if highlights.is_empty() && run.glyphs.is_empty() && end.line > line_i {
                            // Highlight all of internal empty lines
                            let max = buffer.size().0.unwrap_or(0.0) as i32;
                            renderer.rectangle(
                                0,
                                line_top as i32,
                                max as u32,
                                line_height as u32,
                                selection_color,
                            );
                        } else {
                            let len = highlights.len();
                            for (idx, (x, width)) in highlights.into_iter().enumerate() {
                                let mut min = x as i32;
                                let mut max = (x + width) as i32;

                                // Extend the last rect to the line edge for
                                // multi-line selections
                                if idx == len - 1 && end.line > line_i {
                                    if run.rtl {
                                        min = 0;
                                    } else {
                                        max = buffer.size().0.unwrap_or(0.0) as i32;
                                    }
                                }

                                renderer.rectangle(
                                    min,
                                    line_top as i32,
                                    cmp::max(0, max - min) as u32,
                                    line_height as u32,
                                    selection_color,
                                );
                            }
                        }
                    }
                }

                // Draw cursor
                if let Some((x, y)) = cursor_position(&self.cursor, &run) {
                    renderer.rectangle(x, y, 1, line_height as u32, cursor_color);
                }

                render_decoration(renderer, &run, text_color);
                for glyph in run.glyphs {
                    let physical_glyph = glyph.physical((0., line_y), 1.0);

                    let mut glyph_color = glyph.color_opt.map_or(text_color, |some| some);
                    if text_color != selected_text_color {
                        if let Some((start, end)) = selection_bounds {
                            if line_i >= start.line
                                && line_i <= end.line
                                && (start.line != line_i || glyph.end > start.index)
                                && (end.line != line_i || glyph.start < end.index)
                            {
                                glyph_color = selected_text_color;
                            }
                        }
                    }

                    renderer.glyph(physical_glyph, glyph_color);
                }
            }
        });
    }
}

impl<'buffer> Edit<'buffer> for Editor<'buffer> {
    fn buffer_ref(&self) -> &BufferRef<'buffer> {
        &self.buffer_ref
    }

    fn buffer_ref_mut(&mut self) -> &mut BufferRef<'buffer> {
        &mut self.buffer_ref
    }

    fn cursor(&self) -> Cursor {
        self.cursor
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        if self.cursor != cursor {
            self.cursor = cursor;
            self.cursor_moved = true;
            self.with_buffer_mut(|buffer| buffer.set_redraw(true));
        }
    }

    fn selection(&self) -> Selection {
        self.selection
    }

    fn set_selection(&mut self, selection: Selection) {
        if self.selection != selection {
            self.selection = selection;
            self.with_buffer_mut(|buffer| buffer.set_redraw(true));
        }
    }

    fn auto_indent(&self) -> bool {
        self.auto_indent
    }

    fn set_auto_indent(&mut self, auto_indent: bool) {
        self.auto_indent = auto_indent;
    }

    fn tab_width(&self) -> u16 {
        self.with_buffer(super::super::buffer::Buffer::tab_width)
    }

    fn set_tab_width(&mut self, tab_width: u16) {
        self.with_buffer_mut(|buffer| buffer.set_tab_width(tab_width));
    }

    fn shape_as_needed(&mut self, font_system: &mut FontSystem, prune: bool) {
        if self.cursor_moved {
            let cursor = self.cursor;
            self.with_buffer_mut(|buffer| buffer.shape_until_cursor(font_system, cursor, prune));
            self.cursor_moved = false;
        } else {
            self.with_buffer_mut(|buffer| buffer.shape_until_scroll(font_system, prune));
        }
    }

    fn delete_range(&mut self, start: Cursor, end: Cursor) {
        let change_item = self.with_buffer_mut(|buffer| {
            // Collect removed data for change tracking
            let mut change_lines = Vec::new();

            // Delete the selection from the last line
            let end_line_opt = if end.line > start.line {
                // Get part of line after selection
                let after = buffer.lines[end.line].split_off(end.index);

                // Remove end line
                let removed = buffer.lines.remove(end.line);
                change_lines.insert(0, removed);

                Some(after)
            } else {
                None
            };

            // Delete interior lines (in reverse for safety)
            for line_i in (start.line + 1..end.line).rev() {
                let removed = buffer.lines.remove(line_i);
                change_lines.insert(0, removed);
            }

            // Delete the selection from the first line
            {
                let line = &mut buffer.lines[start.line];

                // Get part after selection if start line is also end line
                let after_opt = if start.line == end.line {
                    Some(line.split_off(end.index))
                } else {
                    None
                };

                // Delete selected part of line
                let removed = line.split_off(start.index);
                change_lines.insert(0, removed);

                // Re-add part of line after selection
                if let Some(after) = after_opt {
                    line.append(&after);
                }

                // Re-add valid parts of end line
                if let Some(mut end_line) = end_line_opt {
                    // Preserve line ending of original line
                    if end_line.ending() == LineEnding::None {
                        end_line.set_ending(line.ending());
                    }
                    line.append(&end_line);
                }
            }

            let mut text = String::new();
            let mut last_ending: Option<LineEnding> = None;
            for line in change_lines {
                if let Some(ending) = last_ending {
                    text.push_str(ending.as_str());
                }
                text.push_str(line.text());
                last_ending = Some(line.ending());
            }

            ChangeItem {
                start,
                end,
                text,
                insert: false,
            }
        });

        if let Some(ref mut change) = self.change {
            change.items.push(change_item);
        }
    }

    fn insert_at(
        &mut self,
        mut cursor: Cursor,
        data: &str,
        attrs_list: Option<AttrsList>,
    ) -> Cursor {
        let mut remaining_split_len = data.len();
        if remaining_split_len == 0 {
            return cursor;
        }

        let change_item = self.with_buffer_mut(|buffer| {
            // Save cursor for change tracking
            let start = cursor;

            // Ensure there are enough lines in the buffer to handle this cursor
            while cursor.line >= buffer.lines.len() {
                // Get last line ending
                let mut last_ending = LineEnding::None;
                if let Some(last_line) = buffer.lines.last_mut() {
                    last_ending = last_line.ending();
                    // Ensure a valid line ending is always set on interior lines
                    if last_ending == LineEnding::None {
                        last_line.set_ending(LineEnding::default());
                    }
                }
                let line = BufferLine::new(
                    String::new(),
                    last_ending,
                    AttrsList::new(&attrs_list.as_ref().map_or_else(
                        || {
                            buffer
                                .lines
                                .last()
                                .map_or_else(Attrs::new, |line| line.attrs_list().defaults())
                        },
                        |x| x.defaults(),
                    )),
                    Shaping::Advanced,
                );
                buffer.lines.push(line);
            }

            let line: &mut BufferLine = &mut buffer.lines[cursor.line];
            let insert_line = cursor.line + 1;

            // Collect text after insertion as a line
            let after: BufferLine = line.split_off(cursor.index);
            let after_len = after.text().len();

            // Collect attributes
            let mut final_attrs = attrs_list.unwrap_or_else(|| {
                AttrsList::new(&line.attrs_list().get_span(cursor.index.saturating_sub(1)))
            });

            // Append the inserted text, line by line
            let mut lines: Vec<_> = LineIter::new(data).collect();
            // Ensure there is always an ending line with no line ending
            if lines.last().map(|line| line.1).unwrap_or(LineEnding::None) != LineEnding::None {
                lines.push((Default::default(), LineEnding::None));
            }
            let mut lines_iter = lines.into_iter();

            // Add first line
            if let Some((range, ending)) = lines_iter.next() {
                let data_line = &data[range];
                let mut these_attrs = final_attrs.split_off(data_line.len());
                remaining_split_len -= data_line.len() + ending.as_str().len();
                core::mem::swap(&mut these_attrs, &mut final_attrs);
                line.append(&BufferLine::new(
                    data_line,
                    ending,
                    these_attrs,
                    Shaping::Advanced,
                ));
            }
            // Add last line
            if let Some((range, ending)) = lines_iter.next_back() {
                let data_line = &data[range];
                remaining_split_len -= data_line.len() + ending.as_str().len();
                let mut tmp = BufferLine::new(
                    data_line,
                    ending,
                    final_attrs.split_off(remaining_split_len),
                    Shaping::Advanced,
                );
                tmp.append(&after);
                buffer.lines.insert(insert_line, tmp);
                cursor.line += 1;
            } else {
                line.append(&after);
            }
            // Add middle lines
            for (range, ending) in lines_iter.rev() {
                let data_line = &data[range];
                remaining_split_len -= data_line.len() + ending.as_str().len();
                let tmp = BufferLine::new(
                    data_line,
                    ending,
                    final_attrs.split_off(remaining_split_len),
                    Shaping::Advanced,
                );
                buffer.lines.insert(insert_line, tmp);
                cursor.line += 1;
            }

            assert_eq!(remaining_split_len, 0);

            // Append the text after insertion
            cursor.index = buffer.lines[cursor.line].text().len() - after_len;

            ChangeItem {
                start,
                end: cursor,
                text: data.to_string(),
                insert: true,
            }
        });

        if let Some(ref mut change) = self.change {
            change.items.push(change_item);
        }

        cursor
    }

    fn copy_selection(&self) -> Option<String> {
        let (start, end) = self.selection_bounds()?;
        self.with_buffer(|buffer| {
            let mut selection = String::new();
            // Take the selection from the first line
            {
                // Add selected part of line to string
                if start.line == end.line {
                    selection.push_str(&buffer.lines[start.line].text()[start.index..end.index]);
                } else {
                    selection.push_str(&buffer.lines[start.line].text()[start.index..]);
                    selection.push('\n');
                }
            }

            // Take the selection from all interior lines (if they exist)
            for line_i in start.line + 1..end.line {
                selection.push_str(buffer.lines[line_i].text());
                selection.push('\n');
            }

            // Take the selection from the last line
            if end.line > start.line {
                // Add selected part of line to string
                selection.push_str(&buffer.lines[end.line].text()[..end.index]);
            }

            Some(selection)
        })
    }

    fn delete_selection(&mut self) -> bool {
        let Some((start, end)) = self.selection_bounds() else {
            return false;
        };

        // Reset cursor to start of selection
        self.cursor = start;

        // Reset selection to None
        self.selection = Selection::None;

        // Delete from start to end of selection
        self.delete_range(start, end);

        true
    }

    fn apply_change(&mut self, change: &Change) -> bool {
        // Cannot apply changes if there is a pending change
        if let Some(pending) = self.change.take() {
            if !pending.items.is_empty() {
                //TODO: is this a good idea?
                log::warn!("pending change caused apply_change to be ignored!");
                self.change = Some(pending);
                return false;
            }
        }

        for item in &change.items {
            //TODO: edit cursor if needed?
            if item.insert {
                self.cursor = self.insert_at(item.start, &item.text, None);
            } else {
                self.cursor = item.start;
                self.delete_range(item.start, item.end);
            }
        }
        true
    }

    fn start_change(&mut self) {
        if self.change.is_none() {
            self.change = Some(Change::default());
        }
    }

    fn finish_change(&mut self) -> Option<Change> {
        self.change.take()
    }

    fn action(&mut self, font_system: &mut FontSystem, action: Action) {
        let old_cursor = self.cursor;

        match action {
            Action::Motion(motion) => {
                let cursor = self.cursor;
                let cursor_x_opt = self.cursor_x_opt;
                if let Some((new_cursor, new_cursor_x_opt)) = self.with_buffer_mut(|buffer| {
                    buffer.cursor_motion(font_system, cursor, cursor_x_opt, motion)
                }) {
                    self.cursor = new_cursor;
                    self.cursor_x_opt = new_cursor_x_opt;
                }
            }
            Action::Escape => {
                match self.selection {
                    Selection::None => {}
                    _ => self.with_buffer_mut(|buffer| buffer.set_redraw(true)),
                }
                self.selection = Selection::None;
            }
            Action::Insert(character) => {
                if character.is_control() && !['\t', '\n', '\u{92}'].contains(&character) {
                    // Filter out special chars (except for tab), use Action instead
                    log::debug!("Refusing to insert control character {character:?}");
                } else if character == '\n' {
                    self.action(font_system, Action::Enter);
                } else {
                    let mut str_buf = [0u8; 8];
                    let str_ref = character.encode_utf8(&mut str_buf);
                    self.insert_string(str_ref, None);
                }
            }
            Action::Enter => {
                //TODO: what about indenting more after opening brackets or parentheses?
                if self.auto_indent {
                    let mut string = String::from("\n");
                    self.with_buffer(|buffer| {
                        let line = &buffer.lines[self.cursor.line];
                        let text = line.text();
                        for c in text.chars() {
                            if c.is_whitespace() {
                                string.push(c);
                            } else {
                                break;
                            }
                        }
                    });
                    self.insert_string(&string, None);
                } else {
                    self.insert_string("\n", None);
                }

                // Ensure line is properly shaped and laid out (for potential immediate commands)
                let line_i = self.cursor.line;
                self.with_buffer_mut(|buffer| {
                    buffer.line_layout(font_system, line_i);
                });
            }
            Action::Backspace => {
                if self.delete_selection() {
                    // Deleted selection
                } else {
                    // Save current cursor as end
                    let end = self.cursor;

                    if self.cursor.index > 0 {
                        // Move cursor to previous character index
                        self.cursor.index = self.with_buffer(|buffer| {
                            buffer.lines[self.cursor.line].text()[..self.cursor.index]
                                .char_indices()
                                .next_back()
                                .map_or(0, |(i, _)| i)
                        });
                    } else if self.cursor.line > 0 {
                        // Move cursor to previous line
                        self.cursor.line -= 1;
                        self.cursor.index =
                            self.with_buffer(|buffer| buffer.lines[self.cursor.line].text().len());
                    }

                    if self.cursor != end {
                        // Delete range
                        self.delete_range(self.cursor, end);
                    }
                }
            }
            Action::Delete => {
                if self.delete_selection() {
                    // Deleted selection
                } else {
                    // Save current cursor as start and end
                    let mut start = self.cursor;
                    let mut end = self.cursor;

                    self.with_buffer(|buffer| {
                        if start.index < buffer.lines[start.line].text().len() {
                            let line = &buffer.lines[start.line];

                            let range_opt = line
                                .text()
                                .grapheme_indices(true)
                                .take_while(|(i, _)| *i <= start.index)
                                .last()
                                .map(|(i, c)| i..(i + c.len()));

                            if let Some(range) = range_opt {
                                start.index = range.start;
                                end.index = range.end;
                            }
                        } else if start.line + 1 < buffer.lines.len() {
                            end.line += 1;
                            end.index = 0;
                        }
                    });

                    if start != end {
                        self.cursor = start;
                        self.delete_range(start, end);
                    }
                }
            }
            Action::Indent => {
                // Get start and end of selection
                let (start, end) = match self.selection_bounds() {
                    Some(some) => some,
                    None => (self.cursor, self.cursor),
                };

                // For every line in selection
                let tab_width: usize = self.tab_width().into();
                for line_i in start.line..=end.line {
                    // Determine indexes of last indent and first character after whitespace
                    let mut after_whitespace = 0;
                    let mut required_indent = 0;
                    self.with_buffer(|buffer| {
                        let line = &buffer.lines[line_i];
                        let text = line.text();

                        if self.selection == Selection::None {
                            //Selection::None counts whitespace from the cursor backwards
                            let whitespace_length = match line.text()[0..self.cursor.index]
                                .chars()
                                .rev()
                                .position(|c| !c.is_whitespace())
                            {
                                Some(length) => length,
                                // The whole line is whitespace
                                None => self.cursor.index,
                            };
                            required_indent = tab_width - (whitespace_length % tab_width);
                            after_whitespace = self.cursor.index;
                        } else {
                            // Other selections count whitespace from  the start of the line
                            for (count, (index, c)) in text.char_indices().enumerate() {
                                if !c.is_whitespace() {
                                    after_whitespace = index;
                                    required_indent = tab_width - (count % tab_width);
                                    break;
                                }
                            }
                        }
                    });

                    self.insert_at(
                        Cursor::new(line_i, after_whitespace),
                        &" ".repeat(required_indent),
                        None,
                    );

                    // Adjust cursor
                    if self.cursor.line == line_i {
                        //TODO: should we be forcing cursor index to current indent location?
                        if self.cursor.index < after_whitespace {
                            self.cursor.index = after_whitespace;
                        }
                        self.cursor.index += required_indent;
                    }

                    // Adjust selection
                    match self.selection {
                        Selection::None => {}
                        Selection::Normal(ref mut select)
                        | Selection::Line(ref mut select)
                        | Selection::Word(ref mut select) => {
                            if select.line == line_i && select.index >= after_whitespace {
                                select.index += required_indent;
                            }
                        }
                    }
                    // Request redraw
                    self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                }
            }
            Action::Unindent => {
                // Get start and end of selection
                let (start, end) = match self.selection_bounds() {
                    Some(some) => some,
                    None => (self.cursor, self.cursor),
                };

                // For every line in selection
                let tab_width: usize = self.tab_width().into();
                for line_i in start.line..=end.line {
                    // Determine indexes of last indent and first character after whitespace
                    let mut last_indent = 0;
                    let mut after_whitespace = 0;
                    self.with_buffer(|buffer| {
                        let line = &buffer.lines[line_i];
                        let text = line.text();
                        // Default to end of line if no non-whitespace found
                        after_whitespace = text.len();
                        for (count, (index, c)) in text.char_indices().enumerate() {
                            if !c.is_whitespace() {
                                after_whitespace = index;
                                break;
                            }
                            if count % tab_width == 0 {
                                last_indent = index;
                            }
                        }
                    });

                    // No de-indent required
                    if last_indent == after_whitespace {
                        continue;
                    }

                    // Delete one indent
                    self.delete_range(
                        Cursor::new(line_i, last_indent),
                        Cursor::new(line_i, after_whitespace),
                    );

                    // Adjust cursor
                    if self.cursor.line == line_i && self.cursor.index > last_indent {
                        self.cursor.index -= after_whitespace - last_indent;
                    }

                    // Adjust selection
                    match self.selection {
                        Selection::None => {}
                        Selection::Normal(ref mut select)
                        | Selection::Line(ref mut select)
                        | Selection::Word(ref mut select) => {
                            if select.line == line_i && select.index > last_indent {
                                select.index -= after_whitespace - last_indent;
                            }
                        }
                    }

                    // Request redraw
                    self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                }
            }
            Action::Click { x, y } => {
                self.set_selection(Selection::None);

                if let Some(new_cursor) = self.with_buffer_mut(|buffer| {
                    buffer.shape_until_scroll(font_system, false);
                    buffer.hit(x as f32, y as f32)
                }) {
                    if new_cursor != self.cursor {
                        self.cursor = new_cursor;
                        self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                    }
                }
            }
            Action::DoubleClick { x, y } => {
                self.set_selection(Selection::None);

                if let Some(new_cursor) = self.with_buffer_mut(|buffer| {
                    buffer.shape_until_scroll(font_system, false);
                    buffer.hit(x as f32, y as f32)
                }) {
                    if new_cursor != self.cursor {
                        self.cursor = new_cursor;
                        self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                    }
                    self.selection = Selection::Word(self.cursor);
                    self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                }
            }
            Action::TripleClick { x, y } => {
                self.set_selection(Selection::None);

                if let Some(new_cursor) = self.with_buffer_mut(|buffer| {
                    buffer.shape_until_scroll(font_system, false);
                    buffer.hit(x as f32, y as f32)
                }) {
                    if new_cursor != self.cursor {
                        self.cursor = new_cursor;
                    }
                    self.selection = Selection::Line(self.cursor);
                    self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                }
            }
            Action::Drag { x, y } => {
                if self.selection == Selection::None {
                    self.selection = Selection::Normal(self.cursor);
                    self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                }

                if let Some(new_cursor) = self.with_buffer_mut(|buffer| {
                    buffer.shape_until_scroll(font_system, false);
                    buffer.hit(x as f32, y as f32)
                }) {
                    if new_cursor != self.cursor {
                        self.cursor = new_cursor;
                        self.with_buffer_mut(|buffer| buffer.set_redraw(true));
                    }
                }
            }
            Action::Scroll { pixels } => {
                self.with_buffer_mut(|buffer| {
                    let mut scroll = buffer.scroll();
                    //TODO: align to layout lines
                    scroll.vertical += pixels;
                    buffer.set_scroll(scroll);
                });
            }
        }

        if old_cursor != self.cursor {
            self.cursor_moved = true;
            self.with_buffer_mut(|buffer| buffer.set_redraw(true));

            /*TODO
            if let Some(glyph) = run.glyphs.get(new_cursor_glyph) {
                let font_opt = self.buffer.font_system().get_font(glyph.cache_key.font_id);
                let text_glyph = &run.text[glyph.start..glyph.end];
                log::debug!(
                    "{}, {}: '{}' ('{}'): '{}' ({:?})",
                    self.cursor.line,
                    self.cursor.index,
                    font_opt.as_ref().map_or("?", |font| font.info.family.as_str()),
                    font_opt.as_ref().map_or("?", |font| font.info.post_script_name.as_str()),
                    text_glyph,
                    text_glyph
                );
            }
            */
        }
    }

    fn cursor_position(&self) -> Option<(i32, i32)> {
        self.with_buffer(|buffer| {
            buffer
                .layout_runs()
                .find_map(|run| cursor_position(&self.cursor, &run))
        })
    }
}

impl BorrowedWithFontSystem<'_, Editor<'_>> {
    #[cfg(feature = "swash")]
    pub fn draw<F>(
        &mut self,
        cache: &mut crate::SwashCache,
        text_color: Color,
        cursor_color: Color,
        selection_color: Color,
        selected_text_color: Color,
        f: F,
    ) where
        F: FnMut(i32, i32, u32, u32, Color),
    {
        self.inner.draw(
            self.font_system,
            cache,
            text_color,
            cursor_color,
            selection_color,
            selected_text_color,
            f,
        );
    }
}
