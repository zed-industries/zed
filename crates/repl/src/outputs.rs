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
use gpui::{AnyElement, ClipboardItem, Entity, EventEmitter, Render, WeakEntity};
use language::Buffer;
use runtimelib::{ExecutionState, JupyterMessageContent, MimeBundle, MimeType};
use ui::{CommonAnimationExt, CopyButton, IconButton, Tooltip, prelude::*};

mod image;
use image::ImageView;

mod markdown;
use markdown::MarkdownView;

mod table;
use table::TableView;

mod json;
use json::JsonView;

pub mod plain;
use plain::TerminalOutput;

pub(crate) mod user_error;
use user_error::ErrorView;
use workspace::Workspace;

use crate::repl_settings::ReplSettings;
use settings::Settings;

/// When deciding what to render from a collection of mediatypes, we need to rank them in order of importance
fn rank_mime_type(mimetype: &MimeType) -> usize {
    match mimetype {
        MimeType::DataTable(_) => 6,
        MimeType::Json(_) => 5,
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
    Json {
        content: Entity<JsonView>,
        display_id: Option<String>,
    },
    ClearOutputWaitMarker,
}

impl Output {
    pub fn to_nbformat(&self, cx: &App) -> Option<nbformat::v4::Output> {
        match self {
            Output::Stream { content } => {
                let text = content.read(cx).full_text();
                Some(nbformat::v4::Output::Stream {
                    name: "stdout".to_string(),
                    text: nbformat::v4::MultilineString(text),
                })
            }
            Output::Plain { content, .. } => {
                let text = content.read(cx).full_text();
                let mut data = jupyter_protocol::media::Media::default();
                data.content.push(jupyter_protocol::MediaType::Plain(text));
                Some(nbformat::v4::Output::DisplayData(
                    nbformat::v4::DisplayData {
                        data,
                        metadata: serde_json::Map::new(),
                    },
                ))
            }
            Output::ErrorOutput(error_view) => {
                let traceback_text = error_view.traceback.read(cx).full_text();
                let traceback_lines: Vec<String> =
                    traceback_text.lines().map(|s| s.to_string()).collect();
                Some(nbformat::v4::Output::Error(nbformat::v4::ErrorOutput {
                    ename: error_view.ename.clone(),
                    evalue: error_view.evalue.clone(),
                    traceback: traceback_lines,
                }))
            }
            Output::Image { .. }
            | Output::Markdown { .. }
            | Output::Table { .. }
            | Output::Json { .. } => None,
            Output::Message(_) => None,
            Output::ClearOutputWaitMarker => None,
        }
    }
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
        let max_width = plain::max_width_for_columns(
            ReplSettings::get_global(cx).output_max_width_columns,
            window,
            cx,
        );
        let content = match self {
            Self::Plain { content, .. } => Some(content.clone().into_any_element()),
            Self::Markdown { content, .. } => Some(content.clone().into_any_element()),
            Self::Stream { content, .. } => Some(content.clone().into_any_element()),
            Self::Image { content, .. } => Some(content.clone().into_any_element()),
            Self::Message(message) => Some(div().child(message.clone()).into_any_element()),
            Self::Table { content, .. } => Some(content.clone().into_any_element()),
            Self::Json { content, .. } => Some(content.clone().into_any_element()),
            Self::ErrorOutput(error_view) => error_view.render(window, cx),
            Self::ClearOutputWaitMarker => None,
        };

        let needs_horizontal_scroll = matches!(self, Self::Table { .. } | Self::Image { .. });

        h_flex()
            .id("output-content")
            .w_full()
            .when_some(max_width, |this, max_w| this.max_w(max_w))
            .overflow_x_scroll()
            .items_start()
            .child(
                div()
                    .when(!needs_horizontal_scroll, |el| {
                        el.flex_1().w_full().overflow_x_hidden()
                    })
                    .children(content),
            )
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
                Self::Json { content, .. } => {
                    Self::render_output_controls(content.clone(), workspace, window, cx)
                }
                Self::ErrorOutput(err) => Some(
                    h_flex()
                        .pl_1()
                        .child({
                            let ename = err.ename.clone();
                            let evalue = err.evalue.clone();
                            let traceback = err.traceback.clone();
                            let traceback_text = traceback.read(cx).full_text();
                            let full_error = format!("{}: {}\n{}", ename, evalue, traceback_text);

                            CopyButton::new("copy-full-error", full_error)
                                .tooltip_label("Copy Full Error")
                        })
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
                                        let full_error =
                                            format!("{}: {}\n{}", ename, evalue, traceback_text);
                                        let buffer = cx.new(|cx| {
                                            let mut buffer = Buffer::local(full_error, cx)
                                                .with_language(language::PLAIN_TEXT.clone(), cx);
                                            buffer
                                                .set_capability(language::Capability::ReadOnly, cx);
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
                                            Editor::for_multibuffer(multibuffer, None, window, cx)
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
                ),
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
            Output::Json { display_id, .. } => display_id.clone(),
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
            Some(MimeType::Json(json_value)) => match JsonView::from_value(json_value.clone()) {
                Ok(json_view) => Output::Json {
                    content: cx.new(|_| json_view),
                    display_id,
                },
                Err(_) => Output::Message("Failed to parse JSON".to_string()),
            },
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

pub struct ExecutionViewFinishedEmpty;
pub struct ExecutionViewFinishedSmall(pub String);

/// An ExecutionView shows the outputs of an execution.
/// It can hold zero or more outputs, which the user
/// sees as "the output" for a single execution.
pub struct ExecutionView {
    #[allow(unused)]
    workspace: WeakEntity<Workspace>,
    pub outputs: Vec<Output>,
    pub status: ExecutionStatus,
}

impl EventEmitter<ExecutionViewFinishedEmpty> for ExecutionView {}
impl EventEmitter<ExecutionViewFinishedSmall> for ExecutionView {}

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
                    ExecutionState::Idle => {
                        self.status = ExecutionStatus::Finished;
                        if self.outputs.is_empty() {
                            cx.emit(ExecutionViewFinishedEmpty);
                        } else if ReplSettings::get_global(cx).inline_output {
                            if let Some(small_text) = self.get_small_inline_output(cx) {
                                cx.emit(ExecutionViewFinishedSmall(small_text));
                            }
                        }
                    }
                    ExecutionState::Unknown => self.status = ExecutionStatus::Unknown,
                    ExecutionState::Starting => self.status = ExecutionStatus::ConnectingToKernel,
                    ExecutionState::Restarting => self.status = ExecutionStatus::Restarting,
                    ExecutionState::Terminating => self.status = ExecutionStatus::ShuttingDown,
                    ExecutionState::AutoRestarting => self.status = ExecutionStatus::Restarting,
                    ExecutionState::Dead => self.status = ExecutionStatus::Shutdown,
                    ExecutionState::Other(_) => self.status = ExecutionStatus::Unknown,
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

    /// Check if the output is a single small plain text that can be shown inline.
    /// Returns the text if it's suitable for inline display (single line, short enough).
    fn get_small_inline_output(&self, cx: &App) -> Option<String> {
        // Only consider single outputs
        if self.outputs.len() != 1 {
            return None;
        }

        let output = self.outputs.first()?;

        // Only Plain outputs can be inlined
        let content = match output {
            Output::Plain { content, .. } => content,
            _ => return None,
        };

        let text = content.read(cx).full_text();
        let trimmed = text.trim();

        let max_length = ReplSettings::get_global(cx).inline_output_max_length;

        // Must be a single line and within the configured max length
        if trimmed.contains('\n') || trimmed.len() > max_length {
            return None;
        }

        Some(trimmed.to_string())
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

impl ExecutionView {
    #[cfg(test)]
    fn output_as_stream_text(&self, cx: &App) -> Option<String> {
        self.outputs.iter().find_map(|output| {
            if let Output::Stream { content } = output {
                Some(content.read(cx).full_text())
            } else {
                None
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use runtimelib::{
        ClearOutput, ErrorOutput, ExecutionState, JupyterMessageContent, MimeType, Status, Stdio,
        StreamContent,
    };
    use settings::SettingsStore;
    use std::path::Path;
    use std::sync::Arc;

    #[test]
    fn test_rank_mime_type_ordering() {
        let data_table = MimeType::DataTable(Box::default());
        let json = MimeType::Json(serde_json::json!({}));
        let png = MimeType::Png(String::new());
        let jpeg = MimeType::Jpeg(String::new());
        let markdown = MimeType::Markdown(String::new());
        let plain = MimeType::Plain(String::new());

        assert_eq!(rank_mime_type(&data_table), 6);
        assert_eq!(rank_mime_type(&json), 5);
        assert_eq!(rank_mime_type(&png), 4);
        assert_eq!(rank_mime_type(&jpeg), 3);
        assert_eq!(rank_mime_type(&markdown), 2);
        assert_eq!(rank_mime_type(&plain), 1);

        assert!(rank_mime_type(&data_table) > rank_mime_type(&json));
        assert!(rank_mime_type(&json) > rank_mime_type(&png));
        assert!(rank_mime_type(&png) > rank_mime_type(&jpeg));
        assert!(rank_mime_type(&jpeg) > rank_mime_type(&markdown));
        assert!(rank_mime_type(&markdown) > rank_mime_type(&plain));
    }

    #[test]
    fn test_rank_mime_type_unsupported_returns_zero() {
        let html = MimeType::Html(String::new());
        let svg = MimeType::Svg(String::new());
        let latex = MimeType::Latex(String::new());

        assert_eq!(rank_mime_type(&html), 0);
        assert_eq!(rank_mime_type(&svg), 0);
        assert_eq!(rank_mime_type(&latex), 0);
    }

    async fn init_test(
        cx: &mut TestAppContext,
    ) -> (gpui::VisualTestContext, WeakEntity<workspace::Workspace>) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
        let fs = project::FakeFs::new(cx.background_executor.clone());
        let project = project::Project::test(fs, [] as [&Path; 0], cx).await;
        let window =
            cx.add_window(|window, cx| workspace::Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).expect("workspace should exist");
        let weak_workspace = workspace.downgrade();
        let visual_cx = gpui::VisualTestContext::from_window(window.into(), cx);
        (visual_cx, weak_workspace)
    }

    fn create_execution_view(
        cx: &mut gpui::VisualTestContext,
        weak_workspace: WeakEntity<workspace::Workspace>,
    ) -> Entity<ExecutionView> {
        cx.update(|_window, cx| {
            cx.new(|cx| ExecutionView::new(ExecutionStatus::Queued, weak_workspace, cx))
        })
    }

    #[gpui::test]
    async fn test_push_message_stream_content(cx: &mut TestAppContext) {
        let (mut cx, workspace) = init_test(cx).await;
        let execution_view = create_execution_view(&mut cx, workspace);

        cx.update(|window, cx| {
            execution_view.update(cx, |view, cx| {
                let message = JupyterMessageContent::StreamContent(StreamContent {
                    name: Stdio::Stdout,
                    text: "hello world\n".to_string(),
                });
                view.push_message(&message, window, cx);
            });
        });

        cx.update(|_, cx| {
            let view = execution_view.read(cx);
            assert_eq!(view.outputs.len(), 1);
            assert!(matches!(view.outputs[0], Output::Stream { .. }));
            let text = view.output_as_stream_text(cx);
            assert!(text.is_some());
            assert!(text.as_ref().is_some_and(|t| t.contains("hello world")));
        });
    }

    #[gpui::test]
    async fn test_push_message_stream_appends(cx: &mut TestAppContext) {
        let (mut cx, workspace) = init_test(cx).await;
        let execution_view = create_execution_view(&mut cx, workspace);

        cx.update(|window, cx| {
            execution_view.update(cx, |view, cx| {
                let message1 = JupyterMessageContent::StreamContent(StreamContent {
                    name: Stdio::Stdout,
                    text: "first ".to_string(),
                });
                let message2 = JupyterMessageContent::StreamContent(StreamContent {
                    name: Stdio::Stdout,
                    text: "second".to_string(),
                });
                view.push_message(&message1, window, cx);
                view.push_message(&message2, window, cx);
            });
        });

        cx.update(|_, cx| {
            let view = execution_view.read(cx);
            assert_eq!(
                view.outputs.len(),
                1,
                "consecutive streams should merge into one output"
            );
            let text = view.output_as_stream_text(cx);
            assert!(text.as_ref().is_some_and(|t| t.contains("first ")));
            assert!(text.as_ref().is_some_and(|t| t.contains("second")));
        });
    }

    #[gpui::test]
    async fn test_push_message_error_output(cx: &mut TestAppContext) {
        let (mut cx, workspace) = init_test(cx).await;
        let execution_view = create_execution_view(&mut cx, workspace);

        cx.update(|window, cx| {
            execution_view.update(cx, |view, cx| {
                let message = JupyterMessageContent::ErrorOutput(ErrorOutput {
                    ename: "NameError".to_string(),
                    evalue: "name 'x' is not defined".to_string(),
                    traceback: vec![
                        "Traceback (most recent call last):".to_string(),
                        "NameError: name 'x' is not defined".to_string(),
                    ],
                });
                view.push_message(&message, window, cx);
            });
        });

        cx.update(|_, cx| {
            let view = execution_view.read(cx);
            assert_eq!(view.outputs.len(), 1);
            match &view.outputs[0] {
                Output::ErrorOutput(error_view) => {
                    assert_eq!(error_view.ename, "NameError");
                    assert_eq!(error_view.evalue, "name 'x' is not defined");
                }
                other => panic!(
                    "expected ErrorOutput, got {:?}",
                    std::mem::discriminant(other)
                ),
            }
        });
    }

    #[gpui::test]
    async fn test_push_message_clear_output_immediate(cx: &mut TestAppContext) {
        let (mut cx, workspace) = init_test(cx).await;
        let execution_view = create_execution_view(&mut cx, workspace);

        cx.update(|window, cx| {
            execution_view.update(cx, |view, cx| {
                let stream = JupyterMessageContent::StreamContent(StreamContent {
                    name: Stdio::Stdout,
                    text: "some output\n".to_string(),
                });
                view.push_message(&stream, window, cx);
                assert_eq!(view.outputs.len(), 1);

                let clear = JupyterMessageContent::ClearOutput(ClearOutput { wait: false });
                view.push_message(&clear, window, cx);
                assert_eq!(
                    view.outputs.len(),
                    0,
                    "immediate clear should remove all outputs"
                );
            });
        });
    }

    #[gpui::test]
    async fn test_push_message_clear_output_deferred(cx: &mut TestAppContext) {
        let (mut cx, workspace) = init_test(cx).await;
        let execution_view = create_execution_view(&mut cx, workspace);

        cx.update(|window, cx| {
            execution_view.update(cx, |view, cx| {
                let stream = JupyterMessageContent::StreamContent(StreamContent {
                    name: Stdio::Stdout,
                    text: "old output\n".to_string(),
                });
                view.push_message(&stream, window, cx);
                assert_eq!(view.outputs.len(), 1);

                let clear = JupyterMessageContent::ClearOutput(ClearOutput { wait: true });
                view.push_message(&clear, window, cx);
                assert_eq!(view.outputs.len(), 2, "deferred clear adds a wait marker");
                assert!(matches!(view.outputs[1], Output::ClearOutputWaitMarker));

                let new_stream = JupyterMessageContent::StreamContent(StreamContent {
                    name: Stdio::Stdout,
                    text: "new output\n".to_string(),
                });
                view.push_message(&new_stream, window, cx);
                assert_eq!(
                    view.outputs.len(),
                    1,
                    "next output after wait marker should clear previous outputs"
                );
            });
        });
    }

    #[gpui::test]
    async fn test_push_message_status_transitions(cx: &mut TestAppContext) {
        let (mut cx, workspace) = init_test(cx).await;
        let execution_view = create_execution_view(&mut cx, workspace);

        cx.update(|window, cx| {
            execution_view.update(cx, |view, cx| {
                let busy = JupyterMessageContent::Status(Status {
                    execution_state: ExecutionState::Busy,
                });
                view.push_message(&busy, window, cx);
                assert!(matches!(view.status, ExecutionStatus::Executing));

                let idle = JupyterMessageContent::Status(Status {
                    execution_state: ExecutionState::Idle,
                });
                view.push_message(&idle, window, cx);
                assert!(matches!(view.status, ExecutionStatus::Finished));

                let starting = JupyterMessageContent::Status(Status {
                    execution_state: ExecutionState::Starting,
                });
                view.push_message(&starting, window, cx);
                assert!(matches!(view.status, ExecutionStatus::ConnectingToKernel));

                let dead = JupyterMessageContent::Status(Status {
                    execution_state: ExecutionState::Dead,
                });
                view.push_message(&dead, window, cx);
                assert!(matches!(view.status, ExecutionStatus::Shutdown));

                let restarting = JupyterMessageContent::Status(Status {
                    execution_state: ExecutionState::Restarting,
                });
                view.push_message(&restarting, window, cx);
                assert!(matches!(view.status, ExecutionStatus::Restarting));

                let terminating = JupyterMessageContent::Status(Status {
                    execution_state: ExecutionState::Terminating,
                });
                view.push_message(&terminating, window, cx);
                assert!(matches!(view.status, ExecutionStatus::ShuttingDown));
            });
        });
    }

    #[gpui::test]
    async fn test_push_message_status_idle_emits_finished_empty(cx: &mut TestAppContext) {
        let (mut cx, workspace) = init_test(cx).await;
        let execution_view = create_execution_view(&mut cx, workspace);

        let emitted = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let emitted_clone = emitted.clone();

        cx.update(|_, cx| {
            cx.subscribe(
                &execution_view,
                move |_, _event: &ExecutionViewFinishedEmpty, _cx| {
                    emitted_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                },
            )
            .detach();
        });

        cx.update(|window, cx| {
            execution_view.update(cx, |view, cx| {
                assert!(view.outputs.is_empty());
                let idle = JupyterMessageContent::Status(Status {
                    execution_state: ExecutionState::Idle,
                });
                view.push_message(&idle, window, cx);
            });
        });

        assert!(
            emitted.load(std::sync::atomic::Ordering::SeqCst),
            "should emit ExecutionViewFinishedEmpty when idle with no outputs"
        );
    }
}
