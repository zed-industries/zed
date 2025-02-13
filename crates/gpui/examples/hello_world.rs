use gpui::{
    div, prelude::*, px, relative, rgb, size, App, Application, Bounds, Context, SharedString,
    Window, WindowBounds, WindowOptions,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::white())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_5()
                    .bg(rgb(0x505050))
                    .size(px(500.0))
                    .justify_center()
                    .items_center()
                    .shadow_lg()
                    .border_3()
                    .border_color(rgb(0x303030))
                    .rounded_xl()
                    .text_xl()
                    .line_height(relative(1.))
                    .child(
                        div()
                            .id("hello")
                            .w_48()
                            .py_2()
                            .rounded_md()
                            .text_center()
                            .text_color(gpui::white())
                            .active(|this| {
                                this.text_color(gpui::red())
                                    .text_center()
                                    .text_decoration_1()
                                    .text_decoration_color(gpui::red())
                            })
                            .hover(|this| {
                                this.text_color(gpui::blue())
                                    .text_center()
                                    .text_decoration_1()
                                    .text_decoration_color(gpui::blue())
                            })
                            .child(format!("Hello, {}!", &self.text)),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(div().size_8().bg(gpui::red()))
                            .child(div().size_8().bg(gpui::green()))
                            .child(div().size_8().bg(gpui::blue()))
                            .child(div().size_8().bg(gpui::yellow()))
                            .child(div().size_8().bg(gpui::black()))
                            .child(div().size_8().bg(gpui::white())),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
