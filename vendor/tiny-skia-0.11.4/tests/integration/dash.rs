use tiny_skia::*;

#[test]
fn line() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(90.0, 80.0);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = false;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![5.0, 10.0], 0.0);
    stroke.width = 2.0;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/dash/line.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn quad() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.quad_to(35.0, 75.0, 90.0, 80.0);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = false;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![5.0, 10.0], 0.0);
    stroke.width = 2.0;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/dash/quad.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn cubic() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.cubic_to(95.0, 35.0, 0.0, 75.0, 75.0, 90.0);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = false;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![5.0, 10.0], 0.0);
    stroke.width = 2.0;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/dash/cubic.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn hairline() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.cubic_to(95.0, 35.0, 0.0, 75.0, 75.0, 90.0);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![5.0, 10.0], 0.0);
    stroke.width = 0.5;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/dash/hairline.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn complex() {
    let mut pb = PathBuilder::new();
    pb.move_to(28.7, 23.9);
    pb.line_to(177.4, 35.2);
    pb.line_to(177.4, 68.0);
    pb.line_to(129.7, 68.0);
    pb.cubic_to(81.6, 59.3, 41.8, 63.3, 33.4, 115.2);
    pb.cubic_to(56.8, 128.7, 77.3, 143.8, 53.3, 183.8);
    pb.cubic_to(113.8, 185.7, 91.0, 109.7, 167.3, 111.8);
    pb.cubic_to(-56.2, 90.3, 177.3, 68.0, 110.2, 95.5);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![10.0, 5.0], 2.0);
    stroke.width = 2.0;

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/dash/complex.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn multi_subpaths() {
    let mut pb = PathBuilder::new();
    pb.move_to(49.0, 76.0);
    pb.cubic_to(22.0, 150.0, 11.0, 213.0, 186.0, 151.0);
    pb.cubic_to(194.0, 106.0, 195.0, 64.0, 169.0, 26.0);
    pb.move_to(124.0, 41.0);
    pb.line_to(162.0, 105.0);
    pb.cubic_to(135.0, 175.0, 97.0, 166.0, 53.0, 128.0);
    pb.line_to(93.0, 71.0);
    pb.move_to(24.0, 52.0);
    pb.line_to(108.0, 20.0);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![10.0, 5.0], 2.0);
    stroke.width = 2.0;

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/dash/multi_subpaths.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn closed() {
    let mut pb = PathBuilder::new();
    pb.move_to(22.0, 22.0);
    pb.cubic_to(63.0, 16.0, 82.0, 24.0, 84.0, 46.0);
    pb.cubic_to(86.0, 73.0, 15.0, 58.0, 16.0, 89.0);
    pb.close();
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![10.0, 5.0], 2.0);
    stroke.width = 2.0;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/dash/closed.png").unwrap();
    assert_eq!(pixmap, expected);
}
