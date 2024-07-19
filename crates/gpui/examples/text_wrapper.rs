use gpui::*;

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = "Hello, world 你好世界，this is GPUI.";
        div()
            .size_full()
            .flex()
            .flex_col()
            .p_2()
            .gap_2()
            .bg(gpui::white())
            .child(
                div()
                    .text_xl()
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .w(px(220.))
                    .child(text),
            )
            .child(
                div()
                    .text_xl()
                    .whitespace_nowrap()
                    .truncate()
                    .w(px(220.))
                    .child(text),
            )
            .child(div().text_xl().whitespace_nowrap().w(px(220.)).child(text))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| HelloWorld {}),
        )
        .unwrap();
    });
}
