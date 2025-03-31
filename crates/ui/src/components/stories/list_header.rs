use gpui::Render;
use story::Story;

use crate::{IconButton, prelude::*};
use crate::{IconName, ListHeader};

pub struct ListHeaderStory;

impl Render for ListHeaderStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<ListHeader>())
            .child(Story::label("Default"))
            .child(ListHeader::new("Section 1"))
            .child(Story::label("With left icon"))
            .child(ListHeader::new("Section 2").start_slot(Icon::new(IconName::Bell)))
            .child(Story::label("With left icon and meta"))
            .child(
                ListHeader::new("Section 3")
                    .start_slot(Icon::new(IconName::BellOff))
                    .end_slot(IconButton::new("action_1", IconName::Bolt)),
            )
            .child(Story::label("With multiple meta"))
            .child(
                ListHeader::new("Section 4")
                    .end_slot(IconButton::new("action_1", IconName::Bolt))
                    .end_slot(IconButton::new("action_2", IconName::Warning))
                    .end_slot(IconButton::new("action_3", IconName::Plus)),
            )
    }
}
