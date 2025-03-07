use gpui::{
    div, prelude::*, px, size, App, Application, Bounds, Context, InlineText, Window, WindowBounds,
    WindowOptions,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("page")
            .size_full()
            .flex()
            .flex_col()
            .p_2()
            .gap_2()
            .bg(gpui::black())
            .text_color(gpui::white())
            .child(
                div().border_1().border_color(gpui::red()).child(
                    InlineText::new()
                        .add_text("This is an inline element ->")
                        .add_element(|_window| {
                            div()
                                .h_4()
                                .w_4()
                                .bg(gpui::red())
                                .text_xs()
                                .child("😊")
                                .into_any_element()
                        })
                        .add_text("<-")
                        .add_text(" This is an inline element ->")
                        .add_element(|_window| {
                            div()
                                .h_4()
                                .w_12()
                                .bg(gpui::green())
                                .flex()
                                .justify_center()
                                .text_xs()
                                .child("😊")
                                .into_any_element()
                        })
                        .add_text("<-")
                        .add_text(" This is an inline element ->")
                        .add_element(|_window| {
                            div()
                                .h_4()
                                .w_4()
                                .bg(gpui::blue())
                                .text_xs()
                                .child("😊")
                                .into_any_element()
                        })
                        .add_text("<-"),
                ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| RootView),
        )
        .unwrap();
        cx.activate(true);
    });
}
