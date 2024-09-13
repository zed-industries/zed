use gpui::*;

struct Example {
    text: SharedString,
}

impl Example {
    fn update_text(&mut self, prefix: &str, position: Point<Pixels>, cx: &mut ViewContext<Self>) {
        self.text = format!("{}: {}, {}", prefix, position.x.ceil(), position.y.ceil()).into();
        cx.notify();
    }
}

impl Render for Example {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(gpui::white())
            .child(
                div()
                    .id("move-area")
                    .flex()
                    .items_center()
                    .justify_center()
                    .border_4()
                    .border_color(gpui::blue())
                    .bg(rgb(0xebf3ff))
                    .size_72()
                    .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, cx| {
                        this.update_text("Mouse move", event.position, cx);
                    }))
                    .on_mouse_leave(cx.listener(|this, event: &MouseMoveEvent, cx| {
                        this.update_text("Mouse leave", event.position, cx)
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, cx| {
                            this.update_text("Mouse down (Left)", event.position, cx)
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseUpEvent, cx| {
                            this.update_text("Mouse up (Left)", event.position, cx)
                        }),
                    )
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(|this, event: &MouseDownEvent, cx| {
                            this.update_text("Mouse down (Right)", event.position, cx)
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Right,
                        cx.listener(|this, event: &MouseUpEvent, cx| {
                            this.update_text("Mouse up (Right)", event.position, cx)
                        }),
                    )
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(|this, event: &MouseDownEvent, cx| {
                            this.update_text("Mouse down (Right)", event.position, cx)
                        }),
                    )
                    .on_mouse_down_out(cx.listener(|this, event: &MouseDownEvent, cx| {
                        this.update_text("Mouse down out", event.position, cx)
                    }))
                    .on_mouse_up_out(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseUpEvent, cx| {
                            this.update_text("Mouse up out (Left)", event.position, cx)
                        }),
                    )
                    .text_color(gpui::black())
                    .child(self.text.clone()),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| Example {
                    text: SharedString::from("Try move mouse over this area"),
                })
            },
        )
        .unwrap();
    });
}
