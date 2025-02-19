use std::{ops::Range, sync::Arc};

use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Anchor, Bias, DisplayPoint,
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
        self.marks.insert(text.to_string(), anchors);
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

        self.marks.insert("<".to_string(), starts);
        self.marks.insert(">".to_string(), ends);
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
            _ => self.marks.get(&*text).cloned(),
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
            self.update_editor(window, cx, |vim, editor, window, cx| {
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

                        if vim.mode == Mode::Visual
                            || vim.mode == Mode::VisualLine
                            || vim.mode == Mode::VisualBlock
                        {
                            editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                                s.move_with(|map, selection| {
                                    let was_reversed = selection.reversed;
                                    let mut current_head = selection.head();

                                    // our motions assume the current character is after the cursor,
                                    // but in (forward) visual mode the current character is just
                                    // before the end of the selection.

                                    // If the file ends with a newline (which is common) we don't do this.
                                    // so that if you go to the end of such a file you can use "up" to go
                                    // to the previous line and have it work somewhat as expected.
                                    #[allow(clippy::nonminimal_bool)]
                                    if !selection.reversed
                                        && !selection.is_empty()
                                        && !(selection.end.column() == 0
                                            && selection.end == map.max_point())
                                    {
                                        current_head = movement::left(map, selection.end)
                                    }

                                    selection.set_head(point, SelectionGoal::None);

                                    // ensure the current character is included in the selection.
                                    if !selection.reversed {
                                        let next_point = if vim.mode == Mode::VisualBlock {
                                            movement::saturating_right(map, selection.end)
                                        } else {
                                            movement::right(map, selection.end)
                                        };

                                        if !(next_point.column() == 0
                                            && next_point == map.max_point())
                                        {
                                            selection.end = next_point;
                                        }
                                    }

                                    // vim always ensures the anchor character stays selected.
                                    // if our selection has reversed, we need to move the opposite end
                                    // to ensure the anchor is still selected.
                                    if was_reversed && !selection.reversed {
                                        selection.start = movement::left(map, selection.start);
                                    } else if !was_reversed && selection.reversed {
                                        selection.end = movement::right(map, selection.end);
                                    }
                                })
                            });
                        }
                    }
                    if ranges.last() != Some(&(anchor..anchor)) {
                        ranges.push(anchor..anchor);
                    }
                }

                if vim.mode != Mode::Visual
                    && vim.mode != Mode::VisualLine
                    && vim.mode != Mode::VisualBlock
                {
                    editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                        s.select_anchor_ranges(ranges)
                    })
                }
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
