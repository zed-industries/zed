use gpui::{
    actions, div, prelude::*, Div, FocusHandle, Focusable, KeyBinding, Render, Stateful, View,
    WindowContext,
};
use theme2::ActiveTheme;

actions!(ActionA, ActionB, ActionC);

pub struct FocusStory {
    child_1_focus: FocusHandle,
    child_2_focus: FocusHandle,
}

impl FocusStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.bind_keys([
            KeyBinding::new("cmd-a", ActionA, Some("parent")),
            KeyBinding::new("cmd-a", ActionB, Some("child-1")),
            KeyBinding::new("cmd-c", ActionC, None),
        ]);

        cx.build_view(move |cx| Self {
            child_1_focus: cx.focus_handle(),
            child_2_focus: cx.focus_handle(),
        })
    }
}

impl Render for FocusStory {
    type Element = Focusable<Stateful<Div>>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        let theme = cx.theme();
        let color_1 = theme.status().created;
        let color_2 = theme.status().modified;
        let color_4 = theme.status().conflict;
        let color_5 = theme.status().ignored;
        let color_6 = theme.status().renamed;

        div()
            .id("parent")
            .focusable()
            .key_context("parent")
            .on_action(cx.listener(|_, _action: &ActionA, _cx| {
                println!("Action A dispatched on parent");
            }))
            .on_action(cx.listener(|_, _action: &ActionB, _cx| {
                println!("Action B dispatched on parent");
            }))
            .on_focus(cx.listener(|_, _, _| println!("Parent focused")))
            .on_blur(cx.listener(|_, _, _| println!("Parent blurred")))
            .on_focus_in(cx.listener(|_, _, _| println!("Parent focus_in")))
            .on_focus_out(cx.listener(|_, _, _| println!("Parent focus_out")))
            .on_key_down(cx.listener(|_, event, _| println!("Key down on parent {:?}", event)))
            .on_key_up(cx.listener(|_, event, _| println!("Key up on parent {:?}", event)))
            .size_full()
            .bg(color_1)
            .focus(|style| style.bg(color_2))
            .child(
                div()
                    .track_focus(&self.child_1_focus)
                    .key_context("child-1")
                    .on_action(cx.listener(|_, _action: &ActionB, _cx| {
                        println!("Action B dispatched on child 1 during");
                    }))
                    .w_full()
                    .h_6()
                    .bg(color_4)
                    .focus(|style| style.bg(color_5))
                    .in_focus(|style| style.bg(color_6))
                    .on_focus(cx.listener(|_, _, _| println!("Child 1 focused")))
                    .on_blur(cx.listener(|_, _, _| println!("Child 1 blurred")))
                    .on_focus_in(cx.listener(|_, _, _| println!("Child 1 focus_in")))
                    .on_focus_out(cx.listener(|_, _, _| println!("Child 1 focus_out")))
                    .on_key_down(
                        cx.listener(|_, event, _| println!("Key down on child 1 {:?}", event)),
                    )
                    .on_key_up(cx.listener(|_, event, _| println!("Key up on child 1 {:?}", event)))
                    .child("Child 1"),
            )
            .child(
                div()
                    .track_focus(&self.child_2_focus)
                    .key_context("child-2")
                    .on_action(cx.listener(|_, _action: &ActionC, _cx| {
                        println!("Action C dispatched on child 2");
                    }))
                    .w_full()
                    .h_6()
                    .bg(color_4)
                    .on_focus(cx.listener(|_, _, _| println!("Child 2 focused")))
                    .on_blur(cx.listener(|_, _, _| println!("Child 2 blurred")))
                    .on_focus_in(cx.listener(|_, _, _| println!("Child 2 focus_in")))
                    .on_focus_out(cx.listener(|_, _, _| println!("Child 2 focus_out")))
                    .on_key_down(
                        cx.listener(|_, event, _| println!("Key down on child 2 {:?}", event)),
                    )
                    .on_key_up(cx.listener(|_, event, _| println!("Key up on child 2 {:?}", event)))
                    .child("Child 2"),
            )
    }
}
