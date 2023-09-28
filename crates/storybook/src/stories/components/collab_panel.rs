use ui::prelude::*;
use ui::CollabPanel;

use crate::story::Story;

#[derive(Element, Default)]
pub struct CollabPanelStory {}

impl CollabPanelStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, CollabPanel<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(CollabPanel::new(ScrollState::default()))
    }
}
