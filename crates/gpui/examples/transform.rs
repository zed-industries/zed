use gpui::{
    App, Application, Bounds, Context, IntoElement, Radians, Render, TransformationMatrix, Window,
    WindowBounds, WindowOptions, canvas, div, fill, point, prelude::*, px, rgb, scaled_px, size,
};
use smol::Timer;
use std::{f32::consts::PI, time::Duration};

struct TransformExample {
    epoch: u64,
}

impl Render for TransformExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::black())
            .size_full()
            .child(render_canvas(self.epoch, window, cx))
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

fn render_canvas(epoch: u64, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |_, _, window, _| {
            let epoch = epoch;
            let window_size = window.viewport_size();
            let center_x = window_size.width.0 / 2.0;
            let center_y = window_size.height.0 / 2.0;

            let quad_size = 100.0;
            let original_bounds = Bounds::new(
                point(
                    px(center_x - quad_size / 2.0),
                    px(center_y - quad_size / 2.0),
                ),
                size(px(quad_size), px(quad_size)),
            );
            window.paint_quad(fill(original_bounds, rgb(0x00FFFF)));

            let rotated_bounds = Bounds::new(
                point(
                    px(center_x - quad_size / 2.0 + 300.0),
                    px(center_y - quad_size / 2.0),
                ),
                size(px(quad_size), px(quad_size)),
            );

            let rotated_center = point(px(center_x + 300.0), px(center_y));

            let to_origin = TransformationMatrix::unit().translate(point(
                scaled_px(-rotated_center.x.0, window),
                scaled_px(-rotated_center.y.0, window),
            ));
            let rotation_angle = (epoch as f32 / 360.0) * 2.0 * PI;
            let rotation = TransformationMatrix::unit().rotate(Radians(rotation_angle));
            let from_origin = TransformationMatrix::unit().translate(point(
                scaled_px(rotated_center.x.0, window),
                scaled_px(rotated_center.y.0, window),
            ));

            let scale_factor = 0.75 + 0.5 * ((epoch as f32 / 180.0) * PI).sin().abs();
            let scaling = TransformationMatrix::unit().scale(size(scale_factor, scale_factor));

            let rotation_matrix = from_origin.compose(rotation.compose(scaling.compose(to_origin)));

            let rotated_quad = fill(rotated_bounds, rgb(0xFFFF00));
            window.paint_quad(rotated_quad.transformation(rotation_matrix));

            let translated_bounds = Bounds::new(
                point(
                    px(center_x - quad_size / 2.0 - 300.0),
                    px(center_y - quad_size / 2.0),
                ),
                size(px(quad_size), px(quad_size)),
            );

            let translation_x = (((epoch as f32 / 90.0) * PI).sin() * 100.0) + 50.0;
            let translation = TransformationMatrix::unit().translate(point(
                scaled_px(translation_x, window),
                scaled_px(0.0, window),
            ));

            let translated_quad = fill(translated_bounds, rgb(0x00FF00));
            window.paint_quad(translated_quad.transformation(translation));

            let combined_bounds = Bounds::new(
                point(
                    px(center_x - quad_size / 2.0 + 300.0),
                    px(center_y - quad_size / 2.0 + 150.0),
                ),
                size(px(quad_size), px(quad_size)),
            );

            let combined_center = point(px(center_x + 300.0), px(center_y + 150.0));

            let to_origin = TransformationMatrix::unit().translate(point(
                scaled_px(-combined_center.x.0, window),
                scaled_px(-combined_center.y.0, window),
            ));
            let combined_rotation_angle = -((epoch as f32 / 360.0) * 2.0 * PI);
            let rotation = TransformationMatrix::unit().rotate(Radians(combined_rotation_angle));
            let translation_x = ((epoch as f32 / 90.0) * 2.0 * PI).cos() * 75.0;
            let translation_y = ((epoch as f32 / 90.0) * 2.0 * PI).sin() * 30.0;
            let translation = TransformationMatrix::unit().translate(point(
                scaled_px(translation_x, window),
                scaled_px(translation_y, window),
            ));
            let scale_x = 1.0 + 0.3 * ((epoch as f32 / 120.0) * 2.0 * PI).cos();
            let scale_y = 1.0 + 0.3 * ((epoch as f32 / 120.0) * 2.0 * PI).sin();
            let scaling = TransformationMatrix::unit().scale(size(scale_x, scale_y));
            let from_origin = TransformationMatrix::unit().translate(point(
                scaled_px(combined_center.x.0, window),
                scaled_px(combined_center.y.0, window),
            ));

            let combined_matrix = from_origin
                .compose(translation.compose(scaling.compose(rotation.compose(to_origin))));

            let combined_quad = fill(combined_bounds, rgb(0x00FF00));
            window.paint_quad(combined_quad.transformation(combined_matrix));

            let scale_bounds = Bounds::new(
                point(
                    px(center_x - quad_size / 2.0),
                    px(center_y - quad_size / 2.0 - 150.0),
                ),
                size(px(quad_size), px(quad_size)),
            );

            let scale_center = point(px(center_x), px(center_y - 150.0));

            let to_origin = TransformationMatrix::unit().translate(point(
                scaled_px(-scale_center.x.0, window),
                scaled_px(-scale_center.y.0, window),
            ));

            let breath_scale = 0.5 + ((epoch as f32 / 360.0) * 2.0 * PI).sin().abs();
            let scaling = TransformationMatrix::unit().scale(size(breath_scale, breath_scale));

            let from_origin = TransformationMatrix::unit().translate(point(
                scaled_px(scale_center.x.0, window),
                scaled_px(scale_center.y.0, window),
            ));

            let scale_matrix = from_origin.compose(scaling.compose(to_origin));

            let scale_quad = fill(scale_bounds, rgb(0xFF00FF));
            window.paint_quad(scale_quad.transformation(scale_matrix));
        },
    )
    .size_full()
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
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
