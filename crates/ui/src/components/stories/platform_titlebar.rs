use gpui::Render;
use story::{StoryContainer, StoryItem, StorySection};

use crate::{prelude::*, PlatformStyle, PlatformTitlebar};

pub struct PlatformTitlebarStory;

impl Render for PlatformTitlebarStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        StoryContainer::new(
            "Platform Titlebar",
            "crates/ui/src/components/stories/platform_titlebar.rs",
        )
        .child(
            StorySection::new().child(
                StoryItem::new(
                    "Default (macOS)",
                    PlatformTitlebar::new("macos").platform_style(PlatformStyle::MacOs),
                )
                .description("")
                .usage(""),
            ),
        )
        .child(
            StorySection::new().child(
                StoryItem::new(
                    "Default (Linux)",
                    PlatformTitlebar::new("linux").platform_style(PlatformStyle::Linux),
                )
                .description("")
                .usage(""),
            ),
        )
        .child(
            StorySection::new().child(
                StoryItem::new(
                    "Default (Windows)",
                    PlatformTitlebar::new("windows").platform_style(PlatformStyle::Windows),
                )
                .description("")
                .usage(""),
            ),
        )
        .into_element()
    }
}
