use cocoa::foundation::{NSPoint, NSRect, NSSize};
use pathfinder_geometry::{rect::RectF, vector::Vector2F};
pub trait Vector2FExt {
    fn to_ns_point(&self) -> NSPoint;
    fn to_ns_size(&self) -> NSSize;
}

pub trait RectFExt {
    fn to_ns_rect(&self) -> NSRect;
}

impl Vector2FExt for Vector2F {
    fn to_ns_point(&self) -> NSPoint {
        NSPoint::new(self.x() as f64, self.y() as f64)
    }

    fn to_ns_size(&self) -> NSSize {
        NSSize::new(self.x() as f64, self.y() as f64)
    }
}

impl RectFExt for RectF {
    fn to_ns_rect(&self) -> NSRect {
        NSRect::new(self.origin().to_ns_point(), self.size().to_ns_size())
    }
}
