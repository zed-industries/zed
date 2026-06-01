use std::cmp::{self, min};

use gpui::{Pixels, Point as GpuiPoint, px};

use crate::{Point, SelectionSide, TerminalBounds};

pub(crate) fn alt_scroll(scroll_lines: i32) -> Vec<u8> {
    let cmd = if scroll_lines > 0 { b'A' } else { b'B' };

    let mut content = Vec::with_capacity(scroll_lines.unsigned_abs() as usize * 3);
    for _ in 0..scroll_lines.abs() {
        content.push(0x1b);
        content.push(b'O');
        content.push(cmd);
    }
    content
}

pub(crate) fn grid_point(
    pos: GpuiPoint<Pixels>,
    cur_size: TerminalBounds,
    display_offset: usize,
) -> Point {
    grid_point_and_side(pos, cur_size, display_offset).0
}

pub(crate) fn grid_point_and_side(
    pos: GpuiPoint<Pixels>,
    cur_size: TerminalBounds,
    display_offset: usize,
) -> (Point, SelectionSide) {
    let mut column = (pos.x / cur_size.cell_width) as usize;
    let cell_x = cmp::max(px(0.), pos.x) % cur_size.cell_width;
    let half_cell_width = cur_size.cell_width / 2.0;
    let mut side = if cell_x > half_cell_width {
        SelectionSide::Right
    } else {
        SelectionSide::Left
    };

    let last_column = cur_size.num_columns().saturating_sub(1);
    if column > last_column {
        column = last_column;
        side = SelectionSide::Right;
    }
    let column = min(column, last_column);
    let mut line = (pos.y / cur_size.line_height) as i32;
    let bottommost_line = i32::try_from(cur_size.num_lines().saturating_sub(1)).unwrap_or(i32::MAX);
    if line > bottommost_line {
        line = bottommost_line;
        side = SelectionSide::Right;
    } else if line < 0 {
        side = SelectionSide::Left;
    }

    let display_offset = i32::try_from(display_offset).unwrap_or(i32::MAX);
    (
        Point::new(line.saturating_sub(display_offset), column),
        side,
    )
}
