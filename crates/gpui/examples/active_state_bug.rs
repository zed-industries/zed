/// Click the button — the `.active()` background gets stuck on every other click.
use gpui::*;
use gpui_platform::application;

struct Example;

impl Render for Example {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Colors from Zed's default dark theme
        let bg = hsla(215. / 360., 0.12, 0.15, 1.);
        let text = hsla(221. / 360., 0.11, 0.86, 1.);
        let hover = hsla(225. / 360., 0.118, 0.267, 1.);
        let active = hsla(220. / 360., 0.118, 0.20, 1.);

        div().bg(bg).size_full().p_1().child(
            div()
                .id("button")
                .px_2()
                .py_0p5()
                .rounded_md()
                .text_sm()
                .text_color(text)
                .hover(|s| s.bg(hover))
                .active(|s| s.bg(active))
                .on_click(|_, _, _| {})
                .child("Click me"),
        )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(200.), px(60.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Example),
        )
        .unwrap();
        cx.activate(true);
    });
}
