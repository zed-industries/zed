use gpui::Render;
use story::{Story, StoryItem, StorySection};

use crate::{prelude::*, ToolStrip, Tooltip};

pub struct ToolStripStory;

impl Render for ToolStripStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<ToolStrip>())
            .child(
                StorySection::new().child(StoryItem::new(
                    "Vertical Tool Strip",
                    h_flex().child(
                        ToolStrip::vertical("tool_strip_example")
                            .tool(
                                IconButton::new("example_tool", IconName::AudioOn)
                                    .tooltip(|cx| Tooltip::text("Example tool", cx)),
                            )
                            .tool(
                                IconButton::new("example_tool_2", IconName::MicMute)
                                    .tooltip(|cx| Tooltip::text("Example tool 2", cx)),
                            )
                            .tool(
                                IconButton::new("example_tool_3", IconName::Screen)
                                    .tooltip(|cx| Tooltip::text("Example tool 3", cx)),
                            ),
                    ),
                )),
            )
    }
}
