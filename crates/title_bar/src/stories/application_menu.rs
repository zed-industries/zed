use gpui::Render;
use story::{Story, StoryItem, StorySection};

use ui::prelude::*;

use crate::application_menu::ApplicationMenu;

pub struct ApplicationMenuStory;

impl Render for ApplicationMenuStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container(cx)
            .child(Story::title_for::<ApplicationMenu>(cx))
            .child(StorySection::new().child(StoryItem::new(
                "Application Menu",
                h_flex().child(ApplicationMenu::new()),
            )))
    }
}
