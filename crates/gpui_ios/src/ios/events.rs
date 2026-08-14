//! Conversion from UIKit events to GPUI input events.

use gpui::{Pixels, Point, TouchId, TouchPhase, px};
use objc2::msg_send;
use objc2::runtime::AnyObject;

use super::cg_types::ObjcCGPoint;

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
            UITouchPhase::Cancelled => TouchPhase::Cancelled,
        }
    }
}

/// Returns the touch position in window coordinates.
pub fn touch_location_in_view(touch: *mut AnyObject, view: *mut AnyObject) -> Point<Pixels> {
    unsafe {
        let location: ObjcCGPoint = msg_send![touch, locationInView: view];
        Point::new(px(location.x as f32), px(location.y as f32))
    }
}

/// Returns the current UIKit touch phase.
pub fn touch_phase(touch: *mut AnyObject) -> UITouchPhase {
    unsafe {
        let phase: i64 = msg_send![touch, phase];
        UITouchPhase::from(phase)
    }
}

/// Returns an identifier stable for the lifetime of this UIKit touch.
pub fn touch_id(touch: *mut AnyObject) -> TouchId {
    TouchId(touch as usize as u64)
}

/// Returns normalized pressure when UIKit reports a meaningful force range.
pub fn touch_force(touch: *mut AnyObject) -> Option<f32> {
    unsafe {
        let maximum_force: f64 = msg_send![touch, maximumPossibleForce];
        if maximum_force <= 0.0 {
            return None;
        }

        let force: f64 = msg_send![touch, force];
        Some((force / maximum_force).clamp(0.0, 1.0) as f32)
    }
}
