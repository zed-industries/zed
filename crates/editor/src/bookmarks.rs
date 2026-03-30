use std::ops::Range;

use gpui::Entity;
use multi_buffer::{Anchor, MultiBufferOffset, MultiBufferSnapshot, ToOffset as _, ToPoint as _};
use project::{Project, bookmark_store::BookmarkStore};
use rope::Point;
use ui::{Context, Window};
use util::ResultExt as _;
use workspace::{Workspace, searchable::Direction};

use crate::{
    Editor, MultibufferSelectionMode, SelectionEffects, ViewBookmarks, scroll::Autoscroll,
};

impl Editor {
    pub fn set_show_bookmarks(&mut self, show_bookmarks: bool, cx: &mut Context<Self>) {
        self.show_bookmarks = Some(show_bookmarks);
        cx.notify();
    }

    pub fn toggle_bookmark(
        &mut self,
        _: &crate::actions::ToggleBookmark,
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
            let anchor = multi_buffer_snapshot.anchor_before(head);
            if let Some((buffer_snapshot, _, _excerpt_id)) = multi_buffer_snapshot
                .range_to_buffer_ranges(head..=head)
                .into_iter()
                .next()
            {
                if let Some(buffer) = project
                    .read(cx)
                    .buffer_for_id(buffer_snapshot.remote_id(), cx)
                {
                    let text_anchor = {
                        let point = anchor.to_point(&multi_buffer_snapshot);
                        multi_buffer_snapshot
                            .anchor_before(Point::new(point.row, 0))
                            .text_anchor
                    };
                    bookmark_store.update(cx, |store, cx| {
                        store.toggle_bookmark(buffer, text_anchor, cx);
                    });
                }
            }
        }

        cx.notify();
    }

    pub fn go_to_next_bookmark(
        &mut self,
        _: &crate::actions::GoToNextBookmark,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_bookmark_impl(Direction::Next, window, cx);
    }

    pub fn go_to_previous_bookmark(
        &mut self,
        _: &crate::actions::GoToPreviousBookmark,
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

        let mut before = Self::bookmarks_in_range(
            MultiBufferOffset(0)..selection.head(),
            cx,
            &multi_buffer_snapshot,
            project,
            bookmark_store,
        );
        let mut after = Self::bookmarks_in_range(
            selection.head()..multi_buffer_snapshot.len(),
            cx,
            &multi_buffer_snapshot,
            project,
            bookmark_store,
        );
        before.sort_by_key(|a| a.to_offset(&multi_buffer_snapshot));
        after.sort_by_key(|a| a.to_offset(&multi_buffer_snapshot));

        let anchor = if direction == Direction::Next {
            after
                .into_iter()
                .chain(before)
                .find(|anchor| anchor.to_offset(&multi_buffer_snapshot) != selection.head())
        } else {
            [before, after]
                .into_iter()
                .flat_map(|bookmarks| bookmarks.into_iter().rev())
                .find(|anchor| anchor.to_offset(&multi_buffer_snapshot) != selection.head())
        };

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

        let resolve_task = bookmark_store.update(cx, |store, cx| store.resolve_all(cx));

        cx.spawn_in(window, async move |workspace, cx| {
            resolve_task.await.log_err();

            workspace
                .update_in(cx, |workspace, window, cx| {
                    let bookmark_store = workspace.project().read(cx).bookmark_store();
                    let locations = bookmark_store.read(cx).all_bookmark_locations(cx);

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
        cx: &mut Context<Self>,
        multi_buffer_snapshot: &MultiBufferSnapshot,
        project: &Entity<Project>,
        bookmark_store: &Entity<BookmarkStore>,
    ) -> Vec<Anchor> {
        multi_buffer_snapshot
            .range_to_buffer_ranges(range)
            .into_iter()
            .flat_map(|(buffer_snapshot, buffer_range, excerpt_id)| {
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
                            Some(
                                buffer_snapshot.anchor_before(buffer_range.start)
                                    ..buffer_snapshot.anchor_after(buffer_range.end),
                            ),
                            buffer_snapshot,
                            cx,
                        )
                    })
                    .into_iter()
                    .map(|bookmark| Anchor::in_buffer(excerpt_id, bookmark.anchor()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
