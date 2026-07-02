//! Touch-input demo for GPUI on iOS: tap counters, move tracking, hover
//! lifecycle, pan-to-scroll with momentum, pinch-to-zoom, and hardware
//! keyboard input, all driven by the touch → pointer compatibility shim and
//! drawn by the Metal renderer.

#[cfg(target_os = "ios")]
mod example {
    use gpui::{
        App, Application, ClickEvent, Context, FocusHandle, KeyDownEvent, Keystroke,
        MouseMoveEvent, PinchEvent, Pixels, Point, Rgba, Window, WindowOptions, div, prelude::*,
        px, rgb,
    };
    use std::rc::Rc;

    struct HelloIos {
        focus_handle: FocusHandle,
        tap_counts: [usize; 3],
        row_taps: usize,
        pinch_scale: f32,
        last_touch: Option<Point<Pixels>>,
        last_keystroke: Option<Keystroke>,
        key_down_count: usize,
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

        fn row_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("row-list")
                .h(px(300.))
                .w(px(320.))
                .rounded_lg()
                .border_2()
                .border_color(rgb(0x6b7280))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .text_lg()
                .children((0..40usize).map(|index| {
                    let background = if index % 2 == 0 {
                        rgb(0x334155)
                    } else {
                        rgb(0x475569)
                    };
                    div()
                        .id(("row", index))
                        .flex_none()
                        .h(px(32.))
                        .px_4()
                        .bg(background)
                        .child(format!("Row {index}"))
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                            this.row_taps += 1;
                            cx.notify();
                        }))
                }))
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

            let last_key = self.last_keystroke.as_ref().map_or_else(
                || "Last key: none".to_string(),
                |keystroke| {
                    // `Keystroke::unparse` emits no platform-modifier prefix
                    // on iOS, so render cmd- ourselves.
                    let command_prefix = if keystroke.modifiers.platform {
                        "cmd-"
                    } else {
                        ""
                    };
                    format!(
                        "Last key: {}{} ({} downs)",
                        command_prefix,
                        keystroke.unparse(),
                        self.key_down_count
                    )
                },
            );

            div()
                .size_full()
                .bg(rgb(0x1f2937))
                .flex()
                .flex_col()
                .gap_4()
                .justify_center()
                .items_center()
                .font_family("Helvetica")
                .text_color(rgb(0xffffff))
                .text_2xl()
                .track_focus(&self.focus_handle)
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                    this.last_keystroke = Some(event.keystroke.clone());
                    this.key_down_count += 1;
                    cx.notify();
                }))
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                    // The platform parks the pointer just off-window after a
                    // touch ends to clear hover; don't display that position.
                    if event.position.x >= px(0.) {
                        this.last_touch = Some(event.position);
                        cx.notify();
                    }
                }))
                .on_pinch(cx.listener(|this, event: &PinchEvent, _, cx| {
                    this.pinch_scale = (this.pinch_scale * (1. + event.delta)).clamp(0.5, 2.5);
                    cx.notify();
                }))
                .child("Hello, iOS!")
                .child(div().text_lg().child(last_touch))
                .child(div().text_lg().child(last_key))
                .child(
                    div()
                        .flex()
                        .gap_4()
                        .child(self.tap_counter_box(0, rgb(0xef4444), rgb(0xf87171), cx))
                        .child(self.tap_counter_box(1, rgb(0x22c55e), rgb(0x4ade80), cx))
                        .child(self.tap_counter_box(2, rgb(0x3b82f6), rgb(0x60a5fa), cx)),
                )
                .child(format!("Row taps: {}", self.row_taps))
                .child(self.row_list(cx))
                .child(
                    div()
                        .w(px(280.))
                        .p_4()
                        .bg(gpui::white())
                        .rounded_xl()
                        .shadow_lg()
                        .flex()
                        .items_center()
                        .gap_4()
                        .text_lg()
                        .text_color(rgb(0x111827))
                        .child(
                            div()
                                .flex_none()
                                .size(px(40. * self.pinch_scale))
                                .rounded_md()
                                .bg(rgb(0xa855f7)),
                        )
                        .child(format!("Pinch scale: {:.2}", self.pinch_scale)),
                )
        }
    }

    pub fn run() {
        Application::with_platform(Rc::new(gpui_ios::IosPlatform::new())).run(|cx: &mut App| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| HelloIos {
                    focus_handle: cx.focus_handle(),
                    tap_counts: [0; 3],
                    row_taps: 0,
                    pinch_scale: 1.,
                    last_touch: None,
                    last_keystroke: None,
                    key_down_count: 0,
                });
                let focus_handle = view.read(cx).focus_handle.clone();
                window.focus(&focus_handle, cx);
                view
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
