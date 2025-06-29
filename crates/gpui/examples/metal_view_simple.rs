use gpui::{prelude::*, *};

struct MetalViewSimpleExample {
    show_metal_view: bool,
}

impl MetalViewSimpleExample {
    fn new() -> Self {
        Self {
            show_metal_view: true,
        }
    }
}

impl Render for MetalViewSimpleExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .p_8()
            .gap_4()
            .child(
                div()
                    .child("MetalView Simple Example")
                    .text_2xl()
                    .text_color(rgb(0xffffff)),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .id("toggle-button")
                            .px_3()
                            .py_1()
                            .bg(rgb(0x3b82f6))
                            .hover(|style| style.bg(rgb(0x2563eb)))
                            .rounded_md()
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.show_metal_view = !this.show_metal_view;
                                cx.notify();
                            }))
                            .child(div().child("Toggle MetalView").text_color(rgb(0xffffff))),
                    )
                    .child(
                        div()
                            .child(format!(
                                "MetalView is: {}",
                                if self.show_metal_view {
                                    "visible"
                                } else {
                                    "hidden"
                                }
                            ))
                            .text_color(rgb(0xaaaaaa)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .bg(rgb(0x2a2a2a))
                    .rounded_md()
                    .child(
                        div()
                            .child("Container with MetalView")
                            .text_lg()
                            .text_color(rgb(0xffffff)),
                    )
                    .when(self.show_metal_view, |parent| {
                        parent.child(
                            div()
                                .border_2()
                                .border_color(rgb(0x444444))
                                .rounded_md()
                                .overflow_hidden()
                                .child(
                                    #[cfg(target_os = "macos")]
                                    {
                                        metal_view()
                                            .w_full()
                                            .h(px(200.0))
                                            .bg(rgb(0x1a1a1a))
                                            .render_with(
                                                |_encoder, _target, _bounds, _scale_factor| {
                                                    // This callback would contain custom Metal rendering code
                                                    // For now, it's just a placeholder
                                                },
                                            )
                                    },
                                    #[cfg(not(target_os = "macos"))]
                                    {
                                        div()
                                            .w_full()
                                            .h(px(200.0))
                                            .bg(rgb(0x1a1a1a))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(
                                                div()
                                                    .child("MetalView (macOS only)")
                                                    .text_color(rgb(0x666666)),
                                            )
                                    },
                                ),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div().flex_1().p_3().bg(rgb(0x333333)).rounded_md().child(
                                    div()
                                        .child("Regular GPUI content")
                                        .text_sm()
                                        .text_color(rgb(0xcccccc)),
                                ),
                            )
                            .child(
                                div().flex_1().p_3().bg(rgb(0x333333)).rounded_md().child(
                                    div()
                                        .child("Can be mixed with MetalView")
                                        .text_sm()
                                        .text_color(rgb(0xcccccc)),
                                ),
                            ),
                    ),
            )
            .child(
                div().mt_4().p_4().bg(rgb(0x2a2a2a)).rounded_md().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .child("Notes:")
                                .text_base()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0xffffff)),
                        )
                        .child(
                            div()
                                .child("• MetalView integrates with GPUI's layout system")
                                .text_sm()
                                .text_color(rgb(0xaaaaaa)),
                        )
                        .child(
                            div()
                                .child("• It can be styled with the same methods as other elements")
                                .text_sm()
                                .text_color(rgb(0xaaaaaa)),
                        )
                        .child(
                            div()
                                .child("• On macOS, it would render custom Metal content")
                                .text_sm()
                                .text_color(rgb(0xaaaaaa)),
                        )
                        .child(
                            div()
                                .child("• On other platforms, a fallback can be provided")
                                .text_sm()
                                .text_color(rgb(0xaaaaaa)),
                        ),
                ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(600.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("MetalView Simple Example".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| MetalViewSimpleExample::new()),
        );
    });
}
