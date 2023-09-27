use std::marker::PhantomData;

use crate::theme::{theme, Theme};
use crate::{prelude::*, IconColor};
use crate::{Button, IconAsset, IconButton, ToolDivider};

#[derive(Default, PartialEq)]
pub enum Tool {
    #[default]
    ProjectPanel,
    CollaborationPanel,
    Terminal,
    Assistant,
    Feedback,
    Diagnostics,
}

struct ToolGroup {
    active_index: Option<usize>,
    tools: Vec<Tool>,
}

impl Default for ToolGroup {
    fn default() -> Self {
        ToolGroup {
            active_index: None,
            tools: vec![],
        }
    }
}

#[derive(Element)]
pub struct StatusBar<V: 'static> {
    view_type: PhantomData<V>,
    left_tools: Option<ToolGroup>,
    right_tools: Option<ToolGroup>,
    bottom_tools: Option<ToolGroup>,
}

impl<V: 'static> StatusBar<V> {
    pub fn new() -> Self {
        Self {
            view_type: PhantomData,
            left_tools: None,
            right_tools: None,
            bottom_tools: None,
        }
    }

    pub fn left_tool(mut self, tool: Tool, active_index: Option<usize>) -> Self {
        self.left_tools = {
            let mut tools = vec![tool];
            tools.extend(self.left_tools.take().unwrap_or_default().tools);
            Some(ToolGroup {
                active_index,
                tools,
            })
        };
        self
    }

    pub fn right_tool(mut self, tool: Tool, active_index: Option<usize>) -> Self {
        self.right_tools = {
            let mut tools = vec![tool];
            tools.extend(self.left_tools.take().unwrap_or_default().tools);
            Some(ToolGroup {
                active_index,
                tools,
            })
        };
        self
    }

    pub fn bottom_tool(mut self, tool: Tool, active_index: Option<usize>) -> Self {
        self.bottom_tools = {
            let mut tools = vec![tool];
            tools.extend(self.left_tools.take().unwrap_or_default().tools);
            Some(ToolGroup {
                active_index,
                tools,
            })
        };
        self
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .py_0p5()
            .px_1()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .fill(theme.lowest.base.default.background)
            .child(self.left_tools(&theme))
            .child(self.right_tools(&theme))
    }

    fn left_tools(&self, theme: &Theme) -> impl Element<V> {
        div()
            .flex()
            .items_center()
            .gap_1()
            .child(IconButton::new(IconAsset::FileTree).color(IconColor::Accent))
            .child(IconButton::new(IconAsset::Hash))
            .child(ToolDivider::new())
            .child(IconButton::new(IconAsset::XCircle))
    }
    fn right_tools(&self, theme: &Theme) -> impl Element<V> {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(Button::new("116:25"))
                    .child(Button::new("Rust")),
            )
            .child(ToolDivider::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(IconButton::new(IconAsset::Copilot))
                    .child(IconButton::new(IconAsset::Envelope)),
            )
            .child(ToolDivider::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(IconButton::new(IconAsset::Terminal))
                    .child(IconButton::new(IconAsset::MessageBubbles))
                    .child(IconButton::new(IconAsset::Ai)),
            )
    }
}
