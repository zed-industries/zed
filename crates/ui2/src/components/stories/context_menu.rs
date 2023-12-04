use gpui::{actions, Action, AnchorCorner, Div, Render, View};
use story::Story;

use crate::prelude::*;
use crate::{right_click_menu, ContextMenu, Label};

actions!(PrintCurrentDate, PrintBestFood);

fn build_menu(cx: &mut WindowContext, header: impl Into<SharedString>) -> View<ContextMenu> {
    ContextMenu::build(cx, |menu, _| {
        menu.header(header)
            .separator()
            .entry("Print current time", |cx| {
                println!("dispatching PrintCurrentTime action");
                cx.dispatch_action(PrintCurrentDate.boxed_clone())
            })
            .entry("Print best foot", |cx| {
                cx.dispatch_action(PrintBestFood.boxed_clone())
            })
    })
}

pub struct ContextMenuStory;

impl Render for ContextMenuStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .on_action(|_: &PrintCurrentDate, _: &mut WindowContext| {
                println!("printing unix time!");
                if let Ok(unix_time) = std::time::UNIX_EPOCH.elapsed() {
                    println!("Current Unix time is {:?}", unix_time.as_secs());
                }
            })
            .on_action(|_: &PrintBestFood, _: &mut WindowContext| {
                println!("burrito");
            })
            .flex()
            .flex_row()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .child(
                        right_click_menu("test2")
                            .trigger(Label::new("TOP LEFT"))
                            .menu(move |cx| build_menu(cx, "top left")),
                    )
                    .child(
                        right_click_menu("test1")
                            .trigger(Label::new("BOTTOM LEFT"))
                            .anchor(AnchorCorner::BottomLeft)
                            .attach(AnchorCorner::TopLeft)
                            .menu(move |cx| build_menu(cx, "bottom left")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .child(
                        right_click_menu("test3")
                            .trigger(Label::new("TOP RIGHT"))
                            .anchor(AnchorCorner::TopRight)
                            .menu(move |cx| build_menu(cx, "top right")),
                    )
                    .child(
                        right_click_menu("test4")
                            .trigger(Label::new("BOTTOM RIGHT"))
                            .anchor(AnchorCorner::BottomRight)
                            .attach(AnchorCorner::TopRight)
                            .menu(move |cx| build_menu(cx, "bottom right")),
                    ),
            )
    }
}
