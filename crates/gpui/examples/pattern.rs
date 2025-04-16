use gpui::{
    App, AppContext, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div,
    linear_color_stop, linear_gradient, pattern_slash, prelude::*, px, rgb, size,
};

struct PatternExample;

impl Render for PatternExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0xffffff))
            .size(px(600.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0x000000))
            .child("Pattern Example")
            .child(
                div()
                    .flex()
                    .flex_col()
                    .border_1()
                    .border_color(gpui::blue())
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(
                        gpui::red(),
                        18.0 / 4.0,
                        18.0 / 4.0,
                    )))
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(
                        gpui::red(),
                        18.0 / 4.0,
                        18.0 / 4.0,
                    )))
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(
                        gpui::red(),
                        18.0 / 4.0,
                        18.0 / 4.0,
                    )))
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(
                        gpui::red(),
                        18.0 / 4.0,
                        18.0 / 2.0,
                    ))),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .border_1()
                    .border_color(gpui::blue())
                    .bg(gpui::green().opacity(0.16))
                    .child("Elements the same height should align")
                    .child(div().w(px(256.0)).h(px(56.0)).bg(pattern_slash(
                        gpui::red(),
                        56.0 / 6.0,
                        56.0 / 6.0,
                    )))
                    .child(div().w(px(256.0)).h(px(56.0)).bg(pattern_slash(
                        gpui::green(),
                        56.0 / 6.0,
                        56.0 / 6.0,
                    )))
                    .child(div().w(px(256.0)).h(px(56.0)).bg(pattern_slash(
                        gpui::blue(),
                        56.0 / 6.0,
                        56.0 / 6.0,
                    )))
                    .child(div().w(px(256.0)).h(px(26.0)).bg(pattern_slash(
                        gpui::yellow(),
                        56.0 / 6.0,
                        56.0 / 6.0,
                    ))),
            )
            .child(
                div()
                    .border_1()
                    .border_color(gpui::blue())
                    .w(px(240.0))
                    .h(px(40.0))
                    .bg(gpui::red()),
            )
            .child(
                div()
                    .border_1()
                    .border_color(gpui::blue())
                    .w(px(240.0))
                    .h(px(40.0))
                    .bg(linear_gradient(
                        45.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    )),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| PatternExample),
        )
        .unwrap();

        cx.activate(true);
    });
}
