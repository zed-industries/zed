use acp_thread::AgentSessionInfo;
use agent::{NativeAgentServer, ThreadStore};
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
use std::sync::Arc;
use ui::{App, Context, IconName, IntoElement, ParentElement, Render, Styled, Window};
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
                    history_handle.update(cx, |history, cx| {
                        history.set_session_list(Some(session_list), cx);
                    });
                }
            });
        })
        .detach();

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
        }
    }

    fn restore_utility_pane(
        &mut self,
        serialized_pane: SerializedAgentThreadPane,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread_id) = &serialized_pane.thread_id else {
            return;
        };

        let SerializedHistoryEntryId::AcpThread(id) = thread_id;
        let session_id = acp::SessionId::new(id.clone());
        if let Some(entry) = self.history.read(cx).session_for_id(&session_id) {
            self.open_thread(
                entry,
                serialized_pane.expanded,
                serialized_pane.width,
                window,
                cx,
            );
        } else {
            self.pending_restore = Some(serialized_pane);
        }
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
                self.open_thread(entry.clone(), true, None, window, cx);
            }
        }
    }

    fn maybe_restore_pending(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.agent_thread_pane.is_some() {
            self.pending_restore = None;
            return;
        }

        let Some(pending) = self.pending_restore.as_ref() else {
            return;
        };
        let Some(thread_id) = &pending.thread_id else {
            self.pending_restore = None;
            return;
        };

        let SerializedHistoryEntryId::AcpThread(id) = thread_id;
        let session_id = acp::SessionId::new(id.clone());
        let Some(entry) = self.history.read(cx).session_for_id(&session_id) else {
            return;
        };

        let pending = self.pending_restore.take().expect("pending restore");
        self.open_thread(entry, pending.expanded, pending.width, window, cx);
    }

    fn open_thread(
        &mut self,
        entry: AgentSessionInfo,
        expanded: bool,
        width: Option<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entry_id = entry.session_id.clone();
        self.pending_restore = None;

        if let Some(existing_pane) = &self.agent_thread_pane {
            if existing_pane.read(cx).thread_id() == Some(entry_id) {
                existing_pane.update(cx, |pane, cx| {
                    pane.set_expanded(true, cx);
                });
                return;
            }
        }

        let fs = self.fs.clone();
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let thread_store = self.thread_store.clone();
        let prompt_store = self.prompt_store.clone();

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
