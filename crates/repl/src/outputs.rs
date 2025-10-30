//! # REPL Output Module
//!
//! This module provides the core functionality for handling and displaying
//! various types of output from Jupyter kernels.
//!
//! ## Key Components
//!
//! - `OutputContent`: An enum that encapsulates different types of output content.
//! - `ExecutionView`: Manages the display of outputs for a single execution.
//! - `ExecutionStatus`: Represents the current status of an execution.
//!
//! ## Output Types
//!
//! The module supports several output types, including:
//! - Plain text
//! - Markdown
//! - Images (PNG and JPEG)
//! - Tables
//! - Error messages
//!
//! ## Clipboard Support
//!
//! Most output types implement the `SupportsClipboard` trait, allowing
//! users to easily copy output content to the system clipboard.
//!
//! ## Rendering
//!
//! The module provides rendering capabilities for each output type,
//! ensuring proper display within the REPL interface.
//!
//! ## Jupyter Integration
//!
//! This module is designed to work with Jupyter message protocols,
//! interpreting and displaying various types of Jupyter output.

use editor::{Editor, MultiBuffer};
use gpui::{AnyElement, ClipboardItem, Entity, Render, WeakEntity};
use language::Buffer;
use runtimelib::{ExecutionState, JupyterMessageContent, MimeBundle, MimeType};
use ui::{
    ButtonStyle, CommonAnimationExt, Context, IconButton, IconName, IntoElement, Styled, Tooltip,
    Window, div, h_flex, prelude::*, v_flex,
};

mod image;
use image::ImageView;

mod markdown;
use markdown::MarkdownView;

mod table;
use table::TableView;

pub mod plain;
use plain::TerminalOutput;

pub(crate) mod user_error;
use user_error::ErrorView;
use workspace::Workspace;

/// When deciding what to render from a collection of mediatypes, we need to rank them in order of importance
fn rank_mime_type(mimetype: &MimeType) -> usize {
    match mimetype {
        MimeType::DataTable(_) => 6,
        MimeType::Png(_) => 4,
        MimeType::Jpeg(_) => 3,
        MimeType::Markdown(_) => 2,
        MimeType::Plain(_) => 1,
        // All other media types are not supported in Zed at this time
        _ => 0,
    }
}

pub(crate) trait OutputContent {
    fn clipboard_content(&self, window: &Window, cx: &App) -> Option<ClipboardItem>;
    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn buffer_content(&mut self, _window: &mut Window, _cx: &mut App) -> Option<Entity<Buffer>> {
        None
    }
}

impl<V: OutputContent + 'static> OutputContent for Entity<V> {
    fn clipboard_content(&self, window: &Window, cx: &App) -> Option<ClipboardItem> {
        self.read(cx).clipboard_content(window, cx)
    }

    fn has_clipboard_content(&self, window: &Window, cx: &App) -> bool {
        self.read(cx).has_clipboard_content(window, cx)
    }

    fn has_buffer_content(&self, window: &Window, cx: &App) -> bool {
        self.read(cx).has_buffer_content(window, cx)
    }

    fn buffer_content(&mut self, window: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        self.update(cx, |item, cx| item.buffer_content(window, cx))
    }
}

pub enum Output {
    Plain {
        content: Entity<TerminalOutput>,
        display_id: Option<String>,
    },
    Stream {
        content: Entity<TerminalOutput>,
    },
    Image {
        content: Entity<ImageView>,
        display_id: Option<String>,
    },
    ErrorOutput(ErrorView),
    Message(String),
    Table {
        content: Entity<TableView>,
        display_id: Option<String>,
    },
    Markdown {
        content: Entity<MarkdownView>,
        display_id: Option<String>,
    },
    ClearOutputWaitMarker,
}

impl Output {
    fn render_output_controls<V: OutputContent + 'static>(
        v: Entity<V>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<ExecutionView>,
    ) -> Option<AnyElement> {
        if !v.has_clipboard_content(window, cx) && !v.has_buffer_content(window, cx) {
            return None;
        }

        Some(
            h_flex()
                .pl_1()
                .when(v.has_clipboard_content(window, cx), |el| {
                    let v = v.clone();
                    el.child(
                        IconButton::new(ElementId::Name("copy-output".into()), IconName::Copy)
                            .style(ButtonStyle::Transparent)
                            .tooltip(Tooltip::text("Copy Output"))
                            .on_click(move |_, window, cx| {
                                let clipboard_content = v.clipboard_content(window, cx);

                                if let Some(clipboard_content) = clipboard_content.as_ref() {
                                    cx.write_to_clipboard(clipboard_content.clone());
                                }
                            }),
                    )
                })
                .when(v.has_buffer_content(window, cx), |el| {
                    let v = v.clone();
                    el.child(
                        IconButton::new(
                            ElementId::Name("open-in-buffer".into()),
                            IconName::FileTextOutlined,
                        )
                        .style(ButtonStyle::Transparent)
                        .tooltip(Tooltip::text("Open in Buffer"))
                        .on_click({
                            let workspace = workspace.clone();
                            move |_, window, cx| {
                                let buffer_content =
                                    v.update(cx, |item, cx| item.buffer_content(window, cx));

                                if let Some(buffer_content) = buffer_content.as_ref() {
                                    let buffer = buffer_content.clone();
                                    let editor = Box::new(cx.new(|cx| {
                                        let multibuffer = cx.new(|cx| {
                                            let mut multi_buffer =
                                                MultiBuffer::singleton(buffer.clone(), cx);

                                            multi_buffer.set_title("REPL Output".to_string(), cx);
                                            multi_buffer
                                        });

                                        Editor::for_multibuffer(multibuffer, None, window, cx)
                                    }));
                                    workspace
                                        .update(cx, |workspace, cx| {
                                            workspace.add_item_to_active_pane(
                                                editor, None, true, window, cx,
                                            );
                                        })
                                        .ok();
                                }
                            }
                        }),
                    )
                })
                .into_any_element(),
        )
    }

    pub fn render(
        &self,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<ExecutionView>,
    ) -> impl IntoElement + use<> {
        let content = match self {
            Self::Plain { content, .. } => Some(content.clone().into_any_element()),
            Self::Markdown { content, .. } => Some(content.clone().into_any_element()),
            Self::Stream { content, .. } => Some(content.clone().into_any_element()),
            Self::Image { content, .. } => Some(content.clone().into_any_element()),
            Self::Message(message) => Some(div().child(message.clone()).into_any_element()),
            Self::Table { content, .. } => Some(content.clone().into_any_element()),
            Self::ErrorOutput(error_view) => error_view.render(window, cx),
            Self::ClearOutputWaitMarker => None,
        };

        h_flex()
            .id("output-content")
            .w_full()
            .overflow_x_scroll()
            .items_start()
            .child(div().flex_1().children(content))
            .children(match self {
                Self::Plain { content, .. } => {
                    Self::render_output_controls(content.clone(), workspace, window, cx)
                }
                Self::Markdown { content, .. } => {
                    Self::render_output_controls(content.clone(), workspace, window, cx)
                }
                Self::Stream { content, .. } => {
                    Self::render_output_controls(content.clone(), workspace, window, cx)
                }
                Self::Image { content, .. } => {
                    Self::render_output_controls(content.clone(), workspace, window, cx)
                }
                Self::ErrorOutput(err) => {
                    // Add buttons for the traceback section
                    Some(
                        h_flex()
                            .pl_1()
                            .child(
                                IconButton::new(
                                    ElementId::Name("copy-full-error-traceback".into()),
                                    IconName::Copy,
                                )
                                .style(ButtonStyle::Transparent)
                                .tooltip(Tooltip::text("Copy Full Error"))
                                .on_click({
                                    let ename = err.ename.clone();
                                    let evalue = err.evalue.clone();
                                    let traceback = err.traceback.clone();
                                    move |_, _window, cx| {
                                        let traceback_text = traceback.read(cx).full_text();
                                        let full_error =
                                            format!("{}: {}\n{}", ename, evalue, traceback_text);
                                        let clipboard_content =
                                            ClipboardItem::new_string(full_error);
                                        cx.write_to_clipboard(clipboard_content);
                                    }
                                }),
                            )
                            .child(
                                IconButton::new(
                                    ElementId::Name("open-full-error-in-buffer-traceback".into()),
                                    IconName::FileTextOutlined,
                                )
                                .style(ButtonStyle::Transparent)
                                .tooltip(Tooltip::text("Open Full Error in Buffer"))
                                .on_click({
                                    let ename = err.ename.clone();
                                    let evalue = err.evalue.clone();
                                    let traceback = err.traceback.clone();
                                    move |_, window, cx| {
                                        if let Some(workspace) = workspace.upgrade() {
                                            let traceback_text = traceback.read(cx).full_text();
                                            let full_error = format!(
                                                "{}: {}\n{}",
                                                ename, evalue, traceback_text
                                            );
                                            let buffer = cx.new(|cx| {
                                                let mut buffer = Buffer::local(full_error, cx)
                                                    .with_language(
                                                        language::PLAIN_TEXT.clone(),
                                                        cx,
                                                    );
                                                buffer.set_capability(
                                                    language::Capability::ReadOnly,
                                                    cx,
                                                );
                                                buffer
                                            });
                                            let editor = Box::new(cx.new(|cx| {
                                                let multibuffer = cx.new(|cx| {
                                                    let mut multi_buffer =
                                                        MultiBuffer::singleton(buffer.clone(), cx);
                                                    multi_buffer
                                                        .set_title("Full Error".to_string(), cx);
                                                    multi_buffer
                                                });
                                                Editor::for_multibuffer(
                                                    multibuffer,
                                                    None,
                                                    window,
                                                    cx,
                                                )
                                            }));
                                            workspace.update(cx, |workspace, cx| {
                                                workspace.add_item_to_active_pane(
                                                    editor, None, true, window, cx,
                                                );
                                            });
                                        }
                                    }
                                }),
                            )
                            .into_any_element(),
                    )
                }
                Self::Message(_) => None,
                Self::Table { content, .. } => {
                    Self::render_output_controls(content.clone(), workspace, window, cx)
                }
                Self::ClearOutputWaitMarker => None,
            })
    }

    pub fn display_id(&self) -> Option<String> {
        match self {
            Output::Plain { display_id, .. } => display_id.clone(),
            Output::Stream { .. } => None,
            Output::Image { display_id, .. } => display_id.clone(),
            Output::ErrorOutput(_) => None,
            Output::Message(_) => None,
            Output::Table { display_id, .. } => display_id.clone(),
            Output::Markdown { display_id, .. } => display_id.clone(),
            Output::ClearOutputWaitMarker => None,
        }
    }

    pub fn new(
        data: &MimeBundle,
        display_id: Option<String>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match data.richest(rank_mime_type) {
            Some(MimeType::Plain(text)) => Output::Plain {
                content: cx.new(|cx| TerminalOutput::from(text, window, cx)),
                display_id,
            },
            Some(MimeType::Markdown(text)) => {
                let content = cx.new(|cx| MarkdownView::from(text.clone(), cx));
                Output::Markdown {
                    content,
                    display_id,
                }
            }
            Some(MimeType::Png(data)) | Some(MimeType::Jpeg(data)) => match ImageView::from(data) {
                Ok(view) => Output::Image {
                    content: cx.new(|_| view),
                    display_id,
                },
                Err(error) => Output::Message(format!("Failed to load image: {}", error)),
            },
            Some(MimeType::DataTable(data)) => Output::Table {
                content: cx.new(|cx| TableView::new(data, window, cx)),
                display_id,
            },
            // Any other media types are not supported
            _ => Output::Message("Unsupported media type".to_string()),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum ExecutionStatus {
    #[default]
    Unknown,
    ConnectingToKernel,
    Queued,
    Executing,
    Finished,
    ShuttingDown,
    Shutdown,
    KernelErrored(String),
    Restarting,
}

/// An ExecutionView shows the outputs of an execution.
/// It can hold zero or more outputs, which the user
/// sees as "the output" for a single execution.
pub struct ExecutionView {
    #[allow(unused)]
    workspace: WeakEntity<Workspace>,
    pub outputs: Vec<Output>,
    pub status: ExecutionStatus,
}

impl ExecutionView {
    pub fn new(
        status: ExecutionStatus,
        workspace: WeakEntity<Workspace>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            workspace,
            outputs: Default::default(),
            status,
        }
    }

    /// Accept a Jupyter message belonging to this execution
    pub fn push_message(
        &mut self,
        message: &JupyterMessageContent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let output: Output = match message {
            JupyterMessageContent::ExecuteResult(result) => Output::new(
                &result.data,
                result.transient.as_ref().and_then(|t| t.display_id.clone()),
                window,
                cx,
            ),
            JupyterMessageContent::DisplayData(result) => Output::new(
                &result.data,
                result.transient.as_ref().and_then(|t| t.display_id.clone()),
                window,
                cx,
            ),
            JupyterMessageContent::StreamContent(result) => {
                // Previous stream data will combine together, handling colors, carriage returns, etc
                if let Some(new_terminal) = self.apply_terminal_text(&result.text, window, cx) {
                    new_terminal
                } else {
                    return;
                }
            }
            JupyterMessageContent::ErrorOutput(result) => {
                let terminal =
                    cx.new(|cx| TerminalOutput::from(&result.traceback.join("\n"), window, cx));

                Output::ErrorOutput(ErrorView {
                    ename: result.ename.clone(),
                    evalue: result.evalue.clone(),
                    traceback: terminal,
                })
            }
            JupyterMessageContent::ExecuteReply(reply) => {
                for payload in reply.payload.iter() {
                    if let runtimelib::Payload::Page { data, .. } = payload {
                        let output = Output::new(data, None, window, cx);
                        self.outputs.push(output);
                    }
                }
                cx.notify();
                return;
            }
            JupyterMessageContent::ClearOutput(options) => {
                if !options.wait {
                    self.outputs.clear();
                    cx.notify();
                    return;
                }

                // Create a marker to clear the output after we get in a new output
                Output::ClearOutputWaitMarker
            }
            JupyterMessageContent::Status(status) => {
                match status.execution_state {
                    ExecutionState::Busy => {
                        self.status = ExecutionStatus::Executing;
                    }
                    ExecutionState::Idle => self.status = ExecutionStatus::Finished,
                }
                cx.notify();
                return;
            }
            _msg => {
                return;
            }
        };

        // Check for a clear output marker as the previous output, so we can clear it out
        if let Some(output) = self.outputs.last()
            && let Output::ClearOutputWaitMarker = output
        {
            self.outputs.clear();
        }

        self.outputs.push(output);

        cx.notify();
    }

    pub fn update_display_data(
        &mut self,
        data: &MimeBundle,
        display_id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut any = false;

        self.outputs.iter_mut().for_each(|output| {
            if let Some(other_display_id) = output.display_id().as_ref()
                && other_display_id == display_id
            {
                *output = Output::new(data, Some(display_id.to_owned()), window, cx);
                any = true;
            }
        });

        if any {
            cx.notify();
        }
    }

    fn apply_terminal_text(
        &mut self,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Output> {
        if let Some(last_output) = self.outputs.last_mut()
            && let Output::Stream {
                content: last_stream,
            } = last_output
        {
            // Don't need to add a new output, we already have a terminal output
            // and can just update the most recent terminal output
            last_stream.update(cx, |last_stream, cx| {
                last_stream.append_text(text, cx);
                cx.notify();
            });
            return None;
        }

        Some(Output::Stream {
            content: cx.new(|cx| TerminalOutput::from(text, window, cx)),
        })
    }
}

impl Render for ExecutionView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = match &self.status {
            ExecutionStatus::ConnectingToKernel => Label::new("Connecting to kernel...")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::Executing => h_flex()
                .gap_2()
                .child(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .with_rotate_animation(3),
                )
                .child(Label::new("Executing...").color(Color::Muted))
                .into_any_element(),
            ExecutionStatus::Finished => Icon::new(IconName::Check)
                .size(IconSize::Small)
                .into_any_element(),
            ExecutionStatus::Unknown => Label::new("Unknown status")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::ShuttingDown => Label::new("Kernel shutting down...")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::Restarting => Label::new("Kernel restarting...")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::Shutdown => Label::new("Kernel shutdown")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::Queued => Label::new("Queued...")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::KernelErrored(error) => Label::new(format!("Kernel error: {}", error))
                .color(Color::Error)
                .into_any_element(),
        };

        if self.outputs.is_empty() {
            return v_flex()
                .min_h(window.line_height())
                .justify_center()
                .child(status)
                .into_any_element();
        }

        div()
            .w_full()
            .children(
                self.outputs
                    .iter()
                    .map(|output| output.render(self.workspace.clone(), window, cx)),
            )
            .children(match self.status {
                ExecutionStatus::Executing => vec![status],
                ExecutionStatus::Queued => vec![status],
                _ => vec![],
            })
            .into_any_element()
    }
}
