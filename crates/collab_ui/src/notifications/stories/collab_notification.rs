use gpui::prelude::*;
use story::{StoryContainer, StoryItem, StorySection};
use ui::prelude::*;

use crate::notifications::collab_notification::CollabNotification;

pub struct CollabNotificationStory;

impl Render for CollabNotificationStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let window_container = |width, height| div().w(px(width)).h(px(height));

        StoryContainer::new(
            "CollabNotification Story",
            "crates/collab_ui/src/notifications/stories/collab_notification.rs",
        )
        .child(
            StorySection::new().child(StoryItem::new(
                "Incoming Call Notification",
                window_container(400., 72.).child(
                    CollabNotification::new(
                        "https://avatars.githubusercontent.com/u/1486634?v=4",
                        Button::new("accept", "Accept"),
                        Button::new("decline", "Decline"),
                    )
                    .child(
                        v_flex()
                            .overflow_hidden()
                            .child(Label::new("maxdeviant is sharing a project in Zed")),
                    ),
                ),
            )),
        )
        .child(
            StorySection::new().child(StoryItem::new(
                "Project Shared Notification",
                window_container(400., 72.).child(
                    CollabNotification::new(
                        "https://avatars.githubusercontent.com/u/1714999?v=4",
                        Button::new("open", "Open"),
                        Button::new("dismiss", "Dismiss"),
                    )
                    .child(Label::new("iamnbutler"))
                    .child(Label::new("is sharing a project in Zed:"))
                    .child(Label::new("zed")),
                ),
            )),
        )
    }
}
