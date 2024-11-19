use gpui::{
    div, linear_color_stop, linear_gradient, prelude::*, App, AppContext, Render, ViewContext,
    WindowOptions,
};
struct GradientViewer {}

impl GradientViewer {
    fn new() -> Self {
        Self {}
    }
}

impl Render for GradientViewer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let red = gpui::hsla(0., 1., 0.5, 1.);
        let blue = gpui::hsla(240. / 360., 1., 0.5, 1.);
        let green = gpui::hsla(120. / 360., 1., 0.25, 1.);
        let yellow = gpui::hsla(60. / 360., 1., 0.5, 1.);

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
                            .bg(blue)
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
                            .bg(red)
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
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        135.,
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        225.,
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        315.,
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
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
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        90.,
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        180.,
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
                    )))
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        360.,
                        linear_color_stop(red, 0.),
                        linear_color_stop(blue, 1.),
                    ))),
            )
            .child(div().flex_1().rounded_xl().bg(linear_gradient(
                0.,
                linear_color_stop(green, 0.05),
                linear_color_stop(yellow, 0.95),
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
                                linear_color_stop(blue, 0.5),
                                linear_color_stop(red, 0.5),
                            ))),
                    )
                    .child(div().flex_1().rounded_xl().bg(linear_gradient(
                        180.,
                        linear_color_stop(green, 0.),
                        linear_color_stop(yellow, 0.5),
                    ))),
            )
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
