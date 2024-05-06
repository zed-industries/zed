use std::sync::Arc;

use client::User;
use story::{StoryContainer, StoryItem, StorySection};
use ui::prelude::*;

use crate::ui::{ChatMessage, UserOrAssistant};
use crate::MessageId;

pub struct ChatMessageStory;

impl Render for ChatMessageStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let user_1 = Arc::new(User {
            id: 12345,
            github_login: "iamnbutler".into(),
            avatar_uri: "https://avatars.githubusercontent.com/u/1714999?v=4".into(),
        });

        StoryContainer::new(
            "ChatMessage Story",
            "crates/assistant2/src/ui/stories/chat_message.rs",
        )
        .child(
            StorySection::new()
                .child(StoryItem::new(
                    "User chat message",
                    ChatMessage::new(
                        MessageId(0),
                        UserOrAssistant::User(Some(user_1.clone())),
                        Some(div().child("What can I do here?").into_any_element()),
                        None,
                        false,
                        Box::new(|_, _| {}),
                    ),
                ))
                .child(StoryItem::new(
                    "User chat message (collapsed)",
                    ChatMessage::new(
                        MessageId(0),
                        UserOrAssistant::User(Some(user_1.clone())),
                        Some(div().child("What can I do here?").into_any_element()),
                        None,
                        true,
                        Box::new(|_, _| {}),
                    ),
                )),
        )
        .child(
            StorySection::new()
                .child(StoryItem::new(
                    "Assistant chat message",
                    ChatMessage::new(
                        MessageId(0),
                        UserOrAssistant::Assistant,
                        Some(div().child("You can talk to me!").into_any_element()),
                        None,
                        false,
                        Box::new(|_, _| {}),
                    ),
                ))
                .child(StoryItem::new(
                    "Assistant chat message (collapsed)",
                    ChatMessage::new(
                        MessageId(0),
                        UserOrAssistant::Assistant,
                        Some(div().child(MULTI_LINE_MESSAGE).into_any_element()),
                        None,
                        true,
                        Box::new(|_, _| {}),
                    ),
                )),
        )
        .child(
            StorySection::new().child(StoryItem::new(
                "Conversation between user and assistant",
                v_flex()
                    .gap_2()
                    .child(ChatMessage::new(
                        MessageId(0),
                        UserOrAssistant::User(Some(user_1.clone())),
                        Some(div().child("What is Rust??").into_any_element()),
                        None,
                        false,
                        Box::new(|_, _| {}),
                    ))
                    .child(ChatMessage::new(
                        MessageId(0),
                        UserOrAssistant::Assistant,
                        Some(div().child("Rust is a multi-paradigm programming language focused on performance and safety").into_any_element()),
                        None,
                        false,
                        Box::new(|_, _| {}),
                    ))
                    .child(ChatMessage::new(
                        MessageId(0),
                        UserOrAssistant::User(Some(user_1)),
                        Some(div().child("Sounds pretty cool!").into_any_element()),
                        None,
                        false,
                        Box::new(|_, _| {}),
                    )),
            )),
        )
    }
}

const MULTI_LINE_MESSAGE: &str = "In 2010, the movies nominated for the 82nd Academy Awards, for films released in 2009, were as follows. Note that 2010 nominees were announced for the ceremony happening in that year, but they honor movies from the previous year";
