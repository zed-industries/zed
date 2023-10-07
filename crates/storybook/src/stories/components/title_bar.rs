use ui::prelude::*;
use ui::TitleBar;

use crate::story::Story;

#[derive(Element, Default)]
pub struct TitleBarStory {}

impl TitleBarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, TitleBar<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(TitleBar::new(cx))
    }
}
