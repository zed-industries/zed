//! Conversion from UIKit events to GPUI input events.

use gpui::{Pixels, Point, TouchId, TouchPhase, px};
use objc2_ui_kit::{UITouch, UITouchPhase, UIView};

pub fn touch_location_in_view(touch: &UITouch, view: &UIView) -> Point<Pixels> {
    let location = touch.locationInView(Some(view));
    Point::new(px(location.x as f32), px(location.y as f32))
}

pub fn touch_phase(touch: &UITouch) -> TouchPhase {
    match touch.phase() {
        UITouchPhase::Began => TouchPhase::Started,
        UITouchPhase::Moved | UITouchPhase::Stationary => TouchPhase::Moved,
        UITouchPhase::Ended => TouchPhase::Ended,
        _ => TouchPhase::Cancelled,
    }
}

pub fn touch_id(touch: &UITouch) -> TouchId {
    TouchId(touch as *const UITouch as usize as u64)
}

pub fn touch_force(touch: &UITouch) -> Option<f32> {
    let maximum_force = touch.maximumPossibleForce();
    if maximum_force <= 0.0 {
        return None;
    }
    Some((touch.force() / maximum_force).clamp(0.0, 1.0) as f32)
}
