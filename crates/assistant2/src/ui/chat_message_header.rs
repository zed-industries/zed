use client::User;
use std::sync::Arc;
use ui::{prelude::*, Avatar};

pub enum UserOrAssistant {
    User(Option<Arc<User>>),
    Assistant,
}

#[derive(IntoElement)]
pub struct ChatMessageHeader {
    player: UserOrAssistant,
    contexts: Vec<()>,
}

impl ChatMessageHeader {
    pub fn new(player: UserOrAssistant) -> Self {
        Self {
            player,
            contexts: Vec::new(),
        }
    }
}

impl RenderOnce for ChatMessageHeader {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let (username, avatar_uri) = match self.player {
            UserOrAssistant::Assistant => (
                "Assistant".into(),
                Some("https://zed.dev/assistant_avatar.png".into()),
            ),
            UserOrAssistant::User(Some(user)) => {
                (user.github_login.clone(), Some(user.avatar_uri.clone()))
            }
            UserOrAssistant::User(None) => ("You".into(), None),
        };

        h_flex()
            .justify_between()
            .child(
                h_flex()
                    .gap_3()
                    .map(|this| {
                        let avatar_size = rems(20.0 / 16.0);
                        if let Some(avatar_uri) = avatar_uri {
                            this.child(Avatar::new(avatar_uri).size(avatar_size))
                        } else {
                            this.child(div().size(avatar_size))
                        }
                    })
                    .child(Label::new(username).color(Color::Default)),
            )
            .child(div().when(!self.contexts.is_empty(), |this| {
                this.child(Label::new(self.contexts.len().to_string()).color(Color::Muted))
            }))
    }
}
