use gpui::{Render, View};
use story::{Story, StoryItem, StorySection};

use ui::prelude::*;

use crate::application_menu::ApplicationMenu;

pub struct ApplicationMenuStory {
    menu: View<ApplicationMenu>,
}

impl ApplicationMenuStory {
    pub fn new(cx: &mut WindowContext) -> Self {
        Self {
            menu: cx.new_view(ApplicationMenu::new),
        }
    }
}

impl Render for ApplicationMenuStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<ApplicationMenu>())
            .child(StorySection::new().child(StoryItem::new(
                "Application Menu",
                h_flex().child(self.menu.clone()),
            )))
    }
}
