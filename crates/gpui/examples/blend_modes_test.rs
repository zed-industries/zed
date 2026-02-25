use gpui::*;

struct Test {}

impl Test {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for Test {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .bg(rgba(0xE0E0E0FF))
            .flex()
            .child(
                div()
                    .bg(gpui::red().alpha(0.5))
                    .h(px(100.0))
                    .w(px(100.0))
                    .flex()
                    .child(
                        div()
                            .bg(gpui::blue().alpha(0.5))
                            .h(px(50.0))
                            .w(px(50.0))
                            .m(px(25.0))
                    )
            )
    }
}

fn main() {
    // Use Application::new() to start the runtime
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(150.0), px(150.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| {
                // In this local GPUI version, cx.new is used to create the view entity
                cx.new(|cx| Test::new(cx))
            },
        ).unwrap();
    });
}