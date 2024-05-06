use std::sync::Arc;

use client::User;
use gpui::{AnyElement, ClickEvent};
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
    message: Option<AnyElement>,
    tools_used: Option<AnyElement>,
    collapsed: bool,
    on_collapse_handle_click: Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
}

impl ChatMessage {
    pub fn new(
        id: MessageId,
        player: UserOrAssistant,
        message: Option<AnyElement>,
        tools_used: Option<AnyElement>,
        collapsed: bool,
        on_collapse_handle_click: Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    ) -> Self {
        Self {
            id,
            player,
            message,
            tools_used,
            collapsed,
            on_collapse_handle_click,
        }
    }
}

impl RenderOnce for ChatMessage {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let collapse_handle_id = SharedString::from(format!("{}_collapse_handle", self.id.0));
        let collapse_handle = h_flex()
            .id(collapse_handle_id.clone())
            .group(collapse_handle_id.clone())
            .flex_none()
            .justify_center()
            .w_1()
            .mx_2()
            .h_full()
            .on_click(self.on_collapse_handle_click)
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

        let content_padding = rems(1.);
        // Clamp the message height to exactly 1.5 lines when collapsed.
        let collapsed_height = content_padding.to_pixels(cx.rem_size()) + cx.line_height() * 1.5;

        v_flex()
            .gap_1()
            .child(ChatMessageHeader::new(self.player))
            .when(self.message.is_some() || self.tools_used.is_some(), |el| {
                el.child(
                    h_flex().gap_3().child(collapse_handle).child(
                        div()
                            .overflow_hidden()
                            .w_full()
                            .p(content_padding)
                            .gap_3()
                            .rounded_lg()
                            .when(self.collapsed, |this| this.h(collapsed_height))
                            .bg(cx.theme().colors().surface_background)
                            .children(self.message)
                            .children(self.tools_used),
                    ),
                )
            })
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
                        let avatar_size = rems_from_px(20.);
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
