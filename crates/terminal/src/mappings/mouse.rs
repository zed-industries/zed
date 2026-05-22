use std::cmp::{self, min};
use std::iter::repeat;

/// Most of the code, and specifically the constants, in this are copied from Alacritty,
/// with modifications for our circumstances
use gpui::{Modifiers, MouseButton, Pixels, Point, ScrollWheelEvent, px};

use crate::{TerminalBounds, TerminalModes, TerminalPoint, TerminalSelectionSide};

enum MouseFormat {
    Sgr,
    Normal(bool),
}

impl MouseFormat {
    fn from_mode(mode: TerminalModes) -> Self {
        if mode.contains(TerminalModes::SGR_MOUSE) {
            MouseFormat::Sgr
        } else if mode.contains(TerminalModes::UTF8_MOUSE) {
            MouseFormat::Normal(true)
        } else {
            MouseFormat::Normal(false)
        }
    }
}

#[derive(Debug)]
enum MouseButtonCode {
    LeftButton = 0,
    MiddleButton = 1,
    RightButton = 2,
    LeftMove = 32,
    MiddleMove = 33,
    RightMove = 34,
    NoneMove = 35,
    ScrollUp = 64,
    ScrollDown = 65,
    Other = 99,
}

impl MouseButtonCode {
    fn from_move_button(e: Option<MouseButton>) -> Self {
        match e {
            Some(gpui::MouseButton::Left) => MouseButtonCode::LeftMove,
            Some(gpui::MouseButton::Middle) => MouseButtonCode::MiddleMove,
            Some(gpui::MouseButton::Right) => MouseButtonCode::RightMove,
            Some(gpui::MouseButton::Navigate(_)) => MouseButtonCode::Other,
            None => MouseButtonCode::NoneMove,
        }
    }

    fn from_button(e: MouseButton) -> Self {
        match e {
            gpui::MouseButton::Left => MouseButtonCode::LeftButton,
            gpui::MouseButton::Right => MouseButtonCode::MiddleButton,
            gpui::MouseButton::Middle => MouseButtonCode::RightButton,
            gpui::MouseButton::Navigate(_) => MouseButtonCode::Other,
        }
    }

    fn from_scroll(e: &ScrollWheelEvent) -> Self {
        let is_positive = match e.delta {
            gpui::ScrollDelta::Pixels(pixels) => pixels.y > px(0.),
            gpui::ScrollDelta::Lines(lines) => lines.y > 0.,
        };

        if is_positive {
            MouseButtonCode::ScrollUp
        } else {
            MouseButtonCode::ScrollDown
        }
    }

    fn is_other(&self) -> bool {
        matches!(self, MouseButtonCode::Other)
    }
}

pub(crate) fn scroll_report(
    point: TerminalPoint,
    scroll_lines: i32,
    e: &ScrollWheelEvent,
    mode: TerminalModes,
) -> Option<impl Iterator<Item = Vec<u8>>> {
    if mode.intersects(TerminalModes::MOUSE_MODE) {
        mouse_report(
            point,
            MouseButtonCode::from_scroll(e),
            true,
            e.modifiers,
            MouseFormat::from_mode(mode),
        )
        .map(|report| repeat(report).take(scroll_lines.unsigned_abs() as usize))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{ScrollDelta, TouchPhase, point};

    #[test]
    fn scroll_report_repeats_for_negative_scroll_lines() {
        let grid_point = TerminalPoint::new(0, 0);

        let scroll_event = ScrollWheelEvent {
            delta: ScrollDelta::Lines(point(0., -1.)),
            touch_phase: TouchPhase::Moved,
            ..Default::default()
        };

        let mode = TerminalModes::MOUSE_MODE;
        let reports: Vec<Vec<u8>> = scroll_report(grid_point, -3, &scroll_event, mode)
            .expect("mouse mode should produce a scroll report")
            .collect();

        assert_eq!(reports.len(), 3);
    }

    #[test]
    fn scroll_report_repeats_for_positive_scroll_lines() {
        let grid_point = TerminalPoint::new(0, 0);

        let scroll_event = ScrollWheelEvent {
            delta: ScrollDelta::Lines(point(0., 1.)),
            touch_phase: TouchPhase::Moved,
            ..Default::default()
        };

        let mode = TerminalModes::MOUSE_MODE;
        let reports: Vec<Vec<u8>> = scroll_report(grid_point, 3, &scroll_event, mode)
            .expect("mouse mode should produce a scroll report")
            .collect();

        assert_eq!(reports.len(), 3);
    }
}

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

pub(crate) fn mouse_button_report(
    point: TerminalPoint,
    button: gpui::MouseButton,
    modifiers: Modifiers,
    pressed: bool,
    mode: TerminalModes,
) -> Option<Vec<u8>> {
    let button = MouseButtonCode::from_button(button);
    if !button.is_other() && mode.intersects(TerminalModes::MOUSE_MODE) {
        mouse_report(
            point,
            button,
            pressed,
            modifiers,
            MouseFormat::from_mode(mode),
        )
    } else {
        None
    }
}

pub(crate) fn mouse_moved_report(
    point: TerminalPoint,
    button: Option<MouseButton>,
    modifiers: Modifiers,
    mode: TerminalModes,
) -> Option<Vec<u8>> {
    let button = MouseButtonCode::from_move_button(button);

    if !button.is_other()
        && mode.intersects(TerminalModes::MOUSE_MOTION | TerminalModes::MOUSE_DRAG)
    {
        //Only drags are reported in drag mode, so block NoneMove.
        if mode.contains(TerminalModes::MOUSE_DRAG) && matches!(button, MouseButtonCode::NoneMove) {
            None
        } else {
            mouse_report(point, button, true, modifiers, MouseFormat::from_mode(mode))
        }
    } else {
        None
    }
}

pub(crate) fn grid_point(
    pos: Point<Pixels>,
    cur_size: TerminalBounds,
    display_offset: usize,
) -> TerminalPoint {
    grid_point_and_side(pos, cur_size, display_offset).0
}

pub(crate) fn grid_point_and_side(
    pos: Point<Pixels>,
    cur_size: TerminalBounds,
    display_offset: usize,
) -> (TerminalPoint, TerminalSelectionSide) {
    let mut column = (pos.x / cur_size.cell_width) as usize;
    let cell_x = cmp::max(px(0.), pos.x) % cur_size.cell_width;
    let half_cell_width = cur_size.cell_width / 2.0;
    let mut side = if cell_x > half_cell_width {
        TerminalSelectionSide::Right
    } else {
        TerminalSelectionSide::Left
    };

    let last_column = cur_size.num_columns().saturating_sub(1);
    if column > last_column {
        column = last_column;
        side = TerminalSelectionSide::Right;
    }
    let column = min(column, last_column);
    let mut line = (pos.y / cur_size.line_height) as i32;
    let bottommost_line = i32::try_from(cur_size.num_lines().saturating_sub(1)).unwrap_or(i32::MAX);
    if line > bottommost_line {
        line = bottommost_line;
        side = TerminalSelectionSide::Right;
    } else if line < 0 {
        side = TerminalSelectionSide::Left;
    }

    let display_offset = i32::try_from(display_offset).unwrap_or(i32::MAX);
    (
        TerminalPoint::new(line.saturating_sub(display_offset), column),
        side,
    )
}

///Generate the bytes to send to the terminal, from the cell location, a mouse event, and the terminal mode
fn mouse_report(
    point: TerminalPoint,
    button: MouseButtonCode,
    pressed: bool,
    modifiers: Modifiers,
    format: MouseFormat,
) -> Option<Vec<u8>> {
    if point.line < 0 {
        return None;
    }

    let mut mods = 0;
    if modifiers.shift {
        mods += 4;
    }
    if modifiers.alt {
        mods += 8;
    }
    if modifiers.control {
        mods += 16;
    }

    match format {
        MouseFormat::Sgr => {
            Some(sgr_mouse_report(point, button as u8 + mods, pressed).into_bytes())
        }
        MouseFormat::Normal(utf8) => {
            if pressed {
                normal_mouse_report(point, button as u8 + mods, utf8)
            } else {
                normal_mouse_report(point, 3 + mods, utf8)
            }
        }
    }
}

fn normal_mouse_report(point: TerminalPoint, button: u8, utf8: bool) -> Option<Vec<u8>> {
    let max_point = if utf8 { 2015 } else { 223 };

    if point.line >= max_point || point.column >= max_point as usize {
        return None;
    }

    let mut msg = vec![b'\x1b', b'[', b'M', 32 + button];

    let mouse_pos_encode = |pos: usize| -> Vec<u8> {
        let pos = 32 + 1 + pos;
        let first = 0xC0 + pos / 64;
        let second = 0x80 + (pos & 63);
        vec![first as u8, second as u8]
    };

    if utf8 && point.column >= 95 {
        msg.append(&mut mouse_pos_encode(point.column));
    } else {
        msg.push(32 + 1 + point.column as u8);
    }

    if utf8 && point.line >= 95 {
        msg.append(&mut mouse_pos_encode(point.line as usize));
    } else {
        msg.push(32 + 1 + point.line as u8);
    }

    Some(msg)
}

fn sgr_mouse_report(point: TerminalPoint, button: u8, pressed: bool) -> String {
    let c = if pressed { 'M' } else { 'm' };

    let msg = format!(
        "\x1b[<{};{};{}{}",
        button,
        point.column + 1,
        point.line + 1,
        c
    );

    msg
}
