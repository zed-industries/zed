use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .flex()
            .flex_col()
            .size_full()
            .gap_3()
            .bg(rgb(0x505050))
            .justify_center()
            .items_center()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .overflow_hidden()
                    .rounded(px(24.))
                    .border(px(4.))
                    .border_color(gpui::white())
                    .text_color(gpui::white())
                    .text_center()
                    .child(
                        div()
                            .bg(gpui::black())
                            .py_2()
                            .px_7()
                            .border_l_2()
                            .border_r_2()
                            .border_b_3()
                            .border_color(gpui::red())
                            .child("Let build applications with GPUI"),
                    )
                    .child(
                        div()
                            .bg(rgb(0x222222))
                            .text_sm()
                            .py_1()
                            .px_7()
                            .border_l_3()
                            .border_r_3()
                            .border_color(gpui::green())
                            .child("The fast, productive UI framework for Rust"),
                    )
                    .child(
                        div()
                            .bg(rgb(0x222222))
                            .w_full()
                            .flex()
                            .flex_row()
                            .text_sm()
                            .text_color(rgb(0xc0c0c0))
                            .child(
                                div()
                                    .flex_1()
                                    .p_2()
                                    .border_3()
                                    .border_dashed()
                                    .border_color(gpui::blue())
                                    .child("Rust"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p_2()
                                    .border_t_3()
                                    .border_r_3()
                                    .border_b_3()
                                    .border_dashed()
                                    .border_color(gpui::blue())
                                    .child("GPU Rendering"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::red())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::green())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::blue())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::yellow())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::black())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::white())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::black()),
                    ),
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
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
