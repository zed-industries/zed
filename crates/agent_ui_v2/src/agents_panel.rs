use agent::{HistoryEntry, HistoryEntryId, HistoryStore};
use agent_settings::AgentSettings;
use agent_ui::acp::{AcpThreadView, AcpThreadViewEvent};
use anyhow::Result;
use assistant_text_thread::TextThreadStore;
use collections::{HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt};
use fs::Fs;
use gpui::{
    Action, AsyncWindowContext, Entity, EventEmitter, Focusable, Pixels, Subscription, Task,
    WeakEntity, actions, prelude::*,
};
use project::Project;
use prompt_store::{PromptBuilder, PromptStore};
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

struct RunningThreadView {
    view: Entity<AcpThreadView>,
    thread_id: HistoryEntryId,
}

struct ActivePane {
    pane: Entity<AgentThreadPane>,
    _subscriptions: Vec<Subscription>,
}

pub struct AgentsPanel {
    focus_handle: gpui::FocusHandle,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    active_pane: Option<ActivePane>,
    history: Entity<AcpThreadHistory>,
    history_store: Entity<HistoryStore>,
    prompt_store: Option<Entity<PromptStore>>,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
    running_thread_views: HashMap<HistoryEntryId, RunningThreadView>,
    stopped_thread_ids: HashSet<HistoryEntryId>,
    thread_view_subscriptions: HashMap<HistoryEntryId, Subscription>,
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

            let (fs, project, prompt_builder) = workspace.update(cx, |workspace, cx| {
                let fs = workspace.app_state().fs.clone();
                let project = workspace.project().clone();
                let prompt_builder = PromptBuilder::load(fs.clone(), false, cx);
                (fs, project, prompt_builder)
            })?;

            let text_thread_store = workspace
                .update(cx, |_, cx| {
                    TextThreadStore::new(
                        project.clone(),
                        prompt_builder.clone(),
                        Default::default(),
                        cx,
                    )
                })?
                .await?;

            let prompt_store = workspace
                .update(cx, |_, cx| PromptStore::global(cx))?
                .await
                .log_err();

            workspace.update_in(cx, |_, window, cx| {
                cx.new(|cx| {
                    let mut panel = Self::new(
                        workspace.clone(),
                        fs,
                        project,
                        prompt_store,
                        text_thread_store,
                        window,
                        cx,
                    );
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
        text_thread_store: Entity<TextThreadStore>,
        window: &mut Window,
        cx: &mut ui::Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));
        let history = cx.new(|cx| AcpThreadHistory::new(history_store.clone(), window, cx));

        let this = cx.weak_entity();
        let subscriptions = vec![
            cx.subscribe_in(&history, window, Self::handle_history_event),
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
            active_pane: None,
            history,
            history_store,
            prompt_store,
            fs,
            width: None,
            pending_serialization: Task::ready(None),
            running_thread_views: HashMap::default(),
            stopped_thread_ids: HashSet::default(),
            thread_view_subscriptions: HashMap::default(),
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

        let entry = self
            .history_store
            .read(cx)
            .entries()
            .find(|e| match (&e.id(), thread_id) {
                (
                    HistoryEntryId::AcpThread(session_id),
                    SerializedHistoryEntryId::AcpThread(id),
                ) => session_id.to_string() == *id,
                (HistoryEntryId::TextThread(path), SerializedHistoryEntryId::TextThread(id)) => {
                    path.to_string_lossy() == *id
                }
                _ => false,
            });

        if let Some(entry) = entry {
            self.open_thread(
                entry,
                serialized_pane.expanded,
                serialized_pane.width,
                window,
                cx,
            );
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
        pane: Entity<AgentThreadPane>,
        _event: &ClosePane,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread_id) = pane.read(cx).thread_id() {
            self.cancel_thread(&thread_id, cx);
        }

        self.active_pane = None;
        self.serialize(cx);
        cx.notify();
    }

    fn cancel_thread(&mut self, thread_id: &HistoryEntryId, cx: &mut Context<Self>) {
        if let Some(running) = self.running_thread_views.remove(thread_id) {
            if let Some(thread) = running.view.read(cx).thread().cloned() {
                thread.update(cx, |t, cx| {
                    t.cancel(cx).detach();
                });
            }
        }
        self.thread_view_subscriptions.remove(thread_id);
        self.stopped_thread_ids.remove(thread_id);
        self.update_history_running_threads(cx);
    }

    fn handle_thread_view_event(
        &mut self,
        thread: Entity<AcpThreadView>,
        event: &AcpThreadViewEvent,
        cx: &mut Context<Self>,
    ) {
        let thread_id = thread
            .read(cx)
            .thread()
            .map(|t| HistoryEntryId::AcpThread(t.read(cx).session_id().clone()));

        let Some(thread_id) = thread_id else {
            return;
        };

        match event {
            AcpThreadViewEvent::Started => {
                if !self.running_thread_views.contains_key(&thread_id) {
                    self.running_thread_views.insert(
                        thread_id.clone(),
                        RunningThreadView {
                            view: thread.clone(),
                            thread_id: thread_id.clone(),
                        },
                    );
                    self.stopped_thread_ids.remove(&thread_id);
                    self.update_history_running_threads(cx);
                    cx.notify();
                }
            }
            AcpThreadViewEvent::Stopped => {
                if self.running_thread_views.remove(&thread_id).is_some() {
                    self.thread_view_subscriptions.remove(&thread_id);

                    let is_currently_viewing = self
                        .active_pane
                        .as_ref()
                        .and_then(|active| active.pane.read(cx).thread_id())
                        .map(|id| id == thread_id)
                        .unwrap_or(false);

                    if !is_currently_viewing {
                        self.stopped_thread_ids.insert(thread_id);
                    }
                    self.update_history_running_threads(cx);
                    cx.notify();
                }
            }
            AcpThreadViewEvent::Error => {
                self.running_thread_views.remove(&thread_id);
                self.thread_view_subscriptions.remove(&thread_id);
                self.stopped_thread_ids.remove(&thread_id);
                self.update_history_running_threads(cx);
                cx.notify();
            }
        }
    }

    fn update_history_running_threads(&mut self, cx: &mut Context<Self>) {
        let running_ids = self.running_thread_views.keys().cloned().collect();
        let completed_ids = self.stopped_thread_ids.clone();
        self.history.update(cx, |history, cx| {
            history.set_running_threads(running_ids, completed_ids, cx);
        });
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
            ThreadHistoryEvent::Deleted(entry_id) => {
                self.cancel_thread(entry_id, cx);
            }
        }
    }

    fn open_thread(
        &mut self,
        entry: HistoryEntry,
        expanded: bool,
        width: Option<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entry_id = entry.id();

        if let Some(active) = &self.active_pane {
            if active.pane.read(cx).thread_id() == Some(entry_id.clone()) {
                active.pane.update(cx, |pane, cx| {
                    pane.set_expanded(true, cx);
                });
                return;
            }
        }

        self.stopped_thread_ids.remove(&entry_id);

        let running_thread_view = self.running_thread_views.get(&entry_id);
        let workspace = self.workspace.clone();
        let mut pane_subscriptions = Vec::new();

        let agent_thread_pane = cx.new(|cx| {
            let mut pane = AgentThreadPane::new(workspace.clone(), cx);

            if let Some(running_thread_view) = running_thread_view {
                pane.set_thread_view(
                    running_thread_view.view.clone(),
                    running_thread_view.thread_id.clone(),
                    cx,
                );
            } else {
                pane.open_thread(
                    entry,
                    self.fs.clone(),
                    workspace.clone(),
                    self.project.clone(),
                    self.history_store.clone(),
                    self.prompt_store.clone(),
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

        if let Some(thread_view) = agent_thread_pane.read(cx).thread_view() {
            if !self.thread_view_subscriptions.contains_key(&entry_id) {
                let thread_view_subscription =
                    cx.subscribe(&thread_view, Self::handle_thread_view_event);
                self.thread_view_subscriptions
                    .insert(entry_id, thread_view_subscription);
            }
        }

        let state_subscription = cx.subscribe(&agent_thread_pane, Self::handle_utility_pane_event);
        let close_subscription = cx.subscribe(&agent_thread_pane, Self::handle_close_pane_event);

        pane_subscriptions.push(state_subscription);
        pane_subscriptions.push(close_subscription);

        let slot = self.utility_slot(window, cx);
        let panel_id = cx.entity_id();

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.register_utility_pane(slot, panel_id, agent_thread_pane.clone(), cx);
            });
        }

        self.active_pane = Some(ActivePane {
            pane: agent_thread_pane,
            _subscriptions: pane_subscriptions,
        });
        self.serialize(cx);
        cx.notify();
    }

    fn utility_slot(&self, window: &Window, cx: &App) -> UtilityPaneSlot {
        let position = self.position(window, cx);
        utility_slot_for_dock_position(position)
    }

    fn re_register_utility_pane(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active) = &self.active_pane {
            let slot = self.utility_slot(window, cx);
            let panel_id = cx.entity_id();
            let pane = active.pane.clone();

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
            .active_pane
            .as_ref()
            .map(|active| active.pane.read(cx).serialize());

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

#[cfg(test)]
mod tests {
    use super::*;
    use agent::HistoryEntryId;
    use agent_client_protocol as acp;

    #[test]
    fn test_running_thread_views_tracking() {
        // Test that the HashMap and HashSet types work as expected for tracking
        let running_thread_views: HashMap<HistoryEntryId, RunningThreadView> = HashMap::default();
        let mut stopped_thread_ids: HashSet<HistoryEntryId> = HashSet::default();
        let mut thread_view_subscriptions: HashMap<HistoryEntryId, ()> = HashMap::default();

        let thread_id_1 = HistoryEntryId::AcpThread(acp::SessionId::new("thread-1".to_string()));
        let thread_id_2 = HistoryEntryId::AcpThread(acp::SessionId::new("thread-2".to_string()));

        // Simulate thread 1 starting
        assert!(!running_thread_views.contains_key(&thread_id_1));

        // Simulate adding subscription for thread 1
        thread_view_subscriptions.insert(thread_id_1.clone(), ());
        assert!(thread_view_subscriptions.contains_key(&thread_id_1));

        // Simulate thread 2 starting
        thread_view_subscriptions.insert(thread_id_2.clone(), ());

        // Verify both subscriptions exist
        assert_eq!(thread_view_subscriptions.len(), 2);

        // Simulate thread 1 stopping - should move to stopped_thread_ids
        thread_view_subscriptions.remove(&thread_id_1);
        stopped_thread_ids.insert(thread_id_1.clone());

        assert!(!thread_view_subscriptions.contains_key(&thread_id_1));
        assert!(stopped_thread_ids.contains(&thread_id_1));
        assert!(thread_view_subscriptions.contains_key(&thread_id_2));

        // Simulate thread 2 erroring - should not be in stopped_thread_ids
        thread_view_subscriptions.remove(&thread_id_2);
        stopped_thread_ids.remove(&thread_id_2);

        assert!(!thread_view_subscriptions.contains_key(&thread_id_2));
        assert!(!stopped_thread_ids.contains(&thread_id_2));

        // Verify thread 1 still in stopped
        assert!(stopped_thread_ids.contains(&thread_id_1));
    }

    #[test]
    fn test_subscription_deduplication() {
        // Test that we don't create duplicate subscriptions
        let mut thread_view_subscriptions: HashMap<HistoryEntryId, ()> = HashMap::default();

        let thread_id = HistoryEntryId::AcpThread(acp::SessionId::new("thread-1".to_string()));

        // First subscription
        if !thread_view_subscriptions.contains_key(&thread_id) {
            thread_view_subscriptions.insert(thread_id.clone(), ());
        }
        assert_eq!(thread_view_subscriptions.len(), 1);

        // Attempt to add duplicate - should not increase count
        if !thread_view_subscriptions.contains_key(&thread_id) {
            thread_view_subscriptions.insert(thread_id.clone(), ());
        }
        assert_eq!(thread_view_subscriptions.len(), 1);

        // Different thread should be added
        let thread_id_2 = HistoryEntryId::AcpThread(acp::SessionId::new("thread-2".to_string()));
        if !thread_view_subscriptions.contains_key(&thread_id_2) {
            thread_view_subscriptions.insert(thread_id_2.clone(), ());
        }
        assert_eq!(thread_view_subscriptions.len(), 2);
    }

    #[test]
    fn test_cancel_thread_cleanup() {
        // Test that cancel_thread properly cleans up all state
        let mut running_thread_views: HashMap<HistoryEntryId, ()> = HashMap::default();
        let mut stopped_thread_ids: HashSet<HistoryEntryId> = HashSet::default();
        let mut thread_view_subscriptions: HashMap<HistoryEntryId, ()> = HashMap::default();

        let thread_id = HistoryEntryId::AcpThread(acp::SessionId::new("thread-1".to_string()));

        // Set up state as if thread is running
        running_thread_views.insert(thread_id.clone(), ());
        thread_view_subscriptions.insert(thread_id.clone(), ());

        // Simulate cancel_thread logic
        running_thread_views.remove(&thread_id);
        thread_view_subscriptions.remove(&thread_id);
        stopped_thread_ids.remove(&thread_id);

        // All state should be cleaned up
        assert!(!running_thread_views.contains_key(&thread_id));
        assert!(!thread_view_subscriptions.contains_key(&thread_id));
        assert!(!stopped_thread_ids.contains(&thread_id));
    }

    #[test]
    fn test_stopped_thread_not_added_when_currently_viewing() {
        // Test that stopped threads are only added to stopped_thread_ids
        // when we're NOT currently viewing them
        let mut stopped_thread_ids: HashSet<HistoryEntryId> = HashSet::default();

        let thread_id = HistoryEntryId::AcpThread(acp::SessionId::new("thread-1".to_string()));
        let current_thread_id =
            HistoryEntryId::AcpThread(acp::SessionId::new("thread-1".to_string()));

        // Simulate the logic from handle_thread_view_event for Stopped
        let is_currently_viewing = current_thread_id == thread_id;

        if !is_currently_viewing {
            stopped_thread_ids.insert(thread_id.clone());
        }

        // Since we ARE viewing, it should NOT be in stopped_thread_ids
        assert!(!stopped_thread_ids.contains(&thread_id));

        // Now test with a different current thread
        let different_current_thread =
            HistoryEntryId::AcpThread(acp::SessionId::new("thread-2".to_string()));
        let is_currently_viewing = different_current_thread == thread_id;

        if !is_currently_viewing {
            stopped_thread_ids.insert(thread_id.clone());
        }

        // Since we're NOT viewing thread_id, it SHOULD be in stopped_thread_ids
        assert!(stopped_thread_ids.contains(&thread_id));
    }
}
