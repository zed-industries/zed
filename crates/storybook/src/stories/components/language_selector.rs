use ui::prelude::*;
use ui::LanguageSelector;

use crate::story::Story;

#[derive(Element, Default)]
pub struct LanguageSelectorStory {}

impl LanguageSelectorStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, LanguageSelector>(cx))
            .child(Story::label(cx, "Default"))
            .child(LanguageSelector::new())
    }
}
