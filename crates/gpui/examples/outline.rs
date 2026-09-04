#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, Div, SharedString, Window, WindowBounds, WindowOptions, div, hsla,
    prelude::*, px, relative, rgb, size,
};
use gpui_platform::application;

struct OutlineExample {}

impl OutlineExample {
    /// A square swatch with a small border, used as the base for outline demos.
    fn square() -> Div {
        div()
            .size_16()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.0, 0.1))
    }

    /// A rounded swatch, to show the outline tracking corner radii.
    fn rounded() -> Div {
        div()
            .size_16()
            .bg(rgb(0xffffff))
            .rounded(px(8.))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.0, 0.1))
    }
}

fn example(label: impl Into<SharedString>, example: impl IntoElement) -> impl IntoElement {
    let label = label.into();

    div()
        .flex()
        .flex_col()
        .justify_center()
        .items_center()
        .w(relative(1. / 6.))
        .border_r_1()
        .border_color(hsla(0.0, 0.0, 0.0, 1.0))
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .flex_1()
                .py_12()
                .child(example),
        )
        .child(
            div()
                .w_full()
                .border_t_1()
                .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                .p_1()
                .flex()
                .items_center()
                .child(label),
        )
}

fn row(children: Vec<impl IntoElement + 'static>) -> Div {
    div()
        .border_b_1()
        .border_color(hsla(0.0, 0.0, 0.0, 1.0))
        .flex()
        .w_full()
        .children(children)
}

impl Render for OutlineExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let outline = gpui::blue();

        div()
            .id("outline-example")
            .overflow_y_scroll()
            .bg(rgb(0xffffff))
            .size_full()
            .text_xs()
            .child(
                div().flex().flex_col().w_full().children(vec![
                    // Increasing outline widths.
                    row(vec![
                        example(
                            "Width 0",
                            OutlineExample::square().outline_0().outline_color(outline),
                        ),
                        example(
                            "Width 1",
                            OutlineExample::square().outline_1().outline_color(outline),
                        ),
                        example(
                            "Width 2",
                            OutlineExample::square().outline_2().outline_color(outline),
                        ),
                        example(
                            "Width 4",
                            OutlineExample::square().outline_4().outline_color(outline),
                        ),
                        example(
                            "Width 8",
                            OutlineExample::square().outline_8().outline_color(outline),
                        ),
                    ]),
                    // Increasing offsets (outline floats further from the box).
                    row(vec![
                        example(
                            "Offset 0",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_0(),
                        ),
                        example(
                            "Offset 2",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_2(),
                        ),
                        example(
                            "Offset 4",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_4(),
                        ),
                        example(
                            "Offset 8",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_8(),
                        ),
                        example(
                            "Offset -4",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset(px(-4.)),
                        ),
                    ]),
                    // Styles: solid (default), dashed, and none.
                    row(vec![
                        example(
                            "Solid",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_2(),
                        ),
                        example(
                            "Dashed",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_2()
                                .outline_dashed(),
                        ),
                        example(
                            "None",
                            OutlineExample::square()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_2()
                                .outline_0(),
                        ),
                    ]),
                    // Rounded corners: the outline grows its radius to stay parallel.
                    row(vec![
                        example(
                            "Rounded 0",
                            OutlineExample::rounded()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_0(),
                        ),
                        example(
                            "Rounded +2",
                            OutlineExample::rounded()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_2(),
                        ),
                        example(
                            "Rounded +8",
                            OutlineExample::rounded()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_8(),
                        ),
                        example(
                            "Rounded dashed",
                            OutlineExample::rounded()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset_4()
                                .outline_dashed(),
                        ),
                        // A negative offset larger than the corner radius clamps the
                        // outline radius to 0, so the corners render square.
                        example(
                            "Rounded -12",
                            OutlineExample::rounded()
                                .outline_2()
                                .outline_color(outline)
                                .outline_offset(px(-12.)),
                        ),
                    ]),
                ]),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| OutlineExample {}),
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
