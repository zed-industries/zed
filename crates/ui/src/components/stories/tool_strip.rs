use gpui::{Axis, Render};
use story::{StoryContainer, StoryItem, StorySection};

use crate::{prelude::*, ToolStrip, ToolStripItem, ToolStripStyle};

pub struct ToolStripStory;

impl Render for ToolStripStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        StoryContainer::new(
            "Tool Strip",
            "crates/ui/src/components/stories/tool_strip.rs",
        )
        .child(
            StorySection::new().child(StoryItem::new(
                "Popover Style",
                ToolStrip::popover(
                    "git-hunks",
                    vec![
                        vec![
                            ToolStripItem {
                                id: "expand-diff".into(),
                                icon: IconName::Plus,
                                label: "Expand Diff".into(),
                                keybinding: None,
                                on_click: Box::new(|_, _| {
                                    println!("Expand Diff");
                                }),
                            },
                            ToolStripItem {
                                id: "expand-diff".into(),
                                icon: IconName::Plus,
                                label: "Expand Diff".into(),
                                keybinding: None,
                                on_click: Box::new(|_, _| {
                                    println!("Expand Diff");
                                }),
                            },
                        ],
                        vec![ToolStripItem {
                            id: "expand-diff".into(),
                            icon: IconName::Plus,
                            label: "Expand Diff".into(),
                            keybinding: None,
                            on_click: Box::new(|_, _| {
                                println!("Expand Diff");
                            }),
                        }],
                    ],
                )
                .style(ToolStripStyle::Popover)
                .axis(Axis::Vertical),
            )),
        )
    }
}
