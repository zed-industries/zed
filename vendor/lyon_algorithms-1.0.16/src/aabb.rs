//! Bounding rectangle computation for paths.

use crate::geom::{CubicBezierSegment, QuadraticBezierSegment};
use crate::math::{point, Box2D, Point};
use crate::path::PathEvent;

/// Computes a conservative axis-aligned rectangle that contains the path.
///
/// This bounding rectangle approximation is faster but less precise than
/// [`bounding_box`](fn.bounding_box.html).
pub fn fast_bounding_box<Iter, Evt>(path: Iter) -> Box2D
where
    Iter: IntoIterator<Item = Evt>,
    Evt: FastBoundingBox,
{
    let mut min = point(f32::MAX, f32::MAX);
    let mut max = point(f32::MIN, f32::MIN);
    for e in path {
        e.min_max(&mut min, &mut max);
    }

    // Return an empty rectangle by default if there was no event in the path.
    if min == point(f32::MAX, f32::MAX) {
        return Box2D::zero();
    }

    Box2D { min, max }
}

#[doc(hidden)]
pub trait FastBoundingBox {
    fn min_max(&self, min: &mut Point, max: &mut Point);
}

impl FastBoundingBox for PathEvent {
    fn min_max(&self, min: &mut Point, max: &mut Point) {
        match self {
            PathEvent::Begin { at } => {
                *min = Point::min(*min, *at);
                *max = Point::max(*max, *at);
            }
            PathEvent::Line { to, .. } => {
                *min = Point::min(*min, *to);
                *max = Point::max(*max, *to);
            }
            PathEvent::Quadratic { ctrl, to, .. } => {
                *min = Point::min(*min, Point::min(*ctrl, *to));
                *max = Point::max(*max, Point::max(*ctrl, *to));
            }
            PathEvent::Cubic {
                ctrl1, ctrl2, to, ..
            } => {
                *min = Point::min(*min, Point::min(*ctrl1, Point::min(*ctrl2, *to)));
                *max = Point::max(*max, Point::max(*ctrl1, Point::max(*ctrl2, *to)));
            }
            PathEvent::End { .. } => {}
        }
    }
}

/// Computes the smallest axis-aligned rectangle that contains the path.
pub fn bounding_box<Iter, Evt>(path: Iter) -> Box2D
where
    Iter: IntoIterator<Item = Evt>,
    Evt: TightBoundingBox,
{
    let mut min = point(f32::MAX, f32::MAX);
    let mut max = point(f32::MIN, f32::MIN);

    for evt in path {
        evt.min_max(&mut min, &mut max);
    }

    // Return an empty rectangle by default if there was no event in the path.
    if min == point(f32::MAX, f32::MAX) {
        return Box2D::zero();
    }

    Box2D { min, max }
}

#[doc(hidden)]
pub trait TightBoundingBox {
    fn min_max(&self, min: &mut Point, max: &mut Point);
}

impl TightBoundingBox for PathEvent {
    fn min_max(&self, min: &mut Point, max: &mut Point) {
        match self {
            PathEvent::Begin { at } => {
                *min = Point::min(*min, *at);
                *max = Point::max(*max, *at);
            }
            PathEvent::Line { to, .. } => {
                *min = Point::min(*min, *to);
                *max = Point::max(*max, *to);
            }
            PathEvent::Quadratic { from, ctrl, to } => {
                let r = QuadraticBezierSegment {
                    from: *from,
                    ctrl: *ctrl,
                    to: *to,
                }
                .bounding_box();
                *min = Point::min(*min, r.min);
                *max = Point::max(*max, r.max);
            }
            PathEvent::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => {
                let r = CubicBezierSegment {
                    from: *from,
                    ctrl1: *ctrl1,
                    ctrl2: *ctrl2,
                    to: *to,
                }
                .bounding_box();
                *min = Point::min(*min, r.min);
                *max = Point::max(*max, r.max);
            }
            PathEvent::End { .. } => {}
        }
    }
}

#[test]
fn simple_bounding_box() {
    use crate::path::Path;

    let mut builder = Path::builder();
    builder.begin(point(-10.0, -3.0));
    builder.line_to(point(0.0, -12.0));
    builder.quadratic_bezier_to(point(3.0, 4.0), point(5.0, 3.0));
    builder.end(true);
    let path = builder.build();

    assert_eq!(
        fast_bounding_box(&path),
        Box2D {
            min: point(-10.0, -12.0),
            max: point(5.0, 4.0)
        },
    );

    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.cubic_bezier_to(point(-1.0, 2.0), point(3.0, -4.0), point(1.0, -1.0));
    builder.end(false);
    let path = builder.build();

    assert_eq!(
        fast_bounding_box(path.iter()),
        Box2D {
            min: point(-1.0, -4.0),
            max: point(3.0, 2.0)
        },
    );
}
