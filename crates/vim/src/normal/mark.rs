use std::{ops::Range, path::Path, sync::Arc};

use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Anchor, Bias, DisplayPoint, Editor, MultiBuffer,
};
use gpui::{Context, Entity, EntityId, UpdateGlobal, Window};
use language::SelectionGoal;
use text::Point;
use ui::App;
use workspace::OpenOptions;

use crate::{
    motion::{self, Motion},
    state::{Mark, Mode, VimGlobals},
    Vim,
};

impl Vim {
    pub fn create_mark(&mut self, text: Arc<str>, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(window, cx, |vim, editor, window, cx| {
            let anchors = editor
                .selections
                .disjoint_anchors()
                .iter()
                .map(|s| s.head())
                .collect::<Vec<_>>();
            vim.set_mark(text.to_string(), anchors, editor.buffer(), window, cx);
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

        self.update_editor(window, cx, |vim, editor, window, cx| {
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
            vim.set_mark("<".to_string(), starts, editor.buffer(), window, cx);
            vim.set_mark(">".to_string(), ends, editor.buffer(), window, cx);
        });

        self.stored_visual_mode.replace((mode, reversed));
    }

    fn open_buffer_mark(
        &mut self,
        line: bool,
        entity_id: EntityId,
        anchors: Vec<Anchor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace(window) else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            let item = workspace.items(cx).find(|item| {
                item.act_as::<Editor>(cx)
                    .is_some_and(|editor| editor.read(cx).buffer().entity_id() == entity_id)
            });
            let Some(item) = item.cloned() else {
                return;
            };
            if let Some(pane) = workspace.pane_for(item.as_ref()) {
                pane.update(cx, |pane, cx| {
                    if let Some(index) = pane.index_for_item(item.as_ref()) {
                        pane.activate_item(index, true, true, window, cx);
                    }
                });
            };

            item.act_as::<Editor>(cx).unwrap().update(cx, |editor, cx| {
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
                });
            })
        });
        return;
    }

    fn open_path_mark(
        &mut self,
        line: bool,
        path: Arc<Path>,
        points: Vec<Point>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace(window) else {
            return;
        };
        let task = workspace.update(cx, |workspace, cx| {
            workspace.open_abs_path(
                path.to_path_buf(),
                OpenOptions {
                    visible: Some(workspace::OpenVisible::All),
                    focus: Some(true),
                    ..Default::default()
                },
                window,
                cx,
            )
        });
        cx.spawn_in(window, |this, mut cx| async move {
            let editor = task.await?;
            this.update_in(&mut cx, |_, window, cx| {
                if let Some(editor) = editor.act_as::<Editor>(cx) {
                    editor.update(cx, |editor, cx| {
                        let map = editor.snapshot(window, cx);
                        let points: Vec<_> = points
                            .into_iter()
                            .map(|p| {
                                if line {
                                    let point = p.to_display_point(&map.display_snapshot);
                                    motion::first_non_whitespace(
                                        &map.display_snapshot,
                                        false,
                                        point,
                                    )
                                    .to_point(&map.display_snapshot)
                                } else {
                                    p
                                }
                            })
                            .collect();
                        editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                            s.select_ranges(points.into_iter().map(|p| p..p))
                        })
                    })
                }
            })
        })
        .detach_and_log_err(cx);
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
            _ => {
                let Some(buffer) = self.editor().map(|editor| editor.read(cx).buffer().clone())
                else {
                    return;
                };
                let mark = self.get_mark(&text, &buffer, window, cx);
                match mark {
                    None => None,
                    Some(Mark::Local(anchors)) => Some(anchors),
                    Some(Mark::Buffer(entity_id, anchors)) => {
                        self.open_buffer_mark(line, entity_id, anchors, window, cx);
                        return;
                    }
                    Some(Mark::Path(path, points)) => {
                        self.open_path_mark(line, path, points, window, cx);
                        return;
                    }
                }
            }
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

                if !should_jump && !ranges.is_empty() {
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

    pub fn set_mark(
        &mut self,
        name: String,
        anchors: Vec<Anchor>,
        buffer_entity: &Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(workspace) = self.workspace(window) else {
            return;
        };
        let entity_id = workspace.entity_id();
        Vim::update_globals(cx, |vim_globals, cx| {
            let Some(marks_state) = vim_globals.marks.get(&entity_id) else {
                return;
            };
            marks_state.update(cx, |ms, cx| {
                ms.set_mark(name.clone(), buffer_entity, anchors, cx);
            });
        });
    }

    pub fn get_mark(
        &self,
        name: &str,
        multi_buffer: &Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Mark> {
        VimGlobals::update_global(cx, |globals, cx| {
            let workspace_id = self.workspace(window)?.entity_id();
            globals
                .marks
                .get_mut(&workspace_id)?
                .update(cx, |ms, cx| ms.get_mark(name, multi_buffer, cx))
        })
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
