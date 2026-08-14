//! Native iOS smoke application for GPUI.
//!
//! This static library supplies a small GPUI view to the Objective-C simulator
//! host so the iOS platform can be exercised independently of a product app.

#![cfg(target_os = "ios")]

use gpui::{App, Context, Window, WindowOptions, div, prelude::*, px, rgb};
use std::{cell::Cell, rc::Rc};

struct IosExample {
    tap_count: usize,
}

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
                    .child("UIKit host, direct Metal renderer, GPUI touch events"),
            )
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_example_run() -> bool {
    let did_open_window = Rc::new(Cell::new(false));
    gpui_ios::ios::ffi::set_app_callback(Box::new({
        let did_open_window = did_open_window.clone();
        move |cx: &mut App| match cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| IosExample { tap_count: 0 })
        }) {
            Ok(_) => {
                did_open_window.set(true);
                cx.activate(true);
            }
            Err(error) => log::error!("failed to open GPUI iOS example window: {error:#}"),
        }
    }));
    gpui_ios::ios::ffi::run_app();
    did_open_window.get()
}
