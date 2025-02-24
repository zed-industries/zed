use std::{ops::Range, sync::Arc};

use anyhow::Ok;
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Anchor, Bias, DisplayPoint, Editor,
};
use gpui::{Context, UpdateGlobal, Window};
use language::SelectionGoal;

use crate::{
    motion::{self, Motion},
    state::{Mode, VimGlobals},
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
        self.update_editor(window, cx, |vim, editor, window, cx| {
            let anchors = editor
                .selections
                .disjoint_anchors()
                .iter()
                .map(|s| if tail { s.tail() } else { s.head() })
                .collect::<Vec<_>>();
            if let Some(workspace) = vim.workspace(window) {
                if let Some(id) = workspace.read(cx).database_id() {
                    let multi_buffer = editor.buffer();
                    if let Some(buffer) = multi_buffer.read(cx).as_singleton() {
                        vim.set_mark(text.to_string(), anchors, &buffer, multi_buffer, id, cx);
                    }
                }
            }
        });
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

        self.update_editor(window, cx, |vim, editor, _, cx| {
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
            let Some(workspace) = editor.workspace() else {
                return;
            };
            let Some(wid) = workspace.read(cx).database_id() else {
                return;
            };
            let multi_buffer = editor.buffer();
            let Some(buffer) = multi_buffer.read(cx).as_singleton() else {
                return;
            };
            vim.set_mark("<".to_string(), starts, &buffer, multi_buffer, wid, cx);
            vim.set_mark(">".to_string(), ends, &buffer, multi_buffer, wid, cx);
        });

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
                if let Some(either) = self.get_global_mark_identifier(text.to_string(), window, cx)
                {
                    match either {
                        itertools::Either::Left(path) => {
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
                                        .detach_and_log_err(cx);
                                });
                            }
                        }
                        itertools::Either::Right(buffer_id) => {
                            let Some(workspace) = self.workspace(window) else {
                                return;
                            };
                            workspace.update(cx, |workspace, cx| {
                                let mut panes = workspace.panes().to_vec();
                                panes.insert(0, workspace.active_pane().clone());
                                let name = text.to_string().clone();
                                for pane in panes {
                                    pane.update(cx, |pane, cx| {
                                        let Some(item_handle) =
                                            pane.items().find(|&item_handle| -> bool {
                                                let Some(editor) = item_handle.act_as::<Editor>(cx)
                                                else {
                                                    return false;
                                                };
                                                let Some(buffer) = editor
                                                    .read(cx)
                                                    .buffer()
                                                    .read(cx)
                                                    .as_singleton()
                                                else {
                                                    return false;
                                                };
                                                buffer_id == buffer.read(cx).remote_id()
                                            })
                                        else {
                                            return;
                                        };
                                        let Some(editor) = item_handle.act_as::<Editor>(cx) else {
                                            return;
                                        };

                                        let Some(index) = pane.index_for_item(&editor) else {
                                            return;
                                        };
                                        pane.activate_item(index, true, true, window, cx);
                                    });
                                }
                            });
                        }
                    }
                }
                None
            }
            m if m.starts_with(|c: char| c.is_uppercase()) => {
                if let Some(either) = self.get_global_mark_identifier(text.to_string(), window, cx)
                {
                    match either {
                        itertools::Either::Left(path) => {
                            if let Some(workspace) = self.workspace(window) {
                                workspace.update(cx, |workspace, cx| {
                                    let Some(workspace_id) = workspace.database_id() else {
                                        return;
                                    };
                                    let Some(worktree) = workspace.worktrees(cx).next() else {
                                        return;
                                    };
                                    let worktree_id = worktree.read(cx).id();
                                    let Some(path_str) = path.to_str() else {
                                        return;
                                    };

                                    let task = workspace.open_path(
                                        (worktree_id, path_str),
                                        None,
                                        true,
                                        window,
                                        cx,
                                    );
                                    cx.spawn_in(window, |_, mut cx| async move {
                                        let item = task.await?;

                                        if let Some(editor) = cx
                                            .update(|_, cx| item.act_as::<Editor>(cx))
                                            .ok()
                                            .flatten()
                                        {
                                            editor.update_in(&mut cx, |editor, window, cx| {
                                                if let Some(anchors) = VimGlobals::update_global(
                                                    cx,
                                                    |vim_globals, cx| {
                                                        vim_globals
                                                            .marks
                                                            .get(&workspace_id)?
                                                            .update(cx, |ms, cx| {
                                                                let multi_buffer = editor.buffer();
                                                                let buffer = multi_buffer
                                                                    .read(cx)
                                                                    .as_singleton()?;
                                                                ms.get_mark(
                                                                    text.to_string(),
                                                                    &buffer,
                                                                    multi_buffer,
                                                                    cx,
                                                                )
                                                            })
                                                    },
                                                ) {
                                                    editor.change_selections(
                                                        Some(Autoscroll::fit()),
                                                        window,
                                                        cx,
                                                        |s| {
                                                            s.select_anchor_ranges(
                                                                anchors
                                                                    .iter()
                                                                    .map(|&anchor| anchor..anchor),
                                                            );
                                                        },
                                                    )
                                                }
                                            })?;
                                        }
                                        Ok(())
                                    })
                                    .detach_and_log_err(cx);
                                });
                            }
                        }
                        itertools::Either::Right(buffer_id) => {
                            let Some(workspace) = self.workspace(window) else {
                                return;
                            };
                            workspace.update(cx, |workspace, cx| {
                                let Some(workspace_id) = workspace.database_id() else {
                                    return;
                                };

                                // This currently on checks the current pan. This should pro
                                let mut panes = workspace.panes().to_vec();
                                panes.insert(0, workspace.active_pane().clone());
                                let name = text.to_string().clone();
                                for pane in panes {
                                    let mut found = false;
                                    let name = name.clone();
                                    pane.update(cx, |pane, cx| {
                                        let Some(item_handle) =
                                            pane.items().find(|&item_handle| -> bool {
                                                let Some(editor) = item_handle.act_as::<Editor>(cx)
                                                else {
                                                    return false;
                                                };
                                                let Some(buffer) = editor
                                                    .read(cx)
                                                    .buffer()
                                                    .read(cx)
                                                    .as_singleton()
                                                else {
                                                    return false;
                                                };
                                                buffer_id == buffer.read(cx).remote_id()
                                            })
                                        else {
                                            return;
                                        };
                                        found = true;
                                        let Some(editor) = item_handle.act_as::<Editor>(cx) else {
                                            return;
                                        };

                                        let Some(index) = pane.index_for_item(&editor) else {
                                            return;
                                        };
                                        pane.activate_item(index, true, true, window, cx);

                                        cx.spawn_in(window, |_, mut cx| async move {
                                            editor.update_in(&mut cx, |editor, window, cx| {
                                                if let Some(anchors) = VimGlobals::update_global(
                                                    cx,
                                                    |vim_globals, cx| {
                                                        vim_globals
                                                            .marks
                                                            .get(&workspace_id)?
                                                            .update(cx, |ms, cx| {
                                                                let multi_buffer = editor.buffer();
                                                                let buffer = multi_buffer
                                                                    .read(cx)
                                                                    .as_singleton()?;
                                                                ms.get_mark(
                                                                    name,
                                                                    &buffer,
                                                                    multi_buffer,
                                                                    cx,
                                                                )
                                                            })
                                                    },
                                                ) {
                                                    editor.change_selections(
                                                        Some(Autoscroll::fit()),
                                                        window,
                                                        cx,
                                                        |s| {
                                                            s.select_anchor_ranges(
                                                                anchors
                                                                    .iter()
                                                                    .map(|&anchor| anchor..anchor),
                                                            );
                                                        },
                                                    )
                                                }
                                            })?;
                                            Ok(())
                                        })
                                        .detach_and_log_err(cx);
                                    });
                                    if found {
                                        break;
                                    }
                                }
                            });
                        }
                    }
                }
                return;
            }
            _ => self.get_local_mark(text.to_string(), window, cx),
        };

        let Some(mut anchors) = anchors else { return };

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
            // Save the last anchor so as to jump to it later.
            let anchor: Option<Anchor> = anchors.last_mut().map(|anchor| *anchor);
            let should_jump = self.mode == Mode::Visual
                || self.mode == Mode::VisualLine
                || self.mode == Mode::VisualBlock;

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

                if !should_jump {
                    editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                        s.select_anchor_ranges(ranges)
                    });
                }
            });

            if should_jump {
                if let Some(anchor) = anchor {
                    self.motion(Motion::Jump { anchor, line }, window, cx)
                }
            }
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
