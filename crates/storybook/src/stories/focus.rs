use gpui::{
    App, Entity, FocusHandle, KeyBinding, Render, Subscription, Window, actions, div, prelude::*,
};
use ui::prelude::*;

actions!(focus, [ActionA, ActionB, ActionC]);

pub struct FocusStory {
    parent_focus: FocusHandle,
    child_1_focus: FocusHandle,
    child_2_focus: FocusHandle,
    _focus_subscriptions: Vec<Subscription>,
}

impl FocusStory {
    pub fn model(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.bind_keys([
            KeyBinding::new("cmd-a", ActionA, Some("parent")),
            KeyBinding::new("cmd-a", ActionB, Some("child-1")),
            KeyBinding::new("cmd-c", ActionC, None),
        ]);

        cx.new(|cx| {
            let parent_focus = cx.focus_handle();
            let child_1_focus = cx.focus_handle();
            let child_2_focus = cx.focus_handle();
            let _focus_subscriptions = vec![
                cx.on_focus(&parent_focus, window, |_, _, _| {
                    println!("Parent focused");
                }),
                cx.on_blur(&parent_focus, window, |_, _, _| {
                    println!("Parent blurred");
                }),
                cx.on_focus(&child_1_focus, window, |_, _, _| {
                    println!("Child 1 focused");
                }),
                cx.on_blur(&child_1_focus, window, |_, _, _| {
                    println!("Child 1 blurred");
                }),
                cx.on_focus(&child_2_focus, window, |_, _, _| {
                    println!("Child 2 focused");
                }),
                cx.on_blur(&child_2_focus, window, |_, _, _| {
                    println!("Child 2 blurred");
                }),
            ];

            Self {
                parent_focus,
                child_1_focus,
                child_2_focus,
                _focus_subscriptions,
            }
        })
    }
}

impl Render for FocusStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let color_1 = theme.status().created;
        let color_2 = theme.status().modified;
        let color_4 = theme.status().conflict;
        let color_5 = theme.status().ignored;
        let color_6 = theme.status().renamed;
        let color_7 = theme.status().hint;

        div()
            .id("parent")
            .active(|style| style.bg(color_7))
            .track_focus(&self.parent_focus)
            .key_context("parent")
            .on_action(cx.listener(|_, _action: &ActionA, _window, _cx| {
                println!("Action A dispatched on parent");
            }))
            .on_action(cx.listener(|_, _action: &ActionB, _window, _cx| {
                println!("Action B dispatched on parent");
            }))
            .on_key_down(cx.listener(|_, event, _, _| println!("Key down on parent {:?}", event)))
            .on_key_up(cx.listener(|_, event, _, _| println!("Key up on parent {:?}", event)))
            .size_full()
            .bg(color_1)
            .focus(|style| style.bg(color_2))
            .child(
                div()
                    .track_focus(&self.child_1_focus)
                    .key_context("child-1")
                    .on_action(cx.listener(|_, _action: &ActionB, _window, _cx| {
                        println!("Action B dispatched on child 1 during");
                    }))
                    .w_full()
                    .h_6()
                    .bg(color_4)
                    .focus(|style| style.bg(color_5))
                    .in_focus(|style| style.bg(color_6))
                    .on_key_down(
                        cx.listener(|_, event, _, _| println!("Key down on child 1 {:?}", event)),
                    )
                    .on_key_up(
                        cx.listener(|_, event, _, _| println!("Key up on child 1 {:?}", event)),
                    )
                    .child("Child 1"),
            )
            .child(
                div()
                    .track_focus(&self.child_2_focus)
                    .key_context("child-2")
                    .on_action(cx.listener(|_, _action: &ActionC, _window, _cx| {
                        println!("Action C dispatched on child 2");
                    }))
                    .w_full()
                    .h_6()
                    .bg(color_4)
                    .on_key_down(
                        cx.listener(|_, event, _, _| println!("Key down on child 2 {:?}", event)),
                    )
                    .on_key_up(
                        cx.listener(|_, event, _, _| println!("Key up on child 2 {:?}", event)),
                    )
                    .child("Child 2"),
            )
    }
}
