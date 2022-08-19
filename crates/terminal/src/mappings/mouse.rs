/// Most of the code, and specifically the constants, in this are copied from Alacritty,
/// with modifications for our circumstances
use alacritty_terminal::{index::Point, term::TermMode};
use gpui::{MouseButtonEvent, MouseMovedEvent, ScrollWheelEvent};

pub struct Modifiers {
    ctrl: bool,
    shift: bool,
    alt: bool,
}

impl Modifiers {
    pub fn from_moved(e: &MouseMovedEvent) -> Self {
        Modifiers {
            ctrl: e.ctrl,
            shift: e.shift,
            alt: e.alt,
        }
    }

    pub fn from_button(e: &MouseButtonEvent) -> Self {
        Modifiers {
            ctrl: e.ctrl,
            shift: e.shift,
            alt: e.alt,
        }
    }

    //TODO: Determine if I should add modifiers into the ScrollWheelEvent type
    pub fn from_scroll() -> Self {
        Modifiers {
            ctrl: false,
            shift: false,
            alt: false,
        }
    }
}

pub enum MouseFormat {
    SGR,
    Normal(bool),
}

impl MouseFormat {
    pub fn from_mode(mode: TermMode) -> Self {
        if mode.contains(TermMode::SGR_MOUSE) {
            MouseFormat::SGR
        } else if mode.contains(TermMode::UTF8_MOUSE) {
            MouseFormat::Normal(true)
        } else {
            MouseFormat::Normal(false)
        }
    }
}

pub enum MouseButton {
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
    pub fn from_move(e: &MouseMovedEvent) -> Self {
        match e.pressed_button {
            Some(b) => match b {
                gpui::MouseButton::Left => MouseButton::LeftMove,
                gpui::MouseButton::Middle => MouseButton::MiddleMove,
                gpui::MouseButton::Right => MouseButton::RightMove,
                gpui::MouseButton::Navigate(_) => MouseButton::Other,
            },
            None => MouseButton::NoneMove,
        }
    }

    pub fn from_button(e: &MouseButtonEvent) -> Self {
        match e.button {
            gpui::MouseButton::Left => MouseButton::LeftButton,
            gpui::MouseButton::Right => MouseButton::MiddleButton,
            gpui::MouseButton::Middle => MouseButton::RightButton,
            gpui::MouseButton::Navigate(_) => MouseButton::Other,
        }
    }

    pub fn from_scroll(e: &ScrollWheelEvent) -> Self {
        if e.delta.y() > 0. {
            MouseButton::ScrollUp
        } else {
            MouseButton::ScrollDown
        }
    }

    pub fn is_other(&self) -> bool {
        match self {
            MouseButton::Other => true,
            _ => false,
        }
    }
}

pub fn scroll_report(
    point: Point,
    scroll_lines: i32,
    e: &ScrollWheelEvent,
    mode: TermMode,
) -> Option<Vec<Vec<u8>>> {
    if mode.intersects(TermMode::MOUSE_MODE) && scroll_lines >= 1 {
        if let Some(report) = mouse_report(
            point,
            MouseButton::from_scroll(e),
            true,
            Modifiers::from_scroll(),
            MouseFormat::from_mode(mode),
        ) {
            let mut res = vec![];
            for _ in 0..scroll_lines.abs() {
                res.push(report.clone());
            }
            return Some(res);
        }
    }

    None
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
        mouse_report(
            point,
            button,
            true,
            Modifiers::from_moved(e),
            MouseFormat::from_mode(mode),
        )
    } else {
        None
    }
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
