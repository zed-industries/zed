use gpui::Render;
use story::{Story, StoryItem, StorySection};

use ui::prelude::*;

use crate::application_menu::ApplicationMenu;

pub struct ApplicationMenuStory;

impl Render for ApplicationMenuStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<ApplicationMenu>())
            .child(StorySection::new().child(StoryItem::new(
                "Application Menu",
                h_flex().child(ApplicationMenu::new()),
            )))
    }
}
