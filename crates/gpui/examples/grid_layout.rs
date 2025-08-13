use gpui::{
    App, Application, Bounds, Context, Hsla, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let block = |color: Hsla| {
            div()
                .size_12()
                .bg(color)
                .border_1()
                .border_dashed()
                .rounded_md()
                .border_color(gpui::white())
        };

        let colors = [
            gpui::red(),
            gpui::green(),
            gpui::blue(),
            gpui::yellow(),
            gpui::black(),
        ];

        div()
            .gap_1()
            .grid()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .grid_cols(3)
            .child(block(gpui::white()).col_span(2).items_center())
            .children(colors.map(block))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| HelloWorld {}),
        )
        .unwrap();
        cx.activate(true);
    });
}
