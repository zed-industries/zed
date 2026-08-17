//! Approximate the area of a path.

use crate::geom::vector;
use crate::path::{iterator::PathIterator, PathEvent};

/// Compute the signed area of a path by summing the signed areas of its sub-paths.
pub fn approximate_signed_area<Iter>(tolerance: f32, path: Iter) -> f32
where
    Iter: IntoIterator<Item = PathEvent>,
{
    let mut path = path.into_iter();
    let mut area = 0.0;
    while let Some(sp_area) = approximate_sub_path_signed_area(tolerance, &mut path) {
        area += sp_area;
    }

    area
}

/// Compute the signed area of the next sub-path.
///
/// The iterator is advanced so that `approximate_sub_path_signed_area` can be called multiple times
/// to process the successive sub-paths of a path.
///
/// Returns `None` if there is no more sub-path or if the the iterator is malformed.
pub fn approximate_sub_path_signed_area<Iter>(tolerance: f32, path: &mut Iter) -> Option<f32>
where
    Iter: Iterator<Item = PathEvent>,
{
    let first = if let Some(PathEvent::Begin { at }) = path.next() {
        at
    } else {
        return None;
    };
    let mut double_area = 0.0;
    let mut v0 = vector(0.0, 0.0);

    for evt in path.flattened(tolerance) {
        match evt {
            PathEvent::Begin { .. } => {
                return None;
            }
            PathEvent::End { last, first, .. } => {
                let v1 = last - first;
                double_area += v0.cross(v1);

                return Some(double_area * 0.5);
            }
            PathEvent::Line { to, .. } => {
                let v1 = to - first;
                double_area += v0.cross(v1);
                v0 = v1;
            }
            PathEvent::Quadratic { .. } | PathEvent::Cubic { .. } => {
                debug_assert!(false, "Unexpected curve in a flattened path");
            }
        };
    }

    None
}

/// Iterator over the sub-path areas of a path.
pub struct SignedAreas<Iter = PathEvent>(pub Iter, f32);

impl<Iter: Iterator<Item = PathEvent>> Iterator for SignedAreas<Iter> {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        approximate_sub_path_signed_area(self.1, &mut self.0)
    }
}

#[test]
fn sub_path_signed_area() {
    use crate::geom::point;
    let mut path = crate::path::Path::builder();

    path.begin(point(0.0, 0.0));
    path.line_to(point(1.0, 0.0));
    path.line_to(point(1.0, 1.0));
    path.line_to(point(0.0, 1.0));
    path.close();

    path.begin(point(0.0, 0.0));
    path.line_to(point(0.0, 1.0));
    path.line_to(point(1.0, 1.0));
    path.line_to(point(1.0, 0.0));
    path.close();

    let path = path.build();

    let mut iter = path.iter();

    assert_eq!(approximate_sub_path_signed_area(0.01, &mut iter), Some(1.0));
    assert_eq!(
        approximate_sub_path_signed_area(0.01, &mut iter),
        Some(-1.0)
    );
    assert_eq!(approximate_sub_path_signed_area(0.01, &mut iter), None);

    let mut path = crate::path::Path::builder();

    path.begin(point(0.0, 1.0));
    path.line_to(point(1.0, 1.0));
    path.line_to(point(1.0, 0.0));
    path.line_to(point(2.0, 0.0));
    path.line_to(point(2.0, 1.0));
    path.line_to(point(3.0, 1.0));
    path.line_to(point(3.0, 2.0));
    path.line_to(point(2.0, 2.0));
    path.line_to(point(2.0, 3.0));
    path.line_to(point(1.0, 3.0));
    path.line_to(point(1.0, 2.0));
    path.line_to(point(0.0, 2.0));
    path.close();

    assert_eq!(approximate_signed_area(0.01, path.build().iter()), 5.0);
}
