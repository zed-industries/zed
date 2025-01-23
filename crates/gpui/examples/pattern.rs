use gpui::{
    div, hash_pattern, prelude::*, px, rgb, size, App, AppContext, Bounds, ViewContext,
    WindowBounds, WindowOptions,
};

struct PatternExample;

impl Render for PatternExample {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("Pattern Example")
            .child(div().size(px(60.0)).bg(hash_pattern(gpui::red())))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| PatternExample),
        )
        .unwrap();
    });
}
