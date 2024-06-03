use std::{cmp::Ordering, ops::Range};

use editor::{
    display_map::{DisplayRow, DisplaySnapshot, ToDisplayPoint},
    movement::TextLayoutDetails,
    DisplayPoint, RowExt,
};
use gpui::{EntityId, Point};
use text::{Bias, Selection};
use ui::Pixels;

use crate::Direction;

pub fn manh_distance(point_1: &DisplayPoint, point_2: &DisplayPoint, x_bias: f32) -> f32 {
    x_bias * (point_1.row().as_f32() - point_2.row().as_f32()).abs()
        + (point_1.column() as i32 - point_2.column() as i32).abs() as f32
}

pub fn manh_distance_pixels(point_1: &Point<Pixels>, point_2: &Point<Pixels>, x_bias: f32) -> f32 {
    x_bias * (point_1.x.0 - point_2.x.0).abs() + (point_1.y.0 - point_2.y.0).abs()
}

pub fn end_of_document(map: &DisplaySnapshot) -> DisplayPoint {
    let new_point = DisplayPoint::new(DisplayRow(u32::max_value()), u32::max_value());
    map.clip_point(new_point, Bias::Left)
}

pub fn start_of_document(map: &DisplaySnapshot) -> DisplayPoint {
    let new_point = DisplayPoint::zero();
    map.clip_point(new_point, Bias::Left)
}

pub fn window_top(map: &DisplaySnapshot, text_layout_details: &TextLayoutDetails) -> DisplayPoint {
    let mut point = text_layout_details
        .scroll_anchor
        .anchor
        .to_display_point(map);
    *point.column_mut() = 0;
    map.clip_point(point, text::Bias::Left)
}

pub fn window_bottom(
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

// returns a display point range from the current selection to the start/end
// for a direction of backwards/forwards respectively or the full window for
// bidirectional
pub fn ranges(
    direction: Direction,
    map: &DisplaySnapshot,
    selections: &Selection<DisplayPoint>,
    text_layout_details: &TextLayoutDetails,
) -> Range<DisplayPoint> {
    let start = match direction {
        Direction::BiDirectional | Direction::Backwards => window_top(map, &text_layout_details),
        Direction::Forwards => selections.end,
    };
    let end = match direction {
        Direction::BiDirectional | Direction::Forwards => window_bottom(map, &text_layout_details),
        Direction::Backwards => selections.start,
    };
    start..end
}

pub fn sort_matches_pixel(
    matches: &mut Vec<(DisplayPoint, EntityId, Point<Pixels>)>,
    cursor: &Point<Pixels>,
) {
    matches.sort_unstable_by(|a, b| {
        let a_distance = manh_distance_pixels(&a.2, &cursor, 2.5);
        let b_distance = manh_distance_pixels(&b.2, &cursor, 2.5);
        if a_distance == b_distance {
            Ordering::Equal
        } else if a_distance < b_distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
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

    #[gpui::test]
    fn test_end_of_document(cx: &mut AppContext) {
        init_test(cx);
        let marked_text = "ˇlorem ipsum \n{}\n h";
        let (snapshot, _) = marked_display_snapshot(marked_text, cx);
        assert_eq!(end_of_document(&snapshot), display_point(2, 2));
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
