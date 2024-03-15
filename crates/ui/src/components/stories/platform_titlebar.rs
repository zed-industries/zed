use gpui::Render;
use story::{StoryContainer, StoryItem, StorySection};

use crate::{prelude::*, PlatformStyle, PlatformTitlebar};

pub struct PlatformTitlebarStory;

impl Render for PlatformTitlebarStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        fn add_sample_children(titlebar: PlatformTitlebar) -> PlatformTitlebar {
            titlebar
                .child(div().size_2().bg(gpui::red()))
                .child(div().size_2().bg(gpui::blue()))
                .child(div().size_2().bg(gpui::green()))
        }

        StoryContainer::new(
            "Platform Titlebar",
            "crates/ui/src/components/stories/platform_titlebar.rs",
        )
        .child(
            StorySection::new().child(
                StoryItem::new(
                    "Default (macOS)",
                    PlatformTitlebar::new("macos")
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
                    PlatformTitlebar::new("linux")
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
                    PlatformTitlebar::new("windows")
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
