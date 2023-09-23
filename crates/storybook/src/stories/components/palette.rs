use gpui2::{elements::div::ScrollState, Element, IntoElement, ParentElement, ViewContext};
use ui::palette;

use crate::story::Story;

#[derive(Element, Default)]
pub struct PaletteStory {}

impl PaletteStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container()
            .child(Story::title_for::<_, ui::Palette<V>>())
            .child(Story::label("Default"))
            .child(palette(ScrollState::default()))
    }
}
