use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Window, WindowBounds,
    WindowOptions,
};

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
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
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_around()
                    .gap_3()
                    .child(
                        div()
                            .cursor_pointer()
                            .text_color(gpui::black())
                            .hover(|this| this.text_color(gpui::blue()))
                            .child("Hover Blue"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_color(gpui::black())
                            .hover(|this| this.underline())
                            .child("Hover Underline"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_color(gpui::black())
                            .hover(|this| {
                                this.text_decoration_1()
                                    .text_decoration_wavy()
                                    .text_decoration_color(gpui::blue())
                            })
                            .child("Hover Wavy"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_color(gpui::black())
                            .hover(|this| this.bg(gpui::yellow()))
                            .child("Hover Background"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_around()
                    .gap_3()
                    .child(
                        div()
                            .id("active-0")
                            .cursor_pointer()
                            .text_color(gpui::black())
                            .active(|this| this.text_color(gpui::red()))
                            .child("Active Red"),
                    )
                    .child(
                        div()
                            .id("active-1")
                            .cursor_pointer()
                            .text_color(gpui::black())
                            .active(|this| this.underline())
                            .child("Active Underline"),
                    )
                    .child(
                        div()
                            .id("active-2")
                            .cursor_pointer()
                            .text_color(gpui::black())
                            .active(|this| {
                                this.text_decoration_1()
                                    .text_decoration_wavy()
                                    .text_decoration_color(gpui::red())
                            })
                            .child("Active Wavy"),
                    )
                    .child(
                        div()
                            .id("active-3")
                            .text_color(gpui::black())
                            .active(|this| this.bg(gpui::yellow()))
                            .child("Active Background"),
                    ),
            )
            .child(
                div()
                    .id("btn")
                    .w_40()
                    .py_2()
                    .rounded_md()
                    .text_color(gpui::blue())
                    .text_center()
                    .rounded_md()
                    .active(|this| {
                        this.text_color(gpui::white())
                            .bg(gpui::blue())
                            .text_center()
                            .text_decoration_1()
                            .text_decoration_wavy()
                    })
                    .hover(|this| {
                        this.text_color(rgb(0x973717))
                            .bg(gpui::yellow())
                            .text_center()
                            .text_decoration_1()
                    })
                    .child("Link styles"),
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
