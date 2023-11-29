use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::{Icon, ListItem};

pub struct ListItemStory;

impl Render for ListItemStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<ListItem>())
            .child(Story::label("Default"))
            .child(ListItem::new("hello_world").child("Hello, world!"))
            .child(Story::label("With left icon"))
            .child(
                ListItem::new("with_left_icon")
                    .child("Hello, world!")
                    .left_icon(Icon::Bell),
            )
            .child(Story::label("With left avatar"))
            .child(
                ListItem::new("with_left_avatar")
                    .child("Hello, world!")
                    .left_avatar(SharedString::from(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                    )),
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
