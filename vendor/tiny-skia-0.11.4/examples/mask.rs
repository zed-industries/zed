use tiny_skia::*;

fn main() {
    let clip_path = {
        let mut pb = PathBuilder::new();
        pb.push_circle(250.0, 250.0, 200.0);
        pb.push_circle(250.0, 250.0, 100.0);
        pb.finish().unwrap()
    };

    let clip_path = clip_path
        .transform(Transform::from_row(1.0, -0.3, 0.0, 1.0, 0.0, 75.0))
        .unwrap();

    let mut mask = Mask::new(500, 500).unwrap();
    mask.fill_path(&clip_path, FillRule::EvenOdd, true, Transform::default());

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.set_color_rgba8(50, 127, 150, 200);

    let mut pixmap = Pixmap::new(500, 500).unwrap();
    pixmap.fill_rect(
        Rect::from_xywh(0.0, 0.0, 500.0, 500.0).unwrap(),
        &paint,
        Transform::identity(),
        Some(&mask),
    );
    pixmap.save_png("image.png").unwrap();
}
