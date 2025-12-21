use gpui::{
    App, Application, Bounds, Context, Div, ElementId, FocusHandle, KeyBinding, SharedString,
    Stateful, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};

actions!(example, [Tab, TabPrev]);

struct Example {
    focus_handle: FocusHandle,
    items: Vec<FocusHandle>,
    message: SharedString,
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

        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle, cx);

        Self {
            focus_handle,
            items,
            message: SharedString::from("Press `Tab`, `Shift-Tab` to switch focus."),
        }
    }

    fn on_tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
        self.message = SharedString::from("You have pressed `Tab`.");
    }

    fn on_tab_prev(&mut self, _: &TabPrev, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
        self.message = SharedString::from("You have pressed `Shift-Tab`.");
    }
}

impl Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        fn tab_stop_style<T: Styled>(this: T) -> T {
            this.border_3().border_color(gpui::blue())
        }

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
                .focus(tab_stop_style)
                .shadow_sm()
        }

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
                    .map(|(ix, item_handle)| {
                        div()
                            .id(("item", ix))
                            .track_focus(&item_handle)
                            .h_10()
                            .w_full()
                            .flex()
                            .justify_center()
                            .items_center()
                            .border_1()
                            .border_color(gpui::black())
                            .when(
                                item_handle.tab_stop && item_handle.is_focused(window),
                                tab_stop_style,
                            )
                            .map(|this| match item_handle.tab_stop {
                                true => this
                                    .hover(|this| this.bg(gpui::black().opacity(0.1)))
                                    .child(format!("tab_index: {}", item_handle.tab_index)),
                                false => this.opacity(0.4).child("tab_stop: false"),
                            })
                    }),
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
                    ),
            )
            .child(
                div()
                    .id("group-1")
                    .tab_index(6)
                    .tab_group()
                    .tab_stop(false)
                    .child(
                        button("group-1-button-1")
                            .tab_index(1)
                            .child("Tab index [6, 1]"),
                    )
                    .child(
                        button("group-1-button-2")
                            .tab_index(2)
                            .child("Tab index [6, 2]"),
                    )
                    .child(
                        button("group-1-button-3")
                            .tab_index(3)
                            .child("Tab index [6, 3]"),
                    ),
            )
            .child(
                div()
                    .id("group-2")
                    .tab_index(7)
                    .tab_group()
                    .tab_stop(false)
                    .child(
                        button("group-2-button-1")
                            .tab_index(1)
                            .child("Tab index [7, 1]"),
                    )
                    .child(
                        button("group-2-button-2")
                            .tab_index(2)
                            .child("Tab index [7, 2]"),
                    )
                    .child(
                        button("group-2-button-3")
                            .tab_index(3)
                            .child("Tab index [7, 3]"),
                    ),
            )
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
