use std::{ops::Range, sync::Arc};

use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Anchor, Bias, DisplayPoint, Editor, ToOffset,
};
use gpui::{Context, Window};
use language::SelectionGoal;

use crate::{
    motion::{self, Motion},
    state::Mode,
    Vim,
};

impl Vim {
    pub fn create_mark(
        &mut self,
        text: Arc<str>,
        tail: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(anchors) = self.update_editor(window, cx, |_, editor, _, _| {
            editor
                .selections
                .disjoint_anchors()
                .iter()
                .map(|s| if tail { s.tail() } else { s.head() })
                .collect::<Vec<_>>()
        }) else {
            return;
        };
        self.set_mark(text.to_string(), anchors, cx);
        self.clear_operator(window, cx);
    }

    // When handling an action, you must create visual marks if you will switch to normal
    // mode without the default selection behavior.
    pub(crate) fn store_visual_marks(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode.is_visual() {
            self.create_visual_marks(self.mode, window, cx);
        }
    }

    pub(crate) fn create_visual_marks(
        &mut self,
        mode: Mode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut starts = vec![];
        let mut ends = vec![];
        let mut reversed = vec![];

        self.update_editor(window, cx, |_, editor, _, cx| {
            let (map, selections) = editor.selections.all_display(cx);
            for selection in selections {
                let end = movement::saturating_left(&map, selection.end);
                ends.push(
                    map.buffer_snapshot
                        .anchor_before(end.to_offset(&map, Bias::Left)),
                );
                starts.push(
                    map.buffer_snapshot
                        .anchor_before(selection.start.to_offset(&map, Bias::Left)),
                );
                reversed.push(selection.reversed)
            }
        });

        self.set_mark("<".to_string(), starts, cx);
        self.set_mark(">".to_string(), ends, cx);
        self.stored_visual_mode.replace((mode, reversed));
    }

    pub fn jump(
        &mut self,
        text: Arc<str>,
        line: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pop_operator(window, cx);

        let anchors = match &*text {
            "{" | "}" => self.update_editor(window, cx, |_, editor, _, cx| {
                let (map, selections) = editor.selections.all_display(cx);
                selections
                    .into_iter()
                    .map(|selection| {
                        let point = if &*text == "{" {
                            movement::start_of_paragraph(&map, selection.head(), 1)
                        } else {
                            movement::end_of_paragraph(&map, selection.head(), 1)
                        };
                        map.buffer_snapshot
                            .anchor_before(point.to_offset(&map, Bias::Left))
                    })
                    .collect::<Vec<Anchor>>()
            }),
            "." => self.change_list.last().cloned(),
            m if m.starts_with(|c: char| c.is_digit(10)) => {
                if let Some((path, _points)) = self.get_global_mark(text.to_string(), window, cx) {
                    if let Some(workspace) = self.workspace(window) {
                        workspace.update(cx, |workspace, cx| {
                            let Some(worktree) = workspace.worktrees(cx).next() else {
                                return;
                            };
                            let worktree_id = worktree.read(cx).id();
                            let Some(path_str) = path.to_str() else {
                                return;
                            };

                            workspace
                                .open_path((worktree_id, path_str), None, true, window, cx)
                                .detach();
                        });
                    }
                }
                None
            }
            m if m.starts_with(|c: char| c.is_uppercase()) => {
                let mut result: Option<Vec<Anchor>> = None;
                if let Some((path, points)) = self.get_global_mark(text.to_string(), window, cx) {
                    if let Some(workspace) = self.workspace(window) {
                        workspace.update(cx, |workspace, cx| {
                            let Some(worktree) = workspace.worktrees(cx).next() else {
                                return;
                            };
                            let worktree_id = worktree.read(cx).id();
                            let Some(path_str) = path.to_str() else {
                                return;
                            };

                            if let Some(editor) = workspace
                                .active_item(cx)
                                .and_then(|item| item.downcast::<Editor>())
                            {
                                let editor = editor.read(cx);
                                if let Some(file) = editor
                                    .buffer()
                                    .read(cx)
                                    .as_singleton()
                                    .and_then(|buffer| buffer.read(cx).file())
                                {
                                    if *file.path() == path {
                                        // If we are already in the correct file => get the anchor
                                        result = Some(
                                            points
                                                .iter()
                                                .map(|point| {
                                                    let snapshot =
                                                        editor.buffer().read(cx).snapshot(cx);
                                                    snapshot
                                                        .anchor_before(point.to_offset(&snapshot))
                                                })
                                                .collect(),
                                        );
                                    } else {
                                        // If we are in the wrong file => jump to the correct file
                                        workspace
                                            .open_path(
                                                (worktree_id, path_str),
                                                None,
                                                true,
                                                window,
                                                cx,
                                            )
                                            .detach(); //TODO: Somehow get the editor handle for this
                                    }
                                }
                            }
                        });
                    }
                }
                result
            }
            _ => self.get_local_mark(text.to_string(), window, cx), //self.marks.get(&*text).cloned(),
        };

        let Some(anchors) = anchors else { return };

        let is_active_operator = self.active_operator().is_some();
        if is_active_operator {
            if let Some(anchor) = anchors.last() {
                self.motion(
                    Motion::Jump {
                        anchor: *anchor,
                        line,
                    },
                    window,
                    cx,
                )
            }
        } else {
            self.update_editor(window, cx, |_, editor, window, cx| {
                let map = editor.snapshot(window, cx);
                let mut ranges: Vec<Range<Anchor>> = Vec::new();
                for mut anchor in anchors {
                    if line {
                        let mut point = anchor.to_display_point(&map.display_snapshot);
                        point = motion::first_non_whitespace(&map.display_snapshot, false, point);
                        anchor = map
                            .display_snapshot
                            .buffer_snapshot
                            .anchor_before(point.to_point(&map.display_snapshot));
                    }
                    if ranges.last() != Some(&(anchor..anchor)) {
                        ranges.push(anchor..anchor);
                    }
                }
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                    s.select_anchor_ranges(ranges)
                })
            });
        }
    }
}

pub fn jump_motion(
    map: &DisplaySnapshot,
    anchor: Anchor,
    line: bool,
) -> (DisplayPoint, SelectionGoal) {
    let mut point = anchor.to_display_point(map);
    if line {
        point = motion::first_non_whitespace(map, false, point)
    }

    (point, SelectionGoal::None)
}
