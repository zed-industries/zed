use std::marker::PhantomData;

use chrono::NaiveDateTime;

use crate::prelude::*;
use crate::theme::theme;
use crate::{Icon, IconButton, Input, Label, LabelColor};

#[derive(Element)]
pub struct ChatPanel<S: 'static + Send + Sync + Clone> {
    scroll_state: ScrollState,
    messages: Vec<ChatMessage<S>>,
}

impl<S: 'static + Send + Sync + Clone> ChatPanel<S> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            scroll_state,
            messages: Vec::new(),
        }
    }

    pub fn with_messages(mut self, messages: Vec<ChatMessage<S>>) -> Self {
        self.messages = messages;
        self
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        div()
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
                            .w_full()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .overflow_y_scroll(self.scroll_state.clone())
                            .children(self.messages.clone()),
                    )
                    // Composer
                    .child(div().flex().my_2().child(Input::new("Message #design"))),
            )
    }
}

#[derive(Element, Clone)]
pub struct ChatMessage<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    author: String,
    text: String,
    sent_at: NaiveDateTime,
}

impl<S: 'static + Send + Sync + Clone> ChatMessage<S> {
    pub fn new(author: String, text: String, sent_at: NaiveDateTime) -> Self {
        Self {
            state_type: PhantomData,
            author,
            text,
            sent_at,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(Label::new(self.author.clone()))
                    .child(
                        Label::new(self.sent_at.format("%m/%d/%Y").to_string())
                            .color(LabelColor::Muted),
                    ),
            )
            .child(div().child(Label::new(self.text.clone())))
    }
}
