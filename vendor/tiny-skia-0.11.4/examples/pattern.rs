use tiny_skia::*;

fn main() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Bicubic,
        1.0,
        Transform::from_row(1.5, -0.4, 0.0, -0.8, 5.0, 1.0),
    );

    let path = PathBuilder::from_circle(200.0, 200.0, 180.0).unwrap();

    let mut pixmap = Pixmap::new(400, 400).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    pixmap.save_png("image.png").unwrap();
}

fn crate_triangle() -> Pixmap {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 20.0);
    pb.line_to(20.0, 20.0);
    pb.line_to(10.0, 0.0);
    pb.close();
    let path = pb.finish().unwrap();

    let mut pixmap = Pixmap::new(20, 20).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    pixmap
}
