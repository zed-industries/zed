use gpui::{EventEmitter, Focusable, actions};
use ui::{
    App, Context, IconName, IntoElement, Label, LabelCommon as _, LabelSize, ListItem,
    ListItemSpacing, ParentElement, Render, RenderOnce, Styled, Toggleable as _, Window, div,
    h_flex, px,
};
use workspace::{Panel, Workspace, dock::PanelEvent};

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
}

impl AgentsPanel {
    pub fn new(cx: &mut ui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self { focus_handle }
    }
}

impl Panel for AgentsPanel {
    fn persistent_name() -> &'static str {
        "AgentsPanel"
    }

    fn panel_key() -> &'static str {
        "AgentsPanel"
    }

    fn position(&self, window: &ui::Window, cx: &ui::App) -> workspace::dock::DockPosition {
        workspace::dock::DockPosition::Left
    }

    fn position_is_valid(&self, position: workspace::dock::DockPosition) -> bool {
        match position {
            workspace::dock::DockPosition::Left | workspace::dock::DockPosition::Right => true,
            workspace::dock::DockPosition::Bottom => false,
        }
    }

    fn set_position(
        &mut self,
        _position: workspace::dock::DockPosition,
        _window: &mut ui::Window,
        _cx: &mut ui::Context<Self>,
    ) {
        // TODO!
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
