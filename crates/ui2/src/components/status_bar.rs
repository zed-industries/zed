use std::marker::PhantomData;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::prelude::*;
use crate::{get_workspace_state, Button, Icon, IconButton, IconColor, ToolDivider};

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
pub struct StatusBar<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    left_tools: Option<ToolGroup>,
    right_tools: Option<ToolGroup>,
    bottom_tools: Option<ToolGroup>,
}

impl<S: 'static + Send + Sync + Clone> StatusBar<S> {
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
        let workspace_state = get_workspace_state();

        div()
            .flex()
            .items_center()
            .gap_1()
            .child(
                IconButton::new(Icon::FileTree)
                    .when(workspace_state.is_project_panel_open(), |this| {
                        this.color(IconColor::Accent)
                    })
                    .on_click(|_, cx| {
                        workspace_state.toggle_project_panel();
                        cx.notify();
                    }),
            )
            .child(
                IconButton::new(Icon::Hash)
                    .when(workspace_state.is_collab_panel_open(), |this| {
                        this.color(IconColor::Accent)
                    })
                    .on_click(|_, cx| {
                        workspace_state.toggle_collab_panel();
                        cx.notify();
                    }),
            )
            .child(ToolDivider::new())
            .child(IconButton::new(Icon::XCircle))
    }

    fn right_tools(&self, theme: &Theme) -> impl Element<State = S> {
        let workspace_state = get_workspace_state();

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
                    .child(Button::new("Rust").on_click(Arc::new(|_, cx| {
                        let is_showing_language_selector = workspace_state
                            .show_language_selector
                            .load(Ordering::SeqCst);

                        workspace_state
                            .show_language_selector
                            .compare_exchange(
                                is_showing_language_selector,
                                !is_showing_language_selector,
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .unwrap();

                        cx.notify();
                    }))),
            )
            .child(ToolDivider::new())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        IconButton::new(Icon::Copilot)
                            .on_click(|_, _| println!("Copilot clicked.")),
                    )
                    .child(
                        IconButton::new(Icon::Envelope)
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
                        IconButton::new(Icon::Terminal)
                            .when(
                                workspace_state.show_terminal.load(Ordering::SeqCst),
                                |this| this.color(IconColor::Accent),
                            )
                            .on_click(|_, cx| {
                                let is_showing_terminal =
                                    workspace_state.show_terminal.load(Ordering::SeqCst);

                                workspace_state
                                    .show_terminal
                                    .compare_exchange(
                                        is_showing_terminal,
                                        !is_showing_terminal,
                                        Ordering::SeqCst,
                                        Ordering::SeqCst,
                                    )
                                    .unwrap();

                                cx.notify();
                            }),
                    )
                    .child(
                        IconButton::new(Icon::MessageBubbles)
                            .when(
                                workspace_state.show_chat_panel.load(Ordering::SeqCst),
                                |this| this.color(IconColor::Accent),
                            )
                            .on_click(|_, cx| {
                                let is_showing_chat_panel =
                                    workspace_state.show_chat_panel.load(Ordering::SeqCst);

                                workspace_state
                                    .show_chat_panel
                                    .compare_exchange(
                                        is_showing_chat_panel,
                                        !is_showing_chat_panel,
                                        Ordering::SeqCst,
                                        Ordering::SeqCst,
                                    )
                                    .unwrap();

                                cx.notify();
                            }),
                    )
                    .child(IconButton::new(Icon::Ai)),
            )
    }
}
