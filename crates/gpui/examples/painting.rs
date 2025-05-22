use gpui::{
    Application, Background, Bounds, ColorSpace, Context, MouseDownEvent, Path, PathBuilder,
    PathStyle, Pixels, Point, Render, Size, StrokeOptions, Window, WindowOptions, canvas, div,
    linear_color_stop, linear_gradient, point, prelude::*, px, rgb, size,
};

struct PaintingViewer {
    default_lines: Vec<(Path<Pixels>, Background)>,
    lines: Vec<Vec<Point<Pixels>>>,
    start: Point<Pixels>,
    _painting: bool,
}

impl PaintingViewer {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let mut lines = vec![];

        // draw a Rust logo
        let mut builder = lyon::path::Path::svg_builder();
        lyon::extra::rust_logo::build_logo_path(&mut builder);
        // move down the Path
        let mut builder: PathBuilder = builder.into();
        builder.translate(point(px(10.), px(100.)));
        builder.scale(0.9);
        let path = builder.build().unwrap();
        lines.push((path, gpui::black().into()));

        // draw a lightening bolt ⚡
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(150.), px(200.)));
        builder.line_to(point(px(200.), px(125.)));
        builder.line_to(point(px(200.), px(175.)));
        builder.line_to(point(px(250.), px(100.)));
        let path = builder.build().unwrap();
        lines.push((path, rgb(0x1d4ed8).into()));

        // draw a ⭐
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(350.), px(100.)));
        builder.line_to(point(px(370.), px(160.)));
        builder.line_to(point(px(430.), px(160.)));
        builder.line_to(point(px(380.), px(200.)));
        builder.line_to(point(px(400.), px(260.)));
        builder.line_to(point(px(350.), px(220.)));
        builder.line_to(point(px(300.), px(260.)));
        builder.line_to(point(px(320.), px(200.)));
        builder.line_to(point(px(270.), px(160.)));
        builder.line_to(point(px(330.), px(160.)));
        builder.line_to(point(px(350.), px(100.)));
        let path = builder.build().unwrap();
        lines.push((
            path,
            linear_gradient(
                180.,
                linear_color_stop(rgb(0xFACC15), 0.7),
                linear_color_stop(rgb(0xD56D0C), 1.),
            )
            .color_space(ColorSpace::Oklab),
        ));

        let square_bounds = Bounds {
            origin: point(px(450.), px(100.)),
            size: size(px(200.), px(80.)),
        };
        let height = square_bounds.size.height;
        let horizontal_offset = height;
        let vertical_offset = px(30.);
        let mut builder = PathBuilder::fill();
        builder.move_to(square_bounds.bottom_left());
        builder.curve_to(
            square_bounds.origin + point(horizontal_offset, vertical_offset),
            square_bounds.origin + point(px(0.0), vertical_offset),
        );
        builder.line_to(square_bounds.top_right() + point(-horizontal_offset, vertical_offset));
        builder.curve_to(
            square_bounds.bottom_right(),
            square_bounds.top_right() + point(px(0.0), vertical_offset),
        );
        builder.line_to(square_bounds.bottom_left());
        let path = builder.build().unwrap();
        lines.push((
            path,
            linear_gradient(
                180.,
                linear_color_stop(gpui::blue(), 0.4),
                linear_color_stop(gpui::red(), 1.),
            ),
        ));

        // draw a wave
        let options = StrokeOptions::default()
            .with_line_width(1.)
            .with_line_join(lyon::path::LineJoin::Bevel);
        let mut builder = PathBuilder::stroke(px(1.)).with_style(PathStyle::Stroke(options));
        builder.move_to(point(px(40.), px(320.)));
        for i in 0..50 {
            builder.line_to(point(
                px(40.0 + i as f32 * 10.0),
                px(320.0 + (i as f32 * 10.0).sin() * 40.0),
            ));
        }
        let path = builder.build().unwrap();
        lines.push((path, gpui::green().into()));

        // draw the indicators (aligned and unaligned versions)
        let aligned_indicator = draw_indicator_aligned(size(px(100.), px(15.)));
        lines.push((aligned_indicator, rgb(0x1e88e5).into()));

        let unaligned_indicator = draw_indicator_unaligned(size(px(100.), px(15.)));
        lines.push((unaligned_indicator, rgb(0xe53935).into()));

        Self {
            default_lines: lines.clone(),
            lines: vec![],
            start: point(px(0.), px(0.)),
            _painting: false,
        }
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.lines.clear();
        cx.notify();
    }
}
impl Render for PaintingViewer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let default_lines = self.default_lines.clone();
        let lines = self.lines.clone();
        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_between()
                    .items_center()
                    .child("Mouse down any point and drag to draw lines (Hold on shift key to draw straight lines)")
                    .child(
                        div()
                            .id("clear")
                            .child("Clean up")
                            .bg(gpui::black())
                            .text_color(gpui::white())
                            .active(|this| this.opacity(0.8))
                            .flex()
                            .px_3()
                            .py_1()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.clear(cx);
                            })),
                    ),
            )
            .child(
                div()
                    .size_full()
                    .child(
                        canvas(
                            move |_, _, _| {},
                            move |_, _, window, _| {

                                for (path, color) in default_lines {
                                    window.paint_path(path, color);
                                }

                                for points in lines {
                                    if points.len() < 2 {
                                        continue;
                                    }

                                    let mut builder = PathBuilder::stroke(px(1.));
                                    for (i, p) in points.into_iter().enumerate() {
                                        if i == 0 {
                                            builder.move_to(p);
                                        } else {
                                            builder.line_to(p);
                                        }
                                    }

                                    if let Ok(path) = builder.build() {
                                        window.paint_path(path, gpui::black());
                                    }
                                }
                            },
                        )
                        .size_full(),
                    )
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, ev: &MouseDownEvent, _, _| {
                            this._painting = true;
                            this.start = ev.position;
                            let path = vec![ev.position];
                            this.lines.push(path);
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, ev: &gpui::MouseMoveEvent, _, cx| {
                        if !this._painting {
                            return;
                        }

                        let is_shifted = ev.modifiers.shift;
                        let mut pos = ev.position;
                        // When holding shift, draw a straight line
                        if is_shifted {
                            let dx = pos.x - this.start.x;
                            let dy = pos.y - this.start.y;
                            if dx.abs() > dy.abs() {
                                pos.y = this.start.y;
                            } else {
                                pos.x = this.start.x;
                            }
                        }

                        if let Some(path) = this.lines.last_mut() {
                            path.push(pos);
                        }

                        cx.notify();
                    }))
                    .on_mouse_up(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this._painting = false;
                        }),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |window, cx| cx.new(|cx| PaintingViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

// Draw an indicator without pixel alignment
fn draw_indicator_unaligned(size_param: Size<Pixels>) -> Path<Pixels> {
    let width = size_param.width;
    let height = size_param.height;

    // Position the indicator on the canvas
    let base_x = px(50.0);
    let base_y = px(300.0);

    // Calculate the scaling factor for the height (SVG is 15px tall)
    let scale_factor = height / px(15.0);

    // Calculate how much width to allocate to the stretchable middle section
    // SVG has 32px of fixed elements (corners), so the rest is for the middle
    let fixed_width = px(32.0) * scale_factor;
    let middle_width = width - fixed_width;

    // Create a new path
    let mut builder = PathBuilder::fill();

    // Upper half of the shape - Based on the provided SVG
    // Start at bottom left (0, 8)
    builder.move_to(point(base_x, base_y + px(7.5) * scale_factor));

    // Vertical line to (0, 5)
    builder.line_to(point(base_x, base_y + px(5.0) * scale_factor));

    // Curve to (5, 0) - approximating the cubic Bezier with a quadratic one
    builder.curve_to(
        point(base_x + px(5.0) * scale_factor, base_y),
        point(
            base_x + px(0.75) * scale_factor,
            base_y + px(2.5) * scale_factor,
        ),
    );

    // Horizontal line through the middle section to (37, 0)
    builder.line_to(point(
        base_x + px(5.0) * scale_factor + middle_width,
        base_y,
    ));

    // Horizontal line to (41, 0)
    builder.line_to(point(
        base_x + px(5.0) * scale_factor + middle_width + px(4.0) * scale_factor,
        base_y,
    ));

    // Curve to (50, 7.5) - approximating the cubic Bezier with a quadratic one
    builder.curve_to(
        point(
            base_x + px(5.0) * scale_factor + middle_width + px(13.0) * scale_factor,
            base_y + px(7.5) * scale_factor,
        ),
        point(
            base_x + px(5.0) * scale_factor + middle_width + px(8.5) * scale_factor,
            base_y + px(3.0) * scale_factor,
        ),
    );

    // Lower half of the shape - mirrored vertically
    // Curve from (50, 7.5) to (41, 15)
    builder.curve_to(
        point(
            base_x + px(5.0) * scale_factor + middle_width + px(4.0) * scale_factor,
            base_y + px(15.0) * scale_factor,
        ),
        point(
            base_x + px(5.0) * scale_factor + middle_width + px(8.5) * scale_factor,
            base_y + px(12.0) * scale_factor,
        ),
    );

    // Horizontal line to (37, 15)
    builder.line_to(point(
        base_x + px(5.0) * scale_factor + middle_width,
        base_y + px(15.0) * scale_factor,
    ));

    // Horizontal line through the middle section to (5, 15)
    builder.line_to(point(
        base_x + px(5.0) * scale_factor,
        base_y + px(15.0) * scale_factor,
    ));

    // Curve to (0, 10)
    builder.curve_to(
        point(base_x, base_y + px(10.0) * scale_factor),
        point(
            base_x + px(0.75) * scale_factor,
            base_y + px(12.5) * scale_factor,
        ),
    );

    // Close the path
    builder.line_to(point(base_x, base_y + px(7.5) * scale_factor));

    builder.build().unwrap()
}

// Draw an indicator with quarter-pixel alignment
fn draw_indicator_aligned(size_param: Size<Pixels>) -> Path<Pixels> {
    let width = size_param.width;
    let height = size_param.height;

    // Position the indicator on the canvas
    let base_x = px(50.0);
    let base_y = px(250.0);

    // Calculate the scaling factor for the height (SVG is 15px tall)
    let scale_factor = height / px(15.0);

    // Calculate how much width to allocate to the stretchable middle section
    // SVG has 32px of fixed elements (corners), so the rest is for the middle
    let fixed_width = px(32.0) * scale_factor;
    let middle_width = width - fixed_width;

    // Helper function to round to nearest quarter pixel
    let round_to_quarter = |value: Pixels| -> Pixels {
        let value_f32: f32 = value.into();
        px((value_f32 * 4.0).round() / 4.0)
    };

    // Create a new path
    let mut builder = PathBuilder::fill();

    // Upper half of the shape - Based on the provided SVG
    // Start at bottom left (0, 8)
    let start_x = round_to_quarter(base_x);
    let start_y = round_to_quarter(base_y + px(7.5) * scale_factor);
    builder.move_to(point(start_x, start_y));

    // Vertical line to (0, 5)
    let vert_y = round_to_quarter(base_y + px(5.0) * scale_factor);
    builder.line_to(point(start_x, vert_y));

    // Curve to (5, 0) - approximating the cubic Bezier with a quadratic one
    let curve1_end_x = round_to_quarter(base_x + px(5.0) * scale_factor);
    let curve1_end_y = round_to_quarter(base_y);
    let curve1_ctrl_x = round_to_quarter(base_x + px(0.75) * scale_factor);
    let curve1_ctrl_y = round_to_quarter(base_y + px(2.5) * scale_factor);
    builder.curve_to(
        point(curve1_end_x, curve1_end_y),
        point(curve1_ctrl_x, curve1_ctrl_y),
    );

    // Horizontal line through the middle section to (37, 0)
    let middle_end_x = round_to_quarter(base_x + px(5.0) * scale_factor + middle_width);
    builder.line_to(point(middle_end_x, curve1_end_y));

    // Horizontal line to (41, 0)
    let right_section_x =
        round_to_quarter(base_x + px(5.0) * scale_factor + middle_width + px(4.0) * scale_factor);
    builder.line_to(point(right_section_x, curve1_end_y));

    // Curve to (50, 7.5) - approximating the cubic Bezier with a quadratic one
    let curve2_end_x =
        round_to_quarter(base_x + px(5.0) * scale_factor + middle_width + px(13.0) * scale_factor);
    let curve2_end_y = round_to_quarter(base_y + px(7.5) * scale_factor);
    let curve2_ctrl_x =
        round_to_quarter(base_x + px(5.0) * scale_factor + middle_width + px(8.5) * scale_factor);
    let curve2_ctrl_y = round_to_quarter(base_y + px(3.0) * scale_factor);
    builder.curve_to(
        point(curve2_end_x, curve2_end_y),
        point(curve2_ctrl_x, curve2_ctrl_y),
    );

    // Lower half of the shape - mirrored vertically
    // Curve from (50, 7.5) to (41, 15)
    let curve3_end_y = round_to_quarter(base_y + px(15.0) * scale_factor);
    let curve3_ctrl_y = round_to_quarter(base_y + px(12.0) * scale_factor);
    builder.curve_to(
        point(right_section_x, curve3_end_y),
        point(curve2_ctrl_x, curve3_ctrl_y),
    );

    // Horizontal line to (37, 15)
    builder.line_to(point(middle_end_x, curve3_end_y));

    // Horizontal line through the middle section to (5, 15)
    builder.line_to(point(curve1_end_x, curve3_end_y));

    // Curve to (0, 10)
    let curve4_end_y = round_to_quarter(base_y + px(10.0) * scale_factor);
    let curve4_ctrl_y = round_to_quarter(base_y + px(12.5) * scale_factor);
    builder.curve_to(
        point(start_x, curve4_end_y),
        point(curve1_ctrl_x, curve4_ctrl_y),
    );

    // Close the path
    builder.line_to(point(start_x, start_y));

    builder.build().unwrap()
}
