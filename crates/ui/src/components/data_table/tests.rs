use super::table_row::TableRow;
use crate::{RedistributableColumnsState, ResizableColumnsState, TableResizeBehavior};
use gpui::{AbsoluteLength, px};

fn is_almost_eq(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-6)
}

fn cols_to_str(cols: &[f32], total_size: f32) -> String {
    cols.iter()
        .map(|f| "*".repeat(f32::round(f * total_size) as usize))
        .collect::<Vec<String>>()
        .join("|")
}

fn parse_resize_behavior(
    input: &str,
    total_size: f32,
    expected_cols: usize,
) -> Vec<TableResizeBehavior> {
    let mut resize_behavior = Vec::with_capacity(expected_cols);
    for col in input.split('|') {
        if col.starts_with('X') || col.is_empty() {
            resize_behavior.push(TableResizeBehavior::None);
        } else if col.starts_with('*') {
            resize_behavior.push(TableResizeBehavior::MinSize(col.len() as f32 / total_size));
        } else {
            panic!("invalid test input: unrecognized resize behavior: {}", col);
        }
    }

    if resize_behavior.len() != expected_cols {
        panic!(
            "invalid test input: expected {} columns, got {}",
            expected_cols,
            resize_behavior.len()
        );
    }
    resize_behavior
}

mod reset_column_size {
    use super::*;

    fn parse(input: &str) -> (Vec<f32>, f32, Option<usize>) {
        let mut widths = Vec::new();
        let mut column_index = None;
        for (index, col) in input.split('|').enumerate() {
            widths.push(col.len() as f32);
            if col.starts_with('X') {
                column_index = Some(index);
            }
        }

        for w in &widths {
            assert!(w.is_finite(), "incorrect number of columns");
        }
        let total = widths.iter().sum::<f32>();
        for width in &mut widths {
            *width /= total;
        }
        (widths, total, column_index)
    }

    #[track_caller]
    fn check_reset_size(initial_sizes: &str, widths: &str, expected: &str, resize_behavior: &str) {
        let (initial_sizes, total_1, None) = parse(initial_sizes) else {
            panic!("invalid test input: initial sizes should not be marked");
        };
        let (widths, total_2, Some(column_index)) = parse(widths) else {
            panic!("invalid test input: widths should be marked");
        };
        assert_eq!(
            total_1, total_2,
            "invalid test input: total width not the same {total_1}, {total_2}"
        );
        let (expected, total_3, None) = parse(expected) else {
            panic!("invalid test input: expected should not be marked: {expected:?}");
        };
        assert_eq!(
            total_2, total_3,
            "invalid test input: total width not the same"
        );
        let cols = initial_sizes.len();
        let resize_behavior_vec = parse_resize_behavior(resize_behavior, total_1, cols);
        let resize_behavior = TableRow::from_vec(resize_behavior_vec, cols);
        let result = RedistributableColumnsState::reset_to_initial_size(
            column_index,
            TableRow::from_vec(widths, cols),
            TableRow::from_vec(initial_sizes, cols),
            &resize_behavior,
        );
        let result_slice = result.as_slice();
        let is_eq = is_almost_eq(result_slice, &expected);
        if !is_eq {
            let result_str = cols_to_str(result_slice, total_1);
            let expected_str = cols_to_str(&expected, total_1);
            panic!(
                "resize failed\ncomputed: {result_str}\nexpected: {expected_str}\n\ncomputed values: {result_slice:?}\nexpected values: {expected:?}\n:minimum widths: {resize_behavior:?}"
            );
        }
    }

    macro_rules! check_reset_size {
        (columns: $cols:expr, starting: $initial:expr, snapshot: $current:expr, expected: $expected:expr, resizing: $resizing:expr $(,)?) => {
            check_reset_size($initial, $current, $expected, $resizing);
        };
        ($name:ident, columns: $cols:expr, starting: $initial:expr, snapshot: $current:expr, expected: $expected:expr, minimums: $resizing:expr $(,)?) => {
            #[test]
            fn $name() {
                check_reset_size($initial, $current, $expected, $resizing);
            }
        };
    }

    check_reset_size!(
        basic_right,
        columns: 5,
        starting: "**|**|**|**|**",
        snapshot: "**|**|X|***|**",
        expected: "**|**|**|**|**",
        minimums: "X|*|*|*|*",
    );

    check_reset_size!(
        basic_left,
        columns: 5,
        starting: "**|**|**|**|**",
        snapshot: "**|**|***|X|**",
        expected: "**|**|**|**|**",
        minimums: "X|*|*|*|**",
    );

    check_reset_size!(
        squashed_left_reset_col2,
        columns: 6,
        starting: "*|***|**|**|****|*",
        snapshot: "*|*|X|*|*|********",
        expected: "*|*|**|*|*|*******",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        grow_cascading_right,
        columns: 6,
        starting: "*|***|****|**|***|*",
        snapshot: "*|***|X|**|**|*****",
        expected: "*|***|****|*|*|****",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
       squashed_right_reset_col4,
       columns: 6,
       starting: "*|***|**|**|****|*",
       snapshot: "*|********|*|*|X|*",
       expected: "*|*****|*|*|****|*",
       minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        reset_col6_right,
        columns: 6,
        starting: "*|***|**|***|***|**",
        snapshot: "*|***|**|***|**|XXX",
        expected: "*|***|**|***|***|**",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        reset_col6_left,
        columns: 6,
        starting: "*|***|**|***|***|**",
        snapshot: "*|***|**|***|****|X",
        expected: "*|***|**|***|***|**",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        last_column_grow_cascading,
        columns: 6,
        starting: "*|***|**|**|**|***",
        snapshot: "*|*******|*|**|*|X",
        expected: "*|******|*|*|*|***",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        goes_left_when_left_has_extreme_diff,
        columns: 6,
        starting: "*|***|****|**|**|***",
        snapshot: "*|********|X|*|**|**",
        expected: "*|*****|****|*|**|**",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        basic_shrink_right,
        columns: 6,
        starting: "**|**|**|**|**|**",
        snapshot: "**|**|XXX|*|**|**",
        expected: "**|**|**|**|**|**",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        shrink_should_go_left,
        columns: 6,
        starting: "*|***|**|*|*|*",
        snapshot: "*|*|XXX|**|*|*",
        expected: "*|**|**|**|*|*",
        minimums: "X|*|*|*|*|*",
    );

    check_reset_size!(
        shrink_should_go_right,
        columns: 6,
        starting: "*|***|**|**|**|*",
        snapshot: "*|****|XXX|*|*|*",
        expected: "*|****|**|**|*|*",
        minimums: "X|*|*|*|*|*",
    );
}

mod drag_handle {
    use super::*;

    fn parse(input: &str) -> (Vec<f32>, f32, Option<usize>) {
        let mut widths = Vec::new();
        let column_index = input.replace("*", "").find("I");
        for col in input.replace("I", "|").split('|') {
            widths.push(col.len() as f32);
        }

        for w in &widths {
            assert!(w.is_finite(), "incorrect number of columns");
        }
        let total = widths.iter().sum::<f32>();
        for width in &mut widths {
            *width /= total;
        }
        (widths, total, column_index)
    }

    #[track_caller]
    fn check(distance: i32, widths: &str, expected: &str, resize_behavior: &str) {
        let (widths, total_1, Some(column_index)) = parse(widths) else {
            panic!("invalid test input: widths should be marked");
        };
        let (expected, total_2, None) = parse(expected) else {
            panic!("invalid test input: expected should not be marked: {expected:?}");
        };
        assert_eq!(
            total_1, total_2,
            "invalid test input: total width not the same"
        );
        let cols = widths.len();
        let resize_behavior_vec = parse_resize_behavior(resize_behavior, total_1, cols);
        let resize_behavior = TableRow::from_vec(resize_behavior_vec, cols);

        let distance = distance as f32 / total_1;

        let mut widths_table_row = TableRow::from_vec(widths, cols);
        RedistributableColumnsState::drag_column_handle(
            distance,
            column_index,
            &mut widths_table_row,
            &resize_behavior,
        );

        let result_widths = widths_table_row.as_slice();
        let is_eq = is_almost_eq(result_widths, &expected);
        if !is_eq {
            let result_str = cols_to_str(result_widths, total_1);
            let expected_str = cols_to_str(&expected, total_1);
            panic!(
                "resize failed\ncomputed: {result_str}\nexpected: {expected_str}\n\ncomputed values: {result_widths:?}\nexpected values: {expected:?}\n:minimum widths: {resize_behavior:?}"
            );
        }
    }

    macro_rules! check {
        (columns: $cols:expr, distance: $dist:expr, snapshot: $current:expr, expected: $expected:expr, resizing: $resizing:expr $(,)?) => {
            check($dist, $current, $expected, $resizing);
        };
        ($name:ident, columns: $cols:expr, distance: $dist:expr, snapshot: $current:expr, expected: $expected:expr, minimums: $resizing:expr $(,)?) => {
            #[test]
            fn $name() {
                check($dist, $current, $expected, $resizing);
            }
        };
    }

    check!(
        basic_right_drag,
        columns: 3,
        distance: 1,
        snapshot: "**|**I**",
        expected: "**|***|*",
        minimums: "X|*|*",
    );

    check!(
        drag_left_against_mins,
        columns: 5,
        distance: -1,
        snapshot: "*|*|*|*I*******",
        expected: "*|*|*|*|*******",
        minimums: "X|*|*|*|*",
    );

    check!(
        drag_left,
        columns: 5,
        distance: -2,
        snapshot: "*|*|*|*****I***",
        expected: "*|*|*|***|*****",
        minimums: "X|*|*|*|*",
    );
}

mod drag_with_hidden_columns {
    use super::*;

    // Dragging with hidden (filtered) columns: the resize dividers are laid out using the
    // *redistributed* (visible-only) widths, so `compute_drag_preview` must do its geometry in
    // that same space and skip hidden columns when propagating the resize to a neighbor.

    /// Mirrors how the renderer turns raw widths into the on-screen layout: hidden columns
    /// collapse to zero and their space is redistributed across the visible columns.
    fn redistributed(widths: &[f32], hidden: &[bool]) -> Vec<f32> {
        let visible_sum: f32 = widths
            .iter()
            .zip(hidden)
            .filter(|(_, is_hidden)| !**is_hidden)
            .map(|(width, _)| *width)
            .sum();
        widths
            .iter()
            .zip(hidden)
            .map(|(width, is_hidden)| {
                if *is_hidden {
                    0.0
                } else {
                    *width / visible_sum
                }
            })
            .collect()
    }

    #[test]
    fn drag_without_hidden_columns_is_unchanged() {
        // Guards the pre-existing behavior: with no hidden mask the drag operates on the raw
        // widths directly (the visible space and the raw space are the same).
        let resize_behavior = TableRow::from_vec(vec![TableResizeBehavior::Resizable; 3], 3);
        let widths = TableRow::from_vec(vec![1.0 / 3.0; 3], 3);

        let result = RedistributableColumnsState::compute_drag_preview(
            widths,
            &resize_behavior,
            None,
            1,
            0.8,
            0.0,
        );

        let result = result.as_slice();
        let boundary = result[0] + result[1];
        assert!(
            (boundary - 0.8).abs() < 1e-6,
            "expected the boundary after column 1 to follow the cursor to 0.8: {result:?}",
        );
        assert!(
            (result[0] - 1.0 / 3.0).abs() < 1e-6,
            "column 0 must not be affected: {result:?}",
        );
    }

    #[test]
    fn drag_boundary_follows_cursor_with_hidden_column() {
        // Three equal columns; column 0 is hidden. The two visible columns each render at 0.5
        // of the container. The user grabs the divider between the visible columns (original
        // index 1) and drags the cursor to 70% of the container. The boundary between the
        // visible columns should follow the cursor to 0.7.
        let resize_behavior = TableRow::from_vec(vec![TableResizeBehavior::Resizable; 3], 3);
        let widths = TableRow::from_vec(vec![1.0 / 3.0; 3], 3);
        let hidden = [true, false, false];
        let hidden_mask = TableRow::from_vec(hidden.to_vec(), 3);

        let result = RedistributableColumnsState::compute_drag_preview(
            widths,
            &resize_behavior,
            Some(&hidden_mask),
            1,
            0.7,
            0.0,
        );

        let rendered = redistributed(result.as_slice(), &hidden);
        assert!(
            (rendered[1] - 0.7).abs() < 1e-3,
            "expected the visible boundary to follow the cursor to 0.7, got {} (raw {:?})",
            rendered[1],
            result.as_slice(),
        );
    }

    #[test]
    fn drag_does_not_resize_hidden_neighbor() {
        // Three equal columns; the middle column (1) is hidden. The only divider the user can
        // grab sits between visible columns 0 and 2 (original index 0). Dragging it must resize
        // the next *visible* column (2) and leave the hidden column's width untouched.
        let resize_behavior = TableRow::from_vec(vec![TableResizeBehavior::Resizable; 3], 3);
        let widths = TableRow::from_vec(vec![1.0 / 3.0; 3], 3);
        let hidden = [false, true, false];
        let hidden_mask = TableRow::from_vec(hidden.to_vec(), 3);

        let result = RedistributableColumnsState::compute_drag_preview(
            widths,
            &resize_behavior,
            Some(&hidden_mask),
            0,
            0.7,
            0.0,
        );

        let result = result.as_slice();
        assert!(
            (result[1] - 1.0 / 3.0).abs() < 1e-6,
            "hidden column width must be preserved, but it changed: {result:?}",
        );
        // The drag moved width from visible column 2 to visible column 0, so the total is
        // unchanged and the next *visible* column absorbed the resize.
        let total: f32 = result.iter().sum();
        assert!(
            (total - 1.0).abs() < 1e-6,
            "total must be preserved: {result:?}"
        );
        assert!(
            result[0] > 1.0 / 3.0 && result[2] < 1.0 / 3.0,
            "expected the resize to be absorbed by the next visible column: {result:?}",
        );
    }
}

mod resizable_drag {
    use super::*;

    const REM: f32 = 16.;

    fn state(widths_px: &[f32], behavior: Vec<TableResizeBehavior>) -> ResizableColumnsState {
        let widths: Vec<AbsoluteLength> = widths_px
            .iter()
            .map(|w| AbsoluteLength::Pixels(px(*w)))
            .collect();
        ResizableColumnsState::new(widths.len(), widths, behavior)
    }

    fn widths_px(state: &ResizableColumnsState) -> Vec<f32> {
        state
            .widths
            .as_slice()
            .iter()
            .map(|w| f32::from(w.to_pixels(px(REM))))
            .collect()
    }

    #[test]
    fn drag_first_column_right() {
        let mut s = state(&[100., 100., 100.], vec![TableResizeBehavior::None; 3]);
        s.drag_to(0, px(150.), px(REM));
        assert_eq!(widths_px(&s), vec![150., 100., 100.]);
    }

    #[test]
    fn drag_middle_column_right() {
        let mut s = state(&[100., 100., 100.], vec![TableResizeBehavior::None; 3]);
        s.drag_to(1, px(250.), px(REM));
        assert_eq!(widths_px(&s), vec![100., 150., 100.]);
    }

    #[test]
    fn drag_does_not_affect_other_columns() {
        let mut s = state(&[100., 100., 100.], vec![TableResizeBehavior::None; 3]);
        s.drag_to(1, px(280.), px(REM));
        let w = widths_px(&s);
        assert_eq!(w[0], 100.);
        assert_eq!(w[2], 100.);
    }

    #[test]
    fn drag_below_min_clamps_to_min_size() {
        // MinSize(2.0) with rem=16 → min_px = 32
        let mut s = state(
            &[100., 100.],
            vec![TableResizeBehavior::MinSize(2.0), TableResizeBehavior::None],
        );
        s.drag_to(0, px(5.), px(REM));
        assert_eq!(widths_px(&s), vec![32., 100.]);
    }

    #[test]
    fn drag_x_below_left_edge_clamps_via_min() {
        // drag_x < left_edge would yield negative width; min clamping must catch it.
        let mut s = state(
            &[100., 100.],
            vec![TableResizeBehavior::MinSize(1.0), TableResizeBehavior::None],
        );
        s.drag_to(0, px(-50.), px(REM));
        assert_eq!(widths_px(&s), vec![16., 100.]);
    }

    #[test]
    fn pinned_and_scrollable_width_split() {
        let s = state(&[100., 100., 100.], vec![TableResizeBehavior::None; 3]);
        assert_eq!(f32::from(s.pinned_width(2, px(REM))), 200.);
        assert_eq!(f32::from(s.scrollable_width(2, px(REM))), 100.);
    }
}

mod pin_layout {
    use super::super::is_pinned_layout;

    #[test]
    fn zero_pinned_falls_back_to_single_section() {
        assert!(!is_pinned_layout(0, 5));
    }

    #[test]
    fn all_pinned_falls_back_to_single_section() {
        assert!(!is_pinned_layout(5, 5));
    }

    #[test]
    fn more_than_total_falls_back_to_single_section() {
        assert!(!is_pinned_layout(6, 5));
    }

    #[test]
    fn partial_pinning_uses_split_layout() {
        assert!(is_pinned_layout(1, 5));
        assert!(is_pinned_layout(2, 5));
        assert!(is_pinned_layout(4, 5));
    }
}

mod column_filter {
    use super::super::column_is_visible;
    use super::*;
    use crate::{redistribute_hidden_fractions, redistribute_hidden_widths};
    use gpui::{DefiniteLength, Length};

    fn frac_row(values: &[f32]) -> TableRow<f32> {
        TableRow::from_vec(values.to_vec(), values.len())
    }

    fn hidden_row(values: &[bool]) -> TableRow<bool> {
        TableRow::from_vec(values.to_vec(), values.len())
    }

    fn width_row(values: &[f32]) -> TableRow<Length> {
        TableRow::from_vec(
            values
                .iter()
                .map(|fraction| Length::Definite(DefiniteLength::Fraction(*fraction)))
                .collect(),
            values.len(),
        )
    }

    fn width_fractions(widths: &TableRow<Length>) -> Vec<f32> {
        widths
            .as_slice()
            .iter()
            .map(|length| match length {
                Length::Definite(DefiniteLength::Fraction(fraction)) => *fraction,
                other => panic!("expected fraction, got {other:?}"),
            })
            .collect()
    }

    #[test]
    fn column_is_visible_respects_mask() {
        let filter = Some(hidden_row(&[false, true, false]));
        assert!(column_is_visible(&filter, 0));
        assert!(!column_is_visible(&filter, 1));
        assert!(column_is_visible(&filter, 2));
        // Indices outside the mask default to visible.
        assert!(column_is_visible(&filter, 5));
    }

    #[test]
    fn column_is_visible_without_filter_is_always_visible() {
        let filter: Option<TableRow<bool>> = None;
        assert!(column_is_visible(&filter, 0));
        assert!(column_is_visible(&filter, 100));
    }

    #[test]
    fn redistribute_widths_is_identity_without_hidden_columns() {
        let widths = width_row(&[0.25, 0.25, 0.25, 0.25]);
        assert_eq!(
            width_fractions(&redistribute_hidden_widths(&widths, None)),
            vec![0.25, 0.25, 0.25, 0.25]
        );

        let none_hidden = hidden_row(&[false, false, false, false]);
        assert_eq!(
            width_fractions(&redistribute_hidden_widths(&widths, Some(&none_hidden))),
            vec![0.25, 0.25, 0.25, 0.25]
        );
    }

    #[test]
    fn redistribute_widths_scales_visible_columns_to_fill() {
        let widths = width_row(&[0.25, 0.25, 0.25, 0.25]);
        let hidden = hidden_row(&[false, true, false, false]);
        let result = width_fractions(&redistribute_hidden_widths(&widths, Some(&hidden)));

        // The hidden column keeps its stored width rather than being zeroed out (it is simply
        // not rendered), so its width is restored intact when it is shown again.
        assert_eq!(result[1], 0.25);
        // The visible columns expand to fill the container.
        let visible_sum: f32 = result[0] + result[2] + result[3];
        assert!(
            (visible_sum - 1.0).abs() < 1e-6,
            "visible sum was {visible_sum}"
        );
        // Equal initial fractions stay equal after redistribution.
        assert!((result[0] - result[2]).abs() < 1e-6);
        assert!((result[0] - result[3]).abs() < 1e-6);
    }

    #[test]
    fn redistribute_fractions_scales_visible_columns_to_fill() {
        let fractions = frac_row(&[0.25, 0.25, 0.25, 0.25]);
        let hidden = hidden_row(&[false, true, false, false]);
        let result = redistribute_hidden_fractions(&fractions, Some(&hidden));
        let result = result.as_slice();

        assert_eq!(result[1], 0.25);
        let visible_sum: f32 = result[0] + result[2] + result[3];
        assert!(
            (visible_sum - 1.0).abs() < 1e-6,
            "visible sum was {visible_sum}"
        );
    }

    #[test]
    fn redistribute_fractions_is_identity_without_hidden_columns() {
        let fractions = frac_row(&[0.2, 0.3, 0.5]);
        assert_eq!(
            redistribute_hidden_fractions(&fractions, None).as_slice(),
            &[0.2, 0.3, 0.5]
        );
    }
}
