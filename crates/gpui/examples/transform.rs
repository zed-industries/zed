use gpui::{
    App, Application, Bounds, Context, IntoElement, Radians, Render, Size, TransformationMatrix,
    Window, WindowBounds, WindowOptions, canvas, div, fill, point, prelude::*, px, rgb, scaled_px,
    size,
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
                let rotated_bounds = Bounds::new(
                    point(
                        px(center_x - quad_size / 2.0 + 150.0),
                        px(center_y - quad_size / 2.0),
                    ),
                    size(px(quad_size), px(quad_size)),
                );

                // Calculate center of the quad for transformation
                let rotated_center = point(px(center_x + 150.0), px(center_y));

                // Create transformation matrix that rotates around the center
                let to_origin = TransformationMatrix::unit().translate(point(
                    scaled_px(-rotated_center.x.0, window),
                    scaled_px(-rotated_center.y.0, window),
                ));
                let rotation = TransformationMatrix::unit().rotate(Radians(PI / 4.0));
                let from_origin = TransformationMatrix::unit().translate(point(
                    scaled_px(rotated_center.x.0, window),
                    scaled_px(rotated_center.y.0, window),
                ));

                // Compose the transformations: first to origin, then rotate, then back from origin
                let rotation_matrix = from_origin.compose(rotation.compose(to_origin));

                let rotated_quad = fill(rotated_bounds, rgb(0xFF5252));
                window.paint_quad(rotated_quad.transformation(rotation_matrix));

                // 2. Scaling transformation (1.5x)
                let scaled_bounds = Bounds::new(
                    point(
                        px(center_x - quad_size / 2.0 - 150.0),
                        px(center_y - quad_size / 2.0),
                    ),
                    size(px(quad_size), px(quad_size)),
                );

                // Calculate center of the quad for transformation
                let scaled_center = point(px(center_x - 150.0), px(center_y));

                // Create transformation matrix that scales around the center
                let to_origin = TransformationMatrix::unit().translate(point(
                    scaled_px(-scaled_center.x.0, window),
                    scaled_px(-scaled_center.y.0, window),
                ));
                let scaling = TransformationMatrix::unit().scale(Size::new(1.5, 0.8));
                let from_origin = TransformationMatrix::unit().translate(point(
                    scaled_px(scaled_center.x.0, window),
                    scaled_px(scaled_center.y.0, window),
                ));

                // Compose the transformations: first to origin, then scale, then back from origin
                let scale_matrix = from_origin.compose(scaling.compose(to_origin));

                let scaled_quad = fill(scaled_bounds, rgb(0x4CAF50));
                window.paint_quad(scaled_quad.transformation(scale_matrix));

                // 3. Combined transformation (rotate and scale)
                let combined_bounds = Bounds::new(
                    point(
                        px(center_x - quad_size / 2.0),
                        px(center_y - quad_size / 2.0 + 150.0),
                    ),
                    size(px(quad_size), px(quad_size)),
                );

                // Calculate center of the quad for transformation
                let combined_center = point(px(center_x), px(center_y + 150.0));

                // Create transformation matrix that combines rotation and scaling around the center
                let to_origin = TransformationMatrix::unit().translate(point(
                    scaled_px(-combined_center.x.0, window),
                    scaled_px(-combined_center.y.0, window),
                ));
                let rotation = TransformationMatrix::unit().rotate(Radians(PI / 6.0));
                let scaling = TransformationMatrix::unit().scale(Size::new(1.2, 1.2));
                let from_origin = TransformationMatrix::unit().translate(point(
                    scaled_px(combined_center.x.0, window),
                    scaled_px(combined_center.y.0, window),
                ));

                // Compose the transformations: first to origin, then rotate and scale, then back from origin
                let combined_matrix =
                    from_origin.compose(scaling.compose(rotation.compose(to_origin)));

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
