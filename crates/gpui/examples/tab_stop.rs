use gpui::{
    App, Application, Bounds, Context, Div, ElementId, Entity, FocusHandle, KeyBinding,
    SharedString, Stateful, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px,
    size,
};

actions!(example, [Tab, TabPrev]);

struct Example {
    focus_handle: FocusHandle,
    items: Vec<FocusHandle>,
    modal_items: Vec<FocusHandle>,
    message: SharedString,
    last_focused: Option<FocusHandle>,
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

        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle);

        Self {
            focus_handle,
            items,
            modal_items,
            message: SharedString::from("Press `Tab`, `Shift-Tab` to switch focus."),
            last_focused: None,
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

fn button(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .h_9()
        .flex_1()
        .flex()
        .justify_center()
        .items_center()
        .border_3()
        .border_color(gpui::black())
        .rounded_md()
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

fn modal<E>(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    open_state: Entity<bool>,
    children: impl IntoIterator<Item = E>,
    cx: &mut Context<Example>,
) -> Stateful<Div>
where
    E: IntoElement,
{
    div()
        .id(id)
        .focus_trap()
        .occlude()
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
                .on_mouse_down_out(cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();

                    open_state.update(cx, |open, _| {
                        *open = false;
                    });
                    if let Some(handle) = this.last_focused.as_ref() {
                        window.focus(handle);
                    }
                    cx.notify();
                }))
                .child(title.into())
                .children(children),
        )
}

impl Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let modal1_open = window
            .use_keyed_state("modal1-open", cx, |_, cx| cx.new(|_| false))
            .read(cx)
            .clone();
        let modal1_focus_handle = window
            .use_keyed_state("modal1-focus-handle", cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();

        let modal2_open = window
            .use_keyed_state("modal2-open", cx, |_, cx| cx.new(|_| false))
            .read(cx)
            .clone();
        let modal2_focus_handle = window
            .use_keyed_state("modal2-focus-handle", cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();

        div()
            .id("app")
            .track_focus(&self.focus_handle)
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
                    .child(
                        button("el1")
                            .tab_index(4)
                            .child("Button 1")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.message = "You have clicked Button 1.".into();
                                cx.notify();
                            })),
                    )
                    .child(
                        button("el2")
                            .tab_index(5)
                            .child("Button 2")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.message = "You have clicked Button 2.".into();
                                cx.notify();
                            })),
                    )
                    .child(button("open-modal1").child("Open Modal...").on_click({
                        let first_handle = self.modal_items.first().cloned();
                        let modal1_open = modal1_open.clone();
                        cx.listener(move |this, _, window, cx| {
                            this.last_focused = window.focused(cx);
                            modal1_open.update(cx, |open, _| {
                                *open = true;
                            });
                            if let Some(handle) = first_handle.as_ref() {
                                window.focus(handle);
                            }
                            cx.notify();
                        })
                    }))
                    .child(button("open-modal2").child("Other Modal...").on_click({
                        let modal2_focus_handle = modal2_focus_handle.clone();
                        let modal2_open = modal2_open.clone();
                        cx.listener(move |this, _, window, cx| {
                            this.last_focused = window.focused(cx);
                            modal2_focus_handle.focus(window);
                            modal2_open.update(cx, |open, _| {
                                *open = true;
                            });
                            cx.notify();
                        })
                    })),
            )
            .when(*modal1_open.read(cx), |this| {
                this.child(
                    modal(
                        "modal1",
                        "Focus cycle in Modal",
                        modal1_open.clone(),
                        self.modal_items.clone().into_iter().enumerate().map(
                            |(ix, item_handle)| input(("modal-input", ix), &item_handle, window),
                        ),
                        cx,
                    )
                    .track_focus(&modal1_focus_handle),
                )
            })
            .when(*modal2_open.read(cx), |this| {
                this.child(
                    modal(
                        "modal2",
                        "Empty Modal will block focus",
                        modal2_open.clone(),
                        vec![div().child("This is a empty modal.")],
                        cx,
                    )
                    .track_focus(&modal2_focus_handle),
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
