use gpui::Render;
use story::{StoryContainer, StoryItem, StorySection};

use ui::prelude::*;

use crate::application_menu::ApplicationMenu;

pub struct ApplicationMenuStory;

impl Render for ApplicationMenuStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        StoryContainer::new(
            "ApplicationMenu Story",
            "crates/title_bar/src/stories/application_menu.rs",
        )
        .child(StorySection::new().child(StoryItem::new(
            "Application Menu",
            h_flex().child(ApplicationMenu::new()),
        )))
    }
}
