use gpui::{
    canvas, div, linear_color_stop, linear_gradient, point, prelude::*, px, size, App, AppContext,
    Bounds, Half, Render, ViewContext, WindowOptions,
};
struct GradientViewer {}

impl GradientViewer {
    fn new() -> Self {
        Self {}
    }
}

impl Render for GradientViewer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                    .flex_1()
                    .gap_3()
                    .child(
                        div()
                            .size_full()
                            .rounded_xl()
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(gpui::blue())
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
                            .bg(gpui::red())
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
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        45.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        135.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        225.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        315.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    ))),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .h_24()
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        0.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        90.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        180.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        360.,
                        linear_color_stop(gpui::red(), 0.),
                        linear_color_stop(gpui::blue(), 1.),
                    ))),
            )
            .child(div().flex_1().rounded_xl().bg(linear_gradient(
                0.,
                linear_color_stop(gpui::green(), 0.05),
                linear_color_stop(gpui::yellow(), 0.95),
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
                                linear_color_stop(gpui::blue(), 0.5),
                                linear_color_stop(gpui::red(), 0.5),
                            ))),
                    )
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        180.,
                        linear_color_stop(gpui::green(), 0.),
                        linear_color_stop(gpui::yellow(), 0.5),
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
                            linear_color_stop(gpui::red(), 0.),
                            linear_color_stop(gpui::blue(), 1.),
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
