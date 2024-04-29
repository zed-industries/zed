use std::sync::Arc;

use client::User;
use gpui::AnyElement;
use ui::{prelude::*, Avatar};

use crate::MessageId;

pub enum UserOrAssistant {
    User(Option<Arc<User>>),
    Assistant,
}

#[derive(IntoElement)]
pub struct ChatMessage {
    id: MessageId,
    player: UserOrAssistant,
    message: AnyElement,
    collapsed: bool,
    on_collapse: Box<dyn Fn(bool, &mut WindowContext) + 'static>,
}

impl ChatMessage {
    pub fn new(
        id: MessageId,
        player: UserOrAssistant,
        message: AnyElement,
        collapsed: bool,
        on_collapse: Box<dyn Fn(bool, &mut WindowContext) + 'static>,
    ) -> Self {
        Self {
            id,
            player,
            message,
            collapsed,
            on_collapse,
        }
    }
}

impl RenderOnce for ChatMessage {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        // TODO: This should be top padding + 1.5x line height
        // Set the message height to cut off at exactly 1.5 lines when collapsed
        let collapsed_height = rems(2.875);

        let collapse_handle_id = SharedString::from(format!("{}_collapse_handle", self.id.0));
        let collapse_handle = h_flex()
            .id(collapse_handle_id.clone())
            .group(collapse_handle_id.clone())
            .flex_none()
            .justify_center()
            .w_1()
            .mx_2()
            .h_full()
            .on_click(move |_event, cx| (self.on_collapse)(!self.collapsed, cx))
            .child(
                div()
                    .w_px()
                    .h_full()
                    .rounded_lg()
                    .overflow_hidden()
                    .bg(cx.theme().colors().element_background)
                    .group_hover(collapse_handle_id, |this| {
                        this.bg(cx.theme().colors().element_hover)
                    }),
            );
        let content = div()
            .overflow_hidden()
            .w_full()
            .p_4()
            .rounded_lg()
            .when(self.collapsed, |this| this.h(collapsed_height))
            .bg(cx.theme().colors().surface_background)
            .child(self.message);

        v_flex()
            .gap_1()
            .child(ChatMessageHeader::new(self.player))
            .child(h_flex().gap_3().child(collapse_handle).child(content))
    }
}

#[derive(IntoElement)]
struct ChatMessageHeader {
    player: UserOrAssistant,
    contexts: Vec<()>,
}

impl ChatMessageHeader {
    fn new(player: UserOrAssistant) -> Self {
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
