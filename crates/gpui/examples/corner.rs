#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    AnchoredPositionMode, App, Axis, Bounds, BoxAnchor, Context, Half as _, InteractiveElement,
    ParentElement, Pixels, Point, Render, SharedString, Size, Window, WindowBounds, WindowOptions,
    anchored, deferred, div, point, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct CornerDemo {
    hovered_button: Option<usize>,
}

struct ButtonDemo {
    label: SharedString,
    corner: Option<BoxAnchor>,
}

fn resolved_position(corner: BoxAnchor, button_size: Size<Pixels>) -> Point<Pixels> {
    let offset = Point {
        x: px(0.),
        y: -button_size.height,
    };

    offset
        + match corner.other_side_along(Axis::Vertical) {
            BoxAnchor::TopLeft => point(px(0.0), px(0.0)),
            BoxAnchor::TopCenter => point(button_size.width.half(), px(0.0)),
            BoxAnchor::TopRight => point(button_size.width, px(0.0)),
            BoxAnchor::LeftCenter => point(button_size.width, button_size.height.half()),
            BoxAnchor::RightCenter => point(px(0.), button_size.height.half()),
            BoxAnchor::BottomLeft => point(px(0.0), button_size.height),
            BoxAnchor::BottomCenter => point(button_size.width / 2.0, button_size.height),
            BoxAnchor::BottomRight => point(button_size.width, button_size.height),
        }
}

impl CornerDemo {
    fn buttons() -> Vec<ButtonDemo> {
        vec![
            ButtonDemo {
                label: "TopLeft".into(),
                corner: Some(BoxAnchor::TopLeft),
            },
            ButtonDemo {
                label: "TopCenter".into(),
                corner: Some(BoxAnchor::TopCenter),
            },
            ButtonDemo {
                label: "TopRight".into(),
                corner: Some(BoxAnchor::TopRight),
            },
            ButtonDemo {
                label: "LeftCenter".into(),
                corner: Some(BoxAnchor::LeftCenter),
            },
            ButtonDemo {
                label: "Center".into(),
                corner: None,
            },
            ButtonDemo {
                label: "RightCenter".into(),
                corner: Some(BoxAnchor::RightCenter),
            },
            ButtonDemo {
                label: "BottomLeft".into(),
                corner: Some(BoxAnchor::BottomLeft),
            },
            ButtonDemo {
                label: "BottomCenter".into(),
                corner: Some(BoxAnchor::BottomCenter),
            },
            ButtonDemo {
                label: "BottomRight".into(),
                corner: Some(BoxAnchor::BottomRight),
            },
        ]
    }
}

impl Render for CornerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let buttons = Self::buttons();
        let button_size = size(px(120.0), px(65.0));

        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .bg(gpui::white())
            .gap_4()
            .p_10()
            .child("Anchored Popover")
            .child(
                div()
                    .size_128()
                    .grid()
                    .grid_cols(3)
                    .gap_6()
                    .relative()
                    .children(buttons.iter().enumerate().map(|(index, button)| {
                        let is_hovered = self.hovered_button == Some(index);
                        let is_hoverable = button.corner.is_some();
                        div()
                            .relative()
                            .child(
                                div()
                                    .id(("button", index))
                                    .w(button_size.width)
                                    .h(button_size.height)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(gpui::white())
                                    .when(is_hoverable, |this| {
                                        this.border_1()
                                            .rounded_lg()
                                            .border_color(gpui::black())
                                            .hover(|style| {
                                                style.bg(gpui::black()).text_color(gpui::white())
                                            })
                                            .on_hover(cx.listener(
                                                move |this, hovered, _window, cx| {
                                                    if *hovered {
                                                        this.hovered_button = Some(index);
                                                    } else if this.hovered_button == Some(index) {
                                                        this.hovered_button = None;
                                                    }
                                                    cx.notify();
                                                },
                                            ))
                                            .child(button.label.clone())
                                    }),
                            )
                            .when_some(self.hovered_button.filter(|_| is_hovered), |this, index| {
                                let button = &buttons[index];
                                let Some(corner) = button.corner else {
                                    return this;
                                };

                                let position = resolved_position(corner, button_size);
                                this.child(deferred(
                                    anchored()
                                        .anchor(corner)
                                        .position(position)
                                        .position_mode(AnchoredPositionMode::Local)
                                        .snap_to_window()
                                        .child(
                                            div()
                                                .py_0p5()
                                                .px_2()
                                                .bg(gpui::black().opacity(0.75))
                                                .text_color(rgb(0xffffff))
                                                .rounded_sm()
                                                .shadow_sm()
                                                .min_w(px(100.0))
                                                .text_sm()
                                                .child(button.label.clone()),
                                        ),
                                ))
                            })
                    })),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(750.), px(600.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| CornerDemo {
                    hovered_button: None,
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
