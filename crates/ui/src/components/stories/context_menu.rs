use gpui::{actions, AnchorCorner, Render, View};
use story::Story;

use crate::prelude::*;
use crate::{right_click_menu, ContextMenu, Label};

actions!(context_menu, [PrintCurrentDate, PrintBestFood]);

fn build_menu(cx: &mut WindowContext, header: impl Into<SharedString>) -> View<ContextMenu> {
    ContextMenu::build(cx, |menu, _| {
        menu.header(header)
            .separator()
            .action("Print current time", Box::new(PrintCurrentDate))
            .entry("Print best food", Some(Box::new(PrintBestFood)), |cx| {
                cx.dispatch_action(Box::new(PrintBestFood))
            })
    })
}

pub struct ContextMenuStory;

impl Render for ContextMenuStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .on_action(|_: &PrintCurrentDate, _| {
                println!("printing unix time!");
                if let Ok(unix_time) = std::time::UNIX_EPOCH.elapsed() {
                    println!("Current Unix time is {:?}", unix_time.as_secs());
                }
            })
            .on_action(|_: &PrintBestFood, _| {
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
