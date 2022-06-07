use crate::ViewReleaseNotes;
use gpui::{
    elements::{Flex, MouseEventHandler, Padding, ParentElement, Svg, Text},
    platform::{AppVersion, CursorStyle},
    Element, Entity, View, ViewContext,
};
use menu::Cancel;
use settings::Settings;
use workspace::Notification;

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

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let theme = &theme.update_notification;

        MouseEventHandler::new::<ViewReleaseNotes, _, _>(0, cx, |state, cx| {
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(
                            Text::new(
                                format!("Updated to Zed {}", self.version),
                                theme.message.text.clone(),
                            )
                            .contained()
                            .with_style(theme.message.container)
                            .aligned()
                            .top()
                            .left()
                            .flex(1., true)
                            .boxed(),
                        )
                        .with_child(
                            MouseEventHandler::new::<Cancel, _, _>(0, cx, |state, _| {
                                let style = theme.dismiss_button.style_for(state, false);
                                Svg::new("icons/decline.svg")
                                    .with_color(style.color)
                                    .constrained()
                                    .with_width(style.icon_width)
                                    .aligned()
                                    .contained()
                                    .with_style(style.container)
                                    .constrained()
                                    .with_width(style.button_width)
                                    .with_height(style.button_width)
                                    .boxed()
                            })
                            .with_padding(Padding::uniform(5.))
                            .on_click(move |_, _, cx| cx.dispatch_action(Cancel))
                            .aligned()
                            .constrained()
                            .with_height(cx.font_cache().line_height(theme.message.text.font_size))
                            .aligned()
                            .top()
                            .flex_float()
                            .boxed(),
                        )
                        .boxed(),
                )
                .with_child({
                    let style = theme.action_message.style_for(state, false);
                    Text::new("View the release notes".to_string(), style.text.clone())
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .contained()
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(|_, _, cx| cx.dispatch_action(ViewReleaseNotes))
        .boxed()
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
