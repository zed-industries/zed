// Compute the winding of a path.

use crate::geom::vector;
use crate::path::{PathEvent, Winding};

/// Compute the winding of the next sub-path.
///
/// The sub-path is expected to have a non-null area and no self-intersections, otherwise
/// the result is unspecified.
///
/// The iterator is advanced so that `compute_winding` can be called multiple times
/// to process the successive sub-paths of a path.
///
/// Returns `None` if there is no more sub-path or if the the iterator is malformed.
pub fn compute_winding<Iter>(path: &mut Iter) -> Option<Winding>
where
    Iter: Iterator<Item = PathEvent>,
{
    let first = if let Some(PathEvent::Begin { at }) = path.next() {
        at
    } else {
        return None;
    };
    let mut area = 0.0;
    let mut v0 = vector(0.0, 0.0);

    for evt in path {
        match evt {
            PathEvent::Begin { .. } => {
                return None;
            }
            PathEvent::End { last, first, .. } => {
                let v1 = last - first;
                area += v0.cross(v1);

                return if area > 0.0 {
                    Some(Winding::Positive)
                } else {
                    Some(Winding::Negative)
                };
            }
            PathEvent::Line { to, .. } => {
                let v1 = to - first;
                area += v0.cross(v1);
                v0 = v1;
            }
            PathEvent::Quadratic { ctrl, to, .. } => {
                let v1 = ctrl - first;
                let v2 = to - first;
                area += v0.cross(v1) + v1.cross(v2);
                v0 = v2;
            }
            PathEvent::Cubic {
                ctrl1, ctrl2, to, ..
            } => {
                let v1 = ctrl1 - first;
                let v2 = ctrl2 - first;
                let v3 = to - first;
                area += v0.cross(v1) + v1.cross(v2) + v2.cross(v3);
                v0 = v3;
            }
        };
    }

    None
}

/// Iterator over the sub-path windings of a path.
pub struct Windings<Iter = PathEvent>(pub Iter);

impl<Iter: Iterator<Item = PathEvent>> Iterator for Windings<Iter> {
    type Item = Winding;
    fn next(&mut self) -> Option<Winding> {
        compute_winding(&mut self.0)
    }
}

#[test]
fn path_winding() {
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

    assert_eq!(compute_winding(&mut iter), Some(Winding::Positive));
    assert_eq!(compute_winding(&mut iter), Some(Winding::Negative));
    assert_eq!(compute_winding(&mut iter), None);
}
