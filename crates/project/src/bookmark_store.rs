use std::{collections::BTreeMap, ops::Range, path::Path, sync::Arc};

use anyhow::Result;
use collections::{HashMap, HashSet};
use futures::{StreamExt, TryFutureExt, stream::FuturesUnordered};
use gpui::{App, AppContext, Context, Entity, EventEmitter, Subscription, Task};
use itertools::Itertools;
use language::{Buffer, BufferEvent};
use text::{BufferSnapshot, Point};
use worktree::PathChange;

use crate::{
    ProjectPath,
    buffer_store::BufferStore,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerializedBookmark {
    pub row: u32,
    pub label: String,
}

#[derive(Debug)]
pub struct BufferBookmarks {
    buffer: Entity<Buffer>,
    bookmarks: Vec<Bookmark>,
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
            _subscription: subscription,
        }
    }

    pub fn buffer(&self) -> &Entity<Buffer> {
        &self.buffer
    }

    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }
}

#[derive(Debug)]
pub enum BookmarkEntry {
    Loaded(BufferBookmarks),
    Unloaded(Vec<SerializedBookmark>),
}

impl BookmarkEntry {
    pub fn is_empty(&self) -> bool {
        match self {
            BookmarkEntry::Loaded(buffer_bookmarks) => buffer_bookmarks.bookmarks.is_empty(),
            BookmarkEntry::Unloaded(rows) => rows.is_empty(),
        }
    }

    fn loaded(&self) -> Option<&BufferBookmarks> {
        match self {
            BookmarkEntry::Loaded(buffer_bookmarks) => Some(buffer_bookmarks),
            BookmarkEntry::Unloaded(_) => None,
        }
    }
}

pub struct BookmarkStore {
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    bookmarks: BTreeMap<Arc<Path>, BookmarkEntry>,
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

            self.bookmarks.insert(path, BookmarkEntry::Unloaded(rows));
        }

        Self::emit_bookmarks_changed(cx);
        Task::ready(Ok(()))
    }

    fn resolve_anchors_if_needed(
        &mut self,
        abs_path: &Arc<Path>,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let Some(BookmarkEntry::Unloaded(bookmarks)) = self.bookmarks.get(abs_path) else {
            return;
        };

        let unresolved_count = bookmarks.len();
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

        let was_failed_to_open = self.paths_failed_to_open.remove(abs_path);

        let resolved_count = bookmarks.len();
        if bookmarks.is_empty() {
            self.bookmarks.remove(abs_path);
        } else {
            let mut buffer_bookmarks = BufferBookmarks::new(buffer.clone(), cx);
            buffer_bookmarks.bookmarks = bookmarks;
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

        let snapshot = buffer.read(cx).text_snapshot();

        let existing_index = buffer_bookmarks.bookmarks.iter().position(|existing| {
            existing.anchor.summary::<Point>(&snapshot).row
                == anchor.summary::<Point>(&snapshot).row
        });

        if let Some(index) = existing_index {
            buffer_bookmarks.bookmarks.remove(index);
            if buffer_bookmarks.bookmarks.is_empty() {
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
                BookmarkEntry::Unloaded(_) => true,
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
                    BookmarkEntry::Unloaded(_) => false,
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

    pub fn all_serialized_bookmarks(
        &self,
        cx: &App,
    ) -> BTreeMap<Arc<Path>, Vec<SerializedBookmark>> {
        self.bookmarks
            .iter()
            .filter_map(|(path, entry)| {
                let mut rows = match entry {
                    BookmarkEntry::Unloaded(rows) => rows.clone(),
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
                    BookmarkEntry::Unloaded(_) if !this.paths_failed_to_open.contains(path) => {
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
        self.bookmarks.clear();
        self.paths_failed_to_open.clear();
        Self::emit_bookmarks_changed(cx);
    }
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
