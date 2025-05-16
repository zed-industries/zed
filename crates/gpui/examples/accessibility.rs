use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, world!"))
            .child(
                div()
                    .child(
                        div()
                            .child("Button label")
                            .aria_role(accesskit::Role::Label),
                    )
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .id("button")
                            .size_8()
                            .bg(gpui::red())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white())
                            .aria_role(accesskit::Role::Button)
                            .on_click(|_, _, _| println!("button clicked")),
                    ),
            )
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
