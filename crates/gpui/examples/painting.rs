use gpui::{
    canvas, div, point, prelude::*, px, rgb, size, App, AppContext, Bounds, Hsla, MouseDownEvent,
    Path, Pixels, Point, Render, ViewContext, WindowContext, WindowOptions,
};

struct PaintingViewer {
    default_lines: Vec<(Path<Pixels>, Hsla)>,
    lines: Vec<Vec<Point<Pixels>>>,
    start: Point<Pixels>,
    _painting: bool,
}

/// Build tiny-skia PathBuilder into a Path with stroke
fn stroke_path(
    builder: tiny_skia::PathBuilder,
    stroke: &tiny_skia::Stroke,
    cx: &WindowContext,
) -> Option<Path<Pixels>> {
    let skia_path = builder.finish()?;
    let skia_path = skia_path.stroke(stroke, cx.scale_factor())?;
    let first_p = skia_path.points().first()?;
    let mut path = Path::new(point(px(first_p.x), px(first_p.y)));
    for segment in skia_path.segments() {
        match segment {
            tiny_skia::PathSegment::MoveTo(p) => {
                path.move_to(point(px(p.x), px(p.y)));
            }
            tiny_skia::PathSegment::LineTo(p) => {
                path.line_to(point(px(p.x), px(p.y)));
            }
            tiny_skia::PathSegment::QuadTo(p1, p2) => {
                path.curve_to(point(px(p1.x), px(p1.y)), point(px(p2.x), px(p2.y)));
            }
            tiny_skia::PathSegment::CubicTo(_p1, _p2, _p3) => {
                // TODO: convert cubic to quadratic
            }
            _ => {}
        }
    }
    Some(path)
}

impl PaintingViewer {
    fn new(cx: &WindowContext) -> Self {
        let mut lines = vec![];

        // draw a line
        let stroke = tiny_skia::Stroke {
            width: 4.0,
            ..Default::default()
        };
        let mut builder = tiny_skia::PathBuilder::new();
        builder.move_to(50.0, 180.);
        builder.line_to(100.0, 120.);
        let path = stroke_path(builder, &stroke, cx).unwrap();
        let mut builder = tiny_skia::PathBuilder::new();
        lines.push((path, rgb(0xdc2626).into()));
        builder.move_to(50.0, 120.);
        builder.line_to(100.0, 180.);
        let path = stroke_path(builder, &stroke, cx).unwrap();
        lines.push((path, rgb(0xdc2626).into()));

        // draw a lightening bolt ⚡
        let mut path = Path::new(point(px(150.), px(200.)));
        path.line_to(point(px(200.), px(125.)));
        path.line_to(point(px(200.), px(175.)));
        path.line_to(point(px(250.), px(100.)));
        lines.push((path, rgb(0x1d4ed8).into()));

        // draw a ⭐
        let mut path = Path::new(point(px(350.), px(100.)));
        path.line_to(point(px(370.), px(160.)));
        path.line_to(point(px(430.), px(160.)));
        path.line_to(point(px(380.), px(200.)));
        path.line_to(point(px(400.), px(260.)));
        path.line_to(point(px(350.), px(220.)));
        path.line_to(point(px(300.), px(260.)));
        path.line_to(point(px(320.), px(200.)));
        path.line_to(point(px(270.), px(160.)));
        path.line_to(point(px(330.), px(160.)));
        path.line_to(point(px(350.), px(100.)));
        lines.push((path, rgb(0xfacc15).into()));

        let square_bounds = Bounds {
            origin: point(px(450.), px(100.)),
            size: size(px(200.), px(80.)),
        };
        let height = square_bounds.size.height;
        let horizontal_offset = height;
        let vertical_offset = px(30.);
        let mut path = Path::new(square_bounds.bottom_left());
        path.curve_to(
            square_bounds.origin + point(horizontal_offset, vertical_offset),
            square_bounds.origin + point(px(0.0), vertical_offset),
        );
        path.line_to(square_bounds.top_right() + point(-horizontal_offset, vertical_offset));
        path.curve_to(
            square_bounds.bottom_right(),
            square_bounds.top_right() + point(px(0.0), vertical_offset),
        );
        path.line_to(square_bounds.bottom_left());
        lines.push((path, rgb(0x16a34a).into()));

        // draw a wave
        let mut builder = tiny_skia::PathBuilder::new();
        builder.move_to(40.0, 320.);
        for i in 0..80 {
            builder.line_to(
                40.0 + i as f32 * 10.0,
                320.0 + (i as f32 * 10.0).sin() * 40.0,
            );
        }
        let path = stroke_path(
            builder,
            &tiny_skia::Stroke {
                width: 1.0,
                line_cap: tiny_skia::LineCap::Round,
                ..Default::default()
            },
            cx,
        )
        .unwrap();
        lines.push((path, rgb(0xe00b00).into()));

        Self {
            default_lines: lines.clone(),
            lines: vec![],
            start: point(px(0.), px(0.)),
            _painting: false,
        }
    }

    fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.lines.clear();
        cx.notify();
    }
}
impl Render for PaintingViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                            .on_click(cx.listener(|this, _, cx| {
                                this.clear(cx);
                            })),
                    ),
            )
            .child(
                div()
                    .size_full()
                    .child(
                        canvas(
                            move |_, _| {},
                            move |_, _, cx| {

                                for (path, color) in default_lines {
                                    cx.paint_path(path, color);
                                }

                                let stroke = tiny_skia::Stroke {
                                    width: 1.0,
                                    ..Default::default()
                                };

                                for points in lines {
                                    if points.len() < 2 {
                                        continue;
                                    }

                                    let mut builder = tiny_skia::PathBuilder::new();
                                    let first_p = points.first().unwrap();
                                    builder.move_to(first_p.x.0, first_p.y.0);
                                    for p in points.iter().skip(1) {
                                        builder.line_to(p.x.0, p.y.0);
                                    }

                                    if let Some(path) = stroke_path(builder, &stroke, cx) {
                                        cx.paint_path(path, gpui::black());
                                    }
                                }
                            },
                        )
                        .size_full(),
                    )
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, ev: &MouseDownEvent, _| {
                            this._painting = true;
                            this.start = ev.position;
                            let path = vec![ev.position];
                            this.lines.push(path);
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, ev: &gpui::MouseMoveEvent, cx| {
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
                        cx.listener(|this, _, _| {
                            this._painting = false;
                        }),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |cx| cx.new_view(|cx| PaintingViewer::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
