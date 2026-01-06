mod graph;
mod graph_rendering;

use anyhow::Context as _;
use gpui::{
    AnyElement, App, ClickEvent, Context, Corner, ElementId, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, ListAlignment, ListState, ParentElement, Pixels, Point, Render,
    SharedString, Styled, Subscription, Task, WeakEntity, Window, actions, anchored, deferred,
    list, px,
};
use graph_rendering::accent_colors_count;
use project::{
    Project,
    git_store::{GitStoreEvent, RepositoryEvent},
};
use settings::Settings;
use std::path::PathBuf;
use theme::ThemeSettings;
use ui::{ContextMenu, Tooltip, prelude::*};
use util::ResultExt;
use workspace::{
    Workspace,
    item::{Item, ItemEvent, SerializableItem},
};

use crate::{
    graph::{AllCommitCount, CHUNK_SIZE},
    graph_rendering::render_graph,
};

actions!(
    git_graph,
    [
        /// Opens the Git Graph panel.
        OpenGitGraph,
        /// Opens the commit view for the selected commit.
        OpenCommitView,
    ]
);

pub fn init(cx: &mut App) {
    workspace::register_serializable_item::<GitGraph>(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenGitGraph, window, cx| {
            let project = workspace.project().clone();
            let git_graph = cx.new(|cx| GitGraph::new(project, window, cx));
            workspace.add_item_to_active_pane(Box::new(git_graph), None, true, window, cx);
        });
    })
    .detach();
}

pub struct GitGraph {
    focus_handle: FocusHandle,
    graph: crate::graph::GitGraph,
    project: Entity<Project>,
    max_lanes: usize,
    loading: bool,
    error: Option<SharedString>,
    _load_task: Option<Task<()>>,
    selected_commit: Option<usize>,
    expanded_commit: Option<usize>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    work_dir: Option<PathBuf>,
    row_height: Pixels,
    list_state: ListState,
    _subscriptions: Vec<Subscription>,
}

impl GitGraph {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();

        let git_store = project.read(cx).git_store().clone();
        let git_store_subscription = cx.subscribe(&git_store, |this, _, event, cx| match event {
            GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::BranchChanged, true)
            | GitStoreEvent::ActiveRepositoryChanged(_) => {
                // todo! only call load data from render, we should set a bool here
                // todo! We should check that the repo actually has a change that would affect the graph
                this.load_data(false, cx);
            }
            _ => {}
        });

        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        let row_height = font_size + px(10.0);

        let list_state = ListState::new(0, ListAlignment::Top, px(500.0));

        let accent_colors = cx.theme().accents();
        let mut this = GitGraph {
            focus_handle,
            project,
            graph: crate::graph::GitGraph::new(accent_colors_count(accent_colors)),
            max_lanes: 0,
            loading: true,
            error: None,
            _load_task: None,
            selected_commit: None,
            expanded_commit: None,
            context_menu: None,
            work_dir: None,
            row_height,
            list_state,
            // todo! We can just make this a simple Subscription instead of wrapping it
            _subscriptions: vec![git_store_subscription],
        };

        this.load_data(true, cx);
        this
    }

    fn load_data(&mut self, fetch_chunks: bool, cx: &mut Context<Self>) {
        let project = self.project.clone();
        self.loading = true;
        self.error = None;
        let commit_count_loaded = !matches!(self.graph.max_commit_count, AllCommitCount::NotLoaded);

        if self._load_task.is_some() {
            return;
        }

        let last_loaded_chunk = if !fetch_chunks {
            // When we're refreshing the graph we need to start from the beginning
            // so the cached commits don't matter
            0
        } else {
            self.graph.commits.len() / CHUNK_SIZE
        };

        let first_visible_worktree = project.read_with(cx, |project, cx| {
            project
                .visible_worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        });

        self._load_task = Some(cx.spawn(async move |this: WeakEntity<Self>, cx| {
            let Some(worktree_path) = first_visible_worktree
                .context("Can't open git graph in Project without visible worktrees")
                .ok()
            else {
                // todo! handle error
                return;
            };

            // todo! don't count commits everytime
            let commit_count = if fetch_chunks && commit_count_loaded {
                None
            } else {
                crate::graph::commit_count(&worktree_path).await.ok()
            };
            let result = crate::graph::load_commits(last_loaded_chunk, worktree_path.clone()).await;

            this.update(cx, |this, cx| {
                this.loading = false;
                match result.map(|commits| (commits, commit_count)) {
                    Ok((commits, commit_count)) => {
                        if !fetch_chunks {
                            this.graph.clear();
                        }

                        this.graph.add_commits(commits);
                        this.max_lanes = this.graph.max_lanes;
                        this.work_dir = Some(worktree_path);

                        if let Some(commit_count) = commit_count {
                            this.graph.max_commit_count = AllCommitCount::Loaded(commit_count);
                            this.list_state.reset(commit_count);
                        }
                    }
                    Err(e) => {
                        this.error = Some(format!("{:?}", e).into());
                    }
                };

                this._load_task.take();
                cx.notify();
            })
            .log_err();
        }));
    }

    // todo unflatten this function
    fn render_list_item(
        &mut self,
        idx: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let row_height = self.row_height;
        // let graph_width = px(16.0) * (self.max_lanes.max(2) as f32) + px(24.0);
        // todo! make these widths constant
        let date_width = px(140.0);
        let author_width = px(120.0);
        let commit_width = px(80.0);

        self.render_commit_row(idx, row_height, date_width, author_width, commit_width, cx)
    }

    fn render_commit_row(
        &mut self,
        idx: usize,
        row_height: Pixels,
        date_width: Pixels,
        author_width: Pixels,
        commit_width: Pixels,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if (idx + CHUNK_SIZE).min(self.graph.max_commit_count.count()) > self.graph.commits.len() {
            self.load_data(true, cx);
        }

        let Some(commit) = self.graph.commits.get(idx) else {
            // todo! loading row element
            return div().h(row_height).into_any_element();
        };

        let subject: SharedString = commit.data.subject.clone().into();
        let author_name: SharedString = commit.data.author_name.clone().into();
        let short_sha: SharedString = commit.data.sha.display_short().into();
        let formatted_time: SharedString = commit.data.commit_timestamp.clone().into();

        let is_selected = self.expanded_commit == Some(idx);
        let bg = if is_selected {
            cx.theme().colors().ghost_element_selected
        } else {
            cx.theme().colors().editor_background
        };
        let hover_bg = cx.theme().colors().ghost_element_hover;

        h_flex()
            .id(ElementId::NamedInteger("commit-row".into(), idx as u64))
            .w_full()
            .size_full()
            .px_2()
            .gap_4()
            .h(row_height)
            .min_h(row_height)
            .flex_shrink_0()
            .bg(bg)
            .hover(move |style| style.bg(hover_bg))
            .on_click(cx.listener(move |this, _event: &ClickEvent, _window, _cx| {
                this.selected_commit = Some(idx);
            }))
            .child(
                h_flex()
                    .flex_1()
                    .min_w(px(0.0))
                    .gap_2()
                    .overflow_hidden()
                    .items_center()
                    .child(
                        div()
                            .id(ElementId::NamedInteger("commit-subject".into(), idx as u64))
                            .flex_1()
                            .min_w(px(0.0))
                            .overflow_hidden()
                            .tooltip(Tooltip::text(subject.clone()))
                            .child(Label::new(subject).single_line()),
                    ),
            )
            .child(
                div()
                    .w(date_width)
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(Label::new(formatted_time).color(Color::Muted).single_line()),
            )
            .child(
                div()
                    .w(author_width)
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(Label::new(author_name).color(Color::Muted).single_line()),
            )
            .child(
                div()
                    .w(commit_width)
                    .flex_shrink_0()
                    .child(Label::new(short_sha).color(Color::Accent).single_line()),
            )
            .debug()
            .into_any_element()
    }
}

impl Render for GitGraph {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let graph_width = px(16.0) * (4 as f32) + px(24.0);
        let date_width = px(140.0);
        let author_width = px(120.0);
        let commit_width = px(80.0);

        let error_banner = self.error.as_ref().map(|error| {
            h_flex()
                .id("error-banner")
                .w_full()
                .px_2()
                .py_1()
                .bg(cx.theme().colors().surface_background)
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .justify_between()
                .items_center()
                .child(
                    h_flex()
                        .gap_2()
                        .overflow_hidden()
                        .child(Icon::new(IconName::Warning).color(Color::Error))
                        .child(Label::new(error.clone()).color(Color::Error).single_line()),
                )
                .child(
                    IconButton::new("dismiss-error", IconName::Close)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.error = None;
                            cx.notify();
                        })),
                )
        });

        let content = if self.loading && self.graph.commits.is_empty() && false {
            let message = if self.loading {
                "Loading commits..."
            } else {
                "No commits found"
            };
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new(message).color(Color::Muted))
        } else {
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    h_flex()
                        .w_full()
                        .px_2()
                        .py_1()
                        .gap_4()
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .flex_shrink_0()
                        .child(
                            div()
                                .w(graph_width)
                                .child(Label::new("Graph").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(Label::new("Description").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .w(date_width)
                                .child(Label::new("Date").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .w(author_width)
                                .child(Label::new("Author").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .w(commit_width)
                                .child(Label::new("Commit").color(Color::Muted)),
                        ),
                )
                .child(
                    h_flex()
                        .flex_1()
                        .size_full()
                        .child(div().h_full().overflow_hidden().child(render_graph(&self)))
                        .child(
                            list(
                                self.list_state.clone(),
                                cx.processor(Self::render_list_item),
                            )
                            .flex_1()
                            .h_full()
                            .w_full(),
                        ),
                )
        };

        div()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .key_context("GitGraph")
            .track_focus(&self.focus_handle)
            .child(v_flex().size_full().children(error_banner).child(content))
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl EventEmitter<ItemEvent> for GitGraph {}

impl Focusable for GitGraph {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for GitGraph {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Git Graph".into()
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}

impl SerializableItem for GitGraph {
    fn serialized_item_kind() -> &'static str {
        "GitGraph"
    }

    fn cleanup(
        workspace_id: workspace::WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "git_graphs",
            &persistence::GIT_GRAPHS,
            cx,
        )
    }

    fn deserialize(
        project: Entity<Project>,
        _: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        if persistence::GIT_GRAPHS
            .get_git_graph(item_id, workspace_id)
            .ok()
            .is_some_and(|is_open| is_open)
        {
            let git_graph = cx.new(|cx| GitGraph::new(project, window, cx));
            Task::ready(Ok(git_graph))
        } else {
            Task::ready(Err(anyhow::anyhow!("No git graph to deserialize")))
        }
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        Some(cx.background_spawn(async move {
            persistence::GIT_GRAPHS
                .save_git_graph(item_id, workspace_id, true)
                .await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        event == &ItemEvent::UpdateTab
    }
}

mod persistence {
    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::WorkspaceDb;

    pub struct GitGraphsDb(ThreadSafeConnection);

    impl Domain for GitGraphsDb {
        const NAME: &str = stringify!(GitGraphsDb);

        const MIGRATIONS: &[&str] = (&[sql!(
            CREATE TABLE git_graphs (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                is_open INTEGER DEFAULT FALSE,

                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )]);
    }

    db::static_connection!(GIT_GRAPHS, GitGraphsDb, [WorkspaceDb]);

    impl GitGraphsDb {
        query! {
            pub async fn save_git_graph(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId,
                is_open: bool
            ) -> Result<()> {
                INSERT OR REPLACE INTO git_graphs(item_id, workspace_id, is_open)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_git_graph(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<bool> {
                SELECT is_open
                FROM git_graphs
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
