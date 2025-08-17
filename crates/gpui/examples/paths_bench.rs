use gpui::{
    Application, Background, Bounds, ColorSpace, Context, Path, PathBuilder, Pixels, Render,
    TitlebarOptions, Window, WindowBounds, WindowOptions, canvas, div, linear_color_stop,
    linear_gradient, point, prelude::*, px, rgb, size,
};

const DEFAULT_WINDOW_WIDTH: Pixels = px(1024.0);
const DEFAULT_WINDOW_HEIGHT: Pixels = px(768.0);

struct PaintingViewer {
    default_lines: Vec<(Path<Pixels>, Background)>,
    _painting: bool,
}

impl PaintingViewer {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let mut lines = vec![];

        // draw a lightening bolt ⚡
        for _ in 0..2000 {
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
        }

        Self {
            default_lines: lines,
            _painting: false,
        }
    }
}

impl Render for PaintingViewer {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        window.request_animation_frame();
        let lines = self.default_lines.clone();
        div().size_full().child(
            canvas(
                move |_, _, _| {},
                move |_, _, window, _| {
                    for (path, color) in lines {
                        window.paint_path(path, color);
                    }
                },
            )
            .size_full(),
        )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Vulkan".into()),
                    ..Default::default()
                }),
                focus: true,
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| PaintingViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
