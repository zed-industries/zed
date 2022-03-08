use crate::{geometry::vector::Vector2F, keymap::Keystroke};

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown {
        keystroke: Keystroke,
        input: Option<String>,
        is_held: bool,
    },
    ScrollWheel {
        position: Vector2F,
        delta: Vector2F,
        precise: bool,
    },
    LeftMouseDown {
        position: Vector2F,
        ctrl: bool,
        alt: bool,
        shift: bool,
        cmd: bool,
        click_count: usize,
    },
    LeftMouseUp {
        position: Vector2F,
    },
    LeftMouseDragged {
        position: Vector2F,
    },
    RightMouseDown {
        position: Vector2F,
        ctrl: bool,
        alt: bool,
        shift: bool,
        cmd: bool,
        click_count: usize,
    },
    RightMouseUp {
        position: Vector2F,
    },
    OtherMouseDown {
        position: Vector2F,
        button: u16,
        ctrl: bool,
        alt: bool,
        shift: bool,
        cmd: bool,
        click_count: usize,
    },
    OtherMouseUp {
        position: Vector2F,
        button: u16,
    },
    MouseMoved {
        position: Vector2F,
        left_mouse_down: bool,
    },
}
