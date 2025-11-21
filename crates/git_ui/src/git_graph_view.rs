use crate::commit_view::CommitView;
use anyhow::Context as _;
use git::repository::{CommitGraph, CommitGraphEntry};
use gpui::{
    AnyElement, App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable,
    ListSizingBehavior, MouseButton, Pixels, Render, SharedString, Task, UniformListScrollHandle,
    WeakEntity, Window, actions, uniform_list,
};
use log::debug;
use project::git_store::Repository;
use std::ops::Range;
use ui::{
    Badge, Button, ButtonStyle, Icon, IconName, Label, LabelSize, ScrollAxes, Scrollbars, Tooltip,
    WithScrollbar, prelude::*,
};
use workspace::{Item, Workspace};

actions!(
    git_graph,
    [
        /// Opens a commit graph for the active repository.
        OpenGitGraph
    ]
);

const DEFAULT_GRAPH_LIMIT: usize = 400;
const GRAPH_LIMIT_STEP: usize = 400;
const MAX_GRAPH_LIMIT: usize = 2_000;
const LANE_SPACING: f32 = 16.0;
const GRAPH_PADDING: f32 = 10.0;
const NODE_SIZE: f32 = 8.0;
const LINE_WIDTH: f32 = 2.0;

#[derive(Clone)]
struct GraphRow {
    commit: CommitGraphEntry,
    lane_index: usize,
}

pub struct GitGraphView {
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    rows: Vec<GraphRow>,
    truncated: bool,
    limit: usize,
    loading: bool,
    error: Option<String>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    _load_task: Option<Task<anyhow::Result<()>>>,
}

impl GitGraphView {
    pub fn open(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let repository = workspace
            .project()
            .read(cx)
            .git_store()
            .read(cx)
            .active_repository();

        let graph = cx.new(|cx| GitGraphView::new(workspace.weak_handle(), repository, cx));
        graph.update(cx, |graph, cx| graph.load(DEFAULT_GRAPH_LIMIT, window, cx));

        workspace.add_item_to_active_pane(Box::new(graph), None, true, window, cx);
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            workspace,
            repository,
            rows: Vec::new(),
            truncated: false,
            limit: DEFAULT_GRAPH_LIMIT,
            loading: false,
            error: None,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
            _load_task: None,
        }
    }

    fn load(&mut self, limit: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.repository.clone() else {
            self.loading = false;
            self.error = Some("No active git repository".to_string());
            cx.notify();
            return;
        };

        self.loading = true;
        self.error = None;
        cx.notify();

        let grid_limit = limit.clamp(1, MAX_GRAPH_LIMIT);
        let this = cx.entity();
        let repo_path = repository
            .read(cx)
            .work_directory_abs_path
            .to_string_lossy()
            .to_string();
        debug!("git_graph: loading commit graph (limit {grid_limit}) for repo {repo_path}");
        self._load_task = Some(window.spawn(cx, async move |cx| {
            let graph_result = repository
                .update(cx, |repository, _| repository.commit_graph(grid_limit))
                .context("no repository")?
                .await;

            let _ = this.update(cx, |this, cx| {
                this.loading = false;
                match graph_result {
                    Ok(Ok(graph)) => {
                        this.limit = grid_limit;
                        this.truncated = graph.truncated;
                        this.rows = build_rows(&graph);
                        debug!(
                            "git_graph: loaded {} commits (truncated={})",
                            this.rows.len(),
                            this.truncated
                        );
                        if this.rows.is_empty() {
                            this.error = Some("No commits found in this repository".to_string());
                        }
                    }
                    Ok(Err(err)) => {
                        this.rows.clear();
                        this.error = Some(format!("Failed to load git graph: {err}"));
                        debug!("git_graph: failed to load graph: {err:?}");
                    }
                    Err(err) => {
                        this.rows.clear();
                        this.error = Some(format!("Failed to load git graph: {err}"));
                        debug!("git_graph: failed to load graph: {err:?}");
                    }
                }
                cx.notify();
                anyhow::Ok(())
            })?;

            anyhow::Ok(())
        }));
    }

    fn render_row(
        &self,
        row: &GraphRow,
        graph_width: Pixels,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let lane_x = GRAPH_PADDING + row.lane_index as f32 * LANE_SPACING;
        let colors = cx.theme().colors();
        let lane_colors = [
            colors.icon_accent,
            colors.text_accent,
            colors.text,
            colors.text_muted,
            colors.link_text_hover,
        ];
        let lane_color = lane_colors[row.lane_index % lane_colors.len()];

        let commit = &row.commit;
        let relative_time: SharedString = commit.relative_time.as_ref().trim().to_string().into();

        let commit_oid = commit.oid.clone();

        h_flex()
            .w_full()
            .h(px(26.0))
            .gap_3()
            .px_3()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .when(self.loading, |row| row.opacity(0.7))
            .child(
                div()
                    .w(graph_width)
                    .relative()
                    .h_full()
                    .child(
                        div()
                            .absolute()
                            .left(px(lane_x - LINE_WIDTH / 2.0))
                            .top_0()
                            .bottom_0()
                            .w(px(LINE_WIDTH))
                            .rounded_full()
                            .bg(lane_color.opacity(0.4)),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(px(lane_x - (NODE_SIZE / 2.0)))
                            .top(px(9.0))
                            .w(px(NODE_SIZE))
                            .h(px(NODE_SIZE))
                            .rounded_full()
                            .bg(lane_color),
                    ),
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_w(px(0.0)) // Allow shrinking
                    .gap_2()
                    .items_center()
                    .overflow_hidden() // Clip content that is too long
                    .child(
                        div()
                            .flex_shrink() // Allow text to shrink
                            .min_w(px(0.0))
                            .overflow_hidden()
                            .child(
                                Label::new(if commit.summary.is_empty() {
                                    "(no commit message)".into()
                                } else {
                                    commit.summary.clone()
                                })
                                .single_line(),
                            ),
                    )
                    .child(
                        div()
                            .flex_none() // Don't shrink badges
                            .child(self.render_decorations(&commit)),
                    ),
            )
            .child(
                div()
                    .w(px(120.0))
                    .flex_none() // Prevent shrinking
                    .child(
                        Label::new(relative_time)
                            .color(Color::Muted)
                            .size(LabelSize::Small)
                            .single_line(),
                    ),
            )
            .child(
                div()
                    .w(px(120.0))
                    .flex_none() // Prevent shrinking
                    .child(
                        Label::new(commit.author.clone())
                            .color(Color::Muted)
                            .size(LabelSize::Small)
                            .single_line(),
                    ),
            )
            .child(
                div()
                    .w(px(80.0)) // Match header width
                    .flex_none() // Prevent shrinking
                    .child(
                        Label::new(commit.oid.chars().take(7).collect::<String>())
                            .color(Color::Muted)
                            .size(LabelSize::Small)
                            .single_line(),
                    ),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    this.open_commit(commit_oid.clone(), window, cx);
                }),
            )
            .into_any_element()
    }

    fn render_decorations(&self, commit: &CommitGraphEntry) -> AnyElement {
        let mut badges: Vec<AnyElement> = Vec::new();

        // Helper to create a compact icon badge with tooltip
        let make_badge = |icon: IconName, text: String, color: Color| {
            let id = SharedString::from(format!("badge-{}", text));
            div()
                .child(
                    IconButton::new(id, icon)
                        .icon_size(IconSize::Small)
                        .icon_color(color)
                        .style(ButtonStyle::Transparent)
                        .tooltip(move |window, cx| Tooltip::text(text.clone())(window, cx)),
                )
                .into_any_element()
        };

        if let Some(head) = commit.decorations.head.as_ref() {
            badges.push(make_badge(
                IconName::GitBranch,
                format!("HEAD -> {head}"),
                Color::Accent,
            ));
        }

        badges.extend(
            commit
                .decorations
                .local_branches
                .iter()
                .map(|branch| make_badge(IconName::GitBranch, branch.to_string(), Color::Created)),
        );

        badges.extend(
            commit
                .decorations
                .tags
                .iter()
                .map(|tag| make_badge(IconName::Hash, tag.to_string(), Color::Warning)),
        );

        badges.extend(
            commit.decorations.remote_branches.iter().map(|branch| {
                make_badge(IconName::CloudDownload, branch.to_string(), Color::Muted)
            }),
        );

        if badges.is_empty() {
            div().into_any_element()
        } else {
            h_flex()
                .gap_1()
                .flex_wrap()
                .children(badges)
                .into_any_element()
        }
    }

    fn open_commit(&self, sha: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repo) = self.repository.clone() else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        CommitView::open(
            sha.to_string(),
            repo.downgrade(),
            workspace.downgrade(),
            None,
            window,
            cx,
        );
    }
}

impl Item for GitGraphView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranchAlt).color(Color::Muted))
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Git Graph".into())
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Git Graph".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Git Graph Opened")
    }
}

impl Focusable for GitGraphView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for GitGraphView {}

impl Render for GitGraphView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content: AnyElement = if let Some(error) = &self.error {
            v_flex()
                .justify_center()
                .items_center()
                .h_full()
                .child(Label::new(error.clone()).color(Color::Error))
                .into_any_element()
        } else if self.loading && self.rows.is_empty() {
            v_flex()
                .justify_center()
                .items_center()
                .h_full()
                .child(Label::new("Loading git graph…").color(Color::Muted))
                .into_any_element()
        } else if self.rows.is_empty() {
            v_flex()
                .justify_center()
                .items_center()
                .h_full()
                .gap_2()
                .child(Label::new("No commits found").color(Color::Muted))
                .child(
                    Button::new("git-graph-retry", "Reload").on_click(cx.listener(
                        |this, _, window, cx| {
                            this.load(DEFAULT_GRAPH_LIMIT, window, cx);
                        },
                    )),
                )
                .into_any_element()
        } else {
            let max_lanes = self
                .rows
                .iter()
                .map(|row| row.lane_index + 1)
                .max()
                .unwrap_or(1);
            let graph_width = px(GRAPH_PADDING * 2.0 + max_lanes as f32 * LANE_SPACING);
            let list = uniform_list(
                "git-graph-rows",
                self.rows.len(),
                cx.processor(move |this, range: Range<usize>, window, cx| {
                    let mut items = Vec::with_capacity(range.end - range.start);
                    for ix in range {
                        if let Some(row) = this.rows.get(ix) {
                            items.push(this.render_row(row, graph_width, window, cx));
                        }
                    }
                    items
                }),
            )
            .size_full()
            .with_sizing_behavior(ListSizingBehavior::Auto)
            .track_scroll(self.scroll_handle.clone());

            div()
                .size_full()
                .custom_scrollbars(
                    Scrollbars::new(ScrollAxes::Both)
                        .tracked_scroll_handle(self.scroll_handle.clone()),
                    window,
                    cx,
                )
                .child(list)
                .into_any_element()
        };

        let colors = cx.theme().colors().clone();
        let panel_bg = colors.panel_background;
        let repo_label = self
            .repository
            .as_ref()
            .and_then(|repo| {
                repo.read(cx)
                    .branch
                    .as_ref()
                    .map(|branch| branch.name().to_string())
            })
            .unwrap_or_else(|| "No repository".to_string());
        let truncated_badge = self.truncated.then(|| {
            Badge::new(format!("Showing {}+", self.limit))
                .icon(IconName::CountdownTimer)
                .into_any_element()
        });

        // Calculate graph width for the header to match the rows
        let max_lanes = self
            .rows
            .iter()
            .map(|row| row.lane_index + 1)
            .max()
            .unwrap_or(1);
        let graph_width = px(GRAPH_PADDING * 2.0 + max_lanes as f32 * LANE_SPACING);

        // New Toolbar Area
        let toolbar = h_flex()
            .w_full()
            .justify_between()
            .items_center()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(colors.border)
            .bg(colors.panel_background)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Label::new(repo_label).weight(gpui::FontWeight::BOLD))
                    .when_some(truncated_badge, |row, badge| row.child(badge)),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("git-graph-refresh", "Refresh")
                            .style(ButtonStyle::Transparent)
                            .icon(IconName::RotateCcw)
                            .icon_size(IconSize::Small)
                            .disabled(self.loading)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.load(this.limit, window, cx);
                            }))
                            .tooltip(|window, cx| Tooltip::text("Refresh")(window, cx)),
                    )
                    .child(
                        Button::new("git-graph-more", "More")
                            .style(ButtonStyle::Transparent)
                            .icon(IconName::ArrowDown)
                            .icon_size(IconSize::Small)
                            .disabled(self.loading || self.limit >= MAX_GRAPH_LIMIT)
                            .on_click(cx.listener(|this, _, window, cx| {
                                let next = (this.limit + GRAPH_LIMIT_STEP).min(MAX_GRAPH_LIMIT);
                                this.load(next, window, cx);
                            }))
                            .tooltip(|window, cx| {
                                Tooltip::text(format!("Load more (max {})", MAX_GRAPH_LIMIT))(
                                    window, cx,
                                )
                            }),
                    ),
            );

        let table_header = h_flex()
            .w_full()
            .items_center()
            .gap_3()
            .px_3()
            .py_1() // Reduced padding for header
            .border_b_1()
            .border_color(colors.border)
            .bg(colors.editor_background) // Slightly different bg to distinguish from toolbar
            .child(
                div().w(graph_width).child(
                    Label::new("Graph")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                ),
            )
            .child(
                h_flex().flex_1().min_w(px(0.0)).child(
                    Label::new("Description")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                ),
            )
            .child(
                div().w(px(120.0)).child(
                    Label::new("Date")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                ),
            )
            .child(
                div().w(px(120.0)).child(
                    Label::new("Author")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                ),
            )
            .child(
                div().w(px(80.0)).child(
                    Label::new("Commit")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                ),
            );

        v_flex()
            .track_focus(&self.focus_handle)
            .key_context("GitGraph")
            .bg(panel_bg)
            .size_full()
            .child(toolbar)
            .child(table_header)
            .child(
                div()
                    .flex_1()
                    .min_h(px(0.0))
                    .overflow_hidden()
                    .child(content),
            )
    }
}

fn build_rows(graph: &CommitGraph) -> Vec<GraphRow> {
    let mut rows = Vec::with_capacity(graph.commits.len());
    let mut active_lanes: Vec<Option<SharedString>> = Vec::new();

    for commit in &graph.commits {
        let mut assigned_lane = None;

        // 1. Find if any existing lanes connect to this commit
        // We need to check all lanes to merge them if they converge on this commit
        for (i, lane) in active_lanes.iter_mut().enumerate() {
            if lane.as_ref() == Some(&commit.oid) {
                if assigned_lane.is_none() {
                    assigned_lane = Some(i);
                    // Update this lane to follow the first parent
                    *lane = commit.parents.get(0).cloned();
                } else {
                    // Merge this lane into the assigned one (close it)
                    *lane = None;
                }
            }
        }

        // 2. If no lane connected, start a new one (branch tip)
        let lane_index = if let Some(i) = assigned_lane {
            i
        } else {
            // Find empty slot or push
            if let Some(i) = active_lanes.iter().position(|l| l.is_none()) {
                active_lanes[i] = commit.parents.get(0).cloned();
                i
            } else {
                active_lanes.push(commit.parents.get(0).cloned());
                active_lanes.len() - 1
            }
        };

        // 3. Handle merge commits (multiple parents)
        // The first parent was already handled above (assigned to lane_index).
        // Secondary parents need to spawn new lanes.
        if commit.parents.len() > 1 {
            for parent in commit.parents.iter().skip(1) {
                if let Some(free_idx) = active_lanes.iter().position(|l| l.is_none()) {
                    active_lanes[free_idx] = Some(parent.clone());
                } else {
                    active_lanes.push(Some(parent.clone()));
                }
            }
        }

        // 4. Clean up trailing empty lanes
        while active_lanes.last().is_some_and(|l| l.is_none()) {
            active_lanes.pop();
        }

        rows.push(GraphRow {
            commit: commit.clone(),
            lane_index,
        });
    }

    rows
}
