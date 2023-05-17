use crate::ViewReleaseNotes;
use gpui::{
    elements::{Flex, MouseEventHandler, Padding, ParentElement, Svg, Text},
    platform::{AppVersion, CursorStyle, MouseButton},
    Element, Entity, View, ViewContext,
};
use menu::Cancel;
use util::channel::ReleaseChannel;
use workspace::notifications::Notification;

pub struct UpdateNotification {
    version: AppVersion,
}

pub enum Event {
    Dismiss,
}

impl Entity for UpdateNotification {
    type Event = Event;
}

impl View for UpdateNotification {
    fn ui_name() -> &'static str {
        "UpdateNotification"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> gpui::AnyElement<Self> {
        let theme = theme::current(cx).clone();
        let theme = &theme.update_notification;

        let app_name = cx.global::<ReleaseChannel>().display_name();

        MouseEventHandler::<ViewReleaseNotes, _>::new(0, cx, |state, cx| {
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(
                            Text::new(
                                format!("Updated to {app_name} {}", self.version),
                                theme.message.text.clone(),
                            )
                            .contained()
                            .with_style(theme.message.container)
                            .aligned()
                            .top()
                            .left()
                            .flex(1., true),
                        )
                        .with_child(
                            MouseEventHandler::<Cancel, _>::new(0, cx, |state, _| {
                                let style = theme.dismiss_button.style_for(state, false);
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
                            .with_padding(Padding::uniform(5.))
                            .on_click(MouseButton::Left, move |_, this, cx| {
                                this.dismiss(&Default::default(), cx)
                            })
                            .aligned()
                            .constrained()
                            .with_height(cx.font_cache().line_height(theme.message.text.font_size))
                            .aligned()
                            .top()
                            .flex_float(),
                        ),
                )
                .with_child({
                    let style = theme.action_message.style_for(state, false);
                    Text::new("View the release notes", style.text.clone())
                        .contained()
                        .with_style(style.container)
                })
                .contained()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, _, cx| {
            crate::view_release_notes(&Default::default(), cx)
        })
        .into_any_named("update notification")
    }
}

impl Notification for UpdateNotification {
    fn should_dismiss_notification_on_event(&self, event: &<Self as Entity>::Event) -> bool {
        matches!(event, Event::Dismiss)
    }
}

impl UpdateNotification {
    pub fn new(version: AppVersion) -> Self {
        Self { version }
    }

    pub fn dismiss(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismiss);
    }
}
