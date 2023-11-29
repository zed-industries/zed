use gpui::{Div, Render};
use story::Story;

use crate::prelude::*;
use crate::{Disclosure, Toggle};

pub struct DisclosureStory;

impl Render for DisclosureStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        Story::container()
            .child(Story::title_for::<Disclosure>())
            .child(Story::label("Toggled"))
            .child(Disclosure::new(Toggle::Toggled(true)))
            .child(Story::label("Not Toggled"))
            .child(Disclosure::new(Toggle::Toggled(false)))
            .child(Story::label("Not Toggleable"))
            .child(Disclosure::new(Toggle::NotToggleable))
    }
}
