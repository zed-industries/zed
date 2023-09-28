use std::marker::PhantomData;

use chrono::NaiveDateTime;

use crate::theme::theme;
use crate::{prelude::*, Input, Label};
use crate::{Icon, IconButton};

#[derive(Element)]
pub struct ChatPanel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
    messages: Vec<ChatMessage>,
}

impl<V: 'static> ChatPanel<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
            messages: Vec::new(),
        }
    }

    pub fn with_messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .flex()
            .flex_col()
            .h_full()
            .px_2()
            .gap_2()
            // Header
            .child(
                div()
                    .flex()
                    .justify_between()
                    .gap_2()
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
            // Chat Body
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .children(self.messages.clone()),
            )
            // Composer
            .child(div().flex().gap_2().child(Input::new("Message #design")))
    }
}

#[derive(Element, Clone)]
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

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(Label::new(self.author.clone()))
                    .child(Label::new(self.sent_at.format("%m/%d/%Y").to_string())),
            )
            .child(div().child(Label::new(self.text.clone())))
    }
}
