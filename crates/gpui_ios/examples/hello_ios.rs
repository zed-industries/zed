//! Hello-world example for GPUI on iOS: styled quads, borders, corner radii,
//! shadows, and Core Text rendered text, all drawn by the Metal renderer.

#[cfg(target_os = "ios")]
mod example {
    use gpui::{App, Application, Context, Window, WindowOptions, div, prelude::*, px, rgb};
    use std::rc::Rc;

    struct HelloIos;

    impl Render for HelloIos {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .bg(rgb(0x1f2937))
                .flex()
                .flex_col()
                .gap_8()
                .justify_center()
                .items_center()
                .font_family("Helvetica")
                .text_color(rgb(0xffffff))
                .text_2xl()
                .child("Hello, iOS!")
                .child(
                    div()
                        .flex()
                        .gap_4()
                        .child(
                            div()
                                .size_16()
                                .bg(rgb(0xef4444))
                                .rounded_md()
                                .border_2()
                                .border_dashed()
                                .border_color(rgb(0xfbbf24)),
                        )
                        .child(
                            div()
                                .size_16()
                                .bg(rgb(0x22c55e))
                                .rounded_full()
                                .border_2()
                                .border_color(rgb(0xffffff)),
                        )
                        .child(
                            div()
                                .size_16()
                                .bg(rgb(0x3b82f6))
                                .rounded_lg()
                                .border_4()
                                .border_color(rgb(0x1e3a8a)),
                        ),
                )
                .child(
                    div()
                        .w(px(280.))
                        .p_6()
                        .bg(gpui::white())
                        .rounded_xl()
                        .shadow_lg()
                        .text_lg()
                        .text_color(rgb(0x111827))
                        .child("Rendered by GPUI on Metal"),
                )
        }
    }

    pub fn run() {
        Application::with_platform(Rc::new(gpui_ios::IosPlatform::new())).run(|cx: &mut App| {
            cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| HelloIos))
                .unwrap();
        });
    }
}

#[cfg(target_os = "ios")]
fn main() {
    example::run();
}

#[cfg(not(target_os = "ios"))]
fn main() {}
