use tiny_skia::*;

#[test]
fn round_caps_and_large_scale() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let path = {
        let mut pb = PathBuilder::new();
        pb.move_to(60.0 / 16.0, 100.0 / 16.0);
        pb.line_to(140.0 / 16.0, 100.0 / 16.0);
        pb.finish().unwrap()
    };

    let mut stroke = Stroke::default();
    stroke.width = 6.0;
    stroke.line_cap = LineCap::Round;

    let transform = Transform::from_scale(16.0, 16.0);

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, transform, None);

    let expected = Pixmap::load_png("tests/images/stroke/round-caps-and-large-scale.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn circle() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let path = PathBuilder::from_circle(100.0, 100.0, 50.0).unwrap();
    let mut stroke = Stroke::default();
    stroke.width = 2.0;

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::default(), None);

    let expected = Pixmap::load_png("tests/images/stroke/circle.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn zero_len_subpath_butt_cap() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(50.0, 50.0);
    pb.line_to(50.0, 50.0);
    let path = pb.finish().unwrap();

    let mut stroke = Stroke::default();
    stroke.width = 20.0;
    stroke.line_cap = LineCap::Butt;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::default(), None);

    let expected = Pixmap::load_png("tests/images/stroke/zero-len-subpath-butt-cap.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn zero_len_subpath_round_cap() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(50.0, 50.0);
    pb.line_to(50.0, 50.0);
    let path = pb.finish().unwrap();

    let mut stroke = Stroke::default();
    stroke.width = 20.0;
    stroke.line_cap = LineCap::Round;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::default(), None);

    let expected = Pixmap::load_png("tests/images/stroke/zero-len-subpath-round-cap.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn zero_len_subpath_square_cap() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(50.0, 50.0);
    pb.line_to(50.0, 50.0);
    let path = pb.finish().unwrap();

    let mut stroke = Stroke::default();
    stroke.width = 20.0;
    stroke.line_cap = LineCap::Square;

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::default(), None);

    let expected = Pixmap::load_png("tests/images/stroke/zero-len-subpath-square-cap.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn round_cap_join() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(170.0, 30.0);
    pb.line_to(30.553378, 99.048418);
    pb.cubic_to(30.563658, 99.066835, 30.546308, 99.280724, 30.557592, 99.305282);
    let path = pb.finish().unwrap();

    let mut stroke = Stroke::default();
    stroke.width = 30.0;
    stroke.line_cap = LineCap::Round;
    stroke.line_join = LineJoin::Round;

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::default(), None);

    let expected = Pixmap::load_png("tests/images/stroke/round-cap-join.png").unwrap();
    assert_eq!(pixmap, expected);
}
