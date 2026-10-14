use std::{
    fmt::Write as _,
    ops::Range,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow, ensure};
use futures::{FutureExt as _, future::Shared};
use gpui::{
    AppContext, ClipboardItem, Entity, EventEmitter, FocusHandle, Focusable, KeyDownEvent,
    ListHorizontalSizingBehavior, ScrollStrategy, Task, UniformListScrollHandle, actions,
    uniform_list,
};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::prelude::*;
use ui_input::InputField;
use util::ResultExt;
use workspace::{
    Item, Workspace,
    invalid_item_view::InvalidItemView,
    item::{ItemBufferKind, ItemEvent, ItemHandle, SaveOptions},
};

use crate::actions::{Copy, Paste, Redo, SelectAll, Undo};

mod bookmarks;
mod change_tracking;
mod comparison;
mod data_inspector;
mod search;

use bookmarks::{Bookmark, MAX_BOOKMARKS, METADATA_NAMESPACE, Metadata};
use change_tracking::{ByteSpan, ChangeTracking};
use comparison::{Comparison, Difference};
use data_inspector::{Endian, NumberType};
use search::{SearchMode, SearchPattern};

actions!(
    hex_editor,
    [
        /// Open the current file in the hex editor.
        Open,
        /// Choose a local file to open in the hex editor.
        OpenFile,
        /// Switch between inserting and overwriting bytes.
        ToggleInsertMode,
        /// Switch between hexadecimal and ASCII editing.
        ToggleAsciiMode,
        /// Find or replace hexadecimal byte patterns.
        Find,
        /// Find the next occurrence of the byte pattern.
        FindNext,
        /// Find the previous occurrence of the byte pattern.
        FindPrevious,
        /// Fill selected bytes with a repeating hexadecimal pattern.
        Fill,
        /// Jump to a hexadecimal or decimal byte offset.
        GoToOffset,
        /// Select an exact byte range by offset and length or end offset.
        SelectRange,
        /// Inspect and edit numeric values and individual bits.
        Inspect,
        /// Choose a new local path for the current hex file.
        SaveAs,
        /// Disable or enable byte editing for this file.
        ToggleReadOnly,
        /// Add, navigate, or remove named byte bookmarks.
        Bookmarks,
        /// Add or edit notes attached to a byte range.
        Annotations,
        /// Save the bookmark or annotation being edited.
        SaveAnnotation,
        /// Compare this buffer with another local binary file.
        CompareFiles,
        /// Select the next differing byte region.
        NextDifference,
        /// Select the previous differing byte region.
        PreviousDifference,
        /// Close the binary comparison without changing either file.
        CloseComparison
    ]
);

const BYTES_PER_ROW: usize = 16;
const MAX_FILE_SIZE: usize = 64 * 1024 * 1024;

pub fn init(cx: &mut App) {
    #[cfg(test)]
    if !cx.has_global::<db::AppDatabase>() {
        cx.set_global(db::AppDatabase::test_new());
    }
    if !cx.has_global::<MetadataCache>() {
        let quit = cx.on_app_quit(|cx| {
            let writes: Vec<_> = cx
                .global::<MetadataCache>()
                .writes
                .values()
                .cloned()
                .collect();
            async move {
                for write in writes {
                    write.await.log_err();
                }
            }
        });
        cx.set_global(MetadataCache {
            _quit: Some(quit),
            ..Default::default()
        });
    }
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(open);
        workspace.register_action(open_file);
        workspace.register_action(save_as);
    })
    .detach();
}

#[derive(Default)]
struct MetadataCache {
    snapshots: std::collections::HashMap<String, String>,
    writes: std::collections::HashMap<String, Shared<Task<std::result::Result<(), String>>>>,
    _quit: Option<gpui::Subscription>,
}

impl gpui::Global for MetadataCache {}

fn save_as(
    workspace: &mut Workspace,
    _: &SaveAs,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if workspace.active_item_as::<HexEditor>(cx).is_none() {
        return;
    }
    let save = workspace.save_active_item(workspace::SaveIntent::SaveAs, window, cx);
    cx.spawn(async move |workspace, cx| {
        if let Err(error) = save.await {
            workspace
                .update(cx, |workspace, cx| workspace.show_error(error, cx))
                .log_err();
        }
    })
    .detach();
}

fn open(workspace: &mut Workspace, _: &Open, window: &mut Window, cx: &mut Context<Workspace>) {
    let project = workspace.project().clone();
    if !project.read(cx).is_local() {
        workspace.show_error(
            "Hex editing is currently available only for local files",
            cx,
        );
        return;
    }
    let Some(item) = workspace.active_item(cx) else {
        workspace.show_error("Open a file before opening the hex editor", cx);
        return;
    };
    let path = item
        .project_path(cx)
        .and_then(|path| project.read(cx).absolute_path(&path, cx))
        .or_else(|| {
            item.act_as::<InvalidItemView>(cx)
                .map(|item| item.read(cx).abs_path.to_path_buf())
        });
    let Some(path) = path else {
        workspace.show_error("The current item is not a file", cx);
        return;
    };
    open_path(workspace, path, window, cx);
}

fn open_file(
    workspace: &mut Workspace,
    _: &OpenFile,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if !workspace.project().read(cx).is_local() {
        workspace.show_error(
            "Hex editing is currently available only for local files",
            cx,
        );
        return;
    }
    let paths = cx.prompt_for_paths(gpui::PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: Some("Open in Hex Editor".into()),
    });
    cx.spawn_in(window, async move |workspace, cx| {
        let result: Result<()> = async {
            if let Some(path) = paths.await??.and_then(|paths| paths.into_iter().next()) {
                workspace.update_in(cx, |workspace, window, cx| {
                    open_path(workspace, path, window, cx);
                })?;
            }
            Ok(())
        }
        .await;
        if let Err(error) = result {
            workspace
                .update(cx, |workspace, cx| workspace.show_error(error, cx))
                .log_err();
        }
    })
    .detach();
}

pub fn open_path(
    workspace: &mut Workspace,
    path: PathBuf,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();
    if !project.read(cx).is_local() {
        workspace.show_error(
            "Hex editing is currently available only for local files",
            cx,
        );
        return;
    }
    let existing = workspace
        .items_of_type::<HexEditor>(cx)
        .find(|view| view.read(cx).path == path);
    if let Some(existing) = existing {
        workspace.activate_item(&existing, true, true, window, cx);
        return;
    }
    let has_text_edits = workspace.items(cx).any(|item| {
        item.is_dirty(cx)
            && item.act_as::<HexEditor>(cx).is_none()
            && item
                .project_path(cx)
                .and_then(|path| project.read(cx).absolute_path(&path, cx))
                .as_ref()
                == Some(&path)
    });
    if has_text_edits {
        workspace.show_error(
            "Save the current file before opening it in the hex editor",
            cx,
        );
        return;
    }

    let fs = project.read(cx).fs().clone();
    let load = cx.background_spawn({
        let path = path.clone();
        async move { load_file(fs.as_ref(), &path).await }
    });
    cx.spawn_in(window, async move |workspace, cx| {
        let result: Result<()> = async {
            let bytes = load.await?;
            workspace.update_in(cx, |workspace, window, cx| {
                // Another open request may have finished while the file was loading.
                let existing = workspace
                    .items_of_type::<HexEditor>(cx)
                    .find(|view| view.read(cx).path == path);
                if let Some(existing) = existing {
                    workspace.activate_item(&existing, true, true, window, cx);
                } else {
                    let view = cx.new(|cx| HexEditor::new(path, project, bytes, cx));
                    workspace.add_item_to_center(Box::new(view), window, cx);
                }
            })?;
            Ok(())
        }
        .await;
        if let Err(error) = result {
            workspace
                .update(cx, |workspace, cx| workspace.show_error(error, cx))
                .log_err();
        }
    })
    .detach();
}

async fn load_file(fs: &dyn fs::Fs, path: &Path) -> Result<Vec<u8>> {
    let metadata = fs
        .metadata(path)
        .await?
        .ok_or_else(|| anyhow!("The file no longer exists"))?;
    ensure!(
        fs.is_file(path).await,
        "Open a regular file in the hex editor"
    );
    ensure!(
        metadata.len <= MAX_FILE_SIZE as u64,
        "Hex editing supports files up to 64 MiB"
    );
    let bytes = fs.load_bytes(path).await?;
    ensure!(bytes.len() <= MAX_FILE_SIZE, "The file grew beyond 64 MiB");
    Ok(bytes)
}

pub(crate) fn recover_binary_file(
    project: &Entity<project::Project>,
    path: &Path,
    window: &mut Window,
    cx: &mut App,
) -> Option<Task<Result<Option<Box<dyn ItemHandle>>>>> {
    if !project.read(cx).is_local() {
        return None;
    }
    let project = project.clone();
    let path = path.to_path_buf();
    let fs = project.read(cx).fs().clone();
    let load = cx.background_spawn({
        let path = path.clone();
        async move {
            let mut file = fs.open_sync(&path).await?;
            let (prefix, _) = file_content::read_byte_header(&mut *file)?;
            if file_content::decode_byte_header(&prefix).1 != file_content::ByteContent::Binary {
                return anyhow::Ok(None);
            }
            load_file(fs.as_ref(), &path).await.map(Some)
        }
    });
    Some(window.spawn(cx, async move |cx| {
        if let Some(bytes) = load.await? {
            let view = cx.new(|cx| HexEditor::new(path, project, bytes, cx));
            Ok(Some(Box::new(view) as Box<dyn ItemHandle>))
        } else {
            Ok(None)
        }
    }))
}

struct Edit {
    offset: usize,
    before: Vec<u8>,
    after: Vec<u8>,
    before_spans: Vec<ByteSpan>,
    after_spans: Vec<ByteSpan>,
    before_revision: u64,
    after_revision: u64,
}

struct HexBuffer {
    bytes: Vec<u8>,
    saved_bytes: Vec<u8>,
    changes: ChangeTracking,
    revision: u64,
    saved_revision: u64,
    next_revision: u64,
    undo: Vec<Edit>,
    redo: Vec<Edit>,
}

impl HexBuffer {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            changes: ChangeTracking::new(bytes.len()),
            saved_bytes: bytes.clone(),
            bytes,
            revision: 0,
            saved_revision: 0,
            next_revision: 1,
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    fn is_dirty(&self) -> bool {
        self.revision != self.saved_revision
    }

    fn replace(&mut self, range: Range<usize>, bytes: Vec<u8>) -> Result<()> {
        ensure!(
            range.start <= range.end && range.end <= self.bytes.len(),
            "Invalid byte range"
        );
        ensure!(
            self.bytes.len() - range.len() + bytes.len() <= MAX_FILE_SIZE,
            "Hex editing supports files up to 64 MiB"
        );
        let spans = self.changes.replacement(range.clone(), bytes.len())?;
        self.replace_with_spans(range, bytes, spans)
    }

    fn replace_with_spans(
        &mut self,
        range: Range<usize>,
        bytes: Vec<u8>,
        spans: Vec<ByteSpan>,
    ) -> Result<()> {
        let before = self
            .bytes
            .get(range.clone())
            .ok_or_else(|| anyhow!("Invalid byte range"))?;
        if before == bytes {
            return Ok(());
        }
        ensure!(
            self.bytes.len() - range.len() + bytes.len() <= MAX_FILE_SIZE,
            "Hex editing supports files up to 64 MiB"
        );
        let edit = Edit {
            offset: range.start,
            before: before.to_vec(),
            after: bytes,
            before_spans: self.changes.slice(range.clone()),
            after_spans: spans,
            before_revision: self.revision,
            after_revision: self.next_revision,
        };
        self.next_revision += 1;
        self.changes.splice(range.clone(), &edit.after_spans);
        self.bytes.splice(range, edit.after.iter().copied());
        self.revision = edit.after_revision;
        self.undo.push(edit);
        self.redo.clear();
        Ok(())
    }

    fn undo(&mut self) -> Option<usize> {
        let edit = self.undo.pop()?;
        self.changes.splice(
            edit.offset..edit.offset + edit.after.len(),
            &edit.before_spans,
        );
        self.bytes.splice(
            edit.offset..edit.offset + edit.after.len(),
            edit.before.iter().copied(),
        );
        self.revision = edit.before_revision;
        let offset = edit.offset;
        self.redo.push(edit);
        Some(offset)
    }

    fn redo(&mut self) -> Option<usize> {
        let edit = self.redo.pop()?;
        self.changes.splice(
            edit.offset..edit.offset + edit.before.len(),
            &edit.after_spans,
        );
        self.bytes.splice(
            edit.offset..edit.offset + edit.before.len(),
            edit.after.iter().copied(),
        );
        self.revision = edit.after_revision;
        let offset = edit.offset;
        self.undo.push(edit);
        Some(offset)
    }

    #[cfg(test)]
    fn replace_all(&mut self, pattern: &[u8], replacement: &[u8]) -> Result<usize> {
        ensure!(!pattern.is_empty(), "Enter bytes to find");
        let pattern = SearchMode::Hex.pattern(
            &pattern
                .iter()
                .map(|byte| format!("{byte:02X}"))
                .collect::<String>(),
            NumberType::U8,
            Endian::Little,
        )?;
        self.replace_pattern(&pattern, replacement)
    }

    fn replace_pattern(&mut self, pattern: &SearchPattern, replacement: &[u8]) -> Result<usize> {
        let mut bytes = Vec::new();
        let mut spans = Vec::new();
        let mut count = 0;
        let mut end = 0;
        for found in pattern.matches_in(&self.bytes) {
            let prefix = &self.bytes[end..found.start];
            ensure!(
                bytes.len() + prefix.len() + replacement.len() <= MAX_FILE_SIZE,
                "Hex editing supports files up to 64 MiB"
            );
            bytes.extend_from_slice(prefix);
            bytes.extend_from_slice(replacement);
            spans.extend(self.changes.slice(end..found.start));
            spans.extend(self.changes.replacement(found.clone(), replacement.len())?);
            end = found.end;
            count += 1;
        }
        let remaining = &self.bytes[end..];
        ensure!(
            bytes.len() + remaining.len() <= MAX_FILE_SIZE,
            "Hex editing supports files up to 64 MiB"
        );
        if count > 0 {
            bytes.extend_from_slice(remaining);
            spans.extend(self.changes.slice(end..self.bytes.len()));
            self.replace_with_spans(0..self.bytes.len(), bytes, spans)?;
        }
        Ok(count)
    }

    fn byte_change(&self, offset: usize) -> Option<ByteChange> {
        let byte = self.bytes.get(offset)?;
        match self.changes.saved_offset(offset) {
            None => Some(ByteChange::Inserted),
            Some(saved) if self.saved_bytes.get(saved) != Some(byte) => Some(ByteChange::Modified),
            Some(_) => None,
        }
    }

    fn mark_saved(&mut self) {
        self.saved_bytes = self.bytes.clone();
        self.saved_revision = self.revision;
        self.changes.mark_saved();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ByteChange {
    Inserted,
    Modified,
}

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Find,
    Fill,
    Offset,
    Range,
    Inspector,
    Bookmarks,
}

struct ToolInputs {
    pattern: Entity<InputField>,
    replacement: Entity<InputField>,
    fill: Entity<InputField>,
    offset: Entity<InputField>,
    range_start: Entity<InputField>,
    range_size: Entity<InputField>,
    number: Entity<InputField>,
    bookmark: Entity<InputField>,
    notes: Entity<InputField>,
}

pub struct HexEditor {
    path: PathBuf,
    project: Entity<project::Project>,
    buffer: HexBuffer,
    cursor: usize,
    anchor: Option<usize>,
    low_nibble: bool,
    insert: bool,
    ascii: bool,
    active_tool: Option<Tool>,
    inputs: Option<ToolInputs>,
    message: Option<SharedString>,
    range_uses_end: bool,
    number_type: NumberType,
    endian: Endian,
    number_source: Option<(usize, usize, u64, NumberType, Endian)>,
    search_mode: SearchMode,
    read_only: bool,
    bookmarks: Vec<Bookmark>,
    next_bookmark_id: usize,
    editing_bookmark: Option<usize>,
    saved_fingerprint: String,
    metadata_write: Option<Shared<Task<std::result::Result<(), String>>>>,
    metadata_generation: u64,
    metadata_error: Option<SharedString>,
    comparison: Option<Comparison>,
    comparison_load: Option<Shared<Task<std::result::Result<(), String>>>>,
    comparison_stats_task: Option<Task<()>>,
    comparison_generation: u64,
    diff_region: Option<Range<usize>>,
    busy: bool,
    error: Option<SharedString>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
}

impl HexEditor {
    pub fn annotation_notes(&self, name: &str) -> Option<&str> {
        self.bookmarks
            .iter()
            .find(|bookmark| bookmark.name == name)
            .map(|bookmark| bookmark.notes.as_str())
    }

    fn new(
        path: PathBuf,
        project: Entity<project::Project>,
        bytes: Vec<u8>,
        cx: &mut Context<Self>,
    ) -> Self {
        let saved_fingerprint = bookmarks::fingerprint(&bytes);
        let buffer = HexBuffer::new(bytes);
        let key = bookmarks::metadata_key(&path);
        let metadata = if let Some(json) = cx
            .try_global::<MetadataCache>()
            .and_then(|cache| cache.snapshots.get(&key).cloned())
        {
            Ok(Some(json))
        } else {
            db::kvp::KeyValueStore::global(cx)
                .scoped(METADATA_NAMESPACE)
                .read(&key)
        };
        let mut metadata_error = None;
        let bookmarks = metadata
            .and_then(|json| json.map(|json| Metadata::decode(&json)).transpose())
            .map(|metadata| {
                metadata
                    .map(|metadata| {
                        metadata.restore(&buffer.changes, buffer.bytes.len(), &saved_fingerprint)
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_else(|error| {
                metadata_error = Some(format!("Could not load annotations: {error}").into());
                Vec::new()
            });
        let next_bookmark_id = bookmarks.len();
        Self {
            path,
            project,
            buffer,
            cursor: 0,
            anchor: None,
            low_nibble: false,
            insert: false,
            ascii: false,
            active_tool: None,
            inputs: None,
            message: None,
            range_uses_end: false,
            number_type: NumberType::U8,
            endian: Endian::Little,
            number_source: None,
            search_mode: SearchMode::Hex,
            read_only: false,
            bookmarks,
            next_bookmark_id,
            editing_bookmark: None,
            saved_fingerprint,
            metadata_write: None,
            metadata_generation: 0,
            metadata_error,
            comparison: None,
            comparison_load: None,
            comparison_stats_task: None,
            comparison_generation: 0,
            diff_region: None,
            busy: false,
            error: None,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
        }
    }

    fn selection(&self) -> Range<usize> {
        let anchor = self.anchor.unwrap_or(self.cursor);
        anchor.min(self.cursor)..anchor.max(self.cursor)
    }

    fn bytes_read_only(&self) -> bool {
        self.read_only || self.comparison.is_some()
    }

    fn move_to(&mut self, offset: usize, select: bool, cx: &mut Context<Self>) {
        self.diff_region = None;
        if select {
            self.anchor.get_or_insert(self.cursor);
        } else {
            self.anchor = None;
        }
        self.cursor = offset.min(self.buffer.bytes.len());
        self.low_nibble = false;
        self.scroll_handle
            .scroll_to_item(self.cursor / BYTES_PER_ROW, ScrollStrategy::Nearest);
        cx.notify();
    }

    fn edited(&mut self, result: Result<()>, cx: &mut Context<Self>) {
        self.error = result.err().map(|error| error.to_string().into());
        self.diff_region = None;
        self.update_comparison(cx);
        cx.emit(ItemEvent::Edit);
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    fn input_digit(&mut self, digit: u8, cx: &mut Context<Self>) {
        if self.busy || self.bytes_read_only() {
            return;
        }
        let selection = self.selection();
        let offset = selection.start;
        let (range, value) = if !selection.is_empty() {
            self.cursor = offset;
            self.low_nibble = false;
            (selection, digit << 4)
        } else if self.low_nibble {
            let Some(byte) = self.buffer.bytes.get(offset) else {
                return;
            };
            (offset..offset + 1, (byte & 0xf0) | digit)
        } else if self.insert || offset == self.buffer.bytes.len() {
            (offset..offset, digit << 4)
        } else {
            let Some(byte) = self.buffer.bytes.get(offset) else {
                return;
            };
            (offset..offset + 1, (digit << 4) | (byte & 0x0f))
        };
        let result = self.buffer.replace(range, vec![value]);
        if result.is_ok() {
            self.anchor = None;
            if self.low_nibble {
                self.move_to(offset + 1, false, cx);
            } else {
                self.low_nibble = true;
            }
        }
        self.edited(result, cx);
    }

    fn input_bytes(&mut self, bytes: Vec<u8>, cx: &mut Context<Self>) {
        if self.busy || self.bytes_read_only() || bytes.is_empty() {
            return;
        }
        let mut range = self.selection();
        if range.is_empty() && !self.insert {
            range.end = (range.start + bytes.len()).min(self.buffer.bytes.len());
        }
        let offset = range.start + bytes.len();
        let result = self.buffer.replace(range, bytes);
        if result.is_ok() {
            self.move_to(offset, false, cx);
        }
        self.edited(result, cx);
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Input fields are descendants, but their text must not edit the byte buffer.
        if !self.focus_handle.is_focused(window) {
            if let Some(tool) = self.active_tool {
                match event.keystroke.key.as_str() {
                    "escape" => {
                        self.active_tool = None;
                        self.focus_handle.focus(window, cx);
                        cx.notify();
                    }
                    "enter" => match tool {
                        Tool::Find => {
                            if self.inputs.as_ref().is_some_and(|inputs| {
                                inputs.replacement.focus_handle(cx).is_focused(window)
                            }) {
                                self.replace_match(false, cx);
                            } else {
                                self.find_match(false, cx);
                            }
                        }
                        Tool::Fill => self.fill_selection(cx),
                        Tool::Offset => self.jump_to_offset(window, cx),
                        Tool::Range => self.apply_range(window, cx),
                        Tool::Inspector => self.apply_number(cx),
                        Tool::Bookmarks => {
                            if !event.keystroke.modifiers.platform
                                && !event.keystroke.modifiers.control
                                && self.inputs.as_ref().is_some_and(|inputs| {
                                    inputs.notes.focus_handle(cx).contains_focused(window, cx)
                                })
                            {
                                return;
                            }
                            self.add_bookmark(cx);
                        }
                    },
                    _ => return,
                }
                cx.stop_propagation();
            }
            return;
        }
        let modifiers = event.keystroke.modifiers;
        if modifiers.platform || modifiers.control || modifiers.alt {
            return;
        }
        let select = modifiers.shift;
        let length = self.buffer.bytes.len();
        match event.keystroke.key.as_str() {
            "left" => self.move_to(self.cursor.saturating_sub(1), select, cx),
            "right" => self.move_to(self.cursor.saturating_add(1), select, cx),
            "up" => self.move_to(self.cursor.saturating_sub(BYTES_PER_ROW), select, cx),
            "down" => self.move_to(self.cursor.saturating_add(BYTES_PER_ROW), select, cx),
            "home" => self.move_to(0, select, cx),
            "end" => self.move_to(length, select, cx),
            "pageup" => self.move_to(self.cursor.saturating_sub(16 * BYTES_PER_ROW), select, cx),
            "pagedown" => self.move_to(self.cursor.saturating_add(16 * BYTES_PER_ROW), select, cx),
            "backspace" | "delete" if !self.busy && !self.bytes_read_only() => {
                let mut range = self.selection();
                if range.is_empty() {
                    range = if event.keystroke.key == "backspace" {
                        self.cursor.saturating_sub(1)..self.cursor
                    } else {
                        self.cursor..(self.cursor + 1).min(length)
                    };
                }
                let offset = range.start;
                let result = self.buffer.replace(range, Vec::new());
                self.move_to(offset, false, cx);
                self.edited(result, cx);
            }
            "escape" => {
                self.active_tool = None;
                self.move_to(self.cursor, false, cx);
            }
            _ => {
                let text = event
                    .keystroke
                    .key_char
                    .as_deref()
                    .unwrap_or(&event.keystroke.key);
                if self.ascii {
                    let text = if event.keystroke.key == "space" {
                        " "
                    } else {
                        text
                    };
                    if (event.keystroke.key_char.is_some()
                        || event.keystroke.key == "space"
                        || text.len() == 1)
                        && text.is_ascii()
                        && text.chars().all(|character| !character.is_control())
                    {
                        self.input_bytes(text.as_bytes().to_vec(), cx);
                        cx.stop_propagation();
                    }
                    return;
                }
                let mut characters = text.chars();
                let Some(digit) = characters
                    .next()
                    .and_then(|character| character.to_digit(16))
                else {
                    return;
                };
                if characters.next().is_some() {
                    return;
                }
                self.input_digit(digit as u8, cx);
            }
        }
        cx.stop_propagation();
    }

    fn undo(&mut self, _: &Undo, _: &mut Window, cx: &mut Context<Self>) {
        if !self.busy
            && !self.bytes_read_only()
            && let Some(offset) = self.buffer.undo()
        {
            self.move_to(offset, false, cx);
            self.diff_region = None;
            self.update_comparison(cx);
            self.edited(Ok(()), cx);
        }
    }

    fn redo(&mut self, _: &Redo, _: &mut Window, cx: &mut Context<Self>) {
        if !self.busy
            && !self.bytes_read_only()
            && let Some(offset) = self.buffer.redo()
        {
            self.move_to(offset, false, cx);
            self.diff_region = None;
            self.update_comparison(cx);
            self.edited(Ok(()), cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        let mut range = self.selection();
        if range.is_empty() {
            range.end = (range.start + 1).min(self.buffer.bytes.len());
        }
        if let Some(bytes) = self.buffer.bytes.get(range) {
            if self.ascii {
                if !bytes.is_ascii() {
                    self.error = Some("Switch to hex editing to copy non-ASCII bytes".into());
                    cx.notify();
                    return;
                }
                cx.write_to_clipboard(ClipboardItem::new_string(
                    String::from_utf8_lossy(bytes).into_owned(),
                ));
                return;
            }
            let mut text = String::with_capacity(bytes.len() * 3);
            for byte in bytes {
                if !text.is_empty() {
                    text.push(' ');
                }
                write!(&mut text, "{byte:02X}").log_err();
            }
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        }
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if self.busy || self.bytes_read_only() {
            return;
        }
        let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) else {
            return;
        };
        let bytes = if self.ascii {
            if !text.is_ascii() {
                self.error = Some("ASCII editing accepts only ASCII text".into());
                cx.notify();
                return;
            }
            Ok(text.into_bytes())
        } else {
            parse_hex(&text)
        };
        match bytes {
            Ok(bytes) => self.input_bytes(bytes, cx),
            Err(error) => {
                self.error = Some(error.to_string().into());
                cx.notify();
            }
        }
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, false, cx);
        self.move_to(self.buffer.bytes.len(), true, cx);
    }

    fn toggle_insert(&mut self, _: &ToggleInsertMode, _: &mut Window, cx: &mut Context<Self>) {
        self.insert = !self.insert;
        self.low_nibble = false;
        cx.notify();
    }

    fn toggle_ascii(&mut self, _: &ToggleAsciiMode, window: &mut Window, cx: &mut Context<Self>) {
        self.ascii = !self.ascii;
        self.low_nibble = false;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn show_tool(&mut self, tool: Tool, window: &mut Window, cx: &mut Context<Self>) {
        if self.inputs.is_none() {
            self.inputs = Some(ToolInputs {
                pattern: cx.new(|cx| InputField::new(window, cx, "Search pattern")),
                replacement: cx.new(|cx| InputField::new(window, cx, "Replace with: 42")),
                fill: cx.new(|cx| InputField::new(window, cx, "Pattern: 00 FF")),
                offset: cx.new(|cx| InputField::new(window, cx, "Offset: 0x100 or 256")),
                range_start: cx.new(|cx| InputField::new(window, cx, "Start offset")),
                range_size: cx.new(|cx| InputField::new(window, cx, "Length or exclusive end")),
                number: cx.new(|cx| InputField::new(window, cx, "Numeric value")),
                bookmark: cx.new(|cx| InputField::new(window, cx, "Bookmark name / annotation")),
                notes: cx.new(|cx| {
                    let notes = InputField::new(window, cx, "Annotation notes (optional)");
                    notes.editor().set_multiline(Some(4), window, cx);
                    notes
                }),
            });
        }
        self.active_tool = Some(tool);
        self.message = None;
        self.error = None;
        if let Some(inputs) = &self.inputs {
            let input = match tool {
                Tool::Find => &inputs.pattern,
                Tool::Fill => &inputs.fill,
                Tool::Offset => &inputs.offset,
                Tool::Range => &inputs.range_start,
                Tool::Inspector => &inputs.number,
                Tool::Bookmarks => &inputs.bookmark,
            };
            input.focus_handle(cx).focus(window, cx);
        }
        if tool == Tool::Inspector {
            self.load_number_input(window, cx);
        } else if tool == Tool::Range {
            let selection = self.selection();
            let size = if self.range_uses_end {
                selection.end
            } else {
                selection.len()
            };
            if let Some(inputs) = &self.inputs {
                inputs.range_start.update(cx, |input, cx| {
                    input.set_text(&selection.start.to_string(), window, cx)
                });
                inputs.range_size.update(cx, |input, cx| {
                    input.set_text(&size.to_string(), window, cx)
                });
            }
        }
        cx.notify();
    }

    fn find(&mut self, _: &Find, window: &mut Window, cx: &mut Context<Self>) {
        self.show_tool(Tool::Find, window, cx);
    }

    fn fill(&mut self, _: &Fill, window: &mut Window, cx: &mut Context<Self>) {
        self.show_tool(Tool::Fill, window, cx);
    }

    fn go_to_offset(&mut self, _: &GoToOffset, window: &mut Window, cx: &mut Context<Self>) {
        self.show_tool(Tool::Offset, window, cx);
    }

    fn select_range(&mut self, _: &SelectRange, window: &mut Window, cx: &mut Context<Self>) {
        self.show_tool(Tool::Range, window, cx);
    }

    fn inspect(&mut self, _: &Inspect, window: &mut Window, cx: &mut Context<Self>) {
        self.show_tool(Tool::Inspector, window, cx);
    }

    fn apply_range(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let result = (|| {
            let inputs = self
                .inputs
                .as_ref()
                .ok_or_else(|| anyhow!("Enter a byte range"))?;
            let start = parse_offset(&inputs.range_start.read(cx).text(cx))?;
            let size = parse_offset(&inputs.range_size.read(cx).text(cx))?;
            let range = byte_range(start, size, self.range_uses_end, self.buffer.bytes.len())?;
            self.move_to(range.start, false, cx);
            self.move_to(range.end, true, cx);
            self.message =
                Some(format!("Selected {} bytes at 0x{:X}", range.len(), range.start).into());
            self.focus_handle.focus(window, cx);
            Ok(())
        })();
        self.error = result
            .err()
            .map(|error: anyhow::Error| error.to_string().into());
        cx.notify();
    }

    fn inspected_bytes(&self) -> &[u8] {
        let selection = self.selection();
        let end = if selection.is_empty() {
            self.buffer.bytes.len()
        } else {
            selection.end
        };
        self.buffer
            .bytes
            .get(selection.start..end)
            .unwrap_or_default()
    }

    fn load_number_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let value = self
            .number_type
            .decode(self.inspected_bytes(), self.endian)
            .unwrap_or_default();
        if let Some(inputs) = &self.inputs {
            inputs
                .number
                .update(cx, |input, cx| input.set_text(&value, window, cx));
        }
        self.number_source = Some(self.number_source());
        cx.notify();
    }

    fn number_source(&self) -> (usize, usize, u64, NumberType, Endian) {
        let selection = self.selection();
        (
            selection.start,
            selection.end,
            self.buffer.revision,
            self.number_type,
            self.endian,
        )
    }

    fn apply_number(&mut self, cx: &mut Context<Self>) {
        if self.busy || self.bytes_read_only() {
            return;
        }
        let result = (|| {
            ensure!(
                self.inspected_bytes().len() >= self.number_type.width(),
                "Not enough bytes for this numeric type"
            );
            let inputs = self
                .inputs
                .as_ref()
                .ok_or_else(|| anyhow!("Enter a numeric value"))?;
            let bytes = self
                .number_type
                .encode(&inputs.number.read(cx).text(cx), self.endian)?;
            let offset = self.selection().start;
            self.buffer.replace(offset..offset + bytes.len(), bytes)?;
            self.low_nibble = false;
            self.message =
                Some(format!("Updated {} at 0x{offset:X}", self.number_type.label()).into());
            Ok(())
        })();
        self.edited(result, cx);
    }

    fn toggle_bit(&mut self, bit: u8, window: &mut Window, cx: &mut Context<Self>) {
        if self.busy || self.bytes_read_only() || bit >= 8 {
            return;
        }
        let offset = self.selection().start;
        if let Some(byte) = self.buffer.bytes.get(offset) {
            let result = self
                .buffer
                .replace(offset..offset + 1, vec![byte ^ (1 << bit)]);
            self.low_nibble = false;
            self.edited(result, cx);
            self.load_number_input(window, cx);
        }
    }

    fn pattern(&self, cx: &App) -> Result<SearchPattern> {
        let inputs = self
            .inputs
            .as_ref()
            .ok_or_else(|| anyhow!("Enter bytes to find"))?;
        self.search_mode.pattern(
            &inputs.pattern.read(cx).text(cx),
            self.number_type,
            self.endian,
        )
    }

    fn find_match(&mut self, backwards: bool, cx: &mut Context<Self>) {
        let result = self.pattern(cx).and_then(|pattern| {
            let selection = self.selection();
            let start = if backwards {
                selection.start
            } else if !selection.is_empty() {
                selection.start.saturating_add(1)
            } else {
                self.cursor
            };
            let offset = pattern
                .find(&self.buffer.bytes, start, backwards)?
                .ok_or_else(|| anyhow!("Byte pattern not found"))?;
            self.move_to(offset, false, cx);
            self.move_to(offset + pattern.length(), true, cx);
            self.message = Some(format!("Match at 0x{offset:X}").into());
            Ok(())
        });
        self.error = result.err().map(|error| error.to_string().into());
        cx.notify();
    }

    fn find_next(&mut self, _: &FindNext, window: &mut Window, cx: &mut Context<Self>) {
        if self.inputs.is_none() {
            self.show_tool(Tool::Find, window, cx);
        } else {
            self.find_match(false, cx);
        }
    }

    fn find_previous(&mut self, _: &FindPrevious, window: &mut Window, cx: &mut Context<Self>) {
        if self.inputs.is_none() {
            self.show_tool(Tool::Find, window, cx);
        } else {
            self.find_match(true, cx);
        }
    }

    fn replace_match(&mut self, all: bool, cx: &mut Context<Self>) {
        if self.busy || self.bytes_read_only() {
            return;
        }
        let result = self.pattern(cx).and_then(|pattern| {
            let inputs = self
                .inputs
                .as_ref()
                .ok_or_else(|| anyhow!("Enter replacement bytes"))?;
            let replacement = self.search_mode.replacement(
                &inputs.replacement.read(cx).text(cx),
                self.number_type,
                self.endian,
            )?;
            if all {
                let count = self.buffer.replace_pattern(&pattern, &replacement)?;
                self.move_to(self.cursor, false, cx);
                self.message = Some(format!("Replaced {count} matches").into());
            } else {
                let range = self.selection();
                ensure!(
                    self.buffer
                        .bytes
                        .get(range.clone())
                        .is_some_and(|bytes| pattern.matches(bytes)),
                    "Find and select a matching byte pattern before replacing"
                );
                let offset = range.start + replacement.len();
                self.buffer.replace(range, replacement)?;
                self.move_to(offset, false, cx);
                self.message = Some("Replaced 1 match".into());
            }
            Ok(())
        });
        self.edited(result, cx);
    }

    fn fill_selection(&mut self, cx: &mut Context<Self>) {
        if self.busy || self.bytes_read_only() {
            return;
        }
        let result = (|| {
            let range = self.selection();
            ensure!(!range.is_empty(), "Select bytes to fill");
            let inputs = self
                .inputs
                .as_ref()
                .ok_or_else(|| anyhow!("Enter a fill pattern"))?;
            let pattern = parse_hex(&inputs.fill.read(cx).text(cx))?;
            ensure!(!pattern.is_empty(), "Enter a fill pattern");
            let bytes = pattern.iter().copied().cycle().take(range.len()).collect();
            self.buffer.replace(range, bytes)?;
            self.low_nibble = false;
            self.message = Some("Filled selected bytes".into());
            Ok(())
        })();
        self.edited(result, cx);
    }

    fn jump_to_offset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let result = (|| {
            let inputs = self
                .inputs
                .as_ref()
                .ok_or_else(|| anyhow!("Enter an offset"))?;
            let offset = parse_offset(&inputs.offset.read(cx).text(cx))?;
            ensure!(
                offset <= self.buffer.bytes.len(),
                "Offset is beyond the end of the file"
            );
            self.move_to(offset, false, cx);
            self.focus_handle.focus(window, cx);
            self.message = Some(format!("Offset 0x{offset:X}").into());
            Ok(())
        })();
        self.error = result
            .err()
            .map(|error: anyhow::Error| error.to_string().into());
        cx.notify();
    }

    fn toggle_read_only(&mut self, _: &ToggleReadOnly, _: &mut Window, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        if !self.read_only && self.buffer.is_dirty() {
            self.error = Some("Save or undo pending edits before enabling read-only mode".into());
        } else {
            self.read_only = !self.read_only;
            self.low_nibble = false;
            self.error = None;
        }
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    fn show_bookmarks(&mut self, _: &Bookmarks, window: &mut Window, cx: &mut Context<Self>) {
        self.show_tool(Tool::Bookmarks, window, cx);
    }

    fn annotations(&mut self, _: &Annotations, window: &mut Window, cx: &mut Context<Self>) {
        self.show_tool(Tool::Bookmarks, window, cx);
    }

    fn save_annotation(&mut self, _: &SaveAnnotation, _: &mut Window, cx: &mut Context<Self>) {
        if self.active_tool == Some(Tool::Bookmarks) {
            self.add_bookmark(cx);
        }
    }

    fn new_bookmark(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editing_bookmark = None;
        if let Some(inputs) = &self.inputs {
            inputs
                .bookmark
                .update(cx, |input, cx| input.clear(window, cx));
            inputs.notes.update(cx, |input, cx| input.clear(window, cx));
            inputs.bookmark.focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }

    fn edit_bookmark(&mut self, id: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(bookmark) = self.bookmarks.iter().find(|bookmark| bookmark.id == id) else {
            return;
        };
        self.editing_bookmark = Some(id);
        if let Some(inputs) = &self.inputs {
            inputs
                .bookmark
                .update(cx, |input, cx| input.set_text(&bookmark.name, window, cx));
            inputs
                .notes
                .update(cx, |input, cx| input.set_text(&bookmark.notes, window, cx));
            inputs.bookmark.focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }

    fn remove_bookmark(&mut self, id: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.bookmarks.retain(|bookmark| bookmark.id != id);
        if self.editing_bookmark == Some(id) {
            self.new_bookmark(window, cx);
        }
        self.persist_metadata(cx);
        cx.notify();
    }

    fn retarget_bookmark(&mut self, id: usize, cx: &mut Context<Self>) {
        let selection = self.selection();
        if let Some(bookmark) = self.bookmarks.iter_mut().find(|bookmark| bookmark.id == id) {
            bookmark.retarget(selection, &self.buffer.changes);
            self.persist_metadata(cx);
            cx.notify();
        }
    }

    fn metadata_snapshot(&self) -> Metadata {
        Metadata::capture(
            &self.bookmarks,
            &self.buffer.changes,
            self.buffer.saved_bytes.len(),
            &self.saved_fingerprint,
        )
    }

    fn persist_metadata(&mut self, cx: &mut Context<Self>) {
        let json = match serde_json::to_string(&self.metadata_snapshot()) {
            Ok(json) => json,
            Err(error) => {
                self.metadata_error =
                    Some(format!("Could not serialize annotations: {error}").into());
                cx.notify();
                return;
            }
        };
        let key = bookmarks::metadata_key(&self.path);
        cx.update_global::<MetadataCache, _>(|cache, _| {
            cache.snapshots.insert(key.clone(), json.clone());
        });
        let previous = cx.global::<MetadataCache>().writes.get(&key).cloned();
        let previous_for_view = self.metadata_write.clone();
        let write_key = key.clone();
        let db = db::kvp::KeyValueStore::global(cx);
        let task = cx
            .background_spawn(async move {
                if let Some(previous) = previous_for_view {
                    previous.await.log_err();
                }
                if let Some(previous) = previous {
                    previous.await.log_err();
                }
                db.scoped(METADATA_NAMESPACE)
                    .write(key, json)
                    .await
                    .map_err(|error| error.to_string())
            })
            .shared();
        cx.update_global::<MetadataCache, _>(|cache, _| {
            cache.writes.insert(write_key, task.clone());
        });
        self.metadata_write = Some(task.clone());
        self.metadata_generation += 1;
        let generation = self.metadata_generation;
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                if generation == this.metadata_generation {
                    this.metadata_error = result
                        .err()
                        .map(|error| format!("Could not save annotations: {error}").into());
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn add_bookmark(&mut self, cx: &mut Context<Self>) {
        let result = (|| {
            ensure!(
                self.editing_bookmark.is_some() || self.bookmarks.len() < MAX_BOOKMARKS,
                "Keep at most 256 bookmarks per tab"
            );
            let inputs = self
                .inputs
                .as_ref()
                .ok_or_else(|| anyhow!("Enter a bookmark name"))?;
            let name = inputs.bookmark.read(cx).text(cx).trim().to_owned();
            let notes = inputs.notes.read(cx).text(cx);
            bookmarks::validate(&name, &notes)?;
            if let Some(id) = self.editing_bookmark {
                let bookmark = self
                    .bookmarks
                    .iter_mut()
                    .find(|bookmark| bookmark.id == id)
                    .ok_or_else(|| anyhow!("The annotation was removed"))?;
                bookmark.name = name;
                bookmark.notes = notes;
            } else {
                let mut bookmark = Bookmark::new(
                    self.next_bookmark_id,
                    name,
                    self.selection(),
                    &self.buffer.changes,
                );
                bookmark.notes = notes;
                self.bookmarks.push(bookmark);
                self.next_bookmark_id += 1;
            }
            self.persist_metadata(cx);
            self.error = None;
            Ok(())
        })();
        self.error = result
            .err()
            .map(|error: anyhow::Error| error.to_string().into());
        cx.notify();
    }

    fn jump_to_bookmark(&mut self, id: usize, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(range) = self
            .bookmarks
            .iter()
            .find(|bookmark| bookmark.id == id)
            .and_then(|bookmark| bookmark.range(&self.buffer.changes, self.buffer.bytes.len()))
        {
            self.move_to(range.start, false, cx);
            self.move_to(range.end, true, cx);
            self.focus_handle.focus(window, cx);
        }
    }

    fn compare_files(&mut self, _: &CompareFiles, window: &mut Window, cx: &mut Context<Self>) {
        let paths = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Choose a binary file to compare".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            let result: Result<()> = async {
                if let Some(path) = paths.await??.and_then(|paths| paths.into_iter().next()) {
                    this.update(cx, |this, cx| this.compare_path(path, cx).detach())?;
                }
                Ok(())
            }
            .await;
            if let Err(error) = result {
                this.update(cx, |this, cx| {
                    this.error = Some(error.to_string().into());
                    cx.notify();
                })
                .log_err();
            }
        })
        .detach();
    }

    pub fn compare_path(&mut self, path: PathBuf, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.comparison_generation += 1;
        let generation = self.comparison_generation;
        let fs = self.project.read(cx).fs().clone();
        let load = cx.background_spawn({
            let path = path.clone();
            async move { load_file(fs.as_ref(), &path).await }
        });
        let task = cx
            .spawn(async move |this, cx| {
                let result = load.await;
                let error = result.as_ref().err().map(|error| error.to_string());
                this.update(cx, |this, cx| {
                    if generation != this.comparison_generation {
                        return;
                    }
                    this.comparison_load = None;
                    match result {
                        Ok(bytes) => {
                            this.comparison = Some(Comparison {
                                path,
                                bytes: bytes.into(),
                                stats: None,
                            });
                            this.diff_region = None;
                            this.error = None;
                            this.update_comparison(cx);
                            cx.emit(ItemEvent::UpdateTab);
                        }
                        Err(error) => {
                            this.error = Some(error.to_string().into());
                            this.update_comparison(cx);
                        }
                    }
                    cx.notify();
                })
                .map_err(|error| error.to_string())?;
                error.map_or(Ok(()), Err)
            })
            .shared();
        self.comparison_load = Some(task.clone());
        cx.notify();
        cx.spawn(async move |_, _| task.await.map_err(anyhow::Error::msg))
    }

    fn update_comparison(&mut self, cx: &mut Context<Self>) {
        let Some(comparison) = &self.comparison else {
            return;
        };
        let reference = comparison.bytes.clone();
        let bytes = self.buffer.bytes.clone();
        let revision = self.buffer.revision;
        let generation = self.comparison_generation;
        let calculate = cx.background_spawn(async move { comparison::stats(&bytes, &reference) });
        self.comparison_stats_task = Some(cx.spawn(async move |this, cx| {
            let stats = calculate.await;
            this.update(cx, |this, cx| {
                if generation == this.comparison_generation && revision == this.buffer.revision {
                    if let Some(comparison) = &mut this.comparison {
                        comparison.stats = Some((revision, stats));
                    }
                    cx.notify();
                }
            })
            .log_err();
        }));
    }

    fn close_comparison(
        &mut self,
        _: &CloseComparison,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.comparison_generation += 1;
        self.comparison_load = None;
        self.comparison_stats_task = None;
        self.comparison = None;
        self.diff_region = None;
        self.focus_handle.focus(window, cx);
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    fn next_difference(&mut self, _: &NextDifference, window: &mut Window, cx: &mut Context<Self>) {
        self.find_difference(false, window, cx);
    }

    fn previous_difference(
        &mut self,
        _: &PreviousDifference,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.find_difference(true, window, cx);
    }

    fn find_difference(&mut self, backwards: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(comparison) = &self.comparison else {
            return;
        };
        let start = self
            .diff_region
            .as_ref()
            .map(|range| if backwards { range.start } else { range.end })
            .unwrap_or(if backwards {
                self.selection().start
            } else {
                self.cursor
            });
        if let Some(range) =
            comparison::find_region(&self.buffer.bytes, &comparison.bytes, start, backwards)
        {
            self.move_to(range.start, false, cx);
            self.move_to(range.end, true, cx);
            self.scroll_handle
                .scroll_to_item(range.start / BYTES_PER_ROW, ScrollStrategy::Nearest);
            self.message =
                Some(format!("Difference 0x{:X}..0x{:X}", range.start, range.end).into());
            self.diff_region = Some(range);
        } else {
            self.message = Some("The files are identical".into());
        }
        self.focus_handle.focus(window, cx);
        cx.notify();
    }
}

fn byte_range(start: usize, size: usize, uses_end: bool, length: usize) -> Result<Range<usize>> {
    let end = if uses_end {
        size
    } else {
        start
            .checked_add(size)
            .ok_or_else(|| anyhow!("Byte range is too large"))?
    };
    ensure!(start <= end, "End offset must not precede start offset");
    ensure!(end <= length, "Byte range is beyond the end of the file");
    Ok(start..end)
}

fn parse_offset(text: &str) -> Result<usize> {
    let text = text.trim();
    let offset = if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        usize::from_str_radix(hex, 16)
    } else {
        text.parse()
    };
    offset.map_err(|_| anyhow!("Enter a decimal offset or hexadecimal offset prefixed with 0x"))
}

fn find_bytes(
    bytes: &[u8],
    pattern: &[u8],
    start: usize,
    backwards: bool,
) -> Result<Option<usize>> {
    if pattern.is_empty() || pattern.len() > bytes.len() {
        return Ok(None);
    }
    let end = bytes.len() - pattern.len() + 1;
    let start = start.min(end);
    let matcher = aho_corasick::AhoCorasick::new([pattern])?;
    let offset = if backwards {
        matcher
            .find_overlapping_iter(&bytes[..start + pattern.len() - 1])
            .last()
            .or_else(|| matcher.find_overlapping_iter(bytes).last())
    } else {
        return Ok(matcher
            .find(&bytes[start..])
            .map(|found| start + found.start())
            .or_else(|| matcher.find(bytes).map(|found| found.start())));
    };
    Ok(offset.map(|found| found.start()))
}

fn parse_hex(text: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut high_nibble = None;
    for character in text
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
    {
        let digit = character
            .to_digit(16)
            .ok_or_else(|| anyhow!("Paste hexadecimal bytes, for example: 00 FF 42"))?
            as u8;
        if let Some(high) = high_nibble.take() {
            ensure!(bytes.len() < MAX_FILE_SIZE, "Paste at most 64 MiB of bytes");
            bytes.push((high << 4) | digit);
        } else {
            high_nibble = Some(digit);
        }
    }
    ensure!(
        high_nibble.is_none(),
        "Each byte needs two hexadecimal digits"
    );
    Ok(bytes)
}

impl Focusable for HexEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<ItemEvent> for HexEditor {}

impl Item for HexEditor {
    type Event = ItemEvent;

    fn capability(&self, _: &App) -> language::Capability {
        if self.bytes_read_only() {
            language::Capability::Read
        } else {
            language::Capability::ReadWrite
        }
    }

    fn toggle_read_only(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        HexEditor::toggle_read_only(self, &ToggleReadOnly, window, cx);
    }

    fn tab_content_text(&self, _: usize, _: &App) -> SharedString {
        format!(
            "{} (Hex)",
            self.path.file_name().unwrap_or_default().to_string_lossy()
        )
        .into()
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some(self.path.to_string_lossy().into_owned().into())
    }

    fn active_project_path(&self, cx: &App) -> Option<project::ProjectPath> {
        self.project.read(cx).find_project_path(&self.path, cx)
    }

    fn buffer_kind(&self, _: &App) -> ItemBufferKind {
        ItemBufferKind::Singleton
    }

    fn to_item_events(event: &Self::Event, emit: &mut dyn FnMut(ItemEvent)) {
        emit(*event);
    }

    fn is_dirty(&self, _: &App) -> bool {
        self.buffer.is_dirty()
    }

    fn can_save(&self, _: &App) -> bool {
        !self.bytes_read_only()
    }

    fn can_save_as(&self, _: &App) -> bool {
        true
    }

    fn save_as(
        &mut self,
        project: Entity<project::Project>,
        path: project::ProjectPath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.busy {
            return Task::ready(Err(anyhow!("A file operation is already in progress")));
        }
        if !project.read(cx).is_local() {
            return Task::ready(Err(anyhow!("Save As supports only local files")));
        }
        let Some(path) = project.read(cx).absolute_path(&path, cx) else {
            return Task::ready(Err(anyhow!("Cannot resolve the new file path")));
        };
        if path == self.path {
            return Task::ready(Err(anyhow!("Choose a new file path for Save As")));
        }
        let fs = project.read(cx).fs().clone();
        let bytes = self.buffer.bytes.clone();
        self.persist_metadata(cx);
        self.busy = true;
        cx.notify();
        let write = cx.background_spawn({
            let path = path.clone();
            async move {
                ensure!(
                    fs.metadata(&path).await?.is_none(),
                    "Choose a new file path. Hex Save As does not overwrite existing files"
                );
                fs.create_file(&path, fs::CreateOptions::default()).await?;
                fs.write(&path, &bytes).await?;
                anyhow::Ok(())
            }
        });
        cx.spawn(async move |this, cx| {
            let result = write.await;
            this.update(cx, |this, cx| {
                this.busy = false;
                if result.is_ok() {
                    this.path = path;
                    this.project = project;
                    this.buffer.mark_saved();
                    this.saved_fingerprint = bookmarks::fingerprint(&this.buffer.saved_bytes);
                    this.persist_metadata(cx);
                    this.error = None;
                } else if let Err(error) = &result {
                    this.error = Some(error.to_string().into());
                }
                cx.emit(ItemEvent::UpdateTab);
                cx.emit(ItemEvent::UpdateBreadcrumbs);
                cx.notify();
            })?;
            result
        })
    }

    fn save(
        &mut self,
        _: SaveOptions,
        _: Entity<project::Project>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.bytes_read_only() {
            return Task::ready(Err(anyhow!(
                "Disable read-only mode or close the comparison before saving this file"
            )));
        }
        if self.busy {
            return Task::ready(Err(anyhow!("A file operation is already in progress")));
        }
        let fs = self.project.read(cx).fs().clone();
        let path = self.path.clone();
        let bytes = self.buffer.bytes.clone();
        let saved_bytes = self.buffer.saved_bytes.clone();
        let revision = self.buffer.revision;
        self.busy = true;
        cx.notify();
        let write = cx.background_spawn(async move {
            ensure!(
                load_file(fs.as_ref(), &path).await? == saved_bytes,
                "The file changed on disk. Reload it before saving to avoid losing changes"
            );
            fs.write(&path, &bytes).await?;
            Ok(bytes)
        });
        cx.spawn(async move |this, cx| {
            let result: Result<Vec<u8>> = write.await;
            this.update(cx, |this, cx| {
                this.busy = false;
                if let Ok(bytes) = &result {
                    debug_assert_eq!(this.buffer.revision, revision);
                    debug_assert_eq!(&this.buffer.bytes, bytes);
                    this.buffer.mark_saved();
                    this.saved_fingerprint = bookmarks::fingerprint(&this.buffer.saved_bytes);
                    this.persist_metadata(cx);
                    this.error = None;
                } else if let Err(error) = &result {
                    this.error = Some(error.to_string().into());
                }
                cx.emit(ItemEvent::UpdateTab);
                cx.notify();
            })?;
            result.map(|_| ())
        })
    }

    fn reload(
        &mut self,
        _: Entity<project::Project>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.busy {
            return Task::ready(Err(anyhow!("A file operation is already in progress")));
        }
        let fs = self.project.read(cx).fs().clone();
        let path = self.path.clone();
        self.busy = true;
        cx.notify();
        let load = cx.background_spawn(async move { load_file(fs.as_ref(), &path).await });
        cx.spawn(async move |this, cx| {
            let result: Result<Vec<u8>> = load.await;
            this.update(cx, |this, cx| {
                this.busy = false;
                if let Ok(bytes) = &result {
                    let metadata = this.metadata_snapshot();
                    this.buffer = HexBuffer::new(bytes.clone());
                    this.saved_fingerprint = bookmarks::fingerprint(bytes);
                    this.bookmarks = metadata.restore(
                        &this.buffer.changes,
                        bytes.len(),
                        &this.saved_fingerprint,
                    );
                    this.next_bookmark_id = this.bookmarks.len();
                    this.editing_bookmark = None;
                    this.move_to(this.cursor, false, cx);
                    this.update_comparison(cx);
                    this.error = None;
                } else if let Err(error) = &result {
                    this.error = Some(error.to_string().into());
                }
                cx.emit(ItemEvent::UpdateTab);
                cx.notify();
            })?;
            result.map(|_| ())
        })
    }
}

impl Render for HexEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.active_tool == Some(Tool::Inspector)
            && self.number_source != Some(self.number_source())
        {
            self.load_number_input(window, cx);
        }
        let font = ThemeSettings::get_global(cx).buffer_font.family.clone();
        let comparison_length = self
            .comparison
            .as_ref()
            .map(|comparison| comparison.bytes.len())
            .unwrap_or(0);
        let rows = self.buffer.bytes.len().max(comparison_length) / BYTES_PER_ROW + 1;
        let list = uniform_list(
            "hex-editor-bytes",
            rows,
            cx.processor(|this, range: Range<usize>, _, cx| {
                let selection = this.selection();
                let annotations: Vec<_> = this
                    .bookmarks
                    .iter()
                    .filter_map(|bookmark| {
                        bookmark
                            .range(&this.buffer.changes, this.buffer.bytes.len())
                            .map(|range| (range, bookmark))
                    })
                    .collect();
                range
                    .map(|row| {
                        let offset = row * BYTES_PER_ROW;
                        let mut hex = h_flex().gap_1();
                        let mut ascii = h_flex();
                        let mut reference_hex = h_flex().gap_1();
                        let mut reference_ascii = h_flex();
                        for column in 0..BYTES_PER_ROW {
                            let address = offset + column;
                            let byte = this.buffer.bytes.get(address).copied();
                            let change = this.buffer.byte_change(address);
                            let change_color = match change {
                                Some(ByteChange::Inserted) => {
                                    Some(cx.theme().status().created_background.opacity(0.2))
                                }
                                Some(ByteChange::Modified) => {
                                    Some(cx.theme().status().modified_background.opacity(0.2))
                                }
                                None => None,
                            };
                            let deleted = this.buffer.changes.deletions.get(&address).copied();
                            let difference = this.comparison.as_ref().and_then(|comparison| {
                                comparison::difference(
                                    &this.buffer.bytes,
                                    &comparison.bytes,
                                    address,
                                )
                            });
                            let diff_color = match difference {
                                Some(Difference::Changed) => {
                                    Some(cx.theme().status().modified_background.opacity(0.3))
                                }
                                Some(Difference::CurrentOnly) => {
                                    Some(cx.theme().status().created_background.opacity(0.3))
                                }
                                Some(Difference::ReferenceOnly) => {
                                    Some(cx.theme().status().deleted_background.opacity(0.3))
                                }
                                None => None,
                            };
                            let annotation = annotations
                                .iter()
                                .filter(|(range, _)| {
                                    range.contains(&address)
                                        || (range.is_empty() && range.start == address)
                                })
                                .take(4)
                                .map(|(_, bookmark)| {
                                    if bookmark.notes.is_empty() {
                                        bookmark.name.clone()
                                    } else {
                                        let mut preview: String =
                                            bookmark.notes.chars().take(512).collect();
                                        if preview.len() < bookmark.notes.len() {
                                            preview.push_str("\n… Edit to read the full note");
                                        }
                                        format!("{}\n{preview}", bookmark.name)
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join("\n\n");
                            let annotation = (!annotation.is_empty()).then_some(annotation);
                            let selected = selection.contains(&address)
                                || this
                                    .diff_region
                                    .as_ref()
                                    .is_some_and(|range| range.contains(&address));
                            let cursor = selection.is_empty() && this.cursor == address;
                            let text = byte
                                .map(|byte| format!("{byte:02X}"))
                                .unwrap_or_else(|| "  ".into());
                            let character = match byte {
                                Some(byte) if byte.is_ascii_graphic() || byte == b' ' => {
                                    byte as char
                                }
                                Some(_) => '.',
                                None => ' ',
                            };
                            ascii = ascii.child(
                                div()
                                    .id(("ascii-byte", address))
                                    .w(px(10.))
                                    .when_some(change_color, |cell, color| cell.bg(color))
                                    .when_some(diff_color, |cell, color| cell.bg(color))
                                    .when_some(annotation.clone(), |cell, text| {
                                        cell.border_b_1()
                                            .border_color(cx.theme().colors().text_accent)
                                            .tooltip(ui::Tooltip::text(text))
                                    })
                                    .when(deleted.is_some(), |cell| {
                                        cell.border_l_2().border_color(cx.theme().status().deleted)
                                    })
                                    .when(selected, |cell| {
                                        cell.bg(cx.theme().colors().element_selection_background)
                                    })
                                    .when(cursor && this.ascii, |cell| {
                                        cell.bg(cx.theme().colors().element_selected)
                                            .border_b_2()
                                            .border_color(cx.theme().colors().text_accent)
                                    })
                                    .child(character.to_string())
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(
                                            move |this,
                                                  event: &gpui::MouseDownEvent,
                                                  window,
                                                  cx| {
                                                this.ascii = true;
                                                this.focus_handle.focus(window, cx);
                                                this.move_to(address, event.modifiers.shift, cx);
                                            },
                                        ),
                                    ),
                            );
                            hex = hex.child(
                                div()
                                    .id(("byte", address))
                                    .w(px(24.))
                                    .when_some(change_color, |cell, color| cell.bg(color))
                                    .when_some(diff_color, |cell, color| cell.bg(color))
                                    .when_some(annotation, |cell, text| {
                                        cell.border_b_1()
                                            .border_color(cx.theme().colors().text_accent)
                                            .tooltip(ui::Tooltip::text(text))
                                    })
                                    .when(deleted.is_some(), |cell| {
                                        cell.border_l_2().border_color(cx.theme().status().deleted)
                                    })
                                    .when(selected, |cell| {
                                        cell.bg(cx.theme().colors().element_selection_background)
                                    })
                                    .when(cursor && !this.ascii, |cell| {
                                        cell.bg(cx.theme().colors().element_selected)
                                            .border_b_2()
                                            .border_color(cx.theme().colors().text_accent)
                                    })
                                    .child(text)
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(
                                            move |this,
                                                  event: &gpui::MouseDownEvent,
                                                  window,
                                                  cx| {
                                                this.focus_handle.focus(window, cx);
                                                this.ascii = false;
                                                this.move_to(address, event.modifiers.shift, cx);
                                            },
                                        ),
                                    ),
                            );
                            if let Some(comparison) = &this.comparison {
                                let byte = comparison.bytes.get(address).copied();
                                reference_hex = reference_hex.child(
                                    div()
                                        .w(px(24.))
                                        .when_some(diff_color, |cell, color| cell.bg(color))
                                        .when(selected, |cell| {
                                            cell.bg(cx
                                                .theme()
                                                .colors()
                                                .element_selection_background)
                                        })
                                        .child(
                                            byte.map(|byte| format!("{byte:02X}"))
                                                .unwrap_or_else(|| "  ".into()),
                                        ),
                                );
                                reference_ascii = reference_ascii.child(
                                    div()
                                        .w(px(10.))
                                        .when_some(diff_color, |cell, color| cell.bg(color))
                                        .when(selected, |cell| {
                                            cell.bg(cx
                                                .theme()
                                                .colors()
                                                .element_selection_background)
                                        })
                                        .child(
                                            byte.map(|byte| {
                                                if byte.is_ascii_graphic() || byte == b' ' {
                                                    byte as char
                                                } else {
                                                    '.'
                                                }
                                            })
                                            .unwrap_or(' ')
                                            .to_string(),
                                        ),
                                );
                            }
                        }
                        h_flex()
                            .h(px(24.))
                            .gap_4()
                            .child(
                                div()
                                    .w(px(80.))
                                    .text_color(cx.theme().colors().text_muted)
                                    .child(format!("{offset:08X}")),
                            )
                            .child(hex)
                            .child(
                                div()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child(ascii),
                            )
                            .when(this.comparison.is_some(), |row| {
                                row.child(div().w(px(16.))).child(reference_hex).child(
                                    div()
                                        .text_color(cx.theme().colors().text_muted)
                                        .child(reference_ascii),
                                )
                            })
                    })
                    .collect::<Vec<_>>()
            }),
        )
        .track_scroll(&self.scroll_handle)
        .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
        .min_h_0()
        .flex_1();
        let status = self.error.clone().unwrap_or_else(|| {
            format!(
                "{} bytes | Offset {:08X} | {} selected | {} | {}",
                self.buffer.bytes.len(),
                self.cursor,
                self.selection().len(),
                if self.ascii {
                    "ASCII"
                } else if self.low_nibble {
                    "Low nibble"
                } else {
                    "High nibble"
                },
                if self.busy {
                    "Loading/Saving"
                } else if self.comparison.is_some() {
                    "Compare (Read Only)"
                } else if self.read_only {
                    "Read Only"
                } else if self.insert {
                    "Insert"
                } else {
                    "Overwrite"
                }
            )
            .into()
        });
        v_flex()
            .id("hex-editor")
            .size_full()
            .p_2()
            .gap_2()
            .overflow_hidden()
            .font_family(font)
            .text_sm()
            .bg(cx.theme().colors().editor_background)
            .text_color(cx.theme().colors().text)
            .key_context("HexEditor")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::key_down))
            .on_action(cx.listener(Self::undo))
            .on_action(cx.listener(Self::redo))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::toggle_insert))
            .on_action(cx.listener(Self::toggle_ascii))
            .on_action(cx.listener(Self::find))
            .on_action(cx.listener(Self::find_next))
            .on_action(cx.listener(Self::find_previous))
            .on_action(cx.listener(Self::fill))
            .on_action(cx.listener(Self::go_to_offset))
            .on_action(cx.listener(Self::select_range))
            .on_action(cx.listener(Self::inspect))
            .on_action(cx.listener(Self::toggle_read_only))
            .on_action(cx.listener(Self::show_bookmarks))
            .on_action(cx.listener(Self::annotations))
            .on_action(cx.listener(Self::save_annotation))
            .on_action(cx.listener(Self::compare_files))
            .on_action(cx.listener(Self::next_difference))
            .on_action(cx.listener(Self::previous_difference))
            .on_action(cx.listener(Self::close_comparison))
            .child(
                h_flex().flex_wrap()
                    .gap_2()
                    .child(Button::new("hex-find", "Find / Replace").on_click(
                        cx.listener(|this, _, window, cx| this.find(&Find, window, cx)),
                    ))
                    .child(Button::new("hex-fill", "Fill Selection").on_click(
                        cx.listener(|this, _, window, cx| this.fill(&Fill, window, cx)),
                    ))
                    .child(Button::new("hex-offset", "Go to Offset").on_click(
                        cx.listener(|this, _, window, cx| this.go_to_offset(&GoToOffset, window, cx)),
                    ))
                    .child(Button::new("hex-select-range", "Select Range").on_click(
                        cx.listener(|this, _, window, cx| this.select_range(&SelectRange, window, cx)),
                    ))
                    .child(Button::new("hex-inspect", "Inspect").on_click(
                        cx.listener(|this, _, window, cx| this.inspect(&Inspect, window, cx)),
                    ))
                    .child(Button::new("hex-bookmarks", "Bookmarks").on_click(
                        cx.listener(|this, _, window, cx| this.show_bookmarks(&Bookmarks, window, cx)),
                    ))
                    .child(Button::new("hex-save-as", "Save As").disabled(self.busy).on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(SaveAs), cx);
                    }))
                    .child(Button::new("hex-read-only", if self.bytes_read_only() { "Read Only" } else { "Read / Write" })
                        .disabled(self.comparison.is_some())
                        .toggle_state(self.read_only).on_click(
                            cx.listener(|this, _, window, cx| this.toggle_read_only(&ToggleReadOnly, window, cx)),
                        ))
                    .child(Button::new("hex-ascii-mode", if self.ascii { "Edit ASCII" } else { "Edit Hex" }).on_click(
                        cx.listener(|this, _, window, cx| this.toggle_ascii(&ToggleAsciiMode, window, cx)),
                    ))
                    .child(Button::new("hex-compare", "Compare").on_click(
                        cx.listener(|this, _, window, cx| this.compare_files(&CompareFiles, window, cx)),
                    )),
            )
            .when_some(self.active_tool, |root, tool| {
                let Some(inputs) = &self.inputs else { return root };
                let controls = match tool {
                    Tool::Find => v_flex()
                        .gap_2()
                        .child(h_flex().gap_2().flex_wrap()
                            .children(SearchMode::ALL.into_iter().map(|mode| {
                                Button::new(mode.label(), mode.label()).toggle_state(self.search_mode == mode)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.search_mode = mode;
                                        this.message = None;
                                        this.error = None;
                                        cx.notify();
                                    }))
                            }))
                            .when(self.search_mode == SearchMode::Number, |row| row
                                .child(Button::new("hex-search-number-type", self.number_type.label()).on_click(
                                    cx.listener(|this, _, _, cx| {
                                        let index = NumberType::ALL.iter().position(|kind| *kind == this.number_type).unwrap_or(0);
                                        this.number_type = NumberType::ALL[(index + 1) % NumberType::ALL.len()];
                                        cx.notify();
                                    }),
                                ))
                                .child(Button::new("hex-search-endian", if self.endian == Endian::Little { "Little Endian" } else { "Big Endian" })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.endian = if this.endian == Endian::Little { Endian::Big } else { Endian::Little };
                                        cx.notify();
                                    })))))
                        .child(h_flex().gap_2().child(inputs.pattern.clone())
                            .child(Button::new("hex-find-previous", "Previous").on_click(
                                cx.listener(|this, _, _, cx| this.find_match(true, cx)),
                            ))
                            .child(Button::new("hex-find-next", "Next").on_click(
                                cx.listener(|this, _, _, cx| this.find_match(false, cx)),
                            )))
                        .child(h_flex().gap_2().child(inputs.replacement.clone())
                            .child(Button::new("hex-replace", "Replace").disabled(self.busy || self.bytes_read_only()).on_click(
                                cx.listener(|this, _, _, cx| this.replace_match(false, cx)),
                            ))
                            .child(Button::new("hex-replace-all", "Replace All").disabled(self.busy || self.bytes_read_only()).on_click(
                                cx.listener(|this, _, _, cx| this.replace_match(true, cx)),
                            ))),
                    Tool::Fill => v_flex().child(h_flex().gap_2().child(inputs.fill.clone())
                        .child(Button::new("hex-apply-fill", "Fill Selection")
                            .disabled(self.busy || self.bytes_read_only() || self.selection().is_empty())
                            .on_click(cx.listener(|this, _, _, cx| this.fill_selection(cx))))),
                    Tool::Offset => v_flex().child(h_flex().gap_2().child(inputs.offset.clone())
                        .child(Button::new("hex-jump", "Go").on_click(
                            cx.listener(|this, _, window, cx| this.jump_to_offset(window, cx)),
                        ))),
                    Tool::Range => v_flex().child(
                        h_flex().gap_2()
                            .child(inputs.range_start.clone())
                            .child(Button::new("hex-range-mode", if self.range_uses_end { "End Offset" } else { "Length" })
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.range_uses_end = !this.range_uses_end;
                                    cx.notify();
                                })))
                            .child(inputs.range_size.clone())
                            .child(Button::new("hex-apply-range", "Select").on_click(
                                cx.listener(|this, _, window, cx| this.apply_range(window, cx)),
                            )),
                    ),
                    Tool::Inspector => {
                        let bytes = self.inspected_bytes();
                        let byte = bytes.first().copied();
                        v_flex().gap_2()
                            .child(h_flex().gap_2().flex_wrap()
                                .child(Button::new("hex-endian", if self.endian == Endian::Little { "Little Endian" } else { "Big Endian" })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.endian = if this.endian == Endian::Little { Endian::Big } else { Endian::Little };
                                        this.load_number_input(window, cx);
                                        if let Some(inputs) = &this.inputs {
                                            inputs.number.focus_handle(cx).focus(window, cx);
                                        }
                                    })))
                                .children(NumberType::ALL.into_iter().map(|kind| {
                                    Button::new(kind.label(), kind.label())
                                        .toggle_state(self.number_type == kind)
                                        .disabled(kind.decode(bytes, self.endian).is_none())
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            this.number_type = kind;
                                            this.load_number_input(window, cx);
                                            if let Some(inputs) = &this.inputs {
                                                inputs.number.focus_handle(cx).focus(window, cx);
                                            }
                                        }))
                                })))
                            .child(h_flex().gap_2()
                                .child(Label::new(format!("{} at 0x{:X}", self.number_type.label(), self.selection().start)).size(LabelSize::Small))
                                .child(inputs.number.clone())
                                .child(Button::new("hex-apply-number", "Apply")
                                    .disabled(self.busy || self.bytes_read_only() || bytes.len() < self.number_type.width())
                                    .on_click(cx.listener(|this, _, _, cx| this.apply_number(cx)))))
                            .child(h_flex().gap_2()
                                .child(Label::new("Bits 7..0").size(LabelSize::Small))
                                .children((0..8u8).rev().map(|bit| {
                                    Button::new(("hex-bit", bit as usize),
                                        if byte.is_some_and(|byte| byte & (1 << bit) != 0) { "1" } else { "0" })
                                        .disabled(self.busy || self.bytes_read_only() || byte.is_none())
                                        .on_click(cx.listener(move |this, _, window, cx| this.toggle_bit(bit, window, cx)))
                                })))
                            .child(h_flex().gap_4().flex_wrap().children(NumberType::ALL.into_iter().map(|kind| {
                                Label::new(format!("{}: {}", kind.label(), kind.decode(bytes, self.endian).unwrap_or_else(|| "—".into())))
                                    .size(LabelSize::Small)
                            })))
                    },
                    Tool::Bookmarks => v_flex().gap_2()
                        .child(h_flex().gap_2().child(inputs.bookmark.clone())
                            .child(Button::new("hex-add-bookmark", if self.editing_bookmark.is_some() { "Save Annotation" } else { "Add Bookmark" }).on_click(
                                cx.listener(|this, _, _, cx| this.add_bookmark(cx)),
                            ))
                            .child(Button::new("hex-new-bookmark", "New").on_click(
                                cx.listener(|this, _, window, cx| this.new_bookmark(window, cx)),
                            )))
                        .child(inputs.notes.clone())
                        .child(v_flex().id("hex-bookmark-list").max_h(px(150.)).overflow_y_scroll()
                            .children(self.bookmarks.iter().map(|bookmark| {
                                let id = bookmark.id;
                                let range = bookmark.range(&self.buffer.changes, self.buffer.bytes.len());
                                let location = range.as_ref().map(|range| format!("0x{:X} ({} bytes)", range.start, range.len()))
                                    .unwrap_or_else(|| "Unavailable; undo or retarget".into());
                                h_flex().gap_2().flex_wrap()
                                    .child(Button::new(("hex-bookmark", id), bookmark.name.clone())
                                        .disabled(range.is_none()).on_click(
                                            cx.listener(move |this, _, window, cx| this.jump_to_bookmark(id, window, cx)),
                                        ))
                                    .child(Label::new(location).size(LabelSize::Small))
                                    .child(Button::new(("hex-edit-bookmark", id), "Edit").on_click(
                                        cx.listener(move |this, _, window, cx| this.edit_bookmark(id, window, cx)),
                                    ))
                                    .child(Button::new(("hex-retarget-bookmark", id), "Use Selection").on_click(
                                        cx.listener(move |this, _, _, cx| this.retarget_bookmark(id, cx)),
                                    ))
                                    .child(Button::new(("hex-remove-bookmark", id), "Remove").on_click(
                                        cx.listener(move |this, _, window, cx| this.remove_bookmark(id, window, cx)),
                                    ))
                            }))),
                };
                root.child(v_flex().gap_2().child(controls).child(
                    h_flex().gap_2().justify_between()
                        .child(Label::new(self.message.clone().unwrap_or_else(|| match tool {
                            Tool::Find => "Hex supports ? nibble wildcards. Replacements use the selected format, without wildcards.",
                            Tool::Fill => "Repeat the pattern across the selection without changing file size.",
                            Tool::Offset => "Use decimal offsets or prefix hexadecimal offsets with 0x.",
                            Tool::Range => "Offsets and lengths accept decimal or 0x-prefixed hex. End offset is exclusive.",
                            Tool::Inspector => "Values use the selection start or cursor. Applying overwrites existing bytes.",
                            Tool::Bookmarks => "Bookmarks and notes are saved locally. Unsaved inserted targets persist after saving bytes. Use Selection retargets an entry.",
                        }.into()))
                            .size(LabelSize::Small))
                        .child(Button::new("hex-close-tools", "Close").on_click(
                            cx.listener(|this, _, window, cx| {
                                this.active_tool = None;
                                this.focus_handle.focus(window, cx);
                                cx.notify();
                            }),
                        )),
                ))
            })
            .when(self.comparison_load.is_some(), |root| root.child(Label::new("Loading comparison…").size(LabelSize::Small)))
            .when_some(self.comparison.as_ref(), |root, comparison| root
                .child(h_flex().gap_2().flex_wrap()
                    .child(Label::new(format!("Current ↔ {} (read-only snapshot)", comparison.path.display())).size(LabelSize::Small))
                    .child(Button::new("hex-previous-difference", "Previous Difference").on_click(
                        cx.listener(|this, _, window, cx| this.find_difference(true, window, cx)),
                    ))
                    .child(Button::new("hex-next-difference", "Next Difference").on_click(
                        cx.listener(|this, _, window, cx| this.find_difference(false, window, cx)),
                    ))
                    .child(Button::new("hex-close-comparison", "Close Comparison").on_click(
                        cx.listener(|this, _, window, cx| this.close_comparison(&CloseComparison, window, cx)),
                    )))
                .child(Label::new(comparison.stats.as_ref().filter(|(revision, _)| *revision == self.buffer.revision)
                    .map(|(_, stats)| format!("{} changed | {} current-only | {} reference-only | {} differing regions. Compared at equal offsets, not insertion-aligned.",
                        stats.changed, stats.current_only, stats.reference_only, stats.regions))
                    .unwrap_or_else(|| "Calculating differences…".into())).size(LabelSize::Small))
                .when_some(self.diff_region.as_ref(), |root, range| root.child(Label::new(
                    format!("Difference 0x{:X}..0x{:X}", range.start, range.end)).size(LabelSize::Small))))
            .when_some(self.metadata_error.clone(), |root, error| root.child(Label::new(error).color(Color::Error).size(LabelSize::Small)))
            .child(h_flex().gap_4()
                .child(div().text_color(cx.theme().status().modified).child("Modified"))
                .child(div().text_color(cx.theme().status().created).child(format!("Inserted: {}", self.buffer.changes.inserted)))
                .child(div().text_color(cx.theme().status().deleted).child(format!(
                    "Deleted: {}", self.buffer.changes.deletions.values().sum::<usize>()
                ))))
            .child(
                h_flex()
                    .gap_4()
                    .text_color(cx.theme().colors().text_muted)
                    .child(div().w(px(80.)).child("Offset"))
                    .child(
                        h_flex().gap_1().children(
                            (0..BYTES_PER_ROW)
                                .map(|column| div().w(px(24.)).child(format!("{column:02X}"))),
                        ),
                    )
                    .child(div().w(px(160.)).child(if self.comparison.is_some() { "Current ASCII" } else { "ASCII" }))
                    .when(self.comparison.is_some(), |row| row
                        .child(div().w(px(16.)))
                        .child(h_flex().gap_1().children((0..BYTES_PER_ROW)
                            .map(|column| div().w(px(24.)).child(format!("{column:02X}")))))
                        .child("Reference ASCII")),
            )
            .child(list)
            .child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .child(Label::new(status).size(LabelSize::Small))
                    .child(
                        Button::new(
                            "hex-insert-mode",
                            if self.insert { "Insert" } else { "Overwrite" },
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_insert(&ToggleInsertMode, window, cx);
                            this.focus_handle.focus(window, cx);
                        })),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    async fn test_annotation_database_round_trip_and_reload(cx: &mut gpui::TestAppContext) {
        use fs::Fs as _;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        let root = PathBuf::from(util::path!("/project"));
        fs.insert_tree(&root, serde_json::json!({})).await;
        let path = root.join("metadata.bin");
        fs.insert_file(&path, b"abcdef".to_vec()).await;
        let project = project::Project::test(fs.clone(), [root.as_path()], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view = HexEditor::new(path.clone(), project.clone(), b"abcdef".to_vec(), cx);
            view.move_to(1, false, cx);
            view.move_to(3, true, cx);
            view.annotations(&Annotations, window, cx);
            view
        });
        view.update_in(cx, |view, window, cx| {
            let inputs = view.inputs.as_ref().expect("inputs");
            inputs
                .bookmark
                .update(cx, |input, cx| input.set_text("length", window, cx));
            inputs.notes.update(cx, |input, cx| {
                input.set_text("First line\nSecond line", window, cx)
            });
            view.add_bookmark(cx);
            assert!(!view.buffer.is_dirty());
        });
        view.read_with(cx, |view, _| view.metadata_write.clone().expect("write"))
            .await
            .expect("database write");
        cx.update(|_, cx| cx.global_mut::<MetadataCache>().snapshots.clear());
        let reopened =
            cx.new(|cx| HexEditor::new(path.clone(), project.clone(), b"abcdef".to_vec(), cx));
        reopened.read_with(cx, |view, _| {
            assert_eq!(view.bookmarks.len(), 1);
            assert_eq!(view.bookmarks[0].range(&view.buffer.changes, 6), Some(1..3));
            assert_eq!(view.bookmarks[0].notes, "First line\nSecond line");
        });
        view.update_in(cx, |view, window, cx| {
            view.edit_bookmark(0, window, cx);
            view.inputs
                .as_ref()
                .expect("inputs")
                .bookmark
                .update(cx, |input, cx| input.set_text("renamed length", window, cx));
            view.save_annotation(&SaveAnnotation, window, cx);
            assert_eq!(view.bookmarks.len(), 1);
            view.buffer.replace(0..0, vec![0xff]).expect("insert");
            assert_eq!(view.bookmarks[0].range(&view.buffer.changes, 7), Some(2..4));
        });
        view.update_in(cx, |view, window, cx| {
            view.reload(project.clone(), window, cx)
        })
        .await
        .expect("reload original");
        view.read_with(cx, |view, _| {
            assert_eq!(view.bookmarks[0].name, "renamed length");
            assert_eq!(view.bookmarks[0].range(&view.buffer.changes, 6), Some(1..3));
        });
        fs.write(&path, b"uvwxyz").await.expect("external change");
        view.update_in(cx, |view, window, cx| {
            view.reload(project.clone(), window, cx)
        })
        .await
        .expect("reload changed");
        view.update_in(cx, |view, window, cx| {
            assert!(view.bookmarks[0].range(&view.buffer.changes, 6).is_none());
            assert_eq!(view.bookmarks[0].notes, "First line\nSecond line");
            view.move_to(2, false, cx);
            view.move_to(5, true, cx);
            view.retarget_bookmark(0, cx);
            assert_eq!(view.bookmarks[0].range(&view.buffer.changes, 6), Some(2..5));
            view.remove_bookmark(0, window, cx);
        });
        view.read_with(cx, |view, _| view.metadata_write.clone().expect("write"))
            .await
            .expect("delete metadata");
        cx.update(|_, cx| cx.global_mut::<MetadataCache>().snapshots.clear());
        let reopened = cx.new(|cx| HexEditor::new(path.clone(), project, b"uvwxyz".to_vec(), cx));
        reopened.read_with(cx, |view, _| assert!(view.bookmarks.is_empty()));
        assert_eq!(fs.load_bytes(&path).await.expect("file"), b"uvwxyz");
    }

    #[gpui::test]
    async fn test_annotations_follow_save_as_without_changing_original(
        cx: &mut gpui::TestAppContext,
    ) {
        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        let root = PathBuf::from(util::path!("/project"));
        fs.insert_tree(&root, serde_json::json!({})).await;
        let path = root.join("original.bin");
        fs.insert_file(&path, vec![1, 2, 3]).await;
        let project = project::Project::test(fs, [root.as_path()], cx).await;
        let source = project.read_with(cx, |project, cx| {
            project.find_project_path(&path, cx).expect("source")
        });
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view = HexEditor::new(path.clone(), project.clone(), vec![1, 2, 3], cx);
            view.show_bookmarks(&Bookmarks, window, cx);
            view
        });
        view.update_in(cx, |view, window, cx| {
            view.inputs
                .as_ref()
                .expect("inputs")
                .bookmark
                .update(cx, |input, cx| input.set_text("original byte", window, cx));
            view.add_bookmark(cx);
            view.buffer.replace(0..0, vec![9]).expect("insert");
            view.move_to(0, false, cx);
            view.inputs
                .as_ref()
                .expect("inputs")
                .bookmark
                .update(cx, |input, cx| input.set_text("inserted byte", window, cx));
            view.add_bookmark(cx);
            view.inputs
                .as_ref()
                .expect("inputs")
                .bookmark
                .update(cx, |input, cx| input.set_text("", window, cx));
            view.add_bookmark(cx);
            assert_eq!(view.bookmarks.len(), 2);
            assert!(view.error.is_some());
            let oversized = "x".repeat(bookmarks::MAX_NOTE_BYTES + 1);
            view.inputs
                .as_ref()
                .expect("inputs")
                .bookmark
                .update(cx, |input, cx| input.set_text("invalid note", window, cx));
            view.inputs
                .as_ref()
                .expect("inputs")
                .notes
                .update(cx, |input, cx| input.set_text(&oversized, window, cx));
            view.add_bookmark(cx);
            assert_eq!(view.bookmarks.len(), 2);
        });
        view.read_with(cx, |view, cx| {
            let input = view.inputs.as_ref().expect("inputs").notes.read(cx);
            let erased = input.editor();
            let editor = erased
                .as_any()
                .downcast_ref::<Entity<crate::Editor>>()
                .expect("editor");
            assert!(
                editor
                    .read(cx)
                    .last_bounds()
                    .expect("notes bounds")
                    .size
                    .width
                    > px(100.)
            );
        });
        let destination = (source.worktree_id, util::rel_path::rel_path("copy.bin")).into();
        view.update_in(cx, |view, window, cx| {
            view.save_as(project.clone(), destination, window, cx)
        })
        .await
        .expect("save copy");
        view.read_with(cx, |view, _| view.metadata_write.clone().expect("write"))
            .await
            .expect("copy metadata");
        cx.update(|_, cx| cx.global_mut::<MetadataCache>().snapshots.clear());
        let original = cx.new(|cx| HexEditor::new(path, project.clone(), vec![1, 2, 3], cx));
        original.read_with(cx, |view, _| {
            assert_eq!(view.bookmarks[0].range(&view.buffer.changes, 3), Some(0..0));
            assert!(view.bookmarks[1].range(&view.buffer.changes, 3).is_none());
        });
        let copy =
            cx.new(|cx| HexEditor::new(root.join("copy.bin"), project, vec![9, 1, 2, 3], cx));
        copy.read_with(cx, |view, _| {
            assert_eq!(view.bookmarks[0].range(&view.buffer.changes, 4), Some(1..1));
            assert_eq!(view.bookmarks[1].range(&view.buffer.changes, 4), Some(0..0));
        });
    }

    #[gpui::test]
    async fn test_binary_comparison_picker_navigation_and_protection(
        cx: &mut gpui::TestAppContext,
    ) {
        use fs::Fs as _;
        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        let root = PathBuf::from(util::path!("/project"));
        fs.insert_tree(&root, serde_json::json!({})).await;
        let path = root.join("current.bin");
        let reference = root.join("reference.bin");
        fs.insert_file(&path, vec![0, 1, 2, 3]).await;
        fs.insert_file(&reference, vec![0, 9, 8, 3, 4]).await;
        let project = project::Project::test(fs.clone(), [root.as_path()], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| {
            let view = HexEditor::new(path.clone(), project, vec![0, 1, 2, 3], cx);
            view.focus_handle.focus(window, cx);
            view
        });
        view.update_in(cx, |view, window, cx| {
            view.compare_files(&CompareFiles, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_path_prompt_response(|_| Some(vec![reference.clone()]));
        cx.run_until_parked();
        view.update_in(cx, |view, window, cx| {
            assert!(view.comparison.is_some());
            assert!(!view.capability(cx).editable());
            view.next_difference(&NextDifference, window, cx);
            assert_eq!(view.selection(), 1..3);
            view.next_difference(&NextDifference, window, cx);
            assert_eq!(view.diff_region, Some(4..5));
            assert_eq!(view.cursor, 4);
            view.next_difference(&NextDifference, window, cx);
            assert_eq!(view.diff_region, Some(1..3));
            view.previous_difference(&PreviousDifference, window, cx);
            assert_eq!(view.diff_region, Some(4..5));
            view.input_digit(4, cx);
            view.input_bytes(vec![9], cx);
            view.toggle_bit(0, window, cx);
            assert_eq!(view.buffer.bytes, [0, 1, 2, 3]);
        });
        cx.simulate_keystrokes("4 2 backspace delete cmd-z");
        view.update_in(cx, |view, window, cx| {
            view.compare_files(&CompareFiles, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_path_prompt_response(|_| None);
        cx.run_until_parked();
        view.read_with(cx, |view, _| {
            assert_eq!(
                view.comparison.as_ref().expect("comparison").path,
                reference
            )
        });
        fs.write(&reference, &[0, 1, 2, 3])
            .await
            .expect("external reference edit");
        view.read_with(cx, |view, _| {
            assert_eq!(
                view.comparison.as_ref().expect("snapshot").bytes.as_ref(),
                [0, 9, 8, 3, 4]
            )
        });
        assert!(
            view.update_in(cx, |view, _, cx| {
                view.compare_path(root.join("missing.bin"), cx)
            })
            .await
            .is_err()
        );
        cx.run_until_parked();
        view.read_with(cx, |view, _| {
            assert!(view.error.is_some());
            assert!(view.comparison.is_some());
        });
        view.update_in(cx, |view, window, cx| {
            view.close_comparison(&CloseComparison, window, cx)
        });
        view.read_with(cx, |view, cx| {
            assert!(view.capability(cx).editable());
            assert!(view.comparison.is_none());
            assert!(!view.buffer.is_dirty());
        });
        assert_eq!(fs.load_bytes(&path).await.expect("current"), [0, 1, 2, 3]);
    }

    #[test]
    fn test_hex_buffer_edits_and_history() -> Result<()> {
        let mut buffer = HexBuffer::new(vec![0, 0xff, 0x80]);
        buffer.replace(1..2, vec![0x42])?;
        buffer.replace(3..3, vec![0xab, 0xcd])?;
        buffer.replace(0..1, Vec::new())?;
        assert_eq!(buffer.bytes, [0x42, 0x80, 0xab, 0xcd]);
        assert!(buffer.is_dirty());
        assert_eq!(buffer.undo(), Some(0));
        assert_eq!(buffer.undo(), Some(3));
        assert_eq!(buffer.undo(), Some(1));
        assert_eq!(buffer.bytes, [0, 0xff, 0x80]);
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.redo(), Some(1));
        assert_eq!(buffer.bytes, [0, 0x42, 0x80]);
        buffer.replace(0..1, vec![1])?;
        assert_eq!(buffer.redo(), None);
        Ok(())
    }

    #[test]
    fn test_saved_revision_survives_undo_and_branching() -> Result<()> {
        let mut buffer = HexBuffer::new(vec![0]);
        buffer.replace(0..1, vec![1])?;
        buffer.saved_revision = buffer.revision;
        buffer.saved_bytes = buffer.bytes.clone();
        assert!(!buffer.is_dirty());
        buffer.undo();
        assert!(buffer.is_dirty());
        buffer.redo();
        assert!(!buffer.is_dirty());
        buffer.undo();
        buffer.replace(0..1, vec![2])?;
        assert!(buffer.is_dirty());
        Ok(())
    }

    #[test]
    fn test_no_op_and_invalid_edits() -> Result<()> {
        let mut buffer = HexBuffer::new(vec![0, 1]);
        buffer.replace(0..1, vec![0])?;
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.undo(), None);
        assert!(buffer.replace(1..3, vec![2]).is_err());
        assert_eq!(buffer.bytes, [0, 1]);
        Ok(())
    }

    #[test]
    fn test_empty_buffer_and_size_limit() -> Result<()> {
        let mut buffer = HexBuffer::new(Vec::new());
        buffer.replace(0..0, vec![0xff])?;
        assert_eq!(buffer.bytes, [0xff]);
        buffer.undo();
        assert!(buffer.bytes.is_empty());
        assert!(!buffer.is_dirty());

        let mut buffer = HexBuffer::new(vec![0; MAX_FILE_SIZE]);
        assert!(
            buffer
                .replace(MAX_FILE_SIZE..MAX_FILE_SIZE, vec![1])
                .is_err()
        );
        assert!(!buffer.is_dirty());
        buffer.replace(0..1, vec![0xff])?;
        assert_eq!(buffer.bytes.len(), MAX_FILE_SIZE);
        Ok(())
    }

    #[test]
    fn test_parse_hex() -> Result<()> {
        assert_eq!(parse_hex("00 ff 42\n80\tab")?, [0, 0xff, 0x42, 0x80, 0xab]);
        assert_eq!(parse_hex("0123ABcd")?, [1, 0x23, 0xab, 0xcd]);
        assert!(parse_hex("0").is_err());
        assert!(parse_hex("0g").is_err());
        assert!(parse_hex("é0").is_err());
        assert!(parse_hex("")?.is_empty());
        Ok(())
    }

    #[test]
    fn test_byte_search_wrap_overlap_and_bounds() -> Result<()> {
        let bytes = b"ababab";
        assert_eq!(find_bytes(bytes, b"abab", 0, false)?, Some(0));
        assert_eq!(find_bytes(bytes, b"abab", 1, false)?, Some(2));
        assert_eq!(find_bytes(bytes, b"abab", 3, false)?, Some(0));
        assert_eq!(find_bytes(bytes, b"abab", 2, true)?, Some(0));
        assert_eq!(find_bytes(bytes, b"abab", 0, true)?, Some(2));
        assert_eq!(find_bytes(bytes, b"ab", usize::MAX, true)?, Some(4));
        assert_eq!(find_bytes(bytes, b"missing", 0, false)?, None);
        assert_eq!(find_bytes(bytes, b"", 0, false)?, None);
        assert_eq!(find_bytes(b"", b"ab", 0, true)?, None);
        assert_eq!(find_bytes(&[0, 255, 128], &[255, 128], 0, false)?, Some(1));
        Ok(())
    }

    #[test]
    fn test_replace_all_is_atomic_and_undoable() -> Result<()> {
        let mut buffer = HexBuffer::new(b"aaaa!".to_vec());
        assert_eq!(buffer.replace_all(b"aa", b"XYZ")?, 2);
        assert_eq!(buffer.bytes, b"XYZXYZ!");
        assert_eq!(buffer.undo.len(), 1);
        buffer.undo();
        assert_eq!(buffer.bytes, b"aaaa!");
        assert!(!buffer.is_dirty());
        buffer.redo();
        assert_eq!(buffer.bytes, b"XYZXYZ!");
        assert_eq!(buffer.replace_all(b"XYZ", b"")?, 2);
        assert_eq!(buffer.bytes, b"!");
        assert_eq!(buffer.replace_all(b"missing", b"0")?, 0);
        assert!(buffer.replace_all(b"", b"0").is_err());
        assert_eq!(buffer.bytes, b"!");
        Ok(())
    }

    #[test]
    fn test_replace_all_size_limit_preserves_buffer() {
        let mut bytes = vec![0; MAX_FILE_SIZE];
        bytes[MAX_FILE_SIZE - 1] = 1;
        let mut buffer = HexBuffer::new(bytes);
        assert!(buffer.replace_all(&[1], &[2, 3]).is_err());
        assert_eq!(buffer.bytes.len(), MAX_FILE_SIZE);
        assert_eq!(buffer.bytes.last(), Some(&1));
        assert!(!buffer.is_dirty());
        assert!(buffer.undo.is_empty());
    }

    #[test]
    fn test_parse_offsets() -> Result<()> {
        assert_eq!(parse_offset(" 256 ")?, 256);
        assert_eq!(parse_offset("0x100")?, 256);
        assert_eq!(parse_offset("0Xff")?, 255);
        assert_eq!(parse_offset("0")?, 0);
        for invalid in ["", "-1", "0x", "ff", "1.5", "0xffffffffffffffffffff"] {
            assert!(parse_offset(invalid).is_err(), "{invalid}");
        }
        Ok(())
    }

    #[test]
    fn test_bookmarks_follow_edits_and_return_on_undo() -> Result<()> {
        let mut buffer = HexBuffer::new(b"abcd".to_vec());
        let bookmark = Bookmark::new(0, "header".into(), 1..3, &buffer.changes);
        let eof = Bookmark::new(1, "end".into(), 4..4, &buffer.changes);
        buffer.replace(0..0, vec![0xff])?;
        assert_eq!(
            bookmark.range(&buffer.changes, buffer.bytes.len()),
            Some(2..4)
        );
        assert_eq!(eof.range(&buffer.changes, buffer.bytes.len()), Some(5..5));
        buffer.mark_saved();
        buffer.replace(2..3, Vec::new())?;
        assert!(
            bookmark
                .range(&buffer.changes, buffer.bytes.len())
                .is_none()
        );
        buffer.undo();
        assert_eq!(
            bookmark.range(&buffer.changes, buffer.bytes.len()),
            Some(2..4)
        );
        buffer.undo();
        assert_eq!(
            bookmark.range(&buffer.changes, buffer.bytes.len()),
            Some(1..3)
        );
        Ok(())
    }

    #[test]
    fn test_wildcard_replace_all_preserves_unchanged_bytes() -> Result<()> {
        let mut buffer = HexBuffer::new(vec![0xa1, 0x1f, 7, 0xa2, 0x2f, 8]);
        let pattern = SearchMode::Hex.pattern("A? ?F", NumberType::U8, Endian::Little)?;
        assert_eq!(buffer.replace_pattern(&pattern, &[0x42])?, 2);
        assert_eq!(buffer.bytes, [0x42, 7, 0x42, 8]);
        assert_eq!(buffer.byte_change(1), None);
        assert_eq!(buffer.byte_change(3), None);
        buffer.undo();
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.bytes, [0xa1, 0x1f, 7, 0xa2, 0x2f, 8]);
        Ok(())
    }

    #[gpui::test]
    async fn test_read_only_and_save_as_preserve_original(cx: &mut gpui::TestAppContext) {
        use fs::Fs as _;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        let root = PathBuf::from(util::path!("/project"));
        fs.insert_tree(&root, serde_json::json!({})).await;
        let original = root.join("original.bin");
        fs.insert_file(&original, vec![0xff, 0x80, 0]).await;
        let project = project::Project::test(fs.clone(), [root.as_path()], cx).await;
        let source = project.read_with(cx, |project, cx| {
            project.find_project_path(&original, cx).expect("source")
        });
        let destination = (source.worktree_id, util::rel_path::rel_path("copy.bin")).into();
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view =
                HexEditor::new(original.clone(), project.clone(), vec![0xff, 0x80, 0], cx);
            view.focus_handle.focus(window, cx);
            view.toggle_read_only(&ToggleReadOnly, window, cx);
            view
        });
        cx.simulate_keystrokes("4 2 delete backspace");
        view.update_in(cx, |view, window, cx| {
            view.input_bytes(vec![1, 2], cx);
            view.toggle_bit(0, window, cx);
            view.undo(&Undo, window, cx);
            view.redo(&Redo, window, cx);
            cx.write_to_clipboard(ClipboardItem::new_string("00".into()));
            view.paste(&Paste, window, cx);
            assert_eq!(view.buffer.bytes, [0xff, 0x80, 0]);
            assert!(!view.can_save(cx));
            assert!(view.can_save_as(cx));
        });
        assert!(
            view.update_in(cx, |view, window, cx| view.save(
                SaveOptions::default(),
                project.clone(),
                window,
                cx,
            ))
            .await
            .is_err()
        );
        view.update_in(cx, |view, window, cx| {
            view.save_as(project.clone(), destination, window, cx)
        })
        .await
        .expect("save copy");
        assert_eq!(
            fs.load_bytes(&original).await.expect("original"),
            [0xff, 0x80, 0]
        );
        assert_eq!(
            fs.load_bytes(&root.join("copy.bin")).await.expect("copy"),
            [0xff, 0x80, 0]
        );
        view.update_in(cx, |view, window, cx| {
            assert_eq!(view.path, root.join("copy.bin"));
            assert!(view.read_only);
            view.toggle_read_only(&ToggleReadOnly, window, cx);
            view.input_digit(4, cx);
            view.toggle_read_only(&ToggleReadOnly, window, cx);
            assert!(!view.read_only);
            assert!(view.error.is_some());
        });
        view.update_in(cx, |view, window, cx| {
            view.save(SaveOptions::default(), project.clone(), window, cx)
        })
        .await
        .expect("save copy edits");
        assert_eq!(
            fs.load_bytes(&original).await.expect("unchanged original"),
            [0xff, 0x80, 0]
        );
        for relative_path in ["copy.bin", "missing/copy.bin", ""] {
            let destination = (source.worktree_id, util::rel_path::rel_path(relative_path)).into();
            assert!(
                view.update_in(cx, |view, window, cx| view.save_as(
                    project.clone(),
                    destination,
                    window,
                    cx,
                ))
                .await
                .is_err()
            );
            view.read_with(cx, |view, _| {
                assert_eq!(view.path, root.join("copy.bin"));
                assert!(!view.busy);
            });
        }
        let source = (source.worktree_id, util::rel_path::rel_path("original.bin")).into();
        assert!(
            view.update_in(cx, |view, window, cx| view.save_as(
                project.clone(),
                source,
                window,
                cx,
            ))
            .await
            .is_err()
        );
        view.read_with(cx, |view, _| {
            assert_eq!(view.path, root.join("copy.bin"));
            assert!(!view.busy);
        });
    }

    #[gpui::test]
    async fn test_workspace_save_as_cancel_failure_and_read_only_copy(
        cx: &mut gpui::TestAppContext,
    ) {
        use fs::Fs as _;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        let root = PathBuf::from(util::path!("/project"));
        fs.insert_tree(&root, serde_json::json!({})).await;
        let original = root.join("original.bin");
        let existing = root.join("existing.bin");
        fs.insert_file(&original, vec![0xff, 0x80]).await;
        fs.insert_file(&existing, vec![0x99]).await;
        let project = project::Project::test(fs.clone(), [root.as_path()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let (source, target) = workspace.update_in(cx, |workspace, window, cx| {
            let source = cx
                .new(|cx| HexEditor::new(original.clone(), project.clone(), vec![0xff, 0x80], cx));
            let target = cx.new(|cx| {
                let mut target = HexEditor::new(existing.clone(), project.clone(), vec![0x99], cx);
                target
                    .buffer
                    .replace(0..1, vec![0x42])
                    .expect("dirty target");
                target
            });
            workspace.add_item_to_active_pane(Box::new(source.clone()), None, true, window, cx);
            workspace.add_item_to_active_pane(Box::new(target.clone()), None, true, window, cx);
            workspace
                .active_pane()
                .update(cx, |pane, cx| pane.activate_item(0, true, true, window, cx));
            (source, target)
        });
        let save = workspace.update_in(cx, |workspace, window, cx| {
            workspace.save_active_item(workspace::SaveIntent::SaveAs, window, cx)
        });
        cx.run_until_parked();
        assert!(cx.did_prompt_for_new_path());
        cx.simulate_new_path_selection(|_| None);
        save.await.expect("cancel");
        source.read_with(cx, |source, _| assert_eq!(source.path, original));
        let save = workspace.update_in(cx, |workspace, window, cx| {
            workspace.save_active_item(workspace::SaveIntent::SaveAs, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_new_path_selection(|_| Some(existing.clone()));
        assert!(save.await.is_err());
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items_of_type::<HexEditor>(cx).count(), 2);
            assert_eq!(
                workspace.active_item_as::<HexEditor>(cx),
                Some(source.clone())
            );
        });
        target.read_with(cx, |target, _| {
            assert_eq!(target.buffer.bytes, [0x42]);
            assert!(target.buffer.is_dirty());
        });
        source.update_in(cx, |source, window, cx| {
            source.toggle_read_only(&ToggleReadOnly, window, cx)
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.save_active_item(workspace::SaveIntent::Save, window, cx)
            })
            .await
            .expect("read-only save is a no-op");
        assert!(!cx.did_prompt_for_new_path());
        let save = workspace.update_in(cx, |workspace, window, cx| {
            workspace.save_active_item(workspace::SaveIntent::SaveAs, window, cx)
        });
        cx.run_until_parked();
        let copy = root.join("copy.bin");
        cx.simulate_new_path_selection(|_| Some(copy.clone()));
        save.await.expect("read-only copy");
        assert_eq!(
            fs.load_bytes(&original).await.expect("original"),
            [0xff, 0x80]
        );
        assert_eq!(fs.load_bytes(&copy).await.expect("copy"), [0xff, 0x80]);
        source.read_with(cx, |source, _| {
            assert_eq!(source.path, copy);
            assert!(source.read_only);
        });
        assert_eq!(
            fs.load_bytes(&existing).await.expect("existing target"),
            [0x99]
        );
    }

    #[gpui::test]
    async fn test_search_modes_bookmarks_and_read_only_tools(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let project = project::Project::test(fs, [Path::new(util::path!("/project"))], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view = HexEditor::new(
                util::path!("/project/data.bin").into(),
                project,
                vec![0x34, 0x12, b'a', b'b', b'a', b'b'],
                cx,
            );
            view.find(&Find, window, cx);
            view
        });
        view.update_in(cx, |view, window, cx| {
            view.search_mode = SearchMode::Number;
            view.number_type = NumberType::U16;
            view.inputs
                .as_ref()
                .expect("inputs")
                .pattern
                .update(cx, |input, cx| input.set_text("4660", window, cx));
            view.find_match(false, cx);
            assert_eq!(view.selection(), 0..2);
            view.search_mode = SearchMode::Ascii;
            view.inputs
                .as_ref()
                .expect("inputs")
                .pattern
                .update(cx, |input, cx| input.set_text("ab", window, cx));
            view.find_match(false, cx);
            assert_eq!(view.selection(), 2..4);
            view.show_bookmarks(&Bookmarks, window, cx);
            view.inputs
                .as_ref()
                .expect("inputs")
                .bookmark
                .update(cx, |input, cx| input.set_text("record: magic", window, cx));
            view.add_bookmark(cx);
            assert_eq!(view.bookmarks.len(), 1);
            view.move_to(0, false, cx);
            view.jump_to_bookmark(0, window, cx);
            assert_eq!(view.selection(), 2..4);
            view.read_only = true;
            view.inputs
                .as_ref()
                .expect("inputs")
                .replacement
                .update(cx, |input, cx| input.set_text("x", window, cx));
            view.replace_match(true, cx);
            view.fill_selection(cx);
            view.apply_number(cx);
            assert!(!view.buffer.is_dirty());
            view.read_only = false;
            view.replace_match(true, cx);
            assert_eq!(view.buffer.bytes, [0x34, 0x12, b'x', b'x']);
            assert!(
                view.bookmarks[0]
                    .range(&view.buffer.changes, view.buffer.bytes.len())
                    .is_none()
            );
            view.undo(&Undo, window, cx);
            view.jump_to_bookmark(0, window, cx);
            assert_eq!(view.selection(), 2..4);
        });
    }

    #[test]
    fn test_precise_ranges_and_bounds() -> Result<()> {
        assert_eq!(byte_range(2, 3, false, 8)?, 2..5);
        assert_eq!(byte_range(2, 5, true, 8)?, 2..5);
        assert_eq!(byte_range(8, 0, false, 8)?, 8..8);
        assert_eq!(byte_range(0, 8, false, 8)?, 0..8);
        assert!(byte_range(5, 2, true, 8).is_err());
        assert!(byte_range(7, 2, false, 8).is_err());
        assert!(byte_range(9, 0, false, 8).is_err());
        assert!(byte_range(usize::MAX, 1, false, 8).is_err());
        Ok(())
    }

    #[test]
    fn test_byte_change_tracking_preserves_shifted_bytes() -> Result<()> {
        let mut buffer = HexBuffer::new(vec![0x10, 0x20, 0x30, 0x40]);
        buffer.replace(1..1, vec![0xaa])?;
        assert_eq!(buffer.byte_change(1), Some(ByteChange::Inserted));
        assert_eq!(buffer.byte_change(2), None);
        assert_eq!(buffer.byte_change(4), None);
        assert_eq!(buffer.changes.inserted, 1);
        buffer.replace(2..3, vec![0x22])?;
        assert_eq!(buffer.byte_change(2), Some(ByteChange::Modified));
        buffer.replace(3..4, Vec::new())?;
        assert_eq!(buffer.bytes, [0x10, 0xaa, 0x22, 0x40]);
        assert_eq!(buffer.byte_change(3), None);
        assert_eq!(buffer.changes.deletions.get(&3), Some(&1));
        buffer.undo();
        buffer.undo();
        buffer.undo();
        assert_eq!(buffer.bytes, [0x10, 0x20, 0x30, 0x40]);
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.changes.inserted, 0);
        assert!(buffer.changes.deletions.is_empty());
        assert!((0..buffer.bytes.len()).all(|offset| buffer.byte_change(offset).is_none()));
        Ok(())
    }

    #[test]
    fn test_change_tracking_across_save_undo_and_replace_all() -> Result<()> {
        let mut buffer = HexBuffer::new(b"abc".to_vec());
        buffer.replace(1..1, b"x".to_vec())?;
        buffer.mark_saved();
        assert_eq!(buffer.changes.inserted, 0);
        buffer.undo();
        assert_eq!(buffer.bytes, b"abc");
        assert_eq!(buffer.changes.deletions.get(&1), Some(&1));
        assert!((0..3).all(|offset| buffer.byte_change(offset).is_none()));
        buffer.redo();
        assert!(buffer.changes.deletions.is_empty());
        assert!(!buffer.is_dirty());
        buffer.replace(1..2, Vec::new())?;
        buffer.mark_saved();
        buffer.undo();
        assert_eq!(buffer.byte_change(1), Some(ByteChange::Inserted));
        assert_eq!(buffer.byte_change(2), None);
        buffer.redo();
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.replace_all(b"b", b"YZ")?, 1);
        assert_eq!(buffer.bytes, b"aYZc");
        assert_eq!(buffer.byte_change(0), None);
        assert_eq!(buffer.byte_change(1), Some(ByteChange::Modified));
        assert_eq!(buffer.byte_change(2), Some(ByteChange::Inserted));
        assert_eq!(buffer.byte_change(3), None);
        buffer.mark_saved();
        buffer.undo();
        assert_eq!(buffer.bytes, b"abc");
        assert_eq!(buffer.byte_change(1), Some(ByteChange::Modified));
        assert_eq!(buffer.changes.deletions.get(&2), Some(&1));
        assert_eq!(buffer.byte_change(2), None);
        buffer.redo();
        assert!(!buffer.is_dirty());
        assert!((0..4).all(|offset| buffer.byte_change(offset).is_none()));
        Ok(())
    }

    #[test]
    fn test_change_tracking_deleted_all_and_eof_markers() -> Result<()> {
        let mut buffer = HexBuffer::new(vec![1, 2, 3]);
        buffer.replace(2..3, Vec::new())?;
        assert_eq!(buffer.changes.deletions.get(&2), Some(&1));
        buffer.replace(0..2, Vec::new())?;
        assert_eq!(buffer.changes.deletions.get(&0), Some(&3));
        buffer.mark_saved();
        buffer.undo();
        assert_eq!(buffer.changes.inserted, 2);
        assert!(buffer.changes.deletions.is_empty());
        assert!((0..2).all(|offset| buffer.byte_change(offset) == Some(ByteChange::Inserted)));
        let mut buffer = HexBuffer::new(vec![0; MAX_FILE_SIZE]);
        buffer.replace(1..1, Vec::new())?;
        assert_eq!(buffer.changes.slice(0..MAX_FILE_SIZE).len(), 1);
        Ok(())
    }

    #[gpui::test]
    fn test_change_tracking_generated_history(mut rng: rand::rngs::StdRng) {
        use rand::Rng;

        let mut buffer = HexBuffer::new(vec![0, 1, 2, 3]);
        let mut identities: Vec<u64> = (0..4).collect();
        let mut next_identity = 4;
        let mut saved = identities.clone();
        let mut saved_bytes = buffer.bytes.clone();
        let mut undo = Vec::new();
        let mut redo = Vec::new();
        for _ in 0..200 {
            match rng.random_range(0..5) {
                0 | 1 => {
                    let start = rng.random_range(0..=buffer.bytes.len());
                    let end = rng.random_range(start..=buffer.bytes.len());
                    let bytes: Vec<u8> =
                        (0..rng.random_range(0..5)).map(|_| rng.random()).collect();
                    if buffer.bytes[start..end] != bytes {
                        let before = (buffer.bytes.clone(), identities.clone());
                        let retained = (end - start).min(bytes.len());
                        let mut replacement = identities[start..start + retained].to_vec();
                        for _ in retained..bytes.len() {
                            replacement.push(next_identity);
                            next_identity += 1;
                        }
                        buffer.replace(start..end, bytes).expect("valid edit");
                        identities.splice(start..end, replacement);
                        undo.push((before, (buffer.bytes.clone(), identities.clone())));
                        redo.clear();
                    }
                }
                2 => {
                    if let Some(edit) = undo.pop() {
                        buffer.undo();
                        assert_eq!(buffer.bytes, edit.0.0);
                        identities = edit.0.1.clone();
                        redo.push(edit);
                    }
                }
                3 => {
                    if let Some(edit) = redo.pop() {
                        buffer.redo();
                        assert_eq!(buffer.bytes, edit.1.0);
                        identities = edit.1.1.clone();
                        undo.push(edit);
                    }
                }
                _ => {
                    buffer.mark_saved();
                    saved = identities.clone();
                    saved_bytes = buffer.bytes.clone();
                }
            }
            let mut inserted = 0;
            for (offset, identity) in identities.iter().enumerate() {
                let expected = match saved.iter().position(|saved| saved == identity) {
                    None => {
                        inserted += 1;
                        Some(ByteChange::Inserted)
                    }
                    Some(saved) if saved_bytes[saved] != buffer.bytes[offset] => {
                        Some(ByteChange::Modified)
                    }
                    Some(_) => None,
                };
                assert_eq!(buffer.byte_change(offset), expected);
            }
            assert_eq!(buffer.changes.inserted, inserted);
            assert_eq!(
                buffer.changes.deletions.values().sum::<usize>(),
                saved
                    .iter()
                    .filter(|identity| !identities.contains(identity))
                    .count()
            );
        }
    }

    #[gpui::test]
    async fn test_hex_editor_ranges_inspector_and_bits(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let project = project::Project::test(fs, [Path::new(util::path!("/project"))], cx).await;
        let original = vec![0xff, 0, 0, 0, 0, 0, 0, 0];
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view = HexEditor::new(
                util::path!("/project/data.bin").into(),
                project,
                original.clone(),
                cx,
            );
            view.select_range(&SelectRange, window, cx);
            view
        });
        view.update_in(cx, |view, window, cx| {
            let inputs = view.inputs.as_ref().expect("tool fields");
            inputs
                .range_start
                .update(cx, |input, cx| input.set_text("0x0", window, cx));
            inputs
                .range_size
                .update(cx, |input, cx| input.set_text("8", window, cx));
            view.apply_range(window, cx);
            assert_eq!(view.selection(), 0..8);
            view.inspect(&Inspect, window, cx);
            view.number_type = NumberType::U16;
            view.load_number_input(window, cx);
            view.inputs
                .as_ref()
                .expect("tool fields")
                .number
                .update(cx, |input, cx| input.set_text("4660", window, cx));
            view.apply_number(cx);
            assert_eq!(&view.buffer.bytes[..2], &[0x34, 0x12]);
            assert_eq!(view.selection(), 0..8);
            view.undo(&Undo, window, cx);
            assert_eq!(view.buffer.bytes, original);
            view.move_to(0, false, cx);
            view.move_to(8, true, cx);
            view.endian = Endian::Big;
            view.load_number_input(window, cx);
            view.inputs
                .as_ref()
                .expect("tool fields")
                .number
                .update(cx, |input, cx| input.set_text("4660", window, cx));
            view.apply_number(cx);
            assert_eq!(&view.buffer.bytes[..2], &[0x12, 0x34]);
            view.toggle_bit(0, window, cx);
            assert_eq!(view.buffer.bytes[0], 0x13);
            view.undo(&Undo, window, cx);
            assert_eq!(view.buffer.bytes[0], 0x12);
            view.move_to(7, false, cx);
            view.number_type = NumberType::U32;
            view.load_number_input(window, cx);
            view.apply_number(cx);
            assert!(view.error.is_some());
            assert_eq!(view.buffer.bytes.len(), 8);
            view.move_to(0, false, cx);
            view.move_to(1, true, cx);
            view.apply_number(cx);
            assert!(view.error.is_some());
            view.busy = true;
            view.toggle_bit(0, window, cx);
            assert_eq!(view.buffer.bytes[0], 0x12);
            view.apply_number(cx);
            view.busy = false;
            view.select_range(&SelectRange, window, cx);
            let inputs = view.inputs.as_ref().expect("tool fields");
            inputs
                .range_start
                .update(cx, |input, cx| input.set_text("7", window, cx));
            inputs
                .range_size
                .update(cx, |input, cx| input.set_text("9", window, cx));
            view.apply_range(window, cx);
            assert_eq!(view.selection(), 0..1);
            assert!(view.error.is_some());
        });
    }

    #[gpui::test]
    async fn test_hex_editor_ascii_editing_and_clipboard(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let project = project::Project::test(fs, [Path::new(util::path!("/project"))], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view = HexEditor::new(
                util::path!("/project/data.bin").into(),
                project,
                b"abc".to_vec(),
                cx,
            );
            view.toggle_ascii(&ToggleAsciiMode, window, cx);
            view
        });
        cx.simulate_keystrokes("x space");
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            b"x c"
        );
        cx.simulate_keystrokes("end");
        view.update_in(cx, |view, window, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string("YZ".into()));
            view.paste(&Paste, window, cx);
            assert_eq!(view.buffer.bytes, b"x cYZ");
            view.move_to(0, false, cx);
            view.move_to(3, true, cx);
            view.copy(&Copy, window, cx);
            assert_eq!(
                cx.read_from_clipboard()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("x c")
            );
            cx.write_to_clipboard(ClipboardItem::new_string("é".into()));
            view.paste(&Paste, window, cx);
            assert_eq!(view.buffer.bytes, b"x cYZ");
            assert!(view.error.is_some());
            view.undo(&Undo, window, cx);
            assert_eq!(view.buffer.bytes, b"x c");
            view.toggle_insert(&ToggleInsertMode, window, cx);
            view.move_to(1, false, cx);
        });
        cx.simulate_keystrokes("q");
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            b"xq c"
        );
        cx.simulate_keystrokes("f1");
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            b"xq c"
        );
        view.update_in(cx, |view, window, cx| {
            view.buffer
                .replace(0..1, vec![0xff])
                .expect("non-ASCII byte");
            view.move_to(0, false, cx);
            cx.write_to_clipboard(ClipboardItem::new_string("unchanged".into()));
            view.copy(&Copy, window, cx);
            assert_eq!(
                cx.read_from_clipboard()
                    .and_then(|item| item.text())
                    .as_deref(),
                Some("unchanged")
            );
            assert!(view.error.is_some());
        });
    }

    #[gpui::test]
    async fn test_hex_editor_tools_and_input_focus(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let project = project::Project::test(fs, [Path::new(util::path!("/project"))], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view = HexEditor::new(
                util::path!("/project/data.bin").into(),
                project,
                vec![0xff, 0, 0xff, 0],
                cx,
            );
            view.find(&Find, window, cx);
            view
        });
        cx.simulate_keystrokes("f f");
        view.update_in(cx, |view, window, cx| {
            assert_eq!(view.buffer.bytes, [255, 0, 255, 0]);
            view.find_next(&FindNext, window, cx);
            assert_eq!(view.selection(), 0..1);
            view.find_next(&FindNext, window, cx);
            assert_eq!(view.selection(), 2..3);
            view.find_previous(&FindPrevious, window, cx);
            assert_eq!(view.selection(), 0..1);
            let inputs = view.inputs.as_ref().expect("tool fields");
            inputs
                .replacement
                .update(cx, |input, cx| input.set_text("42 43", window, cx));
            view.replace_match(false, cx);
            assert_eq!(view.buffer.bytes, [0x42, 0x43, 0, 255, 0]);
            view.replace_match(true, cx);
            assert_eq!(view.buffer.bytes, [0x42, 0x43, 0, 0x42, 0x43, 0]);
            view.undo(&Undo, window, cx);
            view.undo(&Undo, window, cx);
            assert_eq!(view.buffer.bytes, [255, 0, 255, 0]);
            assert!(!view.buffer.is_dirty());
            view.move_to(0, false, cx);
            view.move_to(3, true, cx);
            view.fill(&Fill, window, cx);
            view.inputs
                .as_ref()
                .expect("tool fields")
                .fill
                .update(cx, |input, cx| input.set_text("AA BB", window, cx));
            view.fill_selection(cx);
            assert_eq!(view.buffer.bytes, [0xaa, 0xbb, 0xaa, 0]);
            assert_eq!(view.selection(), 0..3);
            view.undo(&Undo, window, cx);
            assert!(!view.buffer.is_dirty());
            view.go_to_offset(&GoToOffset, window, cx);
            view.inputs
                .as_ref()
                .expect("tool fields")
                .offset
                .update(cx, |input, cx| input.set_text("0x4", window, cx));
            view.jump_to_offset(window, cx);
            assert_eq!(view.cursor, 4);
            view.inputs
                .as_ref()
                .expect("tool fields")
                .offset
                .update(cx, |input, cx| input.set_text("5", window, cx));
            view.jump_to_offset(window, cx);
            assert_eq!(view.cursor, 4);
            assert!(view.error.is_some());
            view.busy = true;
            view.fill_selection(cx);
            view.replace_match(true, cx);
            assert_eq!(view.buffer.bytes, [255, 0, 255, 0]);
            view.busy = false;
            view.find(&Find, window, cx);
        });
        cx.simulate_keystrokes("escape");
        view.update_in(cx, |view, window, _| {
            assert!(view.active_tool.is_none());
            assert!(view.focus_handle.is_focused(window));
        });
    }

    #[gpui::test]
    async fn test_hex_editor_keyboard_and_save(cx: &mut gpui::TestAppContext) {
        use fs::Fs;
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/data.bin"));
        fs.insert_file(&path, vec![0xff, 0x80, 0]).await;
        let project =
            project::Project::test(fs.clone(), [path.parent().expect("parent")], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| {
            let view = HexEditor::new(path.clone(), project.clone(), vec![0xff, 0x80, 0], cx);
            view.focus_handle.focus(window, cx);
            view
        });
        cx.simulate_keystrokes("4 2 right a b");
        view.read_with(cx, |view, _| {
            assert_eq!(view.buffer.bytes, [0x42, 0x80, 0xab]);
            assert_eq!(view.cursor, 3);
            assert!(view.buffer.is_dirty());
        });
        view.update_in(cx, |view, window, cx| {
            view.save(SaveOptions::default(), project.clone(), window, cx)
        })
        .await
        .expect("save bytes");
        assert_eq!(
            fs.load_bytes(&path).await.expect("load bytes"),
            [0x42, 0x80, 0xab]
        );
        assert!(!view.read_with(cx, |view, _| view.buffer.is_dirty()));

        cx.simulate_keystrokes("end 0 1");
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            [0x42, 0x80, 0xab, 1]
        );
        view.update_in(cx, |view, window, cx| {
            view.undo(&Undo, window, cx);
            view.undo(&Undo, window, cx);
        });
        assert!(!view.read_with(cx, |view, _| view.buffer.is_dirty()));
    }

    #[gpui::test(iterations = 10)]
    async fn test_hex_editor_refuses_external_changes(cx: &mut gpui::TestAppContext) {
        use fs::Fs;
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/data.bin"));
        fs.insert_file(&path, vec![0xff]).await;
        let project =
            project::Project::test(fs.clone(), [path.parent().expect("parent")], cx).await;
        let (view, cx) = cx
            .add_window_view(|_, cx| HexEditor::new(path.clone(), project.clone(), vec![0xff], cx));
        view.update(cx, |view, cx| view.input_digit(0, cx));
        fs.write(&path, &[0x42]).await.expect("external write");
        let result = view
            .update_in(cx, |view, window, cx| {
                view.save(SaveOptions::default(), project.clone(), window, cx)
            })
            .await;
        assert!(result.is_err());
        assert_eq!(fs.load_bytes(&path).await.expect("load bytes"), [0x42]);
        assert!(view.read_with(cx, |view, _| view.buffer.is_dirty()));
        assert!(!view.read_with(cx, |view, _| view.busy));
        view.update_in(cx, |view, window, cx| {
            view.reload(project.clone(), window, cx)
        })
        .await
        .expect("reload bytes");
        view.read_with(cx, |view, _| {
            assert_eq!(view.buffer.bytes, [0x42]);
            assert!(!view.buffer.is_dirty());
            assert!(view.buffer.undo.is_empty());
        });
    }

    #[gpui::test]
    async fn test_hex_editor_insert_selection_and_paste(cx: &mut gpui::TestAppContext) {
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/data.bin"));
        fs.insert_file(&path, vec![0xff, 0x80]).await;
        let project = project::Project::test(fs, [path.parent().expect("parent")], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| {
            let view = HexEditor::new(path, project, vec![0xff, 0x80], cx);
            view.focus_handle.focus(window, cx);
            view
        });
        view.update_in(cx, |view, window, cx| {
            view.toggle_insert(&ToggleInsertMode, window, cx);
        });
        cx.simulate_keystrokes("a b");
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            [0xab, 0xff, 0x80]
        );
        cx.simulate_keystrokes("shift-right backspace");
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            [0xab, 0x80]
        );
        view.update_in(cx, |view, window, cx| {
            view.select_all(&SelectAll, window, cx);
            cx.write_to_clipboard(ClipboardItem::new_string("00 FF 42".into()));
            view.paste(&Paste, window, cx);
        });
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            [0, 0xff, 0x42]
        );
        view.update_in(cx, |view, window, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string("not hex".into()));
            view.paste(&Paste, window, cx);
        });
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            [0, 0xff, 0x42]
        );
        assert!(view.read_with(cx, |view, _| view.error.is_some()));
    }

    #[gpui::test]
    async fn test_open_file_picker_binary_empty_cancel_and_reuse(cx: &mut gpui::TestAppContext) {
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/data.bin"));
        let empty_path = PathBuf::from(path!("/project/empty.bin"));
        fs.insert_file(&path, vec![0xff, 0x80]).await;
        fs.insert_file(&empty_path, Vec::new()).await;
        let project = project::Project::test(fs, [path.parent().expect("parent")], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

        workspace.update_in(cx, |workspace, window, cx| {
            open_file(workspace, &OpenFile, window, cx);
        });
        cx.simulate_path_prompt_response(|options| {
            assert!(options.files);
            assert!(!options.directories);
            assert!(!options.multiple);
            None
        });
        cx.run_until_parked();
        assert!(workspace.read_with(cx, |workspace, cx| workspace.active_item(cx).is_none()));

        for _ in 0..2 {
            workspace.update_in(cx, |workspace, window, cx| {
                open_file(workspace, &OpenFile, window, cx);
            });
            cx.simulate_path_prompt_response(|_| Some(vec![path.clone()]));
            cx.run_until_parked();
            workspace.read_with(cx, |workspace, cx| {
                assert_eq!(workspace.items_of_type::<HexEditor>(cx).count(), 1);
                assert_eq!(
                    workspace
                        .active_item_as::<HexEditor>(cx)
                        .expect("hex editor")
                        .read(cx)
                        .buffer
                        .bytes,
                    [0xff, 0x80]
                );
            });
        }

        workspace.update_in(cx, |workspace, window, cx| {
            open_file(workspace, &OpenFile, window, cx);
        });
        cx.simulate_path_prompt_response(|_| Some(vec![empty_path]));
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items_of_type::<HexEditor>(cx).count(), 2);
            assert!(
                workspace
                    .active_item_as::<HexEditor>(cx)
                    .expect("empty hex editor")
                    .read(cx)
                    .buffer
                    .bytes
                    .is_empty()
            );
        });
    }

    #[gpui::test]
    async fn test_open_file_refuses_unsaved_text_edits(cx: &mut gpui::TestAppContext) {
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/data.txt"));
        fs.insert_file(&path, b"saved".to_vec()).await;
        let project = project::Project::test(fs, [path.parent().expect("parent")], cx).await;
        let project_path = project.read_with(cx, |project, cx| {
            project.find_project_path(&path, cx).expect("project path")
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(project_path, None, true, window, cx)
            })
            .await
            .expect("open text editor");
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item_as::<crate::Editor>(cx)
                .expect("text editor")
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.insert("unsaved", window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            open_file(workspace, &OpenFile, window, cx);
        });
        cx.simulate_path_prompt_response(|_| Some(vec![path]));
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items_of_type::<HexEditor>(cx).count(), 0);
            assert_eq!(
                workspace.active_item_as::<crate::Editor>(cx),
                Some(editor.clone())
            );
            assert!(editor.read(cx).is_dirty(cx));
        });
    }

    #[gpui::test]
    async fn test_binary_files_open_as_hex_and_reuse_dirty_view(cx: &mut gpui::TestAppContext) {
        use fs::Fs as _;
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/data.bin"));
        let bytes: Vec<u8> = (0..=255).collect();
        fs.insert_file(&path, bytes.clone()).await;
        let project =
            project::Project::test(fs.clone(), [path.parent().expect("parent")], cx).await;
        let project_path = project.read_with(cx, |project, cx| {
            project.find_project_path(&path, cx).expect("project path")
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let item = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(project_path.clone(), None, true, window, cx)
            })
            .await
            .expect("open binary file");
        let view = item.downcast::<HexEditor>().expect("automatic hex editor");
        assert_eq!(
            view.read_with(cx, |view, _| view.buffer.bytes.clone()),
            bytes
        );
        view.update(cx, |view, cx| view.input_digit(4, cx));

        let reopened = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(project_path, None, true, window, cx)
            })
            .await
            .expect("reopen binary file");
        assert_eq!(reopened.downcast::<HexEditor>(), Some(view.clone()));
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items_of_type::<HexEditor>(cx).count(), 1);
            assert_eq!(workspace.items_of_type::<InvalidItemView>(cx).count(), 0);
            assert!(view.read(cx).is_dirty(cx));
            assert_eq!(view.read(cx).buffer.bytes[0], 0x40);
            assert_eq!(view.read(cx).buffer_kind(cx), ItemBufferKind::Singleton);
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.save_active_item(workspace::SaveIntent::Save, window, cx)
            })
            .await
            .expect("save automatically opened binary");
        let mut expected = bytes;
        expected[0] = 0x40;
        assert_eq!(fs.load_bytes(&path).await.expect("saved bytes"), expected);
        assert!(!view.read_with(cx, |view, cx| view.is_dirty(cx)));
    }

    #[gpui::test]
    async fn test_oversized_binary_keeps_error_view(cx: &mut gpui::TestAppContext) {
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/large.bin"));
        fs.insert_file(&path, vec![0; MAX_FILE_SIZE + 1]).await;
        let project = project::Project::test(fs, [path.parent().expect("parent")], cx).await;
        let project_path = project.read_with(cx, |project, cx| {
            project.find_project_path(&path, cx).expect("project path")
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        let item = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(project_path, None, true, window, cx)
            })
            .await
            .expect("open error view");
        let invalid = item.downcast::<InvalidItemView>().expect("file size error");
        assert!(invalid.read_with(cx, |invalid, _| invalid.error.contains("64 MiB")));
        assert_eq!(
            workspace.read_with(cx, |workspace, cx| {
                workspace.items_of_type::<HexEditor>(cx).count()
            }),
            0
        );
    }

    #[gpui::test]
    async fn test_text_and_encoded_files_keep_text_editor(cx: &mut gpui::TestAppContext) {
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), serde_json::json!({}))
            .await;
        let root = PathBuf::from(path!("/project"));
        let cases = [
            ("text.bin", b"plain text\n".to_vec()),
            ("empty.bin", Vec::new()),
            ("utf16.bin", vec![0xff, 0xfe, b'h', 0, b'i', 0]),
            (
                "utf16_no_bom.bin",
                vec![b'h', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0],
            ),
            ("legacy.bin", b"caf\xe9\n".to_vec()),
        ];
        for (name, bytes) in &cases {
            fs.insert_file(root.join(name), bytes.clone()).await;
        }
        let project = project::Project::test(fs, [root.as_path()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        for (name, _) in cases {
            let project_path = project.read_with(cx, |project, cx| {
                project
                    .find_project_path(&root.join(name), cx)
                    .expect("project path")
            });
            let item = workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.open_path(project_path, None, true, window, cx)
                })
                .await
                .expect("open text file");
            assert!(item.downcast::<crate::Editor>().is_some(), "{name}");
        }
        assert_eq!(
            workspace.read_with(cx, |workspace, cx| {
                workspace.items_of_type::<HexEditor>(cx).count()
            }),
            0
        );
    }

    #[gpui::test]
    async fn test_open_binary_error_tab_and_reuse_view(cx: &mut gpui::TestAppContext) {
        use util::path;

        cx.update(|cx| {
            workspace::AppState::test(cx);
            crate::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(util::path!("/project"), serde_json::json!({}))
            .await;
        let path = PathBuf::from(path!("/project/data.bin"));
        fs.insert_file(&path, vec![0xff, 0x80]).await;
        let project = project::Project::test(fs, [path.parent().expect("parent")], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
        workspace.update_in(cx, |workspace, window, cx| {
            let invalid = cx
                .new(|cx| InvalidItemView::new(&path, true, &anyhow!("Invalid UTF-8"), window, cx));
            workspace.add_item_to_center(Box::new(invalid), window, cx);
            open(workspace, &Open, window, cx);
        });
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items_of_type::<HexEditor>(cx).count(), 1);
            let view = workspace
                .active_item_as::<HexEditor>(cx)
                .expect("hex editor");
            assert_eq!(view.read(cx).buffer.bytes, [0xff, 0x80]);
        });
        workspace.update_in(cx, |workspace, window, cx| {
            open(workspace, &Open, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            workspace.read_with(cx, |workspace, cx| {
                workspace.items_of_type::<HexEditor>(cx).count()
            }),
            1
        );
    }
}
