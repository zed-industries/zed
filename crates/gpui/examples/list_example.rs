#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, ListAlignment, ListState, Render, Window, WindowBounds, WindowOptions,
    div, list, prelude::*, px, rgb, size,
};
use gpui_platform::application;

const ITEM_COUNT: usize = 40;
const SCROLLBAR_WIDTH: f32 = 12.;

struct BottomListDemo {
    list_state: ListState,
}

impl BottomListDemo {
    fn new() -> Self {
        Self {
            list_state: ListState::new(ITEM_COUNT, ListAlignment::Bottom, px(500.)).measure_all(),
        }
    }
}

impl Render for BottomListDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let max_offset = self.list_state.max_offset_for_scrollbar().y;
        let current_offset = -self.list_state.scroll_px_offset_for_scrollbar().y;

        let viewport_height = self.list_state.viewport_bounds().size.height;

        let raw_fraction = if max_offset > px(0.) {
            current_offset / max_offset
        } else {
            0.
        };

        let total_height = viewport_height + max_offset;
        let thumb_height = if total_height > px(0.) {
            px(viewport_height.as_f32() * viewport_height.as_f32() / total_height.as_f32())
                .max(px(30.))
        } else {
            px(30.)
        };

        let track_space = viewport_height - thumb_height;
        let thumb_top = track_space * raw_fraction;

        let bug_detected = raw_fraction > 1.0;

        div()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .flex()
            .flex_col()
            .p_4()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(format!(
                        "offset: {:.0} / max: {:.0} | fraction: {:.3}",
                        current_offset.as_f32(),
                        max_offset.as_f32(),
                        raw_fraction,
                    ))
                    .child(
                        div()
                            .text_color(if bug_detected {
                                rgb(0xCC0000)
                            } else {
                                rgb(0x008800)
                            })
                            .child(if bug_detected {
                                format!(
                                    "BUG: fraction is {:.3} (> 1.0) — thumb is off-track!",
                                    raw_fraction
                                )
                            } else {
                                "OK: fraction <= 1.0 — thumb is within track.".to_string()
                            }),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .border_1()
                    .border_color(rgb(0xCCCCCC))
                    .rounded_sm()
                    .child(
                        list(self.list_state.clone(), |index, _window, _cx| {
                            let height = px(30. + (index % 5) as f32 * 10.);
                            div()
                                .h(height)
                                .w_full()
                                .flex()
                                .items_center()
                                .px_3()
                                .border_b_1()
                                .border_color(rgb(0xEEEEEE))
                                .bg(if index % 2 == 0 {
                                    rgb(0xFAFAFA)
                                } else {
                                    rgb(0xFFFFFF)
                                })
                                .text_sm()
                                .child(format!("Item {index}"))
                                .into_any()
                        })
                        .flex_1(),
                    )
                    // Scrollbar track
                    .child(
                        div()
                            .w(px(SCROLLBAR_WIDTH))
                            .h_full()
                            .flex_shrink_0()
                            .bg(rgb(0xE0E0E0))
                            .relative()
                            .child(
                                // Thumb — position is unclamped to expose the bug
                                div()
                                    .absolute()
                                    .top(thumb_top)
                                    .w_full()
                                    .h(thumb_height)
                                    .bg(if bug_detected {
                                        rgb(0xCC0000)
                                    } else {
                                        rgb(0x888888)
                                    })
                                    .rounded_sm(),
                            ),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                focus: true,
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| BottomListDemo::new()),
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
