use std::marker::PhantomData;

use crate::{prelude::*, static_new_notification_items, static_read_notification_items};
use crate::{List, ListHeader};

#[derive(Element)]
pub struct NotificationsPanel<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> NotificationsPanel<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(color.surface)
            .child(
                div()
                    .id("header")
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .child(
                        List::new(static_new_notification_items())
                            .header(ListHeader::new("NEW").set_toggle(ToggleState::Toggled))
                            .set_toggle(ToggleState::Toggled),
                    )
                    .child(
                        List::new(static_read_notification_items())
                            .header(ListHeader::new("EARLIER").set_toggle(ToggleState::Toggled))
                            .empty_message("No new notifications")
                            .set_toggle(ToggleState::Toggled),
                    ),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{Panel, Story};

    use super::*;

    #[derive(Element)]
    pub struct NotificationsPanelStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> NotificationsPanelStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, NotificationsPanel<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new("panel", cx).child(NotificationsPanel::new("notifications_panel")),
                )
        }
    }
}
