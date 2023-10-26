use std::sync::Arc;

use crate::prelude::*;
use crate::{Button, Icon, IconButton, IconColor, ToolDivider, Workspace};

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
#[element(view_state = "Workspace")]
pub struct StatusBar {
    left_tools: Option<ToolGroup>,
    right_tools: Option<ToolGroup>,
    bottom_tools: Option<ToolGroup>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            // state_type: PhantomData,
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

    fn render(
        &mut self,
        view: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> impl Element<Workspace> {
        let theme = theme(cx);

        div()
            .py_0p5()
            .px_1()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .bg(theme.status_bar)
            .child(self.left_tools(view, cx))
            .child(self.right_tools(view, cx))
    }

    fn left_tools(&self, workspace: &mut Workspace, cx: &WindowContext) -> impl Element<Workspace> {
        div()
            .flex()
            .items_center()
            .gap_1()
            .child(
                IconButton::<Workspace>::new("project_panel", Icon::FileTree)
                    .when(workspace.is_project_panel_open(), |this| {
                        this.color(IconColor::Accent)
                    })
                    .on_click(|workspace, cx| {
                        workspace.toggle_project_panel(cx);
                    }),
            )
            .child(
                IconButton::<Workspace>::new("collab_panel", Icon::Hash)
                    .when(workspace.is_collab_panel_open(), |this| {
                        this.color(IconColor::Accent)
                    })
                    .on_click(|workspace, cx| {
                        workspace.toggle_collab_panel();
                    }),
            )
            .child(ToolDivider::new())
            .child(IconButton::new("diagnostics", Icon::XCircle))
    }

    fn right_tools(
        &self,
        workspace: &mut Workspace,
        cx: &WindowContext,
    ) -> impl Element<Workspace> {
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
                    .child(
                        Button::<Workspace>::new("Rust").on_click(Arc::new(|workspace, cx| {
                            workspace.toggle_language_selector(cx);
                        })),
                    ),
            )
            .child(ToolDivider::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        IconButton::new("copilot", Icon::Copilot)
                            .on_click(|_, _| println!("Copilot clicked.")),
                    )
                    .child(
                        IconButton::new("envelope", Icon::Envelope)
                            .on_click(|_, _| println!("Send Feedback clicked.")),
                    ),
            )
            .child(ToolDivider::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        IconButton::<Workspace>::new("terminal", Icon::Terminal)
                            .when(workspace.is_terminal_open(), |this| {
                                this.color(IconColor::Accent)
                            })
                            .on_click(|workspace, cx| {
                                workspace.toggle_terminal(cx);
                            }),
                    )
                    .child(
                        IconButton::<Workspace>::new("chat_panel", Icon::MessageBubbles)
                            .when(workspace.is_chat_panel_open(), |this| {
                                this.color(IconColor::Accent)
                            })
                            .on_click(|workspace, cx| {
                                workspace.toggle_chat_panel(cx);
                            }),
                    )
                    .child(
                        IconButton::<Workspace>::new("assistant_panel", Icon::Ai)
                            .when(workspace.is_assistant_panel_open(), |this| {
                                this.color(IconColor::Accent)
                            })
                            .on_click(|workspace, cx| {
                                workspace.toggle_assistant_panel(cx);
                            }),
                    ),
            )
    }
}
