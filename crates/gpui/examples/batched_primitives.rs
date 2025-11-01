#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("batched_primitives: this example currently renders on macOS/Metal only.");
}

#[cfg(target_os = "macos")]
mod demo {
    use gpui::{
        App, Application, Bounds, Context, Hsla, Pixels, Point, RectInstance, Window, WindowBounds,
        WindowOptions, canvas, div, point, prelude::*, px, rgb, size,
    };

    fn background() -> Hsla { rgb(0x000000).into() }

    struct BatchedPrimitives {
        viewport_x: f32,
        viewport_y: f32,
        dragging: bool,
        last_mouse: Option<Point<Pixels>>,
    }

    impl BatchedPrimitives {
        fn new() -> Self {
            Self { viewport_x: 0.0, viewport_y: 0.0, dragging: false, last_mouse: None }
        }

        fn on_mouse_down(&mut self, _e: &gpui::MouseDownEvent, _w: &mut Window, _cx: &mut Context<Self>) {
            self.dragging = true;
        }
        fn on_mouse_up(&mut self, _e: &gpui::MouseUpEvent, _w: &mut Window, _cx: &mut Context<Self>) {
            self.dragging = false;
            self.last_mouse = None;
        }
        fn on_mouse_move(&mut self, e: &gpui::MouseMoveEvent, w: &mut Window, _cx: &mut Context<Self>) {
            if !self.dragging {
                self.last_mouse = Some(e.position);
                return;
            }
            if let Some(prev) = self.last_mouse {
                let dx = e.position.x - prev.x;
                let dy = e.position.y - prev.y;
                self.viewport_x += (dx / px(1.0)) as f32;
                self.viewport_y += (dy / px(1.0)) as f32;
                w.refresh();
            }
            self.last_mouse = Some(e.position);
        }

        fn generate_dots(&self) -> Vec<RectInstance> {
            let count = std::env::var("GPUI_PRIM_COUNT")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(100_000);
            const SIZE: f32 = 1.0;
            const A_OUT: f32 = 520.0;
            const B_OUT: f32 = 360.0;
            const SCALE_IN: f32 = 0.10;
            const JITTER: f32 = 8.0;

            struct Rng { state: u64 }
            impl Rng {
                fn new(seed: u64) -> Self { Self { state: seed } }
                fn next_f32(&mut self) -> f32 {
                    self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    let v = (self.state >> 32) as u32;
                    (v as f32) / (u32::MAX as f32 + 1.0)
                }
            }
            let mut rng = Rng::new(0xB1u64.wrapping_shl(32) ^ 0xA5A5_A5A5u64);

            let mut rects = Vec::with_capacity(count);
            let cx = 400.0 + self.viewport_x;
            let cy = 280.0 + self.viewport_y;
            for _ in 0..count {
                let theta = rng.next_f32() * std::f32::consts::TAU;
                let u = rng.next_f32();
                let s = ((SCALE_IN * SCALE_IN) + u * (1.0 - SCALE_IN * SCALE_IN)).sqrt();
                let mut x = cx + (A_OUT * s) * theta.cos();
                let mut y = cy + (B_OUT * s) * theta.sin();
                let jx = (rng.next_f32() - 0.5) * 2.0 * JITTER;
                let jy = (rng.next_f32() - 0.5) * 2.0 * JITTER;
                x += jx;
                y += jy;
                rects.push(RectInstance {
                    bounds: Bounds { origin: point(px(x), px(y)), size: size(px(SIZE), px(SIZE)) },
                    color: Hsla { h: 210.0, s: 0.85, l: 0.55, a: 1.0 },
                });
            }
            rects
        }
    }

    impl Render for BatchedPrimitives {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let dots = self.generate_dots();
            div()
                .bg(background())
                .size_full()
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_mouse_up(gpui::MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_up_out(gpui::MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_move(cx.listener(Self::on_mouse_move))
                .child(canvas(
                    move |_bounds, _window, _cx| {},
                    move |_bounds, _state, window, _cx| {
                        window.paint_batched_rects(&dots);
                    },
                ))
        }
    }

    pub fn main() {
        Application::new().run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(800.0), px(560.0)), cx);
            cx.open_window(
                WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), ..Default::default() },
                |_, cx| cx.new(|_| BatchedPrimitives::new()),
            )
            .unwrap();
            cx.activate(true);
        });
    }
}

#[cfg(target_os = "macos")]
fn main() { demo::main(); }
