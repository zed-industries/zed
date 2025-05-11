use gpui::{Corner, Entity, Render, actions};
use story::Story;

use crate::prelude::*;
use crate::{ContextMenu, Label, right_click_menu};

actions!(context_menu, [PrintCurrentDate, PrintBestFood]);

fn build_menu(
    window: &mut Window,
    cx: &mut App,
    header: impl Into<SharedString>,
) -> Entity<ContextMenu> {
    ContextMenu::build(window, cx, |menu, _, _| {
        menu.header(header)
            .separator()
            .action("Print current time", Box::new(PrintCurrentDate))
            .entry(
                "Print best food",
                Some(Box::new(PrintBestFood)),
                |window, cx| window.dispatch_action(Box::new(PrintBestFood), cx),
            )
    })
}

pub struct ContextMenuStory;

impl Render for ContextMenuStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Story::container()
            .on_action(|_: &PrintCurrentDate, _, _| {
                println!("printing unix time!");
                if let Ok(unix_time) = std::time::UNIX_EPOCH.elapsed() {
                    println!("Current Unix time is {:?}", unix_time.as_secs());
                }
            })
            .on_action(|_: &PrintBestFood, _, _| {
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
                            .trigger(|_| Label::new("TOP LEFT"))
                            .menu(move |window, cx| build_menu(window, cx, "top left")),
                    )
                    .child(
                        right_click_menu("test1")
                            .trigger(|_| Label::new("BOTTOM LEFT"))
                            .anchor(Corner::BottomLeft)
                            .attach(Corner::TopLeft)
                            .menu(move |window, cx| build_menu(window, cx, "bottom left")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .child(
                        right_click_menu("test3")
                            .trigger(|_| Label::new("TOP RIGHT"))
                            .anchor(Corner::TopRight)
                            .menu(move |window, cx| build_menu(window, cx, "top right")),
                    )
                    .child(
                        right_click_menu("test4")
                            .trigger(|_| Label::new("BOTTOM RIGHT"))
                            .anchor(Corner::BottomRight)
                            .attach(Corner::TopRight)
                            .menu(move |window, cx| build_menu(window, cx, "bottom right")),
                    ),
            )
    }
}
