use gpui::*;

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = "The longest word in any of the major English language 以及中文的测试 dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis, a word that refers to a lung disease contracted from the inhalation of very fine silica particles, specifically from a volcano; medically, it is the same as silicosis.";
        div()
            .id("page")
            .size_full()
            .flex()
            .flex_col()
            .p_2()
            .gap_2()
            .bg(gpui::white())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .border_1()
                            .border_color(gpui::red())
                            .text_ellipsis()
                            .child("longer text in flex 1"),
                    )
                    .child(
                        div()
                            .flex()
                            .border_1()
                            .border_color(gpui::red())
                            .text_ellipsis()
                            .child("short flex"),
                    )
                    .child(
                        div()
                            .overflow_hidden()
                            .border_1()
                            .border_color(gpui::red())
                            .text_ellipsis()
                            .child("A short text in normal div"),
                    ),
            )
            .child(
                div()
                    .text_xl()
                    .overflow_hidden()
                    .text_ellipsis()
                    .border_1()
                    .border_color(gpui::red())
                    .child("ELLIPSIS: ".to_owned() + text),
            )
            .child(
                div()
                    .text_xl()
                    .overflow_hidden()
                    .truncate()
                    .border_1()
                    .border_color(gpui::green())
                    .child("TRUNCATE: ".to_owned() + text),
            )
            .child(
                div()
                    .text_xl()
                    .whitespace_nowrap()
                    .overflow_hidden()
                    .border_1()
                    .border_color(gpui::blue())
                    .child("NOWRAP: ".to_owned() + text),
            )
            .child(div().text_xl().w_full().child(text))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(600.0), px(480.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| HelloWorld {}),
        )
        .unwrap();
    });
}
