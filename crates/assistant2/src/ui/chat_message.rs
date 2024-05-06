use std::sync::Arc;

use client::User;
use gpui::{hsla, AnyElement, ClickEvent};
use ui::{prelude::*, Avatar, Tooltip};

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
        let message_group = SharedString::from(format!("{}_group", self.id.0));

        let collapse_handle_id = SharedString::from(format!("{}_collapse_handle", self.id.0));

        let content_padding = Spacing::Large.rems(cx);
        // Clamp the message height to exactly 1.5 lines when collapsed.
        let collapsed_height = content_padding.to_pixels(cx.rem_size()) + cx.line_height() * 1.5;

        let background_color = if let UserOrAssistant::User(_) = &self.player {
            Some(cx.theme().colors().surface_background)
        } else {
            None
        };

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

        v_flex()
            .group(message_group.clone())
            .w_full()
            .flex_none()
            .gap(Spacing::Small.rems(cx))
            .py(Spacing::Small.rems(cx))
            .child(
                h_flex()
                    .justify_between()
                    .w_full()
                    .flex_none()
                    .child(
                        h_flex()
                            .gap_2()
                            .map(|this| {
                                let avatar_size = rems_from_px(20.);
                                if let Some(avatar_uri) = avatar_uri {
                                    this.child(Avatar::new(avatar_uri).size(avatar_size))
                                } else {
                                    this.child(div().size(avatar_size))
                                }
                            })
                            .child(Label::new(username).color(Color::Muted)),
                    )
                    .child(
                        h_flex()
                            .visible_on_hover(message_group)
                            .child(
                                IconButton::new(
                                    collapse_handle_id.clone(),
                                    if self.collapsed.clone() {
                                        IconName::ArrowUp
                                    } else {
                                        IconName::ArrowDown
                                    },
                                )
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Muted)
                                .on_click(self.on_collapse_handle_click)
                                .tooltip(|cx| Tooltip::text("Collapse Message", cx)),
                            )
                            .child(
                                IconButton::new("copy-message", IconName::Copy)
                                    .icon_color(Color::Muted)
                                    .icon_size(IconSize::XSmall),
                            )
                            .child(
                                IconButton::new("menu", IconName::Ellipsis)
                                    .icon_color(Color::Muted)
                                    .icon_size(IconSize::XSmall),
                            ),
                    ),
            )
            .when(self.message.is_some() || self.tools_used.is_some(), |el| {
                el.child(
                    h_flex().child(
                        div()
                            .relative()
                            .overflow_y_hidden()
                            .w_full()
                            .p(content_padding)
                            .gap_3()
                            .rounded_lg()
                            .when_some(background_color, |this, background_color| {
                                this.bg(background_color)
                            })
                            .when(self.collapsed, |this| this.h(collapsed_height))
                            .children(self.message)
                            .when_some(self.tools_used, |this, tools_used| this.child(tools_used)),
                    ),
                )
            })
    }
}
