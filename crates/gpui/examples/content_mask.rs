use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct Example {}

impl Render for Example {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .flex()
            .flex_col()
            .size_full()
            .p_4()
            .gap_4()
            .bg(rgb(0x505050))
            .justify_center()
            .items_center()
            .text_center()
            .shadow_lg()
            .text_sm()
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .overflow_hidden()
                    .rounded(px(32.))
                    .border(px(8.))
                    .border_color(gpui::white())
                    .text_color(gpui::white())
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
                    .flex_col()
                    .w(px(320.))
                    .gap_1()
                    .overflow_hidden()
                    .rounded(px(16.))
                    .child(
                        div()
                            .w_full()
                            .p_2()
                            .bg(gpui::red())
                            .child("Clip background"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(320.))
                    .gap_1()
                    .rounded(px(16.))
                    .child(
                        div()
                            .w_full()
                            .p_2()
                            .bg(gpui::yellow())
                            .text_color(gpui::black())
                            .child("No content mask"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(320.))
                    .gap_1()
                    .overflow_hidden()
                    .rounded(px(16.))
                    .child(
                        div()
                            .w_full()
                            .p_2()
                            .border_4()
                            .border_color(gpui::blue())
                            .bg(gpui::blue().alpha(0.4))
                            .child("Clip borders"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(320.))
                    .gap_1()
                    .overflow_hidden()
                    .rounded(px(20.))
                    .child(
                        div().w_full().border_2().border_color(gpui::black()).child(
                            div()
                                .size_full()
                                .bg(gpui::green().alpha(0.4))
                                .p_2()
                                .border_8()
                                .border_color(gpui::green())
                                .child("Clip nested elements"),
                        ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(320.))
                    .gap_1()
                    .overflow_hidden()
                    .rounded(px(32.))
                    .child(
                        div()
                            .w_full()
                            .p_2()
                            .bg(gpui::black())
                            .border_2()
                            .border_dashed()
                            .rounded_lg()
                            .border_color(gpui::white())
                            .child("dash border full and rounded"),
                    )
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .gap_2()
                            .child(
                                div()
                                    .w_full()
                                    .p_2()
                                    .bg(gpui::black())
                                    .border_x_2()
                                    .border_dashed()
                                    .rounded_lg()
                                    .border_color(gpui::white())
                                    .child("border x"),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .p_2()
                                    .bg(gpui::black())
                                    .border_y_2()
                                    .border_dashed()
                                    .rounded_lg()
                                    .border_color(gpui::white())
                                    .child("border y"),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .p_2()
                            .bg(gpui::black())
                            .border_2()
                            .border_dashed()
                            .border_color(gpui::white())
                            .child("border full and no rounded"),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Example {}),
        )
        .unwrap();
        cx.activate(true);
    });
}
