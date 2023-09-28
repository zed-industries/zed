use ui::prelude::*;
use ui::StatusBar;

use crate::story::Story;

#[derive(Element, Default)]
pub struct StatusBarStory {}

impl StatusBarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, StatusBar<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(StatusBar::new())
    }
}
