use language::{char_kind, coerce_punctuation, CharKind};
use multi_buffer::MultiBufferPoint;
use std::{cmp::Ordering, ops::Range};

use editor::{
    display_map::DisplaySnapshot, movement::find_boundary_range, DisplayPoint, Editor, RowRangeExt,
};
use text::Bias;
use ui::ViewContext;

use crate::{
    util::{manh_distance, ranges},
    Direction, WordType,
};

pub fn word_starts_in_range(
    map: &DisplaySnapshot,
    mut from: DisplayPoint,
    to: DisplayPoint,
    full_word: bool,
) -> Vec<DisplayPoint> {
    let scope = map.buffer_snapshot.language_scope_at(from.to_point(map));
    let mut result = Vec::new();

    if from.is_zero() {
        let offset = from.to_offset(map, text::Bias::Left);
        let first_char = map.buffer_snapshot.chars_at(offset).next();
        if let Some(first_char) = first_char {
            if char_kind(&scope, first_char) == CharKind::Word {
                result.push(DisplayPoint::zero());
            }
        }
    }

    while from < to {
        let new_point = find_boundary_range(map, from, to, |left, right| {
            let left_kind = coerce_punctuation(char_kind(&scope, left), false);
            let right_kind = coerce_punctuation(char_kind(&scope, right), false);
            // TODO ignore just punctuation words i.e. ' {} '?
            let found = if full_word {
                left_kind == CharKind::Whitespace && right_kind == CharKind::Word
            } else {
                left_kind != right_kind && right_kind == CharKind::Word
            };

            found
        });

        let Some(new_point) = new_point else {
            break;
        };
        if from == new_point {
            break;
        }
        result.push(new_point);
        from = new_point;
    }
    result
}

pub fn word_starts(
    word_type: WordType,
    direction: Direction,
    editor: &Editor,
    cx: &mut ViewContext<Editor>,
) -> Vec<DisplayPoint> {
    let selections = editor.selections.newest_display(cx);
    let snapshot = editor.snapshot(cx);
    let map = &snapshot.display_snapshot;
    let mut text_layout_details = editor.text_layout_details(cx);
    text_layout_details.vertical_scroll_margin = 0.0;
    let Range { start, end } = ranges(direction, map, &selections, &text_layout_details);
    let full_word = match word_type {
        WordType::Word => false,
        WordType::FullWord => true,
        _ => false,
    };
    word_starts_in_range(&map, start, end, full_word)
}

pub fn row_starts(
    direction: Direction,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Vec<DisplayPoint> {
    let selections = editor.selections.newest_display(cx);
    let snapshot = editor.snapshot(cx);
    let map = &snapshot.display_snapshot;
    let Range { start, end } = ranges(direction, map, &selections, &editor.text_layout_details(cx));
    snapshot
        .buffer_rows(start.row())
        .take((start.row()..end.row()).len())
        .flatten()
        .filter_map(|row| {
            if snapshot.is_line_folded(row) {
                None
            } else {
                Some(map.point_to_display_point(MultiBufferPoint::new(row.0, 0), Bias::Left))
            }
        })
        .collect::<Vec<_>>()
}

pub fn sort_matches_display(matches: &mut [DisplayPoint], cursor: &DisplayPoint) {
    matches.sort_unstable_by(|a, b| {
        let a_distance = manh_distance(a, cursor, 2.5);
        let b_distance = manh_distance(b, cursor, 2.5);
        if a_distance == b_distance {
            Ordering::Equal
        } else if a_distance < b_distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{display_map::DisplayRow, test::marked_display_snapshot};
    use gpui::AppContext;
    use project::Project;
    use settings::SettingsStore;

    fn display_point(x: u32, y: u32) -> DisplayPoint {
        DisplayPoint::new(DisplayRow(y), x)
    }

    fn test_helper(text: &str, list: Vec<DisplayPoint>, full_word: bool, cx: &mut AppContext) {
        let (snapshot, display_points) = marked_display_snapshot(text, cx);
        let point = *display_points.first().unwrap();
        let end = *display_points.last().unwrap();
        let starts = word_starts_in_range(&snapshot, point, end, full_word);
        assert_eq!(starts, list, "full_word: {:?}, text: {}", full_word, text);
    }

    #[gpui::test]
    fn test_get_word_starts(cx: &mut AppContext) {
        init_test(cx);

        let marked_text = "ˇlorem ipsuˇm hi hello ";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(6, 0)],
            false,
            cx,
        );

        let marked_text = "ˇlorem ipsum hi helloˇ";
        test_helper(
            marked_text,
            vec![
                display_point(0, 0),
                display_point(6, 0),
                display_point(12, 0),
                display_point(15, 0),
            ],
            false,
            cx,
        );

        let marked_text = "ˇlorem.ipsum.hi.helloˇ";
        test_helper(
            marked_text,
            vec![
                display_point(0, 0),
                display_point(6, 0),
                display_point(12, 0),
                display_point(15, 0),
            ],
            false,
            cx,
        );

        let marked_text = "ˇ lorem.ipsum.hi.helloˇ";
        test_helper(
            marked_text,
            vec![
                display_point(1, 0),
                display_point(7, 0),
                display_point(13, 0),
                display_point(16, 0),
            ],
            false,
            cx,
        );

        let marked_text = "ˇ lorem.ipsum.hi.helloˇ";
        test_helper(marked_text, vec![display_point(1, 0)], true, cx);

        let marked_text = "ˇlorem.ipsum hi.helloˇ";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(11, 0)],
            true,
            cx,
        );

        let marked_text = "ˇlorem.ipsum\nhi.helloˇ";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(0, 1)],
            true,
            cx,
        );

        let marked_text = "ˇlorem.ipsum \n\n hi.hello \"\"";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(1, 2)],
            true,
            cx,
        );

        let marked_text = "ˇlorem ipsum \n{}\n hi hello \"\"";
        test_helper(
            marked_text,
            vec![
                display_point(0, 0),
                display_point(6, 0),
                display_point(1, 2),
                display_point(4, 2),
            ],
            true,
            cx,
        );
    }

    fn init_test(cx: &mut gpui::AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}
