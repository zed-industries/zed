use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};

struct Scrollable {}

impl Render for Scrollable {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .id("vertical")
            .p_4()
            .overflow_scroll()
            .bg(gpui::white())
            .child("Example for test 2 way scroll in nested layout")
            .child(
                div()
                    .h(px(5000.))
                    .border_1()
                    .border_color(gpui::blue())
                    .bg(gpui::blue().opacity(0.05))
                    .p_4()
                    .child(
                        div()
                            .mb_5()
                            .w_full()
                            .id("horizontal")
                            .overflow_scroll()
                            .child(
                                div()
                                    .w(px(2000.))
                                    .h(px(150.))
                                    .bg(gpui::green().opacity(0.1))
                                    .hover(|this| this.bg(gpui::green().opacity(0.2)))
                                    .border_1()
                                    .border_color(gpui::green())
                                    .p_4()
                                    .child("Scroll Horizontal"),
                            ),
                    )
                    .child("Scroll Vertical"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Scrollable {}),
        )
        .unwrap();
        cx.activate(true);
    });
}
