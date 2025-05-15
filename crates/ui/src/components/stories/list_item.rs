use gpui::Render;
use story::Story;

use crate::{Avatar, prelude::*};
use crate::{IconName, ListItem};

const OVERFLOWING_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean mauris ligula, luctus vel dignissim eu, vestibulum sed libero. Sed at convallis velit.";

pub struct ListItemStory;

impl Render for ListItemStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .bg(cx.theme().colors().background)
            .child(Story::title_for::<ListItem>(cx))
            .child(Story::label("Default", cx))
            .child(ListItem::new("hello_world").child("Hello, world!"))
            .child(Story::label("Inset", cx))
            .child(
                ListItem::new("inset_list_item")
                    .inset(true)
                    .start_slot(
                        Icon::new(IconName::Bell)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child("Hello, world!")
                    .end_slot(
                        Icon::new(IconName::Bell)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(Story::label("With start slot icon", cx))
            .child(
                ListItem::new("with start slot_icon")
                    .child("Hello, world!")
                    .start_slot(
                        Icon::new(IconName::Bell)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(Story::label("With start slot avatar", cx))
            .child(
                ListItem::new("with_start slot avatar")
                    .child("Hello, world!")
                    .start_slot(Avatar::new(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    )),
            )
            .child(Story::label("With end slot", cx))
            .child(
                ListItem::new("with_left_avatar")
                    .child("Hello, world!")
                    .end_slot(Avatar::new(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    )),
            )
            .child(Story::label("With end hover slot", cx))
            .child(
                ListItem::new("with_end_hover_slot")
                    .child("Hello, world!")
                    .end_slot(
                        h_flex()
                            .gap_2()
                            .child(Avatar::new(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            ))
                            .child(Avatar::new(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            ))
                            .child(Avatar::new(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            ))
                            .child(Avatar::new(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            ))
                            .child(Avatar::new(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )),
                    )
                    .end_hover_slot(Avatar::new(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    )),
            )
            .child(Story::label("With `on_click`", cx))
            .child(ListItem::new("with_on_click").child("Click me").on_click(
                |_event, _window, _cx| {
                    println!("Clicked!");
                },
            ))
            .child(Story::label("With `on_secondary_mouse_down`", cx))
            .child(
                ListItem::new("with_on_secondary_mouse_down")
                    .child("Right click me")
                    .on_secondary_mouse_down(|_event, _window, _cx| {
                        println!("Right mouse down!");
                    }),
            )
            .child(Story::label(
                "With overflowing content in the `end_slot`",
                cx,
            ))
            .child(
                ListItem::new("with_overflowing_content_in_end_slot")
                    .child("An excerpt")
                    .end_slot(Label::new(OVERFLOWING_TEXT).color(Color::Muted)),
            )
            .child(Story::label(
                "`inset` with overflowing content in the `end_slot`",
                cx,
            ))
            .child(
                ListItem::new("inset_with_overflowing_content_in_end_slot")
                    .inset(true)
                    .child("An excerpt")
                    .end_slot(Label::new(OVERFLOWING_TEXT).color(Color::Muted)),
            )
            .child(Story::label(
                "`inset` with overflowing content in `children` and `end_slot`",
                cx,
            ))
            .child(
                ListItem::new("inset_with_overflowing_content_in_children_and_end_slot")
                    .inset(true)
                    .child(Label::new(OVERFLOWING_TEXT))
                    .end_slot(Label::new(OVERFLOWING_TEXT).color(Color::Muted)),
            )
    }
}
