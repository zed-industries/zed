//! Approximate path length.

use crate::geom::{CubicBezierSegment, LineSegment, QuadraticBezierSegment};
use crate::path::PathEvent;

use core::iter::IntoIterator;

pub fn approximate_length<Iter>(path: Iter, tolerance: f32) -> f32
where
    Iter: IntoIterator<Item = PathEvent>,
{
    let tolerance = tolerance.max(1e-4);

    let mut length = 0.0;

    for evt in path.into_iter() {
        match evt {
            PathEvent::Line { from, to } => length += LineSegment { from, to }.length(),
            PathEvent::Quadratic { from, ctrl, to } => {
                length += QuadraticBezierSegment { from, ctrl, to }.length()
            }
            PathEvent::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => {
                length += CubicBezierSegment {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                }
                .approximate_length(tolerance)
            }
            PathEvent::End {
                last,
                first,
                close: true,
            } => {
                length += LineSegment {
                    from: last,
                    to: first,
                }
                .length()
            }
            _ => {}
        }
    }

    length
}

#[test]
fn approx_length() {
    use crate::geom::point;

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(1.0, 0.0));
    builder.line_to(point(1.0, 1.0));
    builder.line_to(point(0.0, 1.0));
    builder.end(true);

    let path = builder.build();

    assert!((approximate_length(&path, 0.01) - 4.0).abs() < 0.0001);
}
