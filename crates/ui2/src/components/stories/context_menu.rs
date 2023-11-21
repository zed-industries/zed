#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{story::Story, Label};
    use gpui::{actions, Div, Render};

    actions!(PrintCurrentDate, PrintBestFood);

    fn build_menu(cx: &mut WindowContext, header: impl Into<SharedString>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _| {
            menu.header(header)
                .separator()
                .entry(
                    ListItem::new("Print current time", Label::new("Print current time")),
                    |v, cx| {
                        println!("dispatching PrintCurrentTime action");
                        cx.dispatch_action(PrintCurrentDate.boxed_clone())
                    },
                )
                .entry(
                    ListItem::new("Print best food", Label::new("Print best food")),
                    |v, cx| cx.dispatch_action(PrintBestFood.boxed_clone()),
                )
        })
    }

    pub struct ContextMenuStory;

    impl Render for ContextMenuStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
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
                            menu_handle("test2")
                                .child(|is_open| {
                                    Label::new(if is_open {
                                        "TOP LEFT"
                                    } else {
                                        "RIGHT CLICK ME"
                                    })
                                })
                                .menu(move |cx| build_menu(cx, "top left")),
                        )
                        .child(
                            menu_handle("test1")
                                .child(|is_open| {
                                    Label::new(if is_open {
                                        "BOTTOM LEFT"
                                    } else {
                                        "RIGHT CLICK ME"
                                    })
                                })
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
                            menu_handle("test3")
                                .child(|is_open| {
                                    Label::new(if is_open {
                                        "TOP RIGHT"
                                    } else {
                                        "RIGHT CLICK ME"
                                    })
                                })
                                .anchor(AnchorCorner::TopRight)
                                .menu(move |cx| build_menu(cx, "top right")),
                        )
                        .child(
                            menu_handle("test4")
                                .child(|is_open| {
                                    Label::new(if is_open {
                                        "BOTTOM RIGHT"
                                    } else {
                                        "RIGHT CLICK ME"
                                    })
                                })
                                .anchor(AnchorCorner::BottomRight)
                                .attach(AnchorCorner::TopRight)
                                .menu(move |cx| build_menu(cx, "bottom right")),
                        ),
                )
        }
    }
}
