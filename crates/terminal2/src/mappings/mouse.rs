use std::cmp::{self, max, min};
use std::iter::repeat;

use alacritty_terminal::grid::Dimensions;
/// Most of the code, and specifically the constants, in this are copied from Alacritty,
/// with modifications for our circumstances
use alacritty_terminal::index::{Column as GridCol, Line as GridLine, Point as AlacPoint, Side};
use alacritty_terminal::term::TermMode;
use gpui2::{px, Modifiers, MouseButton, MouseMoveEvent, Pixels, Point, ScrollWheelEvent};

use crate::TerminalSize;

enum MouseFormat {
    SGR,
    Normal(bool),
}

impl MouseFormat {
    fn from_mode(mode: TermMode) -> Self {
        if mode.contains(TermMode::SGR_MOUSE) {
            MouseFormat::SGR
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
    fn from_move(e: &MouseMoveEvent) -> Self {
        match e.pressed_button {
            Some(b) => match b {
                gpui2::MouseButton::Left => AlacMouseButton::LeftMove,
                gpui2::MouseButton::Middle => AlacMouseButton::MiddleMove,
                gpui2::MouseButton::Right => AlacMouseButton::RightMove,
                gpui2::MouseButton::Navigate(_) => AlacMouseButton::Other,
            },
            None => AlacMouseButton::NoneMove,
        }
    }

    fn from_button(e: MouseButton) -> Self {
        match e {
            gpui2::MouseButton::Left => AlacMouseButton::LeftButton,
            gpui2::MouseButton::Right => AlacMouseButton::MiddleButton,
            gpui2::MouseButton::Middle => AlacMouseButton::RightButton,
            gpui2::MouseButton::Navigate(_) => AlacMouseButton::Other,
        }
    }

    fn from_scroll(e: &ScrollWheelEvent) -> Self {
        let is_positive = match e.delta {
            gpui2::ScrollDelta::Pixels(pixels) => pixels.y > px(0.),
            gpui2::ScrollDelta::Lines(lines) => lines.y > 0.,
        };

        if is_positive {
            AlacMouseButton::ScrollUp
        } else {
            AlacMouseButton::ScrollDown
        }
    }

    fn is_other(&self) -> bool {
        match self {
            AlacMouseButton::Other => true,
            _ => false,
        }
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

    let mut content = Vec::with_capacity(scroll_lines.abs() as usize * 3);
    for _ in 0..scroll_lines.abs() {
        content.push(0x1b);
        content.push(b'O');
        content.push(cmd);
    }
    content
}

pub fn mouse_button_report(
    point: AlacPoint,
    button: gpui2::MouseButton,
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

pub fn mouse_moved_report(point: AlacPoint, e: &MouseMoveEvent, mode: TermMode) -> Option<Vec<u8>> {
    let button = AlacMouseButton::from_move(e);

    if !button.is_other() && mode.intersects(TermMode::MOUSE_MOTION | TermMode::MOUSE_DRAG) {
        //Only drags are reported in drag mode, so block NoneMove.
        if mode.contains(TermMode::MOUSE_DRAG) && matches!(button, AlacMouseButton::NoneMove) {
            None
        } else {
            mouse_report(
                point,
                button,
                true,
                e.modifiers,
                MouseFormat::from_mode(mode),
            )
        }
    } else {
        None
    }
}

pub fn mouse_side(
    pos: Point<Pixels>,
    cur_size: TerminalSize,
) -> alacritty_terminal::index::Direction {
    let cell_width = cur_size.cell_width.floor();
    if cell_width == px(0.) {
        return Side::Right;
    }

    let x = pos.x.floor();

    let cell_x = cmp::max(px(0.), x - cell_width) % cell_width;
    let half_cell_width = (cur_size.cell_width / 2.0).floor();
    let additional_padding = (cur_size.width() - cur_size.cell_width * 2.) % cur_size.cell_width;
    let end_of_grid = cur_size.width() - cur_size.cell_width - additional_padding;

    //Width: Pixels or columns?
    if cell_x > half_cell_width
    // Edge case when mouse leaves the window.
    || x >= end_of_grid
    {
        Side::Right
    } else {
        Side::Left
    }
}

pub fn grid_point(pos: Point<Pixels>, cur_size: TerminalSize, display_offset: usize) -> AlacPoint {
    let col = GridCol((pos.x / cur_size.cell_width).as_usize());
    let col = min(col, cur_size.last_column());
    let line = (pos.y / cur_size.line_height).as_isize() as i32;
    let line = min(line, cur_size.bottommost_line().0);
    AlacPoint::new(GridLine(line - display_offset as i32), col)
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
        MouseFormat::SGR => {
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
