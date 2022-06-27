use crate::{geometry::vector::Vector2F, keymap::Keystroke};

#[derive(Copy, Clone, Debug)]
pub enum NavigationDirection {
    Back,
    Forward,
}

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown {
        keystroke: Keystroke,
        input: Option<String>,
        is_held: bool,
    },
    KeyUp {
        keystroke: Keystroke,
        input: Option<String>,
    },
    ModifiersChanged {
        ctrl: bool,
        alt: bool,
        shift: bool,
        cmd: bool,
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
        click_count: usize,
    },
    LeftMouseDragged {
        position: Vector2F,
        ctrl: bool,
        alt: bool,
        shift: bool,
        cmd: bool,
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
        click_count: usize,
    },
    NavigateMouseDown {
        position: Vector2F,
        direction: NavigationDirection,
        ctrl: bool,
        alt: bool,
        shift: bool,
        cmd: bool,
        click_count: usize,
    },
    NavigateMouseUp {
        position: Vector2F,
        direction: NavigationDirection,
    },
    MouseMoved {
        position: Vector2F,
        left_mouse_down: bool,
        ctrl: bool,
        cmd: bool,
        alt: bool,
        shift: bool,
    },
}

impl Event {
    pub fn position(&self) -> Option<Vector2F> {
        match self {
            Event::KeyDown { .. } => None,
            Event::KeyUp { .. } => None,
            Event::ModifiersChanged { .. } => None,
            Event::ScrollWheel { position, .. }
            | Event::LeftMouseDown { position, .. }
            | Event::LeftMouseUp { position, .. }
            | Event::LeftMouseDragged { position, .. }
            | Event::RightMouseDown { position, .. }
            | Event::RightMouseUp { position, .. }
            | Event::NavigateMouseDown { position, .. }
            | Event::NavigateMouseUp { position, .. }
            | Event::MouseMoved { position, .. } => Some(*position),
        }
    }
}
