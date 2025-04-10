use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .gap_4()
            .size_full()
            .child(div().child("Text left"))
            .child(div().text_center().child("Text center"))
            .child(div().text_right().child("Text right"))
            .child(div().text_decoration_1().child("Text left (underline)"))
            .child(
                div()
                    .text_center()
                    .text_decoration_1()
                    .child("Text center (underline)"),
            )
            .child(
                div()
                    .text_right()
                    .text_decoration_1()
                    .child("Text right (underline)"),
            )
            .child(div().line_through().child("Text left (line_through)"))
            .child(
                div()
                    .text_center()
                    .line_through()
                    .child("Text center (line_through)"),
            )
            .child(
                div()
                    .text_right()
                    .line_through()
                    .child("Text right (line_through)"),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_between()
                    .child(
                        div()
                            .w(px(400.))
                            .border_1()
                            .border_color(gpui::blue())
                            .p_1()
                            .whitespace_nowrap()
                            .overflow_hidden()
                            .text_center()
                            .child("A long non-wrapping text align center"),
                    )
                    .child(
                        div()
                            .w_32()
                            .border_1()
                            .border_color(gpui::blue())
                            .p_1()
                            .whitespace_nowrap()
                            .overflow_hidden()
                            .text_right()
                            .child("100%"),
                    ),
            )
            .child(
                div()
                    .id("Text Link")
                    .text_color(gpui::blue())
                    .cursor_pointer()
                    .active(|this| {
                        this.text_color(gpui::white())
                            .bg(gpui::blue())
                            .text_decoration_1()
                            .text_decoration_wavy()
                    })
                    .hover(|this| {
                        this.text_color(gpui::rgb(0x973717))
                            .bg(gpui::yellow())
                            .text_decoration_1()
                    })
                    .child("Text with hover, active styles"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
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
