use std::cmp::{max, min};
use std::iter::repeat;

use alacritty_terminal::grid::Dimensions;
/// Most of the code, and specifically the constants, in this are copied from Alacritty,
/// with modifications for our circumstances
use alacritty_terminal::index::{Column as GridCol, Line as GridLine, Point, Side};
use alacritty_terminal::term::TermMode;
use gpui2::platform;
use gpui2::scene::MouseScrollWheel;
use gpui2::{
    geometry::vector::Vector2F,
    platform::{MouseButtonEvent, MouseMovedEvent, ScrollWheelEvent},
};

use crate::TerminalSize;

struct Modifiers {
    ctrl: bool,
    shift: bool,
    alt: bool,
}

impl Modifiers {
    fn from_moved(e: &MouseMovedEvent) -> Self {
        Modifiers {
            ctrl: e.ctrl,
            shift: e.shift,
            alt: e.alt,
        }
    }

    fn from_button(e: &MouseButtonEvent) -> Self {
        Modifiers {
            ctrl: e.ctrl,
            shift: e.shift,
            alt: e.alt,
        }
    }

    fn from_scroll(scroll: &ScrollWheelEvent) -> Self {
        Modifiers {
            ctrl: scroll.ctrl,
            shift: scroll.shift,
            alt: scroll.alt,
        }
    }
}

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
enum MouseButton {
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

impl MouseButton {
    fn from_move(e: &MouseMovedEvent) -> Self {
        match e.pressed_button {
            Some(b) => match b {
                platform::MouseButton::Left => MouseButton::LeftMove,
                platform::MouseButton::Middle => MouseButton::MiddleMove,
                platform::MouseButton::Right => MouseButton::RightMove,
                platform::MouseButton::Navigate(_) => MouseButton::Other,
            },
            None => MouseButton::NoneMove,
        }
    }

    fn from_button(e: &MouseButtonEvent) -> Self {
        match e.button {
            platform::MouseButton::Left => MouseButton::LeftButton,
            platform::MouseButton::Right => MouseButton::MiddleButton,
            platform::MouseButton::Middle => MouseButton::RightButton,
            platform::MouseButton::Navigate(_) => MouseButton::Other,
        }
    }

    fn from_scroll(e: &ScrollWheelEvent) -> Self {
        if e.delta.raw().y() > 0. {
            MouseButton::ScrollUp
        } else {
            MouseButton::ScrollDown
        }
    }

    fn is_other(&self) -> bool {
        match self {
            MouseButton::Other => true,
            _ => false,
        }
    }
}

pub fn scroll_report(
    point: Point,
    scroll_lines: i32,
    e: &MouseScrollWheel,
    mode: TermMode,
) -> Option<impl Iterator<Item = Vec<u8>>> {
    if mode.intersects(TermMode::MOUSE_MODE) {
        mouse_report(
            point,
            MouseButton::from_scroll(e),
            true,
            Modifiers::from_scroll(e),
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
    point: Point,
    e: &MouseButtonEvent,
    pressed: bool,
    mode: TermMode,
) -> Option<Vec<u8>> {
    let button = MouseButton::from_button(e);
    if !button.is_other() && mode.intersects(TermMode::MOUSE_MODE) {
        mouse_report(
            point,
            button,
            pressed,
            Modifiers::from_button(e),
            MouseFormat::from_mode(mode),
        )
    } else {
        None
    }
}

pub fn mouse_moved_report(point: Point, e: &MouseMovedEvent, mode: TermMode) -> Option<Vec<u8>> {
    let button = MouseButton::from_move(e);

    if !button.is_other() && mode.intersects(TermMode::MOUSE_MOTION | TermMode::MOUSE_DRAG) {
        //Only drags are reported in drag mode, so block NoneMove.
        if mode.contains(TermMode::MOUSE_DRAG) && matches!(button, MouseButton::NoneMove) {
            None
        } else {
            mouse_report(
                point,
                button,
                true,
                Modifiers::from_moved(e),
                MouseFormat::from_mode(mode),
            )
        }
    } else {
        None
    }
}

pub fn mouse_side(pos: Vector2F, cur_size: TerminalSize) -> alacritty_terminal::index::Direction {
    if cur_size.cell_width as usize == 0 {
        return Side::Right;
    }
    let x = pos.0.x() as usize;
    let cell_x = x.saturating_sub(cur_size.cell_width as usize) % cur_size.cell_width as usize;
    let half_cell_width = (cur_size.cell_width / 2.0) as usize;
    let additional_padding = (cur_size.width() - cur_size.cell_width * 2.) % cur_size.cell_width;
    let end_of_grid = cur_size.width() - cur_size.cell_width - additional_padding;
    //Width: Pixels or columns?
    if cell_x > half_cell_width
    // Edge case when mouse leaves the window.
    || x as f32 >= end_of_grid
    {
        Side::Right
    } else {
        Side::Left
    }
}

pub fn grid_point(pos: Vector2F, cur_size: TerminalSize, display_offset: usize) -> Point {
    let col = pos.x() / cur_size.cell_width;
    let col = min(GridCol(col as usize), cur_size.last_column());
    let line = pos.y() / cur_size.line_height;
    let line = min(line as i32, cur_size.bottommost_line().0);
    Point::new(GridLine(line - display_offset as i32), col)
}

///Generate the bytes to send to the terminal, from the cell location, a mouse event, and the terminal mode
fn mouse_report(
    point: Point,
    button: MouseButton,
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
    if modifiers.ctrl {
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

fn normal_mouse_report(point: Point, button: u8, utf8: bool) -> Option<Vec<u8>> {
    let Point { line, column } = point;
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

fn sgr_mouse_report(point: Point, button: u8, pressed: bool) -> String {
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

#[cfg(test)]
mod test {
    use crate::mappings::mouse::grid_point;

    #[test]
    fn test_mouse_to_selection() {
        let term_width = 100.;
        let term_height = 200.;
        let cell_width = 10.;
        let line_height = 20.;
        let mouse_pos_x = 100.; //Window relative
        let mouse_pos_y = 100.; //Window relative
        let origin_x = 10.;
        let origin_y = 20.;

        let cur_size = crate::TerminalSize::new(
            line_height,
            cell_width,
            gpui::geometry::vector::vec2f(term_width, term_height),
        );

        let mouse_pos = gpui::geometry::vector::vec2f(mouse_pos_x, mouse_pos_y);
        let origin = gpui::geometry::vector::vec2f(origin_x, origin_y); //Position of terminal window, 1 'cell' in
        let mouse_pos = mouse_pos - origin;
        let point = grid_point(mouse_pos, cur_size, 0);
        assert_eq!(
            point,
            alacritty_terminal::index::Point::new(
                alacritty_terminal::index::Line(((mouse_pos_y - origin_y) / line_height) as i32),
                alacritty_terminal::index::Column(((mouse_pos_x - origin_x) / cell_width) as usize),
            )
        );
    }
}
