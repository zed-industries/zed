use tiny_skia::*;

#[test]
fn two_stops_linear_pad_lq() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(190.0, 190.0),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.0, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-linear-pad-lq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn two_stops_linear_repeat_lq() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(100.0, 100.0),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.0, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Repeat,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-linear-repeat-lq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn two_stops_linear_reflect_lq() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(100.0, 100.0),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.0, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Reflect,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-linear-reflect-lq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn three_stops_evenly_spaced_lq() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(190.0, 190.0),
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(0.50, Color::from_rgba8(220, 140, 75, 180)),
            GradientStop::new(0.75, Color::from_rgba8(40, 180, 55, 160)),
        ],
        // No need to check other modes. "Two stops" tests will cover them.
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/three-stops-evenly-spaced-lq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn two_stops_unevenly_spaced_lq() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(190.0, 190.0),
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(0.75, Color::from_rgba8(220, 140, 75, 180)),
        ],
        // No need to check other modes. "Two stops" tests will cover them.
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-unevenly-spaced-lq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn two_stops_linear_pad_hq() {
    let mut paint = Paint::default();
    paint.force_hq_pipeline = true;
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(190.0, 190.0),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.0, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-linear-pad-hq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn two_stops_linear_repeat_hq() {
    let mut paint = Paint::default();
    paint.force_hq_pipeline = true;
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(100.0, 100.0),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.0, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Repeat,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-linear-repeat-hq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn two_stops_linear_reflect_hq() {
    let mut paint = Paint::default();
    paint.force_hq_pipeline = true;
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(100.0, 100.0),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.0, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Reflect,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-linear-reflect-hq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn three_stops_evenly_spaced_hq() {
    let mut paint = Paint::default();
    paint.force_hq_pipeline = true;
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(190.0, 190.0),
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(0.50, Color::from_rgba8(220, 140, 75, 180)),
            GradientStop::new(0.75, Color::from_rgba8(40, 180, 55, 160)),
        ],
        // No need to check other modes. "Two stops" tests will cover them.
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/three-stops-evenly-spaced-hq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn two_stops_unevenly_spaced_hq() {
    let mut paint = Paint::default();
    paint.force_hq_pipeline = true;
    paint.anti_alias = false;
    paint.shader = LinearGradient::new(
        Point::from_xy(10.0, 10.0),
        Point::from_xy(190.0, 190.0),
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(0.75, Color::from_rgba8(220, 140, 75, 180)),
        ],
        // No need to check other modes. "Two stops" tests will cover them.
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/two-stops-unevenly-spaced-hq.png").unwrap();
    assert_eq!(pixmap, expected);
}

// The radial gradient is only supported by the high quality pipeline.
// Therefore we do not have a lq/hq split.

#[test]
fn well_behaved_radial() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = RadialGradient::new(
        Point::from_xy(100.0, 100.0),
        Point::from_xy(120.0, 80.0),
        100.0,
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(0.75, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/well-behaved-radial.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn focal_on_circle_radial() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = RadialGradient::new(
        Point::from_xy(100.0, 100.0),
        Point::from_xy(120.0, 80.0),
        28.29, // This radius forces the required pipeline stage.
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(0.75, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/focal-on-circle-radial.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn conical_greater_radial() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = RadialGradient::new(
        Point::from_xy(100.0, 100.0),
        Point::from_xy(120.0, 80.0),
        10.0, // This radius forces the required pipeline stage.
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(0.75, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/conical-greater-radial.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn simple_radial_lq() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = RadialGradient::new(
        Point::from_xy(100.0, 100.0),
        Point::from_xy(100.0, 100.0),
        100.0,
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.00, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/simple-radial-lq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn simple_radial_hq() {
    let mut paint = Paint::default();
    paint.force_hq_pipeline = true;
    paint.anti_alias = false;
    paint.shader = RadialGradient::new(
        Point::from_xy(100.0, 100.0),
        Point::from_xy(100.0, 100.0),
        100.0,
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.00, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/simple-radial-hq.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn simple_radial_with_ts_hq() {
    let mut paint = Paint::default();
    paint.force_hq_pipeline = true;
    paint.anti_alias = false;
    paint.shader = RadialGradient::new(
        Point::from_xy(100.0, 100.0),
        Point::from_xy(100.0, 100.0),
        100.0,
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 200)),
            GradientStop::new(1.00, Color::from_rgba8(220, 140, 75, 180)),
        ],
        SpreadMode::Pad,
        Transform::from_row(2.0, 0.3, -0.7, 1.2, 10.5, -12.3),
    ).unwrap();

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/simple-radial-with-ts-hq.png").unwrap();
    assert_eq!(pixmap, expected);
}

// Gradient doesn't add the Premultiply stage when all stops are opaque.
// But it checks colors only on creation, so we have to recheck them after calling `apply_opacity`.
#[test]
fn global_opacity() {
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = RadialGradient::new(
        Point::from_xy(100.0, 100.0),
        Point::from_xy(100.0, 100.0),
        100.0,
        vec![
            GradientStop::new(0.25, Color::from_rgba8(50, 127, 150, 255)), // no opacity here
            GradientStop::new(1.00, Color::from_rgba8(220, 140, 75, 255)), // no opacity here
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).unwrap();
    paint.shader.apply_opacity(0.5);

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/gradients/global-opacity.png").unwrap();
    assert_eq!(pixmap, expected);
}
