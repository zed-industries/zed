use gpui::{Entity, Render};
use story::{Story, StoryItem, StorySection};

use ui::prelude::*;

use crate::application_menu::ApplicationMenu;

pub struct ApplicationMenuStory {
    menu: Entity<ApplicationMenu>,
}

impl ApplicationMenuStory {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        Self {
            menu: cx.new(|cx| ApplicationMenu::new(window, cx)),
        }
    }
}

impl Render for ApplicationMenuStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<ApplicationMenu>())
            .child(StorySection::new().child(StoryItem::new(
                "Application Menu",
                h_flex().child(self.menu.clone()),
            )))
    }
}
