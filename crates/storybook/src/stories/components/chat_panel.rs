use ui::prelude::*;
use ui::ChatMessage;
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
            .child(Story::label(cx, "With Mesages"))
            .child(ChatPanel::new(ScrollState::default()).with_messages(vec![
                ChatMessage::new(
                    "osiewicz".to_string(),
                    "is this thing on?".to_string(),
                    "09/25/2023".to_string(),
                ),
                ChatMessage::new(
                    "maxdeviant".to_string(),
                    "Reading you loud and clear!".to_string(),
                    "09/25/2023".to_string(),
                ),
            ]))
    }
}
