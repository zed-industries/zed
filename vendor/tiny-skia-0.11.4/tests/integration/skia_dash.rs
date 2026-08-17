use tiny_skia::*;

#[test]
fn crbug_140642() {
    // We used to see this construct, and due to rounding as we accumulated
    // our length, the loop where we apply the phase would run off the end of
    // the array, since it relied on just -= each interval value, which did not
    // behave as "expected". Now the code explicitly checks for walking off the
    // end of that array.
    //
    // A different (better) fix might be to rewrite dashing to do all of its
    // length/phase/measure math using double, but this may need to be
    // coordinated with SkPathMeasure, to be consistent between the two.
    assert!(StrokeDash::new(vec![27734.0, 35660.0, 2157846850.0, 247.0], -248.135982067).is_some());
}

#[test]
fn crbug_124652() {
    // http://code.google.com/p/chromium/issues/detail?id=124652
    // This particular test/bug only applies to the float case, where
    // large values can "swamp" small ones.
    assert!(StrokeDash::new(vec![837099584.0, 33450.0], -10.0).is_some());
}

// Extremely large path_length/dash_length ratios may cause infinite looping
// due to single precision rounding.
#[test]
fn infinite_dash() {
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 5.0);
    pb.line_to(5000000.0, 5.0);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut stroke = Stroke::default();
    stroke.dash = StrokeDash::new(vec![0.2, 0.2], 0.0);

    let mut pixmap = Pixmap::new(100, 100).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None); // Doesn't draw anything.

    assert!(true);
}
