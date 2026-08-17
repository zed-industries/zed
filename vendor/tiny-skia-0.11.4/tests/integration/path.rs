use tiny_skia::*;

#[test]
fn empty() {
    let pb = PathBuilder::new();
    assert!(pb.finish().is_none());
}

#[test]
fn line() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(10.0, 20.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
    ]);

    assert_eq!(format!("{:?}", path),
               "Path { segments: \"M 10 20 L 30 40\", \
                bounds: Rect { left: 10.0, top: 20.0, right: 30.0, bottom: 40.0 } }");
}

#[test]
fn no_move_before_line() {
    let mut pb = PathBuilder::new();
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(0.0, 0.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
       PathSegment::MoveTo(Point::from_xy(0.0, 0.0)),
       PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
    ]);
}

#[test]
fn no_move_before_quad() {
    let mut pb = PathBuilder::new();
    pb.quad_to(40.0, 30.0, 60.0, 75.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(0.0, 0.0, 60.0, 75.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
       PathSegment::MoveTo(Point::from_xy(0.0, 0.0)),
       PathSegment::QuadTo(Point::from_xy(40.0, 30.0), Point::from_xy(60.0, 75.0)),
    ]);
}

#[test]
fn no_move_before_cubic() {
    let mut pb = PathBuilder::new();
    pb.cubic_to(40.0, 30.0, 60.0, 75.0, 33.0, 66.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(0.0, 0.0, 60.0, 75.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(0.0, 0.0)),
        PathSegment::CubicTo(Point::from_xy(40.0, 30.0), Point::from_xy(60.0, 75.0), Point::from_xy(33.0, 66.0)),
    ]);
}

#[test]
fn no_move_before_close() {
    let mut pb = PathBuilder::new();
    pb.close();
    assert!(pb.finish().is_none());
}

#[test]
fn double_close() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 10.0);
    pb.line_to(20.0, 10.0);
    pb.line_to(20.0, 20.0);
    pb.close();
    pb.close();
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(10.0, 10.0, 20.0, 20.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 20.0)),
        PathSegment::Close,
    ]);
}

#[test]
fn double_move_to_1() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.move_to(30.0, 40.0);
    assert!(pb.finish().is_none());
}

#[test]
fn double_move_to_2() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.move_to(20.0, 10.0);
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(20.0, 10.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(20.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
    ]);
}

#[test]
fn two_contours() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    pb.move_to(100.0, 200.0);
    pb.line_to(300.0, 400.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(10.0, 20.0, 300.0, 400.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
        PathSegment::MoveTo(Point::from_xy(100.0, 200.0)),
        PathSegment::LineTo(Point::from_xy(300.0, 400.0)),
    ]);
}

#[test]
fn two_closed_contours() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    pb.close();
    pb.move_to(100.0, 200.0);
    pb.line_to(300.0, 400.0);
    pb.close();
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(10.0, 20.0, 300.0, 400.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
        PathSegment::Close,
        PathSegment::MoveTo(Point::from_xy(100.0, 200.0)),
        PathSegment::LineTo(Point::from_xy(300.0, 400.0)),
        PathSegment::Close,
    ]);
}

#[test]
fn line_after_close() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    pb.close();
    pb.line_to(20.0, 20.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(10.0, 20.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
        PathSegment::Close,
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 20.0)),
    ]);
}

#[test]
fn hor_line() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 10.0);
    pb.line_to(20.0, 10.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(10.0, 10.0, 20.0, 10.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 10.0)),
    ]);
}

#[test]
fn ver_line() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 10.0);
    pb.line_to(10.0, 20.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Rect::from_ltrb(10.0, 10.0, 10.0, 20.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(10.0, 20.0)),
    ]);
}

#[test]
fn translate() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let mut path = pb.finish().unwrap();

    path = path.transform(Transform::from_translate(10.0, 20.0)).unwrap();

    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(20.0, 40.0)),
        PathSegment::LineTo(Point::from_xy(40.0, 60.0)),
    ]);
}

#[test]
fn scale() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let mut path = pb.finish().unwrap();

    path = path.transform(Transform::from_scale(2.0, 0.5)).unwrap();

    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(20.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(60.0, 20.0)),
    ]);
}

#[test]
fn transform() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let mut path = pb.finish().unwrap();

    path = path.transform(Transform::from_row(2.0, 0.7, -0.3, 0.5, 10.0, 20.0)).unwrap();

    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(24.0, 37.0)),
        PathSegment::LineTo(Point::from_xy(58.0, 61.0)),
    ]);
}

#[test]
fn invalid_transform() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    // will produce infinity
    assert_eq!(path.transform(Transform::from_scale(std::f32::MAX, std::f32::MAX)), None);
}

#[test]
fn circle() {
    assert!(PathBuilder::from_circle(250.0, 250.0, 300.0).is_some()); // Must not panic.
}

#[test]
fn large_circle() {
    assert!(PathBuilder::from_circle(250.0, 250.0, 2000.0).is_some()); // Must not panic.
}

#[test]
fn tight_bounds_1() {
    let mut pb = PathBuilder::new();
    pb.move_to(50.0, 85.0);
    pb.line_to(65.0, 135.0);
    pb.line_to(150.0, 135.0);
    pb.line_to(85.0, 135.0);
    pb.quad_to(100.0, 45.0, 50.0, 85.0);
    let path = pb.finish().unwrap();
    let tight_bounds = path.compute_tight_bounds().unwrap();
    assert_eq!(path.bounds(), Rect::from_xywh(50.0, 45.0, 100.0, 90.0).unwrap());
    assert_eq!(tight_bounds, Rect::from_xywh(50.0, 72.692307, 100.0, 62.307693).unwrap());
}

#[test]
fn tight_bounds_2() {
    let mut pb = PathBuilder::new();
    pb.move_to(-19.309214, 72.11173);
    pb.cubic_to(-24.832062, 67.477516, -20.490944, 62.16584, -9.61306, 60.247776);
    pb.cubic_to(1.2648277, 58.329712, 14.560249, 60.53159, 20.083096, 65.16581);
    pb.cubic_to(14.560249, 60.53159, 18.901363, 55.219913, 29.779247, 53.30185);
    pb.cubic_to(40.65713, 51.383785, 53.952557, 53.585663, 59.475407, 58.21988);
    pb.quad_to(74.4754, 70.80637, 50.083096, 90.3388);
    pb.quad_to(-4.3092155, 84.69823, -19.309214, 72.11173);
    pb.close();
    let path = pb.finish().unwrap();
    let tight_bounds = path.compute_tight_bounds().unwrap();
    assert_eq!(tight_bounds, Rect::from_xywh(-21.707121, 52.609154, 86.894302, 37.729645).unwrap());
}
