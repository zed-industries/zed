use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

use gpui::{AppContext as _, Entity};
use language::Buffer;
use multi_buffer::{
    Anchor, MultiBuffer, MultiBufferOffset, MultiBufferSnapshot, PathKey, ToOffset as _,
};
use project::{
    Project,
    bookmark_store::{BookmarkStore, BookmarkStoreEvent},
};
use rope::Point;
use text::Bias;
use ui::{Context, Window};
use util::ResultExt as _;
use workspace::{Workspace, searchable::Direction};

use crate::display_map::{DisplayRow, HighlightKey};
use crate::{
    EditBookmark, Editor, GoToNextBookmark, GoToPreviousBookmark, SelectionEffects, ToggleBookmark,
    ToggleBookmarkWithLabel, ViewBookmarks, multibuffer_context_lines, scroll::Autoscroll,
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
        self.toggle_bookmark_impl(false, window, cx);
    }

    pub fn toggle_bookmark_with_label(
        &mut self,
        _: &ToggleBookmarkWithLabel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_bookmark_impl(true, window, cx);
    }

    fn toggle_bookmark_impl(
        &mut self,
        with_label: bool,
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
        } else if with_label {
            // Only add new ones (prompting for a label) and leave existing ones unchanged.
            self.add_toggle_bookmark_blocks(absent_targets, bookmark_store, window, cx);
        } else {
            // Only add new (unnamed) bookmarks and leave existing ones unchanged.
            self.toggle_bookmarks(absent_targets, String::new(), cx);
        }

        cx.notify();
    }

    pub fn toggle_bookmark_at_row(&mut self, row: DisplayRow, cx: &mut Context<Self>) {
        let display_snapshot = self.display_snapshot(cx);
        let point = display_snapshot.display_point_to_point(row.as_display_point(), Bias::Left);
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let anchor = buffer_snapshot.anchor_before(point);

        self.toggle_bookmark_at_anchor(anchor, cx);
    }

    pub fn toggle_bookmark_at_anchor(&mut self, anchor: Anchor, cx: &mut Context<Self>) {
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

        bookmark_store.update(cx, |bookmark_store, cx| {
            bookmark_store.toggle_bookmark(buffer, position, String::new(), cx);
        });

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
        let existing = workspace.panes().iter().find_map(|pane| {
            let pane_ref = pane.read(cx);
            let editor = pane_ref
                .items()
                .filter_map(|item| item.downcast::<Editor>())
                .find(|editor| editor.read(cx).bookmark_view_subscription.is_some())?;
            let index = pane_ref.index_for_item(&editor)?;
            Some((pane.clone(), index))
        });

        if let Some((pane, index)) = existing {
            pane.update(cx, |pane, cx| {
                pane.activate_item(index, true, true, window, cx);
            });
            return;
        }

        let bookmark_store = workspace.project().read(cx).bookmark_store();
        if bookmark_store.read(cx).is_empty() {
            return;
        }

        let capability = workspace.project().read(cx).capability();
        let excerpt_buffer =
            cx.new(|_cx| MultiBuffer::new(capability).with_title("Bookmarks".into()));
        let editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(
                excerpt_buffer,
                Some(workspace.project().clone()),
                window,
                cx,
            );
            editor.bookmark_view_subscription =
                Some(editor.subscribe_to_bookmark_store(bookmark_store.clone(), window, cx));
            editor.bookmark_initial_selection_pending = true;
            editor.schedule_bookmark_refresh(bookmark_store, window, cx);
            editor
        });

        workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
    }

    pub(super) fn subscribe_to_bookmark_store(
        &mut self,
        bookmark_store: Entity<BookmarkStore>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> gpui::Subscription {
        cx.subscribe_in(
            &bookmark_store,
            window,
            |editor, bookmark_store, _event: &BookmarkStoreEvent, window, cx| {
                editor.schedule_bookmark_refresh(bookmark_store.clone(), window, cx);
            },
        )
    }

    pub(super) fn schedule_bookmark_refresh(
        &mut self,
        bookmark_store: Entity<BookmarkStore>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        self.bookmark_refresh_task = Some(cx.spawn_in(window, async move |this, cx| {
            let Some(locations) = BookmarkStore::all_bookmark_locations(bookmark_store, cx)
                .await
                .log_err()
            else {
                return;
            };

            this.update_in(cx, |editor, window, cx| {
                let ranges = editor.apply_bookmark_locations(locations, cx);
                if editor.bookmark_initial_selection_pending
                    && let Some(first_range) = ranges.first()
                {
                    editor.bookmark_initial_selection_pending = false;
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.clear_disjoint();
                        s.select_anchor_ranges(std::iter::once(first_range.clone()));
                    });
                }
            })
            .log_err();
        }));
    }

    fn apply_bookmark_locations(
        &mut self,
        locations: HashMap<Entity<Buffer>, Vec<Range<Point>>>,
        cx: &mut Context<Self>,
    ) -> Vec<Range<Anchor>> {
        let context_lines = multibuffer_context_lines(cx);
        let mut ranges = <Vec<Range<Anchor>>>::new();

        self.buffer.update(cx, |multibuffer, cx| {
            let mut stale_paths: HashSet<PathKey> = multibuffer
                .snapshot(cx)
                .buffers_with_paths()
                .map(|(_, path_key)| path_key.clone())
                .collect();

            for (buffer, mut buffer_ranges) in locations {
                buffer_ranges.sort_by_key(|range| (range.start, Reverse(range.end)));
                let path_key = PathKey::for_buffer(&buffer, cx);
                stale_paths.remove(&path_key);

                multibuffer.set_excerpts_for_path(
                    path_key,
                    buffer.clone(),
                    buffer_ranges.clone(),
                    context_lines,
                    cx,
                );

                let snapshot = multibuffer.snapshot(cx);
                let buffer_snapshot = buffer.read(cx).snapshot();
                ranges.extend(buffer_ranges.into_iter().filter_map(|range| {
                    let text_range = buffer_snapshot.anchor_range_inside(range);
                    let start = snapshot.anchor_in_buffer(text_range.start)?;
                    let end = snapshot.anchor_in_buffer(text_range.end)?;
                    Some(start..end)
                }));
            }

            for path_key in stale_paths {
                multibuffer.remove_excerpts(path_key, cx);
            }
        });

        let snapshot = self.buffer.read(cx).snapshot(cx);
        ranges.sort_by(|a, b| a.start.cmp(&b.start, &snapshot));

        self.highlight_background(
            HighlightKey::Editor,
            &ranges,
            |_, theme| theme.colors().editor_highlighted_line_background,
            cx,
        );

        ranges
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
