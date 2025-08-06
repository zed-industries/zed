// Translates old acp agents into the new schema
use agent_client_protocol as acp;
use agentic_coding_protocol::{self as acp_old, AgentRequest as _};
use anyhow::{Context as _, Result, anyhow};
use futures::channel::oneshot;
use gpui::{AppContext as _, AsyncApp, Entity, Task, WeakEntity};
use project::Project;
use std::{cell::RefCell, path::Path, rc::Rc};
use ui::App;
use util::ResultExt as _;

use crate::AgentServerCommand;
use acp_thread::{AcpThread, AgentConnection, AuthRequired};

#[derive(Clone)]
struct OldAcpClientDelegate {
    thread: Rc<RefCell<WeakEntity<AcpThread>>>,
    cx: AsyncApp,
    next_tool_call_id: Rc<RefCell<u64>>,
    // sent_buffer_versions: HashMap<Entity<Buffer>, HashMap<u64, BufferSnapshot>>,
}

impl OldAcpClientDelegate {
    fn new(thread: Rc<RefCell<WeakEntity<AcpThread>>>, cx: AsyncApp) -> Self {
        Self {
            thread,
            cx,
            next_tool_call_id: Rc::new(RefCell::new(0)),
        }
    }
}

impl acp_old::Client for OldAcpClientDelegate {
    async fn stream_assistant_message_chunk(
        &self,
        params: acp_old::StreamAssistantMessageChunkParams,
    ) -> Result<(), acp_old::Error> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.thread
                .borrow()
                .update(cx, |thread, cx| match params.chunk {
                    acp_old::AssistantMessageChunk::Text { text } => {
                        thread.push_assistant_content_block(text.into(), false, cx)
                    }
                    acp_old::AssistantMessageChunk::Thought { thought } => {
                        thread.push_assistant_content_block(thought.into(), true, cx)
                    }
                })
                .log_err();
        })?;

        Ok(())
    }

    async fn request_tool_call_confirmation(
        &self,
        request: acp_old::RequestToolCallConfirmationParams,
    ) -> Result<acp_old::RequestToolCallConfirmationResponse, acp_old::Error> {
        let cx = &mut self.cx.clone();

        let old_acp_id = *self.next_tool_call_id.borrow() + 1;
        self.next_tool_call_id.replace(old_acp_id);

        let tool_call = into_new_tool_call(
            acp::ToolCallId(old_acp_id.to_string().into()),
            request.tool_call,
        );

        let mut options = match request.confirmation {
            acp_old::ToolCallConfirmation::Edit { .. } => vec![(
                acp_old::ToolCallConfirmationOutcome::AlwaysAllow,
                acp::PermissionOptionKind::AllowAlways,
                "Always Allow Edits".to_string(),
            )],
            acp_old::ToolCallConfirmation::Execute { root_command, .. } => vec![(
                acp_old::ToolCallConfirmationOutcome::AlwaysAllow,
                acp::PermissionOptionKind::AllowAlways,
                format!("Always Allow {}", root_command),
            )],
            acp_old::ToolCallConfirmation::Mcp {
                server_name,
                tool_name,
                ..
            } => vec![
                (
                    acp_old::ToolCallConfirmationOutcome::AlwaysAllowMcpServer,
                    acp::PermissionOptionKind::AllowAlways,
                    format!("Always Allow {}", server_name),
                ),
                (
                    acp_old::ToolCallConfirmationOutcome::AlwaysAllowTool,
                    acp::PermissionOptionKind::AllowAlways,
                    format!("Always Allow {}", tool_name),
                ),
            ],
            acp_old::ToolCallConfirmation::Fetch { .. } => vec![(
                acp_old::ToolCallConfirmationOutcome::AlwaysAllow,
                acp::PermissionOptionKind::AllowAlways,
                "Always Allow".to_string(),
            )],
            acp_old::ToolCallConfirmation::Other { .. } => vec![(
                acp_old::ToolCallConfirmationOutcome::AlwaysAllow,
                acp::PermissionOptionKind::AllowAlways,
                "Always Allow".to_string(),
            )],
        };

        options.extend([
            (
                acp_old::ToolCallConfirmationOutcome::Allow,
                acp::PermissionOptionKind::AllowOnce,
                "Allow".to_string(),
            ),
            (
                acp_old::ToolCallConfirmationOutcome::Reject,
                acp::PermissionOptionKind::RejectOnce,
                "Reject".to_string(),
            ),
        ]);

        let mut outcomes = Vec::with_capacity(options.len());
        let mut acp_options = Vec::with_capacity(options.len());

        for (index, (outcome, kind, label)) in options.into_iter().enumerate() {
            outcomes.push(outcome);
            acp_options.push(acp::PermissionOption {
                id: acp::PermissionOptionId(index.to_string().into()),
                name: label,
                kind,
            })
        }

        let response = cx
            .update(|cx| {
                self.thread.borrow().update(cx, |thread, cx| {
                    thread.request_tool_call_permission(tool_call, acp_options, cx)
                })
            })?
            .context("Failed to update thread")?
            .await;

        let outcome = match response {
            Ok(option_id) => outcomes[option_id.0.parse::<usize>().unwrap_or(0)],
            Err(oneshot::Canceled) => acp_old::ToolCallConfirmationOutcome::Cancel,
        };

        Ok(acp_old::RequestToolCallConfirmationResponse {
            id: acp_old::ToolCallId(old_acp_id),
            outcome: outcome,
        })
    }

    async fn push_tool_call(
        &self,
        request: acp_old::PushToolCallParams,
    ) -> Result<acp_old::PushToolCallResponse, acp_old::Error> {
        let cx = &mut self.cx.clone();

        let old_acp_id = *self.next_tool_call_id.borrow() + 1;
        self.next_tool_call_id.replace(old_acp_id);

        cx.update(|cx| {
            self.thread.borrow().update(cx, |thread, cx| {
                thread.upsert_tool_call(
                    into_new_tool_call(acp::ToolCallId(old_acp_id.to_string().into()), request),
                    cx,
                )
            })
        })?
        .context("Failed to update thread")?;

        Ok(acp_old::PushToolCallResponse {
            id: acp_old::ToolCallId(old_acp_id),
        })
    }

    async fn update_tool_call(
        &self,
        request: acp_old::UpdateToolCallParams,
    ) -> Result<(), acp_old::Error> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.thread.borrow().update(cx, |thread, cx| {
                thread.update_tool_call(
                    acp::ToolCallUpdate {
                        id: acp::ToolCallId(request.tool_call_id.0.to_string().into()),
                        fields: acp::ToolCallUpdateFields {
                            status: Some(into_new_tool_call_status(request.status)),
                            content: Some(
                                request
                                    .content
                                    .into_iter()
                                    .map(into_new_tool_call_content)
                                    .collect::<Vec<_>>(),
                            ),
                            ..Default::default()
                        },
                    },
                    cx,
                )
            })
        })?
        .context("Failed to update thread")??;

        Ok(())
    }

    async fn update_plan(&self, request: acp_old::UpdatePlanParams) -> Result<(), acp_old::Error> {
        let cx = &mut self.cx.clone();

        cx.update(|cx| {
            self.thread.borrow().update(cx, |thread, cx| {
                thread.update_plan(
                    acp::Plan {
                        entries: request
                            .entries
                            .into_iter()
                            .map(into_new_plan_entry)
                            .collect(),
                    },
                    cx,
                )
            })
        })?
        .context("Failed to update thread")?;

        Ok(())
    }

    async fn read_text_file(
        &self,
        acp_old::ReadTextFileParams { path, line, limit }: acp_old::ReadTextFileParams,
    ) -> Result<acp_old::ReadTextFileResponse, acp_old::Error> {
        let content = self
            .cx
            .update(|cx| {
                self.thread.borrow().update(cx, |thread, cx| {
                    thread.read_text_file(path, line, limit, false, cx)
                })
            })?
            .context("Failed to update thread")?
            .await?;
        Ok(acp_old::ReadTextFileResponse { content })
    }

    async fn write_text_file(
        &self,
        acp_old::WriteTextFileParams { path, content }: acp_old::WriteTextFileParams,
    ) -> Result<(), acp_old::Error> {
        self.cx
            .update(|cx| {
                self.thread
                    .borrow()
                    .update(cx, |thread, cx| thread.write_text_file(path, content, cx))
            })?
            .context("Failed to update thread")?
            .await?;

        Ok(())
    }
}

fn into_new_tool_call(id: acp::ToolCallId, request: acp_old::PushToolCallParams) -> acp::ToolCall {
    acp::ToolCall {
        id: id,
        title: request.label,
        kind: acp_kind_from_old_icon(request.icon),
        status: acp::ToolCallStatus::InProgress,
        content: request
            .content
            .into_iter()
            .map(into_new_tool_call_content)
            .collect(),
        locations: request
            .locations
            .into_iter()
            .map(into_new_tool_call_location)
            .collect(),
        raw_input: None,
    }
}

fn acp_kind_from_old_icon(icon: acp_old::Icon) -> acp::ToolKind {
    match icon {
        acp_old::Icon::FileSearch => acp::ToolKind::Search,
        acp_old::Icon::Folder => acp::ToolKind::Search,
        acp_old::Icon::Globe => acp::ToolKind::Search,
        acp_old::Icon::Hammer => acp::ToolKind::Other,
        acp_old::Icon::LightBulb => acp::ToolKind::Think,
        acp_old::Icon::Pencil => acp::ToolKind::Edit,
        acp_old::Icon::Regex => acp::ToolKind::Search,
        acp_old::Icon::Terminal => acp::ToolKind::Execute,
    }
}

fn into_new_tool_call_status(status: acp_old::ToolCallStatus) -> acp::ToolCallStatus {
    match status {
        acp_old::ToolCallStatus::Running => acp::ToolCallStatus::InProgress,
        acp_old::ToolCallStatus::Finished => acp::ToolCallStatus::Completed,
        acp_old::ToolCallStatus::Error => acp::ToolCallStatus::Failed,
    }
}

fn into_new_tool_call_content(content: acp_old::ToolCallContent) -> acp::ToolCallContent {
    match content {
        acp_old::ToolCallContent::Markdown { markdown } => markdown.into(),
        acp_old::ToolCallContent::Diff { diff } => acp::ToolCallContent::Diff {
            diff: into_new_diff(diff),
        },
    }
}

fn into_new_diff(diff: acp_old::Diff) -> acp::Diff {
    acp::Diff {
        path: diff.path,
        old_text: diff.old_text,
        new_text: diff.new_text,
    }
}

fn into_new_tool_call_location(location: acp_old::ToolCallLocation) -> acp::ToolCallLocation {
    acp::ToolCallLocation {
        path: location.path,
        line: location.line,
    }
}

fn into_new_plan_entry(entry: acp_old::PlanEntry) -> acp::PlanEntry {
    acp::PlanEntry {
        content: entry.content,
        priority: into_new_plan_priority(entry.priority),
        status: into_new_plan_status(entry.status),
    }
}

fn into_new_plan_priority(priority: acp_old::PlanEntryPriority) -> acp::PlanEntryPriority {
    match priority {
        acp_old::PlanEntryPriority::Low => acp::PlanEntryPriority::Low,
        acp_old::PlanEntryPriority::Medium => acp::PlanEntryPriority::Medium,
        acp_old::PlanEntryPriority::High => acp::PlanEntryPriority::High,
    }
}

fn into_new_plan_status(status: acp_old::PlanEntryStatus) -> acp::PlanEntryStatus {
    match status {
        acp_old::PlanEntryStatus::Pending => acp::PlanEntryStatus::Pending,
        acp_old::PlanEntryStatus::InProgress => acp::PlanEntryStatus::InProgress,
        acp_old::PlanEntryStatus::Completed => acp::PlanEntryStatus::Completed,
    }
}

pub struct AcpConnection {
    pub name: &'static str,
    pub connection: acp_old::AgentConnection,
    pub _child_status: Task<Result<()>>,
    pub current_thread: Rc<RefCell<WeakEntity<AcpThread>>>,
}

impl AcpConnection {
    pub fn stdio(
        name: &'static str,
        command: AgentServerCommand,
        root_dir: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Self>> {
        let root_dir = root_dir.to_path_buf();

        cx.spawn(async move |cx| {
            let mut child = util::command::new_smol_command(&command.path)
                .args(command.args.iter())
                .current_dir(root_dir)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::inherit())
                .kill_on_drop(true)
                .spawn()?;

            let stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();
            log::trace!("Spawned (pid: {})", child.id());

            let foreground_executor = cx.foreground_executor().clone();

            let thread_rc = Rc::new(RefCell::new(WeakEntity::new_invalid()));

            let (connection, io_fut) = acp_old::AgentConnection::connect_to_agent(
                OldAcpClientDelegate::new(thread_rc.clone(), cx.clone()),
                stdin,
                stdout,
                move |fut| foreground_executor.spawn(fut).detach(),
            );

            let io_task = cx.background_spawn(async move {
                io_fut.await.log_err();
            });

            let child_status = cx.background_spawn(async move {
                let result = match child.status().await {
                    Err(e) => Err(anyhow!(e)),
                    Ok(result) if result.success() => Ok(()),
                    Ok(result) => Err(anyhow!(result)),
                };
                drop(io_task);
                result
            });

            Ok(Self {
                name,
                connection,
                _child_status: child_status,
                current_thread: thread_rc,
            })
        })
    }
}

impl AgentConnection for AcpConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        _cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>> {
        let task = self.connection.request_any(
            acp_old::InitializeParams {
                protocol_version: acp_old::ProtocolVersion::latest(),
            }
            .into_any(),
        );
        let current_thread = self.current_thread.clone();
        cx.spawn(async move |cx| {
            let result = task.await?;
            let result = acp_old::InitializeParams::response_from_any(result)?;

            if !result.is_authenticated {
                anyhow::bail!(AuthRequired)
            }

            cx.update(|cx| {
                let thread = cx.new(|cx| {
                    let session_id = acp::SessionId("acp-old-no-id".into());
                    AcpThread::new(self.name, self.clone(), project, session_id, cx)
                });
                current_thread.replace(thread.downgrade());
                thread
            })
        })
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &[]
    }

    fn authenticate(&self, _method_id: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>> {
        let task = self
            .connection
            .request_any(acp_old::AuthenticateParams.into_any());
        cx.foreground_executor().spawn(async move {
            task.await?;
            Ok(())
        })
    }

    fn prompt(&self, params: acp::PromptRequest, cx: &mut App) -> Task<Result<()>> {
        let chunks = params
            .prompt
            .into_iter()
            .filter_map(|block| match block {
                acp::ContentBlock::Text(text) => {
                    Some(acp_old::UserMessageChunk::Text { text: text.text })
                }
                acp::ContentBlock::ResourceLink(link) => Some(acp_old::UserMessageChunk::Path {
                    path: link.uri.into(),
                }),
                _ => None,
            })
            .collect();

        let task = self
            .connection
            .request_any(acp_old::SendUserMessageParams { chunks }.into_any());
        cx.foreground_executor().spawn(async move {
            task.await?;
            anyhow::Ok(())
        })
    }

    fn cancel(&self, _session_id: &acp::SessionId, cx: &mut App) {
        let task = self
            .connection
            .request_any(acp_old::CancelSendMessageParams.into_any());
        cx.foreground_executor()
            .spawn(async move {
                task.await?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx)
    }
}
