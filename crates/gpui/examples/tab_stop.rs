use gpui::{
    App, Application, Bounds, Context, Div, ElementId, FocusHandle, KeyBinding, SharedString,
    Stateful, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};

actions!(example, [Tab, TabPrev]);

struct Example {
    items: Vec<FocusHandle>,
    modal_items: Vec<FocusHandle>,
    message: SharedString,
    modal_open: bool,
    last_handle: Option<FocusHandle>,
}

impl Example {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let items = vec![
            cx.focus_handle().tab_index(1).tab_stop(true),
            cx.focus_handle().tab_index(2).tab_stop(true),
            cx.focus_handle().tab_index(3).tab_stop(true),
            cx.focus_handle(),
            cx.focus_handle().tab_index(2).tab_stop(true),
        ];
        let modal_items = vec![
            cx.focus_handle().tab_index(1).tab_stop(true),
            cx.focus_handle().tab_index(2).tab_stop(true),
        ];

        window.focus(items.first().unwrap());
        Self {
            items,
            modal_items,
            message: SharedString::from("Press `Tab`, `Shift-Tab` to switch focus."),
            modal_open: false,
            last_handle: None,
        }
    }

    fn on_tab(&mut self, _: &Tab, window: &mut Window, _: &mut Context<Self>) {
        window.focus_next();
        self.message = SharedString::from("You have pressed `Tab`.");
    }

    fn on_tab_prev(&mut self, _: &TabPrev, window: &mut Window, _: &mut Context<Self>) {
        window.focus_prev();
        self.message = SharedString::from("You have pressed `Shift-Tab`.");
    }
}

impl Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        fn button(id: impl Into<ElementId>) -> Stateful<Div> {
            div()
                .id(id)
                .h_10()
                .flex_1()
                .flex()
                .justify_center()
                .items_center()
                .border_1()
                .border_color(gpui::black())
                .bg(gpui::black())
                .text_color(gpui::white())
                .focus(|this| this.border_color(gpui::blue()))
                .shadow_sm()
        }

        fn input(
            id: impl Into<ElementId>,
            focus_handle: &FocusHandle,
            window: &mut Window,
        ) -> Stateful<Div> {
            div()
                .id(id)
                .track_focus(focus_handle)
                .h_10()
                .w_full()
                .flex()
                .justify_center()
                .items_center()
                .border_1()
                .border_color(gpui::black())
                .when(
                    focus_handle.tab_stop && focus_handle.is_focused(window),
                    |this| this.border_color(gpui::blue()),
                )
                .map(|this| match focus_handle.tab_stop {
                    true => this
                        .hover(|this| this.bg(gpui::black().opacity(0.1)))
                        .child(format!("tab_index: {}", focus_handle.tab_index)),
                    false => this.opacity(0.4).child("tab_stop: false"),
                })
        }

        div()
            .id("app")
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .size_full()
            .flex()
            .flex_col()
            .p_4()
            .gap_3()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(self.message.clone())
            .children(
                self.items
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(ix, item_handle)| input(("item", ix), &item_handle, window)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .child(button("el1").tab_index(4).child("Button 1"))
                    .child(button("el2").tab_index(5).child("Button 2"))
                    .child(button("open-modal").child("Open Modal...").on_click({
                        let first_handle = self.modal_items.first().cloned();
                        cx.listener(move |this, _, window, cx| {
                            this.last_handle = window.focused(cx);
                            this.modal_open = true;
                            if let Some(handle) = first_handle {
                                window.focus(handle);
                            }
                            cx.notify();
                        })
                    })),
            )
            .when(self.modal_open, |this| {
                this.child(
                    div()
                        .id("modal-overlay")
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .left_0()
                        .right_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(gpui::black().opacity(0.5))
                        .child(
                            div()
                                .id("modal")
                                .flex()
                                .flex_col()
                                .gap_3()
                                .w(px(450.))
                                .p_5()
                                .bg(gpui::white())
                                .shadow_md()
                                .rounded_md()
                                .tab_group("modal1")
                                .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                                    cx.stop_propagation();

                                    this.modal_open = false;
                                    if let Some(handle) = this.last_handle.as_ref() {
                                        window.focus(handle);
                                    }
                                    cx.notify();
                                }))
                                .child("Focus cycle in Modal")
                                .children(self.modal_items.clone().into_iter().enumerate().map(
                                    |(ix, item_handle)| {
                                        input(("modal-input", ix), &item_handle, window)
                                    },
                                )),
                        ),
                )
            })
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("tab", Tab, None),
            KeyBinding::new("shift-tab", TabPrev, None),
        ]);

        let bounds = Bounds::centered(None, size(px(800.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Example::new(window, cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
