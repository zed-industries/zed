use gpui::Render;
use story::Story;

use crate::{prelude::*, IconButton};
use crate::{IconName, ListHeader};

pub struct ListHeaderStory;

impl Render for ListHeaderStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<ListHeader>(cx))
            .child(Story::label(cx, "Default"))
            .child(ListHeader::new("Section 1"))
            .child(Story::label(cx, "With left icon"))
            .child(ListHeader::new("Section 2").start_slot(Icon::new(IconName::Bell)))
            .child(Story::label(cx, "With left icon and meta"))
            .child(
                ListHeader::new("Section 3")
                    .start_slot(Icon::new(IconName::BellOff))
                    .end_slot(IconButton::new("action_1", IconName::Bolt)),
            )
            .child(Story::label(cx, "With multiple meta"))
            .child(
                ListHeader::new("Section 4")
                    .end_slot(IconButton::new("action_1", IconName::Bolt))
                    .end_slot(IconButton::new("action_2", IconName::Warning))
                    .end_slot(IconButton::new("action_3", IconName::Plus)),
            )
    }
}
