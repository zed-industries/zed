use std::ops::Deref;

use pathfinder_geometry::vector::vec2f;

use crate::{geometry::vector::Vector2F, keymap_matcher::Keystroke};

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct ModifiersChangedEvent {
    pub modifiers: Modifiers,
}

impl Deref for ModifiersChangedEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// The phase of a touch motion event.
/// Based on the winit enum of the same name,
#[derive(Clone, Copy, Debug)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
}

#[derive(Clone, Copy, Debug)]
pub enum ScrollDelta {
    Pixels(Vector2F),
    Lines(Vector2F),
}

impl Default for ScrollDelta {
    fn default() -> Self {
        Self::Lines(Default::default())
    }
}

impl ScrollDelta {
    pub fn raw(&self) -> &Vector2F {
        match self {
            ScrollDelta::Pixels(v) => v,
            ScrollDelta::Lines(v) => v,
        }
    }

    pub fn precise(&self) -> bool {
        match self {
            ScrollDelta::Pixels(_) => true,
            ScrollDelta::Lines(_) => false,
        }
    }

    pub fn pixel_delta(&self, line_height: f32) -> Vector2F {
        match self {
            ScrollDelta::Pixels(delta) => *delta,
            ScrollDelta::Lines(delta) => vec2f(delta.x() * line_height, delta.y() * line_height),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ScrollWheelEvent {
    pub position: Vector2F,
    pub delta: ScrollDelta,
    pub modifiers: Modifiers,
    /// If the platform supports returning the phase of a scroll wheel event, it will be stored here
    pub phase: Option<TouchPhase>,
}

impl Deref for ScrollWheelEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
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

#[derive(Clone, Copy, Debug, Default)]
pub struct MouseButtonEvent {
    pub button: MouseButton,
    pub position: Vector2F,
    pub modifiers: Modifiers,
    pub click_count: usize,
}

impl Deref for MouseButtonEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MouseMovedEvent {
    pub position: Vector2F,
    pub pressed_button: Option<MouseButton>,
    pub modifiers: Modifiers,
}

impl Deref for MouseMovedEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
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

#[derive(Clone, Copy, Debug, Default)]
pub struct MouseExitedEvent {
    pub position: Vector2F,
    pub pressed_button: Option<MouseButton>,
    pub modifiers: Modifiers,
}

impl Deref for MouseExitedEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
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
    MouseExited(MouseExitedEvent),
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
            Event::MouseExited(event) => Some(event.position),
            Event::ScrollWheel(event) => Some(event.position),
        }
    }
}
