#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, Hsla, Window, WindowBounds, WindowOptions, container_query, div,
    prelude::*, px, rgb, size,
};
use gpui_platform::application;

// https://en.wikipedia.org/wiki/Holy_grail_(web_design)
//
// Resize the window: the layout is chosen by `container_query` based on the
// measured size of the container, collapsing to a single stacked column when
// it becomes too narrow for the three-column grid.
struct HolyGrailExample {}

impl Render for HolyGrailExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        container_query(|container_size, _window, _cx| {
            let block = |color: Hsla| {
                div()
                    .size_full()
                    .bg(color)
                    .border_1()
                    .border_dashed()
                    .rounded_md()
                    .border_color(gpui::white())
                    .items_center()
            };

            let header = block(gpui::white()).child(format!("Header — {}", container_size.width));
            let table_of_contents = block(gpui::red()).child("Table of contents");
            let content = block(gpui::green()).child("Content");
            let ad = block(gpui::blue()).child("AD :(").text_color(gpui::white());
            let footer = block(gpui::black())
                .text_color(gpui::white())
                .child("Footer");

            let container = div().gap_1().bg(rgb(0x505050)).shadow_lg().size_full();

            if container_size.width < px(400.) {
                container
                    .flex()
                    .flex_col()
                    .child(header.h_12().flex_none())
                    .child(table_of_contents.h_20().flex_none())
                    .child(content.flex_1())
                    .child(ad.h_20().flex_none())
                    .child(footer.h_12().flex_none())
            } else {
                container
                    .grid()
                    .grid_cols(5)
                    .grid_rows(5)
                    .child(header.row_span(1).col_span_full())
                    .child(table_of_contents.col_span(1).h_56())
                    .child(content.col_span(3).row_span(3))
                    .child(ad.col_span(1).row_span(3))
                    .child(footer.row_span(1).col_span_full())
            }
        })
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| HolyGrailExample {}),
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
