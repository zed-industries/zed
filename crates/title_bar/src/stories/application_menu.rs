use gpui::{Model, Render, };
use story::{Story, StoryItem, StorySection};

use ui::prelude::*;

use crate::application_menu::ApplicationMenu;

pub struct ApplicationMenuStory {
    menu: Model<ApplicationMenu>,
}

impl ApplicationMenuStory {
    pub fn new(window: &mut Window, cx: &mut AppContext) -> Self {
        Self {
            menu: window.new_view(cx, ApplicationMenu::new),
        }
    }
}

impl Render for ApplicationMenuStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut ModelContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<ApplicationMenu>())
            .child(StorySection::new().child(StoryItem::new(
                "Application Menu",
                h_flex().child(self.menu.clone()),
            )))
    }
}
