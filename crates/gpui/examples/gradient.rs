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
        let color1 = Background::linear_gradient(
            0.,
            [
                BackgroundColorStop::new(0.5, gpui::yellow()),
                BackgroundColorStop::new(1., gpui::red()),
            ],
        );
        let color2 = Background::linear_gradient(
            90.,
            [
                BackgroundColorStop::new(0.5, gpui::blue()),
                BackgroundColorStop::new(0.8, gpui::green()),
            ],
        );
        let color3 = Background::linear_gradient(
            0.,
            [
                BackgroundColorStop::new(0.0, gpui::blue()),
                BackgroundColorStop::new(0.15, gpui::green()),
            ],
        );

        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .gap_3()
            .child(div().size_32().rounded_lg().bg(gpui::blue()))
            .child(div().size_32().rounded_lg().bg(color1))
            .child(div().size_32().rounded_lg().bg(color2))
            .child(div().size_32().rounded_lg().bg(color3))
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
