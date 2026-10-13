use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    fs::{self, DirBuilder, Permissions},
    io::{BufRead, BufReader, Read, Write},
    os::unix::{
        fs::{DirBuilderExt, FileTypeExt, PermissionsExt},
        net::{UnixListener, UnixStream},
    },
    path::{Component, Path, PathBuf},
    rc::Rc,
    sync::mpsc,
    time::Duration,
};

use acp_thread::{
    AcpThread, AcpThreadEvent, AgentThreadEntry, AuthorizationKind, SelectedPermissionOutcome,
    ThreadStatus,
};
use agent_client_protocol::schema::v2 as acp;
use agent_settings::{AgentProfile, AgentProfileId};
use anyhow::{Context as _, Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use dap::SteppingGranularity;
use debugger_ui::debugger_panel::DebugPanel;
use git::{
    repository::{DiffType, RepoPath},
    status::{FileStatus, StageStatus},
};
use gpui::{App, AppContext, ClipboardItem, Entity, Global, Subscription, Task};
use language_model::LanguageModelRegistry;
use project::{TaskSourceKind, debugger::session::ThreadId};
use serde::Deserialize;
use serde_json::{Value, json};
use settings::SettingsStore;
use sha2::{Digest, Sha256};
use task::{
    DebugScenario, ResolvedTask, SharedTaskContext, TaskContext, TaskVariables, VariableName,
};
use terminal::TaskStatus;
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use theme::ActiveTheme;
use util::rel_path::RelPath;

use crate::agent_panel::{AgentPanel, CreateThreadOptions};
use crate::conversation_view::{ConversationView, ThreadView};
use crate::{Agent, AgentThreadSource};

const MAX_REQUEST_BYTES: u64 = 2 * 1024 * 1024;
const MAX_TRANSCRIPT_PAGE_BYTES: usize = 256 * 1024;
const MAX_TRANSCRIPT_PAGE_ENTRIES: usize = 128;
const MAX_ACTIVITY_BYTES: usize = 256 * 1024;
const MAX_SUBAGENT_SNAPSHOT_BYTES: usize = 64 * 1024;
const MAX_SUBAGENTS_PER_SNAPSHOT: usize = 32;
const MAX_APPROVAL_INPUT_BYTES: usize = 8192;
const MAX_THREAD_TITLE_BYTES: usize = 128;
const MAX_PROJECT_FILES: usize = 1000;
const MAX_FILE_BYTES: usize = 256 * 1024;
const MAX_SEARCH_FILES: usize = 500;
const MAX_SEARCH_BYTES: usize = 8 * 1024 * 1024;
const MAX_SEARCH_MATCHES: usize = 200;
const MAX_UPLOAD_BYTES: usize = 1024 * 1024;
const MAX_ATTACHMENTS: usize = 8;
const MAX_DIFF_BYTES: usize = 512 * 1024;
const MAX_TERMINAL_SNAPSHOT_BYTES: usize = 256 * 1024;
const MAX_TERMINAL_INPUT_BYTES: usize = 16 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProbeRequest {
    version: u8,
    request_id: String,
    method: String,
    params: Value,
}

struct ProbeCommand {
    request: ProbeRequest,
    response: mpsc::Sender<Value>,
}

struct ProbeState {
    project_root: PathBuf,
    permissions: Cell<ProbePermissions>,
    instance_id: String,
    epoch: String,
    submissions: RefCell<HashMap<String, SubmissionRecord>>,
    creations: RefCell<HashMap<String, CreationRecord>>,
    exposed_threads: RefCell<HashSet<String>>,
    subscriptions: RefCell<HashMap<String, Subscription>>,
    events: RefCell<HashMap<String, ThreadEvents>>,
    diffs: RefCell<HashMap<String, DiffRecord>>,
    searches: RefCell<HashMap<String, SearchRecord>>,
    change_operations: RefCell<HashMap<String, ChangeOperationRecord>>,
    terminal_operations: RefCell<HashMap<String, String>>,
    clipboard_operations: RefCell<HashMap<String, String>>,
}

#[derive(Clone, Copy)]
struct ProbePermissions {
    session_creation: bool,
    attachments: bool,
    project_context: bool,
    git_changes: bool,
    approval_decisions: bool,
    agent_settings: bool,
    terminal_read: bool,
    terminal_input: bool,
    remote_clipboard: bool,
    task_control: bool,
    task_start: bool,
    debugger_read: bool,
    debugger_start: bool,
    debugger_control: bool,
}

impl ProbePermissions {
    fn from_settings(cx: &App) -> Self {
        let settings = cx
            .global::<SettingsStore>()
            .merged_settings()
            .zedlink
            .clone();
        Self {
            session_creation: settings
                .as_ref()
                .and_then(|settings| settings.allow_session_creation)
                .unwrap_or(true),
            attachments: settings
                .as_ref()
                .and_then(|settings| settings.allow_attachments)
                .unwrap_or(true),
            project_context: settings
                .as_ref()
                .and_then(|settings| settings.allow_project_context)
                .unwrap_or(true),
            git_changes: settings
                .as_ref()
                .and_then(|settings| settings.allow_git_changes)
                .unwrap_or(true),
            approval_decisions: settings
                .as_ref()
                .and_then(|settings| settings.allow_approval_decisions)
                .unwrap_or(true),
            agent_settings: settings
                .as_ref()
                .and_then(|settings| settings.allow_agent_settings)
                .unwrap_or(true),
            terminal_read: settings
                .as_ref()
                .and_then(|settings| settings.allow_terminal_read)
                .unwrap_or(false),
            terminal_input: settings
                .as_ref()
                .and_then(|settings| settings.allow_terminal_input)
                .unwrap_or(false),
            remote_clipboard: settings
                .as_ref()
                .and_then(|settings| settings.allow_remote_clipboard)
                .unwrap_or(false),
            task_control: settings
                .as_ref()
                .and_then(|settings| settings.allow_task_control)
                .unwrap_or(false),
            task_start: settings
                .as_ref()
                .and_then(|settings| settings.allow_task_start)
                .unwrap_or(false),
            debugger_read: settings
                .as_ref()
                .and_then(|settings| settings.allow_debugger_read)
                .unwrap_or(false),
            debugger_start: settings
                .as_ref()
                .and_then(|settings| settings.allow_debugger_start)
                .unwrap_or(false),
            debugger_control: settings
                .as_ref()
                .and_then(|settings| settings.allow_debugger_control)
                .unwrap_or(false),
        }
    }

    fn capabilities(self) -> Vec<&'static str> {
        let mut capabilities = vec![
            "thread.list",
            "thread.snapshot",
            "thread.history",
            "thread.events",
            "thread.commands",
            "thread.settings",
            "thread.elicitations",
            "thread.queue",
            "thread.edit_review",
            "thread.send",
            "thread.cancel",
            "thread.subagents",
            "subagent.cancel",
            "thread.approvals",
            "request.status",
        ];
        if self.session_creation {
            capabilities.extend([
                "thread.create",
                "thread.create.options",
                "thread.rename",
                "thread.unexpose",
            ]);
        }
        if self.attachments {
            capabilities.extend(["thread.attachments", "thread.rewind_image"]);
            capabilities.push("thread.rewind");
        }
        if self.project_context {
            capabilities.extend([
                "project.files",
                "project.directory",
                "project.file",
                "project.search",
            ]);
        }
        if self.git_changes {
            capabilities.extend([
                "changes.snapshot",
                "changes.diff",
                "changes.stage",
                "changes.unstage",
            ]);
        }
        if self.approval_decisions {
            capabilities.push("approval.respond");
        }
        if self.agent_settings {
            capabilities.push("thread.settings.update");
            capabilities.extend([
                "elicitation.respond",
                "thread.queue.clear",
                "thread.edit_review.respond",
            ]);
        }
        if self.terminal_read {
            capabilities.extend(["terminal.list", "terminal.snapshot"]);
        }
        if self.terminal_read && self.terminal_input {
            capabilities.push("terminal.input");
        }
        if self.remote_clipboard {
            capabilities.extend(["clipboard.read", "clipboard.write"]);
        }
        if self.task_control {
            capabilities.extend(["task.list", "task.stop"]);
        }
        if self.task_start {
            capabilities.extend(["task.catalog", "task.run"]);
        }
        if self.debugger_read {
            capabilities.push("debugger.snapshot");
        }
        if self.debugger_start {
            capabilities.extend(["debugger.catalog", "debugger.launch"]);
        }
        if self.debugger_control {
            capabilities.push("debugger.control");
        }
        capabilities
    }

    fn allows(self, method: &str, params: &Value) -> bool {
        match method {
            "thread.create" | "thread.create.options" | "thread.rename" | "thread.unexpose" => {
                self.session_creation
            }
            "thread.rewind_image" | "thread.rewind" => self.attachments,
            "thread.send"
                if params.get("context_paths").is_some() || params.get("uploads").is_some() =>
            {
                self.attachments
            }
            "project.files" | "project.directory" | "project.file" | "project.search" => {
                self.project_context
            }
            "changes.snapshot" | "changes.diff" | "changes.stage" | "changes.unstage" => {
                self.git_changes
            }
            "approval.respond" | "elicitation.respond" | "thread.edit_review.respond" => {
                self.approval_decisions
            }
            "thread.settings.update" | "thread.queue.clear" => self.agent_settings,
            "terminal.list" | "terminal.snapshot" => self.terminal_read,
            "terminal.input" => self.terminal_read && self.terminal_input,
            "clipboard.read" | "clipboard.write" => self.remote_clipboard,
            "task.list" | "task.stop" => self.task_control,
            "task.catalog" | "task.run" => self.task_start,
            "debugger.snapshot" => self.debugger_read,
            "debugger.catalog" | "debugger.launch" => self.debugger_start,
            "debugger.control" => self.debugger_control,
            _ => true,
        }
    }
}

#[derive(Default)]
struct ThreadEvents {
    last_sequence: u64,
    records: VecDeque<Value>,
}

struct ProbeGlobal(Rc<ProbeState>);

impl Global for ProbeGlobal {}

struct SubmissionRecord {
    thread_id: String,
    payload_digest: String,
    outcome: &'static str,
}

struct CreationRecord {
    title: Option<String>,
    agent_id: String,
    thread_id: Option<String>,
}

enum DiffRecord {
    Loading {
        revision: String,
    },
    Ready {
        revision: String,
        diff: String,
        truncated: bool,
    },
    Failed {
        revision: String,
    },
}

enum SearchRecord {
    Loading,
    Ready {
        matches: Vec<Value>,
        truncated: bool,
    },
}

struct ChangeOperationRecord {
    payload_digest: String,
    state: ChangeOperationState,
}

enum ChangeOperationState {
    Running,
    Succeeded,
    Failed,
}

pub fn init(cx: &mut App) {
    let settings = cx
        .global::<SettingsStore>()
        .merged_settings()
        .zedlink
        .clone();
    let permissions = ProbePermissions::from_settings(cx);
    let gateway_manager = settings.as_ref().and_then(|settings| {
        if !settings.start_gateway_on_launch.unwrap_or(true) {
            return None;
        }
        settings
            .manager_path
            .as_deref()
            .map(str::trim)
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
    });
    let environment_paths = || {
        Some((
            PathBuf::from(std::env::var_os("ZEDLINK_PROBE_DIR")?),
            PathBuf::from(std::env::var_os("ZEDLINK_PROBE_PROJECT_ROOT")?),
        ))
    };
    let settings_paths = || {
        let settings = settings.as_ref()?;
        if !settings.enabled.unwrap_or(false) {
            return None;
        }
        let state_directory = settings.state_directory.as_deref()?.trim();
        let project_root = settings.project_root.as_deref()?.trim();
        if state_directory.is_empty() || project_root.is_empty() {
            return None;
        }
        Some((
            PathBuf::from(state_directory).join("probe"),
            PathBuf::from(project_root),
        ))
    };
    let Some((directory, project_root)) = environment_paths().or_else(settings_paths) else {
        return;
    };
    if !directory.is_absolute() || !project_root.is_absolute() {
        log::warn!("ZedLink probe requires absolute directory and project paths");
        return;
    }

    let setup = (|| -> Result<(UnixListener, ProbeState)> {
        let project_root = fs::canonicalize(project_root).context("project root unavailable")?;
        if !project_root.is_dir() {
            return Err(anyhow!("project root is not a directory"));
        }

        let mut builder = DirBuilder::new();
        builder.mode(0o700);
        match builder.create(&directory) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                let metadata = fs::symlink_metadata(&directory)?;
                if !metadata.file_type().is_dir() || metadata.permissions().mode() & 0o077 != 0 {
                    return Err(anyhow!("existing probe directory is not private"));
                }
            }
            Err(error) => return Err(error).context("private socket directory unavailable"),
        }
        let socket = directory.join("zedlink.sock");
        if let Ok(metadata) = fs::symlink_metadata(&socket) {
            if !metadata.file_type().is_socket() || UnixStream::connect(&socket).is_ok() {
                return Err(anyhow!("probe socket path is occupied"));
            }
            fs::remove_file(&socket).context("stale probe socket unavailable")?;
        }
        let listener = UnixListener::bind(&socket).context("socket unavailable")?;
        fs::set_permissions(&socket, Permissions::from_mode(0o600))
            .context("socket permissions unavailable")?;
        Ok((
            listener,
            ProbeState {
                project_root,
                permissions: Cell::new(permissions),
                instance_id: uuid::Uuid::new_v4().to_string(),
                epoch: uuid::Uuid::new_v4().to_string(),
                submissions: RefCell::new(HashMap::new()),
                creations: RefCell::new(HashMap::new()),
                exposed_threads: RefCell::new(HashSet::new()),
                subscriptions: RefCell::new(HashMap::new()),
                events: RefCell::new(HashMap::new()),
                diffs: RefCell::new(HashMap::new()),
                searches: RefCell::new(HashMap::new()),
                change_operations: RefCell::new(HashMap::new()),
                terminal_operations: RefCell::new(HashMap::new()),
                clipboard_operations: RefCell::new(HashMap::new()),
            },
        ))
    })();

    let Ok((listener, state)) = setup else {
        log::warn!("ZedLink local probe could not start");
        return;
    };
    let state = Rc::new(state);

    cx.observe_global::<SettingsStore>({
        let state = state.clone();
        move |cx| state.permissions.set(ProbePermissions::from_settings(cx))
    })
    .detach();

    let (sender, receiver) = async_channel::bounded::<ProbeCommand>(16);
    let listener_thread = std::thread::Builder::new()
        .name("zedlink-probe".into())
        .spawn(move || {
            for incoming in listener.incoming() {
                let Ok(stream) = incoming else {
                    log::warn!("ZedLink local probe listener stopped");
                    break;
                };
                if handle_stream(stream, &sender).is_err() {
                    log::debug!("ZedLink local probe request failed");
                }
            }
        });
    if listener_thread.is_err() {
        log::warn!("ZedLink local probe worker could not start");
        return;
    }
    cx.set_global(ProbeGlobal(state.clone()));

    if let Some(manager_path) = gateway_manager {
        if !manager_path.is_absolute() || !manager_path.is_file() {
            log::warn!("ZedLink gateway manager path is unavailable");
        } else {
            cx.background_spawn(async move {
                match smol::process::Command::new("python3")
                    .arg(manager_path)
                    .arg("start-gateway-detached")
                    .output()
                    .await
                {
                    Ok(output) if output.status.success() => {}
                    Ok(output) => log::warn!(
                        "ZedLink gateway could not be started automatically: {}",
                        String::from_utf8_lossy(&output.stderr).trim()
                    ),
                    Err(error) => {
                        log::warn!("ZedLink gateway could not be started automatically: {error}")
                    }
                }
            })
            .detach();
        }
    }

    cx.spawn(async move |cx| {
        while let Ok(command) = receiver.recv().await {
            let result = cx.update(|cx| handle_request(command.request, &state, cx));
            if command.response.send(result).is_err() {
                continue;
            }
        }
    })
    .detach();
}

pub(crate) fn toggle_thread_exposure(thread: &Entity<AcpThread>, cx: &mut App) -> Option<bool> {
    let state = cx.try_global::<ProbeGlobal>()?.0.clone();
    if !thread_belongs_to_project(thread, &state.project_root, cx) {
        return None;
    }
    let id = thread.read(cx).session_id().to_string();
    let was_exposed = state.exposed_threads.borrow_mut().remove(&id);
    if was_exposed {
        state.subscriptions.borrow_mut().remove(&id);
        Some(false)
    } else {
        expose_thread(&state, thread, cx);
        Some(true)
    }
}

fn expose_thread(state: &Rc<ProbeState>, thread: &Entity<AcpThread>, cx: &mut App) {
    let id = thread.read(cx).session_id().to_string();
    if state.exposed_threads.borrow().contains(&id) {
        return;
    }
    let subscription = cx.subscribe(thread, {
        let state = state.clone();
        let id = id.clone();
        move |_thread, event: &AcpThreadEvent, _cx| {
            let mut events = state.events.borrow_mut();
            let record = events.entry(id.clone()).or_default();
            let Some(sequence) = record.last_sequence.checked_add(1) else {
                return;
            };
            record.last_sequence = sequence;
            record.records.push_back(json!({
                "sequence": sequence,
                "kind": event_kind(event),
            }));
            if record.records.len() > 256 {
                record.records.pop_front();
            }
        }
    });
    state.events.borrow_mut().entry(id.clone()).or_default();
    state
        .subscriptions
        .borrow_mut()
        .insert(id.clone(), subscription);
    state.exposed_threads.borrow_mut().insert(id);
}

fn event_kind(event: &AcpThreadEvent) -> &'static str {
    match event {
        AcpThreadEvent::NewEntry => "entry.added",
        AcpThreadEvent::EntryUpdated(_) => "entry.updated",
        AcpThreadEvent::EntriesRemoved(_) => "entry.removed",
        AcpThreadEvent::StatusChanged => "status.changed",
        AcpThreadEvent::ToolAuthorizationRequested(_) => "approval.requested",
        AcpThreadEvent::ToolAuthorizationReceived(_) => "approval.resolved",
        AcpThreadEvent::Stopped { .. } => "turn.stopped",
        AcpThreadEvent::Error => "turn.error",
        _ => "metadata.changed",
    }
}

fn transcript_page(
    thread: &AcpThread,
    before_entry_index: usize,
    max_entries: usize,
    cx: &App,
) -> (usize, usize, Vec<String>) {
    let end = before_entry_index.min(thread.entries().len());
    let mut used = 0;
    let mut entries = Vec::new();
    for entry in thread.entries()[..end].iter().rev() {
        let markdown = entry.to_markdown(cx);
        if !entries.is_empty()
            && (entries.len() >= max_entries || used + markdown.len() > MAX_TRANSCRIPT_PAGE_BYTES)
        {
            break;
        }
        used += markdown.len();
        entries.push(markdown);
    }
    entries.reverse();
    let start = end.saturating_sub(entries.len());
    (start, end, entries)
}

pub(crate) fn is_available(cx: &App) -> bool {
    cx.has_global::<ProbeGlobal>()
}

fn handle_stream(
    mut stream: UnixStream,
    sender: &async_channel::Sender<ProbeCommand>,
) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;

    let mut bytes = Vec::new();
    let mut reader = BufReader::new((&mut stream).take(MAX_REQUEST_BYTES + 1));
    reader.read_until(b'\n', &mut bytes)?;
    let response = if bytes.len() as u64 > MAX_REQUEST_BYTES {
        error("", "PAYLOAD_TOO_LARGE")
    } else {
        match serde_json::from_slice::<ProbeRequest>(&bytes) {
            Ok(request)
                if request.version == 1
                    && !request.request_id.is_empty()
                    && request.request_id.len() <= 128
                    && request.params.is_object() =>
            {
                let request_id = request.request_id.clone();
                let (response_sender, response_receiver) = mpsc::channel();
                match sender.try_send(ProbeCommand {
                    request,
                    response: response_sender,
                }) {
                    Ok(()) => response_receiver
                        .recv_timeout(Duration::from_secs(5))
                        .unwrap_or_else(|_| error(&request_id, "INSTANCE_UNAVAILABLE")),
                    Err(async_channel::TrySendError::Full(_)) => error(&request_id, "RATE_LIMITED"),
                    Err(async_channel::TrySendError::Closed(_)) => {
                        error(&request_id, "INSTANCE_UNAVAILABLE")
                    }
                }
            }
            _ => error("", "INVALID_REQUEST"),
        }
    };
    serde_json::to_writer(&mut stream, &response)?;
    stream.write_all(b"\n")?;
    Ok(())
}

fn handle_request(request: ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let permissions = state.permissions.get();
    if !permissions.allows(&request.method, &request.params) {
        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
    }
    match request.method.as_str() {
        "instance.info" => success(
            &request.request_id,
            json!({
                "instance_id": state.instance_id,
                "epoch": state.epoch,
                "capabilities": permissions.capabilities(),
                "project": state.project_root.file_name().and_then(|name| name.to_str()),
                "provisional": true,
            }),
        ),
        "request.status" => {
            if request.params.get("instance_id").and_then(Value::as_str)
                != Some(state.instance_id.as_str())
                || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
            {
                return error(&request.request_id, "INSTANCE_UNAVAILABLE");
            }
            let Some(original_request_id) = request
                .params
                .get("original_request_id")
                .and_then(Value::as_str)
                .filter(|id| !id.is_empty())
            else {
                return error(&request.request_id, "INVALID_REQUEST");
            };
            let submissions = state.submissions.borrow();
            let Some(record) = submissions.get(original_request_id) else {
                return success(
                    &request.request_id,
                    json!({"known": false, "outcome": "unknown", "provisional": true}),
                );
            };
            if !state.exposed_threads.borrow().contains(&record.thread_id) {
                return error(&request.request_id, "THREAD_NOT_EXPOSED");
            }
            success(
                &request.request_id,
                json!({"known": true, "outcome": record.outcome, "provisional": true}),
            )
        }
        "thread.list" => {
            let mut seen = HashSet::new();
            let threads = loaded_thread_views(&state.project_root, cx)
                .into_iter()
                .filter_map(|(thread, view)| {
                    let thread = thread.read(cx);
                    let id = thread.session_id().to_string();
                    (state.exposed_threads.borrow().contains(&id) && seen.insert(id.clone())).then(
                        || {
                            let agent_error = zedlink_error_value(
                                thread.had_error(),
                                view.read(cx).zedlink_error_summary(),
                            );
                            let retry_status =
                                zedlink_retry_value(view.read(cx).zedlink_retry_status());
                            json!({
                                "thread_id": id,
                                "agent": thread_agent_value(&view, cx),
                                "title": thread.title().map(|title| title.to_string()),
                                "status": format!("{:?}", thread.status()),
                                "current_turn_id": thread.current_turn_id(),
                                "had_error": thread.had_error(),
                                "agent_error": agent_error,
                                "retry_status": retry_status,
                            })
                        },
                    )
                })
                .collect::<Vec<_>>();
            success(
                &request.request_id,
                json!({
                    "instance_id": state.instance_id,
                    "epoch": state.epoch,
                    "threads": threads,
                }),
            )
        }
        "thread.create" => create_thread(&request, state, cx),
        "thread.create.options" => create_thread_options(&request, state, cx),
        "thread.rename" => rename_thread(&request, state, cx),
        "thread.unexpose" => unexpose_thread(&request, state),
        "thread.snapshot" => {
            let Some(id) = request.params.get("thread_id").and_then(Value::as_str) else {
                return error(&request.request_id, "INVALID_REQUEST");
            };
            if !state.exposed_threads.borrow().contains(id) {
                return error(&request.request_id, "THREAD_NOT_EXPOSED");
            }
            let Some((thread, view)) = loaded_thread_views(&state.project_root, cx)
                .into_iter()
                .find(|(thread, _)| thread.read(cx).session_id().to_string() == id)
            else {
                return error(&request.request_id, "THREAD_NOT_EXPOSED");
            };
            let server_view = view.read(cx).server_view.upgrade();
            let thread = thread.read(cx);
            let agent_error =
                zedlink_error_value(thread.had_error(), view.read(cx).zedlink_error_summary());
            let retry_status = zedlink_retry_value(view.read(cx).zedlink_retry_status());
            let total_entries = thread.entries().len();
            let tail_entry_count = request
                .params
                .get("tail_entry_count")
                .and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .unwrap_or(MAX_TRANSCRIPT_PAGE_ENTRIES);
            if tail_entry_count == 0 || tail_entry_count > MAX_TRANSCRIPT_PAGE_ENTRIES {
                return error(&request.request_id, "INVALID_REQUEST");
            }
            let image_recovery =
                thread
                    .entries()
                    .iter()
                    .enumerate()
                    .find_map(|(entry_index, entry)| {
                        let message = entry.user_message()?;
                        message
                            .content
                            .source_blocks()
                            .iter()
                            .any(|chunk| matches!(chunk, acp::ContentBlock::Image(_)))
                            .then_some(message.client_id.as_ref())
                            .flatten()
                            .map(|client_id| {
                                json!({
                                    "client_user_message_id": client_id,
                                    "entry_index": entry_index,
                                    "entries_removed": total_entries.saturating_sub(entry_index),
                                })
                            })
                    });
            let rewind_points = thread
                .entries()
                .iter()
                .enumerate()
                .filter_map(|(entry_index, entry)| {
                    let message = entry.user_message()?;
                    message.client_id.as_ref().map(|client_id| {
                        json!({
                            "client_user_message_id": client_id,
                            "entry_index": entry_index,
                            "entries_removed": total_entries.saturating_sub(entry_index),
                        })
                    })
                })
                .collect::<Vec<_>>();
            let (entry_start_index, entry_end_index, entries) =
                transcript_page(thread, total_entries, tail_entry_count, cx);
            let truncated = entry_start_index > 0;
            // Retain the older activity projection for compatible clients. Updated clients
            // render the indexed transcript tail directly and request a smaller live tail.
            let (activity_entries, activity_truncated) =
                if request.params.get("tail_entry_count").is_some() {
                    (entries.clone(), truncated)
                } else {
                    let mut activity_used = 0;
                    let mut activity_entries = Vec::new();
                    let mut activity_truncated = false;
                    for entry in thread.entries().iter().rev() {
                        let markdown = entry.to_markdown(cx);
                        if activity_used + markdown.len() > MAX_ACTIVITY_BYTES {
                            activity_truncated = true;
                            if activity_entries.is_empty() {
                                let mut end = MAX_ACTIVITY_BYTES.min(markdown.len());
                                while !markdown.is_char_boundary(end) {
                                    end -= 1;
                                }
                                activity_entries.push(markdown[..end].to_string());
                            }
                            break;
                        }
                        activity_used += markdown.len();
                        activity_entries.push(markdown);
                    }
                    activity_entries.reverse();
                    (activity_entries, activity_truncated)
                };
            success(
                &request.request_id,
                json!({
                    "instance_id": state.instance_id,
                    "epoch": state.epoch,
                    "thread_id": id,
                    "agent": thread_agent_value(&view, cx),
                    "title": thread.title().map(|title| title.to_string()),
                    "status": format!("{:?}", thread.status()),
                    "current_turn_id": thread.current_turn_id(),
                    "had_error": thread.had_error(),
                    "agent_error": agent_error,
                    "image_recovery": image_recovery,
                    "rewind_points": rewind_points,
                    "retry_status": retry_status,
                    "pending_approvals": pending_approvals(thread, cx),
                    "subagents": subagent_snapshots(thread, server_view.as_ref(), cx),
                    "entries_markdown": entries,
                    "entry_start_index": entry_start_index,
                    "entry_end_index": entry_end_index,
                    "total_entries": total_entries,
                    "has_older_entries": entry_start_index > 0,
                    "truncated": truncated,
                    "activity_entries_markdown": activity_entries,
                    "activity_truncated": activity_truncated,
                    "provisional": true,
                    "last_sequence": state.events.borrow().get(id).map_or(0, |events| events.last_sequence),
                }),
            )
        }
        "thread.history" => {
            let Some(id) = request.params.get("thread_id").and_then(Value::as_str) else {
                return error(&request.request_id, "INVALID_REQUEST");
            };
            if !state.exposed_threads.borrow().contains(id) {
                return error(&request.request_id, "THREAD_NOT_EXPOSED");
            }
            let Some(thread) = loaded_threads(&state.project_root, cx)
                .into_iter()
                .find(|thread| thread.read(cx).session_id().to_string() == id)
            else {
                return error(&request.request_id, "THREAD_NOT_EXPOSED");
            };
            let thread = thread.read(cx);
            let total_entries = thread.entries().len();
            let before_entry_index = request
                .params
                .get("before_entry_index")
                .and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .unwrap_or(total_entries);
            if before_entry_index > total_entries {
                return error(&request.request_id, "RESYNC_REQUIRED");
            }
            let (entry_start_index, entry_end_index, entries) =
                transcript_page(thread, before_entry_index, MAX_TRANSCRIPT_PAGE_ENTRIES, cx);
            success(
                &request.request_id,
                json!({
                    "instance_id": state.instance_id,
                    "epoch": state.epoch,
                    "thread_id": id,
                    "entries_markdown": entries,
                    "entry_start_index": entry_start_index,
                    "entry_end_index": entry_end_index,
                    "total_entries": total_entries,
                    "has_older_entries": entry_start_index > 0,
                    "last_sequence": state.events.borrow().get(id).map_or(0, |events| events.last_sequence),
                    "provisional": true,
                }),
            )
        }
        "thread.events" => {
            let Some(id) = request.params.get("thread_id").and_then(Value::as_str) else {
                return error(&request.request_id, "INVALID_REQUEST");
            };
            let Some(after_sequence) = request.params.get("after_sequence").and_then(Value::as_u64)
            else {
                return error(&request.request_id, "INVALID_REQUEST");
            };
            if !state.exposed_threads.borrow().contains(id)
                || !loaded_threads(&state.project_root, cx)
                    .into_iter()
                    .any(|thread| thread.read(cx).session_id().to_string() == id)
            {
                return error(&request.request_id, "THREAD_NOT_EXPOSED");
            }
            let events = state.events.borrow();
            let Some(events) = events.get(id) else {
                return error(&request.request_id, "RESYNC_REQUIRED");
            };
            if after_sequence > events.last_sequence
                || events.records.front().is_some_and(|record| {
                    after_sequence.saturating_add(1) < record["sequence"].as_u64().unwrap_or(0)
                })
            {
                return error(&request.request_id, "RESYNC_REQUIRED");
            }
            let records = events
                .records
                .iter()
                .filter(|record| {
                    record["sequence"]
                        .as_u64()
                        .is_some_and(|seq| seq > after_sequence)
                })
                .cloned()
                .collect::<Vec<_>>();
            success(
                &request.request_id,
                json!({
                    "instance_id": state.instance_id,
                    "epoch": state.epoch,
                    "thread_id": id,
                    "last_sequence": events.last_sequence,
                    "events": records,
                    "provisional": true,
                }),
            )
        }
        "thread.commands" => command_list(&request, state, cx),
        "thread.settings" => thread_settings(&request, state, cx),
        "thread.settings.update" => update_thread_settings(&request, state, cx),
        "thread.elicitations" => elicitation_list(&request, state, cx),
        "elicitation.respond" => elicitation_response(&request, state, cx),
        "thread.queue" => queue_state(&request, state, cx),
        "thread.queue.clear" => clear_queue(&request, state, cx),
        "thread.edit_review" => edit_review_state(&request, state, cx),
        "thread.edit_review.respond" => edit_review_response(&request, state, cx),
        "terminal.list" => terminal_list(&request, state, cx),
        "terminal.snapshot" => terminal_snapshot(&request, state, cx),
        "terminal.input" => terminal_input(&request, state, cx),
        "clipboard.read" | "clipboard.write" => remote_clipboard(&request, state, cx),
        "task.list" => task_list(&request, state, cx),
        "task.stop" => task_stop(&request, state, cx),
        "task.catalog" => task_catalog(&request, state, cx),
        "task.run" => task_run(&request, state, cx),
        "debugger.snapshot" => debugger_snapshot(&request, state, cx),
        "debugger.catalog" => debugger_catalog(&request, state, cx),
        "debugger.launch" => debugger_launch(&request, state, cx),
        "debugger.control" => debugger_control(&request, state, cx),
        "project.files" | "project.directory" => project_files(&request, state, cx),
        "project.file" => project_file(&request, state, cx),
        "project.search" => project_search(&request, state, cx),
        "changes.snapshot" => changes_snapshot(&request, state, cx),
        "changes.diff" => changes_diff(&request, state, cx),
        "changes.stage" => change_stage(&request, state, true, cx),
        "changes.unstage" => change_stage(&request, state, false, cx),
        "thread.send" => send_request(&request, state, cx),
        "thread.cancel" => cancel_request(&request, state, cx),
        "subagent.cancel" => cancel_subagent_request(&request, state, cx),
        "thread.rewind_image" => rewind_image_request(&request, state, cx),
        "thread.rewind" => rewind_request(&request, state, cx),
        "thread.approvals" => approval_list(&request, state, cx),
        "approval.respond" => approval_response(&request, state, cx),
        _ => error(&request.request_id, "UNSUPPORTED_CAPABILITY"),
    }
}

fn create_thread(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let title = match request.params.get("title") {
        None | Some(Value::Null) => None,
        Some(Value::String(title)) => {
            let title = title.trim();
            if title.is_empty() {
                None
            } else if title.len() > MAX_THREAD_TITLE_BYTES {
                return error(&request.request_id, "INVALID_REQUEST");
            } else {
                Some(title.to_string())
            }
        }
        _ => return error(&request.request_id, "INVALID_REQUEST"),
    };
    let agent_id = match request.params.get("agent_id") {
        None | Some(Value::Null) => agent::ZED_AGENT_ID.0.to_string(),
        Some(Value::String(agent_id)) if !agent_id.is_empty() && agent_id.len() <= 128 => {
            agent_id.clone()
        }
        _ => return error(&request.request_id, "INVALID_REQUEST"),
    };

    {
        let creations = state.creations.borrow();
        if let Some(previous) = creations.get(&request.request_id) {
            if previous.title != title || previous.agent_id != agent_id {
                return error(&request.request_id, "REQUEST_CONFLICT");
            }
            return creation_result(&request.request_id, previous);
        }
        if creations.len() >= 64 {
            return error(&request.request_id, "RATE_LIMITED");
        }
    }

    let panel = workspace::AppState::global(cx)
        .workspace_store
        .read(cx)
        .workspaces()
        .filter_map(|workspace| workspace.upgrade())
        .find_map(|workspace| {
            let workspace = workspace.read(cx);
            let project = workspace.project().clone();
            let matches_project = project
                .read(cx)
                .visible_worktrees(cx)
                .any(|worktree| worktree.read(cx).abs_path().as_ref() == state.project_root);
            matches_project
                .then(|| {
                    workspace
                        .panel::<AgentPanel>(cx)
                        .map(|panel| (panel, project))
                })
                .flatten()
        });
    let Some((panel, project)) = panel else {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    };
    let agent = if agent_id == agent::ZED_AGENT_ID.as_ref() {
        Agent::NativeAgent
    } else {
        let store = project.read(cx).agent_server_store().clone();
        if !store
            .read(cx)
            .external_agents()
            .any(|known| known.0.as_ref() == agent_id)
        {
            return error(&request.request_id, "INVALID_REQUEST");
        }
        Agent::Custom {
            id: project::AgentId(agent_id.clone().into()),
        }
    };

    let created = cx.with_window(panel.entity_id(), |window, cx| {
        panel.update(cx, |panel, cx| {
            let thread_id = panel.create_thread_with_options(
                CreateThreadOptions {
                    title: title.clone().map(Into::into),
                    agent: Some(agent),
                    ..Default::default()
                },
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
            let conversation = panel
                .conversation_views()
                .into_iter()
                .find(|conversation| conversation.read(cx).thread_id == thread_id);
            (thread_id, conversation)
        })
    });
    let Some((_thread_id, Some(conversation))) = created else {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    };

    state.creations.borrow_mut().insert(
        request.request_id.clone(),
        CreationRecord {
            title,
            agent_id,
            thread_id: None,
        },
    );
    let probe_state = cx.global::<ProbeGlobal>().0.clone();
    if let Some(thread) = conversation.read(cx).root_thread(cx) {
        finish_thread_creation(&request.request_id, &probe_state, &thread, cx);
    } else {
        let request_id = request.request_id.clone();
        cx.observe(&conversation, move |conversation, cx| {
            let Some(thread) = conversation.read(cx).root_thread(cx) else {
                return;
            };
            finish_thread_creation(&request_id, &probe_state, &thread, cx);
        })
        .detach();
    }

    let creations = state.creations.borrow();
    creation_result(
        &request.request_id,
        creations
            .get(&request.request_id)
            .expect("creation record inserted"),
    )
}

fn create_thread_options(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let panel = workspace::AppState::global(cx)
        .workspace_store
        .read(cx)
        .workspaces()
        .filter_map(|workspace| workspace.upgrade())
        .find_map(|workspace| {
            let workspace = workspace.read(cx);
            let project = workspace.project().clone();
            let matches_project = project
                .read(cx)
                .visible_worktrees(cx)
                .any(|worktree| worktree.read(cx).abs_path().as_ref() == state.project_root);
            matches_project
                .then(|| {
                    workspace
                        .panel::<AgentPanel>(cx)
                        .map(|panel| (panel, project))
                })
                .flatten()
        });
    let Some((_panel, project)) = panel else {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    };
    let agent_store = project.read(cx).agent_server_store().clone();
    let store = agent_store.read(cx);
    let mut agents = vec![json!({
        "id": agent::ZED_AGENT_ID.0.to_string(),
        "name": "Zed Agent",
        "kind": "zed",
    })];
    for id in store.external_agents() {
        agents.push(json!({
            "id": id.0.to_string(),
            "name": store.agent_display_name(id).unwrap_or_else(|| id.0.clone()),
            "kind": "external_acp",
        }));
    }
    success(
        &request.request_id,
        json!({
            "instance_id": state.instance_id,
            "epoch": state.epoch,
            "project": state.project_root.file_name().and_then(|name| name.to_str()),
            "agents": agents,
        }),
    )
}

fn rename_thread(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(title) = request.params.get("title").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let title = title.trim();
    if title.is_empty() || title.len() > MAX_THREAD_TITLE_BYTES {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    if !state.exposed_threads.borrow().contains(thread_id) {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    }
    let Some(thread) = loaded_threads(&state.project_root, cx)
        .into_iter()
        .find(|thread| thread.read(cx).session_id().to_string() == thread_id)
    else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let renamed = thread.update(cx, |thread, cx| {
        if !thread.can_set_title(cx) {
            return false;
        }
        thread.set_title(title.into(), cx).detach();
        true
    });
    if !renamed {
        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
    }
    success(
        &request.request_id,
        json!({"thread_id": thread_id, "title": title, "outcome": "requested"}),
    )
}

fn unexpose_thread(request: &ProbeRequest, state: &ProbeState) -> Value {
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if !state.exposed_threads.borrow_mut().remove(thread_id) {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    }
    state.subscriptions.borrow_mut().remove(thread_id);
    state.events.borrow_mut().remove(thread_id);
    success(
        &request.request_id,
        json!({"thread_id": thread_id, "outcome": "unexposed"}),
    )
}

fn finish_thread_creation(
    request_id: &str,
    state: &Rc<ProbeState>,
    thread: &Entity<AcpThread>,
    cx: &mut App,
) {
    if !thread_belongs_to_project(thread, &state.project_root, cx) {
        return;
    }
    let thread_id = thread.read(cx).session_id().to_string();
    let creation = state.creations.borrow();
    let Some(record) = creation.get(request_id) else {
        return;
    };
    let should_expose = record.thread_id.is_none();
    let title = record.title.clone();
    drop(creation);
    if !should_expose {
        return;
    }
    if let Some(title) = title {
        thread
            .update(cx, |thread, cx| thread.set_title(title.into(), cx))
            .detach();
    }
    expose_thread(state, thread, cx);
    if let Some(record) = state.creations.borrow_mut().get_mut(request_id) {
        record.thread_id = Some(thread_id);
    }
}

fn creation_result(request_id: &str, record: &CreationRecord) -> Value {
    match &record.thread_id {
        Some(thread_id) => success(
            request_id,
            json!({"outcome": "ready", "thread_id": thread_id, "provisional": true}),
        ),
        None => success(
            request_id,
            json!({"outcome": "creating", "provisional": true}),
        ),
    }
}

fn pending_approvals(thread: &AcpThread, cx: &App) -> Vec<Value> {
    pending_approvals_for_session(thread, None, cx)
}

fn permission_option_kind(kind: acp::PermissionOptionKind) -> &'static str {
    match kind {
        acp::PermissionOptionKind::AllowOnce => "allow_once",
        acp::PermissionOptionKind::AllowAlways => "allow_always",
        acp::PermissionOptionKind::RejectOnce => "reject_once",
        acp::PermissionOptionKind::RejectAlways => "reject_always",
        _ => "unknown",
    }
}

fn permission_option_value(
    option: &acp::PermissionOption,
    choice_index: Option<usize>,
    action: Option<&'static str>,
) -> Value {
    json!({
        "option_id": option.option_id.0.as_ref(),
        "label": &option.name,
        "option_kind": permission_option_kind(option.kind.clone()),
        "choice_index": choice_index,
        "action": action,
    })
}

fn permission_options_value(options: &acp_thread::PermissionOptions) -> Value {
    match options {
        acp_thread::PermissionOptions::Flat(options) => json!({
            "presentation": "flat",
            "decisions": options
                .iter()
                .map(|option| permission_option_value(option, None, None))
                .collect::<Vec<_>>(),
            "patterns": [],
        }),
        acp_thread::PermissionOptions::Dropdown(choices) => json!({
            "presentation": "dropdown",
            "decisions": choices
                .iter()
                .enumerate()
                .flat_map(|(index, choice)| [
                    permission_option_value(&choice.allow, Some(index), Some("allow")),
                    permission_option_value(&choice.deny, Some(index), Some("deny")),
                ])
                .collect::<Vec<_>>(),
            "patterns": [],
        }),
        acp_thread::PermissionOptions::DropdownWithPatterns {
            choices, patterns, ..
        } => {
            let mut decisions = choices
                .iter()
                .enumerate()
                .flat_map(|(index, choice)| {
                    [
                        permission_option_value(&choice.allow, Some(index), Some("allow")),
                        permission_option_value(&choice.deny, Some(index), Some("deny")),
                    ]
                })
                .collect::<Vec<_>>();
            if let Some(always) = choices.first() {
                for (index, pattern) in patterns.iter().enumerate() {
                    for (option, action) in [(&always.allow, "allow"), (&always.deny, "deny")] {
                        let mut decision = permission_option_value(option, Some(0), Some(action));
                        decision["label"] = json!(format!("Always for `{}`", pattern.display_name));
                        decision["selected_pattern_indices"] = json!([index]);
                        decisions.push(decision);
                    }
                }
            }
            json!({
                "presentation": "dropdown_with_patterns",
                "decisions": decisions,
                "patterns": patterns
                .iter()
                .enumerate()
                .map(|(index, pattern)| json!({
                    "index": index,
                    "display_name": pattern.display_name,
                }))
                .collect::<Vec<_>>(),
            })
        }
    }
}

fn pending_approvals_for_session(
    thread: &AcpThread,
    subagent_id: Option<&str>,
    cx: &App,
) -> Vec<Value> {
    let turn_id = thread.current_turn_id();
    thread
        .entries()
        .iter()
        .filter_map(|entry| {
            let AgentThreadEntry::ToolCall(tool_call) = entry else {
                return None;
            };
            let authorization = thread.permission_request_for_tool(&tool_call.id)?;
            let options = authorization.legacy_options()?;
            let kind = authorization.legacy_kind()?;
            let raw_input = tool_call
                .raw_input
                .as_ref()
                .filter(|input| input.to_string().len() <= MAX_APPROVAL_INPUT_BYTES);
            let tool_name = tool_call.tool_name.as_ref().map(|name| name.to_string());
            let details_available = raw_input.is_some() && tool_name.is_some();
            let mut approval = json!({
                "approval_id": tool_call.id.0.as_ref(),
                "turn_id": turn_id,
                "tool_name": tool_name,
                "display": tool_call.label.read(cx).source().chars().take(256).collect::<String>(),
                "raw_input": raw_input,
                "details_truncated": !details_available,
                "authorization_kind": match kind {
                    AuthorizationKind::PermissionGrant => "permission_grant",
                    AuthorizationKind::ActionChoice => "action_choice",
                },
                "permission_options": permission_options_value(options),
                "can_approve_once": details_available && options.allow_once_option_id().is_some(),
                "can_deny_once": options.deny_once_option_id().is_some(),
            });
            if let Some(subagent_id) = subagent_id {
                approval["subagent_id"] = json!(subagent_id);
            }
            Some(approval)
        })
        .collect()
}

fn subagent_snapshots(
    root: &AcpThread,
    server_view: Option<&Entity<ConversationView>>,
    cx: &App,
) -> Vec<Value> {
    root.entries()
        .iter()
        .filter_map(|entry| {
            let AgentThreadEntry::ToolCall(tool_call) = entry else {
                return None;
            };
            let info = tool_call.subagent_session_info.as_ref()?;
            let session_id = info.session_id.to_string();
            let label = tool_call
                .label
                .read(cx)
                .source()
                .chars()
                .take(256)
                .collect::<String>();
            let mut snapshot = json!({
                "session_id": session_id,
                "tool_call_id": tool_call.id.0.as_ref(),
                "label": label,
                "status": tool_call.status().to_string(),
                "message_start_index": info.message_start_index,
                "message_end_index": info.message_end_index,
                "current_turn_id": null,
                "title": null,
                "entries_markdown": [],
                "entries_truncated": false,
                "pending_approvals": [],
            });
            let Some(child_view) =
                server_view.and_then(|view| view.read(cx).thread_view(&info.session_id))
            else {
                return Some(snapshot);
            };
            let child = child_view.read(cx).thread.clone();
            let child = child.read(cx);
            let mut used = 0;
            let mut entries = Vec::new();
            let mut truncated = false;
            for entry in child.entries().iter().rev() {
                let markdown = entry.to_markdown(cx);
                if used + markdown.len() > MAX_SUBAGENT_SNAPSHOT_BYTES {
                    truncated = true;
                    break;
                }
                used += markdown.len();
                entries.push(markdown);
            }
            entries.reverse();
            let current_turn_id = child.current_turn_id();
            if current_turn_id.is_some() {
                snapshot["status"] = json!(format!("{:?}", child.status()));
            }
            snapshot["current_turn_id"] = json!(current_turn_id);
            snapshot["title"] = json!(child.title().map(|title| title.to_string()));
            snapshot["entries_markdown"] = json!(entries);
            snapshot["entries_truncated"] = json!(truncated);
            snapshot["pending_approvals"] =
                json!(pending_approvals_for_session(&child, Some(&session_id), cx));
            Some(snapshot)
        })
        .take(MAX_SUBAGENTS_PER_SNAPSHOT)
        .collect()
}

fn approval_list(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if !state.exposed_threads.borrow().contains(thread_id) {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    }
    let Some((thread, view)) = loaded_thread_views(&state.project_root, cx)
        .into_iter()
        .find(|(thread, _)| thread.read(cx).session_id().to_string() == thread_id)
    else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let server_view = view.read(cx).server_view.upgrade();
    let thread = thread.read(cx);
    let mut approvals = pending_approvals(thread, cx);
    for subagent in subagent_snapshots(thread, server_view.as_ref(), cx) {
        if let Some(child_approvals) = subagent["pending_approvals"].as_array() {
            approvals.extend(child_approvals.iter().cloned());
        }
    }
    success(
        &request.request_id,
        json!({"thread_id": thread_id, "approvals": approvals, "provisional": true}),
    )
}

fn approval_response(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(approval_id) = request.params.get("approval_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(turn_id) = request
        .params
        .get("turn_id")
        .and_then(Value::as_u64)
        .and_then(|id| u32::try_from(id).ok())
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    if !state.exposed_threads.borrow().contains(thread_id) {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    }
    let thread =
        if let Some(subagent_id) = request.params.get("subagent_id").and_then(Value::as_str) {
            match exposed_subagent_thread_view(thread_id, subagent_id, state, cx) {
                Ok((thread, _)) => thread,
                Err(code) => return error(&request.request_id, code),
            }
        } else {
            let Some(thread) = loaded_threads(&state.project_root, cx)
                .into_iter()
                .find(|thread| thread.read(cx).session_id().to_string() == thread_id)
            else {
                return error(&request.request_id, "THREAD_NOT_EXPOSED");
            };
            thread
        };
    if thread.read(cx).current_turn_id() != Some(turn_id) {
        return error(&request.request_id, "STALE_TURN");
    }
    let tool_call_id = acp::ToolCallId::new(approval_id);
    let outcome = {
        let thread = thread.read(cx);
        let Some((_, tool_call)) = thread.tool_call(&tool_call_id) else {
            return error(&request.request_id, "APPROVAL_EXPIRED");
        };
        let Some(authorization) = thread.permission_request_for_tool(&tool_call_id) else {
            return error(&request.request_id, "APPROVAL_ALREADY_RESOLVED");
        };
        let (Some(options), Some(authorization_kind)) =
            (authorization.legacy_options(), authorization.legacy_kind())
        else {
            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
        };
        let details_available = tool_call.tool_name.is_some()
            && tool_call
                .raw_input
                .as_ref()
                .is_some_and(|input| input.to_string().len() <= MAX_APPROVAL_INPUT_BYTES);
        let outcome = if let Some(decision) = request.params.get("decision").and_then(Value::as_str)
        {
            if !matches!(authorization_kind, AuthorizationKind::PermissionGrant) {
                return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
            }
            let kind = match decision {
                "approve_once" => acp::PermissionOptionKind::AllowOnce,
                "deny_once" => acp::PermissionOptionKind::RejectOnce,
                _ => return error(&request.request_id, "INVALID_REQUEST"),
            };
            let Some(option) = options.first_option_of_kind(kind) else {
                return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
            };
            SelectedPermissionOutcome::new(option.option_id.clone(), option.kind.clone())
        } else {
            let Some(option_id) = request.params.get("option_id").and_then(Value::as_str) else {
                return error(&request.request_id, "INVALID_REQUEST");
            };
            match options {
                acp_thread::PermissionOptions::Flat(options) => {
                    let Some(option) = options
                        .iter()
                        .find(|option| option.option_id.0.as_ref() == option_id)
                    else {
                        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
                    };
                    SelectedPermissionOutcome::new(option.option_id.clone(), option.kind.clone())
                }
                acp_thread::PermissionOptions::Dropdown(choices)
                | acp_thread::PermissionOptions::DropdownWithPatterns { choices, .. } => {
                    let Some(choice_index) = request
                        .params
                        .get("choice_index")
                        .and_then(Value::as_u64)
                        .and_then(|index| usize::try_from(index).ok())
                    else {
                        return error(&request.request_id, "INVALID_REQUEST");
                    };
                    let Some(choice) = choices.get(choice_index) else {
                        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
                    };
                    let is_allow = if choice.allow.option_id.0.as_ref() == option_id {
                        true
                    } else if choice.deny.option_id.0.as_ref() == option_id {
                        false
                    } else {
                        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
                    };
                    let selected_patterns = request
                        .params
                        .get("selected_pattern_indices")
                        .and_then(Value::as_array)
                        .map(|values| {
                            values
                                .iter()
                                .map(|value| {
                                    value
                                        .as_u64()
                                        .and_then(|index| usize::try_from(index).ok())
                                        .ok_or(())
                                })
                                .collect::<std::result::Result<Vec<_>, _>>()
                        })
                        .transpose();
                    let selected_patterns = match selected_patterns {
                        Ok(patterns) => patterns,
                        Err(()) => return error(&request.request_id, "INVALID_REQUEST"),
                    };
                    if let Some(indices) = selected_patterns.filter(|indices| !indices.is_empty()) {
                        if choice_index != 0 {
                            return error(&request.request_id, "INVALID_REQUEST");
                        }
                        let acp_thread::PermissionOptions::DropdownWithPatterns {
                            patterns, ..
                        } = options
                        else {
                            return error(&request.request_id, "INVALID_REQUEST");
                        };
                        if indices.iter().any(|index| *index >= patterns.len()) {
                            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
                        }
                        let Some(outcome) =
                            options.build_outcome_for_checked_patterns(&indices, is_allow)
                        else {
                            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
                        };
                        outcome
                    } else {
                        choice.build_outcome(is_allow)
                    }
                }
            }
        };
        let is_allow = matches!(
            outcome.option_kind,
            acp::PermissionOptionKind::AllowOnce | acp::PermissionOptionKind::AllowAlways
        );
        if (matches!(authorization_kind, AuthorizationKind::PermissionGrant) && is_allow
            || matches!(authorization_kind, AuthorizationKind::ActionChoice))
            && !details_available
        {
            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
        }
        outcome
    };
    thread.update(cx, |thread, cx| {
        thread.authorize_tool_call(tool_call_id, outcome, cx)
    });
    success(
        &request.request_id,
        json!({"outcome": "resolved", "provisional": true}),
    )
}

fn cancel_request(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(turn_id) = request
        .params
        .get("turn_id")
        .and_then(Value::as_u64)
        .and_then(|id| u32::try_from(id).ok())
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    if !state.exposed_threads.borrow().contains(thread_id) {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    }
    let Some((thread, view)) = loaded_thread_views(&state.project_root, cx)
        .into_iter()
        .find(|(thread, _)| thread.read(cx).session_id().to_string() == thread_id)
    else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    if thread.read(cx).current_turn_id() != Some(turn_id) {
        return error(&request.request_id, "STALE_TURN");
    }
    view.update(cx, |view, cx| view.cancel_generation(cx));
    success(
        &request.request_id,
        json!({"outcome": "requested", "turn_id": turn_id, "provisional": true}),
    )
}

fn exposed_subagent_thread_view(
    parent_thread_id: &str,
    subagent_id: &str,
    state: &ProbeState,
    cx: &App,
) -> Result<(Entity<AcpThread>, Entity<ThreadView>), &'static str> {
    if !state.exposed_threads.borrow().contains(parent_thread_id) {
        return Err("THREAD_NOT_EXPOSED");
    }
    let Some((parent, parent_view)) = loaded_thread_views(&state.project_root, cx)
        .into_iter()
        .find(|(thread, _)| thread.read(cx).session_id().to_string() == parent_thread_id)
    else {
        return Err("THREAD_NOT_EXPOSED");
    };
    let session_id = acp::SessionId::new(subagent_id);
    if parent
        .read(cx)
        .tool_call_for_subagent(&session_id)
        .is_none()
    {
        return Err("SUBAGENT_NOT_FOUND");
    }
    let Some(server_view) = parent_view.read(cx).server_view.upgrade() else {
        return Err("SUBAGENT_NOT_FOUND");
    };
    let Some(view) = server_view.read(cx).thread_view(&session_id) else {
        return Err("SUBAGENT_NOT_FOUND");
    };
    let thread = view.read(cx).thread.clone();
    Ok((thread, view))
}

fn cancel_subagent_request(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(parent_thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(subagent_id) = request.params.get("subagent_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(turn_id) = request
        .params
        .get("turn_id")
        .and_then(Value::as_u64)
        .and_then(|id| u32::try_from(id).ok())
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let (thread, view) =
        match exposed_subagent_thread_view(parent_thread_id, subagent_id, state, cx) {
            Ok(value) => value,
            Err(code) => return error(&request.request_id, code),
        };
    if thread.read(cx).current_turn_id() != Some(turn_id) {
        return error(&request.request_id, "STALE_TURN");
    }
    view.update(cx, |view, cx| view.cancel_generation(cx));
    success(
        &request.request_id,
        json!({"outcome": "requested", "subagent_id": subagent_id, "turn_id": turn_id, "provisional": true}),
    )
}

fn rewind_image_request(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let thread = match exposed_thread_for_request(request, state, cx) {
        Ok(thread) => thread,
        Err(code) => return error(&request.request_id, code),
    };
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(operation) = state.change_operations.borrow().get(&request.request_id) {
        if operation.payload_digest != digest {
            return error(&request.request_id, "REQUEST_CONFLICT");
        }
        return change_operation_result(&request.request_id, &operation.state);
    }
    if state.change_operations.borrow().len() >= 128 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    if thread.read(cx).status() != ThreadStatus::Idle {
        return error(&request.request_id, "THREAD_BUSY");
    }
    let Some(expected_sequence) = request
        .params
        .get("expected_last_sequence")
        .and_then(Value::as_u64)
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if state
        .events
        .borrow()
        .get(thread_id)
        .map_or(0, |events| events.last_sequence)
        != expected_sequence
    {
        return error(&request.request_id, "STALE_SEQUENCE");
    }
    let Some(client_id_value) = request.params.get("client_user_message_id") else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Ok(client_id) =
        serde_json::from_value::<acp_thread::ClientUserMessageId>(client_id_value.clone())
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let first_image_client_id = thread.read(cx).entries().iter().find_map(|entry| {
        let message = entry.user_message()?;
        message
            .content
            .source_blocks()
            .iter()
            .any(|chunk| matches!(chunk, acp::ContentBlock::Image(_)))
            .then_some(message.client_id.clone())
            .flatten()
    });
    if first_image_client_id.as_ref() != Some(&client_id) {
        return error(&request.request_id, "STALE_SEQUENCE");
    }

    let task = thread.update(cx, |thread, cx| thread.rewind(client_id, cx));
    state.change_operations.borrow_mut().insert(
        request.request_id.clone(),
        ChangeOperationRecord {
            payload_digest: digest,
            state: ChangeOperationState::Running,
        },
    );
    let probe_state = cx.global::<ProbeGlobal>().0.clone();
    let request_id = request.request_id.clone();
    cx.spawn(async move |_cx| {
        let next_state = if task.await.is_ok() {
            ChangeOperationState::Succeeded
        } else {
            ChangeOperationState::Failed
        };
        if let Some(operation) = probe_state
            .change_operations
            .borrow_mut()
            .get_mut(&request_id)
        {
            operation.state = next_state;
        }
    })
    .detach();
    success(
        &request.request_id,
        json!({"outcome": "running", "provisional": true}),
    )
}

fn rewind_request(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let thread = match exposed_thread_for_request(request, state, cx) {
        Ok(thread) => thread,
        Err(code) => return error(&request.request_id, code),
    };
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(operation) = state.change_operations.borrow().get(&request.request_id) {
        if operation.payload_digest != digest {
            return error(&request.request_id, "REQUEST_CONFLICT");
        }
        return change_operation_result(&request.request_id, &operation.state);
    }
    if state.change_operations.borrow().len() >= 128 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    if thread.read(cx).status() != ThreadStatus::Idle {
        return error(&request.request_id, "THREAD_BUSY");
    }
    let Some(expected_sequence) = request
        .params
        .get("expected_last_sequence")
        .and_then(Value::as_u64)
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if state
        .events
        .borrow()
        .get(thread_id)
        .map_or(0, |events| events.last_sequence)
        != expected_sequence
    {
        return error(&request.request_id, "STALE_SEQUENCE");
    }
    let Some(client_id_value) = request.params.get("client_user_message_id") else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Ok(client_id) =
        serde_json::from_value::<acp_thread::ClientUserMessageId>(client_id_value.clone())
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if !thread.read(cx).entries().iter().any(|entry| {
        entry
            .user_message()
            .and_then(|message| message.client_id.as_ref())
            == Some(&client_id)
    }) {
        return error(&request.request_id, "STALE_SEQUENCE");
    }
    let task = thread.update(cx, |thread, cx| thread.rewind(client_id, cx));
    state.change_operations.borrow_mut().insert(
        request.request_id.clone(),
        ChangeOperationRecord {
            payload_digest: digest,
            state: ChangeOperationState::Running,
        },
    );
    let probe_state = cx.global::<ProbeGlobal>().0.clone();
    let request_id = request.request_id.clone();
    cx.spawn(async move |_cx| {
        let next_state = if task.await.is_ok() {
            ChangeOperationState::Succeeded
        } else {
            ChangeOperationState::Failed
        };
        if let Some(operation) = probe_state
            .change_operations
            .borrow_mut()
            .get_mut(&request_id)
        {
            operation.state = next_state;
        }
    })
    .detach();
    success(
        &request.request_id,
        json!({"outcome": "running", "provisional": true}),
    )
}

fn command_list(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    if !state.exposed_threads.borrow().contains(thread_id) {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    }
    let Some((thread, view)) = loaded_thread_views(&state.project_root, cx)
        .into_iter()
        .find(|(thread, _)| thread.read(cx).session_id().to_string() == thread_id)
    else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let connection = view.read(cx).as_native_connection(cx);
    if let Some(connection) = connection {
        connection.ensure_skills_scan_started(cx);
        connection.refresh_skills_for_project(thread.read(cx).project().clone(), cx);
    }
    let mut commands = remote_command_catalog(&thread, &view, cx);
    if !state
        .permissions
        .get()
        .allows("thread.unexpose", &request.params)
    {
        commands.retain(|command| {
            command.get("name").and_then(Value::as_str) != Some("remote-control")
        });
    }
    success(
        &request.request_id,
        json!({"thread_id": thread_id, "commands": commands}),
    )
}

fn remote_command_catalog(
    thread: &Entity<AcpThread>,
    view: &Entity<ThreadView>,
    cx: &App,
) -> Vec<Value> {
    let mut commands = thread
        .read(cx)
        .available_commands()
        .iter()
        .map(|command| {
            let input_hint = command.input.as_ref().and_then(|input| match input {
                acp::AvailableCommandInput::Text(input) => Some(input.hint.clone()),
                _ => None,
            });
            json!({
                "name": command.name, "description": command.description,
                "category": match acp_thread::command_category_from_meta(&command.meta) {
                    Some(acp_thread::CommandCategory::Native) => "native",
                    Some(acp_thread::CommandCategory::Mcp) => "mcp",
                    None => "agent",
                },
                "requires_argument": command.input.is_some(), "input_hint": input_hint,
                "execution": "prompt",
            })
        })
        .collect::<Vec<_>>();
    let view = view.read(cx);
    let capabilities = view.session_capabilities.read();
    commands.extend(capabilities.available_skills().iter().map(|skill| json!({
        "name": format!("{}:{}", skill.source, skill.name), "bare_name": skill.name,
        "description": skill.description, "category": "skill", "execution": "prompt",
        "requires_argument": false, "input_hint": "Optional instructions", "warning": skill.warning,
    })));
    commands.extend(view.remote_local_commands(cx).into_iter().map(|command| json!({
        "name": command.keyword(), "description": command.description(), "category": "local",
        "execution": "local", "requires_argument": false,
        "confirmation": if command.keyword() == "remote-control" { "Stops exposing this conversation to remote devices. The desktop agent keeps running." } else { "This feedback shares the current conversation with the Zed team. Negative feedback may require completing a form in desktop Zed." },
    })));
    let mut seen = HashSet::new();
    commands.reverse();
    commands.retain(|command| {
        command
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| seen.insert(name.to_string()))
    });
    commands.reverse();
    commands
}

fn thread_settings(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let Some((thread, view)) = exposed_thread_view(thread_id, state, cx) else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let native_thread = view.read(cx).as_native_thread(cx);
    success(
        &request.request_id,
        thread_settings_value(thread_id, &thread, &view, native_thread.as_ref(), cx),
    )
}

fn update_thread_settings(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let Some(parameters) = request.params.as_object() else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if parameters.keys().any(|key| {
        !matches!(
            key.as_str(),
            "instance_id"
                | "epoch"
                | "thread_id"
                | "profile_id"
                | "thinking_enabled"
                | "model_id"
                | "provider_id"
        )
    }) {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let profile_id = match parameters.get("profile_id") {
        None => None,
        Some(Value::String(profile_id)) if !profile_id.is_empty() && profile_id.len() <= 128 => {
            Some(AgentProfileId(profile_id.as_str().into()))
        }
        _ => return error(&request.request_id, "INVALID_REQUEST"),
    };
    let thinking_enabled = match parameters.get("thinking_enabled") {
        None => None,
        Some(Value::Bool(enabled)) => Some(*enabled),
        _ => return error(&request.request_id, "INVALID_REQUEST"),
    };
    let requested_model = match (parameters.get("provider_id"), parameters.get("model_id")) {
        (None, None) => None,
        (Some(Value::String(provider_id)), Some(Value::String(model_id)))
            if !provider_id.is_empty()
                && provider_id.len() <= 128
                && !model_id.is_empty()
                && model_id.len() <= 256 =>
        {
            Some((provider_id.clone(), model_id.clone()))
        }
        _ => return error(&request.request_id, "INVALID_REQUEST"),
    };
    if profile_id.is_none() && thinking_enabled.is_none() && requested_model.is_none() {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let Some((thread, view)) = exposed_thread_view(thread_id, state, cx) else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let Some(native_thread) = view.read(cx).as_native_thread(cx) else {
        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
    };
    if thread.read(cx).status() != ThreadStatus::Idle {
        return error(&request.request_id, "THREAD_BUSY");
    }
    if let Some(profile_id) = profile_id.as_ref()
        && !AgentProfile::available_profiles(cx).contains_key(profile_id)
    {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    if let Some(enabled) = thinking_enabled {
        let native = native_thread.read(cx);
        let Some(model) = native.model() else {
            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
        };
        if !model.supports_thinking() || (!enabled && !model.supports_disabling_thinking()) {
            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
        }
    }
    let selected_model = requested_model.and_then(|(provider_id, model_id)| {
        LanguageModelRegistry::read_global(cx)
            .available_models(cx)
            .find(|model| {
                model.provider_id().0.as_ref() == provider_id && model.id().0.as_ref() == model_id
            })
    });
    if parameters.get("model_id").is_some() && selected_model.is_none() {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    native_thread.update(cx, |thread, cx| {
        if let Some(profile_id) = profile_id {
            thread.set_profile(profile_id, cx);
        }
        if let Some(enabled) = thinking_enabled {
            thread.set_thinking_enabled(enabled, cx);
        }
        if let Some(model) = selected_model {
            thread.set_model(model, cx);
        }
    });
    success(
        &request.request_id,
        thread_settings_value(thread_id, &thread, &view, Some(&native_thread), cx),
    )
}

fn exposed_thread_view(
    thread_id: &str,
    state: &ProbeState,
    cx: &App,
) -> Option<(Entity<AcpThread>, Entity<ThreadView>)> {
    if !state.exposed_threads.borrow().contains(thread_id) {
        return None;
    }
    let (thread, view) = loaded_thread_views(&state.project_root, cx)
        .into_iter()
        .find(|(thread, _)| thread.read(cx).session_id().to_string() == thread_id)?;
    Some((thread, view))
}

fn thread_settings_value(
    thread_id: &str,
    thread: &Entity<AcpThread>,
    view: &Entity<ThreadView>,
    native_thread: Option<&Entity<agent::Thread>>,
    cx: &App,
) -> Value {
    let agent = thread_agent_value(view, cx);
    let Some(native_thread) = native_thread else {
        return json!({
            "thread_id": thread_id,
            "status": format!("{:?}", thread.read(cx).status()),
            "agent": agent,
            "model": Value::Null,
            "profile_id": Value::Null,
            "profiles": [],
            "thinking_enabled": Value::Null,
            "supports_images": Value::Null,
            "thinking_supported": false,
            "thinking_can_disable": false,
            "editable": false,
            "configuration_owner": "external_agent",
            "provisional": true,
        });
    };
    let native = native_thread.read(cx);
    let model = native.model();
    let token_usage = native.latest_token_usage();
    let models = LanguageModelRegistry::read_global(cx)
        .available_models(cx)
        .map(|model| {
            json!({
                "id": model.id().0.to_string(),
                "name": model.name().0.to_string(),
                "provider_id": model.provider_id().0.to_string(),
                "supports_images": model.supports_images(),
                "supports_thinking": model.supports_thinking(),
            })
        })
        .collect::<Vec<_>>();
    let profiles = AgentProfile::available_profiles(cx)
        .iter()
        .map(|(id, name)| json!({"id": id.as_str(), "name": name.to_string()}))
        .collect::<Vec<_>>();
    json!({
        "thread_id": thread_id,
        "status": format!("{:?}", thread.read(cx).status()),
        "agent": agent,
        "model": model.map(|model| json!({
            "id": model.id().0.to_string(),
            "name": model.name().0.to_string(),
            "provider_id": model.provider_id().0.to_string(),
        })),
        "models": models,
        "context_usage": token_usage.map(|usage| json!({
            "used_tokens": usage.used_tokens,
            "max_tokens": usage.max_tokens,
            "input_tokens": usage.input_tokens,
            "output_tokens": usage.output_tokens,
            "remaining_tokens": usage.max_tokens.saturating_sub(usage.used_tokens),
        })),
        "profile_id": native.profile().as_str(),
        "profiles": profiles,
        "thinking_enabled": native.thinking_enabled(),
        "supports_images": model.is_some_and(|model| model.supports_images()),
        "thinking_supported": model.is_some_and(|model| model.supports_thinking()),
        "thinking_can_disable": model.is_some_and(|model| model.supports_disabling_thinking()),
        "editable": true,
        "configuration_owner": "zed",
        "provisional": true,
    })
}

fn thread_agent_value(view: &Entity<ThreadView>, cx: &App) -> Value {
    let view = view.read(cx);
    json!({
        "id": view.agent_id.to_string(),
        "name": view.agent_display_name.to_string(),
        "kind": if view.as_native_thread(cx).is_some() { "zed" } else { "external_acp" },
    })
}

fn elicitation_list(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Ok(thread) = exposed_thread_for_request(request, state, cx) else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let thread = thread.read(cx);
    let elicitations = thread
        .entries()
        .iter()
        .filter_map(|entry| {
            let AgentThreadEntry::Elicitation(id) = entry else {
                return None;
            };
            let (_, elicitation) = thread.elicitation(id)?;
            let status = match elicitation.status {
                acp_thread::ElicitationStatus::Pending { .. } => "pending",
                acp_thread::ElicitationStatus::Accepted => "accepted",
                acp_thread::ElicitationStatus::Declined => "declined",
                acp_thread::ElicitationStatus::Canceled => "canceled",
                acp_thread::ElicitationStatus::Completed => "completed",
            };
            Some(json!({
                "elicitation_id": id.0.as_ref(),
                "request": &elicitation.request,
                "status": status,
            }))
        })
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({"thread_id": thread.session_id().to_string(), "elicitations": elicitations}),
    )
}

fn elicitation_response(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Ok(thread) = exposed_thread_for_request(request, state, cx) else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let Some(elicitation_id) = request
        .params
        .get("elicitation_id")
        .and_then(Value::as_str)
        .filter(|id| !id.is_empty() && id.len() <= 256)
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let action = request.params.get("action").and_then(Value::as_str);
    let response_value = match action {
        Some("accept") => json!({
            "action": "accept",
            "content": request.params.get("content").cloned().unwrap_or_else(|| json!({})),
        }),
        Some("decline") => json!({"action": "decline"}),
        Some("cancel") => json!({"action": "cancel"}),
        _ => return error(&request.request_id, "INVALID_REQUEST"),
    };
    let Ok(response) = serde_json::from_value::<acp::CreateElicitationResponse>(response_value)
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let id = acp_thread::ElicitationEntryId(elicitation_id.into());
    let is_pending = thread
        .read(cx)
        .elicitation(&id)
        .is_some_and(|(_, elicitation)| {
            matches!(
                elicitation.status,
                acp_thread::ElicitationStatus::Pending { .. }
            )
        });
    if !is_pending {
        return error(&request.request_id, "ELICITATION_EXPIRED");
    }
    thread.update(cx, |thread, cx| {
        thread.respond_to_elicitation(&id, response, cx)
    });
    success(
        &request.request_id,
        json!({"thread_id": thread.read(cx).session_id().to_string(), "outcome": "resolved"}),
    )
}

fn queue_state(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some((thread, view)) = request
        .params
        .get("thread_id")
        .and_then(Value::as_str)
        .and_then(|id| exposed_thread_view(id, state, cx))
    else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let view = view.read(cx);
    let entries = view
        .message_queue
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let preview = entry
                .content
                .iter()
                .filter_map(|block| match block {
                    acp::ContentBlock::Text(text) => Some(text.text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            json!({
                "index": index,
                "preview": preview.chars().take(500).collect::<String>(),
                "steer": entry.steer,
            })
        })
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({
            "thread_id": thread.read(cx).session_id().to_string(),
            "entries": entries,
            "count": entries.len(),
        }),
    )
}

fn clear_queue(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some((_, view)) = exposed_thread_view(thread_id, state, cx) else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let count = view.read(cx).message_queue.len();
    view.update(cx, |view, cx| {
        view.message_queue.clear();
        view.sync_queue_flag_to_native_thread(cx);
        cx.notify();
    });
    success(
        &request.request_id,
        json!({"thread_id": thread_id, "outcome": "cleared", "removed": count}),
    )
}

fn edit_review_state(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some((_, view)) = exposed_thread_view(thread_id, state, cx) else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let Some(native_thread) = view.read(cx).as_native_thread(cx) else {
        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
    };
    let action_log = native_thread.read(cx).action_log().clone();
    let action_log = action_log.read(cx);
    let changed_files = action_log.changed_buffers(cx).count();
    let stats = action_log.diff_stats(cx);
    success(
        &request.request_id,
        json!({
            "thread_id": thread_id,
            "pending": changed_files > 0,
            "changed_files": changed_files,
            "lines_added": stats.lines_added,
            "lines_removed": stats.lines_removed,
        }),
    )
}

fn edit_review_response(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(action) = request.params.get("action").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if !matches!(action, "keep_all" | "reject_all") {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(operation) = state.change_operations.borrow().get(&request.request_id) {
        if operation.payload_digest != digest {
            return error(&request.request_id, "REQUEST_CONFLICT");
        }
        return change_operation_result(&request.request_id, &operation.state);
    }
    if state.change_operations.borrow().len() >= 128 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    let Some(expected_sequence) = request
        .params
        .get("expected_last_sequence")
        .and_then(Value::as_u64)
    else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if state
        .events
        .borrow()
        .get(thread_id)
        .map_or(0, |events| events.last_sequence)
        != expected_sequence
    {
        return error(&request.request_id, "STALE_SEQUENCE");
    }
    let Some((_, view)) = exposed_thread_view(thread_id, state, cx) else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let Some(native_thread) = view.read(cx).as_native_thread(cx) else {
        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
    };
    let action_log = native_thread.read(cx).action_log().clone();
    state.change_operations.borrow_mut().insert(
        request.request_id.clone(),
        ChangeOperationRecord {
            payload_digest: digest,
            state: ChangeOperationState::Running,
        },
    );
    if action == "keep_all" {
        action_log.update(cx, |log, cx| log.keep_all_edits(None, cx));
        if let Some(operation) = state
            .change_operations
            .borrow_mut()
            .get_mut(&request.request_id)
        {
            operation.state = ChangeOperationState::Succeeded;
        }
    } else {
        let task = action_log.update(cx, |log, cx| log.reject_all_edits(None, cx));
        let probe_state = cx.global::<ProbeGlobal>().0.clone();
        let request_id = request.request_id.clone();
        cx.spawn(async move |_cx| {
            task.await;
            if let Some(operation) = probe_state
                .change_operations
                .borrow_mut()
                .get_mut(&request_id)
            {
                operation.state = ChangeOperationState::Succeeded;
            }
        })
        .detach();
    }
    change_operation_result(
        &request.request_id,
        &state.change_operations.borrow()[&request.request_id].state,
    )
}

fn exposed_thread_for_request(
    request: &ProbeRequest,
    state: &ProbeState,
    cx: &App,
) -> std::result::Result<Entity<AcpThread>, &'static str> {
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return Err("INSTANCE_UNAVAILABLE");
    }
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return Err("INVALID_REQUEST");
    };
    if !state.exposed_threads.borrow().contains(thread_id) {
        return Err("THREAD_NOT_EXPOSED");
    }
    loaded_threads(&state.project_root, cx)
        .into_iter()
        .find(|thread| thread.read(cx).session_id().to_string() == thread_id)
        .ok_or("THREAD_NOT_EXPOSED")
}

fn exposed_worktree(
    request: &ProbeRequest,
    state: &ProbeState,
    cx: &App,
) -> std::result::Result<Entity<worktree::Worktree>, &'static str> {
    let thread = exposed_thread_for_request(request, state, cx)?;
    let project = thread.read(cx).project().clone();
    project
        .read(cx)
        .visible_worktrees(cx)
        .find(|worktree| worktree.read(cx).abs_path().as_ref() == state.project_root)
        .ok_or("INSTANCE_UNAVAILABLE")
}

fn excluded_remote_path(path: &str) -> bool {
    let path = Path::new(path);
    if path.components().any(|component| {
        component.as_os_str().to_str().is_none_or(|part| {
            part.starts_with('.') || matches!(part, "target" | "node_modules" | "__pycache__")
        })
    }) {
        return true;
    }
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    matches!(
        file_name.as_str(),
        "credentials" | "credentials.json" | "secrets.json" | "id_rsa" | "id_ed25519"
    ) || matches!(
        extension.as_str(),
        "key" | "pem" | "p12" | "pfx" | "pyc" | "class" | "o" | "so"
    )
}

fn safe_project_entry(
    path: &str,
    request: &ProbeRequest,
    state: &ProbeState,
    cx: &App,
) -> std::result::Result<PathBuf, &'static str> {
    if path.is_empty() || path.len() > 1024 {
        return Err("INVALID_REQUEST");
    }
    let relative = Path::new(path);
    if relative.is_absolute()
        || excluded_remote_path(path)
        || relative.components().any(|component| {
            !matches!(component, Component::Normal(_))
                || component
                    .as_os_str()
                    .to_str()
                    .is_none_or(|part| part.starts_with('.'))
        })
    {
        return Err("INVALID_REQUEST");
    }
    let worktree = exposed_worktree(request, state, cx)?;
    let allowed = worktree.read(cx).entries(false, 0).any(|entry| {
        entry.path.as_unix_str() == path
            && !entry.is_dir()
            && !entry.is_hidden
            && !entry.is_private
            && !entry.is_external
            && !entry.is_fifo
            && !entry.is_ignored
    });
    if !allowed {
        return Err("FILE_UNAVAILABLE");
    }
    let candidate =
        fs::canonicalize(state.project_root.join(relative)).map_err(|_| "FILE_UNAVAILABLE")?;
    if !candidate.starts_with(&state.project_root) || !candidate.is_file() {
        return Err("FILE_UNAVAILABLE");
    }
    Ok(candidate)
}

fn project_files(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let worktree = match exposed_worktree(request, state, cx) {
        Ok(worktree) => worktree,
        Err(code) => return error(&request.request_id, code),
    };
    let directory = if request.method == "project.directory" {
        let Some(path) = request.params.get("path").and_then(Value::as_str) else {
            return error(&request.request_id, "INVALID_REQUEST");
        };
        if !path.is_empty() {
            let relative = Path::new(path);
            if path.len() > 1024
                || relative.is_absolute()
                || excluded_remote_path(path)
                || relative
                    .components()
                    .any(|component| !matches!(component, Component::Normal(_)))
            {
                return error(&request.request_id, "INVALID_REQUEST");
            }
            let allowed = worktree.read(cx).entries(false, 0).any(|entry| {
                entry.path.as_unix_str() == path
                    && entry.is_dir()
                    && !entry.is_hidden
                    && !entry.is_private
                    && !entry.is_external
                    && !entry.is_fifo
                    && !entry.is_ignored
            });
            let absolute = fs::canonicalize(state.project_root.join(path));
            if !allowed
                || !absolute
                    .is_ok_and(|path| path.starts_with(&state.project_root) && path.is_dir())
            {
                return error(&request.request_id, "FILE_UNAVAILABLE");
            }
        }
        Some(path)
    } else {
        None
    };
    let mut truncated = false;
    let entries = worktree
        .read(cx)
        .entries(false, 0)
        .filter(|entry| {
            !entry.path.is_empty()
                && !entry.is_hidden
                && !entry.is_private
                && !entry.is_external
                && !entry.is_fifo
                && !entry.is_ignored
                && !excluded_remote_path(entry.path.as_unix_str())
                && directory.is_none_or(|directory| {
                    let path = entry.path.as_unix_str();
                    let remainder = if directory.is_empty() {
                        Some(path)
                    } else {
                        path.strip_prefix(directory)
                            .and_then(|suffix| suffix.strip_prefix('/'))
                    };
                    remainder.is_some_and(|path| !path.is_empty() && !path.contains('/'))
                })
        })
        .take(MAX_PROJECT_FILES + 1)
        .enumerate()
        .filter_map(|(index, entry)| {
            if index == MAX_PROJECT_FILES {
                truncated = true;
                return None;
            }
            Some(json!({
                "path": entry.path.as_unix_str(),
                "kind": if entry.is_dir() { "directory" } else { "file" },
                "size": entry.size,
            }))
        })
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({"entries": entries, "directory": directory, "truncated": truncated, "provisional": false}),
    )
}

fn read_project_text(path: &Path) -> std::result::Result<(String, bool), &'static str> {
    let file = fs::File::open(path).map_err(|_| "FILE_UNAVAILABLE")?;
    let mut bytes = Vec::new();
    file.take((MAX_FILE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "FILE_UNAVAILABLE")?;
    let truncated = bytes.len() > MAX_FILE_BYTES;
    if truncated {
        bytes.truncate(MAX_FILE_BYTES);
        while std::str::from_utf8(&bytes).is_err() && !bytes.is_empty() {
            bytes.pop();
        }
    }
    if bytes.contains(&0) {
        return Err("UNSUPPORTED_FILE");
    }
    let content = String::from_utf8(bytes).map_err(|_| "UNSUPPORTED_FILE")?;
    Ok((content, truncated))
}

fn project_file(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(path) = request.params.get("path").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let absolute = match safe_project_entry(path, request, state, cx) {
        Ok(path) => path,
        Err(code) => return error(&request.request_id, code),
    };
    let (content, truncated) = match read_project_text(&absolute) {
        Ok(content) => content,
        Err(code) => return error(&request.request_id, code),
    };
    success(
        &request.request_id,
        json!({"path": path, "content": content, "truncated": truncated, "provisional": false}),
    )
}

fn search_project_paths(
    project_root: PathBuf,
    paths: Vec<String>,
    query: String,
    case_sensitive: bool,
    candidate_truncated: bool,
) -> SearchRecord {
    let needle = if case_sensitive {
        query
    } else {
        query.to_ascii_lowercase()
    };
    let mut matches = Vec::new();
    let mut scanned_bytes = 0;
    let mut truncated = candidate_truncated;
    for path in paths.into_iter().take(MAX_SEARCH_FILES) {
        if scanned_bytes >= MAX_SEARCH_BYTES || matches.len() >= MAX_SEARCH_MATCHES {
            truncated = true;
            break;
        }
        let Ok(absolute) = fs::canonicalize(project_root.join(&path)) else {
            continue;
        };
        if !absolute.starts_with(&project_root) || !absolute.is_file() {
            continue;
        }
        let Ok(file) = fs::File::open(&absolute) else {
            continue;
        };
        let remaining = MAX_SEARCH_BYTES.saturating_sub(scanned_bytes);
        let limit = remaining.min(MAX_FILE_BYTES);
        let mut bytes = Vec::new();
        if file
            .take((limit + 1) as u64)
            .read_to_end(&mut bytes)
            .is_err()
        {
            continue;
        }
        if bytes.len() > limit {
            bytes.truncate(limit);
            truncated = true;
        }
        scanned_bytes = scanned_bytes.saturating_add(bytes.len());
        if bytes.contains(&0) {
            continue;
        }
        let Ok(content) = std::str::from_utf8(&bytes) else {
            continue;
        };
        for (line_index, line) in content.lines().enumerate() {
            let haystack = if case_sensitive {
                line.to_string()
            } else {
                line.to_ascii_lowercase()
            };
            let Some(column) = haystack.find(&needle) else {
                continue;
            };
            matches.push(json!({
                "path": path,
                "line": line_index + 1,
                "column": line[..column].chars().count() + 1,
                "preview": line.chars().take(300).collect::<String>(),
            }));
            if matches.len() == MAX_SEARCH_MATCHES {
                truncated = true;
                break;
            }
        }
    }
    SearchRecord::Ready { matches, truncated }
}

fn project_search(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(query) = request.params.get("query").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if query.trim().len() < 2
        || query.len() > 256
        || query
            .chars()
            .any(|character| matches!(character, '\n' | '\r' | '\0'))
    {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let case_sensitive = request
        .params
        .get("case_sensitive")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let worktree = match exposed_worktree(request, state, cx) {
        Ok(worktree) => worktree,
        Err(code) => return error(&request.request_id, code),
    };
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let key = format!("{thread_id}:{case_sensitive}:{query}");
    let existing_search = state.searches.borrow_mut().remove(&key);
    if let Some(record) = existing_search {
        return match record {
            SearchRecord::Loading => {
                state
                    .searches
                    .borrow_mut()
                    .insert(key, SearchRecord::Loading);
                success(
                    &request.request_id,
                    json!({"outcome": "loading", "provisional": true}),
                )
            }
            SearchRecord::Ready { matches, truncated } => success(
                &request.request_id,
                json!({"outcome": "ready", "matches": matches, "truncated": truncated, "provisional": false}),
            ),
        };
    }
    if state.searches.borrow().len() >= 32 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    let mut paths = worktree
        .read(cx)
        .entries(false, 0)
        .filter(|entry| {
            !entry.is_dir()
                && !entry.is_hidden
                && !entry.is_private
                && !entry.is_external
                && !entry.is_fifo
                && !entry.is_ignored
                && !excluded_remote_path(entry.path.as_unix_str())
        })
        .take(MAX_SEARCH_FILES + 1)
        .map(|entry| entry.path.as_unix_str().to_string())
        .collect::<Vec<_>>();
    let candidate_truncated = paths.len() > MAX_SEARCH_FILES;
    paths.truncate(MAX_SEARCH_FILES);
    state
        .searches
        .borrow_mut()
        .insert(key.clone(), SearchRecord::Loading);
    let project_root = state.project_root.clone();
    let query = query.to_string();
    let task = cx.background_spawn(async move {
        search_project_paths(
            project_root,
            paths,
            query,
            case_sensitive,
            candidate_truncated,
        )
    });
    let probe_state = cx.global::<ProbeGlobal>().0.clone();
    cx.spawn(async move |_cx| {
        let record = task.await;
        probe_state.searches.borrow_mut().insert(key, record);
    })
    .detach();
    success(
        &request.request_id,
        json!({"outcome": "loading", "provisional": true}),
    )
}

fn change_kind(status: FileStatus) -> &'static str {
    if status.is_conflicted() {
        "conflict"
    } else if status.is_deleted() {
        "deleted"
    } else if status.is_created() {
        "added"
    } else if status.is_modified() {
        "modified"
    } else if status.is_untracked() {
        "untracked"
    } else {
        "changed"
    }
}

fn stage_kind(status: StageStatus) -> &'static str {
    match status {
        StageStatus::Staged => "staged",
        StageStatus::Unstaged => "unstaged",
        StageStatus::PartiallyStaged => "partial",
    }
}

fn changes_snapshot(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let thread = match exposed_thread_for_request(request, state, cx) {
        Ok(thread) => thread,
        Err(code) => return error(&request.request_id, code),
    };
    let project = thread.read(cx).project().clone();
    let Some(repository) = project.read(cx).active_repository(cx) else {
        return success(
            &request.request_id,
            json!({"repository": null, "entries": [], "provisional": false}),
        );
    };
    let repository = repository.read(cx);
    let head = repository
        .head_commit
        .as_ref()
        .map(|commit| commit.sha.to_string());
    let revision = format!(
        "{}:{}",
        head.as_deref().unwrap_or("unborn"),
        repository.scan_id
    );
    let entries = repository
        .cached_status()
        .filter(|entry| !entry.status.is_ignored())
        .filter(|entry| {
            repository
                .repo_path_to_abs_path(&entry.repo_path)
                .starts_with(&state.project_root)
        })
        .map(|entry| {
            json!({
                "path": entry.repo_path.as_unix_str(),
                "status": change_kind(entry.status),
                "staging": stage_kind(entry.status.staging()),
                "added": entry.diff_stat.map(|stat| stat.added),
                "deleted": entry.diff_stat.map(|stat| stat.deleted),
            })
        })
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({
            "repository": {
                "branch": repository.branch.as_ref().map(|branch| branch.name()),
                "head": head,
                "revision": revision,
            },
            "entries": entries,
            "provisional": false,
        }),
    )
}

fn changes_diff(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let thread = match exposed_thread_for_request(request, state, cx) {
        Ok(thread) => thread,
        Err(code) => return error(&request.request_id, code),
    };
    let Some(expected_revision) = request.params.get("revision").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let project = thread.read(cx).project().clone();
    let Some(repository) = project.read(cx).active_repository(cx) else {
        return error(&request.request_id, "REPOSITORY_UNAVAILABLE");
    };
    let actual_revision = {
        let repository = repository.read(cx);
        format!(
            "{}:{}",
            repository
                .head_commit
                .as_ref()
                .map(|commit| commit.sha.as_ref())
                .unwrap_or("unborn"),
            repository.scan_id
        )
    };
    if expected_revision != actual_revision {
        return error(&request.request_id, "STALE_REVISION");
    }
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if let Some(record) = state.diffs.borrow().get(thread_id) {
        match record {
            DiffRecord::Loading { revision } if revision == expected_revision => {
                return success(
                    &request.request_id,
                    json!({"outcome": "loading", "revision": revision}),
                );
            }
            DiffRecord::Ready {
                revision,
                diff,
                truncated,
            } if revision == expected_revision => {
                return success(
                    &request.request_id,
                    json!({"outcome": "ready", "revision": revision, "diff": diff, "truncated": truncated}),
                );
            }
            DiffRecord::Failed { revision } if revision == expected_revision => {
                return error(&request.request_id, "DIFF_UNAVAILABLE");
            }
            _ => {}
        }
    }

    let receiver = repository.update(cx, |repository, cx| {
        repository.diff(DiffType::HeadToWorktree, cx)
    });
    state.diffs.borrow_mut().insert(
        thread_id.to_string(),
        DiffRecord::Loading {
            revision: actual_revision.clone(),
        },
    );
    let probe_state = cx.global::<ProbeGlobal>().0.clone();
    let thread_id = thread_id.to_string();
    let revision = actual_revision.clone();
    cx.spawn(async move |_cx| {
        let record = match receiver.await {
            Ok(Ok(mut diff)) => {
                let truncated = diff.len() > MAX_DIFF_BYTES;
                if truncated {
                    let mut end = MAX_DIFF_BYTES;
                    while !diff.is_char_boundary(end) {
                        end -= 1;
                    }
                    diff.truncate(end);
                }
                DiffRecord::Ready {
                    revision,
                    diff,
                    truncated,
                }
            }
            _ => DiffRecord::Failed { revision },
        };
        probe_state.diffs.borrow_mut().insert(thread_id, record);
    })
    .detach();
    success(
        &request.request_id,
        json!({"outcome": "loading", "revision": actual_revision}),
    )
}

fn change_operation_result(request_id: &str, state: &ChangeOperationState) -> Value {
    match state {
        ChangeOperationState::Running => success(
            request_id,
            json!({"outcome": "running", "provisional": true}),
        ),
        ChangeOperationState::Succeeded => success(
            request_id,
            json!({"outcome": "succeeded", "provisional": false}),
        ),
        ChangeOperationState::Failed => error(request_id, "OPERATION_FAILED"),
    }
}

fn change_stage(request: &ProbeRequest, state: &ProbeState, stage: bool, cx: &mut App) -> Value {
    let thread = match exposed_thread_for_request(request, state, cx) {
        Ok(thread) => thread,
        Err(code) => return error(&request.request_id, code),
    };
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(operation) = state.change_operations.borrow().get(&request.request_id) {
        if operation.payload_digest != digest {
            return error(&request.request_id, "REQUEST_CONFLICT");
        }
        return change_operation_result(&request.request_id, &operation.state);
    }
    if state.change_operations.borrow().len() >= 128 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    let Some(expected_revision) = request.params.get("revision").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(path_values) = request.params.get("paths").and_then(Value::as_array) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if path_values.is_empty() || path_values.len() > 100 {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let project = thread.read(cx).project().clone();
    let Some(repository) = project.read(cx).active_repository(cx) else {
        return error(&request.request_id, "REPOSITORY_UNAVAILABLE");
    };
    let (actual_revision, paths) = {
        let repository = repository.read(cx);
        let actual_revision = format!(
            "{}:{}",
            repository
                .head_commit
                .as_ref()
                .map(|commit| commit.sha.as_ref())
                .unwrap_or("unborn"),
            repository.scan_id
        );
        if expected_revision != actual_revision {
            return error(&request.request_id, "STALE_REVISION");
        }
        let mut paths = Vec::with_capacity(path_values.len());
        for value in path_values {
            let Some(path) = value.as_str() else {
                return error(&request.request_id, "INVALID_REQUEST");
            };
            let Some(entry) = repository
                .cached_status()
                .find(|entry| entry.repo_path.as_unix_str() == path)
            else {
                return error(&request.request_id, "STALE_REVISION");
            };
            if !repository
                .repo_path_to_abs_path(&entry.repo_path)
                .starts_with(&state.project_root)
                || (stage && entry.status.staging() == StageStatus::Staged)
                || (!stage && entry.status.staging() == StageStatus::Unstaged)
            {
                return error(&request.request_id, "INVALID_REQUEST");
            }
            let relative = match RelPath::from_unix_str(path) {
                Ok(relative) => relative,
                Err(_) => return error(&request.request_id, "INVALID_REQUEST"),
            };
            paths.push(RepoPath::from_rel_path(relative));
        }
        (actual_revision, paths)
    };
    let task = repository.update(cx, |repository, cx| {
        if stage {
            repository.stage_entries(paths, cx)
        } else {
            repository.unstage_entries(paths, cx)
        }
    });
    state.change_operations.borrow_mut().insert(
        request.request_id.clone(),
        ChangeOperationRecord {
            payload_digest: digest,
            state: ChangeOperationState::Running,
        },
    );
    let probe_state = cx.global::<ProbeGlobal>().0.clone();
    let request_id = request.request_id.clone();
    cx.spawn(async move |_cx| {
        let next_state = if task.await.is_ok() {
            ChangeOperationState::Succeeded
        } else {
            ChangeOperationState::Failed
        };
        if let Some(operation) = probe_state
            .change_operations
            .borrow_mut()
            .get_mut(&request_id)
        {
            operation.state = next_state;
        }
    })
    .detach();
    success(
        &request.request_id,
        json!({"outcome": "running", "revision": actual_revision, "provisional": true}),
    )
}

fn payload_digest(params: &Value) -> std::result::Result<String, &'static str> {
    let encoded = serde_json::to_vec(params).map_err(|_| "INVALID_REQUEST")?;
    let digest = Sha256::digest(encoded);
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn attachment_content(
    request: &ProbeRequest,
    state: &ProbeState,
    cx: &App,
) -> std::result::Result<Vec<acp::ContentBlock>, &'static str> {
    let mut content = Vec::new();
    let context_paths = match request.params.get("context_paths") {
        None => &[][..],
        Some(Value::Array(paths)) if paths.len() <= MAX_ATTACHMENTS => paths.as_slice(),
        _ => return Err("INVALID_REQUEST"),
    };
    for value in context_paths {
        let path = value.as_str().ok_or("INVALID_REQUEST")?;
        let absolute = safe_project_entry(path, request, state, cx)?;
        let (text, truncated) = read_project_text(&absolute)?;
        if truncated {
            return Err("FILE_TOO_LARGE");
        }
        let uri = url::Url::from_file_path(&absolute)
            .map_err(|_| "FILE_UNAVAILABLE")?
            .to_string();
        content.push(acp::ContentBlock::Resource(acp::EmbeddedResource::new(
            acp::EmbeddedResourceResource::TextResourceContents(acp::TextResourceContents::new(
                text, uri,
            )),
        )));
    }

    let uploads = match request.params.get("uploads") {
        None => &[][..],
        Some(Value::Array(uploads))
            if uploads.len() <= MAX_ATTACHMENTS
                && uploads.len().saturating_add(context_paths.len()) <= MAX_ATTACHMENTS =>
        {
            uploads.as_slice()
        }
        _ => return Err("INVALID_REQUEST"),
    };
    for upload in uploads {
        let Some(upload) = upload.as_object() else {
            return Err("INVALID_REQUEST");
        };
        if upload
            .keys()
            .any(|key| !matches!(key.as_str(), "name" | "mime_type" | "data"))
        {
            return Err("INVALID_REQUEST");
        }
        let Some(name) = upload.get("name").and_then(Value::as_str) else {
            return Err("INVALID_REQUEST");
        };
        let Some(mime_type) = upload.get("mime_type").and_then(Value::as_str) else {
            return Err("INVALID_REQUEST");
        };
        let Some(data) = upload.get("data").and_then(Value::as_str) else {
            return Err("INVALID_REQUEST");
        };
        if name.is_empty()
            || name.len() > 128
            || Path::new(name).file_name().and_then(|part| part.to_str()) != Some(name)
            || name.starts_with('.')
            || mime_type.len() > 128
        {
            return Err("INVALID_REQUEST");
        }
        let decoded = STANDARD.decode(data).map_err(|_| "INVALID_REQUEST")?;
        if decoded.len() > MAX_UPLOAD_BYTES {
            return Err("PAYLOAD_TOO_LARGE");
        }
        if matches!(mime_type, "image/png" | "image/jpeg" | "image/webp") {
            content.push(acp::ContentBlock::Image(
                acp::ImageContent::new(data, mime_type)
                    .uri(Some(format!("zedlink-upload:///{name}"))),
            ));
        } else if mime_type.starts_with("text/")
            || matches!(mime_type, "application/json" | "application/xml")
        {
            if decoded.len() > MAX_FILE_BYTES || decoded.contains(&0) {
                return Err("FILE_TOO_LARGE");
            }
            let text = String::from_utf8(decoded).map_err(|_| "UNSUPPORTED_FILE")?;
            content.push(acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                acp::EmbeddedResourceResource::TextResourceContents(
                    acp::TextResourceContents::new(text, format!("zedlink-upload:///{name}")),
                ),
            )));
        } else {
            return Err("UNSUPPORTED_FILE");
        }
    }
    Ok(content)
}

fn send_request(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(text) = request.params.get("text").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if text.trim().is_empty() || text.len() > 3000 {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    if !state.exposed_threads.borrow().contains(thread_id) {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    }
    let payload_digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };

    let submissions = state.submissions.borrow();
    if let Some(previous) = submissions.get(&request.request_id) {
        if previous.thread_id != thread_id || previous.payload_digest != payload_digest {
            return error(&request.request_id, "REQUEST_CONFLICT");
        }
        return send_result(&request.request_id, previous.outcome);
    }
    if submissions.len() >= 256 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    drop(submissions);

    let Some((thread, view)) = loaded_thread_views(&state.project_root, cx)
        .into_iter()
        .find(|(thread, _)| thread.read(cx).session_id().to_string() == thread_id)
    else {
        return error(&request.request_id, "THREAD_NOT_EXPOSED");
    };
    let slash_name = text
        .trim_start()
        .strip_prefix('/')
        .map(|rest| rest.split_whitespace().next().unwrap_or(""));
    let catalog = remote_command_catalog(&thread, &view, cx);
    let slash_text = text.trim_start().strip_prefix('/');
    let selected_command = slash_text.and_then(|input| {
        let matches = |name: &str| {
            input.strip_prefix(name).is_some_and(|remainder| {
                remainder.is_empty() || remainder.starts_with(char::is_whitespace)
            })
        };
        catalog
            .iter()
            .filter(|command| {
                command
                    .get("name")
                    .and_then(Value::as_str)
                    .is_some_and(matches)
            })
            .max_by_key(|command| {
                command
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::len)
                    .unwrap_or(0)
            })
            .or_else(|| {
                catalog.iter().find(|command| {
                    command
                        .get("bare_name")
                        .and_then(Value::as_str)
                        .is_some_and(matches)
                })
            })
    });
    if slash_name.is_some() && selected_command.is_none() {
        return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
    }
    if selected_command
        .is_some_and(|command| command.get("execution").and_then(Value::as_str) == Some("local"))
    {
        if request
            .params
            .get("local_confirmed")
            .and_then(Value::as_bool)
            != Some(true)
            || text.trim().split_whitespace().count() != 1
            || request.params.get("context_paths").is_some()
            || request.params.get("uploads").is_some()
        {
            return error(&request.request_id, "INVALID_REQUEST");
        }
        if slash_name == Some("remote-control")
            && !state
                .permissions
                .get()
                .allows("thread.unexpose", &request.params)
        {
            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
        }
        let local_command = view
            .read(cx)
            .remote_local_commands(cx)
            .into_iter()
            .find(|command| Some(command.keyword()) == slash_name);
        let Some(local_command) = local_command else {
            return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
        };
        let executed = cx.with_window(view.entity_id(), |window, cx| {
            view.update(cx, |view, cx| {
                view.run_remote_local_command(local_command, window, cx)
            })
        });
        if executed != Some(true) {
            return error(&request.request_id, "INSTANCE_UNAVAILABLE");
        }
        let outcome = if slash_name == Some("not-helpful") {
            "desktop_review_required"
        } else {
            "local_completed"
        };
        state.submissions.borrow_mut().insert(
            request.request_id.clone(),
            SubmissionRecord {
                thread_id: thread_id.to_string(),
                payload_digest,
                outcome,
            },
        );
        return send_result(&request.request_id, outcome);
    }
    let native_command = selected_command
        .is_some_and(|command| command.get("category").and_then(Value::as_str) == Some("native"));
    if native_command
        && (request.params.get("context_paths").is_some()
            || request.params.get("uploads").is_some())
    {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    if view.read(cx).is_loading_message_contents() {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }

    let mut content = vec![acp::ContentBlock::Text(acp::TextContent::new(text))];
    match attachment_content(request, state, cx) {
        Ok(attachments) => content.extend(attachments),
        Err(code) => return error(&request.request_id, code),
    }
    if let Some(command) = selected_command
        .filter(|command| command.get("category").and_then(Value::as_str) == Some("skill"))
    {
        let name = command.get("name").and_then(Value::as_str).unwrap_or("");
        // Keep bare names on the agent's normal dispatch path, where native MCP/skill precedence is defined.
        if let Some(remainder) = text
            .trim_start()
            .strip_prefix('/')
            .and_then(|input| input.strip_prefix(name))
        {
            let skill = view
                .read(cx)
                .session_capabilities
                .read()
                .available_skills()
                .iter()
                .find(|skill| format!("{}:{}", skill.source, skill.name) == name)
                .cloned();
            let Some(skill) = skill else {
                return error(&request.request_id, "UNSUPPORTED_CAPABILITY");
            };
            if let Some(acp::ContentBlock::Text(text)) = content.first_mut() {
                text.text = remainder.trim_start().to_string();
            }
            let executed = cx.with_window(view.entity_id(), |window, cx| {
                view.update(cx, |view, cx| {
                    view.send_remote_skill(skill, content, window, cx)
                })
            });
            if executed != Some(true) {
                return error(&request.request_id, "INSTANCE_UNAVAILABLE");
            }
            state.submissions.borrow_mut().insert(
                request.request_id.clone(),
                SubmissionRecord {
                    thread_id: thread_id.to_string(),
                    payload_digest,
                    outcome: "unknown",
                },
            );
            return send_result(&request.request_id, "unknown");
        }
    }
    let can_send_now =
        thread.read(cx).status() == ThreadStatus::Idle && !view.read(cx).has_queued_messages();
    let sent = cx.with_window(view.entity_id(), |window, cx| {
        view.update(cx, |view, cx| {
            if can_send_now {
                if native_command {
                    let name = slash_name.unwrap_or("");
                    let remainder = text
                        .trim_start()
                        .strip_prefix('/')
                        .unwrap_or("")
                        .strip_prefix(name)
                        .unwrap_or("")
                        .trim_start();
                    if !remainder.is_empty() {
                        view.add_to_queue(
                            vec![acp::ContentBlock::Text(acp::TextContent::new(remainder))],
                            Vec::new(),
                            window,
                            cx,
                        );
                    }
                    view.send_content(
                        Task::ready(Ok(Some((
                            vec![acp::ContentBlock::Text(acp::TextContent::new(format!(
                                "/{name}"
                            )))],
                            Vec::new(),
                        )))),
                        true,
                        window,
                        cx,
                    );
                } else {
                    view.send_content(
                        Task::ready(Ok(Some((content, Vec::new())))),
                        false,
                        window,
                        cx,
                    );
                }
                "unknown"
            } else {
                view.add_to_queue(content, Vec::new(), window, cx);
                "queued"
            }
        })
    });
    let Some(outcome) = sent else {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    };
    state.submissions.borrow_mut().insert(
        request.request_id.clone(),
        SubmissionRecord {
            thread_id: thread_id.to_string(),
            payload_digest,
            outcome,
        },
    );
    send_result(&request.request_id, outcome)
}

fn send_result(request_id: &str, outcome: &str) -> Value {
    success(request_id, json!({"outcome": outcome, "provisional": true}))
}

fn zedlink_error_value(
    had_error: bool,
    error: Option<(&'static str, String, String, bool)>,
) -> Value {
    match error {
        Some((kind, title, message, retrying)) => json!({
            "kind": kind,
            "title": title,
            "message": message.chars().take(2048).collect::<String>(),
            "retrying": retrying,
        }),
        None if had_error => json!({
            "kind": "agent_error",
            "title": "Agent request failed",
            "message": "The last native Zed Agent turn failed. Open desktop Zed for any additional detail.",
            "retrying": false,
        }),
        None => Value::Null,
    }
}

fn zedlink_retry_value(retry: Option<(String, usize, usize, u64)>) -> Value {
    match retry {
        Some((last_error, attempt, max_attempts, next_attempt_in_ms)) => json!({
            "last_error": last_error.chars().take(1024).collect::<String>(),
            "attempt": attempt,
            "max_attempts": max_attempts,
            "next_attempt_in_ms": next_attempt_in_ms,
        }),
        None => Value::Null,
    }
}

fn loaded_threads(project_root: &Path, cx: &App) -> Vec<Entity<AcpThread>> {
    loaded_thread_views(project_root, cx)
        .into_iter()
        .map(|(thread, _)| thread)
        .collect()
}

fn terminal_list(request: &ProbeRequest, state: &ProbeState, cx: &App) -> Value {
    let terminals = loaded_terminal_views(&state.project_root, cx)
        .iter()
        .map(|view| terminal_value(view, cx))
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({
            "instance_id": state.instance_id,
            "epoch": state.epoch,
            "terminals": terminals,
            "provisional": true,
        }),
    )
}

fn terminal_snapshot(request: &ProbeRequest, state: &ProbeState, cx: &App) -> Value {
    let Some(id) = request.params.get("terminal_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(view) = loaded_terminal_views(&state.project_root, cx)
        .into_iter()
        .find(|view| terminal_id(view) == id)
    else {
        return error(&request.request_id, "TERMINAL_UNAVAILABLE");
    };
    let metadata = terminal_value(&view, cx);
    let terminal = view.read(cx).terminal().clone();
    let terminal = terminal.read(cx);
    let content = terminal.get_content();
    let mut start = content.len().saturating_sub(MAX_TERMINAL_SNAPSHOT_BYTES);
    while !content.is_char_boundary(start) {
        start += 1;
    }
    let mut runs = terminal.get_styled_content(MAX_TERMINAL_SNAPSHOT_BYTES);
    let styles_truncated = runs.len() > 2048;
    if styles_truncated {
        let prefix = runs
            .drain(..runs.len() - 2047)
            .map(|(text, _)| text)
            .collect::<String>();
        runs.insert(0, (prefix, terminal::Cell::default()));
    }
    let colored_runs = runs.into_iter().map(|(text, cell)| {
            let mut fg = cell.foreground();
            let mut bg = cell.background();
            if cell.is_inverse() { std::mem::swap(&mut fg, &mut bg); }
            let hex = |color| {
                let rgba: gpui::Rgba = terminal_view::terminal_element::convert_color(&color, cx.theme()).into();
                format!("#{:02x}{:02x}{:02x}", (rgba.r * 255.0).round() as u8, (rgba.g * 255.0).round() as u8, (rgba.b * 255.0).round() as u8)
            };
            json!({"text": text, "foreground": hex(fg), "background": hex(bg), "bold": cell.is_bold(), "italic": cell.is_italic(), "underline": cell.has_underline(), "dim": cell.is_dim()})
        }).collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({
            "instance_id": state.instance_id,
            "epoch": state.epoch,
            "terminal": metadata,
            "content": &content[start..],
            "styled_runs": colored_runs,
            "styles_truncated": styles_truncated,
            "truncated": start > 0,
            "total_lines": terminal.total_lines(),
            "viewport_lines": terminal.viewport_lines(),
            "provisional": true,
        }),
    )
}

fn remote_clipboard(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if let Err(code) = exposed_thread_for_request(request, state, cx) {
        return error(&request.request_id, code);
    }
    if request.method == "clipboard.read" {
        let text = cx
            .read_from_clipboard()
            .and_then(|item| item.text())
            .unwrap_or_default();
        if text.len() > 65536 {
            return error(&request.request_id, "CLIPBOARD_TOO_LARGE");
        }
        return success(&request.request_id, json!({"text": text}));
    }
    let Some(text) = request.params.get("text").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if text.len() > 65536 || text.contains('\0') {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(prior) = state.clipboard_operations.borrow().get(&request.request_id) {
        return if prior == &digest {
            success(
                &request.request_id,
                json!({"outcome": "accepted", "duplicate": true}),
            )
        } else {
            error(&request.request_id, "REQUEST_ID_CONFLICT")
        };
    }
    if state.clipboard_operations.borrow().len() >= 256 {
        return error(&request.request_id, "CLIPBOARD_WRITE_LIMIT");
    }
    cx.write_to_clipboard(ClipboardItem::new_string(text.to_owned()));
    state
        .clipboard_operations
        .borrow_mut()
        .insert(request.request_id.clone(), digest);
    success(
        &request.request_id,
        json!({"outcome": "accepted", "duplicate": false}),
    )
}

fn terminal_input(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let Some(id) = request.params.get("terminal_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(input) = request.params.get("input").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if input.is_empty() || input.len() > MAX_TERMINAL_INPUT_BYTES {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(existing) = state.terminal_operations.borrow().get(&request.request_id) {
        return if existing == &digest {
            success(
                &request.request_id,
                json!({"outcome": "accepted", "duplicate": true}),
            )
        } else {
            error(&request.request_id, "REQUEST_ID_CONFLICT")
        };
    }
    let Some(view) = loaded_terminal_views(&state.project_root, cx)
        .into_iter()
        .find(|view| terminal_id(view) == id)
    else {
        return error(&request.request_id, "TERMINAL_UNAVAILABLE");
    };
    let terminal = view.read(cx).terminal().clone();
    terminal.update(cx, |terminal, _| terminal.input(input.as_bytes().to_vec()));
    state
        .terminal_operations
        .borrow_mut()
        .insert(request.request_id.clone(), digest);
    success(
        &request.request_id,
        json!({"outcome": "accepted", "duplicate": false}),
    )
}

fn task_list(request: &ProbeRequest, state: &ProbeState, cx: &App) -> Value {
    let tasks = loaded_terminal_views(&state.project_root, cx)
        .into_iter()
        .filter_map(|view| {
            let id = terminal_id(&view);
            let view = view.read(cx);
            let terminal = view.terminal().read(cx);
            let task = terminal.task()?;
            Some(json!({
                "terminal_id": id,
                "task_id": task.spawned_task.id.0,
                "label": task.spawned_task.full_label,
                "command_label": task.spawned_task.command_label,
                "status": terminal_task_status(task.status),
            }))
        })
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({
            "instance_id": state.instance_id,
            "epoch": state.epoch,
            "tasks": tasks,
            "provisional": true,
        }),
    )
}

fn task_stop(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if request.params.get("instance_id").and_then(Value::as_str) != Some(state.instance_id.as_str())
        || request.params.get("epoch").and_then(Value::as_str) != Some(state.epoch.as_str())
    {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let Some(id) = request.params.get("terminal_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(existing) = state.terminal_operations.borrow().get(&request.request_id) {
        return if existing == &digest {
            success(
                &request.request_id,
                json!({"outcome": "requested", "duplicate": true}),
            )
        } else {
            error(&request.request_id, "REQUEST_ID_CONFLICT")
        };
    }
    let Some(view) = loaded_terminal_views(&state.project_root, cx)
        .into_iter()
        .find(|view| terminal_id(view) == id)
    else {
        return error(&request.request_id, "TERMINAL_UNAVAILABLE");
    };
    let terminal = view.read(cx).terminal().clone();
    if !terminal
        .read(cx)
        .task()
        .is_some_and(|task| task.status == TaskStatus::Running)
    {
        return error(&request.request_id, "TASK_NOT_RUNNING");
    }
    terminal.update(cx, |terminal, _| terminal.kill_active_task());
    state
        .terminal_operations
        .borrow_mut()
        .insert(request.request_id.clone(), digest);
    success(
        &request.request_id,
        json!({"outcome": "requested", "duplicate": false}),
    )
}

fn task_catalog(request: &ProbeRequest, state: &ProbeState, cx: &App) -> Value {
    if !matches_instance(request, state) {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let tasks = configured_tasks(&state.project_root, cx)
        .into_iter()
        .map(|(_, source, task)| {
            json!({
                "task_id": task.id.0,
                "label": task.display_label(),
                "source": task_source_label(&source),
            })
        })
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({
            "instance_id": state.instance_id,
            "epoch": state.epoch,
            "tasks": tasks,
            "provisional": true,
        }),
    )
}

fn task_run(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if !matches_instance(request, state) {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    if !params_contain_only(&request.params, &["instance_id", "epoch", "task_id"]) {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let Some(task_id) = request.params.get("task_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if task_id.is_empty() || task_id.len() > 4096 {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(existing) = state.terminal_operations.borrow().get(&request.request_id) {
        return if existing == &digest {
            success(
                &request.request_id,
                json!({"outcome": "requested", "duplicate": true}),
            )
        } else {
            error(&request.request_id, "REQUEST_ID_CONFLICT")
        };
    }
    if state.terminal_operations.borrow().len() >= 256 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    let Some((workspace, source, task)) = configured_tasks(&state.project_root, cx)
        .into_iter()
        .find(|(_, _, task)| task.id.0 == task_id)
    else {
        return error(&request.request_id, "TASK_UNAVAILABLE");
    };
    let scheduled = cx.with_window(workspace.entity_id(), |window, cx| {
        workspace.update(cx, |workspace, cx| {
            workspace.schedule_resolved_task(source, task, false, window, cx);
        });
    });
    if scheduled.is_none() {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    state
        .terminal_operations
        .borrow_mut()
        .insert(request.request_id.clone(), digest);
    success(
        &request.request_id,
        json!({"outcome": "requested", "duplicate": false}),
    )
}

fn debugger_catalog(request: &ProbeRequest, state: &ProbeState, cx: &App) -> Value {
    if !matches_instance(request, state) {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let scenarios = configured_debug_scenarios(&state.project_root, cx)
        .into_iter()
        .map(|(_, source, scenario, _)| {
            json!({
                "scenario_id": debug_scenario_id(&source, &scenario),
                "label": scenario.label,
                "adapter": scenario.adapter,
                "source": task_source_label(&source),
            })
        })
        .collect::<Vec<_>>();
    success(
        &request.request_id,
        json!({
            "instance_id": state.instance_id,
            "epoch": state.epoch,
            "scenarios": scenarios,
            "provisional": true,
        }),
    )
}

fn debugger_launch(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if !matches_instance(request, state) {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    if !params_contain_only(&request.params, &["instance_id", "epoch", "scenario_id"]) {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let Some(scenario_id) = request.params.get("scenario_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if scenario_id.is_empty() || scenario_id.len() > 128 {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(existing) = state.terminal_operations.borrow().get(&request.request_id) {
        return if existing == &digest {
            success(
                &request.request_id,
                json!({"outcome": "requested", "duplicate": true}),
            )
        } else {
            error(&request.request_id, "REQUEST_ID_CONFLICT")
        };
    }
    if state.terminal_operations.borrow().len() >= 256 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    let Some((workspace, source, scenario, context)) =
        configured_debug_scenarios(&state.project_root, cx)
            .into_iter()
            .find(|(_, source, scenario, _)| debug_scenario_id(source, scenario) == scenario_id)
    else {
        return error(&request.request_id, "DEBUGGER_UNAVAILABLE");
    };
    let worktree_id = match &source {
        TaskSourceKind::Worktree { id, .. } => Some(*id),
        _ => None,
    };
    let started = cx.with_window(workspace.entity_id(), |window, cx| {
        workspace.update(cx, |workspace, cx| {
            workspace.start_debug_session(
                scenario,
                SharedTaskContext::from(context),
                None,
                worktree_id,
                window,
                cx,
            );
        });
    });
    if started.is_none() {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    state
        .terminal_operations
        .borrow_mut()
        .insert(request.request_id.clone(), digest);
    success(
        &request.request_id,
        json!({"outcome": "requested", "duplicate": false}),
    )
}

fn debugger_snapshot(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if !matches_instance(request, state) {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    let mut sessions = Vec::new();
    for panel in loaded_debug_panels(&state.project_root, cx) {
        let debug_sessions = panel.read(cx).sessions().collect::<Vec<_>>();
        for debug_session in debug_sessions {
            let session = debug_session.read(cx).session(cx);
            let value = session.update(cx, |session, cx| {
                let restart = session
                    .capabilities()
                    .supports_restart_request
                    .unwrap_or(false);
                let step_back = session
                    .capabilities()
                    .supports_step_back
                    .unwrap_or(false);
                let single_thread = session
                    .capabilities()
                    .supports_single_thread_execution_requests
                    .unwrap_or(false);
                let threads = session
                    .threads(cx)
                    .into_iter()
                    .map(|(thread, status)| {
                        json!({
                            "thread_id": thread.id,
                            "name": thread.name,
                            "status": status.label().to_lowercase(),
                        })
                    })
                    .collect::<Vec<_>>();
                json!({
                    "session_id": session.session_id().0.to_string(),
                    "label": session.label().map(|label| label.to_string()).unwrap_or_else(|| "Debugger".to_string()),
                    "adapter": session.adapter().to_string(),
                    "terminated": session.is_terminated(),
                    "capabilities": {
                        "restart": restart,
                        "step_back": step_back,
                        "single_thread": single_thread,
                    },
                    "threads": threads,
                })
            });
            sessions.push(value);
        }
    }
    success(
        &request.request_id,
        json!({
            "instance_id": state.instance_id,
            "epoch": state.epoch,
            "sessions": sessions,
            "provisional": true,
        }),
    )
}

fn debugger_control(request: &ProbeRequest, state: &ProbeState, cx: &mut App) -> Value {
    if !matches_instance(request, state) {
        return error(&request.request_id, "INSTANCE_UNAVAILABLE");
    }
    if !params_contain_only(
        &request.params,
        &["instance_id", "epoch", "session_id", "action", "thread_id"],
    ) {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let Some(session_id) = request.params.get("session_id").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    let Some(action) = request.params.get("action").and_then(Value::as_str) else {
        return error(&request.request_id, "INVALID_REQUEST");
    };
    if session_id.is_empty() || session_id.len() > 128 {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    if !matches!(
        action,
        "pause"
            | "continue"
            | "step_over"
            | "step_in"
            | "step_out"
            | "step_back"
            | "restart"
            | "stop"
    ) {
        return error(&request.request_id, "INVALID_REQUEST");
    }
    let digest = match payload_digest(&request.params) {
        Ok(digest) => digest,
        Err(code) => return error(&request.request_id, code),
    };
    if let Some(existing) = state.terminal_operations.borrow().get(&request.request_id) {
        return if existing == &digest {
            success(
                &request.request_id,
                json!({"outcome": "requested", "duplicate": true}),
            )
        } else {
            error(&request.request_id, "REQUEST_ID_CONFLICT")
        };
    }
    if state.terminal_operations.borrow().len() >= 256 {
        return error(&request.request_id, "RATE_LIMITED");
    }
    let debug_session = loaded_debug_panels(&state.project_root, cx)
        .into_iter()
        .flat_map(|panel| panel.read(cx).sessions().collect::<Vec<_>>())
        .find(|debug_session| {
            debug_session
                .read(cx)
                .session(cx)
                .read(cx)
                .session_id()
                .0
                .to_string()
                == session_id
        });
    let Some(debug_session) = debug_session else {
        return error(&request.request_id, "DEBUGGER_UNAVAILABLE");
    };
    let session = debug_session.read(cx).session(cx);
    if session.read(cx).is_terminated() {
        return error(&request.request_id, "DEBUGGER_UNAVAILABLE");
    }
    if action == "restart"
        && !session
            .read(cx)
            .capabilities()
            .supports_restart_request
            .unwrap_or(false)
    {
        return error(&request.request_id, "DEBUGGER_STATE_CONFLICT");
    }
    let outcome = if matches!(action, "restart" | "stop") {
        session.update(cx, |session, cx| match action {
            "restart" => {
                session.restart(None, cx);
                true
            }
            "stop" => {
                session.shutdown(cx).detach();
                true
            }
            _ => false,
        })
    } else {
        let Some(thread_id) = request.params.get("thread_id").and_then(Value::as_i64) else {
            return error(&request.request_id, "INVALID_REQUEST");
        };
        if thread_id < 0 {
            return error(&request.request_id, "INVALID_REQUEST");
        }
        session.update(cx, |session, cx| {
            let thread_id = ThreadId(thread_id);
            let threads = session.threads(cx);
            let Some((_, status)) = threads.iter().find(|(thread, _)| thread.id == thread_id.0)
            else {
                return false;
            };
            match action {
                "pause" if matches!(status, project::debugger::session::ThreadStatus::Running) => {
                    session.pause_thread(thread_id, cx);
                }
                "continue"
                    if matches!(status, project::debugger::session::ThreadStatus::Stopped) =>
                {
                    session.continue_program(thread_id, cx);
                }
                "step_over"
                    if matches!(status, project::debugger::session::ThreadStatus::Stopped) =>
                {
                    session.step_over(thread_id, SteppingGranularity::Statement, cx);
                }
                "step_in"
                    if matches!(status, project::debugger::session::ThreadStatus::Stopped) =>
                {
                    session.step_in(thread_id, SteppingGranularity::Statement, cx);
                }
                "step_out"
                    if matches!(status, project::debugger::session::ThreadStatus::Stopped) =>
                {
                    session.step_out(thread_id, SteppingGranularity::Statement, cx);
                }
                "step_back"
                    if matches!(status, project::debugger::session::ThreadStatus::Stopped)
                        && session.capabilities().supports_step_back.unwrap_or(false) =>
                {
                    session.step_back(thread_id, SteppingGranularity::Statement, cx);
                }
                _ => return false,
            }
            true
        })
    };
    if !outcome {
        return error(&request.request_id, "DEBUGGER_STATE_CONFLICT");
    }
    state
        .terminal_operations
        .borrow_mut()
        .insert(request.request_id.clone(), digest);
    success(
        &request.request_id,
        json!({"outcome": "requested", "duplicate": false}),
    )
}

fn matches_instance(request: &ProbeRequest, state: &ProbeState) -> bool {
    request.params.get("instance_id").and_then(Value::as_str) == Some(state.instance_id.as_str())
        && request.params.get("epoch").and_then(Value::as_str) == Some(state.epoch.as_str())
}

fn params_contain_only(params: &Value, allowed: &[&str]) -> bool {
    params
        .as_object()
        .is_some_and(|params| params.keys().all(|key| allowed.contains(&key.as_str())))
}

fn configured_tasks(
    project_root: &Path,
    cx: &App,
) -> Vec<(Entity<workspace::Workspace>, TaskSourceKind, ResolvedTask)> {
    let mut tasks = Vec::new();
    for workspace in loaded_project_workspaces(project_root, cx) {
        let project = workspace.read(cx).project().clone();
        let project = project.read(cx);
        let Some(worktree) = project
            .visible_worktrees(cx)
            .find(|worktree| worktree.read(cx).abs_path().as_ref() == project_root)
        else {
            continue;
        };
        let worktree_id = worktree.read(cx).id();
        let mut task_variables = TaskVariables::default();
        task_variables.insert(
            VariableName::WorktreeRoot,
            project_root.to_string_lossy().into_owned(),
        );
        let context = TaskContext {
            cwd: Some(project_root.to_path_buf()),
            task_variables,
            project_env: project.cli_environment(cx).unwrap_or_default(),
        };
        let Some(inventory) = project.task_store().read(cx).task_inventory().cloned() else {
            continue;
        };
        for (source, task) in inventory
            .read(cx)
            .configured_tasks_for_worktree(worktree_id, &context)
        {
            tasks.push((workspace.clone(), source, task));
        }
    }
    tasks.sort_by(|left, right| left.2.display_label().cmp(right.2.display_label()));
    tasks.dedup_by(|left, right| left.2.id == right.2.id);
    tasks
}

fn configured_debug_scenarios(
    project_root: &Path,
    cx: &App,
) -> Vec<(
    Entity<workspace::Workspace>,
    TaskSourceKind,
    DebugScenario,
    TaskContext,
)> {
    let mut scenarios = Vec::new();
    for workspace in loaded_project_workspaces(project_root, cx) {
        let project = workspace.read(cx).project().clone();
        let project = project.read(cx);
        let Some(worktree) = project
            .visible_worktrees(cx)
            .find(|worktree| worktree.read(cx).abs_path().as_ref() == project_root)
        else {
            continue;
        };
        let worktree_id = worktree.read(cx).id();
        let mut task_variables = TaskVariables::default();
        task_variables.insert(
            VariableName::WorktreeRoot,
            project_root.to_string_lossy().into_owned(),
        );
        let context = TaskContext {
            cwd: Some(project_root.to_path_buf()),
            task_variables,
            project_env: project.cli_environment(cx).unwrap_or_default(),
        };
        let Some(inventory) = project.task_store().read(cx).task_inventory().cloned() else {
            continue;
        };
        for (source, scenario) in inventory
            .read(cx)
            .configured_debug_scenarios_for_worktree(worktree_id)
        {
            scenarios.push((workspace.clone(), source, scenario, context.clone()));
        }
    }
    scenarios.sort_by(|left, right| left.2.label.cmp(&right.2.label));
    scenarios.dedup_by(|left, right| {
        debug_scenario_id(&left.1, &left.2) == debug_scenario_id(&right.1, &right.2)
    });
    scenarios
}

fn debug_scenario_id(source: &TaskSourceKind, scenario: &DebugScenario) -> String {
    let mut hasher = Sha256::new();
    hasher.update(task_source_label(source).as_bytes());
    hasher.update([0]);
    if let Ok(bytes) = serde_json::to_vec(scenario) {
        hasher.update(bytes);
    }
    format!("{:x}", hasher.finalize())
}

fn task_source_label(source: &TaskSourceKind) -> &'static str {
    match source {
        TaskSourceKind::Worktree { .. } => "project",
        TaskSourceKind::AbsPath { .. } => "global",
        TaskSourceKind::Language { .. } => "language",
        TaskSourceKind::Lsp { .. } => "language server",
        TaskSourceKind::UserInput => "history",
    }
}

fn loaded_debug_panels(project_root: &Path, cx: &App) -> Vec<Entity<DebugPanel>> {
    loaded_project_workspaces(project_root, cx)
        .into_iter()
        .filter_map(|workspace| workspace.read(cx).panel::<DebugPanel>(cx))
        .collect()
}

fn loaded_project_workspaces(project_root: &Path, cx: &App) -> Vec<Entity<workspace::Workspace>> {
    workspace::AppState::global(cx)
        .workspace_store
        .read(cx)
        .workspaces()
        .filter_map(|workspace| workspace.upgrade())
        .filter(|workspace| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .visible_worktrees(cx)
                .any(|worktree| worktree.read(cx).abs_path().as_ref() == project_root)
        })
        .collect()
}

fn loaded_terminal_views(project_root: &Path, cx: &App) -> Vec<Entity<TerminalView>> {
    let mut terminals = Vec::new();
    for workspace in loaded_project_workspaces(project_root, cx) {
        let Some(panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
            continue;
        };
        for pane in panel.read(cx).panes() {
            for item in pane.read(cx).items() {
                if let Some(view) = item.downcast::<TerminalView>() {
                    terminals.push(view);
                }
            }
        }
    }
    terminals.sort_by_key(|view| view.entity_id());
    terminals.dedup_by_key(|view| view.entity_id());
    terminals
}

fn terminal_id(view: &Entity<TerminalView>) -> String {
    view.entity_id().as_u64().to_string()
}

fn terminal_task_status(status: TaskStatus) -> Value {
    match status {
        TaskStatus::Unknown => json!({"state": "unknown"}),
        TaskStatus::Running => json!({"state": "running"}),
        TaskStatus::Completed { success } => json!({"state": "completed", "success": success}),
    }
}

fn terminal_value(view: &Entity<TerminalView>, cx: &App) -> Value {
    let id = terminal_id(view);
    let view = view.read(cx);
    let terminal = view.terminal().read(cx);
    let task = terminal.task().map(|task| {
        json!({
            "id": task.spawned_task.id.0,
            "label": task.spawned_task.full_label,
            "status": terminal_task_status(task.status),
        })
    });
    json!({
        "terminal_id": id,
        "title": terminal.title(false),
        "working_directory": terminal.working_directory().map(|path| path.to_string_lossy().into_owned()),
        "pid": terminal.pid().map(|pid| pid.as_u32()),
        "task": task,
    })
}

fn loaded_thread_views(
    project_root: &Path,
    cx: &App,
) -> Vec<(Entity<AcpThread>, Entity<ThreadView>)> {
    let workspaces = workspace::AppState::global(cx)
        .workspace_store
        .read(cx)
        .workspaces()
        .cloned()
        .collect::<Vec<_>>();
    let mut threads = Vec::new();
    for workspace in workspaces {
        let Some(workspace) = workspace.upgrade() else {
            continue;
        };
        let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
            continue;
        };
        for conversation in panel.read(cx).conversation_views() {
            let conversation = conversation.read(cx);
            let Some(view) = conversation.root_thread_view() else {
                continue;
            };
            let thread = view.read(cx).thread.clone();
            if thread_belongs_to_project(&thread, project_root, cx) {
                threads.push((thread, view));
            }
        }
    }
    threads
}

fn thread_belongs_to_project(thread: &Entity<AcpThread>, project_root: &Path, cx: &App) -> bool {
    let project = thread.read(cx).project().clone();
    project
        .read(cx)
        .visible_worktrees(cx)
        .any(|worktree| worktree.read(cx).abs_path().as_ref() == project_root)
}

fn success(request_id: &str, result: Value) -> Value {
    json!({"version": 1, "request_id": request_id, "result": result})
}

fn error(request_id: &str, code: &str) -> Value {
    json!({"version": 1, "request_id": request_id, "error": {"code": code}})
}
