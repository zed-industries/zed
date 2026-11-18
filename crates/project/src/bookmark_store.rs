use std::{
    collections::BTreeMap,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{StreamExt, TryFutureExt, stream::FuturesUnordered};
use gpui::{App, AppContext, Context, Entity, EntityId, EventEmitter, Subscription, Task};
use itertools::Itertools;
use language::{Buffer, BufferEvent};
use serde::{Deserialize, Serialize};
use text::{Bias, BufferSnapshot, Point};
use worktree::PathChange;

use crate::{
    ProjectPath,
    buffer_store::BufferStore,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub const NUMBERED_BOOKMARK_COUNT: usize = 10;
const UNDEFINED_NUMBERED_BOOKMARK_LINE: i32 = -1;
const VSCODE_NUMBERED_BOOKMARKS_DIR: &str = ".vscode";
const VSCODE_NUMBERED_BOOKMARKS_FILE: &str = "numbered-bookmarks.json";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BookmarkStoreEvent {
    BookmarksChanged,
    LabelChanged,
}

#[derive(Clone, Debug)]
pub struct Bookmark {
    pub anchor: text::Anchor,
    pub label: String,
}

pub struct ProjectBookmark {
    pub path: Arc<Path>,
    pub label: String,
    pub buffer: Entity<Buffer>,
    pub anchor: text::Anchor,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerializedBookmark {
    pub row: u32,
    pub label: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerializedNumberedBookmark {
    pub number: u8,
    pub row: u32,
    pub column: u32,
}

#[derive(Clone, Debug)]
pub struct NumberedBookmark {
    pub number: u8,
    pub anchor: text::Anchor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NumberedBookmarksProjectFile {
    #[serde(default)]
    pub files: Vec<NumberedBookmarksProjectFileEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NumberedBookmarksProjectFileEntry {
    pub path: String,
    #[serde(default)]
    pub bookmarks: Vec<NumberedBookmarkPosition>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NumberedBookmarkPosition {
    pub line: i32,
    pub column: i32,
}

#[derive(Debug)]
pub struct BufferBookmarks {
    buffer: Entity<Buffer>,
    bookmarks: Vec<Bookmark>,
    numbered: [Option<text::Anchor>; NUMBERED_BOOKMARK_COUNT],
    _subscription: Subscription,
}

impl BufferBookmarks {
    pub fn new(buffer: Entity<Buffer>, cx: &mut Context<BookmarkStore>) -> Self {
        let subscription = cx.subscribe(
            &buffer,
            |bookmark_store, buffer, event: &BufferEvent, cx| match event {
                BufferEvent::FileHandleChanged => {
                    bookmark_store.handle_file_changed(buffer, cx);
                }
                _ => {}
            },
        );

        Self {
            buffer,
            bookmarks: Vec::new(),
            numbered: std::array::from_fn(|_| None),
            _subscription: subscription,
        }
    }

    pub fn buffer(&self) -> &Entity<Buffer> {
        &self.buffer
    }

    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    pub fn numbered(&self) -> &[Option<text::Anchor>; NUMBERED_BOOKMARK_COUNT] {
        &self.numbered
    }

    fn has_numbered(&self) -> bool {
        self.numbered.iter().any(Option::is_some)
    }
}

/// Numbered bookmarks of a buffer that has no file yet. They aren't persisted, and
/// move to the file's entry once the buffer is saved.
struct UntitledNumberedBookmarks {
    numbered: [Option<text::Anchor>; NUMBERED_BOOKMARK_COUNT],
    _subscriptions: [Subscription; 2],
}

impl UntitledNumberedBookmarks {
    fn new(buffer: &Entity<Buffer>, cx: &mut Context<BookmarkStore>) -> Self {
        let file_changed = cx.subscribe(buffer, |bookmark_store, buffer, event, cx| {
            if let BufferEvent::FileHandleChanged = event {
                bookmark_store.handle_untitled_file_changed(buffer, cx);
            }
        });
        let buffer_id = buffer.entity_id();
        let released = cx.observe_release(buffer, move |bookmark_store, _, cx| {
            if bookmark_store
                .untitled_numbered
                .remove(&buffer_id)
                .is_some()
            {
                BookmarkStore::emit_bookmarks_changed(cx);
            }
        });
        Self {
            numbered: std::array::from_fn(|_| None),
            _subscriptions: [file_changed, released],
        }
    }
}

#[derive(Debug)]
pub enum BookmarkEntry {
    Loaded(BufferBookmarks),
    Unloaded {
        bookmarks: Vec<SerializedBookmark>,
        numbered: Vec<SerializedNumberedBookmark>,
    },
}

impl BookmarkEntry {
    pub fn is_empty(&self) -> bool {
        match self {
            BookmarkEntry::Loaded(buffer_bookmarks) => {
                buffer_bookmarks.bookmarks.is_empty() && !buffer_bookmarks.has_numbered()
            }
            BookmarkEntry::Unloaded {
                bookmarks,
                numbered,
            } => bookmarks.is_empty() && numbered.is_empty(),
        }
    }

    fn loaded(&self) -> Option<&BufferBookmarks> {
        match self {
            BookmarkEntry::Loaded(buffer_bookmarks) => Some(buffer_bookmarks),
            BookmarkEntry::Unloaded { .. } => None,
        }
    }
}

pub struct BookmarkStore {
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    bookmarks: BTreeMap<Arc<Path>, BookmarkEntry>,
    untitled_numbered: HashMap<EntityId, UntitledNumberedBookmarks>,
    paths_failed_to_open: HashSet<Arc<Path>>,
    _worktree_store_subscription: Subscription,
}

impl EventEmitter<BookmarkStoreEvent> for BookmarkStore {}

impl BookmarkStore {
    pub fn new(
        worktree_store: Entity<WorktreeStore>,
        buffer_store: Entity<BufferStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let worktree_store_subscription =
            cx.subscribe(&worktree_store, Self::handle_worktree_store_event);
        Self {
            buffer_store,
            worktree_store,
            bookmarks: BTreeMap::new(),
            untitled_numbered: HashMap::default(),
            paths_failed_to_open: HashSet::default(),
            _worktree_store_subscription: worktree_store_subscription,
        }
    }

    fn handle_worktree_store_event(
        &mut self,
        _worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        if self.paths_failed_to_open.is_empty() {
            return;
        }
        match event {
            WorktreeStoreEvent::WorktreeAdded(worktree) => {
                let worktree_abs_path = worktree.read(cx).abs_path();
                let previous_len = self.paths_failed_to_open.len();
                self.paths_failed_to_open
                    .retain(|path| !path.starts_with(&worktree_abs_path));
                if self.paths_failed_to_open.len() < previous_len {
                    Self::emit_bookmarks_changed(cx);
                }
            }
            WorktreeStoreEvent::WorktreeUpdatedEntries(worktree_id, entries) => {
                let Some(worktree) = self
                    .worktree_store
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                else {
                    return;
                };
                let worktree_abs_path = worktree.read(cx).abs_path();
                let failed_by_rel_path = self
                    .paths_failed_to_open
                    .iter()
                    .filter_map(|abs_path| {
                        abs_path
                            .strip_prefix(&worktree_abs_path)
                            .ok()
                            .map(|rel_path| (rel_path.to_owned(), abs_path.clone()))
                    })
                    .collect::<HashMap<_, _>>();
                if failed_by_rel_path.is_empty() {
                    return;
                }
                let mut removed_any = false;
                for (rel_path, _, change) in entries.iter() {
                    if *change == PathChange::Removed {
                        continue;
                    }
                    if let Some(abs_path) = failed_by_rel_path.get(rel_path.as_std_path()) {
                        removed_any |= self.paths_failed_to_open.remove(abs_path.as_ref());
                    }
                }
                if removed_any {
                    Self::emit_bookmarks_changed(cx);
                }
            }
            _ => {}
        }
    }

    fn emit_bookmarks_changed(cx: &mut Context<Self>) {
        cx.emit(BookmarkStoreEvent::BookmarksChanged);
        cx.notify();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn paths_failed_to_open(&self) -> &HashSet<Arc<Path>> {
        &self.paths_failed_to_open
    }

    pub fn forget_failed_paths(&mut self, cx: &mut Context<Self>) {
        if self.paths_failed_to_open.is_empty() {
            return;
        }
        self.paths_failed_to_open.clear();
        Self::emit_bookmarks_changed(cx);
    }

    pub fn is_empty(&self) -> bool {
        self.bookmarks.values().all(BookmarkEntry::is_empty)
    }

    pub fn load_serialized_bookmarks(
        &mut self,
        bookmark_rows: BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.bookmarks.clear();
        self.paths_failed_to_open.clear();

        for (path, rows) in bookmark_rows {
            if rows.is_empty() {
                continue;
            }

            let count = rows.len();
            log::debug!("Stored {count} unloaded bookmark(s) at {}", path.display());

            self.bookmarks.insert(
                path,
                BookmarkEntry::Unloaded {
                    bookmarks: rows,
                    numbered: Vec::new(),
                },
            );
        }

        Self::emit_bookmarks_changed(cx);
        Task::ready(Ok(()))
    }

    pub fn load_serialized_numbered_bookmarks(
        &mut self,
        numbered_bookmarks: BTreeMap<Arc<Path>, Vec<SerializedNumberedBookmark>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.apply_serialized_numbered_bookmarks(numbered_bookmarks, cx);
        Task::ready(Ok(()))
    }

    fn apply_serialized_numbered_bookmarks(
        &mut self,
        numbered_bookmarks: BTreeMap<Arc<Path>, Vec<SerializedNumberedBookmark>>,
        cx: &mut Context<Self>,
    ) {
        for (path, numbered) in numbered_bookmarks {
            if numbered.is_empty() {
                continue;
            }

            match self.bookmarks.get_mut(&path) {
                Some(BookmarkEntry::Unloaded {
                    numbered: existing, ..
                }) => {
                    *existing = numbered;
                }
                Some(BookmarkEntry::Loaded(buffer_bookmarks)) => {
                    let snapshot = buffer_bookmarks.buffer.read(cx).snapshot();
                    buffer_bookmarks.numbered =
                        Self::resolve_numbered_slots(&numbered, &snapshot, &path);
                }
                None => {
                    self.bookmarks.insert(
                        path,
                        BookmarkEntry::Unloaded {
                            bookmarks: Vec::new(),
                            numbered,
                        },
                    );
                }
            }
        }

        Self::emit_bookmarks_changed(cx);
    }

    fn resolve_numbered_slots(
        numbered: &[SerializedNumberedBookmark],
        snapshot: &BufferSnapshot,
        abs_path: &Path,
    ) -> [Option<text::Anchor>; NUMBERED_BOOKMARK_COUNT] {
        let max_point = snapshot.max_point();
        let mut slots = std::array::from_fn(|_| None);
        for bookmark in numbered {
            let Some(index) = numbered_index(bookmark.number) else {
                continue;
            };
            if bookmark.row > max_point.row {
                log::warn!(
                    "Skipping out-of-range numbered bookmark {}: {} row {} (file has {} rows)",
                    bookmark.number,
                    abs_path.display(),
                    bookmark.row,
                    max_point.row
                );
                continue;
            }
            let line_len = snapshot.line_len(bookmark.row);
            let column = bookmark.column.min(line_len);
            let point = snapshot.clip_point(Point::new(bookmark.row, column), Bias::Left);
            slots[index] = Some(snapshot.anchor_after(point));
        }
        slots
    }

    fn resolve_anchors_if_needed(
        &mut self,
        abs_path: &Arc<Path>,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let Some(BookmarkEntry::Unloaded {
            bookmarks,
            numbered,
        }) = self.bookmarks.get(abs_path)
        else {
            return;
        };

        let unresolved_count = bookmarks.len() + numbered.len();
        let snapshot = buffer.read(cx).snapshot();
        let max_point = snapshot.max_point();

        let bookmarks: Vec<Bookmark> = bookmarks
            .iter()
            .filter_map(|bookmark| {
                let point = Point::new(bookmark.row, 0);

                if point > max_point {
                    log::warn!(
                        "Skipping out-of-range bookmark: {} row {} (file has {} rows)",
                        abs_path.display(),
                        bookmark.row,
                        max_point.row
                    );
                    return None;
                }

                let anchor = snapshot.anchor_after(point);
                Some(Bookmark {
                    anchor,
                    label: bookmark.label.clone(),
                })
            })
            .collect();

        let numbered_slots = Self::resolve_numbered_slots(numbered, &snapshot, abs_path);
        let resolved_numbered = numbered_slots.iter().filter(|slot| slot.is_some()).count();
        let was_failed_to_open = self.paths_failed_to_open.remove(abs_path);

        let resolved_count = bookmarks.len() + resolved_numbered;
        if bookmarks.is_empty() && resolved_numbered == 0 {
            self.bookmarks.remove(abs_path);
        } else {
            let mut buffer_bookmarks = BufferBookmarks::new(buffer.clone(), cx);
            buffer_bookmarks.bookmarks = bookmarks;
            buffer_bookmarks.numbered = numbered_slots;
            self.bookmarks
                .insert(abs_path.clone(), BookmarkEntry::Loaded(buffer_bookmarks));
        }

        if was_failed_to_open || resolved_count < unresolved_count {
            Self::emit_bookmarks_changed(cx);
        }
    }

    pub fn abs_path_from_buffer(buffer: &Entity<Buffer>, cx: &App) -> Option<Arc<Path>> {
        worktree::File::from_dyn(buffer.read(cx).file())
            .map(|file| file.worktree.read(cx).absolutize(&file.path))
            .map(Arc::<Path>::from)
    }

    /// Toggle a bookmark at the given anchor in the buffer.
    /// If a bookmark already exists on the same row, it will be removed.
    /// Otherwise, a new bookmark will be added with the given label.
    pub fn toggle_bookmark(
        &mut self,
        buffer: Entity<Buffer>,
        anchor: text::Anchor,
        label: String,
        cx: &mut Context<Self>,
    ) {
        let Some(abs_path) = Self::abs_path_from_buffer(&buffer, cx) else {
            return;
        };

        self.resolve_anchors_if_needed(&abs_path, &buffer, cx);

        let entry = self
            .bookmarks
            .entry(abs_path.clone())
            .or_insert_with(|| BookmarkEntry::Loaded(BufferBookmarks::new(buffer.clone(), cx)));

        let BookmarkEntry::Loaded(buffer_bookmarks) = entry else {
            unreachable!("resolve_if_needed should have converted to Loaded");
        };

        // Overlapping worktrees can open the same path in different buffers.
        if buffer_bookmarks.buffer != buffer {
            return;
        }

        let snapshot = buffer.read(cx).text_snapshot();

        let existing_index = buffer_bookmarks.bookmarks.iter().position(|existing| {
            existing.anchor.summary::<Point>(&snapshot).row
                == anchor.summary::<Point>(&snapshot).row
        });

        if let Some(index) = existing_index {
            buffer_bookmarks.bookmarks.remove(index);
            if buffer_bookmarks.bookmarks.is_empty() && !buffer_bookmarks.has_numbered() {
                self.bookmarks.remove(&abs_path);
            }
        } else {
            buffer_bookmarks.bookmarks.push(Bookmark { anchor, label });
        }

        Self::emit_bookmarks_changed(cx);
    }

    pub fn find_bookmark(
        &mut self,
        buffer: &Entity<Buffer>,
        anchor: text::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<&Bookmark> {
        let abs_path = Self::abs_path_from_buffer(buffer, cx)?;

        self.resolve_anchors_if_needed(&abs_path, buffer, cx);

        let BookmarkEntry::Loaded(buffer_bookmarks) = self.bookmarks.get(&abs_path)? else {
            return None;
        };

        if &buffer_bookmarks.buffer != buffer {
            return None;
        }

        let snapshot = buffer.read(cx).text_snapshot();

        buffer_bookmarks.bookmarks.iter().find(|existing| {
            existing.anchor.summary::<Point>(&snapshot).row
                == anchor.summary::<Point>(&snapshot).row
        })
    }

    pub fn edit_bookmark(
        &mut self,
        buffer: &Entity<Buffer>,
        anchor: text::Anchor,
        label: String,
        cx: &mut Context<Self>,
    ) {
        let Some(abs_path) = Self::abs_path_from_buffer(buffer, cx) else {
            return;
        };

        self.resolve_anchors_if_needed(&abs_path, buffer, cx);

        let Some(BookmarkEntry::Loaded(buffer_bookmarks)) = self.bookmarks.get_mut(&abs_path)
        else {
            return;
        };

        if &buffer_bookmarks.buffer != buffer {
            return;
        }

        let snapshot = buffer.read(cx).text_snapshot();
        let row = anchor.summary::<Point>(&snapshot).row;

        if let Some(bookmark) = buffer_bookmarks
            .bookmarks
            .iter_mut()
            .find(|existing| existing.anchor.summary::<Point>(&snapshot).row == row)
        {
            bookmark.label = label;
            cx.emit(BookmarkStoreEvent::LabelChanged);
            cx.notify();
        }
    }

    /// Returns the bookmarks for a given buffer within an optional range.
    /// Only returns bookmarks that have been resolved to anchors (loaded).
    /// Unloaded bookmarks for the given buffer will be resolved first.
    pub fn bookmarks_for_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        range: Range<text::Anchor>,
        buffer_snapshot: &BufferSnapshot,
        cx: &mut Context<Self>,
    ) -> Vec<Bookmark> {
        let Some(abs_path) = Self::abs_path_from_buffer(&buffer, cx) else {
            return Vec::new();
        };

        self.resolve_anchors_if_needed(&abs_path, &buffer, cx);

        let Some(BookmarkEntry::Loaded(file_bookmarks)) = self.bookmarks.get(&abs_path) else {
            return Vec::new();
        };

        if file_bookmarks.buffer != buffer {
            return Vec::new();
        }

        file_bookmarks
            .bookmarks
            .iter()
            .filter_map({
                move |bookmark| {
                    if !buffer_snapshot.can_resolve(&bookmark.anchor) {
                        return None;
                    }

                    if bookmark.anchor.cmp(&range.start, buffer_snapshot).is_lt()
                        || bookmark.anchor.cmp(&range.end, buffer_snapshot).is_gt()
                    {
                        return None;
                    }

                    Some(bookmark.clone())
                }
            })
            .collect()
    }

    pub fn toggle_numbered_bookmark(
        &mut self,
        buffer: Entity<Buffer>,
        anchor: text::Anchor,
        number: u8,
        unique_across_files: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = numbered_index(number) else {
            return;
        };
        let Some(abs_path) = Self::abs_path_from_buffer(&buffer, cx) else {
            self.toggle_untitled_numbered_bookmark(buffer, anchor, index, unique_across_files, cx);
            return;
        };

        self.resolve_anchors_if_needed(&abs_path, &buffer, cx);

        let snapshot = buffer.read(cx).text_snapshot();
        let row = anchor.summary::<Point>(&snapshot).row;
        let should_set = {
            let entry = self
                .bookmarks
                .entry(abs_path.clone())
                .or_insert_with(|| BookmarkEntry::Loaded(BufferBookmarks::new(buffer.clone(), cx)));

            let BookmarkEntry::Loaded(buffer_bookmarks) = entry else {
                unreachable!("resolve_if_needed should have converted to Loaded");
            };

            let existing_number = buffer_bookmarks.numbered.iter().position(|slot| {
                slot.is_some_and(|existing| existing.summary::<Point>(&snapshot).row == row)
            });

            if let Some(existing_index) = existing_number {
                buffer_bookmarks.numbered[existing_index] = None;
            }

            let should_set = existing_number != Some(index);
            if should_set {
                buffer_bookmarks.numbered[index] = Some(anchor);
            }
            should_set
        };

        self.remove_entry_if_empty(&abs_path);

        if should_set && unique_across_files {
            let paths_to_clear: Vec<Arc<Path>> = self
                .bookmarks
                .keys()
                .filter(|path| path.as_ref() != abs_path.as_ref())
                .cloned()
                .collect();
            for path in paths_to_clear {
                self.clear_numbered_bookmark_at_path(&path, index);
            }
            self.clear_untitled_numbered_bookmark(None, index);
        }

        Self::emit_bookmarks_changed(cx);
    }

    fn toggle_untitled_numbered_bookmark(
        &mut self,
        buffer: Entity<Buffer>,
        anchor: text::Anchor,
        index: usize,
        unique_across_files: bool,
        cx: &mut Context<Self>,
    ) {
        if buffer.read(cx).file().is_some() {
            return;
        }
        let buffer_id = buffer.entity_id();
        let snapshot = buffer.read(cx).text_snapshot();
        let row = anchor.summary::<Point>(&snapshot).row;
        let entry = match self.untitled_numbered.entry(buffer_id) {
            collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(UntitledNumberedBookmarks::new(&buffer, cx))
            }
        };
        let existing_number = entry.numbered.iter().position(|slot| {
            slot.is_some_and(|existing| existing.summary::<Point>(&snapshot).row == row)
        });
        if let Some(existing_index) = existing_number {
            entry.numbered[existing_index] = None;
        }
        let should_set = existing_number != Some(index);
        if should_set {
            entry.numbered[index] = Some(anchor);
        }
        if !entry.numbered.iter().any(Option::is_some) {
            self.untitled_numbered.remove(&buffer_id);
        }

        if should_set && unique_across_files {
            let paths: Vec<Arc<Path>> = self.bookmarks.keys().cloned().collect();
            for path in paths {
                self.clear_numbered_bookmark_at_path(&path, index);
            }
            self.clear_untitled_numbered_bookmark(Some(buffer_id), index);
        }

        Self::emit_bookmarks_changed(cx);
    }

    fn clear_untitled_numbered_bookmark(&mut self, except: Option<EntityId>, index: usize) {
        self.untitled_numbered.retain(|buffer_id, entry| {
            if except != Some(*buffer_id) {
                entry.numbered[index] = None;
            }
            entry.numbered.iter().any(Option::is_some)
        });
    }

    fn untitled_numbered_slots(
        &self,
        buffer: &Entity<Buffer>,
    ) -> Option<&[Option<text::Anchor>; NUMBERED_BOOKMARK_COUNT]> {
        self.untitled_numbered
            .get(&buffer.entity_id())
            .map(|entry| &entry.numbered)
    }

    fn handle_untitled_file_changed(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let Some(abs_path) = Self::abs_path_from_buffer(&buffer, cx) else {
            return;
        };
        let Some(untitled) = self.untitled_numbered.remove(&buffer.entity_id()) else {
            return;
        };
        self.resolve_anchors_if_needed(&abs_path, &buffer, cx);
        let entry = self
            .bookmarks
            .entry(abs_path)
            .or_insert_with(|| BookmarkEntry::Loaded(BufferBookmarks::new(buffer.clone(), cx)));
        if let BookmarkEntry::Loaded(buffer_bookmarks) = entry {
            for (slot, untitled_slot) in buffer_bookmarks.numbered.iter_mut().zip(untitled.numbered)
            {
                if slot.is_none() {
                    *slot = untitled_slot;
                }
            }
        }
        Self::emit_bookmarks_changed(cx);
    }

    fn clear_numbered_bookmark_at_path(&mut self, path: &Arc<Path>, index: usize) {
        match self.bookmarks.get_mut(path) {
            Some(BookmarkEntry::Loaded(buffer_bookmarks)) => {
                buffer_bookmarks.numbered[index] = None;
            }
            Some(BookmarkEntry::Unloaded { numbered, .. }) => {
                numbered.retain(|bookmark| numbered_index(bookmark.number) != Some(index));
            }
            None => {}
        }
        self.remove_entry_if_empty(path);
    }

    fn remove_entry_if_empty(&mut self, path: &Arc<Path>) {
        if self
            .bookmarks
            .get(path)
            .is_some_and(BookmarkEntry::is_empty)
        {
            self.bookmarks.remove(path);
        }
    }

    pub fn numbered_bookmark_on_row(
        &mut self,
        buffer: &Entity<Buffer>,
        row: u32,
        cx: &mut Context<Self>,
    ) -> Option<u8> {
        let snapshot = buffer.read(cx).text_snapshot();
        let numbered = match Self::abs_path_from_buffer(buffer, cx) {
            Some(abs_path) => {
                self.resolve_anchors_if_needed(&abs_path, buffer, cx);
                let BookmarkEntry::Loaded(buffer_bookmarks) = self.bookmarks.get(&abs_path)? else {
                    return None;
                };
                &buffer_bookmarks.numbered
            }
            None => self.untitled_numbered_slots(buffer)?,
        };
        numbered.iter().enumerate().find_map(|(index, slot)| {
            let anchor = (*slot)?;
            (anchor.summary::<Point>(&snapshot).row == row).then_some(index as u8)
        })
    }

    pub fn numbered_bookmark_in_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        number: u8,
        cx: &mut Context<Self>,
    ) -> Option<NumberedBookmark> {
        let index = numbered_index(number)?;
        let numbered = match Self::abs_path_from_buffer(buffer, cx) {
            Some(abs_path) => {
                self.resolve_anchors_if_needed(&abs_path, buffer, cx);
                let BookmarkEntry::Loaded(buffer_bookmarks) = self.bookmarks.get(&abs_path)? else {
                    return None;
                };
                &buffer_bookmarks.numbered
            }
            None => self.untitled_numbered_slots(buffer)?,
        };
        Some(NumberedBookmark {
            number,
            anchor: numbered[index]?,
        })
    }

    pub fn numbered_bookmarks_for_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        range: Range<text::Anchor>,
        buffer_snapshot: &BufferSnapshot,
        cx: &mut Context<Self>,
    ) -> Vec<NumberedBookmark> {
        let numbered = match Self::abs_path_from_buffer(&buffer, cx) {
            Some(abs_path) => {
                self.resolve_anchors_if_needed(&abs_path, &buffer, cx);
                let Some(BookmarkEntry::Loaded(file_bookmarks)) = self.bookmarks.get(&abs_path)
                else {
                    return Vec::new();
                };
                &file_bookmarks.numbered
            }
            None => {
                let Some(numbered) = self.untitled_numbered_slots(&buffer) else {
                    return Vec::new();
                };
                numbered
            }
        };

        numbered
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                let anchor = (*slot)?;
                if !buffer_snapshot.can_resolve(&anchor) {
                    return None;
                }
                if anchor.cmp(&range.start, buffer_snapshot).is_lt()
                    || anchor.cmp(&range.end, buffer_snapshot).is_gt()
                {
                    return None;
                }
                Some(NumberedBookmark {
                    number: index as u8,
                    anchor,
                })
            })
            .collect()
    }

    pub fn numbered_bookmark_locations(
        &self,
        cx: &App,
    ) -> Vec<(Arc<Path>, SerializedNumberedBookmark)> {
        let mut locations = Vec::new();
        for (path, numbered) in self.all_serialized_numbered_bookmarks(cx) {
            for bookmark in numbered {
                locations.push((path.clone(), bookmark));
            }
        }
        locations.sort_by(|(left_path, left), (right_path, right)| {
            left_path
                .cmp(right_path)
                .then(left.number.cmp(&right.number))
        });
        locations
    }

    pub fn line_text(&self, path: &Path, row: u32, cx: &App) -> String {
        let Some(BookmarkEntry::Loaded(buffer_bookmarks)) = self.bookmarks.get(path) else {
            return String::new();
        };
        let snapshot = buffer_bookmarks.buffer.read(cx).snapshot();
        if row > snapshot.max_point().row {
            return String::new();
        }
        snapshot
            .text_for_range(Point::new(row, 0)..Point::new(row, snapshot.line_len(row)))
            .collect::<String>()
            .trim()
            .to_string()
    }

    pub fn clear_numbered_bookmarks_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let Some(abs_path) = Self::abs_path_from_buffer(buffer, cx) else {
            if self.untitled_numbered.remove(&buffer.entity_id()).is_some() {
                Self::emit_bookmarks_changed(cx);
            }
            return;
        };

        self.resolve_anchors_if_needed(&abs_path, buffer, cx);

        match self.bookmarks.get_mut(&abs_path) {
            Some(BookmarkEntry::Loaded(buffer_bookmarks)) => {
                buffer_bookmarks.numbered = std::array::from_fn(|_| None);
            }
            Some(BookmarkEntry::Unloaded { numbered, .. }) => {
                numbered.clear();
            }
            None => return,
        }

        self.remove_entry_if_empty(&abs_path);
        Self::emit_bookmarks_changed(cx);
    }

    pub fn has_numbered_bookmarks(&self) -> bool {
        !self.untitled_numbered.is_empty()
            || self.bookmarks.values().any(|entry| match entry {
                BookmarkEntry::Loaded(buffer_bookmarks) => buffer_bookmarks.has_numbered(),
                BookmarkEntry::Unloaded { numbered, .. } => !numbered.is_empty(),
            })
    }

    pub fn has_numbered_bookmarks_in_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(abs_path) = Self::abs_path_from_buffer(buffer, cx) else {
            return self.untitled_numbered_slots(buffer).is_some();
        };
        self.resolve_anchors_if_needed(&abs_path, buffer, cx);
        match self.bookmarks.get(&abs_path) {
            Some(BookmarkEntry::Loaded(buffer_bookmarks)) => buffer_bookmarks.has_numbered(),
            Some(BookmarkEntry::Unloaded { numbered, .. }) => !numbered.is_empty(),
            None => false,
        }
    }

    fn handle_file_changed(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let entity_id = buffer.entity_id();

        if buffer
            .read(cx)
            .file()
            .is_none_or(|f| f.disk_state().is_deleted())
        {
            self.bookmarks.retain(|_, entry| match entry {
                BookmarkEntry::Loaded(buffer_bookmarks) => {
                    buffer_bookmarks.buffer.entity_id() != entity_id
                }
                BookmarkEntry::Unloaded { .. } => true,
            });
            Self::emit_bookmarks_changed(cx);
            return;
        }

        if let Some(new_abs_path) = Self::abs_path_from_buffer(&buffer, cx) {
            if self.bookmarks.contains_key(&new_abs_path) {
                return;
            }

            if let Some(old_path) = self
                .bookmarks
                .iter()
                .find(|(_, entry)| match entry {
                    BookmarkEntry::Loaded(buffer_bookmarks) => {
                        buffer_bookmarks.buffer.entity_id() == entity_id
                    }
                    BookmarkEntry::Unloaded { .. } => false,
                })
                .map(|(path, _)| path)
                .cloned()
            {
                let Some(entry) = self.bookmarks.remove(&old_path) else {
                    log::error!(
                        "Couldn't get bookmarks from old path during buffer rename handling"
                    );
                    return;
                };
                self.bookmarks.insert(new_abs_path, entry);
                Self::emit_bookmarks_changed(cx);
            }
        }
    }

    pub async fn all_bookmarks(
        this: &Entity<BookmarkStore>,
        cx: &mut (impl AppContext + Clone),
    ) -> Result<Vec<ProjectBookmark>> {
        Self::resolve_all(this, cx).await?;

        let bookmarks = cx.read_entity(this, |bookmark_store, cx| {
            let mut bookmarks = Vec::new();

            for (path, value) in &bookmark_store.bookmarks {
                let BookmarkEntry::Loaded(buffer_bookmarks) = value else {
                    continue;
                };

                let snapshot = buffer_bookmarks.buffer.read(cx).snapshot();
                for bookmark in &buffer_bookmarks.bookmarks {
                    let label = if bookmark.label.is_empty() {
                        let row = snapshot.summary_for_anchor::<Point>(&bookmark.anchor).row;
                        let line_length = snapshot.line_len(row);
                        let mut label = String::with_capacity(line_length as usize);

                        for chunk in snapshot
                            .text_for_range(Point::new(row, 0)..Point::new(row, line_length))
                        {
                            let chunk = if label.is_empty() {
                                chunk.trim_start()
                            } else {
                                chunk
                            };
                            label.push_str(chunk);
                        }

                        let trimmed_length = label.trim_end().len();
                        label.truncate(trimmed_length);
                        label
                    } else {
                        bookmark.label.clone()
                    };

                    bookmarks.push(ProjectBookmark {
                        path: path.clone(),
                        label,
                        buffer: buffer_bookmarks.buffer.clone(),
                        anchor: bookmark.anchor,
                    });
                }
            }

            bookmarks
        });
        Ok(bookmarks)
    }

    pub fn all_serialized_bookmarks(
        &self,
        cx: &App,
    ) -> BTreeMap<Arc<Path>, Vec<SerializedBookmark>> {
        self.bookmarks
            .iter()
            .filter_map(|(path, entry)| {
                let mut rows = match entry {
                    BookmarkEntry::Unloaded { bookmarks, .. } => bookmarks.clone(),
                    BookmarkEntry::Loaded(buffer_bookmarks) => {
                        let snapshot = buffer_bookmarks.buffer.read(cx).snapshot();
                        buffer_bookmarks
                            .bookmarks
                            .iter()
                            .filter_map(|bookmark| {
                                if !snapshot.can_resolve(&bookmark.anchor) {
                                    return None;
                                }
                                let row =
                                    snapshot.summary_for_anchor::<Point>(&bookmark.anchor).row;
                                Some(SerializedBookmark {
                                    row,
                                    label: bookmark.label.clone(),
                                })
                            })
                            .collect()
                    }
                };

                rows.sort_unstable_by_key(|a| a.row);
                rows.dedup_by_key(|a| a.row);

                if rows.is_empty() {
                    None
                } else {
                    Some((path.clone(), rows))
                }
            })
            .collect()
    }

    pub fn all_serialized_numbered_bookmarks(
        &self,
        cx: &App,
    ) -> BTreeMap<Arc<Path>, Vec<SerializedNumberedBookmark>> {
        self.bookmarks
            .iter()
            .filter_map(|(path, entry)| {
                let mut numbered = match entry {
                    BookmarkEntry::Unloaded { numbered, .. } => numbered.clone(),
                    BookmarkEntry::Loaded(buffer_bookmarks) => {
                        let snapshot = buffer_bookmarks.buffer.read(cx).snapshot();
                        buffer_bookmarks
                            .numbered
                            .iter()
                            .enumerate()
                            .filter_map(|(index, slot)| {
                                let anchor = (*slot)?;
                                if !snapshot.can_resolve(&anchor) {
                                    return None;
                                }
                                let point = snapshot.summary_for_anchor::<Point>(&anchor);
                                Some(SerializedNumberedBookmark {
                                    number: index as u8,
                                    row: point.row,
                                    column: point.column,
                                })
                            })
                            .collect()
                    }
                };

                numbered.sort_unstable_by_key(|bookmark| bookmark.number);
                numbered.dedup_by_key(|bookmark| bookmark.number);

                if numbered.is_empty() {
                    None
                } else {
                    Some((path.clone(), numbered))
                }
            })
            .collect()
    }

    pub fn vscode_project_file_for_worktree(
        &self,
        worktree_root: &Path,
        cx: &App,
    ) -> NumberedBookmarksProjectFile {
        let mut files = Vec::new();
        for (abs_path, numbered) in self.all_serialized_numbered_bookmarks(cx) {
            let Ok(relative_path) = abs_path.strip_prefix(worktree_root) else {
                continue;
            };
            let mut bookmarks = vec![
                NumberedBookmarkPosition {
                    line: UNDEFINED_NUMBERED_BOOKMARK_LINE,
                    column: 0,
                };
                NUMBERED_BOOKMARK_COUNT
            ];
            for bookmark in numbered {
                let Some(index) = numbered_index(bookmark.number) else {
                    continue;
                };
                bookmarks[index] = NumberedBookmarkPosition {
                    line: bookmark.row as i32,
                    column: bookmark.column as i32,
                };
            }
            files.push(NumberedBookmarksProjectFileEntry {
                path: relative_path.to_string_lossy().replace('\\', "/"),
                bookmarks,
            });
        }
        files.sort_by(|left, right| left.path.cmp(&right.path));
        NumberedBookmarksProjectFile { files }
    }

    pub fn load_vscode_project_file(
        &mut self,
        worktree_root: &Path,
        project_file: NumberedBookmarksProjectFile,
        cx: &mut Context<Self>,
    ) {
        let mut numbered_bookmarks = BTreeMap::new();
        for file in project_file.files {
            let abs_path = Arc::<Path>::from(worktree_root.join(file.path));
            let mut numbered = Vec::new();
            for (index, position) in file.bookmarks.into_iter().enumerate() {
                if position.line < 0 {
                    continue;
                }
                numbered.push(SerializedNumberedBookmark {
                    number: index as u8,
                    row: position.line as u32,
                    column: position.column.max(0) as u32,
                });
            }
            if !numbered.is_empty() {
                numbered_bookmarks.insert(abs_path, numbered);
            }
        }
        self.apply_serialized_numbered_bookmarks(numbered_bookmarks, cx);
    }

    pub fn persist_vscode_project_files(&self, fs: Arc<dyn Fs>, cx: &App) -> Task<Result<()>> {
        let mut files_to_write = Vec::new();
        for worktree in self.worktree_store.read(cx).worktrees() {
            let root = worktree.read(cx).abs_path();
            let project_file = self.vscode_project_file_for_worktree(&root, cx);
            files_to_write.push((
                root.join(VSCODE_NUMBERED_BOOKMARKS_DIR)
                    .join(VSCODE_NUMBERED_BOOKMARKS_FILE),
                project_file,
            ));
        }

        cx.background_spawn(async move {
            for (path, project_file) in files_to_write {
                if let Some(parent) = path.parent() {
                    fs.create_dir(parent).await?;
                }
                let json = serde_json::to_string_pretty(&project_file)?;
                fs.atomic_write(path, json).await?;
            }
            Ok(())
        })
    }

    pub fn vscode_project_file_path(worktree_root: &Path) -> PathBuf {
        worktree_root
            .join(VSCODE_NUMBERED_BOOKMARKS_DIR)
            .join(VSCODE_NUMBERED_BOOKMARKS_FILE)
    }

    pub fn replace_numbered_bookmarks_from_project_files(
        &mut self,
        files: Vec<(Arc<Path>, NumberedBookmarksProjectFile)>,
        cx: &mut Context<Self>,
    ) {
        self.clear_all_numbered_bookmarks(cx);
        for (worktree_root, project_file) in files {
            self.load_vscode_project_file(&worktree_root, project_file, cx);
        }
    }

    pub async fn all_bookmark_locations(
        this: Entity<BookmarkStore>,
        cx: &mut (impl AppContext + Clone),
    ) -> Result<HashMap<Entity<Buffer>, Vec<Range<Point>>>> {
        Self::resolve_all(&this, cx).await?;

        cx.read_entity(&this, |this, cx| {
            let mut locations: HashMap<_, Vec<_>> = HashMap::default();
            for bookmarks in this.bookmarks.values().filter_map(BookmarkEntry::loaded) {
                if bookmarks.bookmarks().is_empty() {
                    continue;
                }
                let snapshot = cx.read_entity(bookmarks.buffer(), |b, _| b.snapshot());
                let ranges: Vec<Range<Point>> = bookmarks
                    .bookmarks()
                    .iter()
                    .map(|bookmark| {
                        let row = snapshot.summary_for_anchor::<Point>(&bookmark.anchor).row;
                        Point::row_range(row..row)
                    })
                    .collect();

                locations
                    .entry(bookmarks.buffer().clone())
                    .or_default()
                    .extend(ranges);
            }

            Ok(locations)
        })
    }

    /// Opens buffers for all unloaded bookmark entries and resolves them to anchors. This is used to show all bookmarks in a large multi-buffer.
    async fn resolve_all(this: &Entity<Self>, cx: &mut (impl AppContext + Clone)) -> Result<()> {
        let unloaded_paths: Vec<Arc<Path>> = cx.read_entity(&this, |this, _| {
            this.bookmarks
                .iter()
                .filter_map(|(path, entry)| match entry {
                    BookmarkEntry::Unloaded { .. } if !this.paths_failed_to_open.contains(path) => {
                        Some(path.clone())
                    }
                    _ => None,
                })
                .collect_vec()
        });

        if unloaded_paths.is_empty() {
            return Ok(());
        }

        let worktree_store = cx.read_entity(&this, |this, _| this.worktree_store.clone());
        let buffer_store = cx.read_entity(&this, |this, _| this.buffer_store.clone());

        let open_tasks: FuturesUnordered<_> = unloaded_paths
            .iter()
            .map(|path| {
                open_path(path, &worktree_store, &buffer_store, cx.clone())
                    .map_err(move |e| (path, e))
                    .map_ok(move |b| (path, b))
            })
            .collect();

        let results: Vec<_> = open_tasks.collect().await;

        cx.update_entity(&this, |this, cx| {
            for result in results {
                match result {
                    Ok((path, buffer)) => this.resolve_anchors_if_needed(path, &buffer, cx),
                    Err((path, error)) => {
                        log::warn!(
                            "Could not open buffer for bookmarked path {}: {error}",
                            path.display()
                        );
                        if this.bookmarks.contains_key(path) {
                            this.paths_failed_to_open.insert(path.clone());
                        }
                    }
                }
            }
            cx.notify();
        });

        Ok(())
    }

    pub fn clear_bookmarks(&mut self, cx: &mut Context<Self>) {
        for entry in self.bookmarks.values_mut() {
            match entry {
                BookmarkEntry::Loaded(buffer_bookmarks) => buffer_bookmarks.bookmarks.clear(),
                BookmarkEntry::Unloaded { bookmarks, .. } => bookmarks.clear(),
            }
        }
        let empty_paths: Vec<Arc<Path>> = self
            .bookmarks
            .iter()
            .filter(|(_, entry)| entry.is_empty())
            .map(|(path, _)| path.clone())
            .collect();
        for path in empty_paths {
            self.bookmarks.remove(&path);
            self.paths_failed_to_open.remove(&path);
        }
        Self::emit_bookmarks_changed(cx);
    }

    pub fn clear_all_numbered_bookmarks(&mut self, cx: &mut Context<Self>) {
        self.untitled_numbered.clear();
        for entry in self.bookmarks.values_mut() {
            match entry {
                BookmarkEntry::Loaded(buffer_bookmarks) => {
                    buffer_bookmarks.numbered = std::array::from_fn(|_| None);
                }
                BookmarkEntry::Unloaded { numbered, .. } => numbered.clear(),
            }
        }
        let empty_paths: Vec<Arc<Path>> = self
            .bookmarks
            .iter()
            .filter(|(_, entry)| entry.is_empty())
            .map(|(path, _)| path.clone())
            .collect();
        for path in empty_paths {
            self.bookmarks.remove(&path);
            self.paths_failed_to_open.remove(&path);
        }
        Self::emit_bookmarks_changed(cx);
    }
}

fn numbered_index(number: u8) -> Option<usize> {
    let index = number as usize;
    (index < NUMBERED_BOOKMARK_COUNT).then_some(index)
}

async fn open_path(
    path: &Path,
    worktree_store: &Entity<WorktreeStore>,
    buffer_store: &Entity<BufferStore>,
    mut cx: impl AppContext,
) -> Result<Entity<Buffer>> {
    let (worktree, worktree_path) = cx
        .update_entity(&worktree_store, |worktree_store, cx| {
            worktree_store.find_or_create_worktree(path, false, cx)
        })
        .await?;

    let project_path = ProjectPath {
        worktree_id: cx.read_entity(&worktree, |worktree, _| worktree.id()),
        path: worktree_path,
    };

    let buffer = cx
        .update_entity(&buffer_store, |buffer_store, cx| {
            buffer_store.open_buffer(project_path, cx)
        })
        .await?;

    let exists_on_disk = cx.read_entity(&buffer, |buffer, _| {
        buffer.file().is_some_and(|file| file.disk_state().exists())
    });
    anyhow::ensure!(
        exists_on_disk,
        "bookmarked file {} does not exist on disk",
        path.display()
    );

    Ok(buffer)
}
