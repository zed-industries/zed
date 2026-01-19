use gpui::{
    AnchoredPositionMode, App, Application, Axis, Bounds, Context, Corner, Half as _,
    InteractiveElement, ParentElement, Pixels, Point, Render, SharedString, Size, Window,
    WindowBounds, WindowOptions, anchored, deferred, div, point, prelude::*, px, rgb, size,
};

struct PopoverDemo {
    hovered_button: Option<usize>,
}

struct ButtonDemo {
    label: SharedString,
    corner: Option<Corner>,
}

fn resolved_position(corner: Corner, button_size: Size<Pixels>) -> Point<Pixels> {
    let offset = Point {
        x: px(0.),
        y: -button_size.height,
    };

    offset
        + match corner.other_side_corner_along(Axis::Vertical) {
            Corner::TopLeft => point(px(0.0), px(0.0)),
            Corner::TopCenter => point(button_size.width.half(), px(0.0)),
            Corner::TopRight => point(button_size.width, px(0.0)),
            Corner::LeftCenter => point(button_size.width, button_size.height.half()),
            Corner::RightCenter => point(px(0.), button_size.height.half()),
            Corner::BottomLeft => point(px(0.0), button_size.height),
            Corner::BottomCenter => point(button_size.width / 2.0, button_size.height),
            Corner::BottomRight => point(button_size.width, button_size.height),
        }
}

impl PopoverDemo {
    fn new() -> Self {
        Self {
            hovered_button: None,
        }
    }

    fn buttons() -> Vec<ButtonDemo> {
        vec![
            ButtonDemo {
                label: "TopLeft".into(),
                corner: Some(Corner::TopLeft),
            },
            ButtonDemo {
                label: "TopCenter".into(),
                corner: Some(Corner::TopCenter),
            },
            ButtonDemo {
                label: "TopRight".into(),
                corner: Some(Corner::TopRight),
            },
            ButtonDemo {
                label: "LeftCenter".into(),
                corner: Some(Corner::LeftCenter),
            },
            ButtonDemo {
                label: "Center".into(),
                corner: None,
            },
            ButtonDemo {
                label: "RightCenter".into(),
                corner: Some(Corner::RightCenter),
            },
            ButtonDemo {
                label: "BottomLeft".into(),
                corner: Some(Corner::BottomLeft),
            },
            ButtonDemo {
                label: "BottomCenter".into(),
                corner: Some(Corner::BottomCenter),
            },
            ButtonDemo {
                label: "BottomRight".into(),
                corner: Some(Corner::BottomRight),
            },
        ]
    }
}

impl Render for PopoverDemo {
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
                                                    } else {
                                                        if this.hovered_button == Some(index) {
                                                            this.hovered_button = None;
                                                        }
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

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(750.), px(600.)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| {
                cx.activate(true);
                cx.new(|_cx| PopoverDemo::new())
            },
        )
        .unwrap();
    });
}
