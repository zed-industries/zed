use gpui::{AnyView, AppContext, BorrowAppContext, Entity, EventEmitter, Focusable, actions};
use settings::SettingsStore;
use ui::{
    App, Context, IconName, IntoElement, Label, LabelCommon as _, LabelSize, ListItem,
    ListItemSpacing, ParentElement, Render, RenderOnce, Styled, Window, div, h_flex, px,
};
use workspace::{
    Panel, Workspace,
    dock::{DockPosition, PanelEvent},
};

actions!(
    agents,
    [
        /// Toggle the visibility of the agents panel.
        ToggleAgentsPanel
    ]
);

pub fn init(cx: &mut App) {
    // init_settings(cx);

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
    position: DockPosition,
}

impl AgentsPanel {
    pub fn new(cx: &mut ui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let utility_pane_view = cx.new(|cx| AgentsUtilityPane::new(cx)).into();
        Self {
            focus_handle,
            utility_pane_view,
            position: DockPosition::Left,
        }
    }
}

impl Panel for AgentsPanel {
    fn persistent_name() -> &'static str {
        "AgentsPanel"
    }

    fn panel_key() -> &'static str {
        "AgentsPanel"
    }

    fn position(&self, _window: &ui::Window, _cx: &ui::App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        match position {
            DockPosition::Left | DockPosition::Right => true,
            DockPosition::Bottom => false,
        }
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) {
        self.position = position;
        // Trigger SettingsStore observer in Dock to move the panel
        cx.update_global::<SettingsStore, _>(|_, _| {});
    }

    fn size(&self, _window: &ui::Window, _cx: &ui::App) -> ui::Pixels {
        // TODO!
        px(300.0)
    }

    fn set_size(
        &mut self,
        _size: Option<ui::Pixels>,
        _window: &mut ui::Window,
        _cx: &mut ui::Context<Self>,
    ) {
        // TODO!
    }

    fn icon(&self, _window: &ui::Window, _cx: &ui::App) -> Option<ui::IconName> {
        //todo!
        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _window: &ui::Window, _cx: &ui::App) -> Option<&'static str> {
        //todo!
        Some("Agents panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleAgentsPanel)
    }

    fn activation_priority(&self) -> u32 {
        1
    }

    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        true
    }
    fn enabled(&self, _cx: &App) -> bool {
        true
    }

    fn utility_pane(&self, _window: &Window, _cx: &App) -> Option<AnyView> {
        Some(self.utility_pane_view.clone().into())
    }
}

impl EventEmitter<PanelEvent> for AgentsPanel {}

impl Focusable for AgentsPanel {
    fn focus_handle(&self, _cx: &ui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(IntoElement)]
struct AgentThreadSummary {
    title: gpui::SharedString,
    worktree_branch: Option<gpui::SharedString>,
    diff: AgentThreadDiff,
}

#[derive(IntoElement)]
struct AgentThreadDiff {
    removed: usize,
    modified: usize,
    added: usize,
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
