use ui::prelude::*;
use ui::AssistantPanel;

use crate::story::Story;

#[derive(Element, Default)]
pub struct AssistantPanelStory {}

impl AssistantPanelStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, AssistantPanel<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(AssistantPanel::new())
    }
}
