use gpui::Render;
use story::Story;

use crate::prelude::*;
use crate::Disclosure;

pub struct DisclosureStory;

impl Render for DisclosureStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<Disclosure>())
            .child(Story::label("Toggled"))
            .child(Disclosure::new("toggled", true))
            .child(Story::label("Not Toggled"))
            .child(Disclosure::new("not_toggled", false))
    }
}
