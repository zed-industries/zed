use gpui::{div, Div, EventEmitter, ParentElement, Render, SemanticVersion, ViewContext};
use menu::Cancel;
use workspace::notifications::NotificationEvent;

pub struct UpdateNotification {
    _version: SemanticVersion,
}

impl EventEmitter<NotificationEvent> for UpdateNotification {}

impl Render for UpdateNotification {
    type Output = Div;

    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> Self::Output {
        div().child("Updated zed!")
        // let theme = theme::current(cx).clone();
        // let theme = &theme.update_notification;

        // let app_name = cx.global::<ReleaseChannel>().display_name();

        // MouseEventHandler::new::<ViewReleaseNotes, _>(0, cx, |state, cx| {
        //     Flex::column()
        //         .with_child(
        //             Flex::row()
        //                 .with_child(
        //                     Text::new(
        //                         format!("Updated to {app_name} {}", self.version),
        //                         theme.message.text.clone(),
        //                     )
        //                     .contained()
        //                     .with_style(theme.message.container)
        //                     .aligned()
        //                     .top()
        //                     .left()
        //                     .flex(1., true),
        //                 )
        //                 .with_child(
        //                     MouseEventHandler::new::<Cancel, _>(0, cx, |state, _| {
        //                         let style = theme.dismiss_button.style_for(state);
        //                         Svg::new("icons/x.svg")
        //                             .with_color(style.color)
        //                             .constrained()
        //                             .with_width(style.icon_width)
        //                             .aligned()
        //                             .contained()
        //                             .with_style(style.container)
        //                             .constrained()
        //                             .with_width(style.button_width)
        //                             .with_height(style.button_width)
        //                     })
        //                     .with_padding(Padding::uniform(5.))
        //                     .on_click(MouseButton::Left, move |_, this, cx| {
        //                         this.dismiss(&Default::default(), cx)
        //                     })
        //                     .aligned()
        //                     .constrained()
        //                     .with_height(cx.font_cache().line_height(theme.message.text.font_size))
        //                     .aligned()
        //                     .top()
        //                     .flex_float(),
        //                 ),
        //         )
        //         .with_child({
        //             let style = theme.action_message.style_for(state);
        //             Text::new("View the release notes", style.text.clone())
        //                 .contained()
        //                 .with_style(style.container)
        //         })
        //         .contained()
        // })
        // .with_cursor_style(CursorStyle::PointingHand)
        // .on_click(MouseButton::Left, |_, _, cx| {
        //     crate::view_release_notes(&Default::default(), cx)
        // })
        // .into_any_named("update notification")
    }
}

impl UpdateNotification {
    pub fn new(version: SemanticVersion) -> Self {
        Self { _version: version }
    }

    pub fn _dismiss(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(NotificationEvent::Dismiss);
    }
}
