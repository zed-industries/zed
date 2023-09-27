use ui::prelude::*;
use ui::TabBar;

use crate::story::Story;

#[derive(Element, Default)]
pub struct TabBarStory {}

impl TabBarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, TabBar<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(TabBar::new(ScrollState::default()))
    }
}
