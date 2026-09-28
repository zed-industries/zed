#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use gpui::{
    App, Bounds, Context, ScrollHandle, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};
use gpui_platform::application;

struct NestedScrollDemo {
    outer_scroll: ScrollHandle,
    inner_scroll: ScrollHandle,
    outer_events: usize,
    inner_events: usize,
    horizontal_outer_scroll: ScrollHandle,
    horizontal_inner_scroll: ScrollHandle,
    horizontal_outer_events: usize,
    horizontal_inner_events: usize,
}

impl Render for NestedScrollDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let outer_offset = self.outer_scroll.offset().y;
        let inner_offset = self.inner_scroll.offset().y;
        let horizontal_outer_offset = self.horizontal_outer_scroll.offset().x;
        let horizontal_inner_offset = self.horizontal_inner_scroll.offset().x;

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0xf4f6f8))
            .text_color(rgb(0x17202a))
            .child(
                div()
                    .flex_none()
                    .p_4()
                    .bg(rgb(0xffffff))
                    .border_b_1()
                    .border_color(rgb(0xd5d8dc))
                    .child("GPUI native nested vertical scroll probe")
                    .child(
                        div()
                            .mt_2()
                            .text_sm()
                            .child(format!(
                                "outer offset: {outer_offset}  |  inner offset: {inner_offset}"
                            )),
                    )
                    .child(
                        div().mt_1().text_sm().child(format!(
                            "raw wheel callbacks (bubble): outer {}  |  inner {}",
                            self.outer_events, self.inner_events
                        )),
                    )
                    .child(
                        div().mt_2().text_sm().child(format!(
                            "horizontal outer offset: {horizontal_outer_offset}  |  horizontal inner offset: {horizontal_inner_offset}"
                        )),
                    )
                    .child(
                        div().mt_1().text_sm().child(format!(
                            "horizontal raw callbacks (bubble): outer {}  |  inner {}",
                            self.horizontal_outer_events, self.horizontal_inner_events
                        )),
                    )
                    .child(
                        div().mt_2().text_sm().text_color(rgb(0x566573)).child(
                            "Blue: vertical nested scroll. Purple: horizontal nested scroll (two-finger horizontal swipe). Raw callbacks bubble, but only one offset per axis should change; the outer offset takes over only after the inner reaches an edge.",
                        ),
                    ),
            )
            .child(
                div()
                    .id("outer-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&self.outer_scroll)
                    .on_scroll_wheel(cx.listener(|this, _, _, cx| {
                        this.outer_events += 1;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .w_full()
                            .h(px(1500.))
                            .p_6()
                            .child(
                                div()
                                    .h(px(320.))
                                    .rounded_lg()
                                    .bg(rgb(0xe8f8f5))
                                    .border_1()
                                    .border_color(rgb(0x73c6b6))
                                    .p_4()
                                    .child("Outer content before the nested scroller"),
                            )
                            .child(
                                div()
                                    .id("inner-scroll")
                                    .mt_6()
                                    .h(px(300.))
                                    .overflow_y_scroll()
                                    .track_scroll(&self.inner_scroll)
                                    .rounded_lg()
                                    .border_2()
                                    .border_color(rgb(0x2e86c1))
                                    .bg(rgb(0xeaf2f8))
                                    .on_scroll_wheel(cx.listener(|this, _, _, cx| {
                                        this.inner_events += 1;
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .w_full()
                                            .h(px(1100.))
                                            .p_4()
                                            .children((0..22).map(|index| {
                                                div()
                                                    .h(px(44.))
                                                    .mb_2()
                                                    .px_3()
                                                    .flex()
                                                    .items_center()
                                                    .rounded_md()
                                                    .bg(if index % 2 == 0 {
                                                        rgb(0xd6eaf8)
                                                    } else {
                                                        rgb(0xaed6f1)
                                                    })
                                                    .child(format!("Inner scroll row {}", index + 1))
                                            })),
                                    ),
                            )
                            .child(
                                div()
                                    .mt_6()
                                    .h(px(700.))
                                    .rounded_lg()
                                    .bg(rgb(0xfef9e7))
                                    .border_1()
                                    .border_color(rgb(0xf4d03f))
                                    .p_4()
                                    .child("Outer content after the nested scroller"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex_none()
                    .h(px(280.))
                    .p_4()
                    .bg(rgb(0xffffff))
                    .border_t_1()
                    .border_color(rgb(0xd5d8dc))
                    .child(
                        div()
                            .mb_2()
                            .text_sm()
                            .child("Horizontal nested scroll probe"),
                    )
                    .child(
                        div()
                            .id("horizontal-outer-scroll")
                            .w_full()
                            .h(px(220.))
                            .overflow_x_scroll()
                            .track_scroll(&self.horizontal_outer_scroll)
                            .border_2()
                            .border_color(rgb(0x8e44ad))
                            .bg(rgb(0xf5eef8))
                            .on_scroll_wheel(cx.listener(|this, _, _, cx| {
                                this.horizontal_outer_events += 1;
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .w(px(1800.))
                                    .h_full()
                                    .flex_none()
                                    .flex()
                                    .items_center()
                                    .gap_4()
                                    .px_6()
                                    .child(
                                        div()
                                            .w(px(80.))
                                            .flex_none()
                                            .child("Outer start"),
                                    )
                                    .child(
                                        div()
                                            .id("horizontal-inner-scroll")
                                            .w(px(520.))
                                            .h(px(160.))
                                            .flex_none()
                                            .overflow_x_scroll()
                                            .track_scroll(&self.horizontal_inner_scroll)
                                            .rounded_lg()
                                            .border_2()
                                            .border_color(rgb(0x6c3483))
                                            .bg(rgb(0xebdef0))
                                            .on_scroll_wheel(cx.listener(|this, _, _, cx| {
                                                this.horizontal_inner_events += 1;
                                                cx.notify();
                                            }))
                                            .child(
                                                div()
                                                    .w(px(1320.))
                                                    .h_full()
                                                    .flex_none()
                                                    .flex()
                                                    .items_center()
                                                    .gap_3()
                                                    .p_4()
                                                    .children((0..12).map(|index| {
                                                        div()
                                                            .w(px(92.))
                                                            .h(px(100.))
                                                            .flex_none()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .rounded_md()
                                                            .bg(if index % 2 == 0 {
                                                                rgb(0xd2b4de)
                                                            } else {
                                                                rgb(0xbb8fce)
                                                            })
                                                            .child(format!("H{}", index + 1))
                                                    })),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .w(px(900.))
                                            .flex_none()
                                            .child("Outer content after the nested scroller"),
                                    ),
                            ),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        if !example_support::load_fonts(cx) {
            return;
        }

        let bounds = Bounds::centered(None, size(px(900.), px(900.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| NestedScrollDemo {
                    outer_scroll: ScrollHandle::new(),
                    inner_scroll: ScrollHandle::new(),
                    outer_events: 0,
                    inner_events: 0,
                    horizontal_outer_scroll: ScrollHandle::new(),
                    horizontal_inner_scroll: ScrollHandle::new(),
                    horizontal_outer_events: 0,
                    horizontal_inner_events: 0,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
