use gpui::Render;
use story::Story;

use crate::{prelude::*, IconButton};
use crate::{IconPath, ListHeader};

pub struct ListHeaderStory;

impl Render for ListHeaderStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<ListHeader>())
            .child(Story::label("Default"))
            .child(ListHeader::new("Section 1"))
            .child(Story::label("With left icon"))
            .child(ListHeader::new("Section 2").start_slot(Icon::new(IconPath::Bell)))
            .child(Story::label("With left icon and meta"))
            .child(
                ListHeader::new("Section 3")
                    .start_slot(Icon::new(IconPath::BellOff))
                    .end_slot(IconButton::new("action_1", IconPath::Bolt)),
            )
            .child(Story::label("With multiple meta"))
            .child(
                ListHeader::new("Section 4")
                    .end_slot(IconButton::new("action_1", IconPath::Bolt))
                    .end_slot(IconButton::new("action_2", IconPath::ExclamationTriangle))
                    .end_slot(IconButton::new("action_3", IconPath::Plus)),
            )
    }
}
