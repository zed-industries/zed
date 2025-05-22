use gpui::{
    App, Application, Bounds, Context, TextOverflow, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, size,
};

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text = "The longest word 你好世界这段是中文，こんにちはこの段落は日本語です in any of the major English language dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis, a word that refers to a lung disease contracted from the inhalation of very fine silica particles, specifically from a volcano; medically, it is the same as silicosis.";
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
                    .flex_shrink_0()
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
                            .w_full()
                            .child("A short text in normal div"),
                    ),
            )
            .child(
                div()
                    .flex_shrink_0()
                    .text_xl()
                    .truncate()
                    .border_1()
                    .border_color(gpui::blue())
                    .child("ELLIPSIS: ".to_owned() + text),
            )
            .child(
                div()
                    .flex_shrink_0()
                    .text_xl()
                    .overflow_hidden()
                    .text_ellipsis()
                    .line_clamp(2)
                    .border_1()
                    .border_color(gpui::blue())
                    .child("ELLIPSIS 2 lines: ".to_owned() + text),
            )
            .child(
                div()
                    .flex_shrink_0()
                    .text_xl()
                    .overflow_hidden()
                    .text_overflow(TextOverflow::Ellipsis(""))
                    .border_1()
                    .border_color(gpui::green())
                    .child("TRUNCATE: ".to_owned() + text),
            )
            .child(
                div()
                    .flex_shrink_0()
                    .text_xl()
                    .overflow_hidden()
                    .text_overflow(TextOverflow::Ellipsis(""))
                    .line_clamp(3)
                    .border_1()
                    .border_color(gpui::green())
                    .child("TRUNCATE 3 lines: ".to_owned() + text),
            )
            .child(
                div()
                    .flex_shrink_0()
                    .text_xl()
                    .whitespace_nowrap()
                    .overflow_hidden()
                    .border_1()
                    .border_color(gpui::black())
                    .child("NOWRAP: ".to_owned() + text),
            )
            .child(div().text_xl().w_full().child(text))
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
