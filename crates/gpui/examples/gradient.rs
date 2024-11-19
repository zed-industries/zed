use gpui::{
    div, prelude::*, App, AppContext, Background, BackgroundColorStop, Render, ViewContext,
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
                            .h_24()
                            .w_full()
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
                            .h_24()
                            .w_full()
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
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        45.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    )))
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        135.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    )))
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        225.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    )))
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        315.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    ))),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .h_24()
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        0.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    )))
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        90.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    )))
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        180.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    )))
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        360.,
                        [
                            BackgroundColorStop::new(red, 0.),
                            BackgroundColorStop::new(blue, 1.),
                        ],
                    ))),
            )
            .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                180.,
                [
                    BackgroundColorStop::new(gpui::black(), 0.2),
                    BackgroundColorStop::new(gpui::white(), 1.),
                ],
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
                            .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                                90.,
                                [
                                    BackgroundColorStop::new(blue, 0.5),
                                    BackgroundColorStop::new(green, 0.5),
                                ],
                            ))),
                    )
                    .child(div().flex_1().rounded_xl().bg(Background::linear_gradient(
                        180.,
                        [
                            BackgroundColorStop::new(red, 0.25),
                            BackgroundColorStop::new(yellow, 0.8),
                        ],
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
