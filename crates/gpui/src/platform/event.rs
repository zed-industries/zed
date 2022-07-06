use crate::{geometry::vector::Vector2F, keymap::Keystroke};

#[derive(Clone, Debug)]
pub struct KeyDownEvent {
    pub keystroke: Keystroke,
    pub input: Option<String>,
    pub is_held: bool,
}

#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    pub keystroke: Keystroke,
    pub input: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ModifiersChangedEvent {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
}

#[derive(Clone, Debug)]
pub struct ScrollWheelEvent {
    pub position: Vector2F,
    pub delta: Vector2F,
    pub precise: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum NavigationDirection {
    Back,
    Forward,
}

#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Navigate(NavigationDirection),
}

#[derive(Clone, Debug)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub position: Vector2F,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub click_count: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct MouseMovedEvent {
    pub position: Vector2F,
    pub pressed_button: Option<MouseButton>,
    pub ctrl: bool,
    pub cmd: bool,
    pub alt: bool,
    pub shift: bool,
}

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    ModifiersChanged(ModifiersChangedEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMoved(MouseMovedEvent),
    ScrollWheel(ScrollWheelEvent),
}

impl Event {
    pub fn position(&self) -> Option<Vector2F> {
        match self {
            Event::KeyDown { .. } => None,
            Event::KeyUp { .. } => None,
            Event::ModifiersChanged { .. } => None,
            Event::MouseDown(event) | Event::MouseUp(event) => Some(event.position),
            Event::MouseMoved(event) => Some(event.position),
            Event::ScrollWheel(event) => Some(event.position),
        }
    }
}
