use gpui::{
    App, Application, Bounds, Context, Hsla, IntoElement, Render, RenderOnce, Window, WindowBounds,
    WindowOptions, colors::Colors, div, prelude::*, px, size,
};

#[derive(IntoElement)]
struct Counter;

impl RenderOnce for Counter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let count = window.use_state(cx, |_, _| 0i32);

        let colors = Colors::for_appearance(window);
        let container: Hsla = colors.container.into();
        let bg_hover = container.clone().opacity(0.8);
        let bg_active = container.clone().opacity(0.65);

        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(colors.background)
            .text_color(colors.text)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .p_8()
                    .child(
                        div()
                            .id("decrement")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_10()
                            .h_10()
                            .bg(colors.container)
                            .hover(|style| style.bg(bg_hover))
                            .active(|style| style.bg(bg_active))
                            .rounded_md()
                            .shadow_sm()
                            .cursor_pointer()
                            .text_xl()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .on_click({
                                let count = count.clone();
                                move |_, _, cx| {
                                    count.update(cx, |count, cx| {
                                        *count -= 1;
                                        cx.notify();
                                    });
                                }
                            })
                            .child("-"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .min_w_20()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(count.read(cx).to_string()),
                    )
                    .child(
                        div()
                            .id("increment")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_10()
                            .h_10()
                            .bg(colors.container)
                            .hover(|style| style.bg(bg_hover))
                            .active(|style| style.bg(bg_active))
                            .rounded_md()
                            .shadow_sm()
                            .cursor_pointer()
                            .text_xl()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .on_click({
                                let count = count.clone();
                                move |_, _, cx| {
                                    count.update(cx, |count, cx| {
                                        *count += 1;
                                        cx.notify();
                                    });
                                }
                            })
                            .child("+"),
                    ),
            )
    }
}

struct CounterView;

impl Render for CounterView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Counter
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.), px(300.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                ..Default::default()
            },
            |_, cx| cx.new(|_| CounterView),
        )
        .unwrap();
        cx.activate(true);
    });
}
