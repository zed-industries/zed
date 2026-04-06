use super::table_row::TableRow;
use crate::{RedistributableColumnsState, TableResizeBehavior};

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
