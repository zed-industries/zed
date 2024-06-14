use std::{ops::Range, sync::Arc};

use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
    scroll::Autoscroll,
    Anchor, Bias, DisplayPoint,
};
use gpui::WindowContext;
use language::SelectionGoal;

use crate::{
    motion::{self, Motion},
    state::Mode,
    Vim,
};

pub fn create_mark(vim: &mut Vim, text: Arc<str>, tail: bool, cx: &mut WindowContext) {
    let Some(anchors) = vim.update_active_editor(cx, |_, editor, _| {
        editor
            .selections
            .disjoint_anchors()
            .iter()
            .map(|s| if tail { s.tail() } else { s.head() })
            .collect::<Vec<_>>()
    }) else {
        return;
    };
    vim.update_state(|state| state.marks.insert(text.to_string(), anchors));
    vim.clear_operator(cx);
}

pub fn create_visual_marks(vim: &mut Vim, mode: Mode, cx: &mut WindowContext) {
    let mut starts = vec![];
    let mut ends = vec![];
    let mut reversed = vec![];

    vim.update_active_editor(cx, |_, editor, cx| {
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

    vim.update_state(|state| {
        state.marks.insert("<".to_string(), starts);
        state.marks.insert(">".to_string(), ends);
        state.stored_visual_mode.replace((mode, reversed));
    });
    vim.clear_operator(cx);
}

pub fn jump(text: Arc<str>, line: bool, cx: &mut WindowContext) {
    let anchors = Vim::update(cx, |vim, cx| {
        vim.pop_operator(cx);

        match &*text {
            "{" | "}" => vim.update_active_editor(cx, |_, editor, cx| {
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
            "." => vim.state().change_list.last().cloned(),
            _ => vim.state().marks.get(&*text).cloned(),
        }
    });

    let Some(anchors) = anchors else { return };

    let is_active_operator = Vim::read(cx).state().active_operator().is_some();
    if is_active_operator {
        if let Some(anchor) = anchors.last() {
            motion::motion(
                Motion::Jump {
                    anchor: *anchor,
                    line,
                },
                cx,
            )
        }
        return;
    } else {
        Vim::update(cx, |vim, cx| {
            vim.update_active_editor(cx, |_, editor, cx| {
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
