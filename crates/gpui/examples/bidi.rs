use gpui::{
    App, Application, Bounds, Context, LayoutDirection, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

#[derive(IntoElement)]
struct BidiExampleComponent {
    header: &'static str,
    sub_title: &'static str,
    content: &'static str,
}

impl RenderOnce for BidiExampleComponent {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let main_color = rgb(0xF0F0F3);

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(20.0))
            .gap_2()
            .child(div().text_3xl().child(self.header))
            .child(self.sub_title)
            .child(
                div()
                    .border_r_1()
                    .border_color(main_color)
                    .pr_1()
                    .flex_shrink()
                    .child(self.content),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .gap_1()
                    .child(
                        div()
                            .border_1()
                            .p_1()
                            .border_color(main_color)
                            .child("Child 1"),
                    )
                    .child(
                        div()
                            .border_1()
                            .p_1()
                            .border_color(main_color)
                            .child("Child 2"),
                    )
                    .child(
                        div()
                            .border_1()
                            .p_1()
                            .border_color(main_color)
                            .child("Child 3"),
                    ),
            )
            .child(div().child(format!(
                "window.current_layout_direction(): {:?}",
                window.current_layout_direction()
            )))
    }
}

struct BidiView;

impl Render for BidiView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let main_color = rgb(0xF0F0F3);

        div()
            .bg(rgb(0x0c0c11))
            .text_color(main_color)
            .flex()
            .w_full()
            .h_full()
            .flex_col()
            .child(BidiExampleComponent {
                header: "This div uses the window's default window direction!",
                sub_title: "Try changing layout_direction in the example code's WindowOptions!",
                content: "This div has a border and padding on its right side, but it's \
                              rendered in RTL, so it shows up on the left instead. Margins are \
                              also automatically switched based on the layout direction.",
            })
            .child(div().w_full().dir_ltr().child(BidiExampleComponent {
                header: "This div is manually set to left-to-right!",
                sub_title: "Except for the strings, the code for these elements are the exact \
                                    as the RTL example! Directionality propagates to child \
                                    elements, but you can always set children to a different \
                                    directionality with dir_rtl() or dir_ltr().",
                content: "This div has the border and padding on the right side, and it's \
                                  displayed on the right side, as the directionality for the \
                                  parent is set to left-to-right.",
            }))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                layout_direction: LayoutDirection::RightToLeft,
                ..Default::default()
            },
            |_, cx| cx.new(|_| BidiView),
        )
        .unwrap();
        cx.activate(true);
    });
}
