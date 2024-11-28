use gpui::{
    canvas, div, linear_color_stop, linear_gradient, point, prelude::*, px, size, App, AppContext,
    Bounds, Half, Hsla, Render, ViewContext, WindowOptions,
};

const COLORS: [(Hsla, Hsla); 12] = [
    (gpui::red(), gpui::blue()),
    (gpui::red(), gpui::green()),
    (gpui::red(), gpui::yellow()),
    (gpui::blue(), gpui::red()),
    (gpui::blue(), gpui::green()),
    (gpui::blue(), gpui::yellow()),
    (gpui::green(), gpui::red()),
    (gpui::green(), gpui::blue()),
    (gpui::green(), gpui::yellow()),
    (gpui::yellow(), gpui::red()),
    (gpui::yellow(), gpui::blue()),
    (gpui::yellow(), gpui::green()),
];

struct GradientViewer {
    color_ix: usize,
}

impl GradientViewer {
    fn new() -> Self {
        Self { color_ix: 0 }
    }
}

impl Render for GradientViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let (color0, color1) = COLORS[self.color_ix];

        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_between()
                    .items_center()
                    .child("Gradient Examples")
                    .child(
                        div()
                            .id("next")
                            .flex()
                            .px_3()
                            .py_1()
                            .text_sm()
                            .bg(gpui::black())
                            .text_color(gpui::white())
                            .child("Switch Color")
                            .active(|this| this.opacity(0.8))
                            .on_click(cx.listener(move |this, _, cx| {
                                this.color_ix = (this.color_ix + 1) % COLORS.len();
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .child(
                        div()
                            .size_full()
                            .rounded_xl()
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(color0)
                            .text_color(gpui::white())
                            .child("Solid Color"),
                    )
                    .child(
                        div()
                            .size_full()
                            .rounded_xl()
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(color1)
                            .text_color(gpui::white())
                            .child("Solid Color"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .h_24()
                    .text_color(gpui::white())
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        45.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        135.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        225.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        315.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    ))),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .h_24()
                    .text_color(gpui::white())
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        0.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        90.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        180.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        360.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 1.),
                    ))),
            )
            .child(div().flex_1().rounded_xl().bg(linear_gradient(
                0.,
                linear_color_stop(color0, 0.05),
                linear_color_stop(color1, 0.95),
            )))
            .child(div().flex_1().rounded_xl().bg(linear_gradient(
                90.,
                linear_color_stop(color0, 0.05),
                linear_color_stop(color1, 0.95),
            )))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .gap_3()
                            .child(div().flex_1().rounded_xl().bg(linear_gradient(
                                90.,
                                linear_color_stop(color0, 0.5),
                                linear_color_stop(color1, 0.5),
                            ))),
                    )
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        180.,
                        linear_color_stop(color0, 0.),
                        linear_color_stop(color1, 0.5),
                    ))),
            )
            .child(div().h_24().child(canvas(
                move |_, _| {},
                move |bounds, _, cx| {
                    let size = size(bounds.size.width * 0.8, px(80.));
                    let square_bounds = Bounds {
                        origin: point(
                            bounds.size.width.half() - size.width.half(),
                            bounds.origin.y,
                        ),
                        size,
                    };
                    let height = square_bounds.size.height;
                    let horizontal_offset = height;
                    let vertical_offset = px(30.);
                    let mut path = gpui::Path::new(square_bounds.lower_left());
                    path.line_to(square_bounds.origin + point(horizontal_offset, vertical_offset));
                    path.line_to(
                        square_bounds.upper_right() + point(-horizontal_offset, vertical_offset),
                    );
                    path.line_to(square_bounds.lower_right());
                    path.line_to(square_bounds.lower_left());
                    cx.paint_path(
                        path,
                        linear_gradient(
                            180.,
                            linear_color_stop(color0, 0.),
                            linear_color_stop(color1, 1.),
                        ),
                    );
                },
            )))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |cx| cx.new_view(|_| GradientViewer::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}
