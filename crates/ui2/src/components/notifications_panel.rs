use crate::{prelude::*, static_new_notification_items, static_read_notification_items};
use crate::{List, ListHeader};

#[derive(Component)]
pub struct NotificationsPanel {
    id: ElementId,
}

impl NotificationsPanel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let theme = old_theme(cx);

        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.surface)
            .child(
                div()
                    .id("header")
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .child(
                        List::new(static_new_notification_items())
                            .header(ListHeader::new("NEW").toggle(ToggleState::Toggled))
                            .toggle(ToggleState::Toggled),
                    )
                    .child(
                        List::new(static_read_notification_items())
                            .header(ListHeader::new("EARLIER").toggle(ToggleState::Toggled))
                            .empty_message("No new notifications")
                            .toggle(ToggleState::Toggled),
                    ),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{Panel, Story};
    use gpui2::{Div, Render};

    pub struct NotificationsPanelStory;

    impl Render for NotificationsPanelStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, NotificationsPanel>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new("panel", cx).child(NotificationsPanel::new("notifications_panel")),
                )
        }
    }
}
