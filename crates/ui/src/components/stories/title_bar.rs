use gpui::{NoAction, Render};
use story::{StoryContainer, StoryItem, StorySection};

use crate::{prelude::*, PlatformStyle, TitleBar};

pub struct TitleBarStory;

impl Render for TitleBarStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        fn add_sample_children(titlebar: TitleBar) -> TitleBar {
            titlebar
                .child(div().size_2().bg(gpui::red()))
                .child(div().size_2().bg(gpui::blue()))
                .child(div().size_2().bg(gpui::green()))
        }

        StoryContainer::new("TitleBar", "crates/ui/src/components/stories/title_bar.rs")
            .child(
                StorySection::new().child(
                    StoryItem::new(
                        "Default (macOS)",
                        TitleBar::new("macos", Box::new(NoAction))
                            .platform_style(PlatformStyle::Mac)
                            .map(add_sample_children),
                    )
                    .description("")
                    .usage(""),
                ),
            )
            .child(
                StorySection::new().child(
                    StoryItem::new(
                        "Default (Linux)",
                        TitleBar::new("linux", Box::new(NoAction))
                            .platform_style(PlatformStyle::Linux)
                            .map(add_sample_children),
                    )
                    .description("")
                    .usage(""),
                ),
            )
            .child(
                StorySection::new().child(
                    StoryItem::new(
                        "Default (Windows)",
                        TitleBar::new("windows", Box::new(NoAction))
                            .platform_style(PlatformStyle::Windows)
                            .map(add_sample_children),
                    )
                    .description("")
                    .usage(""),
                ),
            )
            .into_element()
    }
}
