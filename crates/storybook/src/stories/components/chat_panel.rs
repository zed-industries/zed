use ui::prelude::*;
use ui::ChatPanel;

use crate::story::Story;

#[derive(Element, Default)]
pub struct ChatPanelStory {}

impl ChatPanelStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, ChatPanel<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(ChatPanel::new(ScrollState::default()))
    }
}
