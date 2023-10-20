use crate::themes::rose_pine;
use gpui3::{
    div, view, Context, Focusable, KeyBinding, ParentElement, StatelessInteractive, Styled, View,
    WindowContext,
};

#[derive(Clone, PartialEq)]
struct ActionA;

#[derive(Clone, PartialEq)]
struct ActionB;

#[derive(Clone, PartialEq)]
struct ActionC;

pub struct FocusStory {
    text: View<()>,
}

impl FocusStory {
    pub fn view(cx: &mut WindowContext) -> View<()> {
        cx.bind_keys([
            KeyBinding::new("cmd-a", ActionA, Some("parent")),
            KeyBinding::new("cmd-a", ActionB, Some("child-1")),
            KeyBinding::new("cmd-c", ActionC, None),
        ]);
        let theme = rose_pine();

        let color_1 = theme.lowest.negative.default.foreground;
        let color_2 = theme.lowest.positive.default.foreground;
        let color_3 = theme.lowest.warning.default.foreground;
        let color_4 = theme.lowest.accent.default.foreground;
        let color_5 = theme.lowest.variant.default.foreground;
        let color_6 = theme.highest.negative.default.foreground;

        let child_1 = cx.focus_handle();
        let child_2 = cx.focus_handle();
        view(cx.entity(|cx| ()), move |_, cx| {
            div()
                .id("parent")
                .focusable()
                .context("parent")
                .on_action(|_, action: &ActionA, phase, cx| {
                    println!("Action A dispatched on parent during {:?}", phase);
                })
                .on_action(|_, action: &ActionB, phase, cx| {
                    println!("Action B dispatched on parent during {:?}", phase);
                })
                .on_focus(|_, _, _| println!("Parent focused"))
                .on_blur(|_, _, _| println!("Parent blurred"))
                .on_focus_in(|_, _, _| println!("Parent focus_in"))
                .on_focus_out(|_, _, _| println!("Parent focus_out"))
                .on_key_down(|_, event, phase, _| {
                    println!("Key down on parent {:?} {:?}", phase, event)
                })
                .on_key_up(|_, event, phase, _| {
                    println!("Key up on parent {:?} {:?}", phase, event)
                })
                .size_full()
                .bg(color_1)
                .focus(|style| style.bg(color_2))
                .focus_in(|style| style.bg(color_3))
                .child(
                    div()
                        .track_focus(&child_1)
                        .context("child-1")
                        .on_action(|_, action: &ActionB, phase, cx| {
                            println!("Action B dispatched on child 1 during {:?}", phase);
                        })
                        .w_full()
                        .h_6()
                        .bg(color_4)
                        .focus(|style| style.bg(color_5))
                        .in_focus(|style| style.bg(color_6))
                        .on_focus(|_, _, _| println!("Child 1 focused"))
                        .on_blur(|_, _, _| println!("Child 1 blurred"))
                        .on_focus_in(|_, _, _| println!("Child 1 focus_in"))
                        .on_focus_out(|_, _, _| println!("Child 1 focus_out"))
                        .on_key_down(|_, event, phase, _| {
                            println!("Key down on child 1 {:?} {:?}", phase, event)
                        })
                        .on_key_up(|_, event, phase, _| {
                            println!("Key up on child 1 {:?} {:?}", phase, event)
                        })
                        .child("Child 1"),
                )
                .child(
                    div()
                        .track_focus(&child_2)
                        .context("child-2")
                        .on_action(|_, action: &ActionC, phase, cx| {
                            println!("Action C dispatched on child 2 during {:?}", phase);
                        })
                        .w_full()
                        .h_6()
                        .bg(color_4)
                        .on_focus(|_, _, _| println!("Child 2 focused"))
                        .on_blur(|_, _, _| println!("Child 2 blurred"))
                        .on_focus_in(|_, _, _| println!("Child 2 focus_in"))
                        .on_focus_out(|_, _, _| println!("Child 2 focus_out"))
                        .on_key_down(|_, event, phase, _| {
                            println!("Key down on child 2 {:?} {:?}", phase, event)
                        })
                        .on_key_up(|_, event, phase, _| {
                            println!("Key up on child 2 {:?} {:?}", phase, event)
                        })
                        .child("Child 2"),
                )
        })
    }
}
