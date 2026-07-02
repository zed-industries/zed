//! Touch-input demo for GPUI on iOS: tap counters, move tracking, and hover
//! lifecycle, all driven by the touch → pointer compatibility shim and drawn
//! by the Metal renderer.

#[cfg(target_os = "ios")]
mod example {
    use gpui::{
        App, Application, ClickEvent, Context, MouseMoveEvent, Pixels, Point, Rgba, Window,
        WindowOptions, div, prelude::*, px, rgb,
    };
    use std::rc::Rc;

    struct HelloIos {
        tap_counts: [usize; 3],
        last_touch: Option<Point<Pixels>>,
    }

    impl HelloIos {
        fn tap_counter_box(
            &self,
            index: usize,
            background: Rgba,
            hover_background: Rgba,
            cx: &mut Context<Self>,
        ) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .id(index)
                        .size_16()
                        .bg(background)
                        .rounded_md()
                        .border_2()
                        .border_color(rgb(0xffffff))
                        .hover(|style| {
                            style
                                .bg(hover_background)
                                .border_4()
                                .border_color(rgb(0xfbbf24))
                        })
                        .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                            this.tap_counts[index] += 1;
                            cx.notify();
                        })),
                )
                .child(self.tap_counts[index].to_string())
        }
    }

    impl Render for HelloIos {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let last_touch = self.last_touch.map_or_else(
                || "Last touch: none".to_string(),
                |position| {
                    format!(
                        "Last touch: ({:.0}, {:.0})",
                        f32::from(position.x),
                        f32::from(position.y)
                    )
                },
            );

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
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                    // The platform parks the pointer just off-window after a
                    // touch ends to clear hover; don't display that position.
                    if event.position.x >= px(0.) {
                        this.last_touch = Some(event.position);
                        cx.notify();
                    }
                }))
                .child("Hello, iOS!")
                .child(div().text_lg().child(last_touch))
                .child(
                    div()
                        .flex()
                        .gap_4()
                        .child(self.tap_counter_box(0, rgb(0xef4444), rgb(0xf87171), cx))
                        .child(self.tap_counter_box(1, rgb(0x22c55e), rgb(0x4ade80), cx))
                        .child(self.tap_counter_box(2, rgb(0x3b82f6), rgb(0x60a5fa), cx)),
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
            cx.open_window(WindowOptions::default(), |_, cx| {
                cx.new(|_| HelloIos {
                    tap_counts: [0; 3],
                    last_touch: None,
                })
            })
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
