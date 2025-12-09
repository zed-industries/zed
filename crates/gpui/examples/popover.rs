use gpui::{
    App, Application, Context, Corner, Div, Stateful, Window, WindowOptions, anchored, deferred,
    div, prelude::*, px,
};

/// An example show use deferred to create a floating layers.
struct HelloWorld {
    open: bool,
    secondary_open: bool,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        fn button(id: &'static str) -> Stateful<Div> {
            div()
                .id(id)
                .bg(gpui::black())
                .text_color(gpui::white())
                .px_3()
                .py_1()
        }

        fn popover() -> Div {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .shadow_lg()
                .p_3()
                .rounded_md()
                .bg(gpui::white())
                .text_color(gpui::black())
                .border_1()
                .text_sm()
                .border_color(gpui::black().opacity(0.1))
        }

        div()
            .flex()
            .flex_row()
            .gap_3()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .justify_center()
            .items_center()
            .child(
                button("button")
                    .child("Open Popover")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.open = true;
                        cx.notify();
                    }))
                    .when(self.open, |this| {
                        this.child(
                            deferred(
                                anchored()
                                    .anchor(Corner::TopLeft)
                                    .snap_to_window_with_margin(px(8.))
                                    .child(
                                        popover()
                                            .child("This is first level Popover.")
                                            .child("Click outside to close.")
                                            .when(!self.secondary_open, |this| {
                                                this.on_mouse_down_out(cx.listener(
                                                    |this, _, _, cx| {
                                                        this.open = false;
                                                        cx.notify();
                                                    },
                                                ))
                                            })
                                            .child(
                                                button("secondary-btn")
                                                    .mt_2()
                                                    .child("Child Popover")
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.secondary_open = true;
                                                        cx.notify();
                                                    }))
                                                    .when(self.secondary_open, |this| {
                                                        this.child(
                                                deferred(
                                                    anchored()
                                                        .snap_to_window_with_margin(px(8.))
                                                        .child(
                                                            popover()
                                                                .child(
                                                                    "This is second level Popover",
                                                                )
                                                                .on_mouse_down_out(cx.listener(
                                                                    |this, _, _, cx| {
                                                                        this.secondary_open = true;
                                                                        cx.notify();
                                                                    },
                                                                )),
                                                        ),
                                                )
                                                .priority(2),
                                            )
                                                    }),
                                            ),
                                    ),
                            )
                            .priority(0),
                        )
                    }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld {
                open: false,
                secondary_open: false,
            })
        })
        .unwrap();
        cx.activate(true);
    });
}
