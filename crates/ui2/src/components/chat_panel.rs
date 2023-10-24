use std::marker::PhantomData;

use chrono::NaiveDateTime;

use crate::prelude::*;
use crate::{Icon, IconButton, Input, Label, LabelColor};

#[derive(IntoAnyElement)]
pub struct ChatPanel {
    element_id: ElementId,
    messages: Vec<ChatMessage>,
}

impl ChatPanel {
    pub fn new(element_id: impl Into<ElementId>) -> Self {
        Self {
            element_id: element_id.into(),
            messages: Vec::new(),
        }
    }

    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    fn render<S: 'static + Send + Sync>(self) -> impl IntoAnyElement<S> {
        div()
            .id(self.element_id)
            .flex()
            .flex_col()
            .justify_between()
            .h_full()
            .px_2()
            .gap_2()
            // Header
            .child(
                div()
                    .flex()
                    .justify_between()
                    .py_2()
                    .child(div().flex().child(Label::new("#design")))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(IconButton::new(Icon::File))
                            .child(IconButton::new(Icon::AudioOn)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    // Chat Body
                    .child(
                        div()
                            .id("chat-body")
                            .w_full()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .overflow_y_scroll()
                            .children(self.messages),
                    )
                    // Composer
                    .child(div().flex().my_2().child(Input::new("Message #design"))),
            )
    }
}

#[derive(IntoAnyElement)]
pub struct ChatMessage {
    author: String,
    text: String,
    sent_at: NaiveDateTime,
}

impl ChatMessage {
    pub fn new(author: String, text: String, sent_at: NaiveDateTime) -> Self {
        Self {
            author,
            text,
            sent_at,
        }
    }

    fn render<S: 'static + Send + Sync>(mut self) -> impl Element<ViewState = S> {
        div()
            .flex()
            .flex_col()
            .child(div().flex().gap_2().child(Label::new(self.author)).child(
                Label::new(self.sent_at.format("%m/%d/%Y").to_string()).color(LabelColor::Muted),
            ))
            .child(div().child(Label::new(self.text)))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use chrono::DateTime;

    use crate::{Panel, Story};

    use super::*;

    #[derive(Element)]
    pub struct ChatPanelStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> ChatPanelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, ChatPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new("chat-panel-1-outer", cx)
                        .child(ChatPanel::new("chat-panel-1-inner")),
                )
                .child(Story::label(cx, "With Mesages"))
                .child(Panel::new("chat-panel-2-outer", cx).child(
                    ChatPanel::new("chat-panel-2-inner").messages(vec![
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
                    ]),
                ))
        }
    }
}
