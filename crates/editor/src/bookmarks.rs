use std::{cmp::Reverse, ops::Range};

use collections::{HashMap, HashSet};
use futures::{StreamExt as _, channel::mpsc};
use gpui::{AppContext as _, Entity, EventEmitter, Subscription, Task};
use language::Buffer;
use multi_buffer::{
    Anchor, Event as MultiBufferEvent, MultiBuffer, MultiBufferOffset, MultiBufferSnapshot,
    PathKey, ToOffset as _, ToPoint as _,
};
use project::{
    Project,
    bookmark_store::{BookmarkStore, BookmarkStoreEvent},
};
use rope::Point;
use text::Bias;
use theme::ActiveTheme as _;
use ui::{Context, Window};
use util::ResultExt as _;
use workspace::{Workspace, searchable::Direction};

use crate::display_map::DisplayRow;
use crate::{
    EditBookmark, Editor, GoToNextBookmark, GoToPreviousBookmark, RowHighlightOptions,
    SelectionEffects, ToggleBookmark, ToggleBookmarkWithLabel, ViewBookmarks,
    multibuffer_context_lines, scroll::Autoscroll,
};

pub(crate) enum BookmarkRowHighlights {}

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
        selections.sort_unstable_by_key(|s| s.head());
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
        bookmark_store.update(cx, |store, cx| store.forget_failed_paths(cx));

        if Self::activate_bookmarks_tab(workspace, window, cx) {
            return;
        }

        if bookmark_store.read(cx).is_empty() {
            return;
        }

        cx.spawn_in(window, async move |workspace, cx| {
            let Some(locations) = BookmarkStore::all_bookmark_locations(bookmark_store.clone(), cx)
                .await
                .log_err()
            else {
                return;
            };
            if locations.is_empty() {
                return;
            }

            workspace
                .update_in(cx, |workspace, window, cx| {
                    if Self::activate_bookmarks_tab(workspace, window, cx) {
                        return;
                    }

                    let capability = workspace.project().read(cx).capability();
                    let excerpt_buffer =
                        cx.new(|_cx| MultiBuffer::new(capability).with_title("Bookmarks".into()));
                    let bookmarks_tab_state = cx.new(|cx| {
                        BookmarksTabState::new(
                            excerpt_buffer.clone(),
                            bookmark_store,
                            locations,
                            cx,
                        )
                    });
                    let first_range = bookmarks_tab_state
                        .read(cx)
                        .populated_ranges
                        .first()
                        .cloned();
                    let editor = cx.new(|cx| {
                        let mut editor = Editor::for_multibuffer(
                            excerpt_buffer,
                            Some(workspace.project().clone()),
                            window,
                            cx,
                        );
                        editor.set_bookmarks_tab_state(bookmarks_tab_state, cx);
                        if let Some(first_range) = first_range {
                            editor.change_selections(
                                SelectionEffects::no_scroll(),
                                window,
                                cx,
                                |s| {
                                    s.clear_disjoint();
                                    s.select_anchor_ranges(std::iter::once(first_range));
                                },
                            );
                        }
                        editor
                    });

                    workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
                })
                .log_err();
        })
        .detach();
    }

    fn activate_bookmarks_tab(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        let activation_history = workspace.recently_activated_items(cx);
        let existing = workspace
            .items_of_type::<Editor>(cx)
            .filter(|editor| editor.read(cx).bookmarks_tab_state.is_some())
            .max_by_key(|editor| {
                (
                    activation_history
                        .get(&editor.entity_id())
                        .copied()
                        .unwrap_or(0),
                    editor.entity_id(),
                )
            });
        existing.is_some_and(|existing| workspace.activate_item(&existing, true, true, window, cx))
    }

    pub(super) fn set_bookmarks_tab_state(
        &mut self,
        bookmarks_tab_state: Entity<BookmarksTabState>,
        cx: &mut Context<Self>,
    ) {
        self.bookmarks_tab_subscription = Some(cx.subscribe(
            &bookmarks_tab_state,
            |editor, _bookmarks_tab_state, event, cx| {
                let BookmarksTabEvent::Populated(ranges) = event;
                editor.on_bookmarks_populated(ranges, cx);
            },
        ));

        let populated_ranges = bookmarks_tab_state.read(cx).populated_ranges.clone();
        self.bookmarks_tab_state = Some(bookmarks_tab_state);
        self.on_bookmarks_populated(&populated_ranges, cx);
    }

    fn on_bookmarks_populated(&mut self, ranges: &[Range<Anchor>], cx: &mut Context<Self>) {
        self.clear_row_highlights::<BookmarkRowHighlights>();
        let snapshot = self.buffer.read(cx).snapshot(cx);
        for range in ranges {
            if !snapshot.can_resolve(&range.start) {
                continue;
            }
            let row = range.start.to_point(&snapshot).row;
            let start = snapshot.anchor_before(Point::new(row, 0));
            let max_point = snapshot.max_point();
            let end = if row >= max_point.row {
                if max_point.column == 0 {
                    Anchor::Max
                } else {
                    snapshot.anchor_before(max_point)
                }
            } else {
                snapshot.anchor_before(Point::new(row + 1, 0))
            };
            self.highlight_rows::<BookmarkRowHighlights>(
                start..end,
                |cx| cx.theme().colors().editor_highlighted_line_background,
                RowHighlightOptions::default(),
                cx,
            );
        }
        cx.notify();
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

pub(crate) struct BookmarksTabState {
    multibuffer: Entity<MultiBuffer>,
    populated_ranges: Vec<Range<Anchor>>,
    populated_emit_scheduled: bool,
    refresh_tx: mpsc::UnboundedSender<()>,
    _refresh_task: Task<()>,
    _store_subscription: Subscription,
    _multibuffer_subscription: Subscription,
}

pub(crate) enum BookmarksTabEvent {
    Populated(Vec<Range<Anchor>>),
}

impl EventEmitter<BookmarksTabEvent> for BookmarksTabState {}

impl BookmarksTabState {
    fn new(
        multibuffer: Entity<MultiBuffer>,
        bookmark_store: Entity<BookmarkStore>,
        locations: HashMap<Entity<Buffer>, Vec<Range<Point>>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let store_subscription = cx.subscribe(
            &bookmark_store,
            |this, _bookmark_store, event: &BookmarkStoreEvent, _cx| {
                if *event == BookmarkStoreEvent::BookmarksChanged {
                    this.schedule_refresh();
                }
            },
        );
        let multibuffer_subscription = cx.subscribe(
            &multibuffer,
            |this, _multibuffer, event: &MultiBufferEvent, cx| {
                if let MultiBufferEvent::Edited { .. } = event {
                    this.schedule_populated_emit(cx);
                }
            },
        );
        let context_lines = multibuffer_context_lines(cx);
        let populated_ranges = multibuffer.update(cx, |multibuffer, cx| {
            set_bookmark_excerpts(multibuffer, locations, context_lines, cx)
        });
        let (refresh_tx, mut refresh_rx) = mpsc::unbounded::<()>();
        let refresh_task = cx.spawn(async move |this, cx| {
            while refresh_rx.next().await.is_some() {
                while refresh_rx.try_recv().is_ok() {}

                let Some(locations) =
                    BookmarkStore::all_bookmark_locations(bookmark_store.clone(), cx)
                        .await
                        .log_err()
                else {
                    continue;
                };

                let applied = this.update(cx, |this, cx| {
                    let context_lines = multibuffer_context_lines(cx);
                    this.populated_ranges = this.multibuffer.update(cx, |multibuffer, cx| {
                        set_bookmark_excerpts(multibuffer, locations, context_lines, cx)
                    });
                    this.schedule_populated_emit(cx);
                });
                if applied.is_err() {
                    break;
                }
            }
        });
        Self {
            multibuffer,
            populated_ranges,
            populated_emit_scheduled: false,
            refresh_tx,
            _refresh_task: refresh_task,
            _store_subscription: store_subscription,
            _multibuffer_subscription: multibuffer_subscription,
        }
    }

    fn schedule_refresh(&self) {
        self.refresh_tx.unbounded_send(()).ok();
    }

    fn schedule_populated_emit(&mut self, cx: &mut Context<Self>) {
        if self.populated_emit_scheduled {
            return;
        }
        self.populated_emit_scheduled = true;
        let this = cx.weak_entity();
        cx.defer(move |cx| {
            this.update(cx, |this, cx| {
                this.populated_emit_scheduled = false;
                cx.emit(BookmarksTabEvent::Populated(this.populated_ranges.clone()));
            })
            .ok();
        });
    }
}

fn set_bookmark_excerpts(
    multibuffer: &mut MultiBuffer,
    locations: impl IntoIterator<Item = (Entity<Buffer>, Vec<Range<Point>>)>,
    context_line_count: u32,
    cx: &mut Context<MultiBuffer>,
) -> Vec<Range<Anchor>> {
    let mut stale_paths = multibuffer
        .snapshot(cx)
        .buffers_with_paths()
        .map(|(_, path_key)| path_key.clone())
        .collect::<HashSet<_>>();

    let mut anchor_ranges = <Vec<Range<Anchor>>>::new();
    for (buffer, mut ranges) in locations {
        ranges.sort_by_key(|range| (range.start, Reverse(range.end)));
        let path_key = PathKey::for_buffer(&buffer, cx);
        stale_paths.remove(&path_key);
        multibuffer.set_excerpts_for_path(
            path_key,
            buffer.clone(),
            ranges.clone(),
            context_line_count,
            cx,
        );
        let snapshot = multibuffer.snapshot(cx);
        let buffer_snapshot = buffer.read(cx).snapshot();
        anchor_ranges.extend(ranges.into_iter().filter_map(|range| {
            let text_range = buffer_snapshot.anchor_range_inside(range);
            let start = snapshot.anchor_in_buffer(text_range.start)?;
            let end = snapshot.anchor_in_buffer(text_range.end)?;
            Some(start..end)
        }));
    }

    for path_key in stale_paths {
        multibuffer.remove_excerpts(path_key, cx);
    }

    let snapshot = multibuffer.snapshot(cx);
    anchor_ranges
        .retain(|range| snapshot.can_resolve(&range.start) && snapshot.can_resolve(&range.end));
    anchor_ranges.sort_by(|a, b| a.start.cmp(&b.start, &snapshot));
    anchor_ranges
}
