use gpui::{
    App, Application, Context, Corner, Div, Hsla, Stateful, Window, WindowOptions, anchored,
    deferred, div, prelude::*, px,
};

/// An example show use deferred to create a floating layers.
struct HelloWorld {
    open: bool,
    secondary_open: bool,
}

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

fn line(color: Hsla) -> Div {
    div().w(px(480.)).h_2().bg(color.opacity(0.25))
}

impl HelloWorld {
    fn render_secondary_popover(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        button("secondary-btn")
            .mt_2()
            .child("Child Popover")
            .on_click(cx.listener(|this, _, _, cx| {
                this.secondary_open = true;
                cx.notify();
            }))
            .when(self.secondary_open, |this| {
                this.child(
                    // GPUI can't support deferred here yet,
                    // it was inside another deferred element.
                    anchored()
                        .anchor(Corner::TopLeft)
                        .snap_to_window_with_margin(px(8.))
                        .child(
                            popover()
                                .child("This is second level Popover")
                                .bg(gpui::white())
                                .border_color(gpui::blue())
                                .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                                    this.secondary_open = false;
                                    cx.notify();
                                })),
                        ),
                )
            })
    }
}

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .justify_center()
            .items_center()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(
                        button("popover0").child("Opened Popover").child(
                            deferred(
                                anchored()
                                    .anchor(Corner::TopLeft)
                                    .snap_to_window_with_margin(px(8.))
                                    .child(popover().w_96().gap_3().child(
                                        "This is a default opened Popover, \
                                        we can use deferred to render it \
                                        in a floating layer.",
                                    )),
                            )
                            .priority(0),
                        ),
                    )
                    .child(
                        button("popover1")
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
                                                    .w_96()
                                                    .gap_3()
                                                    .child(
                                                        "This is first level Popover, \
                                                   we can use deferred to render it \
                                                   in a floating layer.\n\
                                                   Click outside to close.",
                                                    )
                                                    .when(!self.secondary_open, |this| {
                                                        this.on_mouse_down_out(cx.listener(
                                                            |this, _, _, cx| {
                                                                this.open = false;
                                                                cx.notify();
                                                            },
                                                        ))
                                                    })
                                                    // Here we need render popover after the content
                                                    // to ensure it will be on top layer.
                                                    .child(
                                                        self.render_secondary_popover(window, cx),
                                                    ),
                                            ),
                                    )
                                    .priority(1),
                                )
                            }),
                    ),
            )
            .child(
                "Here is an example text rendered, \
                to ensure the Popover will float above this contents.",
            )
            .children([
                line(gpui::red()),
                line(gpui::yellow()),
                line(gpui::blue()),
                line(gpui::green()),
            ])
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
