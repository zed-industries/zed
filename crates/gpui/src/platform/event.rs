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
    MouseMoved {
        position: Vector2F,
        left_mouse_down: bool,
    },
}
