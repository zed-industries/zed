use gpui::{actions, AnchorCorner, Model, Render};
use story::Story;

use crate::prelude::*;
use crate::{right_click_menu, ContextMenu, Label};

actions!(context_menu, [PrintCurrentDate, PrintBestFood]);

fn build_menu(
    header: impl Into<SharedString>,
    window: &mut gpui::Window,
    cx: &mut gpui::AppContext,
) -> gpui::Model<ContextMenu> {
    ContextMenu::build(window, cx, |menu, model, window, cx| {
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
    fn render(
        &mut self,
        model: &Model<Self>,
        _window: &mut gpui::Window,
        _cx: &mut AppContext,
    ) -> impl IntoElement {
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
                            .trigger(Label::new("TOP LEFT"))
                            .menu(move |window, cx| build_menu("top left", window, cx)),
                    )
                    .child(
                        right_click_menu("test1")
                            .trigger(Label::new("BOTTOM LEFT"))
                            .anchor(AnchorCorner::BottomLeft)
                            .attach(AnchorCorner::TopLeft)
                            .menu(move |window, cx| build_menu("bottom left", window, cx)),
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
                            .menu(move |window, cx| build_menu("top right", window, cx)),
                    )
                    .child(
                        right_click_menu("test4")
                            .trigger(Label::new("BOTTOM RIGHT"))
                            .anchor(AnchorCorner::BottomRight)
                            .attach(AnchorCorner::TopRight)
                            .menu(move |window, cx| build_menu("bottom right", window, cx)),
                    ),
            )
    }
}
