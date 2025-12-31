use anyhow::Result;
use git::repository::{CommitGraph, CommitGraphNode, GitRef};
use git::{GitHostingProviderRegistry, GitRemote, parse_git_remote_url};
use gpui::{
    AnyElement, AnyEntity, App, Background, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Hsla, IntoElement, PathBuilder, Render, ScrollStrategy, Task, UniformListScrollHandle, 
    WeakEntity, Window, actions, canvas, point, px, uniform_list,
};
use project::{
    Project, ProjectPath,
    git_store::{GitStore, Repository},
};
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use time::OffsetDateTime;
use ui::{Divider, ListItem, WithScrollbar, prelude::*};
use util::ResultExt;
use workspace::{
    Item, Workspace,
    item::{ItemEvent, SaveOptions},
};

use crate::commit_view::CommitView;

actions!(git, [OpenCommitGraph, LoadMoreGraphHistory]);

const PAGE_SIZE: usize = 100;
const LANE_WIDTH: f32 = 20.0;
const NODE_RADIUS: f32 = 5.0;

/// Colors for different lanes in the graph.
const LANE_COLORS: &[u32] = &[
    0x4EC9B0, // Teal
    0xDCDCAA, // Yellow
    0x9CDCFE, // Light Blue
    0xC586C0, // Purple
    0xCE9178, // Orange
    0x6A9955, // Green
    0xD16969, // Red
    0x569CD6, // Blue
];

/// Represents a lane (vertical column) in the git graph.
#[derive(Clone, Debug)]
pub struct GraphLane {
    pub color: Hsla,
    pub active: bool,
}

/// Represents a merge connection in the graph (curved line from child to parent).
#[derive(Clone, Debug)]
pub struct MergeConnection {
    /// Row index where the merge occurs (child commit row)
    pub child_row: usize,
    /// Lane of the child commit
    pub child_lane: usize,
    /// Row index of the parent commit
    pub parent_row: usize,
    /// Lane of the parent commit
    pub parent_lane: usize,
}

/// Represents a branch merging into another lane.
#[derive(Clone, Debug)]
pub struct BranchTermination {
    pub row: usize,
    pub lane: usize,
    pub connect_to: usize,
}

/// Layout information for the git graph.
#[derive(Clone, Debug, Default)]
pub struct GraphLayout {
    /// Maps commit SHA to (lane_index, row_index).
    pub positions: HashMap<String, (usize, usize)>,
    /// Number of lanes needed.
    pub lane_count: usize,
    /// Active lanes per row - lanes_per_row[row] contains set of active lane indices
    pub lanes_per_row: Vec<HashSet<usize>>,
    /// Merge connections - curved lines connecting merge commits to secondary parents
    pub merge_connections: Vec<MergeConnection>,
    /// Locations where branches merge into another lane
    pub branch_terminations: Vec<BranchTermination>,
    /// Names of branches associated with each lane (for tooltips)
    pub lane_names: HashMap<usize, String>,
}

impl GraphLayout {
    /// Compute the layout for a list of commits.
    ///
    /// Commits are processed in topological order (newest to oldest).
    /// Each lane tracks the SHA it expects to see next (the parent of the commit that used it).
    /// For merge commits, secondary parents create merge connections.
    pub fn compute(commits: &[CommitGraphNode]) -> Self {
        let mut positions = HashMap::new();
        // Each lane tracks the SHA it expects to see next (or None if empty)
        let mut expected_shas: Vec<Option<String>> = Vec::new();
        // Track which lanes are active at each row
        let mut lanes_per_row: Vec<HashSet<usize>> = Vec::new();
        // Merge connections
        let mut merge_connections = Vec::new();
        // Branch terminations (converging lanes)
        let mut branch_terminations = Vec::new();
        // Lane names (from refs)
        let mut lane_names: HashMap<usize, String> = HashMap::new();
        // Map SHA to row for merge connection lookup
        let sha_to_row: HashMap<String, usize> = commits
            .iter()
            .enumerate()
            .map(|(row, c)| (c.sha.to_string(), row))
            .collect();

        for (row, commit) in commits.iter().enumerate() {
            let sha = commit.sha.to_string();
            let first_parent = commit.parent_shas.first().map(|s| s.to_string());

            // Find a lane that is expecting this commit's SHA
            let lane = expected_shas
                .iter()
                .position(|l| l.as_ref() == Some(&sha))
                .unwrap_or_else(|| {
                    // No lane is expecting this commit, find an empty lane or create a new one.
                    expected_shas
                        .iter()
                        .position(|l| l.is_none())
                        .unwrap_or_else(|| {
                            expected_shas.push(None);
                            expected_shas.len() - 1
                        })
                });

            // Handle converging lanes (branches merging into this one)
            let mut closing_lanes = Vec::new();
            for (i, expected) in expected_shas.iter_mut().enumerate() {
                if i != lane && expected.as_ref() == Some(&sha) {
                    closing_lanes.push(i);
                    *expected = None;
                    branch_terminations.push(BranchTermination {
                        row,
                        lane: i,
                        connect_to: lane,
                    });
                }
            }

            // Ensure lane exists
            if lane >= expected_shas.len() {
                expected_shas.resize(lane + 1, None);
            }

            // Update the lane to expect this commit's first parent SHA
            // If this commit has no parent, clear the lane
            expected_shas[lane] = first_parent.clone();
            positions.insert(sha.clone(), (lane, row));

            // Assign branch name to lane from refs (first local branch wins)
            if !lane_names.contains_key(&lane) {
                for r in &commit.refs {
                    if let git::repository::GitRef::LocalBranch(name) = r {
                        lane_names.insert(lane, name.to_string());
                        break;
                    }
                }
            }

            // Track active lanes for this row
            let mut active_lanes = HashSet::new();
            for (lane_idx, expected) in expected_shas.iter().enumerate() {
                if expected.is_some() {
                    active_lanes.insert(lane_idx);
                }
            }
            // Add lanes that technically closed at this row, so we draw the incoming vertical line
            for &l in &closing_lanes {
                active_lanes.insert(l);
            }
            
            // Handle merge commits - secondary parents create merge connections
            // We do this BEFORE adding to lanes_per_row so that new lanes are included
            for parent_sha in commit.parent_shas.iter().skip(1) {
                let parent_sha_str = parent_sha.to_string();
                
                // Find or create a lane for this secondary parent
                let parent_lane = expected_shas
                    .iter()
                    .position(|l| l.as_ref() == Some(&parent_sha_str))
                    .unwrap_or_else(|| {
                        // Allocate a new lane for the secondary parent
                        let new_lane = expected_shas
                            .iter()
                            .position(|l| l.is_none())
                            .unwrap_or_else(|| {
                                expected_shas.push(None);
                                expected_shas.len() - 1
                            });
                        expected_shas[new_lane] = Some(parent_sha_str.clone());
                        new_lane
                    });

                // Add this new lane to active lanes for current row so vertical line starts here
                active_lanes.insert(parent_lane);

                // If we know where the parent is, create a merge connection
                if let Some(&parent_row) = sha_to_row.get(&parent_sha_str) {
                    merge_connections.push(MergeConnection {
                        child_row: row,
                        child_lane: lane,
                        parent_row,
                        parent_lane,
                    });
                }
            }

            // Also add the current commit's lane
            active_lanes.insert(lane);
            lanes_per_row.push(active_lanes);
        }

        let lane_count = expected_shas.len().max(1);
        Self {
            positions,
            lane_count,
            lanes_per_row,
            merge_connections,
            branch_terminations,
            lane_names,
        }
    }

    /// Get the color for a lane.
    pub fn lane_color(lane: usize) -> Hsla {
        let color_u32 = LANE_COLORS[lane % LANE_COLORS.len()];
        let r = ((color_u32 >> 16) & 0xFF) as f32 / 255.0;
        let g = ((color_u32 >> 8) & 0xFF) as f32 / 255.0;
        let b = (color_u32 & 0xFF) as f32 / 255.0;
        Hsla::from(gpui::Rgba { r, g, b, a: 1.0 })
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_workspace, _: &OpenCommitGraph, _window, _cx| {});
        workspace.register_action(|_workspace, _: &LoadMoreGraphHistory, _window, _cx| {});
    })
    .detach();
}

pub struct GitGraphView {
    graph: CommitGraph,
    layout: GraphLayout,
    repository: WeakEntity<Repository>,
    git_store: WeakEntity<GitStore>,
    workspace: WeakEntity<Workspace>,
    remote: Option<GitRemote>,
    selected_entry: Option<usize>,
    scroll_handle: UniformListScrollHandle,
    focus_handle: FocusHandle,
    loading_more: bool,
}

impl GitGraphView {
    pub fn open(
        git_store: WeakEntity<GitStore>,
        repo: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let graph_task = git_store
            .update(cx, |git_store, cx| {
                repo.upgrade().map(|repo| {
                    git_store.commit_graph(&repo, 0, PAGE_SIZE, cx)
                })
            })
            .ok()
            .flatten();

        window
            .spawn(cx, async move |cx| {
                let graph = graph_task?.await.log_err()?;
                let repo = repo.upgrade()?;

                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let project = workspace.project();
                        let view = cx.new(|cx| {
                            GitGraphView::new(
                                graph,
                                git_store.clone(),
                                repo.clone(),
                                workspace.weak_handle(),
                                project.clone(),
                                window,
                                cx,
                            )
                        });

                        let pane = workspace.active_pane();
                        pane.update(cx, |pane, cx| {
                            let ix = pane.items().position(|item| {
                                item.downcast::<GitGraphView>().is_some()
                            });
                            if let Some(ix) = ix {
                                pane.activate_item(ix, true, true, window, cx);
                            } else {
                                pane.add_item(Box::new(view), true, true, None, window, cx);
                            }
                        })
                    })
                    .log_err()
            })
            .detach();
    }

    fn new(
        graph: CommitGraph,
        git_store: WeakEntity<GitStore>,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        _project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let scroll_handle = UniformListScrollHandle::new();
        let layout = GraphLayout::compute(&graph.commits);

        let snapshot = repository.read(cx).snapshot();
        let remote_url = snapshot
            .remote_upstream_url
            .as_ref()
            .or(snapshot.remote_origin_url.as_ref());

        let remote = remote_url.and_then(|url| {
            let provider_registry = GitHostingProviderRegistry::default_global(cx);
            parse_git_remote_url(provider_registry, url).map(|(host, parsed)| GitRemote {
                host,
                owner: parsed.owner.into(),
                repo: parsed.repo.into(),
            })
        });

        Self {
            graph,
            layout,
            git_store,
            repository: repository.downgrade(),
            workspace,
            remote,
            selected_entry: None,
            scroll_handle,
            focus_handle,
            loading_more: false,
        }
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.graph.commits.len();
        let ix = match self.selected_entry {
            _ if entry_count == 0 => None,
            None => Some(0),
            Some(ix) => {
                if ix == entry_count - 1 {
                    Some(0)
                } else {
                    Some(ix + 1)
                }
            }
        };
        self.select_ix(ix, cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entry_count = self.graph.commits.len();
        let ix = match self.selected_entry {
            _ if entry_count == 0 => None,
            None => Some(entry_count - 1),
            Some(ix) => {
                if ix == 0 {
                    Some(entry_count - 1)
                } else {
                    Some(ix - 1)
                }
            }
        };
        self.select_ix(ix, cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.graph.commits.len();
        let ix = if entry_count != 0 { Some(0) } else { None };
        self.select_ix(ix, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.graph.commits.len();
        let ix = if entry_count != 0 {
            Some(entry_count - 1)
        } else {
            None
        };
        self.select_ix(ix, cx);
    }

    fn select_ix(&mut self, ix: Option<usize>, cx: &mut Context<Self>) {
        self.selected_entry = ix;
        if let Some(ix) = ix {
            self.scroll_handle.scroll_to_item(ix, ScrollStrategy::Top);
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.open_commit_view(window, cx);
    }

    fn open_commit_view(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self
            .selected_entry
            .and_then(|ix| self.graph.commits.get(ix))
        else {
            return;
        };

        if let Some(repo) = self.repository.upgrade() {
            let sha_str = entry.sha.to_string();
            CommitView::open(
                sha_str,
                repo.downgrade(),
                self.workspace.clone(),
                None,
                None,
                window,
                cx,
            );
        }
    }

    fn render_graph_lane(
        &self,
        commit: &CommitGraphNode,
        row: usize,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let lane_count = self.layout.lane_count.max(1);
        let lane_width = px(LANE_WIDTH);
        let total_width = lane_width * lane_count as f32;

        let sha = commit.sha.to_string();
        let (commit_lane, _) = self.layout.positions.get(&sha).copied().unwrap_or((0, row));
        let commit_color = GraphLayout::lane_color(commit_lane);

        // Active lanes
        let active_lanes = self.layout.lanes_per_row.get(row).cloned().unwrap_or_default();
        let prev_active_lanes = if row > 0 {
            self.layout.lanes_per_row.get(row - 1).cloned().unwrap_or_default()
        } else {
            HashSet::new()
        };
        let next_active_lanes = self.layout.lanes_per_row.get(row + 1).cloned().unwrap_or_default();

        // Merge connections (Diverging)
        let merge_connections: Vec<_> = self.layout.merge_connections
            .iter()
            .filter(|mc| mc.child_row == row)
            .cloned()
            .collect();
            
        // Branch terminations (Converging)
        let branch_terminations: Vec<_> = self.layout.branch_terminations
            .iter()
            .filter(|bt| bt.row == row)
            .cloned()
            .collect();

        // Clone values for the canvas closure
        let lane_count_clone = lane_count;
        let commit_lane_clone = commit_lane;
        let active_lanes_clone = active_lanes.clone();
        let prev_active_lanes_clone = prev_active_lanes.clone();
        let next_active_lanes_clone = next_active_lanes.clone();
        let merge_connections_clone = merge_connections;
        let branch_terminations_clone = branch_terminations;

        div()
            .w(total_width)
            .h_full()
            .flex_none()
            .relative()
            .child(
                canvas(
                    move |_, _, _| {},
                    move |bounds, _, window, _| {
                        let lane_width_f = LANE_WIDTH;
                        let origin_x: f32 = bounds.origin.x.into();
                        let origin_y: f32 = bounds.origin.y.into();
                        let row_height: f32 = bounds.size.height.into();
                        let node_y = row_height / 2.0;

                        // Draw vertical lines
                        for lane in 0..lane_count_clone {
                            if !active_lanes_clone.contains(&lane) {
                                continue;
                            }
                            
                            let x = origin_x + (lane as f32 + 0.5) * lane_width_f;
                            let color = GraphLayout::lane_color(lane);
                            let mut path = PathBuilder::stroke(px(2.0));
                            
                            let has_top = prev_active_lanes_clone.contains(&lane);
                            let has_bottom = next_active_lanes_clone.contains(&lane);
                            
                            if lane == commit_lane_clone {
                                // Line from top to node
                                if has_top {
                                    path.move_to(point(px(x), px(origin_y)));
                                    path.line_to(point(px(x), px(origin_y + node_y - NODE_RADIUS)));
                                }
                                // Line from node to bottom
                                if has_bottom {
                                    path.move_to(point(px(x), px(origin_y + node_y + NODE_RADIUS)));
                                    path.line_to(point(px(x), px(origin_y + row_height)));
                                }
                            } else {
                                // Pass through lane (or converging/diverging start/end)
                                // Logic: If has_top, draws from Top. If has_bottom, draws to Bottom.
                                
                                let start_y = if has_top { origin_y } else { origin_y + node_y };
                                let end_y = if has_bottom { origin_y + row_height } else { origin_y + node_y };
                                
                                if end_y > start_y {
                                    path.move_to(point(px(x), px(start_y)));
                                    path.line_to(point(px(x), px(end_y)));
                                }
                            }
                            
                            if let Ok(p) = path.build() {
                                window.paint_path(p, Background::from(color));
                            }
                        }
                        
                        // Draw Horizontal Connectors
                        
                        // Diverging (Merge Commit): commit_lane -> parent_lane
                        for mc in &merge_connections_clone {
                            let from_x = origin_x + (mc.child_lane as f32 + 0.5) * lane_width_f;
                            let to_x = origin_x + (mc.parent_lane as f32 + 0.5) * lane_width_f;
                            let y = origin_y + node_y;
                            let color = GraphLayout::lane_color(mc.parent_lane);
                            
                            let mut path = PathBuilder::stroke(px(2.0));
                            path.move_to(point(px(from_x), px(y)));
                            path.line_to(point(px(to_x), px(y)));
                            
                            if let Ok(p) = path.build() {
                                window.paint_path(p, Background::from(color));
                            }
                        }
                        
                        // Converging (Branch Termination): lane -> connect_to (commit_lane)
                        for bt in &branch_terminations_clone {
                            let from_x = origin_x + (bt.lane as f32 + 0.5) * lane_width_f;
                            let to_x = origin_x + (bt.connect_to as f32 + 0.5) * lane_width_f;
                            let y = origin_y + node_y;
                            let color = GraphLayout::lane_color(bt.lane);
                            
                            let mut path = PathBuilder::stroke(px(2.0));
                            path.move_to(point(px(from_x), px(y)));
                            path.line_to(point(px(to_x), px(y)));
                            
                            if let Ok(p) = path.build() {
                                window.paint_path(p, Background::from(color));
                            }
                        }
                        
                        // Draw Commit Node (filled circle)
                        let node_x = origin_x + (commit_lane_clone as f32 + 0.5) * lane_width_f;
                        let mut node_path = PathBuilder::fill();
                        let radius = NODE_RADIUS;
                        node_path.move_to(point(px(node_x + radius), px(node_y + origin_y)));
                        node_path.arc_to(
                            point(px(radius), px(radius)),
                            px(0.0),
                            false,
                            true,
                            point(px(node_x - radius), px(node_y + origin_y)),
                        );
                        node_path.arc_to(
                            point(px(radius), px(radius)),
                            px(0.0),
                            false,
                            true,
                            point(px(node_x + radius), px(node_y + origin_y)),
                        );
                        node_path.close();
                        if let Ok(p) = node_path.build() {
                            window.paint_path(p, Background::from(commit_color));
                        }
                    },
                )
                .size_full(),
            )
            // Add tooltip overlays for lanes with branch names
            .children(
                active_lanes.iter().filter_map(|&lane| {
                    self.layout.lane_names.get(&lane).map(|name| {
                        let lane_x = lane_width * lane as f32;
                        let name_clone = name.clone();
                        div()
                            .id(SharedString::from(format!("lane-{}-{}", row, lane)))
                            .absolute()
                            .left(lane_x)
                            .top_0()
                            .w(lane_width)
                            .h_full()
                            .tooltip(move |_window, cx| {
                                ui::Tooltip::simple(&name_clone, cx)
                            })
                    })
                })
            )
    }

    fn render_ref_badges(&self, refs: &[GitRef], _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().gap_1().children(refs.iter().map(|git_ref| {
            let (label, color) = match git_ref {
                GitRef::Head => ("HEAD".to_string(), Color::Accent),
                GitRef::LocalBranch(name) => (name.to_string(), Color::Success),
                GitRef::RemoteBranch(name) => (name.to_string(), Color::Warning),
                GitRef::Tag(name) => (format!("tag: {}", name), Color::Muted),
            };
            Label::new(label).size(LabelSize::XSmall).color(color)
        }))
    }

    fn render_commit_entry(
        &self,
        ix: usize,
        commit: &CommitGraphNode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let commit_time = OffsetDateTime::from_unix_timestamp(commit.commit_timestamp)
            .unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH);
        let relative_timestamp = time_format::format_localized_timestamp(
            commit_time,
            OffsetDateTime::now_utc(),
            time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC),
            time_format::TimestampFormat::Relative,
        );

        ListItem::new(("commit", ix))
            .toggle_state(Some(ix) == self.selected_entry)
            .child(
                h_flex()
                    .h_8()
                    .w_full()
                    .gap_2()
                    // Graph lane visualization
                    .child(self.render_graph_lane(commit, ix, window, cx))
                    // Commit info
                    .child(
                        h_flex()
                            .min_w_0()
                            .w_full()
                            .gap_2()
                            .child(self.render_ref_badges(&commit.refs, cx))
                            .child(
                                h_flex()
                                    .min_w_0()
                                    .flex_1()
                                    .gap_1()
                                    .child(
                                        Label::new(commit.author_name.clone())
                                            .size(LabelSize::Small)
                                            .color(Color::Default)
                                            .truncate(),
                                    )
                                    .child(
                                        Label::new(&commit.subject)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .truncate(),
                                    ),
                            )
                            .child(
                                h_flex().flex_none().child(
                                    Label::new(relative_timestamp)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                            ),
                    ),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selected_entry = Some(ix);
                cx.notify();
                this.open_commit_view(window, cx);
            }))
            .into_any_element()
    }
}

impl EventEmitter<ItemEvent> for GitGraphView {}

impl Focusable for GitGraphView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for GitGraphView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entry_count = self.graph.commits.len();

        v_flex()
            .id("git_graph_view")
            .key_context("GitGraphView")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .h(rems_from_px(41.))
                    .pl_3()
                    .pr_2()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Icon::new(IconName::GitBranch)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                Label::new("Commit Graph")
                                    .size(LabelSize::Default)
                                    .color(Color::Default),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(
                                Label::new(format!("{} commits", entry_count))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .when(self.graph.has_more, |this| {
                                this.child(Divider::vertical()).child(
                                    Button::new("load-more", "Load More")
                                        .disabled(self.loading_more)
                                        .label_size(LabelSize::Small)
                                        .icon(IconName::ArrowCircle)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .icon_position(IconPosition::Start)
                                        .on_click(cx.listener(|_this, _, _window, _cx| {
                                            // TODO: Implement load more
                                        })),
                                )
                            }),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .size_full()
                    .child({
                        let view = cx.weak_entity();
                        uniform_list(
                            "git-graph-list",
                            entry_count,
                            move |range, window, cx| {
                                let Some(view) = view.upgrade() else {
                                    return Vec::new();
                                };
                                view.update(cx, |this, cx| {
                                    let mut items = Vec::with_capacity(range.end - range.start);
                                    for ix in range {
                                        if let Some(commit) = this.graph.commits.get(ix) {
                                            items.push(
                                                this.render_commit_entry(ix, commit, window, cx),
                                            );
                                        }
                                    }
                                    items
                                })
                            },
                        )
                        .flex_1()
                        .size_full()
                        .track_scroll(&self.scroll_handle)
                    })
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx),
            )
    }
}

impl Item for GitGraphView {
    type Event = ItemEvent;

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Commit Graph".into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some("Git commit graph visualization".into())
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("git graph")
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        Task::ready(None)
    }

    fn navigate(&mut self, _: Box<dyn Any>, _window: &mut Window, _: &mut Context<Self>) -> bool {
        false
    }

    fn deactivated(&mut self, _window: &mut Window, _: &mut Context<Self>) {}

    fn can_save(&self, _: &App) -> bool {
        false
    }

    fn save(
        &mut self,
        _options: SaveOptions,
        _project: Entity<Project>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn save_as(
        &mut self,
        _project: Entity<Project>,
        _path: ProjectPath,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn reload(
        &mut self,
        _project: Entity<Project>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn is_dirty(&self, _: &App) -> bool {
        false
    }

    fn has_conflict(&self, _: &App) -> bool {
        false
    }

    fn breadcrumbs(
        &self,
        _theme: &theme::Theme,
        _cx: &App,
    ) -> Option<Vec<workspace::item::BreadcrumbText>> {
        None
    }

    fn added_to_workspace(
        &mut self,
        _workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, _: &App) -> Option<gpui::Point<gpui::Pixels>> {
        None
    }

    fn set_nav_history(
        &mut self,
        _: workspace::ItemNavHistory,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else {
            None
        }
    }
}
