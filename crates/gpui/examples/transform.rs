use gpui::{
    App, Application, Bounds, Context, IntoElement, Radians, Render, Size, TransformationMatrix,
    Window, WindowBounds, WindowOptions, canvas, div, fill, point, prelude::*, px, rgb, size,
};
use std::f32::consts::PI;

struct QuadTransformationsDemo;

impl Render for QuadTransformationsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(canvas(
            |_, _, _| {},
            move |_, _, window, _| {
                let window_size = window.viewport_size();
                let center_x = window_size.width.0 / 2.0;
                let center_y = window_size.height.0 / 2.0;

                // Draw background grid
                for i in 0..40 {
                    let x = i as f32 * 50.0;
                    window.paint_quad(fill(
                        Bounds::new(point(px(x), px(0.0)), size(px(1.0), px(600.0))),
                        rgb(0xEEEEEE),
                    ));

                    let y = i as f32 * 50.0;
                    window.paint_quad(fill(
                        Bounds::new(point(px(0.0), px(y)), size(px(800.0), px(1.0))),
                        rgb(0xEEEEEE),
                    ));
                }

                // Original quad (no transformation)
                let quad_size = 100.0;
                let original_bounds = Bounds::new(
                    point(
                        px(center_x - quad_size / 2.0),
                        px(center_y - quad_size / 2.0),
                    ),
                    size(px(quad_size), px(quad_size)),
                );
                window.paint_quad(fill(original_bounds, rgb(0x888888)));

                // 1. Rotation transformation (45 degrees)
                let rotation_matrix = TransformationMatrix::unit().rotate(Radians(PI / 4.0));

                let rotated_bounds = Bounds::new(
                    point(
                        px(center_x - quad_size / 2.0 + 150.0),
                        px(center_y - quad_size / 2.0),
                    ),
                    size(px(quad_size), px(quad_size)),
                );

                let rotated_quad = fill(rotated_bounds, rgb(0xFF5252));
                window.paint_quad(rotated_quad.transformation(rotation_matrix));

                // 2. Scaling transformation (1.5x)
                let scale_matrix = TransformationMatrix::unit().scale(Size::new(1.5, 0.8));

                let scaled_bounds = Bounds::new(
                    point(
                        px(center_x - quad_size / 2.0 - 150.0),
                        px(center_y - quad_size / 2.0),
                    ),
                    size(px(quad_size), px(quad_size)),
                );

                let scaled_quad = fill(scaled_bounds, rgb(0x4CAF50));
                window.paint_quad(scaled_quad.transformation(scale_matrix));

                // 3. Combined transformation (rotate and scale)
                let combined_matrix = TransformationMatrix::unit()
                    .rotate(Radians(PI / 6.0))
                    .scale(Size::new(1.2, 1.2));

                let combined_bounds = Bounds::new(
                    point(
                        px(center_x - quad_size / 2.0),
                        px(center_y - quad_size / 2.0 + 150.0),
                    ),
                    size(px(quad_size), px(quad_size)),
                );

                let combined_quad = fill(combined_bounds, rgb(0x2196F3));
                window.paint_quad(combined_quad.transformation(combined_matrix));
            },
        ))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| QuadTransformationsDemo),
        )
        .unwrap();
        cx.activate(true);
    });
}
