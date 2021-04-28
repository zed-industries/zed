use crate::{geometry::vector::Vector2F, keymap::Keystroke};

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown {
        keystroke: Keystroke,
        chars: String,
    },
    ScrollWheel {
        position: Vector2F,
        delta: Vector2F,
        precise: bool,
    },
    LeftMouseDown {
        position: Vector2F,
        cmd: bool,
    },
    LeftMouseUp {
        position: Vector2F,
    },
    LeftMouseDragged {
        position: Vector2F,
    },
    MouseMoved {
        position: Vector2F,
    },
}
