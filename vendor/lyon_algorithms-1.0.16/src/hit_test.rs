//! Determine whether a point is inside a path.

use crate::geom::{CubicBezierSegment, LineSegment, QuadraticBezierSegment};
use crate::math::Point;
use crate::path::{FillRule, PathEvent};

/// Returns whether the point is inside the path.
pub fn hit_test_path<Iter>(point: &Point, path: Iter, fill_rule: FillRule, tolerance: f32) -> bool
where
    Iter: IntoIterator<Item = PathEvent>,
{
    let winding = path_winding_number_at_position(point, path, tolerance);

    match fill_rule {
        FillRule::EvenOdd => winding % 2 != 0,
        FillRule::NonZero => winding != 0,
    }
}

/// Compute the winding number of a given position with respect to the path.
pub fn path_winding_number_at_position<Iter>(point: &Point, path: Iter, tolerance: f32) -> i32
where
    Iter: IntoIterator<Item = PathEvent>,
{
    // Loop over the edges and compute the winding number at that point by accumulating the
    // winding of all edges intersecting the horizontal line passing through our point which are
    // left of it.
    let mut winding = 0;

    for evt in path {
        match evt {
            PathEvent::Begin { .. } => {}
            PathEvent::Line { from, to } => {
                test_segment(
                    *point,
                    &LineSegment { from, to },
                    &mut winding,
                );
            }
            PathEvent::End { last, first, .. } => {
                test_segment(
                    *point,
                    &LineSegment {
                        from: last,
                        to: first,
                    },
                    &mut winding,
                );
            }
            PathEvent::Quadratic { from, ctrl, to } => {
                let segment = QuadraticBezierSegment { from, ctrl, to };
                let (min, max) = segment.fast_bounding_range_y();
                if min > point.y || max < point.y {
                    continue;
                }
                segment.for_each_flattened(tolerance, &mut |line| {
                    test_segment(*point, line, &mut winding);
                });
            }
            PathEvent::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => {
                let segment = CubicBezierSegment {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                };
                let (min, max) = segment.fast_bounding_range_y();
                if min > point.y || max < point.y {
                    continue;
                }
                segment.for_each_flattened(tolerance, &mut |line| {
                    test_segment(*point, line, &mut winding);
                });
            }
        }
    }

    winding
}

fn test_segment(
    point: Point,
    segment: &LineSegment<f32>,
    winding: &mut i32,
) {
    let y0 = segment.from.y;
    let y1 = segment.to.y;
    let min_y = f32::min(y0, y1);
    let max_y = f32::max(y0, y1);

    if min_y > point.y
        || max_y <= point.y
        || f32::min(segment.from.x, segment.to.x) > point.x
    {
        return;
    }

    if y0 == y1 {
        return;
    }

    let d = y1 - y0;


    let t = (point.y - y0) / d;
    let x = segment.sample(t).x;

    if x > point.x {
        return;
    }

    let w = if d > 0.0 { 1 } else { -1 };

    *winding += w;
}

#[test]
fn test_hit_test() {
    use crate::math::point;
    use crate::path::Path;

    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(1.0, 0.0));
    builder.line_to(point(1.0, 1.0));
    builder.line_to(point(0.0, 1.0));
    builder.end(true);
    builder.begin(point(0.25, 0.25));
    builder.line_to(point(0.75, 0.25));
    builder.line_to(point(0.75, 0.75));
    builder.line_to(point(0.20, 0.75));
    builder.end(true);
    let path = builder.build();

    assert!(!hit_test_path(
        &point(-1.0, 0.5),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));
    assert!(!hit_test_path(
        &point(2.0, 0.5),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));
    std::println!(
        "winding {:?}",
        path_winding_number_at_position(&point(2.0, 0.0), path.iter(), 0.1)
    );
    assert!(!hit_test_path(
        &point(2.0, 0.0),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));
    assert!(!hit_test_path(
        &point(0.5, -1.0),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));
    assert!(!hit_test_path(
        &point(0.5, 2.0),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));

    assert!(!hit_test_path(
        &point(0.5, 0.5),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));
    assert!(hit_test_path(
        &point(0.5, 0.5),
        path.iter(),
        FillRule::NonZero,
        0.1
    ));
    assert!(hit_test_path(
        &point(0.2, 0.5),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));
    assert!(hit_test_path(
        &point(0.8, 0.5),
        path.iter(),
        FillRule::EvenOdd,
        0.1
    ));
}

#[test]
fn hit_test_point_aligned() {
    use crate::math::point;
    use crate::path::polygon::Polygon;

    let poly = Polygon {
        points: &[
            point(-10.0, 10.0),
            point(10.0, 10.0),
            point(10.0, 5.0),
            point(10.0, -10.0),
            point(-10.0, -10.0),
        ],
        closed: true,
    };

    assert!(hit_test_path(
        &point(0.0, 5.0),
        poly.path_events(),
        FillRule::NonZero,
        0.1
    ));
    assert!(!hit_test_path(
        &point(15.0, 5.0),
        poly.path_events(),
        FillRule::NonZero,
        0.1
    ));
}

#[test]
fn hit_test_double_square() {
    use crate::math::point;
    use crate::path::Path;

    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(1.0, 0.0));
    builder.line_to(point(1.0, 1.0));
    builder.line_to(point(0.0, 1.0));
    builder.line_to(point(0.0, 0.0));
    builder.line_to(point(1.0, 0.0));
    builder.line_to(point(1.0, 1.0));
    builder.line_to(point(0.0, 1.0));
    builder.end(true);
    let path = builder.build();

    assert_eq!(
        path_winding_number_at_position(&point(0.5, 0.5), &path, 0.1),
        -2
    );
}

#[test]
fn hit_test_double_count() {
    use crate::math::point;
    use crate::path::Path;

    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 1.0));
    builder.line_to(point(1.0, 1.0));
    builder.line_to(point(1.0, 2.0));
    builder.line_to(point(1.0, 3.0));
    builder.line_to(point(3.0, 3.0));
    builder.line_to(point(3.0, 0.0));
    builder.end(true);
    let path = builder.build();

    assert_eq!(
        path_winding_number_at_position(&point(2.0, 1.0), &path, 0.1),
        1
    );
    assert_eq!(
        path_winding_number_at_position(&point(2.0, 2.0), &path, 0.1),
        1
    );
}

#[test]
fn issue_882() {
    use crate::math::point;
    use crate::path::Path;
    let mut pb = Path::builder();
    pb.begin(point(0.0, 50.0));
    pb.line_to(point(50.0, 50.0));
    pb.line_to(point(50.0, 0.0));
    pb.line_to(point(100.0, 0.0));
    pb.line_to(point(100.0, 100.0));
    pb.line_to(point(0.0, 100.0));
    pb.line_to(point(0.0, 50.0));
    pb.end(true);
    let p = pb.build();

    let x = point(55.0, 50.0);

    assert!(hit_test_path(&x, p.iter(), FillRule::EvenOdd, 1.0))
}