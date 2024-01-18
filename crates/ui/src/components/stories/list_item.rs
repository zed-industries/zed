use gpui::{Render, SharedUrl};
use story::Story;

use crate::{prelude::*, Avatar};
use crate::{IconName, ListItem};

pub struct ListItemStory;

impl Render for ListItemStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .bg(cx.theme().colors().background)
            .child(Story::title_for::<ListItem>())
            .child(Story::label("Default"))
            .child(ListItem::new("hello_world").child("Hello, world!"))
            .child(Story::label("Inset"))
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
            .child(Story::label("With start slot icon"))
            .child(
                ListItem::new("with start slot_icon")
                    .child("Hello, world!")
                    .start_slot(
                        Icon::new(IconName::Bell)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(Story::label("With start slot avatar"))
            .child(
                ListItem::new("with_start slot avatar")
                    .child("Hello, world!")
                    .start_slot(Avatar::new(SharedUrl::from(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    ))),
            )
            .child(Story::label("With end slot"))
            .child(
                ListItem::new("with_left_avatar")
                    .child("Hello, world!")
                    .end_slot(Avatar::new(SharedUrl::from(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    ))),
            )
            .child(Story::label("With end hover slot"))
            .child(
                ListItem::new("with_end_hover_slot")
                    .child("Hello, world!")
                    .end_slot(
                        h_flex()
                            .gap_2()
                            .child(Avatar::new(SharedUrl::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedUrl::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedUrl::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedUrl::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedUrl::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            ))),
                    )
                    .end_hover_slot(Avatar::new(SharedUrl::from(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    ))),
            )
            .child(Story::label("With `on_click`"))
            .child(
                ListItem::new("with_on_click")
                    .child("Click me")
                    .on_click(|_event, _cx| {
                        println!("Clicked!");
                    }),
            )
            .child(Story::label("With `on_secondary_mouse_down`"))
            .child(
                ListItem::new("with_on_secondary_mouse_down")
                    .child("Right click me")
                    .on_secondary_mouse_down(|_event, _cx| {
                        println!("Right mouse down!");
                    }),
            )
    }
}
