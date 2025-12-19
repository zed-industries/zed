//! Git Graph View - integrates git_graph crate with Zed's git infrastructure

use git_graph::{GitGraph, GitGraphView};
use gpui::{
    actions, App, Context, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, Render, SharedString, Task, WeakEntity, Window,
};
use project::git_store::{GitStore, Repository};
use ui::prelude::*;
use workspace::{
    Item, Workspace,
    item::ItemEvent,
};

actions!(git_graph, [ToggleGitGraph, RefreshGraph]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &ToggleGitGraph, window, cx| {
            GitGraphPanel::toggle(workspace, window, cx);
        });
    })
    .detach();
}

/// Panel wrapper for GitGraphView
pub struct GitGraphPanel {
    view: Entity<GitGraphView>,
    #[allow(dead_code)]
    repository: Option<WeakEntity<Repository>>,
    #[allow(dead_code)]
    git_store: Option<WeakEntity<GitStore>>,
    #[allow(dead_code)]
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    loading: bool,
}

impl GitGraphPanel {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();

        // Get current repository
        let repo = git_store.read(cx).active_repository();

        if let Some(repo) = repo {
            let weak_workspace = workspace.weak_handle();
            let weak_git_store = git_store.downgrade();
            let weak_repo = repo.downgrade();

            // Check if we already have a GitGraphPanel open
            let existing = workspace.active_pane().read(cx).items().find_map(|item| {
                item.downcast::<GitGraphPanel>()
            });

            if let Some(existing) = existing {
                // Focus existing panel
                workspace.activate_item(&existing, true, true, window, cx);
            } else {
                // Create new panel
                let panel = cx.new(|cx| {
                    GitGraphPanel::new(
                        weak_workspace,
                        Some(weak_git_store),
                        Some(weak_repo.clone()),
                        window,
                        cx,
                    )
                });

                workspace.active_pane().update(cx, |pane, cx| {
                    pane.add_item(Box::new(panel.clone()), true, true, None, window, cx);
                });

                // Load graph data
                panel.update(cx, |panel, cx| {
                    panel.load_graph(weak_repo, window, cx);
                });
            }
        }
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        git_store: Option<WeakEntity<GitStore>>,
        repository: Option<WeakEntity<Repository>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let view = cx.new(|cx| {
            GitGraphView::new(GitGraph::new(), cx)
        });

        Self {
            view,
            repository,
            git_store,
            workspace,
            focus_handle,
            loading: false,
        }
    }

    fn load_graph(
        &mut self,
        _repo: WeakEntity<Repository>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.loading = true;
        cx.notify();

        // For now, create a simple placeholder graph
        // TODO: Implement actual git log fetching with parent info
        cx.spawn(async move |this, cx| {
            // Simulate async loading
            cx.background_executor().timer(std::time::Duration::from_millis(100)).await;

            // Create sample graph for testing
            let sample_output = create_sample_graph_data();
            let delimiter = "<<COMMIT_END>>";

            if let Ok(graph) = GitGraph::from_git_log(&sample_output, delimiter) {
                let _ = this.update(cx, |this, cx| {
                    this.loading = false;
                    this.view.update(cx, |view, cx| {
                        view.update_graph(graph, cx);
                    });
                    cx.notify();
                });
            }
        }).detach();
    }
}

fn create_sample_graph_data() -> String {
    // Sample git log format: SHA\0PARENTS\0SUBJECT\0TIMESTAMP\0AUTHOR_NAME\0AUTHOR_EMAIL\0REFS
    let commits = vec![
        ("abc1234567890", "", "feat: Add git graph visualization", "1703001600", "Roberto", "roberto@example.com", "(HEAD -> main, origin/main)"),
        ("def2345678901", "abc1234567890", "fix: Resolve layout issues", "1702997600", "Roberto", "roberto@example.com", ""),
        ("fed3456789012", "abc1234567890", "feat: Add keyboard navigation", "1702994000", "Roberto", "roberto@example.com", "(feature-branch)"),
        ("012456789abcd", "def2345678901 fed3456789012", "Merge branch 'feature-branch'", "1702990000", "Roberto", "roberto@example.com", ""),
        ("789abcdef0123", "012456789abcd", "Initial commit", "1702980000", "Roberto", "roberto@example.com", ""),
    ];

    commits.iter()
        .map(|(sha, parents, subject, ts, author, email, refs)| {
            format!("{}\0{}\0{}\0{}\0{}\0{}\0{}", sha, parents, subject, ts, author, email, refs)
        })
        .collect::<Vec<_>>()
        .join("<<COMMIT_END>>")
}

impl Focusable for GitGraphPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<ItemEvent> for GitGraphPanel {}

impl Item for GitGraphPanel {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Git Graph".into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch))
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(ItemEvent)) {
        // No conversion needed
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(&self, _workspace_id: Option<workspace::WorkspaceId>, _window: &mut Window, _cx: &mut Context<Self>) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(None)
    }
}

impl Render for GitGraphPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.loading {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("Loading git graph...").size(LabelSize::Default))
        } else {
            div()
                .size_full()
                .child(self.view.clone())
        }
    }
}
