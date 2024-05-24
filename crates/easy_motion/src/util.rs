use editor::{
    display_map::{DisplayRow, DisplaySnapshot, ToDisplayPoint},
    movement::{find_boundary_range, TextLayoutDetails},
    DisplayPoint, RowExt,
};
use language::{char_kind, CharKind};
use vim::utils::coerce_punctuation;

pub(crate) fn manh_distance(point_1: &DisplayPoint, point_2: &DisplayPoint, x_bias: f32) -> f32 {
    x_bias * (point_1.row().as_f32() - point_2.row().as_f32()).abs()
        + (point_1.column() as i32 - point_2.column() as i32).abs() as f32
}

pub(crate) fn window_top(
    map: &DisplaySnapshot,
    text_layout_details: &TextLayoutDetails,
) -> DisplayPoint {
    let mut point = text_layout_details
        .scroll_anchor
        .anchor
        .to_display_point(map);
    *point.column_mut() = 0;
    map.clip_point(point, text::Bias::Left)
}

pub(crate) fn window_bottom(
    map: &DisplaySnapshot,
    text_layout_details: &TextLayoutDetails,
) -> DisplayPoint {
    let Some(visible_rows) = text_layout_details.visible_rows else {
        return DisplayPoint::default();
    };

    let point = text_layout_details
        .scroll_anchor
        .anchor
        .to_display_point(map);
    let new_row =
        point.row().0 + (visible_rows + text_layout_details.scroll_anchor.offset.y).floor() as u32;
    let new_col = point.column().min(map.line_len(point.row()));
    map.clip_point(
        DisplayPoint::new(DisplayRow(new_row), new_col),
        text::Bias::Left,
    )
}

pub(crate) fn word_starts_in_range(
    map: &DisplaySnapshot,
    mut from: DisplayPoint,
    to: DisplayPoint,
    ignore_punctuation: bool,
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
        let mut crossed_newline = false;
        let new_point = find_boundary_range(map, from, to, |left, right| {
            let left_kind = coerce_punctuation(char_kind(&scope, left), ignore_punctuation);
            let right_kind = coerce_punctuation(char_kind(&scope, right), ignore_punctuation);
            let at_newline = right == '\n';
            let found = (left_kind != right_kind && right_kind != CharKind::Whitespace);

            crossed_newline |= at_newline;
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

        let marked_text = "ˇ lorem ipsuˇm hi hello ";
        let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
        let point = display_points.first().unwrap().clone();
        let end = display_points.last().unwrap().clone();
        let starts = word_starts_in_range(&snapshot, point, end, true);
        assert_eq!(starts, vec![display_point(1, 0), display_point(7, 0)]);

        let marked_text = "ˇ lorem ipsum hi helloˇ";
        let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
        let point = display_points.first().unwrap().clone();
        let end = display_points.last().unwrap().clone();
        let starts = word_starts_in_range(&snapshot, point, end, true);
        assert_eq!(
            starts,
            vec![
                display_point(1, 0),
                display_point(7, 0),
                display_point(13, 0),
                display_point(16, 0)
            ]
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
