use std::ops::Range;

use editor::{
    display_map::{DisplayRow, DisplaySnapshot},
    movement::{window_bottom, window_top, TextLayoutDetails},
    DisplayPoint, RowExt,
};
use text::Selection;

use crate::Direction;

pub fn manh_distance(point_1: &DisplayPoint, point_2: &DisplayPoint, x_bias: f32) -> f32 {
    x_bias * (point_1.row().as_f32() - point_2.row().as_f32()).abs()
        + (point_1.column() as i32 - point_2.column() as i32).abs() as f32
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
