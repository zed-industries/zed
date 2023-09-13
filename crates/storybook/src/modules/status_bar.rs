use std::marker::PhantomData;

use crate::components::icon_button;
use crate::theme::{theme, Theme};
use gpui2::style::StyleHelpers;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

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

pub fn status_bar<V: 'static>() -> StatusBar<V> {
    StatusBar {
        view_type: PhantomData,
        left_tools: None,
        right_tools: None,
        bottom_tools: None,
    }
}

impl<V: 'static> StatusBar<V> {
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
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .fill(theme.lowest.base.default.background)
            .child(self.left_tools(theme))
            .child(div())
    }

    fn left_tools(&self, theme: &Theme) -> impl Element<V> {
        div()
            .flex()
            .items_center()
            .gap_px()
            .child(icon_button("icons/folder_tree_16.svg"))
            .child(icon_button("icons/bolt_16.svg"))
    }
}
