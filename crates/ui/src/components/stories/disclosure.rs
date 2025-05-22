use gpui::Render;
use story::Story;

use crate::Disclosure;
use crate::prelude::*;

pub struct DisclosureStory;

impl Render for DisclosureStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<Disclosure>(cx))
            .child(Story::label("Toggled"))
            .child(Disclosure::new("toggled", true))
            .child(Story::label("Not Toggled"))
            .child(Disclosure::new("not_toggled", false))
    }
}
