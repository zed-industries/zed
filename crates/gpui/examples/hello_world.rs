use gpui::*;
use prelude::FluentBuilder;

struct HelloWorld {
    text: SharedString,
}

/*
Things to do:
1. We need a way of calculating which edge or corner the mouse is on,
    and then dispatch on that
2. We need to improve the shadow rendering significantly
3. We need to implement the techniques in here in Zed
*/

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let decorations = cx.window_decorations();

        div()
            .id("window-backdrop")
            .when(decorations == WindowDecorations::Client, |div| {
                div.bg(gpui::transparent_black())
                    .shadow_lg()
                    .rounded_t(px(10.0))
                    .p(px(16.0))
                    .on_mouse_move(|e, cx| {
                        if e.dragging() {
                            cx.start_window_resize(ResizeEdge::Left)
                        }
                    })
            })
            .size_full()
            .child(
                canvas(
                    |_, _| {},
                    |bounds, _, cx| {
                        cx.set_content_area(bounds);
                    },
                )
                .size_full()
                .absolute(),
            )
            .child(
                div()
                    .when(decorations == WindowDecorations::Client, |div| {
                        div.rounded_t(px(10.0))
                    })
                    .on_mouse_move(|_e, cx| {
                        cx.stop_propagation();
                    })
                    .bg(gpui::black())
                    .size_full()
                    .flex()
                    .flex_col()
                    .justify_around()
                    .child(
                        div().w_full().flex().flex_row().justify_around().child(
                            div()
                                .id("hello")
                                .flex()
                                .bg(rgb(0x2e7d32))
                                .size(Length::Definite(Pixels(300.0).into()))
                                .justify_center()
                                .items_center()
                                .shadow_lg()
                                .border_1()
                                .border_color(rgb(0x0000ff))
                                .text_xl()
                                .text_color(rgb(0xffffff))
                                .child(format!("Hello, {}!", &self.text))
                                .on_mouse_move(|e, cx| {
                                    if e.dragging() {
                                        cx.start_window_move();
                                    }
                                }),
                        ),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
    });
}
