use tiny_skia::*;

// This example will crate a 20_000x20_000px image, which can take a while in a debug mode.
// This example is used mainly to tests that our tiling algorithm actually works and doesn't panic.

fn main() {
    let path1 = {
        let mut pb = PathBuilder::new();
        pb.move_to(1200.0, 1200.0);
        pb.line_to(3200.0, 18800.0);
        pb.cubic_to(7600.0, 16800.0, 13200.0, 16000.0, 18800.0, 16000.0);
        pb.cubic_to(14800.0, 9200.0, 8800.0, 3200.0, 1200.0, 1200.0);
        pb.close();
        pb.finish().unwrap()
    };

    let path2 = {
        let mut pb = PathBuilder::new();
        pb.move_to(18800.0, 1200.0);
        pb.line_to(16800.0, 18800.0);
        pb.cubic_to(12400.0, 16800.0, 6800.0, 16000.0, 1200.0, 16000.0);
        pb.cubic_to(5200.0, 9200.0, 11200.0, 3200.0, 18800.0, 1200.0);
        pb.close();
        pb.finish().unwrap()
    };

    let mut pixmap = Pixmap::new(20000, 20000).unwrap();

    let clip_path = {
        let mut pb = PathBuilder::new();
        pb.push_circle(10000.0, 10000.0, 7000.0);
        pb.finish().unwrap()
    };

    let mut mask = Mask::new(20000, 20000).unwrap();
    mask.fill_path(&clip_path, FillRule::Winding, true, Transform::default());

    let mut paint = Paint::default();
    paint.set_color_rgba8(90, 175, 100, 150);
    paint.anti_alias = true;
    let large_rect = Rect::from_xywh(500.0, 500.0, 19000.0, 19000.0).unwrap();
    pixmap.fill_rect(large_rect, &paint, Transform::identity(), None);

    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;
    pixmap.fill_path(
        &path1,
        &paint,
        FillRule::Winding,
        Transform::default(),
        Some(&mask),
    );

    paint.set_color_rgba8(220, 140, 75, 180);
    paint.anti_alias = false;
    pixmap.fill_path(
        &path2,
        &paint,
        FillRule::Winding,
        Transform::default(),
        None,
    );

    paint.set_color_rgba8(255, 10, 15, 180);
    paint.anti_alias = true;
    let mut stroke = Stroke::default();
    stroke.width = 0.8; // hairline
    pixmap.stroke_path(&path2, &paint, &stroke, Transform::default(), None);

    pixmap.save_png("image.png").unwrap();
}
