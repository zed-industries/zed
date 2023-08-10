use client::User;
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    AnyElement, Element, ViewContext,
};
use std::sync::Arc;

enum Dismiss {}
enum Button {}

pub fn render_user_notification<F, V: 'static>(
    user: Arc<User>,
    title: &'static str,
    body: Option<&'static str>,
    on_dismiss: F,
    buttons: Vec<(&'static str, Box<dyn Fn(&mut V, &mut ViewContext<V>)>)>,
    cx: &mut ViewContext<V>,
) -> AnyElement<V>
where
    F: 'static + Fn(&mut V, &mut ViewContext<V>),
{
    let theme = theme::current(cx).clone();
    let theme = &theme.contact_notification;

    Flex::column()
        .with_child(
            Flex::row()
                .with_children(user.avatar.clone().map(|avatar| {
                    Image::from_data(avatar)
                        .with_style(theme.header_avatar)
                        .aligned()
                        .constrained()
                        .with_height(
                            cx.font_cache()
                                .line_height(theme.header_message.text.font_size),
                        )
                        .aligned()
                        .top()
                }))
                .with_child(
                    Text::new(
                        format!("{} {}", user.github_login, title),
                        theme.header_message.text.clone(),
                    )
                    .contained()
                    .with_style(theme.header_message.container)
                    .aligned()
                    .top()
                    .left()
                    .flex(1., true),
                )
                .with_child(
                    MouseEventHandler::<Dismiss, V>::new(user.id as usize, cx, |state, _| {
                        let style = theme.dismiss_button.style_for(state);
                        Svg::new("icons/x_mark_8.svg")
                            .with_color(style.color)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .contained()
                            .with_style(style.container)
                            .constrained()
                            .with_width(style.button_width)
                            .with_height(style.button_width)
                    })
                    .with_cursor_style(CursorStyle::PointingHand)
                    .with_padding(Padding::uniform(5.))
                    .on_click(MouseButton::Left, move |_, view, cx| on_dismiss(view, cx))
                    .aligned()
                    .constrained()
                    .with_height(
                        cx.font_cache()
                            .line_height(theme.header_message.text.font_size),
                    )
                    .aligned()
                    .top()
                    .flex_float(),
                )
                .into_any_named("contact notification header"),
        )
        .with_children(body.map(|body| {
            Label::new(body, theme.body_message.text.clone())
                .contained()
                .with_style(theme.body_message.container)
        }))
        .with_children(if buttons.is_empty() {
            None
        } else {
            Some(
                Flex::row()
                    .with_children(buttons.into_iter().enumerate().map(
                        |(ix, (message, handler))| {
                            MouseEventHandler::<Button, V>::new(ix, cx, |state, _| {
                                let button = theme.button.style_for(state);
                                Label::new(message, button.text.clone())
                                    .contained()
                                    .with_style(button.container)
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_click(MouseButton::Left, move |_, view, cx| handler(view, cx))
                        },
                    ))
                    .aligned()
                    .right(),
            )
        })
        .contained()
        .into_any()
}
