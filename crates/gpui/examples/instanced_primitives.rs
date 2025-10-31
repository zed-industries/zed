#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("instanced_primitives: this example currently renders on macOS/Metal only.");
}

#[cfg(target_os = "macos")]
mod demo {
    use gpui::{
        App, Application, Bounds, Context, Hsla, Pixels, Point, RectInstance, Window, WindowBounds,
        WindowOptions, canvas, div, point, prelude::*, px, rgb, size,
    };

    fn background() -> Hsla {
        rgb(0x000000).into()
    }

    struct InstancedPrimitivesDemo {
        viewport_x: f32,
        viewport_y: f32,
        dragging: bool,
        last_mouse: Option<Point<Pixels>>,
    }

    impl InstancedPrimitivesDemo {
        fn new() -> Self {
            Self {
                viewport_x: 0.0,
                viewport_y: 0.0,
                dragging: false,
                last_mouse: None,
            }
        }
    }

    impl InstancedPrimitivesDemo {
        fn on_mouse_down(
            &mut self,
            _e: &gpui::MouseDownEvent,
            _w: &mut Window,
            _cx: &mut Context<Self>,
        ) {
            self.dragging = true;
        }

        fn on_mouse_up(
            &mut self,
            _e: &gpui::MouseUpEvent,
            _w: &mut Window,
            _cx: &mut Context<Self>,
        ) {
            self.dragging = false;
            self.last_mouse = None;
        }

        fn on_mouse_move(
            &mut self,
            e: &gpui::MouseMoveEvent,
            w: &mut Window,
            _cx: &mut Context<Self>,
        ) {
            if !self.dragging {
                self.last_mouse = Some(e.position);
                return;
            }
            if let Some(prev) = self.last_mouse {
                let delta_x = e.position.x - prev.x;
                let delta_y = e.position.y - prev.y;
                self.viewport_x += (delta_x / px(1.0)) as f32;
                self.viewport_y += (delta_y / px(1.0)) as f32;
                w.refresh();
            }
            self.last_mouse = Some(e.position);
        }

        fn generate_rect_grid(&self) -> Vec<RectInstance> {
            let mut rects = Vec::new();

            const GRID_SIZE: i32 = 100;
            const RECT_SIZE: f32 = 3.0;
            const SPACING: f32 = 4.0;

            for x in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    let pos_x = x as f32 * SPACING + self.viewport_x;
                    let pos_y = y as f32 * SPACING + self.viewport_y;

                    let hue = 200.0 + (x as f32 / GRID_SIZE as f32) * 60.0;
                    let lightness = 0.5 + (y as f32 / GRID_SIZE as f32) * 0.4;
                    let color = Hsla {
                        h: hue,
                        s: 0.8,
                        l: lightness,
                        a: 1.0,
                    };

                    rects.push(RectInstance {
                        bounds: Bounds {
                            origin: point(px(pos_x), px(pos_y)),
                            size: size(px(RECT_SIZE), px(RECT_SIZE)),
                        },
                        color,
                    });
                }
            }

            rects
        }

        fn generate_line_grid(&self) -> Vec<RectInstance> {
            let mut lines = Vec::new();

            const GRID_SIZE: i32 = 80;
            const LINE_LENGTH: f32 = 4.0;
            const LINE_WIDTH: f32 = 0.5;
            const SPACING: f32 = 5.0;

            for x in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    let base_x = x as f32 * SPACING + self.viewport_x + 500.0;
                    let base_y = y as f32 * SPACING + self.viewport_y;

                    let h_hue = 120.0 + (x as f32 / GRID_SIZE as f32) * 80.0;
                    let h_color = Hsla {
                        h: h_hue,
                        s: 0.9,
                        l: 0.6 + (y as f32 / GRID_SIZE as f32) * 0.3,
                        a: 1.0,
                    };

                    lines.push(RectInstance {
                        bounds: Bounds {
                            origin: point(px(base_x), px(base_y)),
                            size: size(px(LINE_LENGTH), px(LINE_WIDTH)),
                        },
                        color: h_color,
                    });

                    let v_hue = 280.0 + (y as f32 / GRID_SIZE as f32) * 60.0;
                    let v_color = Hsla {
                        h: v_hue,
                        s: 0.9,
                        l: 0.5 + (x as f32 / GRID_SIZE as f32) * 0.4,
                        a: 1.0,
                    };

                    lines.push(RectInstance {
                        bounds: Bounds {
                            origin: point(px(base_x + 2.0), px(base_y)),
                            size: size(px(LINE_WIDTH), px(LINE_LENGTH)),
                        },
                        color: v_color,
                    });
                }
            }

            lines
        }
    }

    impl Render for InstancedPrimitivesDemo {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            // Generate both grids
            let mut all_rects = self.generate_rect_grid();
            all_rects.extend(self.generate_line_grid());

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
                        window.paint_rects_instanced(&all_rects);
                    },
                ))
        }
    }

    pub fn main() {
        Application::new().run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(800.0), px(560.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| InstancedPrimitivesDemo::new()),
            )
            .unwrap();
            cx.activate(true);
        });
    }
}

#[cfg(target_os = "macos")]
fn main() {
    demo::main();
}
