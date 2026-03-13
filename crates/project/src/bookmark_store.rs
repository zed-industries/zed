use std::{collections::BTreeMap, ops::Range, path::Path, sync::Arc};

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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SerializedBookmark {
    pub row: u32,
    pub path: Arc<Path>,
}

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

pub struct BookmarkStore {
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    bookmarks: BTreeMap<Arc<Path>, BufferBookmarks>,
}

impl BookmarkStore {
    pub fn new(worktree_store: Entity<WorktreeStore>, buffer_store: Entity<BufferStore>) -> Self {
        Self {
            buffer_store,
            worktree_store,
            bookmarks: BTreeMap::new(),
        }
    }

    pub fn with_serialized_bookmarks(
        &mut self,
        bookmarks: BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let worktree_store = self.worktree_store.downgrade();
        let buffer_store = self.buffer_store.downgrade();

        cx.spawn(async move |this, cx| {
            let mut new_bookmarks = BTreeMap::default();

            for (path, bookmarks) in bookmarks {
                if bookmarks.is_empty() {
                    continue;
                }

                let (worktree, relative_path) = worktree_store
                    .update(cx, |this, cx| {
                        this.find_or_create_worktree(&path, false, cx)
                    })?
                    .await?;

                let buffer = buffer_store
                    .update(cx, |this, cx| {
                        let path = ProjectPath {
                            worktree_id: worktree.read(cx).id(),
                            path: relative_path,
                        };
                        this.open_buffer(path, cx)
                    })?
                    .await;

                let Ok(buffer) = buffer else {
                    log::warn!("Could not load buffer for bookmarked path: {:?}", path);
                    continue;
                };

                let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

                let mut buffer_bookmarks =
                    this.update(cx, |_, cx| BufferBookmarks::new(buffer, cx))?;

                let max_point = snapshot.max_point();
                let bookmarks_vec: Vec<BookmarkAnchor> = bookmarks
                    .into_iter()
                    .filter_map(|bookmark| {
                        let point = Point::new(bookmark.row, 0);

                        if point > max_point {
                            log::warn!(
                                "Skipping out-of-range bookmark: {} row {} (file has {} rows)",
                                path.display(),
                                bookmark.row,
                                max_point.row
                            );
                            return None;
                        }

                        let anchor = snapshot.anchor_after(point);
                        Some(BookmarkAnchor(anchor))
                    })
                    .collect();

                if !bookmarks_vec.is_empty() {
                    buffer_bookmarks.bookmarks = bookmarks_vec;
                    new_bookmarks.insert(path, buffer_bookmarks);
                }
            }

            this.update(cx, |this, cx| {
                for (path, count) in new_bookmarks
                    .iter()
                    .map(|(p, b)| (p.to_string_lossy(), b.bookmarks.len()))
                {
                    let word = if count == 1 { "bookmark" } else { "bookmarks" };
                    log::debug!("Restored {count} {word} at {path}");
                }

                this.bookmarks = new_bookmarks;

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

        let buffer_bookmarks = self
            .bookmarks
            .entry(abs_path.clone())
            .or_insert_with(|| BufferBookmarks::new(buffer.clone(), cx));

        let snapshot = buffer.read(cx).text_snapshot();

        let existing_index = buffer_bookmarks.bookmarks.iter().position(|existing| {
            existing.0.summary::<Point>(&snapshot).row == anchor.summary::<Point>(&snapshot).row
        });

        if let Some(index) = existing_index {
            buffer_bookmarks.bookmarks.remove(index);
            // Clean up empty entries to save memory
            if buffer_bookmarks.bookmarks.is_empty() {
                self.bookmarks.remove(&abs_path);
            }
        } else {
            buffer_bookmarks.bookmarks.push(BookmarkAnchor(anchor));
        }

        cx.notify();
    }

    pub fn bookmarks(&self) -> &BTreeMap<Arc<Path>, BufferBookmarks> {
        &self.bookmarks
    }

    /// Returns the bookmarks for a given buffer within an optional range.
    pub fn bookmarks_for_buffer(
        &self,
        buffer: Entity<Buffer>,
        range: Option<Range<text::Anchor>>,
        buffer_snapshot: &BufferSnapshot,
        cx: &App,
    ) -> impl Iterator<Item = &BookmarkAnchor> {
        let abs_path = Self::abs_path_from_buffer(&buffer, cx);
        abs_path
            .and_then(|path| self.bookmarks.get(&path))
            .into_iter()
            .flat_map(move |file_bookmarks| {
                file_bookmarks.bookmarks.iter().filter_map({
                    let range = range.clone();
                    move |bookmark| {
                        if !buffer_snapshot.can_resolve(&bookmark.anchor()) {
                            return None;
                        }

                        if let Some(range) = &range
                            && (bookmark.anchor().cmp(&range.start, buffer_snapshot).is_lt()
                                || bookmark.anchor().cmp(&range.end, buffer_snapshot).is_gt())
                        {
                            return None;
                        }

                        Some(bookmark)
                    }
                })
            })
    }

    fn handle_file_changed(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let entity_id = buffer.entity_id();

        if buffer
            .read(cx)
            .file()
            .is_none_or(|f| f.disk_state().is_deleted())
        {
            self.bookmarks
                .retain(|_, buffer_bookmarks| buffer_bookmarks.buffer.entity_id() != entity_id);
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
                .find(|(_, buffer_bookmarks)| buffer_bookmarks.buffer.entity_id() == entity_id)
                .map(|(path, _)| path)
                .cloned()
            {
                let Some(buffer_bookmarks) = self.bookmarks.remove(&old_path) else {
                    log::error!(
                        "Couldn't get bookmarks from old path during buffer rename handling"
                    );
                    return;
                };
                self.bookmarks.insert(new_abs_path, buffer_bookmarks);
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
            .filter_map(|(path, buffer_bookmarks)| {
                let snapshot = buffer_bookmarks.buffer.read(cx).snapshot();
                let mut bookmarks: Vec<SerializedBookmark> = buffer_bookmarks
                    .bookmarks
                    .iter()
                    .filter_map(|bookmark| {
                        if !snapshot.can_resolve(&bookmark.anchor()) {
                            return None;
                        }
                        let row = snapshot.summary_for_anchor::<Point>(&bookmark.anchor()).row;
                        Some(SerializedBookmark {
                            row,
                            path: path.clone(),
                        })
                    })
                    .collect();

                bookmarks.sort_by_key(|a| a.row);
                bookmarks.dedup_by(|a, b| a.row == b.row);

                if bookmarks.is_empty() {
                    None
                } else {
                    Some((path.clone(), bookmarks))
                }
            })
            .collect()
    }

    pub fn has_bookmarks_for_buffer(&self, buffer: Entity<Buffer>, cx: &App) -> bool {
        let Some(abs_path) = Self::abs_path_from_buffer(&buffer, cx) else {
            return false;
        };
        self.bookmarks
            .get(&abs_path)
            .is_some_and(|b| !b.bookmarks.is_empty())
    }

    pub fn clear_bookmarks(&mut self, cx: &mut Context<Self>) {
        self.bookmarks.clear();
        cx.notify();
    }
}
