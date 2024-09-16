use gpui::Render;
use story::Story;

use crate::prelude::*;
use crate::Disclosure;

pub struct DisclosureStory;

impl Render for DisclosureStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<Disclosure>(cx))
            .child(Story::label(cx, "Toggled"))
            .child(Disclosure::new("toggled", true))
            .child(Story::label(cx, "Not Toggled"))
            .child(Disclosure::new("not_toggled", false))
    }
}
