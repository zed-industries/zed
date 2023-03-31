use cocoa::{
    base::id,
    foundation::{NSPoint, NSRect},
};
use objc::{msg_send, sel, sel_impl};
use pathfinder_geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};

///! Macos screen have a y axis that goings up from the bottom of the screen and
///! an origin at the bottom left of the main display.

pub trait Vector2FExt {
    /// Converts self to an NSPoint with y axis pointing up.
    fn to_screen_ns_point(&self, native_window: id, window_height: f64) -> NSPoint;
}
impl Vector2FExt for Vector2F {
    fn to_screen_ns_point(&self, native_window: id, window_height: f64) -> NSPoint {
        unsafe {
            let point = NSPoint::new(self.x() as f64, window_height - self.y() as f64);
            msg_send![native_window, convertPointToScreen: point]
        }
    }
}

pub trait NSRectExt {
    fn to_rectf(&self) -> RectF;
    fn intersects(&self, other: Self) -> bool;
}

impl NSRectExt for NSRect {
    fn to_rectf(&self) -> RectF {
        RectF::new(
            vec2f(self.origin.x as f32, self.origin.y as f32),
            vec2f(self.size.width as f32, self.size.height as f32),
        )
    }

    fn intersects(&self, other: Self) -> bool {
        self.size.width > 0.
            && self.size.height > 0.
            && other.size.width > 0.
            && other.size.height > 0.
            && self.origin.x <= other.origin.x + other.size.width
            && self.origin.x + self.size.width >= other.origin.x
            && self.origin.y <= other.origin.y + other.size.height
            && self.origin.y + self.size.height >= other.origin.y
    }
}
