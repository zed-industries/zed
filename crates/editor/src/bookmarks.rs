use std::ops::Range;

use gpui::Entity;
use multi_buffer::{Anchor, MultiBufferOffset, MultiBufferSnapshot, ToOffset as _};
use project::{Project, bookmark_store::BookmarkStore};
use rope::Point;
use text::Bias;
use ui::{Context, Window};
use util::ResultExt as _;
use workspace::{Workspace, searchable::Direction};

use crate::display_map::DisplayRow;
use crate::{
    Editor, GoToNextBookmark, GoToPreviousBookmark, MultibufferSelectionMode, SelectionEffects,
    ToggleBookmark, ViewBookmarks, scroll::Autoscroll,
};

impl Editor {
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

        for selection in &selections {
            let head = selection.head();
            let multibuffer_anchor = multi_buffer_snapshot.anchor_before(Point::new(head.row, 0));

            if let Some((buffer_anchor, _)) =
                multi_buffer_snapshot.anchor_to_buffer_anchor(multibuffer_anchor)
            {
                let buffer_id = buffer_anchor.buffer_id;
                if let Some(buffer) = project.read(cx).buffer_for_id(buffer_id, cx) {
                    bookmark_store.update(cx, |store, cx| {
                        store.toggle_bookmark(buffer, buffer_anchor, cx);
                    });
                }
            }
        }

        cx.notify();
    }

    pub fn toggle_bookmark_at_row(&mut self, row: DisplayRow, cx: &mut Context<Self>) {
        let Some(bookmark_store) = &self.bookmark_store else {
            return;
        };
        let display_snapshot = self.display_snapshot(cx);
        let point = display_snapshot.display_point_to_point(row.as_display_point(), Bias::Left);
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let anchor = buffer_snapshot.anchor_before(point);

        let Some((position, _)) = buffer_snapshot.anchor_to_buffer_anchor(anchor) else {
            return;
        };
        let Some(buffer) = self.buffer.read(cx).buffer(position.buffer_id) else {
            return;
        };

        bookmark_store.update(cx, |bookmark_store, cx| {
            bookmark_store.toggle_bookmark(buffer, position, cx);
        });

        cx.notify();
    }

    pub fn toggle_bookmark_at_anchor(&mut self, anchor: Anchor, cx: &mut Context<Self>) {
        let Some(bookmark_store) = &self.bookmark_store else {
            return;
        };
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let Some((position, _)) = buffer_snapshot.anchor_to_buffer_anchor(anchor) else {
            return;
        };
        let Some(buffer) = self.buffer.read(cx).buffer(position.buffer_id) else {
            return;
        };

        bookmark_store.update(cx, |bookmark_store, cx| {
            bookmark_store.toggle_bookmark(buffer, position, cx);
        });

        cx.notify();
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
                    .filter_map(|bookmark| {
                        multi_buffer_snapshot.anchor_in_buffer(bookmark.anchor())
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
