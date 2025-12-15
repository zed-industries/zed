use agent_settings::AgentSettings;
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use fs::Fs;
use gpui::{
    Action, AnyElement, AsyncWindowContext, Entity, EventEmitter, Focusable, Pixels, Subscription,
    Task, WeakEntity, actions, prelude::*, px,
};
use serde::{Deserialize, Serialize};
use settings::{Settings as _, update_settings_file};
use std::sync::Arc;
use ui::{
    App, Context, IconName, IntoElement, Label, LabelCommon as _, LabelSize, ListItem,
    ListItemSpacing, ParentElement, Render, RenderOnce, Styled, Window, div, h_flex,
};
use util::ResultExt;
use workspace::{
    Panel, Workspace,
    dock::{ClosePane, DockPosition, MinimizePane, PanelEvent, UtilityPane, UtilityPanePosition},
};

const AGENTS_PANEL_KEY: &str = "agents_panel";
const DEFAULT_UTILITY_PANE_WIDTH: Pixels = px(400.0);

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentsPanel {
    width: Option<Pixels>,
    #[serde(default)]
    utility_pane_expanded: bool,
    #[serde(default)]
    utility_pane_width: Option<Pixels>,
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
    utility_pane_view: Entity<AgentThreadPane>,
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

            workspace.update_in(cx, |workspace, _window, cx| {
                let fs = workspace.app_state().fs.clone();
                cx.new(|cx| {
                    let utility_pane_expanded = serialized_panel
                        .as_ref()
                        .map(|s| s.utility_pane_expanded)
                        .unwrap_or(false);
                    let utility_pane_width =
                        serialized_panel.as_ref().and_then(|s| s.utility_pane_width);

                    let mut panel = Self::new(fs, utility_pane_expanded, utility_pane_width, cx);
                    if let Some(serialized_panel) = serialized_panel {
                        panel.width = serialized_panel.width;
                    }
                    panel
                })
            })
        })
    }

    fn new(
        fs: Arc<dyn Fs>,
        utility_pane_expanded: bool,
        utility_pane_width: Option<Pixels>,
        cx: &mut ui::Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let agent_thread_pane =
            cx.new(|cx| AgentThreadPane::new(utility_pane_expanded, utility_pane_width, cx));

        let subscriptions = vec![cx.subscribe(&agent_thread_pane, Self::handle_utility_pane_event)];

        Self {
            focus_handle,
            utility_pane_view: agent_thread_pane,
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

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let utility_pane_expanded = self.utility_pane_view.read(cx).expanded(cx);
        let utility_pane_width = self.utility_pane_view.read(cx).width;

        self.pending_serialization = cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(
                    AGENTS_PANEL_KEY.into(),
                    serde_json::to_string(&SerializedAgentsPanel {
                        width,
                        utility_pane_expanded,
                        utility_pane_width,
                    })
                    .unwrap(),
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
        Some(Box::new(self.utility_pane_view.clone()))
    }
}

impl Render for AgentsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let agent_threads = vec![
            AgentThreadSummary {
                title: "Building the agents panel".into(),
                worktree_branch: Some("new-threads-pane".into()),
                diff: AgentThreadDiff {
                    removed: 0,
                    modified: 0,
                    added: 1,
                },
            },
            AgentThreadSummary {
                title: "Integrate Delta DB".into(),
                worktree_branch: Some("integrate-deltadb".into()),
                diff: AgentThreadDiff {
                    removed: 2,
                    modified: 10,
                    added: 3,
                },
            },
        ];

        div().size_full().children(agent_threads)
    }
}

#[derive(IntoElement)]
struct AgentThreadSummary {
    title: gpui::SharedString,
    worktree_branch: Option<gpui::SharedString>,
    diff: AgentThreadDiff,
}

impl RenderOnce for AgentThreadSummary {
    fn render(self, _window: &mut Window, _cx: &mut ui::App) -> impl IntoElement {
        ListItem::new("list-item")
            .rounded()
            .spacing(ListItemSpacing::Sparse)
            .start_slot(
                h_flex()
                    .w_full()
                    .gap_2()
                    .justify_between()
                    .child(Label::new(self.title).size(LabelSize::Default).truncate())
                    .children(
                        self.worktree_branch
                            .map(|branch| Label::new(branch).size(LabelSize::Small).truncate()),
                    )
                    .child(self.diff),
            )
    }
}

#[derive(IntoElement)]
struct AgentThreadDiff {
    removed: usize,
    modified: usize,
    added: usize,
}

impl RenderOnce for AgentThreadDiff {
    fn render(self, _window: &mut Window, _cx: &mut ui::App) -> impl IntoElement {
        Label::new(format!("{}:{}:{}", self.added, self.modified, self.removed))
    }
}

pub enum AgentsUtilityPaneEvent {
    StateChanged,
}

impl EventEmitter<AgentsUtilityPaneEvent> for AgentThreadPane {}
impl EventEmitter<MinimizePane> for AgentThreadPane {}
impl EventEmitter<ClosePane> for AgentThreadPane {}

pub struct AgentThreadPane {
    focus_handle: gpui::FocusHandle,
    expanded: bool,
    width: Option<Pixels>,
}

impl AgentThreadPane {
    pub fn new(expanded: bool, width: Option<Pixels>, cx: &mut ui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            focus_handle,
            expanded,
            width,
        }
    }
}

impl Focusable for AgentThreadPane {
    fn focus_handle(&self, _cx: &ui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl UtilityPane for AgentThreadPane {
    fn position(&self, _window: &Window, cx: &App) -> UtilityPanePosition {
        let dock_position = AgentSettings::get_global(cx).dock.into();
        match dock_position {
            DockPosition::Left | DockPosition::Bottom => UtilityPanePosition::Left,
            DockPosition::Right => UtilityPanePosition::Right,
        }
    }

    fn toggle_button(&self, _cx: &App) -> AnyElement {
        Label::new("Toggle Utility Pane").into_any_element()
    }

    fn expanded(&self, _cx: &App) -> bool {
        self.expanded
    }

    fn set_expanded(&mut self, expanded: bool, cx: &mut Context<Self>) {
        self.expanded = expanded;
        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }

    fn width(&self, _cx: &App) -> Pixels {
        self.width.unwrap_or(DEFAULT_UTILITY_PANE_WIDTH)
    }

    fn set_width(&mut self, width: Option<Pixels>, cx: &mut Context<Self>) {
        self.width = width;
        cx.emit(AgentsUtilityPaneEvent::StateChanged);
        cx.notify();
    }
}

impl Render for AgentThreadPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(Label::new("Thread Details (Placeholder)").size(LabelSize::Default))
    }
}
