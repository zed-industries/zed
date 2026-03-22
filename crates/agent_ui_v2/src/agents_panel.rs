use acp_thread::{AgentSessionInfo, AgentSessionList};
use agent::{CLI_PROJECT_PATH_KEY, CLI_SOURCE_KEY, ClaudeCodeSessionIndex, ClaudeCodeSessionList, CodexSessionIndex, CodexSessionList, NativeAgentServer, ThreadStore};
use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use agent_settings::AgentSettings;
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt};
use fs::Fs;
use gpui::{
    Action, AsyncWindowContext, Entity, EventEmitter, Focusable, Pixels, Subscription, Task,
    WeakEntity, actions, prelude::*,
};
use project::Project;
use prompt_store::PromptStore;
use serde::{Deserialize, Serialize};
use settings::{Settings as _, update_settings_file};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use ui::{App, Context, IconName, IntoElement, ParentElement, Render, SharedString, Styled, Window};
use terminal_view::terminal_panel::TerminalPanel;
use util::ResultExt;
use workspace::{
    Panel, Workspace,
    dock::{ClosePane, DockPosition, PanelEvent, UtilityPane},
    utility_pane::{UtilityPaneSlot, utility_slot_for_dock_position},
};

use crate::agent_thread_pane::{
    AgentThreadPane, AgentsUtilityPaneEvent, SerializedAgentThreadPane, SerializedHistoryEntryId,
};
use crate::thread_history::{AcpThreadHistory, ThreadHistoryEvent};

const AGENTS_PANEL_KEY: &str = "agents_panel";

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentsPanel {
    width: Option<Pixels>,
    pane: Option<SerializedAgentThreadPane>,
}

actions!(
    agents,
    [
        /// Toggle the visibility of the agents panel.
        ToggleAgentsPanel
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleAgentsPanel, window, cx| {
            workspace.toggle_panel_focus::<AgentsPanel>(window, cx);
        });
    })
    .detach();
}

pub struct AgentsPanel {
    focus_handle: gpui::FocusHandle,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    agent_thread_pane: Option<Entity<AgentThreadPane>>,
    history: Entity<AcpThreadHistory>,
    thread_store: Entity<ThreadStore>,
    prompt_store: Option<Entity<PromptStore>>,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    pending_restore: Option<SerializedAgentThreadPane>,
    pending_serialization: Task<Option<()>>,
    _subscriptions: Vec<Subscription>,
    _session_poll_task: Option<Task<()>>,
}

impl AgentsPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>, anyhow::Error>> {
        cx.spawn(async move |cx| {
            let serialized_panel = cx
                .background_spawn(async move {
                    KEY_VALUE_STORE
                        .read_kvp(AGENTS_PANEL_KEY)
                        .ok()
                        .flatten()
                        .and_then(|panel| {
                            serde_json::from_str::<SerializedAgentsPanel>(&panel).ok()
                        })
                })
                .await;

            let (fs, project) = workspace.update(cx, |workspace, _| {
                let fs = workspace.app_state().fs.clone();
                let project = workspace.project().clone();
                (fs, project)
            })?;

            let prompt_store = workspace
                .update(cx, |_, cx| PromptStore::global(cx))?
                .await
                .log_err();

            workspace.update_in(cx, |_, window, cx| {
                cx.new(|cx| {
                    let mut panel =
                        Self::new(workspace.clone(), fs, project, prompt_store, window, cx);
                    if let Some(serialized_panel) = serialized_panel {
                        panel.width = serialized_panel.width;
                        if let Some(serialized_pane) = serialized_panel.pane {
                            panel.restore_utility_pane(serialized_pane, window, cx);
                        }
                    }
                    panel
                })
            })
        })
    }

    fn try_load_cli_sessions(
        project: &Entity<Project>,
        history: &Entity<AcpThreadHistory>,
        cx: &mut App,
    ) {
        let project_path = project
            .read(cx)
            .worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf());

        let Some(project_path) = project_path else {
            return;
        };

        if !history.read(cx).is_empty() {
            return;
        }

        history.update(cx, |history, cx| {
            if let Some(index) = ClaudeCodeSessionIndex::for_project(&project_path) {
                log::info!("AgentsPanel: Found Claude Code sessions for project {:?}", project_path);
                let session_list: Rc<dyn AgentSessionList> =
                    Rc::new(ClaudeCodeSessionList::new(index));
                history.set_claude_session_list(session_list, cx);
            }

            if let Some(index) = CodexSessionIndex::for_project(&project_path) {
                log::info!("AgentsPanel: Found Codex sessions");
                let session_list: Rc<dyn AgentSessionList> =
                    Rc::new(CodexSessionList::new(index));
                history.set_codex_session_list(session_list, cx);
            }
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        fs: Arc<dyn Fs>,
        project: Entity<Project>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut ui::Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        let history = cx.new(|cx| AcpThreadHistory::new(None, window, cx));

        // Load Claude Code CLI sessions immediately and also observe for worktree changes
        Self::try_load_cli_sessions(&project, &history, cx);
        {
            let history_for_worktree = history.clone();
            let project_for_worktree = project.clone();
            cx.observe(&project, move |_, _, cx| {
                Self::try_load_cli_sessions(&project_for_worktree, &history_for_worktree, cx);
            })
            .detach();
        }

        let history_handle = history.clone();
        let connect_project = project.clone();
        let connect_thread_store = thread_store.clone();
        let connect_fs = fs.clone();
        cx.spawn(async move |_, cx| {
            let connect_task = cx.update(|cx| {
                let delegate = AgentServerDelegate::new(
                    connect_project.read(cx).agent_server_store().clone(),
                    connect_project.clone(),
                    None,
                    None,
                );
                let server = NativeAgentServer::new(connect_fs, connect_thread_store);
                server.connect(None, delegate, cx)
            });
            let connection = match connect_task.await {
                Ok((connection, _)) => connection,
                Err(error) => {
                    log::error!("Failed to connect native agent for history: {error:#}");
                    return;
                }
            };

            cx.update(|cx| {
                if let Some(session_list) = connection.session_list(cx) {
                    // Only set native session list if CLI sessions weren't already loaded
                    if history_handle.read(cx).is_empty() {
                        history_handle.update(cx, |history, cx| {
                            history.set_session_list(Some(session_list), cx);
                        });
                    }
                }
            });
        })
        .detach();

        // Poll the sessions directory every 5 seconds for new/removed .jsonl files
        let poll_project = project.clone();
        let poll_history = history.clone();
        let session_poll_task = cx.spawn(async move |_, cx| {
            loop {
                cx.background_executor().timer(std::time::Duration::from_secs(5)).await;
                let should_refresh = cx.update(|cx| {
                    let project_path = poll_project
                        .read(cx)
                        .worktrees(cx)
                        .next()
                        .map(|worktree| worktree.read(cx).abs_path().to_path_buf());
                    if let Some(project_path) = project_path {
                        if let Some(index) = ClaudeCodeSessionIndex::for_project(&project_path) {
                            let current_count = poll_history.read(cx).sessions().len();
                            let disk_count = index.list_sessions().map(|s| s.len()).unwrap_or(0);
                            return current_count != disk_count;
                        }
                    }
                    false
                });
                if should_refresh {
                    cx.update(|cx| {
                        poll_history.update(cx, |history, cx| {
                            history.refresh_sessions_pub(cx);
                        });
                    });
                }
            }
        });

        let this = cx.weak_entity();
        let subscriptions = vec![
            cx.subscribe_in(&history, window, Self::handle_history_event),
            cx.observe_in(&history, window, Self::handle_history_updated),
            cx.on_flags_ready(move |_, cx| {
                this.update(cx, |_, cx| {
                    cx.notify();
                })
                .ok();
            }),
        ];

        Self {
            focus_handle,
            workspace,
            project,
            agent_thread_pane: None,
            history,
            thread_store,
            prompt_store,
            fs,
            width: None,
            pending_restore: None,
            pending_serialization: Task::ready(None),
            _subscriptions: subscriptions,
            _session_poll_task: Some(session_poll_task),
        }
    }

    fn restore_utility_pane(
        &mut self,
        serialized_pane: SerializedAgentThreadPane,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if serialized_pane.tabs.is_empty() {
            return;
        }

        let mut entries_to_restore = Vec::new();
        let mut has_pending = false;

        for serialized_tab in &serialized_pane.tabs {
            let SerializedHistoryEntryId::AcpThread(id) = &serialized_tab.thread_id;
            let session_id = acp::SessionId::new(id.clone());
            if let Some(entry) = self.history.read(cx).session_for_id(&session_id) {
                entries_to_restore.push((entry, serialized_tab.custom_name.clone()));
            } else {
                has_pending = true;
            }
        }

        if entries_to_restore.is_empty() {
            self.pending_restore = Some(serialized_pane);
            return;
        }

        self.restore_tabs_from_entries(
            entries_to_restore,
            serialized_pane.active_tab_index,
            serialized_pane.expanded,
            serialized_pane.width,
            window,
            cx,
        );

        if has_pending {
            self.pending_restore = Some(serialized_pane);
        }
    }

    fn restore_tabs_from_entries(
        &mut self,
        entries: Vec<(AgentSessionInfo, Option<String>)>,
        _active_tab_index: usize,
        expanded: bool,
        width: Option<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if entries.is_empty() {
            return;
        }

        let fs = self.fs.clone();
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let thread_store = self.thread_store.clone();
        let prompt_store = self.prompt_store.clone();

        let agent_thread_pane = cx.new(|cx| {
            let mut pane = AgentThreadPane::new(workspace.clone(), cx);

            for (entry, _custom_name) in entries {
                pane.open_thread(
                    entry,
                    fs.clone(),
                    workspace.clone(),
                    project.clone(),
                    thread_store.clone(),
                    prompt_store.clone(),
                    window,
                    cx,
                );
            }

            if let Some(width) = width {
                pane.set_width(Some(width), cx);
            }
            pane.set_expanded(expanded, cx);
            pane
        });

        let state_subscription = cx.subscribe(&agent_thread_pane, Self::handle_utility_pane_event);
        let close_subscription = cx.subscribe(&agent_thread_pane, Self::handle_close_pane_event);

        self._subscriptions.push(state_subscription);
        self._subscriptions.push(close_subscription);

        let slot = self.utility_slot(window, cx);
        let panel_id = cx.entity_id();

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.register_utility_pane(slot, panel_id, agent_thread_pane.clone(), cx);
            });
        }

        self.agent_thread_pane = Some(agent_thread_pane);
        self.serialize(cx);
        cx.notify();
    }

    fn handle_utility_pane_event(
        &mut self,
        _utility_pane: Entity<AgentThreadPane>,
        event: &AgentsUtilityPaneEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            AgentsUtilityPaneEvent::StateChanged => {
                self.serialize(cx);
                cx.notify();
            }
        }
    }

    fn handle_close_pane_event(
        &mut self,
        _utility_pane: Entity<AgentThreadPane>,
        _event: &ClosePane,
        cx: &mut Context<Self>,
    ) {
        self.agent_thread_pane = None;
        self.serialize(cx);
        cx.notify();
    }

    fn handle_history_updated(
        &mut self,
        _history: Entity<AcpThreadHistory>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.maybe_restore_pending(window, cx);
    }

    fn handle_history_event(
        &mut self,
        _history: &Entity<AcpThreadHistory>,
        event: &ThreadHistoryEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ThreadHistoryEvent::Open(entry) => {
                let cli_source = entry.meta.as_ref().and_then(|m| {
                    let cli = m.get(CLI_SOURCE_KEY)?.as_str()?;
                    let path = m.get(CLI_PROJECT_PATH_KEY)?.as_str()?;
                    Some((cli.to_string(), std::path::PathBuf::from(path)))
                });
                if let Some((cli_command, project_path)) = cli_source {
                    let title = entry.title.as_ref().map(|t| t.to_string());
                    self.resume_cli_session(&cli_command, &entry.session_id.0, title.as_deref(), &project_path, window, cx);
                } else {
                    self.open_thread(entry.clone(), true, None, window, cx);
                }
            }
            ThreadHistoryEvent::EditContent(entry) => {
                let session_id = entry.session_id.clone();
                let fallback: SharedString =
                    session_id.0[..8.min(session_id.0.len())].to_string().into();
                let title = entry.title.clone().unwrap_or(fallback);
                let codex_path = entry
                    .meta
                    .as_ref()
                    .and_then(|m| m.get("codex_file_path"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from);
                self.edit_thread_content(session_id, title, codex_path, window, cx);
            }
        }
    }

    fn resume_cli_session(
        &self,
        cli_command: &str,
        session_id: &str,
        title: Option<&str>,
        project_path: &std::path::Path,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(terminal_panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
            log::error!("No terminal panel available to resume CLI session");
            return;
        };

        let is_codex = cli_command == "codex";
        let command = if is_codex {
            format!("{} resume {}\n", cli_command, session_id)
        } else {
            format!("{} --resume {}\n", cli_command, session_id)
        };

        let default_label = if is_codex { "Codex Session" } else { "Claude Code Session" };
        let label = title.unwrap_or(default_label).to_string();
        let cwd = Some(project_path.to_path_buf());

        let task = terminal_panel.update(cx, |panel, cx| {
            panel.add_terminal_shell(cwd, task::RevealStrategy::Always, window, cx)
        });

        cx.spawn_in(window, async move |_, cx| {
            let terminal = task.await?;
            terminal.update(cx, |terminal, cx| {
                terminal.set_title_override(Some(label), cx);
                terminal.input(command.into_bytes());
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn maybe_restore_pending(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.agent_thread_pane.is_some() {
            self.pending_restore = None;
            return;
        }

        let Some(pending) = self.pending_restore.as_ref() else {
            return;
        };

        if pending.tabs.is_empty() {
            self.pending_restore = None;
            return;
        }

        let mut entries_to_restore = Vec::new();
        let mut still_pending_tabs = Vec::new();

        for serialized_tab in &pending.tabs {
            let SerializedHistoryEntryId::AcpThread(id) = &serialized_tab.thread_id;
            let session_id = acp::SessionId::new(id.clone());
            if let Some(entry) = self.history.read(cx).session_for_id(&session_id) {
                entries_to_restore.push((entry, serialized_tab.custom_name.clone()));
            } else {
                still_pending_tabs.push(serialized_tab.clone());
            }
        }

        if entries_to_restore.is_empty() {
            return;
        }

        let pending = self.pending_restore.take().expect("pending restore");

        self.restore_tabs_from_entries(
            entries_to_restore,
            pending.active_tab_index,
            pending.expanded,
            pending.width,
            window,
            cx,
        );

        if !still_pending_tabs.is_empty() {
            self.pending_restore = Some(SerializedAgentThreadPane {
                expanded: pending.expanded,
                width: pending.width,
                tabs: still_pending_tabs,
                active_tab_index: 0,
            });
        }
    }

    fn edit_thread_content(
        &self,
        session_id: acp::SessionId,
        title: SharedString,
        codex_path: Option<PathBuf>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let file_path = if let Some(path) = codex_path {
            path
        } else {
            let project_path = self
                .project
                .read(cx)
                .worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf());

            let Some(project_path) = project_path else {
                log::error!("No project path available for edit_thread_content");
                return;
            };

            let Some(index) = ClaudeCodeSessionIndex::for_project(&project_path) else {
                log::error!("No Claude Code sessions directory found for project");
                return;
            };

            index.sessions_dir().join(format!("{}.jsonl", session_id.0))
        };

        if !file_path.exists() {
            log::error!(
                "Session file not found: {}",
                file_path.display()
            );
            return;
        }

        agent_ui::thread_content_editor::ThreadContentEditor::open(
            file_path,
            title,
            workspace,
            window,
            cx,
        )
        .detach_and_log_err(cx);
    }

    fn open_thread(
        &mut self,
        entry: AgentSessionInfo,
        expanded: bool,
        width: Option<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pending_restore = None;

        let fs = self.fs.clone();
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let thread_store = self.thread_store.clone();
        let prompt_store = self.prompt_store.clone();

        if let Some(existing_pane) = &self.agent_thread_pane {
            existing_pane.update(cx, |pane, cx| {
                pane.open_thread(
                    entry,
                    fs,
                    workspace,
                    project,
                    thread_store,
                    prompt_store,
                    window,
                    cx,
                );
                pane.set_expanded(true, cx);
            });
            self.serialize(cx);
            cx.notify();
            return;
        }

        let agent_thread_pane = cx.new(|cx| {
            let mut pane = AgentThreadPane::new(workspace.clone(), cx);
            pane.open_thread(
                entry,
                fs,
                workspace.clone(),
                project,
                thread_store,
                prompt_store,
                window,
                cx,
            );
            if let Some(width) = width {
                pane.set_width(Some(width), cx);
            }
            pane.set_expanded(expanded, cx);
            pane
        });

        let state_subscription = cx.subscribe(&agent_thread_pane, Self::handle_utility_pane_event);
        let close_subscription = cx.subscribe(&agent_thread_pane, Self::handle_close_pane_event);

        self._subscriptions.push(state_subscription);
        self._subscriptions.push(close_subscription);

        let slot = self.utility_slot(window, cx);
        let panel_id = cx.entity_id();

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.register_utility_pane(slot, panel_id, agent_thread_pane.clone(), cx);
            });
        }

        self.agent_thread_pane = Some(agent_thread_pane);
        self.serialize(cx);
        cx.notify();
    }

    fn utility_slot(&self, window: &Window, cx: &App) -> UtilityPaneSlot {
        let position = self.position(window, cx);
        utility_slot_for_dock_position(position)
    }

    fn re_register_utility_pane(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(pane) = &self.agent_thread_pane {
            let slot = self.utility_slot(window, cx);
            let panel_id = cx.entity_id();
            let pane = pane.clone();

            if let Some(workspace) = self.workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    workspace.register_utility_pane(slot, panel_id, pane, cx);
                });
            }
        }
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let pane = self
            .agent_thread_pane
            .as_ref()
            .map(|pane| pane.read(cx).serialize());

        self.pending_serialization = cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(
                    AGENTS_PANEL_KEY.into(),
                    serde_json::to_string(&SerializedAgentsPanel { width, pane }).unwrap(),
                )
                .await
                .log_err()
        });
    }
}

impl EventEmitter<PanelEvent> for AgentsPanel {}

impl Focusable for AgentsPanel {
    fn focus_handle(&self, _cx: &ui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for AgentsPanel {
    fn persistent_name() -> &'static str {
        "AgentsPanel"
    }

    fn panel_key() -> &'static str {
        AGENTS_PANEL_KEY
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        match AgentSettings::get_global(cx).agents_panel_dock {
            settings::DockSide::Left => DockPosition::Left,
            settings::DockSide::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position != DockPosition::Bottom
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.agent.get_or_insert_default().agents_panel_dock = Some(match position {
                DockPosition::Left => settings::DockSide::Left,
                DockPosition::Right | DockPosition::Bottom => settings::DockSide::Right,
            });
        });
        self.re_register_utility_pane(window, cx);
    }

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = AgentSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.width.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => {}
        }
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        (self.enabled(cx) && AgentSettings::get_global(cx).button).then_some(IconName::ZedAgentTwo)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Agents Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleAgentsPanel)
    }

    fn activation_priority(&self) -> u32 {
        4
    }

    fn enabled(&self, cx: &App) -> bool {
        AgentSettings::get_global(cx).enabled(cx) && cx.has_flag::<AgentV2FeatureFlag>()
    }
}

impl Render for AgentsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div().size_full().child(self.history.clone())
    }
}
