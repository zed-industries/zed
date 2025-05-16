use gpui::{
    AnyElement, App, Application, Bounds, Context, Hsla, IntoElement, Radians, Render,
    TransformationMatrix, Window, WindowBounds, WindowOptions, canvas, div, fill, point,
    prelude::*, px, rgb, scaled_px, size,
};
use smol::Timer;
use std::{f32::consts::PI, time::Duration};

#[derive(IntoElement)]
struct ExampleTile {
    fg_color: Hsla,
    bg_color: Hsla,
    example: AnyElement,
}

impl ExampleTile {
    fn new(bg_color: Hsla, fg_color: Hsla, example: impl IntoElement) -> Self {
        Self {
            bg_color,
            fg_color,
            example: example.into_any_element(),
        }
    }
}

impl RenderOnce for ExampleTile {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            // .overflow_hidden()
            .w(px(200.))
            .h(px(200.))
            .bg(self.bg_color)
            .border_1()
            .border_color(self.fg_color)
            .child(self.example)
    }
}

struct TransformExample {
    epoch: u64,
}

impl Render for TransformExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let grid = div().flex().flex_wrap().size_full();

        // Create a 5x3 grid with 15 example slots
        let mut grid_with_examples = grid;

        // grid_with_examples = grid_with_examples.child(ExampleTile::new(
        //     gpui::green(),
        //     gpui::white(),
        //     render_slow_rotation_scaling_example(self.epoch, window, cx),
        // ));
        grid_with_examples = grid_with_examples.child(ExampleTile::new(
            gpui::blue(),
            gpui::white(),
            render_rotation_scaling_example(self.epoch, window, cx),
        ));

        div()
            .bg(rgb(0x121212))
            .size_full()
            .child(grid_with_examples)
    }
}

impl TransformExample {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.spawn_in(window, async move |example, cx| {
            loop {
                Timer::after(Duration::from_millis(16)).await;
                example
                    .update(cx, |example, cx| {
                        example.update_epoch(cx);
                    })
                    .ok();
            }
        })
        .detach();

        Self { epoch: 0 }
    }

    fn update_epoch(&mut self, cx: &mut Context<Self>) {
        const MAX_EPOCH: u64 = 360;

        let direction = if (self.epoch / MAX_EPOCH) % 2 == 0 {
            1
        } else {
            -1
        };

        self.epoch = (self.epoch as i64 + direction).rem_euclid(MAX_EPOCH as i64) as u64;

        cx.notify();
    }
}

fn render_rotation_scaling_example(
    epoch: u64,
    _window: &mut Window,
    _cx: &mut App,
) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |canvas_bounds, _, window, _| {
            let canvas_center = canvas_bounds.center();

            let quad_size = 80.0;
            // Quad centered on the origin.
            let quad_bounds =
                Bounds::new(point(px(0.), px(0.)), size(px(quad_size), px(quad_size)));

            let scale_factor = 0.75 + 0.5 * ((epoch as f32 / 180.0) * PI).sin().abs();
            let rotation_angle = (epoch as f32 / 360.0) * 2.0 * PI;

            let matrix = TransformationMatrix::unit()
                .translate(point(
                    scaled_px(canvas_center.x.0, window),
                    scaled_px(canvas_center.y.0, window),
                ))
                .rotate(Radians(rotation_angle))
                .scale(size(scale_factor, scale_factor))
                .translate(point(
                    scaled_px(quad_size / -2.0, window),
                    scaled_px(quad_size / -2.0, window),
                ));

            let quad = fill(quad_bounds, rgb(0xFFFF00));
            window.paint_quad(quad.transformation(matrix));
            // window.paint_quad(quad);
        },
    )
    .size_full()
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(980.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| TransformExample::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
