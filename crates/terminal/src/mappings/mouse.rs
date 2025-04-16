use std::cmp::{self, max, min};
use std::iter::repeat;

use alacritty_terminal::grid::Dimensions;
/// Most of the code, and specifically the constants, in this are copied from Alacritty,
/// with modifications for our circumstances
use alacritty_terminal::index::{Column as GridCol, Line as GridLine, Point as AlacPoint, Side};
use alacritty_terminal::term::TermMode;
use gpui::{Modifiers, MouseButton, Pixels, Point, ScrollWheelEvent, px};

use crate::TerminalBounds;

enum MouseFormat {
    Sgr,
    Normal(bool),
}

impl MouseFormat {
    fn from_mode(mode: TermMode) -> Self {
        if mode.contains(TermMode::SGR_MOUSE) {
            MouseFormat::Sgr
        } else if mode.contains(TermMode::UTF8_MOUSE) {
            MouseFormat::Normal(true)
        } else {
            MouseFormat::Normal(false)
        }
    }
}

#[derive(Debug)]
enum AlacMouseButton {
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

impl AlacMouseButton {
    fn from_move_button(e: Option<MouseButton>) -> Self {
        match e {
            Some(gpui::MouseButton::Left) => AlacMouseButton::LeftMove,
            Some(gpui::MouseButton::Middle) => AlacMouseButton::MiddleMove,
            Some(gpui::MouseButton::Right) => AlacMouseButton::RightMove,
            Some(gpui::MouseButton::Navigate(_)) => AlacMouseButton::Other,
            None => AlacMouseButton::NoneMove,
        }
    }

    fn from_button(e: MouseButton) -> Self {
        match e {
            gpui::MouseButton::Left => AlacMouseButton::LeftButton,
            gpui::MouseButton::Right => AlacMouseButton::MiddleButton,
            gpui::MouseButton::Middle => AlacMouseButton::RightButton,
            gpui::MouseButton::Navigate(_) => AlacMouseButton::Other,
        }
    }

    fn from_scroll(e: &ScrollWheelEvent) -> Self {
        let is_positive = match e.delta {
            gpui::ScrollDelta::Pixels(pixels) => pixels.y > px(0.),
            gpui::ScrollDelta::Lines(lines) => lines.y > 0.,
        };

        if is_positive {
            AlacMouseButton::ScrollUp
        } else {
            AlacMouseButton::ScrollDown
        }
    }

    fn is_other(&self) -> bool {
        matches!(self, AlacMouseButton::Other)
    }
}

pub fn scroll_report(
    point: AlacPoint,
    scroll_lines: i32,
    e: &ScrollWheelEvent,
    mode: TermMode,
) -> Option<impl Iterator<Item = Vec<u8>>> {
    if mode.intersects(TermMode::MOUSE_MODE) {
        mouse_report(
            point,
            AlacMouseButton::from_scroll(e),
            true,
            e.modifiers,
            MouseFormat::from_mode(mode),
        )
        .map(|report| repeat(report).take(max(scroll_lines, 1) as usize))
    } else {
        None
    }
}

pub fn alt_scroll(scroll_lines: i32) -> Vec<u8> {
    let cmd = if scroll_lines > 0 { b'A' } else { b'B' };

    let mut content = Vec::with_capacity(scroll_lines.unsigned_abs() as usize * 3);
    for _ in 0..scroll_lines.abs() {
        content.push(0x1b);
        content.push(b'O');
        content.push(cmd);
    }
    content
}

pub fn mouse_button_report(
    point: AlacPoint,
    button: gpui::MouseButton,
    modifiers: Modifiers,
    pressed: bool,
    mode: TermMode,
) -> Option<Vec<u8>> {
    let button = AlacMouseButton::from_button(button);
    if !button.is_other() && mode.intersects(TermMode::MOUSE_MODE) {
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

pub fn mouse_moved_report(
    point: AlacPoint,
    button: Option<MouseButton>,
    modifiers: Modifiers,
    mode: TermMode,
) -> Option<Vec<u8>> {
    let button = AlacMouseButton::from_move_button(button);

    if !button.is_other() && mode.intersects(TermMode::MOUSE_MOTION | TermMode::MOUSE_DRAG) {
        //Only drags are reported in drag mode, so block NoneMove.
        if mode.contains(TermMode::MOUSE_DRAG) && matches!(button, AlacMouseButton::NoneMove) {
            None
        } else {
            mouse_report(point, button, true, modifiers, MouseFormat::from_mode(mode))
        }
    } else {
        None
    }
}

pub fn grid_point(
    pos: Point<Pixels>,
    cur_size: TerminalBounds,
    display_offset: usize,
) -> AlacPoint {
    grid_point_and_side(pos, cur_size, display_offset).0
}

pub fn grid_point_and_side(
    pos: Point<Pixels>,
    cur_size: TerminalBounds,
    display_offset: usize,
) -> (AlacPoint, Side) {
    let mut col = GridCol((pos.x / cur_size.cell_width) as usize);
    let cell_x = cmp::max(px(0.), pos.x) % cur_size.cell_width;
    let half_cell_width = cur_size.cell_width / 2.0;
    let mut side = if cell_x > half_cell_width {
        Side::Right
    } else {
        Side::Left
    };

    if col > cur_size.last_column() {
        col = cur_size.last_column();
        side = Side::Right;
    }
    let col = min(col, cur_size.last_column());
    let mut line = (pos.y / cur_size.line_height) as i32;
    if line > cur_size.bottommost_line() {
        line = cur_size.bottommost_line().0;
        side = Side::Right;
    } else if line < 0 {
        side = Side::Left;
    }

    (
        AlacPoint::new(GridLine(line - display_offset as i32), col),
        side,
    )
}

///Generate the bytes to send to the terminal, from the cell location, a mouse event, and the terminal mode
fn mouse_report(
    point: AlacPoint,
    button: AlacMouseButton,
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

fn normal_mouse_report(point: AlacPoint, button: u8, utf8: bool) -> Option<Vec<u8>> {
    let AlacPoint { line, column } = point;
    let max_point = if utf8 { 2015 } else { 223 };

    if line >= max_point || column >= max_point {
        return None;
    }

    let mut msg = vec![b'\x1b', b'[', b'M', 32 + button];

    let mouse_pos_encode = |pos: usize| -> Vec<u8> {
        let pos = 32 + 1 + pos;
        let first = 0xC0 + pos / 64;
        let second = 0x80 + (pos & 63);
        vec![first as u8, second as u8]
    };

    if utf8 && column >= 95 {
        msg.append(&mut mouse_pos_encode(column.0));
    } else {
        msg.push(32 + 1 + column.0 as u8);
    }

    if utf8 && line >= 95 {
        msg.append(&mut mouse_pos_encode(line.0 as usize));
    } else {
        msg.push(32 + 1 + line.0 as u8);
    }

    Some(msg)
}

fn sgr_mouse_report(point: AlacPoint, button: u8, pressed: bool) -> String {
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
