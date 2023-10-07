use ui::prelude::*;
use ui::ThemeSelector;

use crate::story::Story;

#[derive(Element, Default)]
pub struct ThemeSelectorStory {}

impl ThemeSelectorStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, ThemeSelector>(cx))
            .child(Story::label(cx, "Default"))
            .child(ThemeSelector::new())
    }
}
