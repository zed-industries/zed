use crate::{geometry::vector::Vector2F, keymap::Keystroke};

#[derive(Copy, Clone, Debug)]
pub enum NavigationDirection {
    Back,
    Forward,
}

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

#[derive(Clone, Debug)]
pub struct LeftMouseDownEvent {
    pub position: Vector2F,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub click_count: usize,
}

#[derive(Clone, Debug)]
pub struct LeftMouseUpEvent {
    pub position: Vector2F,
    pub click_count: usize,
}

#[derive(Clone, Debug)]
pub struct LeftMouseDraggedEvent {
    pub position: Vector2F,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
}

#[derive(Clone, Debug)]
pub struct RightMouseDownEvent {
    pub position: Vector2F,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub click_count: usize,
}

#[derive(Clone, Debug)]
pub struct RightMouseUpEvent {
    pub position: Vector2F,
    pub click_count: usize,
}

#[derive(Clone, Debug)]
pub struct NavigateMouseDownEvent {
    pub position: Vector2F,
    pub direction: NavigationDirection,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub click_count: usize,
}

#[derive(Clone, Debug)]
pub struct NavigateMouseUpEvent {
    pub position: Vector2F,
    pub direction: NavigationDirection,
}

#[derive(Clone, Debug)]
pub struct MouseMovedEvent {
    pub position: Vector2F,
    pub left_mouse_down: bool,
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
    ScrollWheel(ScrollWheelEvent),
    LeftMouseDown(LeftMouseDownEvent),
    LeftMouseUp(LeftMouseUpEvent),
    LeftMouseDragged(LeftMouseDraggedEvent),
    RightMouseDown(RightMouseDownEvent),
    RightMouseUp(RightMouseUpEvent),
    NavigateMouseDown(NavigateMouseDownEvent),
    NavigateMouseUp(NavigateMouseUpEvent),
    MouseMoved(MouseMovedEvent),
}

impl Event {
    pub fn position(&self) -> Option<Vector2F> {
        match self {
            Event::KeyDown { .. } => None,
            Event::KeyUp { .. } => None,
            Event::ModifiersChanged { .. } => None,
            Event::ScrollWheel(ScrollWheelEvent { position, .. })
            | Event::LeftMouseDown(LeftMouseDownEvent { position, .. })
            | Event::LeftMouseUp(LeftMouseUpEvent { position, .. })
            | Event::LeftMouseDragged(LeftMouseDraggedEvent { position, .. })
            | Event::RightMouseDown(RightMouseDownEvent { position, .. })
            | Event::RightMouseUp(RightMouseUpEvent { position, .. })
            | Event::NavigateMouseDown(NavigateMouseDownEvent { position, .. })
            | Event::NavigateMouseUp(NavigateMouseUpEvent { position, .. })
            | Event::MouseMoved(MouseMovedEvent { position, .. }) => Some(*position),
        }
    }
}
