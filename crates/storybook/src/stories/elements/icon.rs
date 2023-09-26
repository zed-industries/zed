use strum::IntoEnumIterator;
use ui::prelude::*;
use ui::{Icon, IconAsset};

use crate::story::Story;

#[derive(Element, Default)]
pub struct IconStory {}

impl IconStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let icons = IconAsset::iter();

        Story::container(cx)
            .child(Story::title_for::<_, ui::Icon>(cx))
            .child(Story::label(cx, "All Icons"))
            .child(div().flex().gap_3().children(icons.map(Icon::new)))
    }
}
