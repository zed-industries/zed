use crate::render_icon_button;
use client::User;
use gpui::{
    elements::{Flex, Image, Label, MouseEventHandler, Padding, ParentElement, Text},
    platform::CursorStyle,
    Action, Element, ElementBox, RenderContext, View,
};
use settings::Settings;
use std::sync::Arc;

enum Dismiss {}
enum Button {}

pub fn render_user_notification<V: View, A: Action + Clone>(
    user: Arc<User>,
    message: &str,
    dismiss_action: A,
    buttons: Vec<(&'static str, Box<dyn Action>)>,
    cx: &mut RenderContext<V>,
) -> ElementBox {
    let theme = cx.global::<Settings>().theme.clone();
    let theme = &theme.contact_notification;

    Flex::column()
        .with_child(
            Flex::row()
                .with_children(user.avatar.clone().map(|avatar| {
                    Image::new(avatar)
                        .with_style(theme.header_avatar)
                        .aligned()
                        .constrained()
                        .with_height(
                            cx.font_cache()
                                .line_height(theme.header_message.text.font_size),
                        )
                        .aligned()
                        .top()
                        .boxed()
                }))
                .with_child(
                    Text::new(
                        format!("{} {}", user.github_login, message),
                        theme.header_message.text.clone(),
                    )
                    .contained()
                    .with_style(theme.header_message.container)
                    .aligned()
                    .top()
                    .left()
                    .flex(1., true)
                    .boxed(),
                )
                .with_child(
                    MouseEventHandler::new::<Dismiss, _, _>(user.id as usize, cx, |state, _| {
                        render_icon_button(
                            theme.dismiss_button.style_for(state, false),
                            "icons/decline.svg",
                        )
                        .boxed()
                    })
                    .with_cursor_style(CursorStyle::PointingHand)
                    .with_padding(Padding::uniform(5.))
                    .on_click(move |_, cx| cx.dispatch_any_action(dismiss_action.boxed_clone()))
                    .aligned()
                    .constrained()
                    .with_height(
                        cx.font_cache()
                            .line_height(theme.header_message.text.font_size),
                    )
                    .aligned()
                    .top()
                    .flex_float()
                    .boxed(),
                )
                .named("contact notification header"),
        )
        .with_child(
            Label::new(
                "They won't know if you decline.".to_string(),
                theme.body_message.text.clone(),
            )
            .contained()
            .with_style(theme.body_message.container)
            .boxed(),
        )
        .with_children(if buttons.is_empty() {
            None
        } else {
            Some(
                Flex::row()
                    .with_children(buttons.into_iter().enumerate().map(
                        |(ix, (message, action))| {
                            MouseEventHandler::new::<Button, _, _>(ix, cx, |state, _| {
                                let button = theme.button.style_for(state, false);
                                Label::new(message.to_string(), button.text.clone())
                                    .contained()
                                    .with_style(button.container)
                                    .boxed()
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_click(move |_, cx| cx.dispatch_any_action(action.boxed_clone()))
                            .boxed()
                        },
                    ))
                    .aligned()
                    .right()
                    .boxed(),
            )
        })
        .contained()
        .boxed()
}
