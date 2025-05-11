use gpui::{
    App, Application, Bounds, BoxShadow, Context, Div, SharedString, Window, WindowBounds,
    WindowOptions, div, hsla, point, prelude::*, px, relative, rgb, size,
};

use smallvec::smallvec;

struct Shadow {}

impl Shadow {
    fn base() -> Div {
        div()
            .size_16()
            .bg(rgb(0xffffff))
            .rounded_full()
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.0, 0.1))
    }

    fn square() -> Div {
        div()
            .size_16()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.0, 0.1))
    }

    fn rounded_small() -> Div {
        div()
            .size_16()
            .bg(rgb(0xffffff))
            .rounded(px(4.))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.0, 0.1))
    }

    fn rounded_medium() -> Div {
        div()
            .size_16()
            .bg(rgb(0xffffff))
            .rounded(px(8.))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.0, 0.1))
    }

    fn rounded_large() -> Div {
        div()
            .size_16()
            .bg(rgb(0xffffff))
            .rounded(px(12.))
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

impl Render for Shadow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("shadow-example")
            .overflow_y_scroll()
            .bg(rgb(0xffffff))
            .size_full()
            .text_xs()
            .child(div().flex().flex_col().w_full().children(vec![
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .flex_row()
                    .children(vec![
                        example(
                            "Square",
                            Shadow::square()
                                .shadow(smallvec![BoxShadow {
                                    color: hsla(0.0, 0.5, 0.5, 0.3),
                                    offset: point(px(0.), px(8.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(0.),
                                }]),
                        ),
                        example(
                            "Rounded 4",
                            Shadow::rounded_small()
                                .shadow(smallvec![BoxShadow {
                                    color: hsla(0.0, 0.5, 0.5, 0.3),
                                    offset: point(px(0.), px(8.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(0.),
                                }]),
                        ),
                        example(
                            "Rounded 8",
                            Shadow::rounded_medium()
                                .shadow(smallvec![BoxShadow {
                                    color: hsla(0.0, 0.5, 0.5, 0.3),
                                    offset: point(px(0.), px(8.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(0.),
                                }]),
                        ),
                        example(
                            "Rounded 16",
                            Shadow::rounded_large()
                                .shadow(smallvec![BoxShadow {
                                    color: hsla(0.0, 0.5, 0.5, 0.3),
                                    offset: point(px(0.), px(8.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(0.),
                                }]),
                        ),
                        example(
                            "Circle",
                            Shadow::base()
                                .shadow(smallvec![BoxShadow {
                                    color: hsla(0.0, 0.5, 0.5, 0.3),
                                    offset: point(px(0.), px(8.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(0.),
                                }]),
                        ),
                    ]),
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .w_full()
                    .children(vec![
                        example("None", Shadow::base()),
                        // Small shadow
                        example("Small", Shadow::base().shadow_sm()),
                        // Medium shadow
                        example("Medium", Shadow::base().shadow_md()),
                        // Large shadow
                        example("Large", Shadow::base().shadow_lg()),
                        example("Extra Large", Shadow::base().shadow_xl()),
                        example("2X Large", Shadow::base().shadow_2xl()),
                    ]),
                // Horizontal list of increasing blur radii
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Blur 0",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(0.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Blur 2",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(2.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Blur 4",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(4.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Blur 8",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Blur 16",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(16.),
                                spread_radius: px(0.),
                            }]),
                        ),
                    ]),
                // Horizontal list of increasing spread radii
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Spread 0",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Spread 2",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(2.),
                            }]),
                        ),
                        example(
                            "Spread 4",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(4.),
                            }]),
                        ),
                        example(
                            "Spread 8",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(8.),
                            }]),
                        ),
                        example(
                            "Spread 16",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(16.),
                            }]),
                        ),
                    ]),
                // Square spread examples
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Square Spread 0",
                            Shadow::square().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Square Spread 8",
                            Shadow::square().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(8.),
                            }]),
                        ),
                        example(
                            "Square Spread 16",
                            Shadow::square().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(16.),
                            }]),
                        ),
                    ]),
                // Rounded large spread examples
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Rounded Large Spread 0",
                            Shadow::rounded_large().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Rounded Large Spread 8",
                            Shadow::rounded_large().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(8.),
                            }]),
                        ),
                        example(
                            "Rounded Large Spread 16",
                            Shadow::rounded_large().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(16.),
                            }]),
                        ),
                    ]),
                // Directional shadows
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Left",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(-8.), px(0.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Right",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(8.), px(0.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Top",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(0.), px(-8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Bottom",
                            Shadow::base().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                    ]),
                // Square directional shadows
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Square Left",
                            Shadow::square().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(-8.), px(0.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Square Right",
                            Shadow::square().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(8.), px(0.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Square Top",
                            Shadow::square().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(0.), px(-8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Square Bottom",
                            Shadow::square().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                    ]),
                // Rounded large directional shadows
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Rounded Large Left",
                            Shadow::rounded_large().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(-8.), px(0.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Rounded Large Right",
                            Shadow::rounded_large().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(8.), px(0.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Rounded Large Top",
                            Shadow::rounded_large().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(0.), px(-8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                        example(
                            "Rounded Large Bottom",
                            Shadow::rounded_large().shadow(smallvec![BoxShadow {
                                color: hsla(0.0, 0.5, 0.5, 0.3),
                                offset: point(px(0.), px(8.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            }]),
                        ),
                    ]),
                // Multiple shadows for different shapes
                div()
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.0, 1.0))
                    .flex()
                    .children(vec![
                        example(
                            "Circle Multiple",
                            Shadow::base().shadow(smallvec![
                                BoxShadow {
                                    color: hsla(0.0 / 360., 1.0, 0.5, 0.3), // Red
                                    offset: point(px(0.), px(-12.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(60.0 / 360., 1.0, 0.5, 0.3), // Yellow
                                    offset: point(px(12.), px(0.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(120.0 / 360., 1.0, 0.5, 0.3), // Green
                                    offset: point(px(0.), px(12.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(240.0 / 360., 1.0, 0.5, 0.3), // Blue
                                    offset: point(px(-12.), px(0.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                            ]),
                        ),
                        example(
                            "Square Multiple",
                            Shadow::square().shadow(smallvec![
                                BoxShadow {
                                    color: hsla(0.0 / 360., 1.0, 0.5, 0.3), // Red
                                    offset: point(px(0.), px(-12.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(60.0 / 360., 1.0, 0.5, 0.3), // Yellow
                                    offset: point(px(12.), px(0.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(120.0 / 360., 1.0, 0.5, 0.3), // Green
                                    offset: point(px(0.), px(12.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(240.0 / 360., 1.0, 0.5, 0.3), // Blue
                                    offset: point(px(-12.), px(0.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                            ]),
                        ),
                        example(
                            "Rounded Large Multiple",
                            Shadow::rounded_large().shadow(smallvec![
                                BoxShadow {
                                    color: hsla(0.0 / 360., 1.0, 0.5, 0.3), // Red
                                    offset: point(px(0.), px(-12.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(60.0 / 360., 1.0, 0.5, 0.3), // Yellow
                                    offset: point(px(12.), px(0.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(120.0 / 360., 1.0, 0.5, 0.3), // Green
                                    offset: point(px(0.), px(12.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                                BoxShadow {
                                    color: hsla(240.0 / 360., 1.0, 0.5, 0.3), // Blue
                                    offset: point(px(-12.), px(0.)),
                                    blur_radius: px(8.),
                                    spread_radius: px(2.),
                                },
                            ]),
                        ),
                    ]),
            ]))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Shadow {}),
        )
        .unwrap();

        cx.activate(true);
    });
}
