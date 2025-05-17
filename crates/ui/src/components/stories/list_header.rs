use gpui::Render;
use story::Story;

use crate::{IconButton, prelude::*};
use crate::{IconName, ListHeader};

pub struct ListHeaderStory;

impl Render for ListHeaderStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<ListHeader>(cx))
            .child(Story::label("Default", cx))
            .child(ListHeader::new("Section 1"))
            .child(Story::label("With left icon", cx))
            .child(ListHeader::new("Section 2").start_slot(Icon::new(IconName::Bell)))
            .child(Story::label("With left icon and meta", cx))
            .child(
                ListHeader::new("Section 3")
                    .start_slot(Icon::new(IconName::BellOff))
                    .end_slot(IconButton::new("action_1", IconName::Bolt)),
            )
            .child(Story::label("With multiple meta", cx))
            .child(
                ListHeader::new("Section 4")
                    .end_slot(IconButton::new("action_1", IconName::Bolt))
                    .end_slot(IconButton::new("action_2", IconName::Warning))
                    .end_slot(IconButton::new("action_3", IconName::Plus)),
            )
    }
}
