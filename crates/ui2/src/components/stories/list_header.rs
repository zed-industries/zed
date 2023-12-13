use gpui::{Div, Render};
use story::Story;

use crate::{prelude::*, IconButton};
use crate::{Icon, ListHeader};

pub struct ListHeaderStory;

impl Render for ListHeaderStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<ListHeader>())
            .child(Story::label("Default"))
            .child(ListHeader::new("Section 1"))
            .child(Story::label("With left icon"))
            .child(ListHeader::new("Section 2").start_slot(IconElement::new(Icon::Bell)))
            .child(Story::label("With left icon and meta"))
            .child(
                ListHeader::new("Section 3")
                    .start_slot(IconElement::new(Icon::BellOff))
                    .end_slot(IconButton::new("action_1", Icon::Bolt)),
            )
            .child(Story::label("With multiple meta"))
            .child(
                ListHeader::new("Section 4")
                    .end_slot(IconButton::new("action_1", Icon::Bolt))
                    .end_slot(IconButton::new("action_2", Icon::ExclamationTriangle))
                    .end_slot(IconButton::new("action_3", Icon::Plus)),
            )
    }
}
