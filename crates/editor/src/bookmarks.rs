use std::ops::Range;

use gpui::Entity;
use language::Buffer;
use multi_buffer::{Anchor, MultiBufferOffset, MultiBufferSnapshot, ToOffset as _};
use project::{Project, bookmark_store::BookmarkStore};
use rope::Point;
use text::Bias;
use ui::{Context, Window};
use util::ResultExt as _;
use workspace::{Workspace, searchable::Direction};

use crate::display_map::DisplayRow;
use crate::{
    EditBookmark, Editor, GoToNextBookmark, GoToPreviousBookmark, MultibufferSelectionMode,
    SelectionEffects, ToggleBookmark, ViewBookmarks, scroll::Autoscroll,
};

#[derive(Clone, Debug)]
struct BookmarkTarget {
    buffer: Entity<Buffer>,
    anchor: Anchor,
    buffer_anchor: text::Anchor,
}

impl Editor {
    fn bookmark_exists_for_target(
        bookmark_store: &Entity<BookmarkStore>,
        target: &BookmarkTarget,
        cx: &mut Context<Self>,
    ) -> bool {
        bookmark_store.update(cx, |bookmark_store, cx| {
            bookmark_store
                .find_bookmark(&target.buffer, target.buffer_anchor, cx)
                .is_some()
        })
    }

    pub fn set_show_bookmarks(&mut self, show_bookmarks: bool, cx: &mut Context<Self>) {
        self.show_bookmarks = Some(show_bookmarks);
        cx.notify();
    }

    pub fn toggle_bookmark(
        &mut self,
        _: &ToggleBookmark,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(bookmark_store) = self.bookmark_store.clone() else {
            return;
        };
        let Some(project) = self.project() else {
            return;
        };

        let snapshot = self.snapshot(window, cx);
        let multi_buffer_snapshot = snapshot.buffer_snapshot();

        let mut selections = self.selections.all::<Point>(&snapshot.display_snapshot);
        selections.sort_by_key(|s| s.head());
        selections.dedup_by_key(|s| s.head().row);

        let mut exist_targets: Vec<BookmarkTarget> = vec![];
        let mut absent_targets: Vec<BookmarkTarget> = vec![];

        for selection in &selections {
            let head = selection.head();
            let multibuffer_anchor = multi_buffer_snapshot.anchor_before(Point::new(head.row, 0));

            if let Some((buffer_anchor, _)) =
                multi_buffer_snapshot.anchor_to_buffer_anchor(multibuffer_anchor)
            {
                let buffer_id = buffer_anchor.buffer_id;
                if let Some(buffer) = project.read(cx).buffer_for_id(buffer_id, cx) {
                    let target = BookmarkTarget {
                        buffer,
                        anchor: multibuffer_anchor,
                        buffer_anchor,
                    };

                    if Self::bookmark_exists_for_target(&bookmark_store, &target, cx) {
                        exist_targets.push(target);
                    } else {
                        absent_targets.push(target);
                    }
                }
            }
        }

        if absent_targets.is_empty() {
            // All cursors are on existing bookmarks, remove all bookmarks.
            self.toggle_bookmarks(exist_targets, String::new(), cx);
        } else {
            // Only add new ones and leave existing ones unchanged.
            self.add_toggle_bookmark_blocks(absent_targets, bookmark_store, window, cx);
        }

        cx.notify();
    }

    pub fn toggle_bookmark_at_row(
        &mut self,
        row: DisplayRow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_snapshot = self.display_snapshot(cx);
        let point = display_snapshot.display_point_to_point(row.as_display_point(), Bias::Left);
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let anchor = buffer_snapshot.anchor_before(point);

        self.toggle_bookmark_at_anchor(anchor, window, cx);
    }

    pub fn toggle_bookmark_at_anchor(
        &mut self,
        anchor: Anchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let Some((position, _)) = buffer_snapshot.anchor_to_buffer_anchor(anchor) else {
            return;
        };
        let Some(buffer) = self.buffer.read(cx).buffer(position.buffer_id) else {
            return;
        };

        let Some(bookmark_store) = self.bookmark_store.clone() else {
            return;
        };

        let target = BookmarkTarget {
            buffer,
            anchor,
            buffer_anchor: position,
        };
        if Self::bookmark_exists_for_target(&bookmark_store, &target, cx) {
            bookmark_store.update(cx, |bookmark_store, cx| {
                bookmark_store.toggle_bookmark(target.buffer, position, String::new(), cx);
            });
        } else {
            self.add_toggle_bookmark_blocks(vec![target], bookmark_store, window, cx)
        }

        cx.notify();
    }

    pub fn edit_bookmark(&mut self, _: &EditBookmark, window: &mut Window, cx: &mut Context<Self>) {
        let snapshot = self.snapshot(window, cx);
        let multi_buffer_snapshot = snapshot.buffer_snapshot();
        let selection = self
            .selections
            .newest::<Point>(&snapshot.display_snapshot)
            .head();
        let anchor = multi_buffer_snapshot.anchor_before(Point::new(selection.row, 0));
        self.edit_bookmark_at_anchor(anchor, window, cx);
    }

    pub fn edit_bookmark_at_anchor(
        &mut self,
        anchor: Anchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(bookmark_store) = self.bookmark_store.clone() else {
            return;
        };
        let Some(project) = self.project() else {
            return;
        };

        let editor_buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let Some((buffer_anchor, _)) = editor_buffer_snapshot.anchor_to_buffer_anchor(anchor)
        else {
            return;
        };
        let Some(buffer) = project.read(cx).buffer_for_id(buffer_anchor.buffer_id, cx) else {
            return;
        };
        let Some(label) = bookmark_store.update(cx, |store, cx| {
            store
                .find_bookmark(&buffer, buffer_anchor, cx)
                .map(|bookmark| bookmark.label.clone())
        }) else {
            return;
        };

        self.add_edit_bookmark_block(
            BookmarkTarget {
                anchor,
                buffer,
                buffer_anchor,
            },
            &label,
            bookmark_store,
            window,
            cx,
        );
    }

    fn add_edit_bookmark_block(
        &mut self,
        target: BookmarkTarget,
        label: &str,
        bookmark_store: Entity<BookmarkStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.add_edit_block(
            target.anchor,
            label,
            "Enter bookmark label (Optional)",
            Some(Box::new(move |label, _, cx| {
                bookmark_store.update(cx, |store, cx| {
                    store.edit_bookmark(&target.buffer, target.buffer_anchor, label, cx)
                });
            })),
            None,
            window,
            cx,
        );
    }

    fn add_toggle_bookmark_blocks(
        &mut self,
        targets: Vec<BookmarkTarget>,
        bookmark_store: Entity<BookmarkStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for target in targets {
            let bookmark_store = bookmark_store.clone();
            self.add_edit_block(
                target.anchor,
                "",
                "Enter bookmark label (Optional)",
                Some(Box::new(move |label: String, _, cx| {
                    bookmark_store.update(cx, |store, cx| {
                        store.toggle_bookmark(target.buffer, target.buffer_anchor, label, cx);
                    });
                })),
                None,
                window,
                cx,
            );
        }
    }

    fn toggle_bookmarks(
        &mut self,
        targets: Vec<BookmarkTarget>,
        label: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(bookmark_store) = self.bookmark_store.clone() {
            bookmark_store.update(cx, |store, cx| {
                for target in targets {
                    store.toggle_bookmark(target.buffer, target.buffer_anchor, label.clone(), cx);
                }
            });
        }
    }

    pub fn go_to_next_bookmark(
        &mut self,
        _: &GoToNextBookmark,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_bookmark_impl(Direction::Next, window, cx);
    }

    pub fn go_to_previous_bookmark(
        &mut self,
        _: &GoToPreviousBookmark,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_bookmark_impl(Direction::Prev, window, cx);
    }

    fn go_to_bookmark_impl(
        &mut self,
        direction: Direction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(project) = &self.project else {
            return;
        };
        let Some(bookmark_store) = &self.bookmark_store else {
            return;
        };

        let selection = self
            .selections
            .newest::<MultiBufferOffset>(&self.display_snapshot(cx));
        let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);

        let mut all_bookmarks = Self::bookmarks_in_range(
            MultiBufferOffset(0)..multi_buffer_snapshot.len(),
            &multi_buffer_snapshot,
            project,
            bookmark_store,
            cx,
        );
        all_bookmarks.sort_by_key(|a| a.to_offset(&multi_buffer_snapshot));

        let anchor = match direction {
            Direction::Next => all_bookmarks
                .iter()
                .find(|anchor| anchor.to_offset(&multi_buffer_snapshot) > selection.head())
                .or_else(|| all_bookmarks.first()),
            Direction::Prev => all_bookmarks
                .iter()
                .rfind(|anchor| anchor.to_offset(&multi_buffer_snapshot) < selection.head())
                .or_else(|| all_bookmarks.last()),
        }
        .cloned();

        if let Some(anchor) = anchor {
            self.unfold_ranges(&[anchor..anchor], true, false, cx);
            self.change_selections(
                SelectionEffects::scroll(Autoscroll::center()),
                window,
                cx,
                |s| {
                    s.select_anchor_ranges([anchor..anchor]);
                },
            );
        }
    }

    pub fn view_bookmarks(
        workspace: &mut Workspace,
        _: &ViewBookmarks,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let bookmark_store = workspace.project().read(cx).bookmark_store();
        cx.spawn_in(window, async move |workspace, cx| {
            let Some(locations) = BookmarkStore::all_bookmark_locations(bookmark_store, cx)
                .await
                .log_err()
            else {
                return;
            };

            workspace
                .update_in(cx, |workspace, window, cx| {
                    Editor::open_locations_in_multibuffer(
                        workspace,
                        locations,
                        "Bookmarks".into(),
                        false,
                        false,
                        MultibufferSelectionMode::First,
                        window,
                        cx,
                    );
                })
                .log_err();
        })
        .detach();
    }

    fn bookmarks_in_range(
        range: Range<MultiBufferOffset>,
        multi_buffer_snapshot: &MultiBufferSnapshot,
        project: &Entity<Project>,
        bookmark_store: &Entity<BookmarkStore>,
        cx: &mut Context<Self>,
    ) -> Vec<Anchor> {
        multi_buffer_snapshot
            .range_to_buffer_ranges(range)
            .into_iter()
            .flat_map(|(buffer_snapshot, buffer_range, _excerpt_range)| {
                let Some(buffer) = project
                    .read(cx)
                    .buffer_for_id(buffer_snapshot.remote_id(), cx)
                else {
                    return Vec::new();
                };
                bookmark_store
                    .update(cx, |store, cx| {
                        store.bookmarks_for_buffer(
                            buffer,
                            buffer_snapshot.anchor_before(buffer_range.start)
                                ..buffer_snapshot.anchor_after(buffer_range.end),
                            &buffer_snapshot,
                            cx,
                        )
                    })
                    .into_iter()
                    .filter_map(|bookmark| multi_buffer_snapshot.anchor_in_buffer(bookmark.anchor))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
