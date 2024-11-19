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
        let red = gpui::red();
        let blue = gpui::blue();

        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(
div().flex().flex_1().gap_3().child(
    div()
        .h_24()
        .w_full()
        .rounded_lg()
        .flex()
        .items_center()
        .justify_center()
        .bg(blue)
        .text_color(gpui::white())
        .child("Solid Color"),
).child(
    div()
        .h_24()
        .w_full()
        .rounded_lg()
        .flex()
        .items_center()
        .justify_center()
        .bg(red)
        .text_color(gpui::white())
        .child("Solid Color"),
)
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .h_24()
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        45.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    )))
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        135.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    )))
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        225.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    )))
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        315.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    ))),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .h_24()
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        0.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    )))
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        90.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    )))
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        180.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    )))
                    .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                        360.,
                        [
                            BackgroundColorStop::new(0., red),
                            BackgroundColorStop::new(1., blue),
                        ],
                    ))),
            )
            .child(div().flex_1().rounded_lg().bg(Background::linear_gradient(
                180.,
                [
                    BackgroundColorStop::new(0., gpui::green()),
                    BackgroundColorStop::new(1., gpui::yellow()),
                ],
            )))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap_3()
                    .child(
                        div().flex().flex_1().gap_3().child(
                            div().flex_1().rounded_lg().bg(Background::linear_gradient(
                                0.,
                                [
                                    BackgroundColorStop::new(0.5, blue),
                                    BackgroundColorStop::new(1., gpui::green()),
                                ],
                            )
                            .opacity(0.8)),
                        ),
                    )
                    .child(
                        div().flex_1().rounded_lg().bg(Background::linear_gradient(
                            180.,
                            [
                                BackgroundColorStop::new(0.25, gpui::yellow()),
                                BackgroundColorStop::new(0.8, gpui::green()),
                            ],
                        )
                        .opacity(0.8)),
                    ),
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
