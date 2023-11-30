use gpui::{Div, Render};
use story::Story;

use crate::{prelude::*, ListHeader, ListSeparator, ListSubHeader};
use crate::{List, ListItem};

pub struct ListStory;

impl Render for ListStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<List>())
            .child(Story::label("Default"))
            .child(
                List::new()
                    .child(ListItem::new("apple").child("Apple"))
                    .child(ListItem::new("banana").child("Banana"))
                    .child(ListItem::new("cherry").child("Cherry")),
            )
            .child(Story::label("With sections"))
            .child(
                List::new()
                    .child(ListHeader::new("Fruits"))
                    .child(ListItem::new("apple").child("Apple"))
                    .child(ListItem::new("banana").child("Banana"))
                    .child(ListItem::new("cherry").child("Cherry"))
                    .child(ListSeparator)
                    .child(ListHeader::new("Vegetables"))
                    .child(ListSubHeader::new("Root Vegetables"))
                    .child(ListItem::new("carrot").child("Carrot"))
                    .child(ListItem::new("potato").child("Potato"))
                    .child(ListSubHeader::new("Leafy Vegetables"))
                    .child(ListItem::new("kale").child("Kale")),
            )
    }
}
