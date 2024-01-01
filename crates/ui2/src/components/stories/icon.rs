use gpui::Render;
use story::Story;
use strum::IntoEnumIterator;

use crate::prelude::*;
use crate::{Icon, IconElement};

pub struct IconStory;

impl Render for IconStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl Element {
        let icons = Icon::iter();

        Story::container()
            .child(Story::title_for::<IconElement>())
            .child(Story::label("All Icons"))
            .child(div().flex().gap_3().children(icons.map(IconElement::new)))
    }
}
