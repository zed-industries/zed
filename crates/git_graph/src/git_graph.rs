mod graph;
mod graph_rendering;

use anyhow::Context as _;
use git;
use git_ui::commit_view::CommitView;
use gpui::{
    App, ClickEvent, Context, Corner, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, ListAlignment, ListState, ParentElement, Pixels, Point, Render,
    SharedString, Styled, Subscription, Task, WeakEntity, Window, actions, anchored, deferred,
    list, px,
};
use project::Project;
use project::git_store::{GitStoreEvent, Repository};
use settings::Settings;
use std::path::PathBuf;
use theme::ThemeSettings;
use ui::prelude::*;
use ui::{ContextMenu, Tooltip};
use util::ResultExt;
use workspace::Workspace;
use workspace::item::{Item, ItemEvent};

use graph_rendering::{
    BRANCH_COLORS, BadgeType, parse_refs_to_badges, render_graph_cell, render_graph_continuation,
};

use crate::graph::CommitEntry;

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
    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenGitGraph, window, cx| {
            let project = workspace.project().clone();
            let workspace_handle = workspace.weak_handle();
            let git_graph = cx.new(|cx| GitGraph::new(project, workspace_handle, window, cx));
            workspace.add_item_to_active_pane(Box::new(git_graph), None, true, window, cx);
        });
    })
    .detach();
}

#[derive(Clone)]
pub enum InputModalKind {
    CreateBranch { sha: String },
    CreateTag { sha: String },
    RenameBranch { old_name: String },
    CheckoutRemoteBranch { remote_branch: String },
}

pub struct GitGraph {
    focus_handle: FocusHandle,
    graph: crate::graph::GitGraph,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    max_lanes: usize,
    loading: bool,
    error: Option<SharedString>,
    _load_task: Option<Task<()>>,
    selected_commit: Option<usize>,
    expanded_commit: Option<usize>,
    expanded_files: Vec<ChangedFile>,
    loading_files: bool,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    work_dir: Option<PathBuf>,
    row_height: Pixels,
    list_state: ListState,
    _subscriptions: Vec<Subscription>,
}

#[derive(Clone, Debug)]
pub struct ChangedFile {
    pub path: String,
    pub status: FileStatus,
}

#[derive(Clone, Debug)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Unknown,
}

impl GitGraph {
    pub fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();

        let git_store = project.read(cx).git_store().clone();
        let git_store_subscription = cx.subscribe(&git_store, |this, _, event, cx| match event {
            GitStoreEvent::RepositoryUpdated(_, _, _)
            | GitStoreEvent::RepositoryAdded
            | GitStoreEvent::RepositoryRemoved(_) => {
                this.load_data(cx);
            }
            _ => {}
        });

        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        let row_height = font_size + px(10.0);

        let list_state = ListState::new(0, ListAlignment::Top, px(500.0));

        let mut this = GitGraph {
            focus_handle,
            project,
            workspace,
            graph: crate::graph::GitGraph::new(),
            max_lanes: 0,
            loading: true,
            error: None,
            _load_task: None,
            selected_commit: None,
            expanded_commit: None,
            expanded_files: Vec::new(),
            loading_files: false,
            context_menu: None,
            work_dir: None,
            row_height,
            list_state,
            _subscriptions: vec![git_store_subscription],
        };

        this.load_data(cx);
        this
    }

    fn get_selected_commit(&self) -> Option<&CommitEntry> {
        self.selected_commit
            .and_then(|idx| self.graph.commits.get(idx))
    }

    fn get_repository(&self, cx: &App) -> Option<Entity<Repository>> {
        let git_store = self.project.read(cx).git_store();
        git_store.read(cx).repositories().values().next().cloned()
    }

    fn open_commit_view(
        &mut self,
        file_path: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(commit) = self.get_selected_commit() else {
            return;
        };
        let sha = commit.data.sha.clone();

        let Some(repository) = self.get_repository(cx) else {
            self.error = Some("No repository found".into());
            cx.notify();
            return;
        };

        let file_filter = file_path.and_then(|p| git::repository::RepoPath::new(&p).ok());

        CommitView::open(
            sha.to_string(),
            repository.downgrade(),
            self.workspace.clone(),
            None,
            file_filter,
            window,
            cx,
        );
    }

    fn load_data(&mut self, cx: &mut Context<Self>) {
        let project = self.project.clone();
        self.loading = true;
        self.error = None;
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

            let result = crate::graph::load_commits(worktree_path.clone()).await;

            this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok(commits) => {
                        this.graph.add_commits(commits);
                        let commit_count = this.graph.commits.len();
                        this.max_lanes = this.graph.max_lanes;
                        this.work_dir = Some(worktree_path);
                        this.list_state.reset(commit_count);
                    }
                    Err(e) => {
                        this.error = Some(format!("{:?}", e).into());
                    }
                };

                cx.notify();
            })
            .log_err();
        }));
    }

    fn render_badges(
        &self,
        refs: &[SharedString],
        color_idx: usize,
        commit_idx: usize,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let badges = parse_refs_to_badges(refs);
        let branch_color = BRANCH_COLORS[color_idx % BRANCH_COLORS.len()];
        let tag_color = gpui::hsla(140.0 / 360.0, 0.55, 0.45, 1.0);
        let hover_bg = cx.theme().colors().ghost_element_hover;
        let accent_color = cx.theme().colors().border_focused;

        h_flex()
            .gap_1()
            .flex_shrink_0()
            .children(
                badges
                    .into_iter()
                    .take(5)
                    .enumerate()
                    .map(|(badge_idx, badge)| match badge {
                        BadgeType::Tag(name) => h_flex()
                            .gap_0p5()
                            .px_1()
                            .rounded_sm()
                            .child(
                                Icon::new(IconName::Hash)
                                    .size(IconSize::Small)
                                    .color(Color::Custom(tag_color)),
                            )
                            .child(
                                Label::new(name)
                                    .size(LabelSize::Default)
                                    .color(Color::Default),
                            )
                            .into_any_element(),
                        BadgeType::CurrentBranch(name, has_origin) => h_flex()
                            .id(ElementId::NamedInteger(
                                SharedString::from(format!(
                                    "badge-current-{}-{}",
                                    commit_idx, badge_idx
                                )),
                                commit_idx as u64,
                            ))
                            .gap_0p5()
                            .px_1()
                            .rounded_sm()
                            .border_1()
                            .border_color(accent_color)
                            .cursor_pointer()
                            .hover(move |style| style.bg(hover_bg))
                            .child(
                                Icon::new(IconName::GitBranch)
                                    .size(IconSize::Small)
                                    .color(Color::Custom(branch_color)),
                            )
                            .child(
                                Label::new(name)
                                    .size(LabelSize::Default)
                                    .color(Color::Default),
                            )
                            .when(has_origin, |el| {
                                el.child(
                                    Label::new("origin")
                                        .size(LabelSize::Default)
                                        .color(Color::Muted),
                                )
                            })
                            .into_any_element(),
                        BadgeType::LocalBranch(name, has_origin) => h_flex()
                            .id(ElementId::NamedInteger(
                                SharedString::from(format!(
                                    "badge-local-{}-{}",
                                    commit_idx, badge_idx
                                )),
                                commit_idx as u64,
                            ))
                            .gap_0p5()
                            .px_1()
                            .rounded_sm()
                            .cursor_pointer()
                            .hover(move |style| style.bg(hover_bg))
                            .child(
                                Icon::new(IconName::GitBranch)
                                    .size(IconSize::Small)
                                    .color(Color::Custom(branch_color)),
                            )
                            .child(
                                Label::new(name)
                                    .size(LabelSize::Default)
                                    .color(Color::Default),
                            )
                            .when(has_origin, |el| {
                                el.child(
                                    Label::new("origin")
                                        .size(LabelSize::Default)
                                        .color(Color::Muted),
                                )
                            })
                            .into_any_element(),
                        BadgeType::RemoteBranch(name) => h_flex()
                            .id(ElementId::NamedInteger(
                                SharedString::from(format!(
                                    "badge-remote-{}-{}",
                                    commit_idx, badge_idx
                                )),
                                commit_idx as u64,
                            ))
                            .gap_0p5()
                            .px_1()
                            .rounded_sm()
                            .cursor_pointer()
                            .hover(move |style| style.bg(hover_bg))
                            .child(
                                Icon::new(IconName::GitBranch)
                                    .size(IconSize::Small)
                                    .color(Color::Custom(branch_color)),
                            )
                            .child(
                                Label::new(name)
                                    .size(LabelSize::Default)
                                    .color(Color::Muted),
                            )
                            .into_any_element(),
                    }),
            )
    }

    fn render_list_item(
        &mut self,
        idx: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        let row_height = self.row_height;
        let graph_width = px(16.0) * (self.max_lanes.max(2) as f32) + px(24.0);
        let date_width = px(140.0);
        let author_width = px(120.0);
        let commit_width = px(80.0);

        self.render_commit_row_inline(
            idx,
            row_height,
            graph_width,
            date_width,
            author_width,
            commit_width,
            cx,
        )
    }

    fn render_commit_row_inline(
        &self,
        idx: usize,
        row_height: Pixels,
        graph_width: Pixels,
        date_width: Pixels,
        author_width: Pixels,
        commit_width: Pixels,
        cx: &Context<Self>,
    ) -> gpui::AnyElement {
        let is_expanded = self.expanded_commit == Some(idx);
        let row = self.render_commit_row(
            idx,
            row_height,
            graph_width,
            date_width,
            author_width,
            commit_width,
            cx,
        );

        if is_expanded {
            v_flex()
                .w_full()
                .child(row)
                .child(self.render_inline_expansion(idx, graph_width, cx))
                .into_any_element()
        } else {
            row
        }
    }

    fn render_commit_row(
        &self,
        idx: usize,
        row_height: Pixels,
        graph_width: Pixels,
        date_width: Pixels,
        author_width: Pixels,
        commit_width: Pixels,
        cx: &Context<Self>,
    ) -> gpui::AnyElement {
        let Some(commit) = self.graph.commits.get(idx) else {
            return div().into_any_element();
        };

        let subject: SharedString = commit.data.subject.clone().into();
        let author_name: SharedString = commit.data.author_name.clone().into();
        let short_sha: SharedString = commit.data.sha.display_short().into();
        let formatted_time: SharedString = commit.data.commit_timestamp.clone().into();
        let refs = commit.data.ref_names.clone();
        let lane = commit.lane;
        let lines = commit.lines.clone();
        let color_idx = commit.color_idx;

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
                div()
                    .w(graph_width)
                    .h_full()
                    .flex_shrink_0()
                    .child(render_graph_cell(
                        lane,
                        lines,
                        color_idx,
                        row_height,
                        graph_width,
                    )),
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_w(px(0.0))
                    .gap_2()
                    .overflow_hidden()
                    .items_center()
                    .when(!refs.is_empty(), |el| {
                        el.child(self.render_badges(&refs, color_idx, idx, cx))
                    })
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
            .into_any_element()
    }

    fn render_inline_expansion(
        &self,
        idx: usize,
        graph_width: Pixels,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let Some(commit) = self.graph.commits.get(idx) else {
            return div().into_any_element();
        };

        let commit_sha = commit.data.sha.clone();
        let parents = commit.data.parents.clone();
        let author = commit.data.author_name.clone();
        let subject = commit.data.subject.clone();
        let formatted_time = commit.data.commit_timestamp.clone();
        let lines = commit.lines.clone();
        let loading_files = self.loading_files;
        let expanded_files = &self.expanded_files;

        h_flex()
            .id(ElementId::NamedInteger(
                "expanded-details".into(),
                idx as u64,
            ))
            .w_full()
            .min_h(px(120.0))
            .px_2()
            .gap_4()
            .bg(cx.theme().colors().background)
            .flex_shrink_0()
            .child(
                div()
                    .w(graph_width)
                    .h_full()
                    .flex_shrink_0()
                    .child(render_graph_continuation(lines, graph_width)),
            )
            .child(
                h_flex()
                    .flex_1()
                    .h_full()
                    .items_start()
                    .child(
                        v_flex()
                            .w(px(400.0))
                            .p_2()
                            .gap_0p5()
                            .border_r_1()
                            .border_color(cx.theme().colors().border)
                            .child(
                                h_flex()
                                    .w_full()
                                    .h(px(28.0))
                                    .pb_1()
                                    .mb_1()
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                                    .items_center()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                Icon::new(IconName::Info)
                                                    .color(Color::Muted)
                                                    .size(IconSize::Small),
                                            )
                                            .child(
                                                Label::new("Info")
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            ),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Label::new("Commit:")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        Label::new(commit_sha.to_string())
                                            .size(LabelSize::Small)
                                            .color(Color::Accent),
                                    ),
                            )
                            .when(!parents.is_empty(), |el| {
                                let parent_str = parents
                                    .iter()
                                    .map(|parent| parent.display_short())
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                el.child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Label::new("Parents:")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new(parent_str)
                                                .size(LabelSize::Small)
                                                .color(Color::Accent),
                                        ),
                                )
                            })
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Label::new("Author:")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(Label::new(author).size(LabelSize::Small)),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Label::new("Date:")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(Label::new(formatted_time).size(LabelSize::Small)),
                            )
                            .child(div().h_2())
                            .child(Label::new(subject).size(LabelSize::Small)),
                    )
                    .child(
                        v_flex()
                            .id("file-list-scroll")
                            .flex_1()
                            .h_full()
                            .p_2()
                            .gap_0p5()
                            .overflow_y_scroll()
                            .child(
                                h_flex()
                                    .w_full()
                                    .h(px(28.0))
                                    .pb_1()
                                    .mb_1()
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                Icon::new(IconName::FileTree)
                                                    .color(Color::Muted)
                                                    .size(IconSize::Small),
                                            )
                                            .child(
                                                Label::new(format!(
                                                    "{} files",
                                                    expanded_files.len()
                                                ))
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                            ),
                                    )
                                    .child(
                                        Button::new("view-diff", "View Diff")
                                            .style(ButtonStyle::Filled)
                                            .label_size(LabelSize::Small)
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.open_commit_view(None, window, cx);
                                            })),
                                    ),
                            )
                            .when(loading_files, |el| {
                                el.child(
                                    div().w_full().py_2().child(
                                        Label::new("Loading...")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                )
                            })
                            .when(!loading_files && expanded_files.is_empty(), |el| {
                                el.child(
                                    div().w_full().py_2().child(
                                        Label::new("No files changed")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                )
                            })
                            .when(!loading_files && !expanded_files.is_empty(), |el| {
                                el.children(expanded_files.iter().enumerate().map(
                                    |(file_idx, file)| {
                                        let file_path = file.path.clone();
                                        let status_icon = match file.status {
                                            FileStatus::Added => IconName::Plus,
                                            FileStatus::Modified => IconName::Pencil,
                                            FileStatus::Deleted => IconName::Trash,
                                            FileStatus::Renamed => IconName::Replace,
                                            FileStatus::Copied => IconName::Copy,
                                            FileStatus::Unknown => IconName::File,
                                        };
                                        let status_color = match file.status {
                                            FileStatus::Added => Color::Created,
                                            FileStatus::Modified => Color::Modified,
                                            FileStatus::Deleted => Color::Deleted,
                                            _ => Color::Muted,
                                        };

                                        h_flex()
                                            .id(ElementId::NamedInteger(
                                                "file-row".into(),
                                                file_idx as u64,
                                            ))
                                            .w_full()
                                            .px_1()
                                            .py_0p5()
                                            .gap_2()
                                            .items_center()
                                            .cursor_pointer()
                                            .hover(|style| {
                                                style.bg(cx.theme().colors().ghost_element_hover)
                                            })
                                            .rounded_sm()
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.open_commit_view(
                                                    Some(file_path.clone()),
                                                    window,
                                                    cx,
                                                );
                                            }))
                                            .child(
                                                Icon::new(status_icon)
                                                    .color(status_color)
                                                    .size(IconSize::Small),
                                            )
                                            .child(
                                                Label::new(file.path.clone())
                                                    .size(LabelSize::Small)
                                                    .single_line(),
                                            )
                                    },
                                ))
                            }),
                    ),
            )
            .child(
                div()
                    .w(px(24.0))
                    .h_full()
                    .flex_shrink_0()
                    .flex()
                    .items_start()
                    .justify_center()
                    .pt_1()
                    .child(
                        IconButton::new("close-expanded", IconName::Close)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.expanded_commit = None;
                                this.expanded_files.clear();
                                cx.notify();
                            })),
                    ),
            )
            .into_any_element()
    }
}

impl Render for GitGraph {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let graph_width = px(16.0) * (self.max_lanes.max(2) as f32) + px(24.0);
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

        let content = if self.loading {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("Loading commits...").color(Color::Muted))
        } else if self.graph.commits.is_empty() {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("No commits found").color(Color::Muted))
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
                    list(
                        self.list_state.clone(),
                        cx.processor(Self::render_list_item),
                    )
                    .flex_1()
                    .w_full(),
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
