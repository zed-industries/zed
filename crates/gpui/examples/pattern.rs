use gpui::{
    div, linear_color_stop, linear_gradient, pattern_horizontal_dash, pattern_slash,
    pattern_vertical_dash, prelude::*, px, rgb, size, App, AppContext, Application, Bounds,
    Context, Window, WindowBounds, WindowOptions,
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
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .w(px(160.0))
                                    .h(px(1.0))
                                    .bg(pattern_horizontal_dash(gpui::red())),
                            )
                            .child(
                                div()
                                    .w(px(160.0))
                                    .h(px(4.0))
                                    .bg(pattern_horizontal_dash(gpui::red())),
                            )
                            .child(
                                div()
                                    .w(px(160.0))
                                    .h(px(8.0))
                                    .bg(pattern_horizontal_dash(gpui::red())),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .child(
                                div()
                                    .w(px(1.0))
                                    .h(px(160.0))
                                    .bg(pattern_vertical_dash(gpui::blue())),
                            )
                            .child(
                                div()
                                    .w(px(4.0))
                                    .h(px(160.0))
                                    .bg(pattern_vertical_dash(gpui::blue())),
                            )
                            .child(
                                div()
                                    .w(px(8.0))
                                    .h(px(160.0))
                                    .bg(pattern_vertical_dash(gpui::blue())),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .border_1()
                    .border_color(gpui::blue())
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(gpui::red())))
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(gpui::red())))
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(gpui::red())))
                    .child(div().w(px(54.0)).h(px(18.0)).bg(pattern_slash(gpui::red()))),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .border_1()
                    .border_color(gpui::blue())
                    .bg(gpui::green().opacity(0.16))
                    .child("Elements the same height should align")
                    .child(
                        div()
                            .w(px(256.0))
                            .h(px(56.0))
                            .bg(pattern_slash(gpui::red())),
                    )
                    .child(
                        div()
                            .w(px(256.0))
                            .h(px(56.0))
                            .bg(pattern_slash(gpui::green())),
                    )
                    .child(
                        div()
                            .w(px(256.0))
                            .h(px(56.0))
                            .bg(pattern_slash(gpui::blue())),
                    )
                    .child(
                        div()
                            .w(px(256.0))
                            .h(px(26.0))
                            .bg(pattern_slash(gpui::yellow())),
                    ),
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
