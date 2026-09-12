//! Input event value types shared between GPUI and its platform backends.
//!
//! These are the passive event payloads that platform backends produce and
//! that GPUI consumes. The `InputEvent` and `Sealed` implementations that
//! convert them into `gpui::PlatformInput` stay in the `gpui` crate.

use crate::{Keystroke, Modifiers, Pixels, Point};

/// An enum representing the mouse button that was pressed.
#[derive(Hash, Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum MouseButton {
    /// The left mouse button.
    #[default]
    Left,

    /// The right mouse button.
    Right,

    /// The middle mouse button.
    Middle,

    /// A navigation button, such as back or forward.
    Navigate(NavigationDirection),
}

impl MouseButton {
    /// Get all the mouse buttons in a list.
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

/// A navigation direction, such as back or forward.
#[derive(Hash, Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum NavigationDirection {
    /// The back button.
    #[default]
    Back,

    /// The forward button.
    Forward,
}

/// The key down event equivalent for the platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyDownEvent {
    /// The keystroke that was generated.
    pub keystroke: Keystroke,

    /// Whether the key is currently held down.
    pub is_held: bool,

    /// Whether to prefer character input over keybindings for this keystroke.
    /// In some cases, like AltGr on Windows, modifiers are significant for character input.
    pub prefer_character_input: bool,
}

/// The key up event equivalent for the platform.
#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    /// The keystroke that was released.
    pub keystroke: Keystroke,
}

/// A mouse down event from the platform
#[derive(Clone, Debug, Default)]
pub struct MouseDownEvent {
    /// Which mouse button was pressed.
    pub button: MouseButton,

    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The modifiers that were held down when the mouse was pressed.
    pub modifiers: Modifiers,

    /// The number of times the button has been clicked.
    pub click_count: usize,

    /// Whether this is the first, focusing click.
    pub first_mouse: bool,
}

impl MouseDownEvent {
    /// Returns true if this mouse up event should focus the element.
    pub fn is_focusing(&self) -> bool {
        match self.button {
            MouseButton::Left => true,
            _ => false,
        }
    }
}

/// A mouse move event from the platform.
#[derive(Clone, Debug, Default)]
pub struct MouseMoveEvent {
    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The mouse button that was pressed, if any.
    pub pressed_button: Option<MouseButton>,

    /// The modifiers that were held down when the mouse was moved.
    pub modifiers: Modifiers,
}

impl MouseMoveEvent {
    /// Returns true if the left mouse button is currently held down.
    pub fn dragging(&self) -> bool {
        self.pressed_button == Some(MouseButton::Left)
    }
}
