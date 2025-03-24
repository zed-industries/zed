use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .w(px(1900.0))
            .h(px(1000.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .size_64()
                            .bg(gpui::red())
                            .border_2()
                            .border_dashed()
                            // .rounded_lg()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_64()
                            .bg(gpui::green())
                            .border_20()
                            .border_r_1()
                            .border_dashed()
                            .rounded_full()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_64()
                            .bg(gpui::blue())
                            .border_t_16()
                            .border_r_3()
                            .border_l_1()
                            .border_dashed()
                            .rounded(px(100.0))
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_64()
                            .bg(gpui::blue())
                            .border_t_5()
                            .border_r_7()
                            .border_l_2()
                            .border_dashed()
                            .rounded(px(100.0))
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_64()
                            .bg(gpui::red())
                            .border_b_3()
                            .border_l_10()
                            .border_t_16()
                            .border_r_1()
                            .rounded(px(100.0))
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_64()
                            .bg(gpui::black())
                            .border_2()
                            .border_dashed()
                            .rounded_tl_xl()
                            .rounded_tr_2xl()
                            .rounded_br_3xl()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_24()
                            .bg(gpui::white())
                            .border_8()
                            .border_dashed()
                            .rounded(px(60.0))
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
