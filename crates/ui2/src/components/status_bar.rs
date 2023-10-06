use std::marker::PhantomData;

use crate::prelude::*;
use crate::theme::{theme, Theme};
use crate::{Icon, IconButton, IconColor, ToolDivider};

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
pub struct StatusBar<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    left_tools: Option<ToolGroup>,
    right_tools: Option<ToolGroup>,
    bottom_tools: Option<ToolGroup>,
}

impl<S: 'static + Send + Sync> StatusBar<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
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

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
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

    fn left_tools(&self, theme: &Theme) -> impl Element<State = S> {
        div()
            .flex()
            .items_center()
            .gap_1()
            .child(IconButton::new(Icon::FileTree).color(IconColor::Accent))
            .child(IconButton::new(Icon::Hash))
            .child(ToolDivider::new())
            .child(IconButton::new(Icon::XCircle))
    }

    fn right_tools(&self, theme: &Theme) -> impl Element<State = S> {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    // .child(Button::new("116:25"))
                    // .child(Button::new("Rust")),
            )
            .child(ToolDivider::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(IconButton::new(Icon::Copilot))
                    .child(IconButton::new(Icon::Envelope)),
            )
            .child(ToolDivider::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(IconButton::new(Icon::Terminal))
                    .child(IconButton::new(Icon::MessageBubbles))
                    .child(IconButton::new(Icon::Ai)),
            )
    }
}
