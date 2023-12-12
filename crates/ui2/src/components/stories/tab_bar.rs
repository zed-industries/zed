use gpui::{Div, FocusHandle, Render};
use story::Story;

use crate::{prelude::*, Tab, TabBar, TabPosition};

pub struct TabBarStory {
    tab_bar_focus_handle: FocusHandle,
}

impl TabBarStory {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            tab_bar_focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for TabBarStory {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        let tab_count = 20;
        let selected_tab_index = 3;

        let tabs = (0..tab_count)
            .map(|index| {
                Tab::new(index)
                    .selected(index == selected_tab_index)
                    .position(if index == 0 {
                        TabPosition::First
                    } else if index == tab_count - 1 {
                        TabPosition::Last
                    } else {
                        TabPosition::Middle(index.cmp(&selected_tab_index))
                    })
                    .child(Label::new(format!("Tab {}", index + 1)).color(
                        if index == selected_tab_index {
                            Color::Default
                        } else {
                            Color::Muted
                        },
                    ))
            })
            .collect::<Vec<_>>();

        Story::container()
            .child(Story::title_for::<TabBar>())
            .child(Story::label("Default"))
            .child(
                h_stack().child(
                    TabBar::new("tab_bar_1", self.tab_bar_focus_handle.clone())
                        .start_child(
                            IconButton::new("navigate_backward", Icon::ArrowLeft)
                                .icon_size(IconSize::Small),
                        )
                        .start_child(
                            IconButton::new("navigate_forward", Icon::ArrowRight)
                                .icon_size(IconSize::Small),
                        )
                        .end_child(IconButton::new("new", Icon::Plus).icon_size(IconSize::Small))
                        .end_child(
                            IconButton::new("split_pane", Icon::Split).icon_size(IconSize::Small),
                        )
                        .children(tabs),
                ),
            )
    }
}
