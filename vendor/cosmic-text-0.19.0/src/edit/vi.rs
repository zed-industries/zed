use alloc::{collections::BTreeMap, string::String};
use core::cmp;
use modit::{Event, Key, Parser, TextObject, WordIter};

use crate::{
    Action, AttrsList, BorrowedWithFontSystem, BufferRef, Change, Color, Cursor, Edit, FontSystem,
    Motion, Renderer, Selection, SyntaxEditor, SyntaxTheme,
};

pub use modit::{ViMode, ViParser};
use unicode_segmentation::UnicodeSegmentation;

fn undo_2_action<'buffer, E: Edit<'buffer>>(
    editor: &mut E,
    action: cosmic_undo_2::Action<&Change>,
) {
    match action {
        cosmic_undo_2::Action::Do(change) => {
            editor.apply_change(change);
        }
        cosmic_undo_2::Action::Undo(change) => {
            //TODO: make this more efficient
            let mut reversed = change.clone();
            reversed.reverse();
            editor.apply_change(&reversed);
        }
    }
}

fn finish_change<'buffer, E: Edit<'buffer>>(
    editor: &mut E,
    commands: &mut cosmic_undo_2::Commands<Change>,
    changed: &mut bool,
    pivot: Option<usize>,
) -> Option<Change> {
    //TODO: join changes together
    match editor.finish_change() {
        Some(change) => {
            if !change.items.is_empty() {
                commands.push(change.clone());
                *changed = eval_changed(commands, pivot);
            }
            Some(change)
        }
        None => None,
    }
}

/// Evaluate if an [`ViEditor`] changed based on its last saved state.
fn eval_changed(commands: &cosmic_undo_2::Commands<Change>, pivot: Option<usize>) -> bool {
    // Editors are considered modified if the current change index is unequal to the last
    // saved index or if `pivot` is None.
    // The latter case handles a never saved editor with a current command index of None.
    // Check the unit tests for an example.
    match (commands.current_command_index(), pivot) {
        (Some(current), Some(pivot)) => current != pivot,
        // Edge case for an editor with neither a save point nor any changes.
        // This could be a new editor or an editor without a save point where undo() is called
        // until the editor is fresh.
        (None, None) => false,
        // Default to true because it's safer to assume a buffer has been modified so as to not
        // lose changes
        _ => true,
    }
}

fn search<'buffer, E: Edit<'buffer>>(editor: &mut E, value: &str, forwards: bool) -> bool {
    let mut cursor = editor.cursor();
    let start_line = cursor.line;
    if forwards {
        while cursor.line < editor.with_buffer(|buffer| buffer.lines.len()) {
            if let Some(index) = editor.with_buffer(|buffer| {
                buffer.lines[cursor.line]
                    .text()
                    .match_indices(value)
                    .filter_map(|(i, _)| {
                        if cursor.line != start_line || i > cursor.index {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .next()
            }) {
                cursor.index = index;
                editor.set_cursor(cursor);
                return true;
            }

            cursor.line += 1;
        }
    } else {
        cursor.line += 1;
        while cursor.line > 0 {
            cursor.line -= 1;

            if let Some(index) = editor.with_buffer(|buffer| {
                buffer.lines[cursor.line]
                    .text()
                    .rmatch_indices(value)
                    .filter_map(|(i, _)| {
                        if cursor.line != start_line || i < cursor.index {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .next()
            }) {
                cursor.index = index;
                editor.set_cursor(cursor);
                return true;
            }
        }
    }
    false
}

fn select_in<'buffer, E: Edit<'buffer>>(editor: &mut E, start_c: char, end_c: char, include: bool) {
    // Find the largest encompasing object, or if there is none, find the next one.
    let cursor = editor.cursor();
    let (start, end) = editor.with_buffer(|buffer| {
        // Search forwards for isolated end character, counting start and end characters found
        let mut end = cursor;
        let mut starts = 0;
        let mut ends = 0;
        'find_end: loop {
            let line = &buffer.lines[end.line];
            let text = line.text();
            for (i, c) in text[end.index..].char_indices() {
                if c == end_c {
                    ends += 1;
                } else if c == start_c {
                    starts += 1;
                }
                if ends > starts {
                    end.index += if include { i + c.len_utf8() } else { i };
                    break 'find_end;
                }
            }
            if end.line + 1 < buffer.lines.len() {
                end.line += 1;
                end.index = 0;
            } else {
                break 'find_end;
            }
        }

        // Search backwards to resolve starts and ends
        let mut start = cursor;
        'find_start: loop {
            let line = &buffer.lines[start.line];
            let text = line.text();
            for (i, c) in text[..start.index].char_indices().rev() {
                if c == start_c {
                    starts += 1;
                } else if c == end_c {
                    ends += 1;
                }
                if starts >= ends {
                    start.index = if include { i } else { i + c.len_utf8() };
                    break 'find_start;
                }
            }
            if start.line > 0 {
                start.line -= 1;
                start.index = buffer.lines[start.line].text().len();
            } else {
                break 'find_start;
            }
        }

        (start, end)
    });

    editor.set_selection(Selection::Normal(start));
    editor.set_cursor(end);
}

#[derive(Debug)]
pub struct ViEditor<'syntax_system, 'buffer> {
    editor: SyntaxEditor<'syntax_system, 'buffer>,
    parser: ViParser,
    passthrough: bool,
    registers: BTreeMap<char, (Selection, String)>,
    search_opt: Option<(String, bool)>,
    commands: cosmic_undo_2::Commands<Change>,
    changed: bool,
    save_pivot: Option<usize>,
}

impl<'syntax_system, 'buffer> ViEditor<'syntax_system, 'buffer> {
    pub fn new(editor: SyntaxEditor<'syntax_system, 'buffer>) -> Self {
        Self {
            editor,
            parser: ViParser::new(),
            passthrough: false,
            registers: BTreeMap::new(),
            search_opt: None,
            commands: cosmic_undo_2::Commands::new(),
            changed: false,
            save_pivot: None,
        }
    }

    /// Modifies the theme of the [`SyntaxEditor`], returning false if the theme is missing
    pub fn update_theme(&mut self, theme_name: &str) -> bool {
        self.editor.update_theme(theme_name)
    }

    /// Load text from a file, and also set syntax to the best option
    ///
    /// ## Errors
    /// Returns an `io::Error` if reading the file fails
    #[cfg(feature = "std")]
    pub fn load_text<P: AsRef<std::path::Path>>(
        &mut self,
        font_system: &mut FontSystem,
        path: P,
        attrs: crate::Attrs,
    ) -> std::io::Result<()> {
        self.editor.load_text(font_system, path, attrs)
    }

    /// Get the default background color
    pub fn background_color(&self) -> Color {
        self.editor.background_color()
    }

    /// Get the default foreground (text) color
    pub fn foreground_color(&self) -> Color {
        self.editor.foreground_color()
    }

    /// Get the default cursor color
    pub fn cursor_color(&self) -> Color {
        self.editor.cursor_color()
    }

    /// Get the default selection color
    pub fn selection_color(&self) -> Color {
        self.editor.selection_color()
    }

    /// Get the current syntect theme
    pub fn theme(&self) -> &SyntaxTheme {
        self.editor.theme()
    }

    /// Get changed flag
    pub fn changed(&self) -> bool {
        self.changed
    }

    /// Set changed flag
    pub fn set_changed(&mut self, changed: bool) {
        self.changed = changed;
    }

    /// Set current change as the save (or pivot) point.
    ///
    /// A pivot point is the last saved index. Anything before or after the pivot indicates that
    /// the editor has been changed or is unsaved.
    ///
    /// Undoing changes down to the pivot point sets the editor as unchanged.
    /// Redoing changes up to the pivot point sets the editor as unchanged.
    ///
    /// Undoing or redoing changes beyond the pivot point sets the editor to changed.
    pub fn save_point(&mut self) {
        self.save_pivot = Some(self.commands.current_command_index().unwrap_or_default());
        self.changed = false;
    }

    /// Set passthrough mode (true will turn off vi features)
    pub fn set_passthrough(&mut self, passthrough: bool) {
        if passthrough != self.passthrough {
            self.passthrough = passthrough;
            self.with_buffer_mut(|buffer| buffer.set_redraw(true));
        }
    }

    /// Get current vi parser
    pub fn parser(&self) -> &ViParser {
        &self.parser
    }

    /// Redo a change
    pub fn redo(&mut self) {
        log::debug!("Redo");
        for action in self.commands.redo() {
            undo_2_action(&mut self.editor, action);
        }
        self.changed = eval_changed(&self.commands, self.save_pivot);
    }

    /// Undo a change
    pub fn undo(&mut self) {
        log::debug!("Undo");
        for action in self.commands.undo() {
            undo_2_action(&mut self.editor, action);
        }
        self.changed = eval_changed(&self.commands, self.save_pivot);
    }

    /// Draw the editor.
    ///
    /// Automatically resolves any pending dirty state before drawing.
    #[cfg(feature = "swash")]
    pub fn draw<F>(
        &mut self,
        font_system: &mut FontSystem,
        cache: &mut crate::SwashCache,
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
        self.render(&mut renderer);
    }

    /// Render the editor using the provided renderer.
    ///
    /// The caller is responsible for calling [`Edit::shape_as_needed`] first
    /// to ensure layout is up to date.
    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        let background_color = self.background_color();
        let foreground_color = self.foreground_color();
        let cursor_color = self.cursor_color();
        let selection_color = self.selection_color();
        self.with_buffer(|buffer| {
            let size = buffer.size();
            if let Some(width) = size.0 {
                if let Some(height) = size.1 {
                    renderer.rectangle(0, 0, width as u32, height as u32, background_color);
                }
            }
            let font_size = buffer.metrics().font_size;
            for run in buffer.layout_runs() {
                let line_i = run.line_i;
                let line_y = run.line_y;
                let line_top = run.line_top;
                let line_height = run.line_height;

                let cursor_glyph_opt = |cursor: &Cursor| -> Option<(usize, f32, f32)> {
                    //TODO: better calculation of width
                    let default_width = font_size / 2.0;
                    if cursor.line == line_i {
                        for (glyph_i, glyph) in run.glyphs.iter().enumerate() {
                            if cursor.index >= glyph.start && cursor.index < glyph.end {
                                // Guess x offset based on characters
                                let mut before = 0;
                                let mut total = 0;

                                let cluster = &run.text[glyph.start..glyph.end];
                                for (i, _) in cluster.grapheme_indices(true) {
                                    if glyph.start + i < cursor.index {
                                        before += 1;
                                    }
                                    total += 1;
                                }

                                let width = glyph.w / (total as f32);
                                let offset = (before as f32) * width;
                                return Some((glyph_i, offset, width));
                            }
                        }
                        match run.glyphs.last() {
                            Some(glyph) => {
                                if cursor.index == glyph.end {
                                    return Some((run.glyphs.len(), 0.0, default_width));
                                }
                            }
                            None => {
                                return Some((0, 0.0, default_width));
                            }
                        }
                    }
                    None
                };

                // Highlight selection
                if let Some((start, end)) = self.selection_bounds() {
                    if line_i >= start.line && line_i <= end.line {
                        let mut range_opt = None;
                        for glyph in run.glyphs.iter() {
                            // Guess x offset based on characters
                            let cluster = &run.text[glyph.start..glyph.end];
                            let total = cluster.grapheme_indices(true).count();
                            let mut c_x = glyph.x;
                            let c_w = glyph.w / total as f32;
                            for (i, c) in cluster.grapheme_indices(true) {
                                let c_start = glyph.start + i;
                                let c_end = glyph.start + i + c.len();
                                if (start.line != line_i || c_end > start.index)
                                    && (end.line != line_i || c_start < end.index)
                                {
                                    range_opt = match range_opt.take() {
                                        Some((min, max)) => Some((
                                            cmp::min(min, c_x as i32),
                                            cmp::max(max, (c_x + c_w) as i32),
                                        )),
                                        None => Some((c_x as i32, (c_x + c_w) as i32)),
                                    };
                                } else if let Some((min, max)) = range_opt.take() {
                                    renderer.rectangle(
                                        min,
                                        line_top as i32,
                                        cmp::max(0, max - min) as u32,
                                        line_height as u32,
                                        selection_color,
                                    );
                                }
                                c_x += c_w;
                            }
                        }

                        if run.glyphs.is_empty() && end.line > line_i {
                            // Highlight all of internal empty lines
                            range_opt = Some((0, buffer.size().0.unwrap_or(0.0) as i32));
                        }

                        if let Some((mut min, mut max)) = range_opt.take() {
                            if end.line > line_i {
                                // Draw to end of line
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

                // Draw cursor
                if let Some((cursor_glyph, cursor_glyph_offset, cursor_glyph_width)) =
                    cursor_glyph_opt(&self.cursor())
                {
                    let block_cursor = if self.passthrough {
                        false
                    } else {
                        match self.parser.mode {
                            ViMode::Insert | ViMode::Replace => false,
                            _ => true, /*TODO: determine block cursor in other modes*/
                        }
                    };

                    let (start_x, end_x) = match run.glyphs.get(cursor_glyph) {
                        Some(glyph) => {
                            // Start of detected glyph
                            if glyph.level.is_rtl() {
                                (
                                    (glyph.x + glyph.w - cursor_glyph_offset) as i32,
                                    (glyph.x + glyph.w - cursor_glyph_offset - cursor_glyph_width)
                                        as i32,
                                )
                            } else {
                                (
                                    (glyph.x + cursor_glyph_offset) as i32,
                                    (glyph.x + cursor_glyph_offset + cursor_glyph_width) as i32,
                                )
                            }
                        }
                        None => match run.glyphs.last() {
                            Some(glyph) => {
                                // End of last glyph
                                if glyph.level.is_rtl() {
                                    (glyph.x as i32, (glyph.x - cursor_glyph_width) as i32)
                                } else {
                                    (
                                        (glyph.x + glyph.w) as i32,
                                        (glyph.x + glyph.w + cursor_glyph_width) as i32,
                                    )
                                }
                            }
                            None => {
                                // Start of empty line
                                (0, cursor_glyph_width as i32)
                            }
                        },
                    };

                    if block_cursor {
                        let left_x = cmp::min(start_x, end_x);
                        let right_x = cmp::max(start_x, end_x);
                        renderer.rectangle(
                            left_x,
                            line_top as i32,
                            (right_x - left_x) as u32,
                            line_height as u32,
                            selection_color,
                        );
                    } else {
                        renderer.rectangle(
                            start_x,
                            line_top as i32,
                            1,
                            line_height as u32,
                            cursor_color,
                        );
                    }
                }

                for glyph in run.glyphs.iter() {
                    let physical_glyph = glyph.physical((0., line_y), 1.0);

                    let glyph_color = match glyph.color_opt {
                        Some(some) => some,
                        None => foreground_color,
                    };

                    renderer.glyph(physical_glyph, glyph_color);
                }
            }
        });
    }
}

impl<'buffer> Edit<'buffer> for ViEditor<'_, 'buffer> {
    fn buffer_ref(&self) -> &BufferRef<'buffer> {
        self.editor.buffer_ref()
    }

    fn buffer_ref_mut(&mut self) -> &mut BufferRef<'buffer> {
        self.editor.buffer_ref_mut()
    }

    fn cursor(&self) -> Cursor {
        self.editor.cursor()
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        self.editor.set_cursor(cursor);
    }

    fn selection(&self) -> Selection {
        self.editor.selection()
    }

    fn set_selection(&mut self, selection: Selection) {
        self.editor.set_selection(selection);
    }

    fn auto_indent(&self) -> bool {
        self.editor.auto_indent()
    }

    fn set_auto_indent(&mut self, auto_indent: bool) {
        self.editor.set_auto_indent(auto_indent);
    }

    fn tab_width(&self) -> u16 {
        self.editor.tab_width()
    }

    fn set_tab_width(&mut self, tab_width: u16) {
        self.editor.set_tab_width(tab_width);
    }

    fn shape_as_needed(&mut self, font_system: &mut FontSystem, prune: bool) {
        self.editor.shape_as_needed(font_system, prune);
    }

    fn delete_range(&mut self, start: Cursor, end: Cursor) {
        self.editor.delete_range(start, end);
    }

    fn insert_at(&mut self, cursor: Cursor, data: &str, attrs_list: Option<AttrsList>) -> Cursor {
        self.editor.insert_at(cursor, data, attrs_list)
    }

    fn copy_selection(&self) -> Option<String> {
        self.editor.copy_selection()
    }

    fn delete_selection(&mut self) -> bool {
        self.editor.delete_selection()
    }

    fn apply_change(&mut self, change: &Change) -> bool {
        self.editor.apply_change(change)
    }

    fn start_change(&mut self) {
        self.editor.start_change();
    }

    fn finish_change(&mut self) -> Option<Change> {
        finish_change(
            &mut self.editor,
            &mut self.commands,
            &mut self.changed,
            self.save_pivot,
        )
    }

    fn action(&mut self, font_system: &mut FontSystem, action: Action) {
        log::debug!("Action {action:?}");

        let editor = &mut self.editor;

        // Ensure a change is always started
        editor.start_change();

        if self.passthrough {
            editor.action(font_system, action);
            // Always finish change when passing through (TODO: group changes)
            finish_change(
                editor,
                &mut self.commands,
                &mut self.changed,
                self.save_pivot,
            );
            return;
        }

        let key = match action {
            //TODO: this leaves lots of room for issues in translation, should we directly accept Key?
            Action::Backspace => Key::Backspace,
            Action::Delete => Key::Delete,
            Action::Motion(Motion::Down) => Key::Down,
            Action::Motion(Motion::End) => Key::End,
            Action::Enter => Key::Enter,
            Action::Escape => Key::Escape,
            Action::Motion(Motion::Home) => Key::Home,
            Action::Indent => Key::Tab,
            Action::Insert(c) => Key::Char(c),
            Action::Motion(Motion::Left) => Key::Left,
            Action::Motion(Motion::PageDown) => Key::PageDown,
            Action::Motion(Motion::PageUp) => Key::PageUp,
            Action::Motion(Motion::Right) => Key::Right,
            Action::Unindent => Key::Backtab,
            Action::Motion(Motion::Up) => Key::Up,
            _ => {
                log::debug!("Pass through action {action:?}");
                editor.action(font_system, action);
                // Always finish change when passing through (TODO: group changes)
                finish_change(
                    editor,
                    &mut self.commands,
                    &mut self.changed,
                    self.save_pivot,
                );
                return;
            }
        };

        let has_selection = !matches!(editor.selection(), Selection::None);

        self.parser.parse(key, has_selection, |event| {
            log::debug!("  Event {event:?}");
            let action = match event {
                Event::AutoIndent => {
                    log::info!("TODO: AutoIndent");
                    return;
                }
                Event::Backspace => Action::Backspace,
                Event::BackspaceInLine => {
                    let cursor = editor.cursor();
                    if cursor.index > 0 {
                        Action::Backspace
                    } else {
                        return;
                    }
                }
                Event::ChangeStart => {
                    editor.start_change();
                    return;
                }
                Event::ChangeFinish => {
                    finish_change(
                        editor,
                        &mut self.commands,
                        &mut self.changed,
                        self.save_pivot,
                    );
                    return;
                }
                Event::Delete => Action::Delete,
                Event::DeleteInLine => {
                    let cursor = editor.cursor();
                    if cursor.index
                        < editor.with_buffer(|buffer| buffer.lines[cursor.line].text().len())
                    {
                        Action::Delete
                    } else {
                        return;
                    }
                }
                Event::Escape => Action::Escape,
                Event::Insert(c) => Action::Insert(c),
                Event::NewLine => Action::Enter,
                Event::Put { register, after } => {
                    if let Some((selection, data)) = self.registers.get(&register) {
                        editor.start_change();
                        if editor.delete_selection() {
                            editor.insert_string(data, None);
                        } else {
                            match selection {
                                Selection::None | Selection::Normal(_) | Selection::Word(_) => {
                                    let mut cursor = editor.cursor();
                                    if after {
                                        editor.with_buffer(|buffer| {
                                            let text = buffer.lines[cursor.line].text();
                                            if let Some(c) = text[cursor.index..].chars().next() {
                                                cursor.index += c.len_utf8();
                                            }
                                        });
                                        editor.set_cursor(cursor);
                                    }
                                    editor.insert_at(cursor, data, None);
                                }
                                Selection::Line(_) => {
                                    let mut cursor = editor.cursor();
                                    if after {
                                        // Insert at next line
                                        cursor.line += 1;
                                    } else {
                                        // Previous line will be moved down, so set cursor to next line
                                        cursor.line += 1;
                                        editor.set_cursor(cursor);
                                        cursor.line -= 1;
                                    }
                                    // Insert at start of line
                                    cursor.index = 0;

                                    // Insert text
                                    editor.insert_at(cursor, "\n", None);
                                    editor.insert_at(cursor, data, None);

                                    //TODO: Hack to allow immediate up/down
                                    editor.shape_as_needed(font_system, false);

                                    // Move to inserted line, preserving cursor x position
                                    if after {
                                        editor.action(font_system, Action::Motion(Motion::Down));
                                    } else {
                                        editor.action(font_system, Action::Motion(Motion::Up));
                                    }
                                }
                            }
                        }
                        finish_change(
                            editor,
                            &mut self.commands,
                            &mut self.changed,
                            self.save_pivot,
                        );
                    }
                    return;
                }
                Event::Redraw => {
                    editor.with_buffer_mut(|buffer| buffer.set_redraw(true));
                    return;
                }
                Event::SelectClear => {
                    editor.set_selection(Selection::None);
                    return;
                }
                Event::SelectStart => {
                    let cursor = editor.cursor();
                    editor.set_selection(Selection::Normal(cursor));
                    return;
                }
                Event::SelectLineStart => {
                    let cursor = editor.cursor();
                    editor.set_selection(Selection::Line(cursor));
                    return;
                }
                Event::SelectTextObject(text_object, include) => {
                    match text_object {
                        TextObject::AngleBrackets => select_in(editor, '<', '>', include),
                        TextObject::CurlyBrackets => select_in(editor, '{', '}', include),
                        TextObject::DoubleQuotes => select_in(editor, '"', '"', include),
                        TextObject::Parentheses => select_in(editor, '(', ')', include),
                        TextObject::Search { forwards } => {
                            if let Some((value, _)) = &self.search_opt {
                                if search(editor, value, forwards) {
                                    let mut cursor = editor.cursor();
                                    editor.set_selection(Selection::Normal(cursor));
                                    //TODO: traverse lines if necessary
                                    cursor.index += value.len();
                                    editor.set_cursor(cursor);
                                }
                            }
                        }
                        TextObject::SingleQuotes => select_in(editor, '\'', '\'', include),
                        TextObject::SquareBrackets => select_in(editor, '[', ']', include),
                        TextObject::Ticks => select_in(editor, '`', '`', include),
                        TextObject::Word(word) => {
                            let mut cursor = editor.cursor();
                            let mut selection = editor.selection();
                            editor.with_buffer(|buffer| {
                                let text = buffer.lines[cursor.line].text();
                                match WordIter::new(text, word)
                                    .find(|&(i, w)| i <= cursor.index && i + w.len() > cursor.index)
                                {
                                    Some((i, w)) => {
                                        cursor.index = i;
                                        selection = Selection::Normal(cursor);
                                        cursor.index += w.len();
                                    }
                                    None => {
                                        //TODO
                                    }
                                }
                            });
                            editor.set_selection(selection);
                            editor.set_cursor(cursor);
                        }
                        _ => {
                            log::info!("TODO: {text_object:?}");
                        }
                    }
                    return;
                }
                Event::SetSearch(value, forwards) => {
                    self.search_opt = Some((value, forwards));
                    return;
                }
                Event::ShiftLeft => Action::Unindent,
                Event::ShiftRight => Action::Indent,
                Event::SwapCase => {
                    log::info!("TODO: SwapCase");
                    return;
                }
                Event::Undo => {
                    for action in self.commands.undo() {
                        undo_2_action(editor, action);
                    }
                    return;
                }
                Event::Yank { register } => {
                    if let Some(data) = editor.copy_selection() {
                        self.registers.insert(register, (editor.selection(), data));
                    }
                    return;
                }
                Event::Motion(motion) => {
                    match motion {
                        modit::Motion::Around => {
                            //TODO: what to do for this psuedo-motion?
                            return;
                        }
                        modit::Motion::Down => Action::Motion(Motion::Down),
                        modit::Motion::End => Action::Motion(Motion::End),
                        modit::Motion::GotoLine(line) => {
                            Action::Motion(Motion::GotoLine(line.saturating_sub(1)))
                        }
                        modit::Motion::GotoEof => Action::Motion(Motion::GotoLine(
                            editor.with_buffer(|buffer| buffer.lines.len().saturating_sub(1)),
                        )),
                        modit::Motion::Home => Action::Motion(Motion::Home),
                        modit::Motion::Inside => {
                            //TODO: what to do for this psuedo-motion?
                            return;
                        }
                        modit::Motion::Left => Action::Motion(Motion::Left),
                        modit::Motion::LeftInLine => {
                            let cursor = editor.cursor();
                            if cursor.index > 0 {
                                Action::Motion(Motion::Left)
                            } else {
                                return;
                            }
                        }
                        modit::Motion::Line => {
                            //TODO: what to do for this psuedo-motion?
                            return;
                        }
                        modit::Motion::NextChar(find_c) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                let text = buffer.lines[cursor.line].text();
                                if cursor.index < text.len() {
                                    if let Some((i, _)) = text[cursor.index..]
                                        .char_indices()
                                        .find(|&(i, c)| i > 0 && c == find_c)
                                    {
                                        cursor.index += i;
                                    }
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::NextCharTill(find_c) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                let text = buffer.lines[cursor.line].text();
                                if cursor.index < text.len() {
                                    let mut last_i = 0;
                                    for (i, c) in text[cursor.index..].char_indices() {
                                        if last_i > 0 && c == find_c {
                                            cursor.index += last_i;
                                            break;
                                        } else {
                                            last_i = i;
                                        }
                                    }
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::NextSearch => match &self.search_opt {
                            Some((value, forwards)) => {
                                search(editor, value, *forwards);
                                return;
                            }
                            None => return,
                        },
                        modit::Motion::NextWordEnd(word) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                loop {
                                    let text = buffer.lines[cursor.line].text();
                                    if cursor.index < text.len() {
                                        cursor.index = WordIter::new(text, word)
                                            .map(|(i, w)| {
                                                i + w
                                                    .char_indices()
                                                    .last()
                                                    .map(|(i, _)| i)
                                                    .unwrap_or(0)
                                            })
                                            .find(|&i| i > cursor.index)
                                            .unwrap_or(text.len());
                                        if cursor.index == text.len() {
                                            // Try again, searching next line
                                            continue;
                                        }
                                    } else if cursor.line + 1 < buffer.lines.len() {
                                        // Go to next line and rerun loop
                                        cursor.line += 1;
                                        cursor.index = 0;
                                        continue;
                                    }
                                    break;
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::NextWordStart(word) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                loop {
                                    let text = buffer.lines[cursor.line].text();
                                    if cursor.index < text.len() {
                                        cursor.index = WordIter::new(text, word)
                                            .map(|(i, _)| i)
                                            .find(|&i| i > cursor.index)
                                            .unwrap_or(text.len());
                                        if cursor.index == text.len() {
                                            // Try again, searching next line
                                            continue;
                                        }
                                    } else if cursor.line + 1 < buffer.lines.len() {
                                        // Go to next line and rerun loop
                                        cursor.line += 1;
                                        cursor.index = 0;
                                        continue;
                                    }
                                    break;
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::PageDown => Action::Motion(Motion::PageDown),
                        modit::Motion::PageUp => Action::Motion(Motion::PageUp),
                        modit::Motion::PreviousChar(find_c) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                let text = buffer.lines[cursor.line].text();
                                if cursor.index > 0 {
                                    if let Some((i, _)) = text[..cursor.index]
                                        .char_indices()
                                        .rfind(|&(_, c)| c == find_c)
                                    {
                                        cursor.index = i;
                                    }
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::PreviousCharTill(find_c) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                let text = buffer.lines[cursor.line].text();
                                if cursor.index > 0 {
                                    if let Some(i) = text[..cursor.index]
                                        .char_indices()
                                        .filter_map(|(i, c)| {
                                            if c == find_c {
                                                let end = i + c.len_utf8();
                                                if end < cursor.index {
                                                    return Some(end);
                                                }
                                            }
                                            None
                                        })
                                        .next_back()
                                    {
                                        cursor.index = i;
                                    }
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::PreviousSearch => match &self.search_opt {
                            Some((value, forwards)) => {
                                search(editor, value, !*forwards);
                                return;
                            }
                            None => return,
                        },
                        modit::Motion::PreviousWordEnd(word) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                loop {
                                    let text = buffer.lines[cursor.line].text();
                                    if cursor.index > 0 {
                                        cursor.index = WordIter::new(text, word)
                                            .map(|(i, w)| {
                                                i + w
                                                    .char_indices()
                                                    .last()
                                                    .map(|(i, _)| i)
                                                    .unwrap_or(0)
                                            })
                                            .filter(|&i| i < cursor.index)
                                            .last()
                                            .unwrap_or(0);
                                        if cursor.index == 0 {
                                            // Try again, searching previous line
                                            continue;
                                        }
                                    } else if cursor.line > 0 {
                                        // Go to previous line and rerun loop
                                        cursor.line -= 1;
                                        cursor.index = buffer.lines[cursor.line].text().len();
                                        continue;
                                    }
                                    break;
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::PreviousWordStart(word) => {
                            let mut cursor = editor.cursor();
                            editor.with_buffer(|buffer| {
                                loop {
                                    let text = buffer.lines[cursor.line].text();
                                    if cursor.index > 0 {
                                        cursor.index = WordIter::new(text, word)
                                            .map(|(i, _)| i)
                                            .filter(|&i| i < cursor.index)
                                            .last()
                                            .unwrap_or(0);
                                        if cursor.index == 0 {
                                            // Try again, searching previous line
                                            continue;
                                        }
                                    } else if cursor.line > 0 {
                                        // Go to previous line and rerun loop
                                        cursor.line -= 1;
                                        cursor.index = buffer.lines[cursor.line].text().len();
                                        continue;
                                    }
                                    break;
                                }
                            });
                            editor.set_cursor(cursor);
                            return;
                        }
                        modit::Motion::Right => Action::Motion(Motion::Right),
                        modit::Motion::RightInLine => {
                            let cursor = editor.cursor();
                            if cursor.index
                                < editor
                                    .with_buffer(|buffer| buffer.lines[cursor.line].text().len())
                            {
                                Action::Motion(Motion::Right)
                            } else {
                                return;
                            }
                        }
                        modit::Motion::ScreenHigh => {
                            //TODO: is this efficient?
                            if let Some(line_i) = editor.with_buffer(|buffer| {
                                buffer.layout_runs().next().map(|first| first.line_i)
                            }) {
                                Action::Motion(Motion::GotoLine(line_i))
                            } else {
                                return;
                            }
                        }
                        modit::Motion::ScreenLow => {
                            //TODO: is this efficient?
                            if let Some(line_i) = editor.with_buffer(|buffer| {
                                buffer.layout_runs().last().map(|last| last.line_i)
                            }) {
                                Action::Motion(Motion::GotoLine(line_i))
                            } else {
                                return;
                            }
                        }
                        modit::Motion::ScreenMiddle => {
                            //TODO: is this efficient?
                            let action_opt = editor.with_buffer(|buffer| {
                                let mut layout_runs = buffer.layout_runs();

                                match (layout_runs.next(), layout_runs.last()) {
                                    (Some(first), Some(last)) => Some(Action::Motion(
                                        Motion::GotoLine((last.line_i + first.line_i) / 2),
                                    )),
                                    _ => None,
                                }
                            });
                            match action_opt {
                                Some(action) => action,
                                None => return,
                            }
                        }
                        modit::Motion::Selection => {
                            //TODO: what to do for this psuedo-motion?
                            return;
                        }
                        modit::Motion::SoftHome => Action::Motion(Motion::SoftHome),
                        modit::Motion::Up => Action::Motion(Motion::Up),
                    }
                }
            };
            editor.action(font_system, action);
        });
    }

    fn cursor_position(&self) -> Option<(i32, i32)> {
        self.editor.cursor_position()
    }
}

impl BorrowedWithFontSystem<'_, ViEditor<'_, '_>> {
    /// Load text from a file, and also set syntax to the best option
    ///
    /// ## Errors
    /// Returns an `io::Error` if reading the file fails
    #[cfg(feature = "std")]
    pub fn load_text<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
        attrs: crate::Attrs,
    ) -> std::io::Result<()> {
        self.inner.load_text(self.font_system, path, attrs)
    }

    #[cfg(feature = "swash")]
    pub fn draw<F>(&mut self, cache: &mut crate::SwashCache, f: F)
    where
        F: FnMut(i32, i32, u32, u32, Color),
    {
        self.inner.draw(self.font_system, cache, f);
    }
}
