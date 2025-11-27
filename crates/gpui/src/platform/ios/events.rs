//! iOS event handling - converting UIKit events to GPUI's event types.
//!
//! iOS uses touch-based input rather than mouse input, so we need to map
//! touch gestures to appropriate GPUI events:
//! - Single tap → MouseDown + MouseUp (left button)
//! - Long press → MouseDown + MouseUp (right button) for context menus
//! - Pan gesture → ScrollWheel events
//! - Pinch gesture → Zoom events (if supported)
//! - Touch move → MouseMove events

use crate::{
    Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformInput,
    Point, ScrollDelta, ScrollWheelEvent, TouchPhase, px,
};
use core_graphics::geometry::CGPoint;
use objc::{msg_send, runtime::Object, sel, sel_impl};

/// Touch phase from UIKit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum UITouchPhase {
    Began = 0,
    Moved = 1,
    Stationary = 2,
    Ended = 3,
    Cancelled = 4,
}

impl From<i64> for UITouchPhase {
    fn from(value: i64) -> Self {
        match value {
            0 => UITouchPhase::Began,
            1 => UITouchPhase::Moved,
            2 => UITouchPhase::Stationary,
            3 => UITouchPhase::Ended,
            4 => UITouchPhase::Cancelled,
            _ => UITouchPhase::Cancelled,
        }
    }
}

impl From<UITouchPhase> for TouchPhase {
    fn from(phase: UITouchPhase) -> Self {
        match phase {
            UITouchPhase::Began => TouchPhase::Started,
            UITouchPhase::Moved => TouchPhase::Moved,
            UITouchPhase::Stationary => TouchPhase::Moved,
            UITouchPhase::Ended => TouchPhase::Ended,
            UITouchPhase::Cancelled => TouchPhase::Ended,
        }
    }
}

/// Convert a UITouch to a mouse position
pub fn touch_location_in_view(touch: *mut Object, view: *mut Object) -> Point<Pixels> {
    unsafe {
        let location: CGPoint = msg_send![touch, locationInView: view];
        Point::new(px(location.x as f32), px(location.y as f32))
    }
}

/// Get the touch phase from a UITouch
pub fn touch_phase(touch: *mut Object) -> UITouchPhase {
    unsafe {
        let phase: i64 = msg_send![touch, phase];
        UITouchPhase::from(phase)
    }
}

/// Get the number of taps for a touch (for detecting double-tap, etc.)
pub fn touch_tap_count(touch: *mut Object) -> u32 {
    unsafe {
        let count: i64 = msg_send![touch, tapCount];
        count as u32
    }
}

/// Convert a single touch began event to a mouse down event
pub fn touch_began_to_mouse_down(
    position: Point<Pixels>,
    tap_count: u32,
    modifiers: Modifiers,
) -> PlatformInput {
    PlatformInput::MouseDown(MouseDownEvent {
        button: MouseButton::Left,
        position,
        modifiers,
        click_count: tap_count as usize,
        first_mouse: false,
    })
}

/// Convert a touch ended event to a mouse up event
pub fn touch_ended_to_mouse_up(
    position: Point<Pixels>,
    tap_count: u32,
    modifiers: Modifiers,
) -> PlatformInput {
    PlatformInput::MouseUp(MouseUpEvent {
        button: MouseButton::Left,
        position,
        modifiers,
        click_count: tap_count as usize,
    })
}

/// Convert a touch moved event to a mouse move event
pub fn touch_moved_to_mouse_move(
    position: Point<Pixels>,
    modifiers: Modifiers,
    pressed_button: Option<MouseButton>,
) -> PlatformInput {
    PlatformInput::MouseMove(MouseMoveEvent {
        position,
        modifiers,
        pressed_button,
    })
}

/// Convert a pan gesture to a scroll wheel event
pub fn pan_gesture_to_scroll(
    position: Point<Pixels>,
    delta: Point<Pixels>,
    modifiers: Modifiers,
    touch_phase: TouchPhase,
) -> PlatformInput {
    PlatformInput::ScrollWheel(ScrollWheelEvent {
        position,
        delta: ScrollDelta::Pixels(delta),
        modifiers,
        touch_phase,
    })
}

/// Get current keyboard modifiers from UIKit.
/// iOS doesn't have the same modifier key concept as macOS,
/// but external keyboards can provide modifiers.
pub fn get_current_modifiers() -> Modifiers {
    // iOS 13.4+ supports modifier keys from external keyboards
    // For now, return empty modifiers - this can be enhanced later
    // to read from UIKeyModifierFlags when available
    Modifiers::default()
}

/// Check if a long press should trigger a context menu (right-click equivalent)
pub fn is_long_press_for_context_menu(touch: *mut Object) -> bool {
    unsafe {
        // Check the force of the touch (3D Touch / Haptic Touch)
        let force: f64 = msg_send![touch, force];
        let max_force: f64 = msg_send![touch, maximumPossibleForce];

        // If force touch is available and pressed hard enough
        if max_force > 0.0 && force / max_force > 0.5 {
            return true;
        }

        false
    }
}

/// Convert a force touch to a right-click for context menus
pub fn force_touch_to_right_click(position: Point<Pixels>, modifiers: Modifiers) -> PlatformInput {
    PlatformInput::MouseDown(MouseDownEvent {
        button: MouseButton::Right,
        position,
        modifiers,
        click_count: 1,
        first_mouse: false,
    })
}
