use std::{ops::Range, sync::Arc};

use collections::HashSet;
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    Anchor, DisplayPoint,
};
use gpui::WindowContext;
use language::SelectionGoal;

use crate::{
    motion::{self, Motion},
    Vim,
};

pub fn create_mark(vim: &mut Vim, text: Arc<str>, cx: &mut WindowContext) {
    let Some(anchors) = vim.update_active_editor(cx, |_, editor, _| {
        editor
            .selections
            .disjoint_anchors()
            .iter()
            .map(|s| s.head().clone())
            .collect::<Vec<_>>()
    }) else {
        return;
    };

    vim.update_state(|state| state.marks.insert(text.to_string(), anchors));
    vim.clear_operator(cx);
}

pub fn jump(text: Arc<str>, line: bool, cx: &mut WindowContext) {
    let Some(anchors) = Vim::read(cx).state().marks.get(&*text).cloned() else {
        return;
    };

    Vim::update(cx, |vim, cx| {
        vim.pop_operator(cx);
    });

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
                let mut ranges: HashSet<Range<Anchor>> = HashSet::default();
                for mut anchor in anchors {
                    if line {
                        let mut point = anchor.to_display_point(&map.display_snapshot);
                        point = motion::first_non_whitespace(&map.display_snapshot, false, point);
                        anchor = map
                            .display_snapshot
                            .buffer_snapshot
                            .anchor_before(point.to_point(&map.display_snapshot));
                    }
                    ranges.insert(anchor..anchor);
                }
                editor.change_selections(None, cx, |s| s.select_anchor_ranges(ranges))
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
