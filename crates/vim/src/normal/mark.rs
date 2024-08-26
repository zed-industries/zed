use std::{ops::Range, sync::Arc};

use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Anchor, Bias, DisplayPoint,
};
use gpui::ViewContext;
use language::SelectionGoal;

use crate::{
    motion::{self, Motion},
    state::Mode,
    Vim,
};

impl Vim {
    pub fn create_mark(&mut self, text: Arc<str>, tail: bool, cx: &mut ViewContext<Self>) {
        let Some(anchors) = self.update_editor(cx, |_, editor, _| {
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
        self.clear_operator(cx);
    }

    // When handling an action, you must create visual marks if you will switch to normal
    // mode without the default selection behavior.
    pub(crate) fn store_visual_marks(&mut self, cx: &mut ViewContext<Self>) {
        if self.mode.is_visual() {
            self.create_visual_marks(self.mode, cx);
        }
    }

    pub(crate) fn create_visual_marks(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        let mut starts = vec![];
        let mut ends = vec![];
        let mut reversed = vec![];

        self.update_editor(cx, |_, editor, cx| {
            let (map, selections) = editor.selections.all_display(cx);
            for selection in selections {
                let end = movement::saturating_left(&map, selection.end);
                ends.push(
                    map.buffer_snapshot
                        .anchor_before(end.to_offset(&map, Bias::Left)),
                );
                starts.push(
                    map.buffer_snapshot
                        .anchor_after(selection.start.to_offset(&map, Bias::Right)),
                );
                reversed.push(selection.reversed)
            }
        });

        self.marks.insert("<".to_string(), starts);
        self.marks.insert(">".to_string(), ends);
        self.stored_visual_mode.replace((mode, reversed));
        self.clear_operator(cx);
    }

    pub fn jump(&mut self, text: Arc<str>, line: bool, cx: &mut ViewContext<Self>) {
        self.pop_operator(cx);

        let anchors = match &*text {
            "{" | "}" => self.update_editor(cx, |_, editor, cx| {
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
                    cx,
                )
            }
            return;
        } else {
            self.update_editor(cx, |_, editor, cx| {
                let map = editor.snapshot(cx);
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
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
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
