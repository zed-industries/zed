use std::ops::Range;

use editor::{
    display_map::{DisplayRow, DisplaySnapshot},
    movement::{window_bottom as _window_bottom, window_top as _window_top, TextLayoutDetails},
    DisplayPoint, RowExt,
};
use text::{Bias, Selection};

use crate::Direction;

pub fn manh_distance(point_1: &DisplayPoint, point_2: &DisplayPoint, x_bias: f32) -> f32 {
    x_bias * (point_1.row().as_f32() - point_2.row().as_f32()).abs()
        + (point_1.column() as i32 - point_2.column() as i32).abs() as f32
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
    _window_top(map, DisplayPoint::zero(), text_layout_details, 1).0
}

pub fn window_bottom(
    map: &DisplaySnapshot,
    text_layout_details: &TextLayoutDetails,
) -> DisplayPoint {
    _window_bottom(
        map,
        DisplayPoint::new(DisplayRow(0), u32::max_value()),
        text_layout_details,
        0,
    )
    .0
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
