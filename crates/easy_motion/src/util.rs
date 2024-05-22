use editor::{
    display_map::DisplaySnapshot,
    movement::{find_boundary, FindRange},
    DisplayPoint, RowExt,
};
use language::{char_kind, CharKind};
use vim::utils::coerce_punctuation;

pub(crate) fn manh_distance(point_1: &DisplayPoint, point_2: &DisplayPoint, x_bias: f32) -> f32 {
    x_bias * (point_1.row().as_f32() - point_2.row().as_f32()).abs()
        + (point_1.column() as i32 - point_2.column() as i32).abs() as f32
}

pub(crate) fn word_starts(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    max_times: usize,
) -> Vec<DisplayPoint> {
    let scope = map.buffer_snapshot.language_scope_at(point.to_point(map));
    let mut result = Vec::new();
    for _ in 0..max_times {
        let mut crossed_newline = false;
        let new_point = find_boundary(map, point, FindRange::MultiLine, |left, right| {
            let left_kind = coerce_punctuation(char_kind(&scope, left), ignore_punctuation);
            let right_kind = coerce_punctuation(char_kind(&scope, right), ignore_punctuation);
            let at_newline = right == '\n';

            let found = (left_kind != right_kind && right_kind != CharKind::Whitespace)
                || at_newline && crossed_newline
                || at_newline && left == '\n'; // Prevents skipping repeated empty lines

            crossed_newline |= at_newline;
            found
        });
        if point == new_point {
            break;
        }
        result.push(new_point);
        point = new_point;
    }
    result
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

    #[gpui::test]
    fn test_get_word_starts(cx: &mut AppContext) {
        init_test(cx);
        let marked_text = "ˇ lorem ipsum ";
        let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
        let point = display_points.first().unwrap().clone();
        let word_starts = word_starts(&snapshot, point, true, 4);
        assert_eq!(word_starts, vec![display_point(1, 0), display_point(7, 0)]);
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
