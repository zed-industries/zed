use story::{StoryContainer, StoryItem, StorySection};
use ui::prelude::*;

use crate::ui::ChatNotice;

pub struct ChatNoticeStory;

impl Render for ChatNoticeStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        StoryContainer::new(
            "ChatNotice Story",
            "crates/assistant2/src/ui/stories/chat_notice.rs",
        )
        .child(
            StorySection::new().child(StoryItem::new(
                "Project index request",
                ChatNotice::new("Allow assistant to index your project?")
                    .meta("Enabling will allow responses more relevant to this project."),
            )),
        )
    }
}
