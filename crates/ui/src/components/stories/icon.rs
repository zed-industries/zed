use gpui::Render;
use story::Story;
use strum::IntoEnumIterator;

use crate::prelude::*;
use crate::{IconPath, Icon};

pub struct IconStory;

impl Render for IconStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let icons = IconPath::iter();

        Story::container()
            .child(Story::title_for::<Icon>())
            .child(Story::label("All Icons"))
            .child(div().flex().gap_3().children(icons.map(Icon::new)))
    }
}
