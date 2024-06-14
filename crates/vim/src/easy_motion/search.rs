use std::{cmp::Ordering, ops::Range};

use editor::{
    display_map::{DisplayRow, DisplaySnapshot},
    movement::{find_boundary_range_fold, TextLayoutDetails},
    DisplayPoint, Editor, RowExt, RowRangeExt,
};
use language::{char_kind, CharKind, LanguageScope};
use multi_buffer::MultiBufferPoint;
use text::{Bias, Selection};
use ui::ViewContext;

use crate::{
    easy_motion::{Direction, WordType},
    motion::{window_bottom, window_top},
};

pub fn manh_distance(point_1: &DisplayPoint, point_2: &DisplayPoint, x_bias: f32) -> f32 {
    x_bias * (point_1.row().as_f32() - point_2.row().as_f32()).abs()
        + (point_1.column() as i32 - point_2.column() as i32).abs() as f32
}

fn is_boundary(scope: &Option<LanguageScope>, full_word: bool, left: char, right: char) -> bool {
    let left_kind = char_kind(&scope, left);
    let right_kind = char_kind(&scope, right);

    let found = if full_word {
        left_kind == CharKind::Whitespace && right_kind == CharKind::Word
    } else {
        left_kind != right_kind && right_kind == CharKind::Word
    };

    found
}

// returns a display point range from the current selection to the start/end
// for a direction of backwards/forwards respectively or the full window for
// bidirectional
pub fn ranges(
    direction: Direction,
    map: &DisplaySnapshot,
    selection: &Selection<DisplayPoint>,
    text_layout_details: &TextLayoutDetails,
) -> Range<DisplayPoint> {
    let start = match direction {
        Direction::Both | Direction::Backwards => {
            let times = if text_layout_details.scroll_anchor.offset.y == 0. {
                0
            } else {
                1
            };
            window_top(map, DisplayPoint::zero(), &text_layout_details, times).0
        }
        Direction::Forwards => selection.end,
    };
    let end = match direction {
        Direction::Both | Direction::Forwards => {
            window_bottom(
                map,
                DisplayPoint::new(DisplayRow(0), u32::max_value()),
                &text_layout_details,
                0,
            )
            .0
        }
        Direction::Backwards => selection.start,
    };
    start..end
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

// TODO investigate interaction with inlay hints
pub fn word_starts_fold(
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
    // TODO subword
    let full_word = match word_type {
        WordType::Word => false,
        WordType::FullWord => true,
        _ => false,
    };

    word_starts_in_range_fold(map, start, end, full_word)
}

pub fn word_starts_in_range_fold(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    to: DisplayPoint,
    full_word: bool,
) -> Vec<DisplayPoint> {
    let scope = map.buffer_snapshot.language_scope_at(from.to_point(map));
    let mut results = Vec::new();

    let fold_snapshot = &map.fold_snapshot;

    if from.is_zero() {
        let first_char = fold_snapshot
            .chars_at(map.display_point_to_fold_point(from, Bias::Right))
            .next();
        if let Some(first_char) = first_char {
            if char_kind(&scope, first_char) == CharKind::Word {
                results.push(DisplayPoint::zero());
            }
        }
    }

    let mut from = map.display_point_to_fold_point(from, Bias::Right);
    let to = map.display_point_to_fold_point(to, Bias::Right);
    while from < to {
        let new_point = find_boundary_range_fold(fold_snapshot, from, to, |left, right| {
            is_boundary(&scope, full_word, left, right)
        });

        let Some(new_point) = new_point else {
            break;
        };
        if from == new_point {
            break;
        }
        let new_point_display = map.fold_point_to_display_point(new_point);
        results.push(new_point_display);
        from = new_point;
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{
        display_map::{DisplayMap, DisplayRow},
        test::marked_display_snapshot,
        FoldPlaceholder, MultiBuffer,
    };
    use gpui::{font, px, AppContext, Model, TestAppContext};
    // use indoc::indoc;
    use project::Project;
    use rope::Point;
    use settings::SettingsStore;
    use ui::Context;

    fn display_point(x: u32, y: u32) -> DisplayPoint {
        DisplayPoint::new(DisplayRow(y), x)
    }

    fn test_helper_fold(text: &str, list: Vec<DisplayPoint>, full_word: bool, cx: &mut AppContext) {
        let (snapshot, display_points) = marked_display_snapshot(text, cx);
        let point = *display_points.first().unwrap();
        let end = *display_points.last().unwrap();
        let starts = word_starts_in_range_fold(&snapshot, point, end, full_word);
        assert_eq!(starts, list, "full_word: {:?}, text: {}", full_word, text);
    }

    #[gpui::test]
    fn test_easy_get_word_starts(cx: &mut AppContext) {
        init_test(cx);

        let marked_text = "ˇlorem ipsuˇm hi hello ";
        test_helper_fold(
            marked_text,
            vec![display_point(0, 0), display_point(6, 0)],
            false,
            cx,
        );

        let marked_text = "ˇlorem.ipsum \n\n hi.hello \"\"ˇ";
        test_helper_fold(
            marked_text,
            vec![display_point(0, 0), display_point(1, 2)],
            true,
            cx,
        );

        let marked_text = "ˇlorem ipsum \n{}\n hi hello \"\"ˇ";
        test_helper_fold(
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

    fn display_map_helper(text: &str, cx: &mut TestAppContext) -> Model<DisplayMap> {
        let buffer_start_excerpt_header_height = 1;
        let excerpt_header_height = 1;
        let font_size = px(14.0);
        let wrap_width = Some(px(0.0));

        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));

        cx.new_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                font("Helvetica"),
                font_size,
                wrap_width,
                true,
                buffer_start_excerpt_header_height,
                excerpt_header_height,
                0,
                FoldPlaceholder::test(),
                cx,
            )
        })
    }

    #[gpui::test]
    fn test_easy_get_word_starts_with_folds(cx: &mut TestAppContext) {
        cx.update(|cx| init_test(cx));

        let map = display_map_helper("lorem ipsum hi hello", cx);
        let snapshot = map.update(cx, |map, cx| {
            let folds = vec![
                (Point::new(0, 2)..Point::new(0, 8), FoldPlaceholder::test()),
                (
                    Point::new(0, 11)..Point::new(0, 16),
                    FoldPlaceholder::test(),
                ),
            ];
            map.fold(folds, cx);
            map.snapshot(cx)
        });

        assert_eq!(snapshot.fold_snapshot.text(), "lo⋯sum⋯ello");

        let starts =
            word_starts_in_range_fold(&snapshot, DisplayPoint::zero(), snapshot.max_point(), false);
        assert_eq!(
            starts,
            [
                display_point(0, 0),
                display_point(0, 3),
                display_point(0, 7)
            ]
        );

        // TODO - get this test working
        // everything is appearing on a single line and I don't know why

        // let text = indoc! {r#"
        //     fn hi() {
        //     print!("hi");
        //     }

        //     fn main() {
        //     hi();
        //     }
        // "#};
        // dbg!(text);
        // let map = display_map_helper(text, cx);
        // let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        // dbg!(snapshot.fold_snapshot.text());
        // dbg!(snapshot.text());

        // let starts =
        //     word_starts_in_range_fold(&snapshot, DisplayPoint::zero(), snapshot.max_point(), false);
        // assert_eq!(
        //     starts,
        //     [
        //         display_point(0, 0),
        //         display_point(0, 3),
        //         display_point(1, 1),
        //         display_point(1, 8),
        //         display_point(3, 0),
        //         display_point(3, 3),
        //         display_point(4, 1),
        //     ]
        // );

        // let snapshot = map.update(cx, |map, cx| {
        //     let folds = vec![(Point::new(0, 9)..Point::new(1, 17), FoldPlaceholder::test())];
        //     map.fold(folds, cx);
        //     map.snapshot(cx)
        // });

        // let output_text = indoc! {r#"
        //     fn hi() {⋯
        //     }

        //     fn main() {
        //         hi();
        //     }
        // "#};
        // assert_eq!(snapshot.fold_snapshot.text(), output_text);

        // let starts =
        //     word_starts_in_range_fold(&snapshot, DisplayPoint::zero(), snapshot.max_point(), false);
        // assert_eq!(
        //     starts,
        //     [
        //         display_point(0, 0),
        //         display_point(0, 3),
        //         display_point(1, 1),
        //         display_point(1, 8),
        //         display_point(3, 0),
        //         display_point(3, 3),
        //         display_point(4, 1),
        //     ]
        // );
    }

    fn init_test(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        Project::init_settings(cx);
    }
}
