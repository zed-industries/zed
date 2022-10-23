use derive_more::Deref;

use crate::{geometry::vector::Vector2F, keymap::Keystroke};

#[derive(Clone, Debug)]
pub struct KeyDownEvent {
    pub keystroke: Keystroke,
    pub is_held: bool,
}

#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    pub keystroke: Keystroke,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub fun: bool,
}

#[derive(Clone, Copy, Debug, Default, Deref)]
pub struct ModifiersChangedEvent {
    #[deref]
    pub modifiers: Modifiers,
}

/// The phase of a touch motion event.
/// Based on the winit enum of the same name,
#[derive(Clone, Copy, Debug)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
}

#[derive(Clone, Copy, Debug, Default, Deref)]
pub struct ScrollWheelEvent {
    pub position: Vector2F,
    pub delta: Vector2F,
    pub precise: bool,
    #[deref]
    pub modifiers: Modifiers,
    /// If the platform supports returning the phase of a scroll wheel event, it will be stored here
    pub phase: Option<TouchPhase>,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum NavigationDirection {
    Back,
    Forward,
}

impl Default for NavigationDirection {
    fn default() -> Self {
        Self::Back
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Navigate(NavigationDirection),
}

impl MouseButton {
    pub fn all() -> Vec<Self> {
        vec![
            MouseButton::Left,
            MouseButton::Right,
            MouseButton::Middle,
            MouseButton::Navigate(NavigationDirection::Back),
            MouseButton::Navigate(NavigationDirection::Forward),
        ]
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Clone, Copy, Debug, Default, Deref)]
pub struct MouseButtonEvent {
    pub button: MouseButton,
    pub position: Vector2F,
    #[deref]
    pub modifiers: Modifiers,
    pub click_count: usize,
}

#[derive(Clone, Copy, Debug, Default, Deref)]
pub struct MouseMovedEvent {
    pub position: Vector2F,
    pub pressed_button: Option<MouseButton>,
    #[deref]
    pub modifiers: Modifiers,
}

impl MouseMovedEvent {
    pub fn to_button_event(&self, button: MouseButton) -> MouseButtonEvent {
        MouseButtonEvent {
            position: self.position,
            button: self.pressed_button.unwrap_or(button),
            modifiers: self.modifiers,
            click_count: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    ModifiersChanged(ModifiersChangedEvent),
    MouseDown(MouseButtonEvent),
    MouseUp(MouseButtonEvent),
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
