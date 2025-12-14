use agent_settings::AgentSettings;
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use fs::Fs;
use gpui::{
    Action, AnyView, AsyncWindowContext, Entity, EventEmitter, Focusable, Pixels, Task, WeakEntity,
    actions, prelude::*,
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
    dock::{DockPosition, PanelEvent},
};

const AGENTS_PANEL_KEY: &str = "agents_panel";

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentsPanel {
    width: Option<Pixels>,
    #[serde(default)]
    utility_pane_expanded: bool,
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
    utility_pane_view: Entity<AgentsUtilityPane>,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    utility_pane_expanded: bool,
    pending_serialization: Task<Option<()>>,
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
                    let mut panel = Self::new(fs, cx);
                    if let Some(serialized_panel) = serialized_panel {
                        panel.width = serialized_panel.width;
                        panel.utility_pane_expanded = serialized_panel.utility_pane_expanded;
                    }
                    panel
                })
            })
        })
    }

    fn new(fs: Arc<dyn Fs>, cx: &mut ui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let utility_pane_view = cx.new(|cx| AgentsUtilityPane::new(cx));
        Self {
            focus_handle,
            utility_pane_view,
            fs,
            width: None,
            utility_pane_expanded: false,
            pending_serialization: Task::ready(None),
        }
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let utility_pane_expanded = self.utility_pane_expanded;
        self.pending_serialization = cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(
                    AGENTS_PANEL_KEY.into(),
                    serde_json::to_string(&SerializedAgentsPanel {
                        width,
                        utility_pane_expanded,
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
        1
    }

    fn enabled(&self, cx: &App) -> bool {
        AgentSettings::get_global(cx).enabled(cx)
    }

    fn utility_pane(&self, _window: &Window, _cx: &App) -> Option<AnyView> {
        Some(self.utility_pane_view.clone().into())
    }

    fn utility_pane_expanded(&self, _cx: &App) -> bool {
        self.utility_pane_expanded
    }

    fn set_utility_pane_expanded(&mut self, expanded: bool, cx: &mut Context<Self>) {
        self.utility_pane_expanded = expanded;
        self.serialize(cx);
        cx.notify();
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

pub struct AgentsUtilityPane {
    focus_handle: gpui::FocusHandle,
}

impl AgentsUtilityPane {
    pub fn new(cx: &mut ui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self { focus_handle }
    }
}

impl Focusable for AgentsUtilityPane {
    fn focus_handle(&self, _cx: &ui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AgentsUtilityPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(Label::new("Thread Details (Placeholder)").size(LabelSize::Default))
    }
}
