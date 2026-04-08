use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
    path::Path,
    sync::Arc,
};

use anyhow::Result;
use gpui::{App, Context, Entity, Subscription, Task};
use language::{Buffer, BufferEvent};
use text::{BufferSnapshot, Point};

use crate::{ProjectPath, buffer_store::BufferStore, worktree_store::WorktreeStore};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct BookmarkAnchor(text::Anchor);

impl BookmarkAnchor {
    pub fn anchor(&self) -> text::Anchor {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerializedBookmark(pub u32);

#[derive(Debug)]
pub struct BufferBookmarks {
    buffer: Entity<Buffer>,
    bookmarks: Vec<BookmarkAnchor>,
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

    pub fn bookmarks(&self) -> &[BookmarkAnchor] {
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
}

pub struct BookmarkStore {
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    bookmarks: BTreeMap<Arc<Path>, BookmarkEntry>,
}

impl BookmarkStore {
    pub fn new(worktree_store: Entity<WorktreeStore>, buffer_store: Entity<BufferStore>) -> Self {
        Self {
            buffer_store,
            worktree_store,
            bookmarks: BTreeMap::new(),
        }
    }

    pub fn load_serialized_bookmarks(
        &mut self,
        bookmark_rows: BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.bookmarks.clear();

        for (path, rows) in bookmark_rows {
            if rows.is_empty() {
                continue;
            }

            let count = rows.len();
            log::debug!("Stored {count} unloaded bookmark(s) at {}", path.display());

            self.bookmarks.insert(path, BookmarkEntry::Unloaded(rows));
        }

        cx.notify();
        Task::ready(Ok(()))
    }

    fn resolve_anchors_if_needed(
        &mut self,
        abs_path: &Arc<Path>,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let Some(BookmarkEntry::Unloaded(rows)) = self.bookmarks.get(abs_path) else {
            return;
        };

        let snapshot = buffer.read(cx).snapshot();
        let max_point = snapshot.max_point();

        let anchors: Vec<BookmarkAnchor> = rows
            .iter()
            .filter_map(|bookmark_row| {
                let point = Point::new(bookmark_row.0, 0);

                if point > max_point {
                    log::warn!(
                        "Skipping out-of-range bookmark: {} row {} (file has {} rows)",
                        abs_path.display(),
                        bookmark_row.0,
                        max_point.row
                    );
                    return None;
                }

                let anchor = snapshot.anchor_after(point);
                Some(BookmarkAnchor(anchor))
            })
            .collect();

        if anchors.is_empty() {
            self.bookmarks.remove(abs_path);
        } else {
            let mut buffer_bookmarks = BufferBookmarks::new(buffer.clone(), cx);
            buffer_bookmarks.bookmarks = anchors;
            self.bookmarks
                .insert(abs_path.clone(), BookmarkEntry::Loaded(buffer_bookmarks));
        }
    }

    /// Opens buffers for all unloaded bookmark entries and resolves them to anchors. This is used to show all bookmarks in a large multi-buffer.
    pub fn resolve_all(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let unloaded_paths: Vec<Arc<Path>> = self
            .bookmarks
            .iter()
            .filter_map(|(path, entry)| match entry {
                BookmarkEntry::Unloaded(_) => Some(path.clone()),
                BookmarkEntry::Loaded(_) => None,
            })
            .collect();

        if unloaded_paths.is_empty() {
            return Task::ready(Ok(()));
        }

        let worktree_store = self.worktree_store.downgrade();
        let buffer_store = self.buffer_store.downgrade();

        cx.spawn(async move |this, cx| {
            let open_tasks: Vec<_> = unloaded_paths
                .into_iter()
                .map(|path| {
                    let worktree_store = worktree_store.clone();
                    let buffer_store = buffer_store.clone();
                    let mut cx = cx.clone();
                    async move {
                        let result: Result<Entity<Buffer>> = async {
                            let (worktree, relative_path) = worktree_store
                                .update(&mut cx, |worktree_store, cx| {
                                    worktree_store.find_or_create_worktree(&path, false, cx)
                                })?
                                .await?;

                            let buffer = buffer_store
                                .update(&mut cx, |buffer_store, cx| {
                                    let project_path = ProjectPath {
                                        worktree_id: worktree.read(cx).id(),
                                        path: relative_path,
                                    };
                                    buffer_store.open_buffer(project_path, cx)
                                })?
                                .await?;

                            Ok(buffer)
                        }
                        .await;

                        (path, result)
                    }
                })
                .collect();

            let results = futures::future::join_all(open_tasks).await;

            this.update(cx, |this, cx| {
                for (path, result) in results {
                    match result {
                        Ok(buffer) => {
                            this.resolve_anchors_if_needed(&path, &buffer, cx);
                        }
                        Err(error) => {
                            log::warn!(
                                "Could not open buffer for bookmarked path {}: {error}",
                                path.display()
                            );
                        }
                    }
                }
                cx.notify();
            })?;

            Ok(())
        })
    }

    pub fn abs_path_from_buffer(buffer: &Entity<Buffer>, cx: &App) -> Option<Arc<Path>> {
        worktree::File::from_dyn(buffer.read(cx).file())
            .map(|file| file.worktree.read(cx).absolutize(&file.path))
            .map(Arc::<Path>::from)
    }

    /// Toggle a bookmark at the given anchor in the buffer.
    /// If a bookmark already exists on the same row, it will be removed.
    /// Otherwise, a new bookmark will be added.
    pub fn toggle_bookmark(
        &mut self,
        buffer: Entity<Buffer>,
        anchor: text::Anchor,
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
            existing.0.summary::<Point>(&snapshot).row == anchor.summary::<Point>(&snapshot).row
        });

        if let Some(index) = existing_index {
            buffer_bookmarks.bookmarks.remove(index);
            if buffer_bookmarks.bookmarks.is_empty() {
                self.bookmarks.remove(&abs_path);
            }
        } else {
            buffer_bookmarks.bookmarks.push(BookmarkAnchor(anchor));
        }

        cx.notify();
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
    ) -> Vec<BookmarkAnchor> {
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
                    if !buffer_snapshot.can_resolve(&bookmark.anchor()) {
                        return None;
                    }

                    if bookmark.anchor().cmp(&range.start, buffer_snapshot).is_lt()
                        || bookmark.anchor().cmp(&range.end, buffer_snapshot).is_gt()
                    {
                        return None;
                    }

                    Some(*bookmark)
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
            cx.notify();
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
                cx.notify();
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
                                if !snapshot.can_resolve(&bookmark.anchor()) {
                                    return None;
                                }
                                let row =
                                    snapshot.summary_for_anchor::<Point>(&bookmark.anchor()).row;
                                Some(SerializedBookmark(row))
                            })
                            .collect()
                    }
                };

                rows.sort();
                rows.dedup();

                if rows.is_empty() {
                    None
                } else {
                    Some((path.clone(), rows))
                }
            })
            .collect()
    }

    pub fn all_bookmark_locations(&self, cx: &App) -> HashMap<Entity<Buffer>, Vec<Range<Point>>> {
        let mut locations: HashMap<Entity<Buffer>, Vec<Range<Point>>> = HashMap::default();

        for (_, entry) in &self.bookmarks {
            let BookmarkEntry::Loaded(buffer_bookmarks) = entry else {
                continue;
            };
            let buffer = buffer_bookmarks.buffer().clone();
            let snapshot = buffer.read(cx).snapshot();
            let ranges: Vec<Range<Point>> = buffer_bookmarks
                .bookmarks()
                .iter()
                .map(|anchor| {
                    let row = snapshot.summary_for_anchor::<Point>(&anchor.anchor()).row;
                    Point::row_range(row..row)
                })
                .collect();
            locations.entry(buffer).or_default().extend(ranges);
        }

        locations
    }

    pub fn clear_bookmarks(&mut self, cx: &mut Context<Self>) {
        self.bookmarks.clear();
        cx.notify();
    }
}
