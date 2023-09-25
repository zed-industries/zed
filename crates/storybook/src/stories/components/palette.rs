use gpui2::{elements::div::ScrollState, Element, IntoElement, ParentElement, ViewContext};
use ui::Palette;

use crate::story::Story;

#[derive(Element, Default)]
pub struct PaletteStory {}

impl PaletteStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Palette<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Palette::new(ScrollState::default()))
    }
}
