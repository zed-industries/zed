//! Platform input event vocabulary shared by `gpui` and its platform backends.

use crate::ExternalPaths;
use gpui_types::{
    Capslock, KeyDownEvent, KeyUpEvent, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent,
    Pixels, Point, point,
};
use std::{any::Any, ops::Deref};

/// A direct touch drag claimed by an element before touch input becomes a tap,
/// long press, or scrolling gesture.
#[derive(Clone, Debug)]
pub struct TouchDragEvent {
    /// The phase of the touch drag.
    pub phase: TouchPhase,
    /// The position where the touch started.
    pub start_position: Point<Pixels>,
    /// The touch's current position.
    pub position: Point<Pixels>,
}

/// A phased long-press gesture recognized from a touch.
#[derive(Clone, Debug)]
pub struct LongPressEvent {
    /// The phase of the long press.
    pub phase: TouchPhase,
    /// The position where the touch started.
    pub start_position: Point<Pixels>,
    /// The touch's current position.
    pub position: Point<Pixels>,
}

impl Default for LongPressEvent {
    fn default() -> Self {
        Self {
            phase: TouchPhase::Started,
            start_position: Point::default(),
            position: Point::default(),
        }
    }
}

/// The modifiers changed event equivalent for the platform.
#[derive(Clone, Debug, Default)]
pub struct ModifiersChangedEvent {
    /// The new state of the modifier keys
    pub modifiers: Modifiers,
    /// The new state of the capslock key
    pub capslock: Capslock,
}

impl Deref for ModifiersChangedEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// The phase of a touch motion event.
/// Based on the winit enum of the same name.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TouchPhase {
    /// The touch started.
    Started,
    /// The touch event is moving.
    #[default]
    Moved,
    /// The touch phase has ended
    Ended,
    /// The touch was cancelled: the system took it and it will not end
    /// normally. Consumers must fully unwind any in-progress interaction,
    /// treating the touch as if it never committed.
    Cancelled,
}

/// Identifies one touch (finger or stylus contact) for its lifetime, from
/// [`TouchPhase::Started`] through [`TouchPhase::Ended`] or
/// [`TouchPhase::Cancelled`].
///
/// The value is opaque and assigned by the platform. A platform window must
/// not reuse an identifier for a later touch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TouchId(pub u64);

/// A raw touch event from the platform.
///
///
/// Dispatch contract (core implementation pending): a touch is hit-tested
/// once, at [`TouchPhase::Started`], occlusion-aware; all subsequent events
/// for the same [`TouchId`] are delivered to the elements under the starting
/// position, even after the touch moves outside them.
#[derive(Clone, Debug, Default)]
pub struct TouchEvent {
    /// Which touch this event belongs to.
    pub id: TouchId,
    /// The phase of the touch.
    pub phase: TouchPhase,
    /// The position of the touch in window coordinates.
    pub position: Point<Pixels>,
    /// Where the platform predicts the touch will be roughly one frame from
    /// now, in the same coordinate space as `position`, when the platform
    /// offers a prediction for a [`TouchPhase::Moved`] event.
    ///
    /// Best-effort latency compensation only: it may influence how far a
    /// recognized pan scrolls within a frame, but never hit testing, gesture
    /// classification, or velocity estimation, and any error it introduces
    /// must be corrected by later events for the same touch.
    pub predicted_position: Option<Point<Pixels>>,
    /// Normalized touch force in `0.0..=1.0`, if the hardware reports it.
    pub force: Option<f32>,
}

/// A mouse up event from the platform
#[derive(Clone, Debug, Default)]
pub struct MouseUpEvent {
    /// Which mouse button was released.
    pub button: MouseButton,

    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The modifiers that were held down when the mouse was released.
    pub modifiers: Modifiers,

    /// The number of times the button has been clicked.
    pub click_count: usize,
}

impl MouseUpEvent {
    /// Returns true if this mouse up event should focus the element.
    pub fn is_focusing(&self) -> bool {
        matches!(self.button, MouseButton::Left)
    }
}

/// The stage of a pressure click event.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PressureStage {
    /// No pressure.
    #[default]
    Zero,
    /// Normal click pressure.
    Normal,
    /// High pressure, enough to trigger a force click.
    Force,
}

/// A mouse pressure event from the platform. Generated when a force-sensitive trackpad is pressed hard.
/// Currently only implemented for macOS trackpads.
#[derive(Debug, Clone, Default)]
pub struct MousePressureEvent {
    /// Pressure of the current stage as a float between 0 and 1
    pub pressure: f32,
    /// The pressure stage of the event.
    pub stage: PressureStage,
    /// The position of the mouse on the window.
    pub position: Point<Pixels>,
    /// The modifiers that were held down when the mouse pressure changed.
    pub modifiers: Modifiers,
}

/// A mouse wheel event from the platform.
#[derive(Clone, Debug, Default)]
pub struct ScrollWheelEvent {
    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The change in scroll wheel position for this event.
    pub delta: ScrollDelta,

    /// The modifiers that were held down when the mouse was moved.
    pub modifiers: Modifiers,

    /// The phase of the touch event.
    pub touch_phase: TouchPhase,
}

impl Deref for ScrollWheelEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// The scroll delta for a scroll wheel event.
#[derive(Clone, Copy, Debug)]
pub enum ScrollDelta {
    /// An exact scroll delta in pixels.
    Pixels(Point<Pixels>),
    /// An inexact scroll delta in lines.
    Lines(Point<f32>),
}

impl Default for ScrollDelta {
    fn default() -> Self {
        Self::Lines(Default::default())
    }
}

impl ScrollDelta {
    /// Returns true if this is a precise scroll delta in pixels.
    pub fn precise(&self) -> bool {
        match self {
            ScrollDelta::Pixels(_) => true,
            ScrollDelta::Lines(_) => false,
        }
    }

    /// Converts this scroll event into exact pixels.
    pub fn pixel_delta(&self, line_height: Pixels) -> Point<Pixels> {
        match self {
            ScrollDelta::Pixels(delta) => *delta,
            ScrollDelta::Lines(delta) => point(line_height * delta.x, line_height * delta.y),
        }
    }

    /// Combines two scroll deltas into one.
    /// If the signs of the deltas are the same (both positive or both negative),
    /// the deltas are added together. If the signs are opposite, the second delta
    /// (other) is used, effectively overriding the first delta.
    pub fn coalesce(self, other: ScrollDelta) -> ScrollDelta {
        match (self, other) {
            (ScrollDelta::Pixels(a), ScrollDelta::Pixels(b)) => {
                let x = if a.x.signum() == b.x.signum() {
                    a.x + b.x
                } else {
                    b.x
                };

                let y = if a.y.signum() == b.y.signum() {
                    a.y + b.y
                } else {
                    b.y
                };

                ScrollDelta::Pixels(point(x, y))
            }

            (ScrollDelta::Lines(a), ScrollDelta::Lines(b)) => {
                let x = if a.x.signum() == b.x.signum() {
                    a.x + b.x
                } else {
                    b.x
                };

                let y = if a.y.signum() == b.y.signum() {
                    a.y + b.y
                } else {
                    b.y
                };

                ScrollDelta::Lines(point(x, y))
            }

            _ => other,
        }
    }
}

/// A pinch gesture event from the platform, generated when the user performs
/// a pinch-to-zoom gesture (typically on a trackpad).
#[derive(Clone, Debug, Default)]
pub struct PinchEvent {
    /// The position of the pinch center on the window.
    pub position: Point<Pixels>,

    /// The zoom delta for this event.
    /// Positive values indicate zooming in, negative values indicate zooming out.
    /// For example, 0.1 represents a 10% zoom increase.
    pub delta: f32,

    /// The modifiers that were held down during the pinch gesture.
    pub modifiers: Modifiers,

    /// The phase of the pinch gesture.
    pub phase: TouchPhase,
}

impl Deref for PinchEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// A mouse exit event from the platform, generated when the mouse leaves the window.
#[derive(Clone, Debug, Default)]
pub struct MouseExitEvent {
    /// The position of the mouse relative to the window.
    pub position: Point<Pixels>,
    /// The mouse button that was pressed, if any.
    pub pressed_button: Option<MouseButton>,
    /// The modifiers that were held down when the mouse was moved.
    pub modifiers: Modifiers,
}

impl Deref for MouseExitEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// A file drop event from the platform, generated when files are dragged and dropped onto the window.
#[derive(Debug, Clone)]
pub enum FileDropEvent {
    /// The files have entered the window.
    Entered {
        /// The position of the mouse relative to the window.
        position: Point<Pixels>,
        /// The paths of the files that are being dragged.
        paths: ExternalPaths,
    },
    /// The files are being dragged over the window
    Pending {
        /// The position of the mouse relative to the window.
        position: Point<Pixels>,
    },
    /// The files have been dropped onto the window.
    Submit {
        /// The position of the mouse relative to the window.
        position: Point<Pixels>,
    },
    /// The user has stopped dragging the files over the window.
    Exited,
    /// The platform-owned drag session has ended.
    Ended,
}

/// An enum corresponding to all kinds of platform input events.
#[derive(Clone, Debug)]
pub enum PlatformInput {
    /// A key was pressed.
    KeyDown(KeyDownEvent),
    /// A key was released.
    KeyUp(KeyUpEvent),
    /// The keyboard modifiers were changed.
    ModifiersChanged(ModifiersChangedEvent),
    /// The mouse was pressed.
    MouseDown(MouseDownEvent),
    /// The mouse was released.
    MouseUp(MouseUpEvent),
    /// Mouse pressure.
    MousePressure(MousePressureEvent),
    /// The mouse was moved.
    MouseMove(MouseMoveEvent),
    /// The mouse exited the window.
    MouseExited(MouseExitEvent),
    /// The scroll wheel was used.
    ScrollWheel(ScrollWheelEvent),
    /// A pinch gesture was performed.
    Pinch(PinchEvent),
    /// A long-press gesture recognized from touch input.
    LongPress(LongPressEvent),
    /// A direct touch drag claimed by an element.
    TouchDrag(TouchDragEvent),
    /// Files were dragged and dropped onto the window.
    FileDrop(FileDropEvent),
    /// A raw touch event on a touch screen.
    Touch(TouchEvent),
}

impl PlatformInput {
    /// Returns this input as a mouse event, if it is one.
    pub fn mouse_event(&self) -> Option<&dyn Any> {
        match self {
            PlatformInput::KeyDown { .. } => None,
            PlatformInput::KeyUp { .. } => None,
            PlatformInput::ModifiersChanged { .. } => None,
            PlatformInput::MouseDown(event) => Some(event),
            PlatformInput::MouseUp(event) => Some(event),
            PlatformInput::MouseMove(event) => Some(event),
            PlatformInput::MousePressure(event) => Some(event),
            PlatformInput::MouseExited(event) => Some(event),
            PlatformInput::ScrollWheel(event) => Some(event),
            PlatformInput::Pinch(event) => Some(event),
            PlatformInput::LongPress(event) => Some(event),
            PlatformInput::TouchDrag(event) => Some(event),
            PlatformInput::FileDrop(event) => Some(event),
            PlatformInput::Touch(_) => None,
        }
    }

    /// Returns this input as a keyboard event, if it is one.
    pub fn keyboard_event(&self) -> Option<&dyn Any> {
        match self {
            PlatformInput::KeyDown(event) => Some(event),
            PlatformInput::KeyUp(event) => Some(event),
            PlatformInput::ModifiersChanged(event) => Some(event),
            PlatformInput::MouseDown(_) => None,
            PlatformInput::MouseUp(_) => None,
            PlatformInput::MouseMove(_) => None,
            PlatformInput::MousePressure(_) => None,
            PlatformInput::MouseExited(_) => None,
            PlatformInput::ScrollWheel(_) => None,
            PlatformInput::Pinch(_) => None,
            PlatformInput::LongPress(_) => None,
            PlatformInput::TouchDrag(_) => None,
            PlatformInput::FileDrop(_) => None,
            PlatformInput::Touch(_) => None,
        }
    }

    /// A short static name for this input's variant, for diagnostics and
    /// telemetry.
    pub fn kind_name(&self) -> &'static str {
        match self {
            PlatformInput::KeyDown(_) => "key_down",
            PlatformInput::KeyUp(_) => "key_up",
            PlatformInput::ModifiersChanged(_) => "modifiers_changed",
            PlatformInput::MouseDown(_) => "mouse_down",
            PlatformInput::MouseUp(_) => "mouse_up",
            PlatformInput::MousePressure(_) => "mouse_pressure",
            PlatformInput::MouseMove(_) => "mouse_move",
            PlatformInput::MouseExited(_) => "mouse_exited",
            PlatformInput::ScrollWheel(_) => "scroll_wheel",
            PlatformInput::Pinch(_) => "pinch",
            PlatformInput::LongPress(_) => "long_press",
            PlatformInput::TouchDrag(_) => "touch_drag",
            PlatformInput::FileDrop(_) => "file_drop",
            PlatformInput::Touch(_) => "touch",
        }
    }

    /// Returns the touch event contained in this input, if any.
    pub fn touch_event(&self) -> Option<&TouchEvent> {
        match self {
            PlatformInput::Touch(event) => Some(event),
            _ => None,
        }
    }
}
