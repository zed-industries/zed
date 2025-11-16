use gpui::{
    App, AppContext, Application, Bounds, Context, ParentElement, Render, Styled, Window,
    WindowBounds, WindowOptions, div, px, rgb, shader, size,
};

struct ShaderExample {}

impl Render for ShaderExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0x202060))
            .gap_2()
            .child(shader().size_full())
            .child(shader().size_full())
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
            |_, cx| cx.new(|_| ShaderExample {}),
        )
        .unwrap();
        cx.activate(true);
    });
}
