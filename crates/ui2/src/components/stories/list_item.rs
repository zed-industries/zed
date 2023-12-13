use gpui::{Div, Render};
use story::Story;

use crate::{prelude::*, Avatar};
use crate::{Icon, ListItem};

pub struct ListItemStory;

impl Render for ListItemStory {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .bg(cx.theme().colors().background)
            .child(Story::title_for::<ListItem>())
            .child(Story::label("Default"))
            .child(ListItem::new("hello_world").child("Hello, world!"))
            .child(Story::label("Inset"))
            .child(
                ListItem::new("hello_world")
                    .inset(true)
                    .start_slot(
                        IconElement::new(Icon::Bell)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child("Hello, world!")
                    .end_slot(
                        IconElement::new(Icon::Bell)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(Story::label("With start slot icon"))
            .child(
                ListItem::new("with start slot_icon")
                    .child("Hello, world!")
                    .start_slot(
                        IconElement::new(Icon::Bell)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(Story::label("With start slot avatar"))
            .child(
                ListItem::new("with_start slot avatar")
                    .child("Hello, world!")
                    .start_slot(Avatar::new(SharedString::from(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    ))),
            )
            .child(Story::label("With end slot"))
            .child(
                ListItem::new("with_left_avatar")
                    .child("Hello, world!")
                    .end_slot(Avatar::new(SharedString::from(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    ))),
            )
            .child(Story::label("With end hover slot"))
            .child(
                ListItem::new("with_left_avatar")
                    .child("Hello, world!")
                    .end_slot(
                        h_stack()
                            .gap_2()
                            .child(Avatar::new(SharedString::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedString::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedString::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedString::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            )))
                            .child(Avatar::new(SharedString::from(
                                "https://avatars.githubusercontent.com/u/1789?v=4",
                            ))),
                    )
                    .end_hover_slot(Avatar::new(SharedString::from(
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
