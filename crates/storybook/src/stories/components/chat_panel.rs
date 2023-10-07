use chrono::DateTime;
use ui::prelude::*;
use ui::{ChatMessage, ChatPanel, Panel};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ChatPanelStory {}

impl ChatPanelStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, ChatPanel<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Panel::new(
                ScrollState::default(),
                |_, _| vec![ChatPanel::new(ScrollState::default()).into_any()],
                Box::new(()),
            ))
            .child(Story::label(cx, "With Mesages"))
            .child(Panel::new(
                ScrollState::default(),
                |_, _| {
                    vec![ChatPanel::new(ScrollState::default())
                        .with_messages(vec![
                            ChatMessage::new(
                                "osiewicz".to_string(),
                                "is this thing on?".to_string(),
                                DateTime::parse_from_rfc3339("2023-09-27T15:40:52.707Z")
                                    .unwrap()
                                    .naive_local(),
                            ),
                            ChatMessage::new(
                                "maxdeviant".to_string(),
                                "Reading you loud and clear!".to_string(),
                                DateTime::parse_from_rfc3339("2023-09-28T15:40:52.707Z")
                                    .unwrap()
                                    .naive_local(),
                            ),
                        ])
                        .into_any()]
                },
                Box::new(()),
            ))
    }
}
