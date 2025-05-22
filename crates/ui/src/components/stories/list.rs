use gpui::Render;
use story::Story;

use crate::{List, ListItem};
use crate::{ListHeader, ListSeparator, ListSubHeader, prelude::*};

pub struct ListStory;

impl Render for ListStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<List>(cx))
            .child(Story::label("Default", cx))
            .child(
                List::new()
                    .child(ListItem::new("apple").child("Apple"))
                    .child(ListItem::new("banana").child("Banana"))
                    .child(ListItem::new("cherry").child("Cherry")),
            )
            .child(Story::label("With sections", cx))
            .child(
                List::new()
                    .header(ListHeader::new("Produce"))
                    .child(ListSubHeader::new("Fruits"))
                    .child(ListItem::new("apple").child("Apple"))
                    .child(ListItem::new("banana").child("Banana"))
                    .child(ListItem::new("cherry").child("Cherry"))
                    .child(ListSeparator)
                    .child(ListSubHeader::new("Root Vegetables"))
                    .child(ListItem::new("carrot").child("Carrot"))
                    .child(ListItem::new("potato").child("Potato"))
                    .child(ListSubHeader::new("Leafy Vegetables"))
                    .child(ListItem::new("kale").child("Kale")),
            )
    }
}
