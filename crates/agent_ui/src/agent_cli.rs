use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use acp_thread::{AcpThread, ThreadStatus};
use agent_client_protocol::schema::v1 as acp;
use agent_settings::{AgentProfileId, AgentSettings};
use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Utc};
use collections::HashSet;
use gpui::{App, AsyncApp, Entity, Task, WindowHandle};
use language_model::LanguageModelRegistry;
use remote::RemoteConnectionOptions;
use settings::Settings as _;
use workspace::{AppState, MultiWorkspace, PathList, SerializedWorkspaceLocation, Workspace};

use crate::{
    Agent, AgentInitialContent, AgentPanel, AgentThreadSource, ExternalSourcePrompt, ThreadId,
    agent_panel::CreateThreadOptions,
    conversation_view::ThreadView,
    thread_metadata_store::{ThreadMetadata, ThreadMetadataStore},
};

/// How the caller identified the thread to target.
pub enum ThreadSelector {
    /// Exact `ThreadId`.
    Id(ThreadId),
    /// Case-insensitive prefix of a `ThreadId`, compared with hyphens ignored
    /// so that `550e8400-e29b` and `550e8400e29b` behave identically.
    Prefix(String),
    /// An ACP session id.
    Session(String),
    /// Most recently updated non-archived thread for the project.
    MostRecent,
    /// Always create a fresh thread.
    New,
}

impl ThreadSelector {
    /// Interprets a user-supplied thread argument.
    ///
    /// A value that parses as a complete UUID is an exact id; anything else is
    /// treated as a prefix. `uuid` accepts several complete forms (hyphenated,
    /// simple, URN, braced), so this deliberately does not assume hyphens.
    pub fn from_thread_argument(value: &str) -> Self {
        match value.parse::<ThreadId>() {
            Ok(thread_id) => Self::Id(thread_id),
            Err(_) => Self::Prefix(value.to_string()),
        }
    }
}

/// Lower-cases and strips hyphens so that thread id prefixes can be given with
/// or without the hyphens of the canonical form.
fn normalize_thread_id_fragment(fragment: &str) -> String {
    fragment
        .chars()
        .filter(|character| *character != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

pub struct CliPromptRequest {
    pub selector: ThreadSelector,
    pub prompt: String,
    /// Absolute path scoping lookup/creation. `None` means "any".
    pub project: Option<PathBuf>,
    /// Agent profile for a thread being created, which decides the available
    /// tools. Only honored alongside [`ThreadSelector::New`].
    pub profile: Option<String>,
    /// Model for a thread being created, as `provider/model-id`. Only honored
    /// alongside [`ThreadSelector::New`].
    pub model: Option<String>,
    pub wait: bool,
}

/// Profile and model applied to a thread being created, validated up front.
struct TurnSettings {
    profile: Option<AgentProfileId>,
    model: Option<String>,
}

/// One row of `--agent-list` output.
pub struct ThreadListEntry {
    pub thread_id: ThreadId,
    pub title: String,
    pub updated_at: DateTime<Utc>,
    /// When a person last engaged with the thread from the panel, as opposed
    /// to `updated_at`, which also moves when the agent makes progress.
    /// Prompts delivered by this module deliberately leave it alone.
    pub interacted_at: Option<DateTime<Utc>>,
    pub is_open: bool,
    /// The worktree folders the thread belongs to. For a thread created in a
    /// linked git worktree these are that worktree's paths, not the main
    /// checkout's.
    pub paths: Vec<PathBuf>,
}

/// What happened to a prompt that was dispatched successfully.
pub enum DispatchOutcome {
    /// Prompt was sent, either into an idle thread or auto-submitted on a
    /// freshly created one.
    Sent { thread_id: ThreadId },
    /// Thread was generating; prompt was appended to the message queue.
    Queued { thread_id: ThreadId },
}

/// Lists threads, most-recently-updated first. When `project` is `Some`,
/// only threads whose worktree paths match are returned.
pub fn list_threads(project: Option<&Path>, cx: &App) -> Vec<ThreadListEntry> {
    let store = ThreadMetadataStore::global(cx);
    let store = store.read(cx);

    let open_thread_ids = scan_open_thread_ids(cx);

    let mut entries: Vec<ThreadListEntry> = store
        .entries()
        .filter(|metadata| !metadata.archived)
        .filter(|metadata| {
            if let Some(project_path) = project {
                thread_matches_path(metadata, project_path)
            } else {
                true
            }
        })
        .map(|metadata| ThreadListEntry {
            thread_id: metadata.thread_id,
            title: metadata.display_title().to_string(),
            updated_at: metadata.updated_at,
            interacted_at: metadata.interacted_at,
            is_open: open_thread_ids.contains(&metadata.thread_id),
            paths: metadata
                .folder_paths()
                .paths()
                .iter()
                .map(|p| p.as_path().to_path_buf())
                .collect(),
        })
        .collect();

    entries.sort_by_key(|entry| std::cmp::Reverse(entry.updated_at));
    entries
}

/// Resolves the selector and dispatches the prompt. When `wait` is true, the
/// returned task additionally waits for the turn to complete.
pub fn dispatch_cli_prompt(
    request: CliPromptRequest,
    app_state: Arc<AppState>,
    cx: &mut AsyncApp,
) -> Task<Result<DispatchOutcome>> {
    cx.spawn(async move |mut cx| {
        let CliPromptRequest {
            selector,
            prompt,
            project: project_filter,
            profile,
            model,
            wait,
        } = request;

        // Prompts are frequently piped in from webhooks, so strip bidi and
        // control characters that could misrepresent what is being sent.
        let prompt = ExternalSourcePrompt::new(&prompt)
            .context("prompt contained no usable text")?
            .into_string();
        let blocks = vec![acp::ContentBlock::Text(acp::TextContent::new(prompt))];

        // Validated before anything is dispatched, so a typo can't leave a
        // thread running under the wrong tool set.
        let turn = cx.update(|cx| {
            let profile = profile
                .as_deref()
                .map(|profile| validate_profile(profile, cx))
                .transpose()?;
            if let Some(model) = model.as_deref() {
                validate_model(model, cx)?;
            }
            anyhow::Ok(TurnSettings { profile, model })
        })?;

        match selector {
            ThreadSelector::New => {
                dispatch_into_new_thread(blocks, wait, app_state, project_filter, turn, &mut cx)
                    .await
            }
            _ => {
                // Both settings persist on the thread, and switching the
                // profile also selects that profile's preferred model and
                // applies to running subagents, so they configure a thread
                // being created rather than reconfigure an existing one.
                anyhow::ensure!(
                    turn.profile.is_none() && turn.model.is_none(),
                    "a profile or model can only be set when creating a thread"
                );

                let resolved_id = resolve_thread_id(&selector, project_filter.as_deref(), &cx)?;

                // Dispatching into a thread that is already open avoids both a
                // reload from disk and any disturbance to the user's focus.
                let open_result = find_open_thread_window(resolved_id, &mut cx);
                if let Some(window_handle) = open_result {
                    return dispatch_into_open_window(
                        window_handle,
                        resolved_id,
                        blocks,
                        wait,
                        &mut cx,
                    )
                    .await;
                }

                dispatch_into_stored_thread(resolved_id, blocks, wait, app_state, &mut cx).await
            }
        }
    })
}

/// Waits for `window` to register an [`AgentPanel`] in any of its workspaces.
///
/// A workspace registers its docks asynchronously after it opens, so a request
/// that arrives during startup would otherwise find no panel at all. Polling
/// rather than awaiting `Workspace::take_panels_task` is deliberate: that task
/// can only be taken once, so whichever caller got there first would leave
/// everyone else believing initialization had finished.
async fn wait_for_agent_panel(window: &WindowHandle<MultiWorkspace>, cx: &mut AsyncApp) {
    const PANEL_LOAD_TIMEOUT: Duration = Duration::from_secs(30);
    const POLL_INTERVAL: Duration = Duration::from_millis(50);

    let deadline = std::time::Instant::now() + PANEL_LOAD_TIMEOUT;
    loop {
        let registered = window
            .update(cx, |multi, _window, cx| {
                multi
                    .workspaces()
                    .any(|workspace| workspace.read(cx).panel::<AgentPanel>(cx).is_some())
            })
            .unwrap_or(true);
        if registered || std::time::Instant::now() >= deadline {
            return;
        }
        cx.background_executor().timer(POLL_INTERVAL).await;
    }
}

/// Collects all `ThreadId`s currently open in any workspace's `AgentPanel`.
fn scan_open_thread_ids(cx: &App) -> HashSet<ThreadId> {
    let mut open_ids = HashSet::default();
    for window in cx.windows() {
        let Some(multi_workspace) = window.downcast::<MultiWorkspace>() else {
            continue;
        };
        let Ok(multi) = multi_workspace.read(cx) else {
            continue;
        };
        for workspace in multi.workspaces() {
            let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
                continue;
            };
            let panel = panel.read(cx);
            for thread_id in panel.retained_threads().keys() {
                open_ids.insert(*thread_id);
            }
            if let Some(active_id) = panel.active_thread_id(cx) {
                open_ids.insert(active_id);
            }
            if let Some(draft_id) = panel.ephemeral_draft_thread_id(cx) {
                open_ids.insert(draft_id);
            }
        }
    }
    open_ids
}

/// Find the `WindowHandle<MultiWorkspace>` that contains an already-open
/// thread. Returns `None` if the thread isn't currently open.
fn find_open_thread_window(
    thread_id: ThreadId,
    cx: &mut AsyncApp,
) -> Option<WindowHandle<MultiWorkspace>> {
    let windows = cx.update(|cx| cx.windows());
    for window in windows {
        let Some(multi_workspace) = window.downcast::<MultiWorkspace>() else {
            continue;
        };
        let check_result = multi_workspace.update(cx, |multi, _window, cx| {
            for workspace in multi.workspaces() {
                let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
                    continue;
                };
                let panel = panel.read(cx);
                if panel.conversation_view_for_id(&thread_id, cx).is_some() {
                    return true;
                }
            }
            false
        });
        if check_result.is_ok_and(|found| found) {
            return Some(multi_workspace);
        }
    }
    None
}

/// Dispatch the prompt into a thread that is open in some window.
async fn dispatch_into_open_window(
    window_handle: WindowHandle<MultiWorkspace>,
    thread_id: ThreadId,
    blocks: Vec<acp::ContentBlock>,
    wait: bool,
    cx: &mut AsyncApp,
) -> Result<DispatchOutcome> {
    // Perform the send or queue operation inside `window_handle.update`
    // because both `send_content` and `add_to_queue` require `&mut Window`.
    let outcome = window_handle.update(cx, |multi, window, cx| {
        let Some((thread_view, acp_thread)) = find_thread_view_in_workspaces(multi, thread_id, cx)
        else {
            return Err(anyhow!("thread not found in window after initial lookup"));
        };

        let is_generating = acp_thread.read(cx).status() != ThreadStatus::Idle;

        if is_generating {
            thread_view.update(cx, |view, cx| {
                view.add_to_queue(blocks, Vec::new(), window, cx);
            });
            Ok(DispatchOutcome::Queued { thread_id })
        } else {
            // A queue left in the paused state by a manual stop would never
            // drain on its own, because no further `Stop` event is coming.
            thread_view.update(cx, |view, cx| {
                view.resume_message_queue();
                view.send_content(
                    Task::ready(Ok(Some((blocks, Vec::new())))),
                    false,
                    window,
                    cx,
                );
            });
            Ok(DispatchOutcome::Sent { thread_id })
        }
    })??;

    if wait {
        wait_for_idle_and_empty_queue_in_window(&window_handle, thread_id, cx).await;
    }

    Ok(outcome)
}

/// Checks a profile name against the configured profiles.
///
/// Automation depends on the tool set a profile implies, so an unknown name is
/// an error rather than a silent fallback to the default.
fn validate_profile(profile: &str, cx: &App) -> Result<AgentProfileId> {
    let profile_id = AgentProfileId(profile.into());
    let settings = AgentSettings::get_global(cx);
    if settings.profiles.contains_key(&profile_id) {
        return Ok(profile_id);
    }
    let mut available = settings
        .profiles
        .keys()
        .map(|id| id.as_str())
        .collect::<Vec<_>>();
    available.sort_unstable();
    Err(anyhow!(
        "unknown agent profile \"{profile}\"; available profiles: {}",
        available.join(", ")
    ))
}

/// Checks a `provider/model-id` string against the registered models.
fn validate_model(model: &str, cx: &mut App) -> Result<()> {
    let selected = crate::agent_panel::parse_provider_slash_model(model).with_context(|| {
        format!("could not parse model \"{model}\"; expected `provider/model-id`")
    })?;
    LanguageModelRegistry::global(cx)
        .update(cx, |registry, cx| registry.select_model(&selected, cx))
        .with_context(|| format!("no configured model matches \"{model}\""))?;
    Ok(())
}

/// The agent panel for `preferred`, falling back to any panel in the window.
///
/// The window can come from the "any active workspace" fallback, in which case
/// no workspace in it matches the thread but its panel is still the right place
/// to put the conversation.
fn agent_panel_for_window(
    multi_workspace: &MultiWorkspace,
    preferred: Option<&Entity<Workspace>>,
    cx: &App,
) -> Result<Entity<AgentPanel>> {
    preferred
        .and_then(|workspace| workspace.read(cx).panel::<AgentPanel>(cx))
        .or_else(|| {
            multi_workspace
                .workspaces()
                .find_map(|workspace| workspace.read(cx).panel::<AgentPanel>(cx))
        })
        .ok_or_else(|| anyhow!("no agent panel available (is `disable_ai` set?)"))
}

/// Whether `workspace` is the one a thread with these worktree paths belongs to.
fn workspace_hosts_thread(
    workspace: &Entity<Workspace>,
    host: Option<&RemoteConnectionOptions>,
    folder_paths: &PathList,
    cx: &App,
) -> bool {
    let workspace = workspace.read(cx);
    if workspace.project_group_key(cx).host().as_ref() != host {
        return false;
    }
    let root_paths = PathList::new(&workspace.root_paths(cx));
    folder_paths.paths().iter().any(|folder_path| {
        root_paths
            .paths()
            .iter()
            .any(|root_path| folder_path.as_path().starts_with(root_path.as_path()))
    })
}

/// Finds a `ThreadView` and its `AcpThread` among a window's workspaces.
///
/// This takes `&MultiWorkspace` rather than looking the window up by handle
/// because callers run inside `WindowHandle::update`, which temporarily removes
/// the window from the app; reading it by handle from there always fails.
fn find_thread_view_in_workspaces(
    multi_workspace: &MultiWorkspace,
    thread_id: ThreadId,
    cx: &App,
) -> Option<(Entity<ThreadView>, Entity<AcpThread>)> {
    for workspace in multi_workspace.workspaces() {
        let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
            continue;
        };
        let Some(conversation_view) = panel
            .read(cx)
            .conversation_view_for_id(&thread_id, cx)
            .cloned()
        else {
            continue;
        };
        let Some(thread_view) = conversation_view.read(cx).root_thread_view() else {
            continue;
        };
        let acp_thread = thread_view.read(cx).thread.clone();
        return Some((thread_view, acp_thread));
    }
    None
}

/// Wait for the AcpThread to become Idle AND for its message queue to drain.
/// There is deliberately no timeout.
async fn wait_for_idle_and_empty_queue_in_window(
    window_handle: &WindowHandle<MultiWorkspace>,
    thread_id: ThreadId,
    cx: &mut AsyncApp,
) {
    loop {
        // A closed window or a vanished thread leaves nothing to wait for.
        let done = window_handle
            .update(cx, |multi, _window, cx| {
                let Some((thread_view, acp_thread)) =
                    find_thread_view_in_workspaces(multi, thread_id, cx)
                else {
                    return true;
                };
                let is_idle = acp_thread.read(cx).status() == ThreadStatus::Idle;
                is_idle && thread_view.read(cx).is_message_queue_empty()
            })
            .unwrap_or(true);

        if done {
            return;
        }

        cx.background_executor()
            .timer(Duration::from_millis(200))
            .await;
    }
}

/// Load a thread from the metadata store and dispatch into it.
async fn dispatch_into_stored_thread(
    thread_id: ThreadId,
    blocks: Vec<acp::ContentBlock>,
    wait: bool,
    app_state: Arc<AppState>,
    cx: &mut AsyncApp,
) -> Result<DispatchOutcome> {
    // The thread's own stored worktree paths decide where it reopens, so any
    // caller-supplied project filter has already done its job during lookup.
    let metadata = cx.update(|cx| {
        let store = ThreadMetadataStore::global(cx);
        let store = store.read(cx);
        store
            .entry(thread_id)
            .cloned()
            .ok_or_else(|| anyhow!("thread \"{thread_id}\" not found in metadata store"))
    })?;

    let location = metadata
        .remote_connection
        .as_ref()
        .map(|conn| SerializedWorkspaceLocation::Remote(conn.clone()))
        .unwrap_or(SerializedWorkspaceLocation::Local);

    let folder_paths = metadata.folder_paths().clone();
    let host = metadata.remote_connection.as_ref();
    let agent: Agent = metadata.agent_id.clone().into();
    let work_dirs = Some(folder_paths.clone());
    let title = metadata.title();

    let candidate_windows =
        cx.update(|cx| workspace::workspace_windows_for_location(&location, cx));

    let mut found_window: Option<WindowHandle<MultiWorkspace>> = None;
    for multi_window in &candidate_windows {
        let matches = multi_window
            .update(cx, |multi, _window, cx| {
                multi
                    .workspaces()
                    .any(|workspace| workspace_hosts_thread(workspace, host, &folder_paths, cx))
            })
            .unwrap_or(false);
        if matches {
            found_window = Some(*multi_window);
            break;
        }
    }

    let window_handle = match found_window {
        Some(window) => window,
        None => workspace::get_any_active_multi_workspace(app_state.clone(), cx.clone())
            .await
            .context("no workspace available to load thread into")?,
    };

    wait_for_agent_panel(&window_handle, cx).await;

    window_handle.update(cx, |multi, window, cx| {
        let preferred = multi
            .workspaces()
            .find(|workspace| workspace_hosts_thread(workspace, host, &folder_paths, cx))
            .cloned();
        let panel = agent_panel_for_window(multi, preferred.as_ref(), cx)?;

        panel.update(cx, |panel, cx| {
            panel.load_agent_thread(
                agent.clone(),
                thread_id,
                work_dirs.clone(),
                title.clone(),
                false,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        anyhow::Ok(())
    })??;

    wait_for_thread_view_and_dispatch(window_handle, thread_id, blocks, wait, cx).await
}

/// After `load_agent_thread`, poll until the ThreadView is ready
/// (the agent server connection is asynchronous) and then dispatch.
async fn wait_for_thread_view_and_dispatch(
    window_handle: WindowHandle<MultiWorkspace>,
    thread_id: ThreadId,
    blocks: Vec<acp::ContentBlock>,
    wait: bool,
    cx: &mut AsyncApp,
) -> Result<DispatchOutcome> {
    const THREAD_LOAD_TIMEOUT: Duration = Duration::from_secs(60);
    const POLL_INTERVAL: Duration = Duration::from_millis(100);

    let deadline = std::time::Instant::now() + THREAD_LOAD_TIMEOUT;
    loop {
        // A failed update means the window was closed while we were waiting;
        // propagate rather than polling a handle that can never become ready.
        let ready = window_handle
            .update(cx, |multi, _window, cx| {
                find_thread_view_in_workspaces(multi, thread_id, cx).is_some()
            })
            .with_context(|| {
                format!("window closed while waiting for thread {thread_id} to load")
            })?;

        if ready {
            return dispatch_into_open_window(window_handle, thread_id, blocks, wait, cx).await;
        }
        anyhow::ensure!(
            std::time::Instant::now() < deadline,
            "timed out waiting for thread {thread_id} to finish loading"
        );

        cx.background_executor().timer(POLL_INTERVAL).await;
    }
}

/// Create a new thread and dispatch into it.
async fn dispatch_into_new_thread(
    blocks: Vec<acp::ContentBlock>,
    wait: bool,
    app_state: Arc<AppState>,
    project_filter: Option<PathBuf>,
    turn: TurnSettings,
    cx: &mut AsyncApp,
) -> Result<DispatchOutcome> {
    let initial_content = AgentInitialContent::ContentBlock {
        blocks,
        auto_submit: true,
    };

    // Remembering which workspace matched keeps the thread in the requested
    // project; a window can host several workspaces, each with its own panel.
    let mut matched_workspace = None;
    let window_handle = if let Some(ref project_path) = project_filter {
        let path_list = PathList::new(std::slice::from_ref(project_path));
        let location = SerializedWorkspaceLocation::Local;

        let candidate_windows =
            cx.update(|cx| workspace::workspace_windows_for_location(&location, cx));

        let mut found_window = None;
        for multi_window in &candidate_windows {
            let workspace = multi_window
                .update(cx, |multi, _window, cx| {
                    multi.workspace_for_paths(&path_list, None, cx)
                })
                .ok()
                .flatten();
            if let Some(workspace) = workspace {
                matched_workspace = Some(workspace);
                found_window = Some(*multi_window);
                break;
            }
        }

        // Falling back to whichever window happens to be active would put the
        // thread in the wrong project while still reporting success.
        found_window.with_context(|| {
            format!(
                "{} is not open in Zed; open it first, or omit --agent-project \
                 to use the active window",
                project_path.display()
            )
        })?
    } else {
        workspace::get_any_active_multi_workspace(app_state.clone(), cx.clone())
            .await
            .context("no workspace available to create thread in")?
    };

    wait_for_agent_panel(&window_handle, cx).await;

    let thread_id = window_handle.update(cx, |multi, window, cx| {
        let panel = agent_panel_for_window(multi, matched_workspace.as_ref(), cx)?;

        let result = panel.update(cx, |panel, cx| {
            panel.create_thread_with_options(
                CreateThreadOptions {
                    initial_content: Some(initial_content),
                    model: turn.model.clone(),
                    profile: turn.profile.clone(),
                    work_dirs: project_filter
                        .as_ref()
                        .map(|project_path| PathList::new(std::slice::from_ref(project_path))),
                    ..Default::default()
                },
                AgentThreadSource::AgentPanel,
                window,
                cx,
            )
        });
        anyhow::Ok(result)
    })??;

    if wait {
        // The thread was created with `auto_submit`, so waiting for it to go
        // idle also covers the automatic first turn.
        wait_for_idle_and_empty_queue_in_window(&window_handle, thread_id, cx).await;
    }

    Ok(DispatchOutcome::Sent { thread_id })
}

/// Resolve a `ThreadSelector` to a `ThreadId` by consulting the metadata store.
fn resolve_thread_id(
    selector: &ThreadSelector,
    project_filter: Option<&Path>,
    cx: &AsyncApp,
) -> Result<ThreadId> {
    cx.update(|cx| {
        let store = ThreadMetadataStore::global(cx);
        let store = store.read(cx);

        match selector {
            ThreadSelector::New => Err(anyhow!(
                "ThreadSelector::New should be handled before resolve_thread_id"
            )),
            ThreadSelector::Id(thread_id) => {
                if store.entry(*thread_id).is_none() {
                    return Err(anyhow!("thread \"{thread_id}\" not found"));
                }
                Ok(*thread_id)
            }
            ThreadSelector::Prefix(prefix) => {
                let candidates = selectable_threads(&store, project_filter);
                select_by_prefix(&candidates, prefix)
            }
            ThreadSelector::Session(session_id) => {
                let session_id = acp::SessionId::new(session_id.clone());
                if let Some(metadata) = store.entry_by_session(&session_id) {
                    Ok(metadata.thread_id)
                } else {
                    Err(anyhow!("no thread found for session \"{}\"", session_id))
                }
            }
            ThreadSelector::MostRecent => {
                let candidates = selectable_threads(&store, project_filter);
                select_most_recent(&candidates, &scan_open_thread_ids(cx))
            }
        }
    })
}

/// Non-archived threads belonging to `project_filter`, if one was given.
fn selectable_threads<'a>(
    store: &'a ThreadMetadataStore,
    project_filter: Option<&Path>,
) -> Vec<&'a ThreadMetadata> {
    store
        .entries()
        .filter(|metadata| !metadata.archived)
        .filter(|metadata| {
            project_filter.is_none_or(|project_path| thread_matches_path(metadata, project_path))
        })
        .collect()
}

fn select_by_prefix(candidates: &[&ThreadMetadata], prefix: &str) -> Result<ThreadId> {
    let normalized_prefix = normalize_thread_id_fragment(prefix);
    // An empty prefix matches every thread, so it would silently resolve
    // whenever exactly one thread exists.
    if normalized_prefix.is_empty() {
        return Err(anyhow!("thread id prefix must not be empty"));
    }

    let mut matches: Vec<&ThreadMetadata> = candidates
        .iter()
        .copied()
        .filter(|metadata| {
            normalize_thread_id_fragment(&metadata.thread_id.to_string())
                .starts_with(&normalized_prefix)
        })
        .collect();
    matches.sort_by_key(|metadata| std::cmp::Reverse(metadata.updated_at));

    match matches.as_slice() {
        [] => Err(anyhow!("no thread found matching prefix \"{prefix}\"")),
        [only] => Ok(only.thread_id),
        ambiguous => {
            let listed = ambiguous
                .iter()
                .map(|metadata| {
                    format!("  {} \"{}\"", metadata.thread_id, metadata.display_title())
                })
                .collect::<Vec<_>>()
                .join("\n");
            Err(anyhow!(
                "ambiguous prefix \"{prefix}\" matches {} threads:\n{listed}",
                ambiguous.len()
            ))
        }
    }
}

fn select_most_recent(
    candidates: &[&ThreadMetadata],
    open_thread_ids: &HashSet<ThreadId>,
) -> Result<ThreadId> {
    candidates
        .iter()
        .max_by_key(|metadata| {
            // Prefer an already-open thread when timestamps tie.
            (
                metadata.updated_at,
                open_thread_ids.contains(&metadata.thread_id),
            )
        })
        .map(|metadata| metadata.thread_id)
        .ok_or_else(|| anyhow!("no threads found"))
}

/// Returns `true` when the thread's worktree paths intersect with the
/// given project path.
///
/// The comparison is containment in either direction because `project_path`
/// usually defaults to the caller's working directory, which is commonly a
/// subdirectory of the worktree root the thread was created against.
fn thread_matches_path(metadata: &ThreadMetadata, project_path: &Path) -> bool {
    metadata
        .folder_paths()
        .paths()
        .iter()
        .chain(metadata.main_worktree_paths().paths().iter())
        .any(|thread_path| {
            let thread_path = thread_path.as_path();
            thread_path.starts_with(project_path) || project_path.starts_with(thread_path)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use project::WorktreePaths;

    fn metadata_for_paths(paths: &[&str]) -> ThreadMetadata {
        let path_list = PathList::new(&paths.iter().map(PathBuf::from).collect::<Vec<_>>());
        ThreadMetadata {
            thread_id: ThreadId::new(),
            session_id: None,
            agent_id: agent::ZED_AGENT_ID.clone(),
            title: None,
            title_override: None,
            updated_at: Utc::now(),
            created_at: None,
            interacted_at: None,
            worktree_paths: WorktreePaths::from_folder_paths(&path_list),
            remote_connection: None,
            archived: false,
        }
    }

    fn metadata_with_id(thread_id: ThreadId, updated_at: DateTime<Utc>) -> ThreadMetadata {
        ThreadMetadata {
            thread_id,
            updated_at,
            ..metadata_for_paths(&["/home/user/project"])
        }
    }

    #[test]
    fn test_prompts_are_stripped_of_invisible_characters() {
        // Webhook payloads are attacker-influenced; a bidi override could
        // otherwise make the rendered prompt disagree with what is sent.
        let sanitized = ExternalSourcePrompt::new("delete\u{202E}everything")
            .expect("prompt should survive sanitization")
            .into_string();
        assert_eq!(sanitized, "deleteeverything");
    }

    #[test]
    fn test_prompts_of_only_invisible_characters_are_rejected() {
        assert!(ExternalSourcePrompt::new("\u{202E}\u{0000}").is_none());
    }

    #[test]
    fn test_prefixes_select_a_single_matching_thread() {
        let wanted = ThreadId::new();
        let other = ThreadId::new();
        let entries = [
            metadata_with_id(wanted, Utc::now()),
            metadata_with_id(other, Utc::now()),
        ];
        let candidates: Vec<&ThreadMetadata> = entries.iter().collect();

        let fragment = &wanted.to_string()[..8];
        assert_eq!(
            select_by_prefix(&candidates, fragment).expect("prefix should resolve"),
            wanted
        );
    }

    #[test]
    fn test_prefixes_match_regardless_of_hyphens_and_case() {
        let wanted = ThreadId::new();
        let entries = [metadata_with_id(wanted, Utc::now())];
        let candidates: Vec<&ThreadMetadata> = entries.iter().collect();

        // The first two hyphen-delimited groups, with and without the hyphen.
        let rendered = wanted.to_string();
        let hyphenated = &rendered[..13];
        let hyphenless = hyphenated.replace('-', "");

        for fragment in [
            hyphenated.to_string(),
            hyphenless,
            hyphenated.to_uppercase(),
        ] {
            assert_eq!(
                select_by_prefix(&candidates, &fragment)
                    .unwrap_or_else(|_| panic!("{fragment} should resolve")),
                wanted
            );
        }
    }

    #[test]
    fn test_ambiguous_prefixes_report_every_candidate() {
        // Both ids share a leading `0`, so a one-character prefix is ambiguous.
        let first: ThreadId = "0aaaaaaa-0000-4000-8000-000000000000"
            .parse()
            .expect("valid uuid");
        let second: ThreadId = "0bbbbbbb-0000-4000-8000-000000000000"
            .parse()
            .expect("valid uuid");
        let entries = [
            metadata_with_id(first, Utc::now()),
            metadata_with_id(second, Utc::now()),
        ];
        let candidates: Vec<&ThreadMetadata> = entries.iter().collect();

        let error = select_by_prefix(&candidates, "0")
            .expect_err("an ambiguous prefix should not resolve")
            .to_string();
        assert!(error.contains(&first.to_string()), "{error}");
        assert!(error.contains(&second.to_string()), "{error}");
    }

    #[test]
    fn test_unmatched_prefixes_are_rejected() {
        let entries = [metadata_with_id(ThreadId::new(), Utc::now())];
        let candidates: Vec<&ThreadMetadata> = entries.iter().collect();

        assert!(select_by_prefix(&candidates, "ffffffff").is_err());
    }

    #[test]
    fn test_hyphen_only_prefixes_are_rejected() {
        // Normalization strips hyphens, so `-` must not become a match-all.
        let entries = [metadata_with_id(ThreadId::new(), Utc::now())];
        let candidates: Vec<&ThreadMetadata> = entries.iter().collect();

        assert!(select_by_prefix(&candidates, "-").is_err());
        assert!(select_by_prefix(&candidates, "").is_err());
    }

    #[test]
    fn test_most_recent_selects_the_newest_thread() {
        let older = ThreadId::new();
        let newer = ThreadId::new();
        let now = Utc::now();
        let entries = [
            metadata_with_id(older, now - chrono::Duration::hours(2)),
            metadata_with_id(newer, now),
        ];
        let candidates: Vec<&ThreadMetadata> = entries.iter().collect();

        assert_eq!(
            select_most_recent(&candidates, &HashSet::default()).expect("should resolve"),
            newer
        );
    }

    #[test]
    fn test_most_recent_prefers_an_open_thread_when_timestamps_tie() {
        let closed = ThreadId::new();
        let open = ThreadId::new();
        let now = Utc::now();
        let entries = [metadata_with_id(closed, now), metadata_with_id(open, now)];
        let candidates: Vec<&ThreadMetadata> = entries.iter().collect();

        let mut open_thread_ids = HashSet::default();
        open_thread_ids.insert(open);

        assert_eq!(
            select_most_recent(&candidates, &open_thread_ids).expect("should resolve"),
            open
        );
    }

    #[test]
    fn test_most_recent_reports_when_no_threads_exist() {
        assert!(select_most_recent(&[], &HashSet::default()).is_err());
    }

    #[test]
    fn test_thread_id_round_trips_through_string() {
        let thread_id = ThreadId::new();
        let parsed: ThreadId = thread_id
            .to_string()
            .parse()
            .expect("a rendered thread id should parse back");
        assert_eq!(thread_id, parsed);
    }

    #[test]
    fn test_thread_id_parses_hyphenless_form() {
        let thread_id = ThreadId::new();
        let hyphenless = thread_id.to_string().replace('-', "");
        let parsed: ThreadId = hyphenless
            .parse()
            .expect("the hyphenless form should also parse");
        assert_eq!(thread_id, parsed);
    }

    #[test]
    fn test_complete_thread_ids_select_by_exact_id() {
        let thread_id = ThreadId::new();

        for form in [
            thread_id.to_string(),
            thread_id.to_string().replace('-', ""),
            thread_id.to_string().to_uppercase(),
        ] {
            match ThreadSelector::from_thread_argument(&form) {
                ThreadSelector::Id(parsed) => assert_eq!(parsed, thread_id),
                _ => panic!("{form} should select by exact id"),
            }
        }
    }

    #[test]
    fn test_partial_thread_ids_select_by_prefix() {
        let thread_id = ThreadId::new().to_string();
        let fragment = &thread_id[..8];

        match ThreadSelector::from_thread_argument(fragment) {
            ThreadSelector::Prefix(prefix) => assert_eq!(prefix, fragment),
            _ => panic!("a partial id should select by prefix"),
        }
    }

    #[test]
    fn test_thread_id_fragments_normalize_case_and_hyphens() {
        assert_eq!(
            normalize_thread_id_fragment("550E8400-E29B"),
            normalize_thread_id_fragment("550e8400e29b")
        );
    }

    #[test]
    fn test_empty_thread_id_fragments_normalize_to_empty() {
        // Guards the check that stops a bare `-` from matching every thread.
        assert!(normalize_thread_id_fragment("-").is_empty());
        assert!(normalize_thread_id_fragment("").is_empty());
    }

    #[test]
    fn test_threads_match_their_own_worktree_root() {
        let metadata = metadata_for_paths(&["/home/user/project"]);
        assert!(thread_matches_path(
            &metadata,
            Path::new("/home/user/project")
        ));
    }

    #[test]
    fn test_threads_match_when_invoked_from_a_subdirectory() {
        // `--agent-project` defaults to the shell's working directory, which is
        // usually below the worktree root the thread was created against.
        let metadata = metadata_for_paths(&["/home/user/project"]);
        assert!(thread_matches_path(
            &metadata,
            Path::new("/home/user/project/crates/agent_ui")
        ));
    }

    #[test]
    fn test_threads_match_when_scoped_to_a_parent_directory() {
        let metadata = metadata_for_paths(&["/home/user/project/crates"]);
        assert!(thread_matches_path(
            &metadata,
            Path::new("/home/user/project")
        ));
    }

    #[test]
    fn test_threads_do_not_match_sibling_projects() {
        let metadata = metadata_for_paths(&["/home/user/project"]);
        assert!(!thread_matches_path(
            &metadata,
            Path::new("/home/user/other")
        ));
    }

    #[test]
    fn test_threads_do_not_match_partial_path_components() {
        // `starts_with` compares whole components, so `project-two` must not
        // match a thread rooted at `project`.
        let metadata = metadata_for_paths(&["/home/user/project"]);
        assert!(!thread_matches_path(
            &metadata,
            Path::new("/home/user/project-two")
        ));
    }
}
