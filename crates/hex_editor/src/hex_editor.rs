mod hex_editor_settings;
mod status_button;

use std::path::PathBuf;

use anyhow::{Context as _, Result};
use gpui::{
    actions, div, prelude::*, px, uniform_list, AnyElement, App, ClipboardItem, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, Styled, Subscription, Task, UniformListScrollHandle, WeakEntity, Window,
};
use project::{PathChange, Project, ProjectPath, WorktreeId};
use settings::Settings;
use theme::ThemeSettings;
use ui::{h_flex, prelude::*, Icon, IconName};
use workspace::{
    delete_unloaded_items,
    item::{BreadcrumbText, Item, ItemEvent, SerializableItem, TabContentParams},
    ItemId, ItemSettings, ToolbarItemLocation, Workspace, WorkspaceId,
};

pub use crate::hex_editor_settings::*;
pub use crate::status_button::*;

// Actions
actions!(
    hex_editor,
    [
        OpenHexEditor,
        CloseHexEditor,
        ToggleHexEditor,
        GoToOffset,
        CopyAsHex,
        CopyAsAscii,
        CopyAsBinary,
        CopyAsDecimal,
        SelectAll,
        Find,
        FindNext,
        FindPrevious,
        ToggleDataInspector,
    ]
);

const BYTES_PER_ROW: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditMode {
    Hex,
    Ascii,
}

#[derive(Clone, Debug)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
}

impl Selection {
    pub fn new(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        let start = self.start.min(self.end);
        let end = self.start.max(self.end);
        start..end + 1
    }

    pub fn len(&self) -> usize {
        self.range().len()
    }
}

pub struct HexEditorView {
    project: Entity<Project>,
    file_path: PathBuf,
    project_path: Option<ProjectPath>,
    worktree_id: Option<WorktreeId>,
    data: Vec<u8>,
    modified_data: Vec<u8>,
    is_dirty: bool,
    scroll_handle: UniformListScrollHandle,
    cursor_offset: usize,
    selection: Option<Selection>,
    edit_mode: EditMode,
    focus_handle: FocusHandle,
    search_query: String,
    search_results: Vec<usize>,
    current_search_index: Option<usize>,
    is_dragging: bool,
    drag_start_offset: Option<usize>,
    append_buffer: String,

    _subscription: Subscription,
}

pub enum HexEditorEvent {
    TitleChanged,
    DirtyChanged,
}

impl EventEmitter<HexEditorEvent> for HexEditorView {}
impl EventEmitter<ItemEvent> for HexEditorView {}

impl HexEditorView {
    pub fn new(
        project: Entity<Project>,
        file_path: PathBuf,
        data: Vec<u8>,
        cx: &mut Context<Self>,
    ) -> Self {
        // Get the project path and worktree ID for file change detection
        let project_path = project
            .read(cx)
            .project_path_for_absolute_path(&file_path, cx);
        let worktree_id = project_path.as_ref().map(|pp| pp.worktree_id);

        // Subscribe to project events to detect file changes
        let subscription = cx.subscribe(&project, Self::handle_project_event);

        Self {
            project,
            file_path,
            project_path,
            worktree_id,
            modified_data: data.clone(),
            data,
            is_dirty: false,
            scroll_handle: UniformListScrollHandle::new(),
            cursor_offset: 0,
            selection: None,
            edit_mode: EditMode::Hex,
            focus_handle: cx.focus_handle(),
            search_query: String::new(),
            search_results: Vec::new(),
            current_search_index: None,
            is_dragging: false,
            drag_start_offset: None,
            append_buffer: String::new(),
            _subscription: subscription,
        }
    }

    fn handle_project_event(
        &mut self,
        _project: Entity<Project>,
        event: &project::Event,
        cx: &mut Context<Self>,
    ) {
        // Check if our file was updated
        if let project::Event::WorktreeUpdatedEntries(worktree_id, entries) = event {
            // Only process if this is our worktree
            if self.worktree_id != Some(*worktree_id) {
                return;
            }

            // Check if our file is in the updated entries
            if let Some(project_path) = &self.project_path {
                let our_path = &project_path.path;
                let file_changed = entries.iter().any(|(path, _, change)| {
                    path.as_ref() == our_path.as_ref()
                        && matches!(change, PathChange::Updated | PathChange::AddedOrUpdated)
                });

                if file_changed && !self.is_dirty {
                    // Reload the file if it changed and we don't have unsaved changes
                    self.reload_file(cx);
                }
            }
        }
    }

    fn reload_file(&mut self, cx: &mut Context<Self>) {
        let path = self.file_path.clone();
        cx.spawn(async move |this, cx| {
            let data = std::fs::read(&path)
                .with_context(|| format!("Failed to reload file: {}", path.display()))?;

            this.update(cx, |this, cx| {
                this.data = data.clone();
                this.modified_data = data;
                cx.notify();
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn open(
        project: Entity<Project>,
        file_path: PathBuf,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let path = file_path.clone();
        window.spawn(cx, async move |cx| {
            let data = std::fs::read(&path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;

            cx.update(|_window, cx| {
                Ok(cx.new(|cx| HexEditorView::new(project, file_path, data, cx)))
            })?
        })
    }

    pub fn from_project_path(
        project: Entity<Project>,
        project_path: ProjectPath,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let abs_path = project.read(cx).absolute_path(&project_path, cx);
        if let Some(path) = abs_path {
            Self::open(project, path, window, cx)
        } else {
            Task::ready(Err(anyhow::anyhow!("Could not resolve project path")))
        }
    }

    fn file_name(&self) -> String {
        self.file_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    // Public accessors for status bar
    pub fn cursor_offset(&self) -> usize {
        self.cursor_offset
    }

    pub fn file_size(&self) -> usize {
        self.modified_data.len()
    }

    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    pub fn edit_mode(&self) -> EditMode {
        self.edit_mode
    }

    /// Returns true if the cursor is at the append position (after the last byte)
    fn is_at_append_position(&self) -> bool {
        self.cursor_offset == self.modified_data.len()
    }

    fn total_rows(&self) -> usize {
        // Add 1 to account for the append position at the end
        // This ensures we have an extra row when data fills exactly to the end of a row
        (self.modified_data.len() + 1)
            .div_ceil(BYTES_PER_ROW)
            .max(1)
    }

    fn scroll_to_row(&mut self, row: usize) {
        self.scroll_handle
            .scroll_to_item(row, gpui::ScrollStrategy::Top);
    }

    fn scroll_to_offset(&mut self, offset: usize, _cx: &App) {
        let row = offset / BYTES_PER_ROW;
        self.scroll_to_row(row);
    }

    fn move_cursor(&mut self, new_offset: usize, extend_selection: bool, cx: &mut Context<Self>) {
        // Allow cursor to go to append position (data.len())
        let new_offset = new_offset.min(self.modified_data.len());

        // Clear append buffer when moving away from append position
        if self.is_at_append_position() && new_offset != self.modified_data.len() {
            self.append_buffer.clear();
        }

        if extend_selection {
            if let Some(ref mut sel) = self.selection {
                sel.end = new_offset;
            } else {
                self.selection = Some(Selection {
                    start: self.cursor_offset,
                    end: new_offset,
                });
            }
        } else {
            self.selection = None;
        }

        self.cursor_offset = new_offset;
        self.scroll_to_offset(new_offset, cx);
        cx.notify();
    }

    fn handle_key_down(
        &mut self,
        event: &gpui::KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let extend = event.keystroke.modifiers.shift;
        let data_len = self.modified_data.len();

        match event.keystroke.key.as_str() {
            "up" => {
                let new_offset = self.cursor_offset.saturating_sub(BYTES_PER_ROW);
                self.move_cursor(new_offset, extend, cx);
            }
            "down" => {
                let new_offset = (self.cursor_offset + BYTES_PER_ROW).min(data_len);
                self.move_cursor(new_offset, extend, cx);
            }
            "left" => {
                let new_offset = self.cursor_offset.saturating_sub(1);
                self.move_cursor(new_offset, extend, cx);
            }
            "right" => {
                // Allow moving to append position
                let new_offset = (self.cursor_offset + 1).min(data_len);
                self.move_cursor(new_offset, extend, cx);
            }
            "home" => {
                if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
                    self.move_cursor(0, extend, cx);
                } else {
                    let row_start = (self.cursor_offset / BYTES_PER_ROW) * BYTES_PER_ROW;
                    self.move_cursor(row_start, extend, cx);
                }
            }
            "end" => {
                if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
                    // End goes to append position
                    self.move_cursor(data_len, extend, cx);
                } else {
                    let row_start = (self.cursor_offset / BYTES_PER_ROW) * BYTES_PER_ROW;
                    let row_end = (row_start + BYTES_PER_ROW - 1).min(data_len);
                    self.move_cursor(row_end, extend, cx);
                }
            }
            "pageup" => {
                // Use a reasonable page size (20 rows)
                const PAGE_ROWS: usize = 20;
                let new_offset = self.cursor_offset.saturating_sub(BYTES_PER_ROW * PAGE_ROWS);
                self.move_cursor(new_offset, extend, cx);
            }
            "pagedown" => {
                // Use a reasonable page size (20 rows)
                const PAGE_ROWS: usize = 20;
                let new_offset = (self.cursor_offset + BYTES_PER_ROW * PAGE_ROWS).min(data_len);
                self.move_cursor(new_offset, extend, cx);
            }
            "tab" => {
                self.edit_mode = match self.edit_mode {
                    EditMode::Hex => EditMode::Ascii,
                    EditMode::Ascii => EditMode::Hex,
                };
                // Clear append buffer when switching modes
                self.append_buffer.clear();
                cx.notify();
            }
            "escape" => {
                if self.is_at_append_position() {
                    // Exit append mode - move cursor to the last byte
                    self.append_buffer.clear();
                    if !self.modified_data.is_empty() {
                        self.cursor_offset = self.modified_data.len() - 1;
                    }
                    cx.notify();
                } else {
                    self.selection = None;
                    cx.notify();
                }
            }
            "backspace" | "delete" => {
                self.handle_delete(event.keystroke.modifiers.shift, cx);
            }
            "a" if event.keystroke.modifiers.platform || event.keystroke.modifiers.control => {
                self.select_all(cx);
            }
            "c" if event.keystroke.modifiers.platform || event.keystroke.modifiers.control => {
                self.copy_selection(cx);
            }
            "x" if event.keystroke.modifiers.platform || event.keystroke.modifiers.control => {
                self.cut_selection(cx);
            }
            "v" if event.keystroke.modifiers.platform || event.keystroke.modifiers.control => {
                self.paste(cx);
            }
            "space" => {
                if self.edit_mode == EditMode::Ascii {
                    self.handle_character_input(' ', cx);
                }
            }
            key if key.len() == 1 => {
                let ch = key.chars().next().unwrap();
                // Handle caps lock and shift for case in ASCII mode
                let ch = if self.edit_mode == EditMode::Ascii && ch.is_ascii_alphabetic() {
                    let caps_on = window.capslock().on;
                    let shift = event.keystroke.modifiers.shift;
                    // CapsLock XOR Shift determines uppercase
                    if caps_on ^ shift {
                        ch.to_ascii_uppercase()
                    } else {
                        ch.to_ascii_lowercase()
                    }
                } else {
                    ch
                };
                self.handle_character_input(ch, cx);
            }
            _ => {}
        }
    }

    fn handle_character_input(&mut self, ch: char, cx: &mut Context<Self>) {
        // Handle append mode (cursor at append position)
        if self.is_at_append_position() {
            match self.edit_mode {
                EditMode::Hex => {
                    if let Some(_nibble) = ch.to_digit(16) {
                        self.append_buffer.push(ch.to_ascii_uppercase());
                        // When we have 2 hex characters, append the byte
                        if self.append_buffer.len() == 2 {
                            if let Ok(byte) = u8::from_str_radix(&self.append_buffer, 16) {
                                self.modified_data.push(byte);
                                // Keep cursor at append position (which is now data.len())
                                self.cursor_offset = self.modified_data.len();
                                self.is_dirty = true;
                                // Scroll to keep append position visible
                                self.scroll_to_offset(self.modified_data.len(), cx);
                                cx.emit(HexEditorEvent::DirtyChanged);
                            }
                            self.append_buffer.clear();
                        }
                        cx.notify();
                    }
                }
                EditMode::Ascii => {
                    if ch.is_ascii() && !ch.is_control() {
                        self.modified_data.push(ch as u8);
                        // Keep cursor at append position
                        self.cursor_offset = self.modified_data.len();
                        self.is_dirty = true;
                        // Scroll to keep append position visible
                        self.scroll_to_offset(self.modified_data.len(), cx);
                        cx.emit(HexEditorEvent::DirtyChanged);
                        cx.notify();
                    }
                }
            }
            return;
        }

        match self.edit_mode {
            EditMode::Hex => {
                if let Some(nibble) = ch.to_digit(16) {
                    let current = self.modified_data[self.cursor_offset];
                    // Alternate between high and low nibble
                    let new_byte = (current << 4) | (nibble as u8);
                    self.modified_data[self.cursor_offset] = new_byte;
                    self.is_dirty = true;
                    cx.emit(HexEditorEvent::DirtyChanged);
                    cx.notify();
                }
            }
            EditMode::Ascii => {
                if ch.is_ascii() && !ch.is_control() {
                    self.modified_data[self.cursor_offset] = ch as u8;
                    self.is_dirty = true;
                    // Move to next position, including append position
                    let new_offset = (self.cursor_offset + 1).min(self.modified_data.len());
                    self.cursor_offset = new_offset;
                    cx.emit(HexEditorEvent::DirtyChanged);
                    cx.notify();
                }
            }
        }
    }

    fn select_all(&mut self, cx: &mut Context<Self>) {
        if self.modified_data.is_empty() {
            return;
        }
        self.selection = Some(Selection {
            start: 0,
            end: self.modified_data.len() - 1,
        });
        cx.notify();
    }

    fn copy_selection(&self, cx: &mut Context<Self>) {
        let (start, end) = if let Some(selection) = &self.selection {
            let range = selection.range();
            (range.start, range.end)
        } else {
            // Copy single byte at cursor
            if self.cursor_offset >= self.modified_data.len() {
                return;
            }
            (self.cursor_offset, self.cursor_offset + 1)
        };

        let bytes = &self.modified_data[start..end];
        let text = match self.edit_mode {
            EditMode::Hex => bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" "),
            EditMode::Ascii => bytes
                .iter()
                .map(|&b| {
                    if b.is_ascii_graphic() || b == b' ' {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect(),
        };
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn cut_selection(&mut self, cx: &mut Context<Self>) {
        // First copy the selection
        self.copy_selection(cx);

        // Then delete it
        if let Some(selection) = self.selection.take() {
            let range = selection.range();
            let start = range.start.min(self.modified_data.len());
            let end = range.end.min(self.modified_data.len());

            if start < self.modified_data.len() {
                self.modified_data.drain(start..end);
                self.cursor_offset = start.min(self.modified_data.len().saturating_sub(1));
                self.is_dirty = true;
                cx.emit(HexEditorEvent::DirtyChanged);
                cx.notify();
            }
        } else if self.cursor_offset < self.modified_data.len() {
            // Cut single byte at cursor
            self.modified_data.remove(self.cursor_offset);
            if self.cursor_offset >= self.modified_data.len() && !self.modified_data.is_empty() {
                self.cursor_offset = self.modified_data.len() - 1;
            }
            self.is_dirty = true;
            cx.emit(HexEditorEvent::DirtyChanged);
            cx.notify();
        }
    }

    fn paste(&mut self, cx: &mut Context<Self>) {
        let Some(clipboard) = cx.read_from_clipboard() else {
            return;
        };
        let Some(text) = clipboard.text() else {
            return;
        };

        let bytes: Vec<u8> = match self.edit_mode {
            EditMode::Hex => {
                // Parse hex string (supports "FF FF FF" or "FFFFFF" formats)
                let hex_str: String = text.chars().filter(|c| c.is_ascii_hexdigit()).collect();
                hex_str
                    .as_bytes()
                    .chunks(2)
                    .filter_map(|chunk| {
                        if chunk.len() == 2 {
                            let s = std::str::from_utf8(chunk).ok()?;
                            u8::from_str_radix(s, 16).ok()
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            EditMode::Ascii => text
                .bytes()
                .filter(|&b| b.is_ascii() && b >= 0x20)
                .collect(),
        };

        if bytes.is_empty() {
            return;
        }

        // If at append position, append the bytes
        if self.is_at_append_position() {
            self.modified_data.extend(bytes);
            self.cursor_offset = self.modified_data.len();
        } else if let Some(selection) = self.selection.take() {
            // Replace selection with pasted bytes
            let range = selection.range();
            let start = range.start.min(self.modified_data.len());
            let end = range.end.min(self.modified_data.len());
            self.modified_data.splice(start..end, bytes.iter().cloned());
            self.cursor_offset = start + bytes.len();
        } else {
            // Insert at cursor position
            for (i, byte) in bytes.iter().enumerate() {
                self.modified_data.insert(self.cursor_offset + i, *byte);
            }
            self.cursor_offset += bytes.len();
        }

        self.cursor_offset = self.cursor_offset.min(self.modified_data.len());
        self.is_dirty = true;
        cx.emit(HexEditorEvent::DirtyChanged);
        cx.notify();
    }

    fn handle_append_click(&mut self, cx: &mut Context<Self>) {
        // Move cursor to append position
        self.cursor_offset = self.modified_data.len();
        self.append_buffer.clear();
        self.selection = None;
        self.scroll_to_offset(self.modified_data.len(), cx);
        cx.notify();
    }

    fn handle_delete(&mut self, delete_selection: bool, cx: &mut Context<Self>) {
        // If at append position, just move back
        if self.is_at_append_position() {
            self.append_buffer.clear();
            if !self.modified_data.is_empty() {
                self.cursor_offset = self.modified_data.len() - 1;
            }
            cx.notify();
            return;
        }

        if self.modified_data.is_empty() {
            return;
        }

        // Delete selection if shift is held and there's a selection
        if delete_selection {
            if let Some(selection) = &self.selection {
                let range = selection.range();
                let start = range.start.min(self.modified_data.len());
                let end = range.end.min(self.modified_data.len());

                if start < self.modified_data.len() {
                    self.modified_data.drain(start..end);
                    self.cursor_offset = start.min(self.modified_data.len().saturating_sub(1));
                    self.selection = None;
                    self.is_dirty = true;
                    cx.emit(HexEditorEvent::DirtyChanged);
                    cx.notify();
                }
                return;
            }
        }

        // Delete single byte at cursor
        if self.cursor_offset < self.modified_data.len() {
            self.modified_data.remove(self.cursor_offset);
            // Adjust cursor if we deleted the last byte
            if self.cursor_offset >= self.modified_data.len() && !self.modified_data.is_empty() {
                self.cursor_offset = self.modified_data.len() - 1;
            }
            self.selection = None;
            self.is_dirty = true;
            cx.emit(HexEditorEvent::DirtyChanged);
            cx.notify();
        }
    }

    fn handle_cell_mouse_down(
        &mut self,
        offset: usize,
        event: &gpui::MouseDownEvent,
        cx: &mut Context<Self>,
    ) {
        // Clear append buffer when clicking on a cell
        if self.is_at_append_position() {
            self.append_buffer.clear();
        }

        self.is_dragging = true;

        if event.modifiers.shift {
            let anchor = self.drag_start_offset.unwrap_or(offset);
            self.selection = Some(Selection {
                start: anchor,
                end: offset,
            });
            self.cursor_offset = offset;
        } else {
            self.drag_start_offset = Some(offset);
            self.cursor_offset = offset;
            self.selection = None;
        }
        cx.notify();
    }

    fn handle_cell_mouse_move(
        &mut self,
        offset: usize,
        _event: &gpui::MouseMoveEvent,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging {
            self.cursor_offset = offset;
            if let Some(start) = self.drag_start_offset {
                self.selection = Some(Selection { start, end: offset });
            }
            cx.notify();
        }
    }

    fn handle_mouse_up(
        &mut self,
        _event: &gpui::MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_dragging = false;
        cx.notify();
    }

    #[allow(dead_code)]
    fn search(&mut self, query: &str, cx: &mut Context<Self>) {
        self.search_query = query.to_string();
        self.search_results.clear();
        self.current_search_index = None;

        if query.is_empty() {
            cx.notify();
            return;
        }

        // Try to parse as hex bytes
        let search_bytes: Option<Vec<u8>> = if query.starts_with("0x") || query.contains(' ') {
            let hex_str = query.replace("0x", "").replace(' ', "");
            (0..hex_str.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex_str[i..i.min(hex_str.len()) + 2], 16).ok())
                .collect()
        } else {
            // Search as ASCII string
            Some(query.as_bytes().to_vec())
        };

        if let Some(bytes) = search_bytes {
            for i in 0..self.modified_data.len().saturating_sub(bytes.len() - 1) {
                if self.modified_data[i..].starts_with(&bytes) {
                    self.search_results.push(i);
                }
            }

            if !self.search_results.is_empty() {
                self.current_search_index = Some(0);
                self.cursor_offset = self.search_results[0];
                self.scroll_to_offset(self.cursor_offset, cx);
            }
        }

        cx.notify();
    }

    #[allow(dead_code)]
    fn find_next(&mut self, cx: &mut Context<Self>) {
        if self.search_results.is_empty() {
            return;
        }

        let next_index = match self.current_search_index {
            Some(i) => (i + 1) % self.search_results.len(),
            None => 0,
        };

        self.current_search_index = Some(next_index);
        self.cursor_offset = self.search_results[next_index];
        self.scroll_to_offset(self.cursor_offset, cx);
        cx.notify();
    }

    #[allow(dead_code)]
    fn find_previous(&mut self, cx: &mut Context<Self>) {
        if self.search_results.is_empty() {
            return;
        }

        let prev_index = match self.current_search_index {
            Some(0) => self.search_results.len() - 1,
            Some(i) => i - 1,
            None => self.search_results.len() - 1,
        };

        self.current_search_index = Some(prev_index);
        self.cursor_offset = self.search_results[prev_index];
        self.scroll_to_offset(self.cursor_offset, cx);
        cx.notify();
    }

    #[allow(dead_code)]
    fn copy_selection_as_hex(&self, cx: &mut App) {
        let range = self
            .selection
            .as_ref()
            .map(|s| s.range())
            .unwrap_or(self.cursor_offset..self.cursor_offset + 1);
        let bytes = &self.modified_data[range.start..range.end.min(self.modified_data.len())];
        let hex_string: String = bytes.iter().map(|b| format!("{:02X} ", b)).collect();
        cx.write_to_clipboard(ClipboardItem::new_string(hex_string.trim().to_string()));
    }

    #[allow(dead_code)]
    fn copy_selection_as_ascii(&self, cx: &mut App) {
        let range = self
            .selection
            .as_ref()
            .map(|s| s.range())
            .unwrap_or(self.cursor_offset..self.cursor_offset + 1);
        let bytes = &self.modified_data[range.start..range.end.min(self.modified_data.len())];
        let ascii_string: String = bytes
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        cx.write_to_clipboard(ClipboardItem::new_string(ascii_string));
    }

    fn render_row(&self, row_index: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        let font_family = settings.buffer_font.family.clone();
        let selection_range = self.selection.as_ref().map(|s| s.range());
        let row_offset = row_index * BYTES_PER_ROW;
        let border_color = cx.theme().colors().border;

        // Calculate character width based on font size (monospace fonts are ~0.6 * font_size wide)
        let char_width = font_size * 0.65;
        // Hex cell width: 2 characters + slight padding
        let hex_cell_width = char_width * 2.2;
        // Offset column width: 8 chars + padding
        let offset_width = char_width * 10.0;
        // ASCII cell width: 1 character + slight padding
        let ascii_cell_width = char_width * 1.2;
        // Gap after column 7 in hex view
        let hex_gap = char_width * 1.0;

        // Offset column - width for 8 hex digits
        let offset_element = div()
            .min_w(offset_width)
            .px_2()
            .text_size(font_size)
            .text_color(cx.theme().colors().text_muted)
            .font_family(font_family.clone())
            .whitespace_nowrap()
            .child(format!("{:08X}", row_offset));

        // Hex column
        let hex_elements: Vec<_> = (0..=BYTES_PER_ROW)
            .filter_map(|col| {
                let byte_offset = row_offset + col;

                // Show append cell at the position right after the last byte
                let append_col = self.modified_data.len() % BYTES_PER_ROW;
                let append_row = self.modified_data.len() / BYTES_PER_ROW;
                if byte_offset == self.modified_data.len()
                    && row_index == append_row
                    && col == append_col
                {
                    let is_active = self.is_at_append_position() && self.edit_mode == EditMode::Hex;
                    let display_text: SharedString = if is_active {
                        if self.append_buffer.is_empty() {
                            "__".into()
                        } else {
                            format!("{}_", self.append_buffer).into()
                        }
                    } else {
                        "+".into()
                    };
                    let bg_color = if is_active {
                        cx.theme().colors().element_selected
                    } else {
                        gpui::transparent_black()
                    };
                    return Some(
                        div()
                            .w(hex_cell_width)
                            .flex()
                            .justify_center()
                            .rounded_sm()
                            .bg(bg_color)
                            .text_size(font_size)
                            .text_color(cx.theme().colors().text_accent)
                            .font_family(font_family.clone())
                            .whitespace_nowrap()
                            .cursor_pointer()
                            .child(display_text)
                            .when(col == 7, |el| el.mr(hex_gap))
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    this.handle_append_click(cx);
                                }),
                            )
                            .into_any_element(),
                    );
                }

                // Skip the 17th column (col == 16) which is only for append cell positioning
                if col == BYTES_PER_ROW {
                    return None;
                }

                // Show placeholder for bytes past the data length (incomplete last row)
                if byte_offset >= self.modified_data.len() {
                    return Some(
                        div()
                            .w(hex_cell_width)
                            .text_size(font_size)
                            .font_family(font_family.clone())
                            .whitespace_nowrap()
                            .when(col == 7, |el| el.mr(hex_gap))
                            .child("  ")
                            .into_any_element(),
                    );
                }

                let byte = self.modified_data[byte_offset];
                let is_cursor =
                    byte_offset == self.cursor_offset && self.edit_mode == EditMode::Hex;
                let is_selected = selection_range
                    .as_ref()
                    .map(|r| r.contains(&byte_offset))
                    .unwrap_or(false);
                let is_modified = self.data.get(byte_offset) != self.modified_data.get(byte_offset);
                let is_search_result = self.search_results.contains(&byte_offset);

                let bg_color = if is_cursor {
                    cx.theme().colors().element_selected
                } else if is_selected {
                    cx.theme().colors().element_hover
                } else if is_search_result {
                    cx.theme()
                        .colors()
                        .editor_document_highlight_write_background
                } else {
                    gpui::transparent_black()
                };

                let text_color = if is_modified {
                    cx.theme().status().warning
                } else {
                    cx.theme().colors().text
                };

                let hex_str: SharedString = format!("{:02X}", byte).into();

                Some(
                    div()
                        .w(hex_cell_width)
                        .rounded_sm()
                        .bg(bg_color)
                        .text_size(font_size)
                        .text_color(text_color)
                        .font_family(font_family.clone())
                        .whitespace_nowrap()
                        .child(hex_str)
                        .when(col == 7, |el| el.mr(hex_gap))
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(move |this, event, _window, cx| {
                                this.handle_cell_mouse_down(byte_offset, event, cx);
                            }),
                        )
                        .on_mouse_move(cx.listener(move |this, event, _window, cx| {
                            this.handle_cell_mouse_move(byte_offset, event, cx);
                        }))
                        .into_any_element(),
                )
            })
            .collect();

        // ASCII column
        let ascii_elements: Vec<_> = (0..=BYTES_PER_ROW)
            .filter_map(|col| {
                let byte_offset = row_offset + col;

                // Show append cell at the position right after the last byte
                let append_col = self.modified_data.len() % BYTES_PER_ROW;
                let append_row = self.modified_data.len() / BYTES_PER_ROW;
                if byte_offset == self.modified_data.len()
                    && row_index == append_row
                    && col == append_col
                {
                    let is_active =
                        self.is_at_append_position() && self.edit_mode == EditMode::Ascii;
                    let display_text: SharedString =
                        if is_active { "_".into() } else { "+".into() };
                    let bg_color = if is_active {
                        cx.theme().colors().element_selected
                    } else {
                        gpui::transparent_black()
                    };
                    return Some(
                        div()
                            .w(ascii_cell_width)
                            .flex()
                            .justify_center()
                            .rounded_sm()
                            .bg(bg_color)
                            .text_size(font_size)
                            .text_color(cx.theme().colors().text_accent)
                            .font_family(font_family.clone())
                            .whitespace_nowrap()
                            .cursor_pointer()
                            .child(display_text)
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    this.handle_append_click(cx);
                                }),
                            )
                            .into_any_element(),
                    );
                }

                // Skip the 17th column (col == 16) which is only for append cell positioning
                if col == BYTES_PER_ROW {
                    return None;
                }

                // Show placeholder for bytes past the data length (incomplete last row)
                if byte_offset >= self.modified_data.len() {
                    return Some(
                        div()
                            .w(ascii_cell_width)
                            .text_size(font_size)
                            .font_family(font_family.clone())
                            .whitespace_nowrap()
                            .child(" ")
                            .into_any_element(),
                    );
                }

                let byte = self.modified_data[byte_offset];
                let ch = if byte.is_ascii_graphic() || byte == b' ' {
                    byte as char
                } else {
                    '.'
                };

                let is_cursor =
                    byte_offset == self.cursor_offset && self.edit_mode == EditMode::Ascii;
                let is_selected = selection_range
                    .as_ref()
                    .map(|r| r.contains(&byte_offset))
                    .unwrap_or(false);
                let is_printable = byte.is_ascii_graphic() || byte == b' ';

                let bg_color = if is_cursor {
                    cx.theme().colors().element_selected
                } else if is_selected {
                    cx.theme().colors().element_hover
                } else {
                    gpui::transparent_black()
                };

                let text_color = if is_printable {
                    cx.theme().colors().text
                } else {
                    cx.theme().colors().text_muted
                };

                let char_str: SharedString = ch.to_string().into();

                Some(
                    div()
                        .w(ascii_cell_width)
                        .rounded_sm()
                        .bg(bg_color)
                        .text_size(font_size)
                        .text_color(text_color)
                        .font_family(font_family.clone())
                        .whitespace_nowrap()
                        .child(char_str)
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(move |this, event, _window, cx| {
                                this.handle_cell_mouse_down(byte_offset, event, cx);
                            }),
                        )
                        .on_mouse_move(cx.listener(move |this, event, _window, cx| {
                            this.handle_cell_mouse_move(byte_offset, event, cx);
                        }))
                        .into_any_element(),
                )
            })
            .collect();

        h_flex()
            .w_full()
            .flex_shrink_0()
            .items_start()
            .child(offset_element)
            .child(div().w(px(1.0)).h_full().bg(border_color))
            .child(
                div()
                    .flex_shrink_0()
                    .px_2()
                    .child(div().flex().flex_row().gap_0().children(hex_elements)),
            )
            .child(div().w(px(1.0)).h_full().bg(border_color))
            .child(
                div()
                    .flex_shrink_0()
                    .px_2()
                    .child(div().flex().flex_row().gap_0().children(ascii_elements)),
            )
    }
}

impl Focusable for HexEditorView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for HexEditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let total_rows = self.total_rows();

        div()
            .track_focus(&focus_handle)
            .key_context("HexEditor")
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().colors().editor_background)
            .on_key_down(cx.listener(Self::handle_key_down))
            .on_mouse_up(gpui::MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .child(
                uniform_list(
                    "hex-editor-rows",
                    total_rows,
                    cx.processor(|this, range: std::ops::Range<usize>, _window, cx| {
                        range
                            .map(|row_index| this.render_row(row_index, cx).into_any_element())
                            .collect()
                    }),
                )
                .size_full()
                .track_scroll(&self.scroll_handle),
            )
    }
}

impl Item for HexEditorView {
    type Event = HexEditorEvent;

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            HexEditorEvent::TitleChanged => {
                f(ItemEvent::UpdateTab);
                f(ItemEvent::UpdateBreadcrumbs);
            }
            HexEditorEvent::DirtyChanged => {
                f(ItemEvent::UpdateTab);
            }
        }
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(self.file_path.display().to_string().into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, _cx: &App) -> AnyElement {
        let label = if self.is_dirty {
            format!("{} (Hex) •", self.file_name())
        } else {
            format!("{} (Hex)", self.file_name())
        };

        Label::new(label)
            .color(params.text_color())
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("{} (Hex)", self.file_name()).into()
    }

    fn tab_icon(&self, _window: &Window, cx: &App) -> Option<Icon> {
        if ItemSettings::get_global(cx).file_icons {
            Some(Icon::new(IconName::Binary))
        } else {
            None
        }
    }

    fn breadcrumb_location(&self, _cx: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, _theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        let settings = ThemeSettings::get_global(cx);
        Some(vec![BreadcrumbText {
            text: self.file_path.display().to_string(),
            highlights: None,
            font: Some(settings.buffer_font.clone()),
        }])
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        self.is_dirty
    }

    fn can_save(&self, _cx: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        _options: workspace::item::SaveOptions,
        _project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let data = self.modified_data.clone();
        let path = self.file_path.clone();

        cx.spawn(async move |this, cx| {
            std::fs::write(&path, &data)
                .with_context(|| format!("Failed to save file: {}", path.display()))?;

            this.update(cx, |this, cx| {
                this.data = this.modified_data.clone();
                this.is_dirty = false;
                cx.emit(HexEditorEvent::DirtyChanged);
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        let project = self.project.clone();
        let file_path = self.file_path.clone();
        let project_path = self.project_path.clone();
        let worktree_id = self.worktree_id;
        let data = self.data.clone();
        let modified_data = self.modified_data.clone();
        let is_dirty = self.is_dirty;
        let cursor_offset = self.cursor_offset;
        let selection = self.selection.clone();
        let edit_mode = self.edit_mode;
        let search_query = self.search_query.clone();
        let search_results = self.search_results.clone();
        let current_search_index = self.current_search_index;

        Task::ready(Some(cx.new(|cx| {
            let subscription = cx.subscribe(&project, Self::handle_project_event);
            Self {
                project,
                file_path,
                project_path,
                worktree_id,
                data,
                modified_data,
                is_dirty,
                scroll_handle: UniformListScrollHandle::new(),
                cursor_offset,
                selection,
                edit_mode,
                focus_handle: cx.focus_handle(),
                search_query,
                search_results,
                current_search_index,
                is_dragging: false,
                drag_start_offset: None,
                append_buffer: String::new(),
                _subscription: subscription,
            }
        })))
    }

    fn show_toolbar(&self) -> bool {
        true
    }
}

impl SerializableItem for HexEditorView {
    fn serialized_item_kind() -> &'static str {
        "HexEditorView"
    }

    fn deserialize(
        project: Entity<Project>,
        _workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            let hex_path = persistence::HEX_EDITOR_DB
                .get_hex_path(item_id, workspace_id)?
                .context("No hex path found")?;

            let data = std::fs::read(&hex_path)
                .with_context(|| format!("Failed to read file: {}", hex_path.display()))?;

            cx.update(|_window, cx| {
                Ok(cx.new(|cx| HexEditorView::new(project, hex_path, data, cx)))
            })?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "hex_editors",
            &persistence::HEX_EDITOR_DB,
            cx,
        )
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let hex_path = self.file_path.clone();

        Some(cx.background_spawn(async move {
            log::debug!("Saving hex editor at path {:?}", hex_path);
            persistence::HEX_EDITOR_DB
                .save_hex_path(item_id, workspace_id, hex_path)
                .await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

/// Opens a file in the hex editor
pub fn open_hex_editor(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();

    // Get the current active item's path
    let Some(active_item) = workspace.active_item(cx) else {
        return;
    };

    let Some(project_path) = active_item.project_path(cx) else {
        return;
    };

    let Some(abs_path) = project.read(cx).absolute_path(&project_path, cx) else {
        return;
    };

    let task = HexEditorView::open(project, abs_path, window, cx);

    cx.spawn_in(window, async move |workspace, cx| {
        let hex_editor = task.await?;
        workspace.update_in(&mut cx.clone(), |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(hex_editor), None, true, window, cx);
        })?;
        Ok::<_, anyhow::Error>(())
    })
    .detach_and_log_err(cx);
}

pub fn init(cx: &mut App) {
    HexEditorSettings::register(cx);

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &OpenHexEditor, window, cx| {
            open_hex_editor(workspace, window, cx);
        });
    })
    .detach();

    workspace::register_serializable_item::<HexEditorView>(cx);
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct HexEditorDb(ThreadSafeConnection);

    impl Domain for HexEditorDb {
        const NAME: &str = stringify!(HexEditorDb);

        const MIGRATIONS: &[&str] = &[sql!(
            CREATE TABLE hex_editors (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                hex_path BLOB,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
    }

    db::static_connection!(HEX_EDITOR_DB, HexEditorDb, [WorkspaceDb]);

    impl HexEditorDb {
        query! {
            pub async fn save_hex_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                hex_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO hex_editors(item_id, workspace_id, hex_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_hex_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT hex_path
                FROM hex_editors
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
