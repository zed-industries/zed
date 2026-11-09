//! Native iOS smoke application for GPUI.
//!
//! This uses the same platform-neutral application entry point as desktop GPUI apps.

#[cfg(target_os = "ios")]
use gpui::{Context, Window, WindowOptions, div, prelude::*, px, rgb};

#[cfg(target_os = "ios")]
struct IosExample {
    tap_count: usize,
}

#[cfg(target_os = "ios")]
impl Render for IosExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .bg(rgb(0x101622))
            .text_color(rgb(0xf5f7ff))
            .child(div().text_3xl().child("GPUI on iOS"))
            .child(
                div()
                    .id("tap-counter")
                    .px_6()
                    .py_3()
                    .rounded_lg()
                    .bg(rgb(0x246bfd))
                    .text_xl()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.tap_count += 1;
                        cx.notify();
                    }))
                    .child(format!("Taps: {}", self.tap_count)),
            )
            .child(
                div()
                    .max_w(px(320.))
                    .text_center()
                    .child("Rust app, native UIKit loop, GPUI rendering"),
            )
    }
}

#[cfg(target_os = "ios")]
fn main() {
    gpui_platform::application().run(|cx| {
        gpui_ios::ios::set_status_bar_style(gpui_ios::StatusBarContentStyle::Light);
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| IosExample { tap_count: 0 })
        })
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_os = "ios"))]
fn main() {
    eprintln!("Build this example for an iOS target and package it with build-simulator.sh.");
}
