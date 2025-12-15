use agent::{HistoryEntry, HistoryStore};
use agent_settings::AgentSettings;
use anyhow::Result;
use assistant_text_thread::TextThreadStore;
use db::kvp::KEY_VALUE_STORE;
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
    dock::{DockPosition, PanelEvent, UtilityPane},
};

use crate::agent_thread_pane::{
    AgentThreadPane, AgentsUtilityPaneEvent, SerializedAgentThreadPane,
};
use crate::thread_history::{AcpThreadHistory, ThreadHistoryEvent};

const AGENTS_PANEL_KEY: &str = "agents_panel";

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentsPanel {
    width: Option<Pixels>,
    pane: SerializedAgentThreadPane,
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
    agent_thread_pane: Entity<AgentThreadPane>,
    history: Entity<AcpThreadHistory>,
    history_store: Entity<HistoryStore>,
    prompt_store: Option<Entity<PromptStore>>,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
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
                        panel
                            .agent_thread_pane
                            .update(cx, |pane, cx| pane.load(serialized_panel.pane, cx))
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
        let agent_thread_pane = cx.new(|cx| AgentThreadPane::new(workspace.clone(), cx));

        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));
        let history = cx.new(|cx| AcpThreadHistory::new(history_store.clone(), window, cx));

        let subscriptions = vec![
            cx.subscribe(&agent_thread_pane, Self::handle_utility_pane_event),
            cx.subscribe_in(&history, window, Self::handle_history_event),
        ];

        Self {
            focus_handle,
            workspace,
            project,
            agent_thread_pane,
            history,
            history_store,
            prompt_store,
            fs,
            width: None,
            pending_serialization: Task::ready(None),
            _subscriptions: subscriptions,
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

    fn handle_history_event(
        &mut self,
        _history: &Entity<AcpThreadHistory>,
        event: &ThreadHistoryEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ThreadHistoryEvent::Open(entry) => {
                self.open_thread(entry.clone(), window, cx);
            }
        }
    }

    fn open_thread(&mut self, entry: HistoryEntry, window: &mut Window, cx: &mut Context<Self>) {
        let fs = self.fs.clone();
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let history_store = self.history_store.clone();
        let prompt_store = self.prompt_store.clone();

        self.agent_thread_pane.update(cx, |pane, cx| {
            pane.open_thread(
                entry,
                fs,
                workspace,
                project,
                history_store,
                prompt_store,
                window,
                cx,
            );
            pane.set_expanded(true, cx);
        });
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let pane = self.agent_thread_pane.read(cx).serialize();

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
        AgentSettings::get_global(cx).dock.into()
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position != DockPosition::Bottom
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings
                .agent
                .get_or_insert_default()
                .set_dock(position.into());
        });
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
        (self.enabled(cx) && AgentSettings::get_global(cx).button).then_some(IconName::ZedAgent)
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
        AgentSettings::get_global(cx).enabled(cx)
    }

    fn utility_pane(
        &self,
        _window: &Window,
        _cx: &App,
    ) -> Option<Box<dyn workspace::dock::UtilityPaneHandle>> {
        Some(Box::new(self.agent_thread_pane.clone()))
    }
}

impl Render for AgentsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div().size_full().child(self.history.clone())
    }
}
