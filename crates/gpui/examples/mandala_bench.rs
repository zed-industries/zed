//! A benchmark example that renders a mandala pattern with ~1M points across ~500 paths.
//! Each path is rotated slightly to create a symmetric, kaleidoscopic effect.
//! Displays an FPS counter in the top left and continuously redraws.

use std::f32::consts::{PI, TAU};
use std::sync::Arc;
use std::time::Instant;

use gpui::{
    Application, Background, Bounds, Context, Path, PathBuilder, Pixels, Point, Render, ScaledPixels, TitlebarOptions, Window, WindowBounds, WindowOptions, canvas, div, hsla, linear_color_stop, linear_gradient, point, prelude::*, px, size
};

const DEFAULT_WINDOW_WIDTH: Pixels = px(1200.0);
const DEFAULT_WINDOW_HEIGHT: Pixels = px(1000.0);

// Benchmark parameters - adjust these to test different loads
const NUM_PATHS: usize = 500;
const POINTS_PER_PATH: usize = 2000; // ~1M total points

struct MandalaViewer {
    paths: Vec<(Arc<Path<ScaledPixels>>, Background)>,
    frame_times: Vec<f32>,
    last_frame: Instant,
    total_points: usize,
}

impl MandalaViewer {
    fn new(window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let center = point(DEFAULT_WINDOW_WIDTH / 2.0, DEFAULT_WINDOW_HEIGHT / 2.0);
        let max_radius = DEFAULT_WINDOW_WIDTH.min(DEFAULT_WINDOW_HEIGHT) / 2.0 - px(50.0);

        let mut paths = Vec::with_capacity(NUM_PATHS);
        let mut total_points = 0;

        let scale_factor = window.scale_factor();

        for path_idx in 0..NUM_PATHS {
            // Rotation angle for this path in the mandala
            let rotation = (path_idx as f32 / NUM_PATHS as f32) * TAU;

            // Generate a beautiful spiral/flower petal shape
            let (path, point_count) =
                Self::create_petal_path(center, max_radius, rotation, POINTS_PER_PATH);
            total_points += point_count;

            // Create gradient color based on path index for a rainbow effect
            let hue = path_idx as f32 / NUM_PATHS as f32;
            let color = linear_gradient(
                rotation.to_degrees(),
                linear_color_stop(hsla(hue, 0.8, 0.5, 0.6), 0.0),
                linear_color_stop(hsla((hue + 0.1) % 1.0, 0.9, 0.6, 0.4), 1.0),
            );

            paths.push((Arc::new(path.scale(scale_factor)), color));
        }

        Self {
            paths,
            frame_times: Vec::with_capacity(120),
            last_frame: Instant::now(),
            total_points,
        }
    }

    /// Creates a flower petal/spiral path with the specified number of points
    fn create_petal_path(
        center: Point<Pixels>,
        max_radius: Pixels,
        rotation: f32,
        num_points: usize,
    ) -> (Path<Pixels>, usize) {
        let mut builder = PathBuilder::stroke(px(1.0));

        // Parameters for the petal shape
        let petal_count = 5.0; // Number of lobes in each petal
        let inner_ratio = 0.15; // How deep the valleys go
        let spiral_factor = 0.3; // Amount of spiral twist
        let wave_amplitude = 0.08; // Secondary wave for detail

        let cos_rot = rotation.cos();
        let sin_rot = rotation.sin();

        // Generate points along the petal curve
        for i in 0..num_points {
            let t = i as f32 / (num_points - 1) as f32;
            let angle = t * TAU * 2.0; // Go around twice for more detail

            // Rose curve formula with modifications for interesting shape
            // r = cos(k * theta) creates a rose curve
            let k = petal_count;
            let base_r = (k * angle).cos().abs();

            // Add inner variation
            let inner_wave = 1.0 - inner_ratio * (petal_count * 2.0 * angle).sin().abs();

            // Add spiral twist
            let spiral = 1.0 + spiral_factor * t;

            // Add fine detail waves
            let detail = 1.0 + wave_amplitude * (angle * 20.0).sin();

            // Combine all factors
            let r = max_radius * base_r * inner_wave * spiral * detail * 0.8;

            // Add the spiral angle offset
            let final_angle = angle + t * PI * spiral_factor;

            // Calculate point position (before rotation)
            let local_x = r * final_angle.cos();
            let local_y = r * final_angle.sin();

            // Apply rotation for mandala effect
            let rotated_x = local_x * cos_rot - local_y * sin_rot;
            let rotated_y = local_x * sin_rot + local_y * cos_rot;

            let point = point(center.x + rotated_x, center.y + rotated_y);

            if i == 0 {
                builder.move_to(point);
            } else {
                builder.line_to(point);
            }
        }

        builder.close();

        let path = builder.build().unwrap();
        (path, num_points)
    }

    fn update_fps(&mut self) -> f32 {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame).as_secs_f32() * 1000.0;
        self.last_frame = now;

        self.frame_times.push(frame_time);
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        let avg_frame_time: f32 =
            self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        1000.0 / avg_frame_time
    }
}

impl Render for MandalaViewer {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // Request continuous redraw for benchmarking
        window.request_animation_frame();

        let fps = self.update_fps();
        let total_points = self.total_points;
        let num_paths = self.paths.len();

        let paths = self.paths.clone();

        div()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.05, 1.0)) // Dark background
            .child(
                // FPS counter overlay
                div()
                    .absolute()
                    .top_4()
                    .left_4()
                    .p_3()
                    .bg(hsla(0.0, 0.0, 0.0, 0.7))
                    .rounded_md()
                    .text_color(gpui::white())
                    .text_sm()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(format!("FPS: {:.1}", fps))
                    .child(format!("Paths: {}", num_paths))
                    .child(format!("Points: {}K", total_points / 1000))
                    .child(format!("Frame: {:.2}ms", 1000.0 / fps)),
            )
            .child(
                canvas(
                    move |_, _, _| {},
                    move |_, _, window, _| {
                        for (path, color) in &paths {
                            window.paint_cached_path(path.clone(), *color);
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
                    title: Some("Mandala Benchmark - Path Rendering".into()),
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
            |window, cx| cx.new(|cx| MandalaViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

