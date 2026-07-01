use gpui::{
    Action, AsyncWindowContext, Entity, EventEmitter, FocusHandle, Focusable, Subscription,
    WeakEntity, actions,
};
use project::{
    Project,
    git_store::{GitStore, GitStoreEvent},
};
use ui::prelude::*;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use crate::git_graph::{GitGraph, GitGraphHost};

actions!(
    git_graph_panel,
    [
        /// Toggles focus on the git graph panel.
        ToggleFocus
    ]
);

const GIT_GRAPH_PANEL_KEY: &str = "GitGraphPanel";
const DEFAULT_WIDTH: Pixels = px(360.);

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<GitGraphPanel>(window, cx);
    });
}

/// A dockable sidebar panel that hosts the commit [`GitGraph`] for the active
/// repository, as an alternative to opening the graph as a full editor tab.
pub struct GitGraphPanel {
    graph: Option<Entity<GitGraph>>,
    project: Entity<Project>,
    git_store: Entity<GitStore>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    position: DockPosition,
    _subscriptions: Vec<Subscription>,
}

impl GitGraphPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            Self::new(workspace, window, cx)
        })
    }

    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let weak_workspace = workspace.weak_handle();

        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            let subscriptions = vec![cx.subscribe_in(
                &git_store,
                window,
                |this: &mut Self, _git_store, event, window, cx| match event {
                    GitStoreEvent::ActiveRepositoryChanged(_)
                    | GitStoreEvent::RepositoryAdded
                    | GitStoreEvent::RepositoryRemoved(_) => {
                        this.update_active_repository(window, cx);
                    }
                    _ => {}
                },
            )];

            let mut this = Self {
                graph: None,
                project,
                git_store,
                workspace: weak_workspace,
                focus_handle,
                position: DockPosition::Right,
                _subscriptions: subscriptions,
            };
            this.update_active_repository(window, cx);
            this
        })
    }

    /// Points the hosted graph at the project's active repository, constructing
    /// the graph on first use. Does nothing when there is no active repository,
    /// leaving any previously constructed graph in place.
    fn update_active_repository(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.project.read(cx).active_repository(cx) else {
            return;
        };
        let repo_id = repository.read(cx).id;

        if let Some(graph) = &self.graph {
            graph.update(cx, |graph, cx| graph.set_repo_id(repo_id, cx));
        } else {
            let git_store = self.git_store.clone();
            let workspace = self.workspace.clone();
            let graph = cx.new(|cx| {
                let mut graph = GitGraph::new(repo_id, git_store, workspace, None, window, cx);
                graph.set_host(GitGraphHost::Panel);
                graph
            });
            self.graph = Some(graph);
        }
        cx.notify();
    }
}

impl Focusable for GitGraphPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.graph {
            Some(graph) => graph.read(cx).focus_handle(cx),
            None => self.focus_handle.clone(),
        }
    }
}

impl EventEmitter<PanelEvent> for GitGraphPanel {}

impl Render for GitGraphPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let Some(graph) = self.graph.clone() else {
            return v_flex()
                .key_context("GitGraphPanel")
                .track_focus(&self.focus_handle)
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("No active repository").color(Color::Muted))
                .into_any_element();
        };

        v_flex()
            .key_context("GitGraphPanel")
            .size_full()
            .child(graph)
            .into_any_element()
    }
}

impl Panel for GitGraphPanel {
    fn persistent_name() -> &'static str {
        "GitGraphPanel"
    }

    fn panel_key() -> &'static str {
        GIT_GRAPH_PANEL_KEY
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        DEFAULT_WIDTH
    }

    fn icon(&self, _: &Window, _: &App) -> Option<IconName> {
        Some(IconName::GitGraph)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Git Graph Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        5
    }
}
