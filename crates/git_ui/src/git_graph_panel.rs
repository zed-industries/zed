use crate::{
    commit_details_section::{CommitDetails, CommitDetailsSection},
    commit_view::CommitView,
    files_changed_section::{
        CommitFileInfo, FileClickedEvent, FileContextMenuRequestEvent, FileStatus,
        FilesChangedDelegate, FilesChangedSection, OpenAllFilesDiffEvent,
    },
    git_diff_view::GitDiffView,
    git_graph_element::{BranchPath, Coordinate, GitGraphDecoration, PositionedCommit},
};
use anyhow::{Context as _, Result};
use collections::HashMap;
use git::{
    RestoreFile,
    repository::{CommitOrder, CommitSummary, RepoPath},
};
use git_to_graph::{CommitInput, GraphResult, PointType, build_graph};
use gpui::{
    Action, App, AsyncWindowContext, ClickEvent, Context, Corner, DismissEvent, Entity,
    EventEmitter, FocusHandle, Focusable, Hsla, IntoElement, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Point, Subscription, Task, UniformListScrollHandle,
    WeakEntity, Window, actions, anchored, prelude::*, px, uniform_list,
};
use project::{
    Project,
    git_store::{GitStoreEvent, Repository, RepositoryEvent},
};
use std::ops::Range;
use std::path::Path;
use theme::{ActiveTheme, ensure_minimum_contrast};
use time::OffsetDateTime;
use time_format::{TimestampFormat, format_localized_timestamp};
use ui::{
    ContextMenu, Icon, IconName, IconSize, Label, LabelSize, ParentElement, PopoverMenu, Render,
    SpinnerLabel, SplitButton, WithScrollbar, div, h_flex, prelude::*, v_flex,
};
use util::{ResultExt, paths::PathStyle};
use workspace::dock::{DockPosition, PanelEvent};
use workspace::notifications::DetachAndPromptErr;
use workspace::{Panel, Workspace};

actions!(
    git_graph_panel,
    [
        ToggleGitGraphPanel,
        OpenSelectedFile,
        CheckoutFileAtCommit,
        OpenDiff,
        CompareWithCurrent,
        ToggleRemoteBranches,
        ToggleCommitOrder
    ]
);

const ROW_HEIGHT: Pixels = px(60.0);
const COLUMN_WIDTH: Pixels = px(12.0);
const GRAPH_PADDING: Pixels = px(20.0);
const INITIAL_COMMITS: usize = 50;
const COMMITS_PER_BATCH: usize = 50;
const LOAD_MORE_THRESHOLD: usize = 10;
const DEFAULT_COMMIT_DETAILS_HEIGHT: Pixels = px(200.0);

pub struct GitGraphPanel {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    active_repository: Option<Entity<Repository>>,
    project: Entity<Project>,
    graph_data: Option<GraphResult>,
    positioned_commits: Vec<PositionedCommit>,
    branch_paths: Vec<BranchPath>,
    partial_paths: Vec<BranchPath>,
    commits_loaded: usize,
    is_loading_more: bool,
    has_more_commits: bool,
    loading_commits: bool,
    load_commits_task: Option<Task<Result<()>>>,
    graph_scroll_x: Pixels,
    graph_width: Pixels,
    is_resizing: bool,
    resize_drag_start_x: Option<Pixels>,
    resize_initial_width: Option<Pixels>,
    commit_details_height: Pixels,
    is_resizing_details: bool,
    resize_details_drag_start_y: Option<Pixels>,
    resize_details_initial_height: Option<Pixels>,
    selected_commit_index: Option<usize>,
    selected_commit_hash: Option<String>,
    commit_details: Option<CommitDetails>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    selected_file: Option<CommitFileInfo>,
    selected_file_index: Option<usize>,
    position: DockPosition,
    size: Option<Pixels>,
    active: bool,
    commit_details_section: CommitDetailsSection,
    files_changed_section: Entity<FilesChangedSection<GitFilesChangedDelegate>>,
    scroll_handle: UniformListScrollHandle,
    show_remote_branches: bool,
    commit_order: CommitOrder,
    _file_event_subscriptions: Vec<Subscription>,
}

/// Delegate for providing files changed data in the Git graph context
pub struct GitFilesChangedDelegate {
    selected_index: Option<usize>,
    selected_commit_hash: Option<String>,
    commit_details: Option<CommitDetails>,
    pub(crate) pending_context_menu_request: Option<(usize, Point<Pixels>)>,
    pub(crate) pending_open_all_files_diff: bool,
}

impl GitFilesChangedDelegate {
    pub fn new(
        _project: Entity<Project>,
        _workspace: WeakEntity<Workspace>,
        _active_repository: Option<Entity<Repository>>,
    ) -> Self {
        Self {
            selected_index: None,
            selected_commit_hash: None,
            commit_details: None,
            pending_context_menu_request: None,
            pending_open_all_files_diff: false,
        }
    }

    pub fn set_commit_hash(&mut self, hash: Option<String>) {
        self.selected_commit_hash = hash;
    }
}

impl FilesChangedDelegate for GitFilesChangedDelegate {
    fn files(&self) -> &[CommitFileInfo] {
        self.commit_details
            .as_ref()
            .map(|details| details.files.as_slice())
            .unwrap_or(&[])
    }

    fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    fn set_selected_index(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    fn on_file_click(&mut self, file_index: usize, window: &mut Window, cx: &mut App) {
        self.selected_index = Some(file_index);
        window.dispatch_action(Box::new(OpenSelectedFile), cx);
    }

    fn on_file_context_menu(
        &mut self,
        file_index: usize,
        position: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        self.selected_index = Some(file_index);
        self.pending_context_menu_request = Some((file_index, position));
    }
}

impl GitFilesChangedDelegate {
    pub fn set_commit_details(&mut self, details: Option<CommitDetails>) {
        self.commit_details = details;
        // Reset selected index when commit changes
        self.selected_index = None;
        // Clear any pending context menu request when commit changes
        self.pending_context_menu_request = None;
    }
}

impl GitGraphPanel {
    fn initialize_panel(
        workspace: &mut Workspace,
        workspace_handle: WeakEntity<Workspace>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let project = workspace.project().clone();
        let active_repository = project.read(cx).active_repository(cx);

        let files_changed_delegate = GitFilesChangedDelegate::new(
            project.clone(),
            workspace_handle.clone(),
            active_repository.clone(),
        );

        let files_changed_section =
            cx.new(|_cx| FilesChangedSection::new(files_changed_delegate, focus_handle.clone()));

        // Subscribe to file events
        let file_clicked_subscription = cx.subscribe(
            &files_changed_section,
            |_this, _section, _event: &FileClickedEvent, _cx| {
                // File click is already handled by delegate
            },
        );

        let file_context_menu_subscription = cx.subscribe(
            &files_changed_section,
            |_this, section, event: &FileContextMenuRequestEvent, cx| {
                // Store the context menu request in the delegate for the render method to handle
                section.update(cx, |section, _cx| {
                    section.delegate_mut().pending_context_menu_request =
                        Some((event.file_index, event.position));
                });
            },
        );

        let open_all_files_diff_subscription = cx.subscribe(
            &files_changed_section,
            |_this, section, _event: &OpenAllFilesDiffEvent, cx| {
                section.update(cx, |section, _cx| {
                    section.delegate_mut().pending_open_all_files_diff = true;
                });
            },
        );

        let panel = Self {
            focus_handle: focus_handle.clone(),
            workspace: workspace_handle,
            active_repository,
            project,
            graph_data: None,
            positioned_commits: Vec::new(),
            branch_paths: Vec::new(),
            partial_paths: Vec::new(),
            commits_loaded: 0,
            is_loading_more: false,
            has_more_commits: true,
            loading_commits: false,
            load_commits_task: None,
            graph_scroll_x: px(0.0),
            graph_width: px(200.0),
            is_resizing: false,
            resize_drag_start_x: None,
            resize_initial_width: None,
            commit_details_height: DEFAULT_COMMIT_DETAILS_HEIGHT,
            is_resizing_details: false,
            resize_details_drag_start_y: None,
            resize_details_initial_height: None,
            selected_commit_index: None,
            selected_commit_hash: None,
            commit_details: None,
            context_menu: None,
            selected_file: None,
            selected_file_index: None,
            position: DockPosition::Bottom,
            size: None,
            active: false,
            commit_details_section: CommitDetailsSection::new(),
            files_changed_section,
            scroll_handle: UniformListScrollHandle::new(),
            show_remote_branches: true,
            commit_order: CommitOrder::Topological,
            _file_event_subscriptions: vec![
                file_clicked_subscription,
                file_context_menu_subscription,
                open_all_files_diff_subscription,
            ],
        };

        panel
    }

    fn deploy_file_context_menu_from_delegate(
        &mut self,
        context_menu: Entity<ContextMenu>,
        position: Point<Pixels>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |this, _, _: &DismissEvent, window, cx| {
                if this.context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu.0.focus_handle(cx).contains_focused(window, cx)
                }) {
                    cx.focus_self(window);
                }
                this.context_menu.take();
                cx.notify();
            },
        );
        self.context_menu = Some((context_menu, position, subscription));
    }

    fn load_commits(&mut self, count: usize, cx: &mut Context<Self>) {
        // Only load if the panel is active to avoid queueing multiple loads across editors
        if !self.active {
            return;
        }

        if let Some(active_repository) = self.active_repository.clone() {
            // Don't start a new load if one is already in progress
            if self.loading_commits {
                return;
            }

            self.loading_commits = true;
            cx.notify();

            let include_remotes = self.show_remote_branches;
            let commit_order = self.commit_order;
            let commits_receiver = active_repository.update(cx, |repo, _cx| {
                repo.send_job(
                    Some("Loading git graph".into()),
                    move |state, _cx| async move {
                        if let project::git_store::RepositoryState::Local { backend, .. } = state {
                            backend
                                .get_commits(Some(count), None, include_remotes, commit_order)
                                .await
                        } else {
                            Err(anyhow::anyhow!("Not a local repository"))
                        }
                    },
                )
            });

            let task = cx.spawn(async move |this, cx| {
                let commits_result = commits_receiver.await;

                let commits = match commits_result.context("Failed to receive commits") {
                    Ok(Ok(commits)) => commits,
                    Ok(Err(_)) => {
                        this.update(cx, |this, cx| {
                            this.loading_commits = false;
                            this.is_loading_more = false;
                            this.load_commits_task = None;
                            cx.notify();
                        })
                        .ok();
                        return Ok(());
                    }
                    Err(e) => {
                        this.update(cx, |this, cx| {
                            this.loading_commits = false;
                            this.is_loading_more = false;
                            this.load_commits_task = None;
                            cx.notify();
                        })
                        .ok();
                        return Err(e);
                    }
                };

                this.update(cx, |this, cx| {
                    // Always clear loading state
                    this.loading_commits = false;
                    this.is_loading_more = false;
                    this.load_commits_task = None;

                    let total_commits = commits.len();
                    this.commits_loaded = total_commits;
                    this.has_more_commits = total_commits >= count;

                    let commit_inputs: Vec<CommitInput> = commits
                        .iter()
                        .map(|commit| {
                            let oid = commit.oid.to_string();
                            let parents = commit.parents.iter().map(|p| p.to_string()).collect();

                            CommitInput { oid, parents }
                        })
                        .collect();

                    // Extract branch information from commits (now included in TopoCommit)
                    let mut commit_branches: HashMap<String, Vec<String>> = HashMap::default();
                    let mut head_commit: Option<String> = None;

                    for commit in &commits {
                        let oid_str = commit.oid.to_string();
                        if !commit.branches.is_empty() {
                            commit_branches.insert(oid_str.clone(), commit.branches.clone());
                        }
                        if commit.is_head {
                            head_commit = Some(oid_str);
                        }
                    }

                    match build_graph(commit_inputs) {
                        Ok(graph_result) => {
                            let (positioned, paths, partial_paths) = this.create_layout(
                                &graph_result,
                                &commits,
                                &commit_branches,
                                head_commit.as_deref(),
                                cx,
                            );
                            this.positioned_commits = positioned;
                            this.branch_paths = paths;
                            this.partial_paths = partial_paths;
                            this.graph_data = Some(graph_result);

                            // Calculate required graph width - only increase, never decrease
                            // Clamp to 30% of panel width to prevent hiding commit list
                            let max_x = this
                                .positioned_commits
                                .iter()
                                .map(|c| c.position.x)
                                .max_by(|a, b| {
                                    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                                })
                                .unwrap_or(px(0.0));
                            let new_width = (max_x + px(50.0)).max(px(200.0));
                            let panel_width = this.size.unwrap_or(px(800.0));
                            let max_graph_width = panel_width * 0.3;
                            this.graph_width = this.graph_width.max(new_width).min(max_graph_width);

                            // Load first commit details
                            if !this.positioned_commits.is_empty()
                                && this.selected_commit_index.is_none()
                            {
                                this.selected_commit_index = Some(0);
                                this.selected_commit_hash =
                                    Some(this.positioned_commits[0].oid.clone());
                                this.load_commit_details(0, cx);
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to build git graph: {}", e);
                        }
                    }
                    cx.notify();
                })
                .log_err();

                Ok(())
            });

            self.load_commits_task = Some(task);
        }
    }

    fn load_more_commits(&mut self, cx: &mut Context<Self>) {
        if self.is_loading_more || !self.has_more_commits {
            return;
        }
        self.is_loading_more = true;
        let next_count = self.commits_loaded + COMMITS_PER_BATCH;
        self.load_commits(next_count, cx);
    }

    fn create_layout(
        &self,
        graph_result: &GraphResult,
        commits: &[git::repository::TopoCommit],
        commit_branches: &HashMap<String, Vec<String>>,
        head_commit: Option<&str>,
        cx: &mut Context<Self>,
    ) -> (Vec<PositionedCommit>, Vec<BranchPath>, Vec<BranchPath>) {
        let theme = cx.theme();
        let background = theme.colors().panel_background;

        let colors: Vec<Hsla> = (0..20)
            .map(|i| {
                let hue = (i as f32 * 137.5) % 360.0;
                let base_color = gpui::hsla(hue / 360.0, 0.75, 0.7, 1.0);
                ensure_minimum_contrast(base_color, background, 3.0)
            })
            .collect();

        let mut positioned_commits = Vec::new();
        let mut branch_paths = Vec::new();

        let mut oid_to_row: HashMap<String, usize> = HashMap::default();
        for (row, commit) in commits.iter().enumerate() {
            oid_to_row.insert(commit.oid.to_string(), row);
        }

        log::debug!(
            "create_layout: Processing {} commits, graph_result has {} nodes",
            commits.len(),
            graph_result.nodes.len()
        );

        for (row, commit) in commits.iter().enumerate() {
            let oid_str = commit.oid.to_string();
            let node_info = graph_result.nodes.get(&oid_str);

            let column = node_info.map(|n| n.column as usize).unwrap_or(0);
            let color_idx = node_info.map(|n| n.color_idx).unwrap_or(0);

            let x = GRAPH_PADDING + (column as f32 * COLUMN_WIDTH);
            let y = GRAPH_PADDING + (row as f32 * ROW_HEIGHT);

            // Get branches and tags for this commit
            let branches = commit_branches.get(&oid_str).cloned().unwrap_or_default();
            let tags = commit.tags.clone();

            // Check if this is the HEAD commit
            let is_head = head_commit.map_or(false, |head| head == oid_str);

            positioned_commits.push(PositionedCommit {
                oid: oid_str.clone(),
                author: commit.author.clone().unwrap_or_default(),
                message: commit.summary.clone().unwrap_or_default(),
                date: OffsetDateTime::from_unix_timestamp(commit.timestamp)
                    .map(|timestamp| {
                        let now = OffsetDateTime::now_utc();
                        format_localized_timestamp(
                            timestamp,
                            now,
                            time::UtcOffset::UTC,
                            TimestampFormat::Absolute,
                        )
                    })
                    .unwrap_or_else(|_| "Unknown".to_string()),
                position: Coordinate { x, y },
                color: colors[color_idx % colors.len()],
                branches,
                tags,
                is_head,
            });

            if let Some(node) = node_info {
                for (parent_oid, path) in &node.parents_paths {
                    if path.points.is_empty() {
                        continue;
                    }

                    let mut coordinates = Vec::new();
                    const GAP: f32 = 2.0 / 5.0;

                    for point in &path.points {
                        let y = if point.y < 0 {
                            if let Some(&parent_row) = oid_to_row.get(parent_oid) {
                                parent_row as f32
                            } else {
                                // Parent not in loaded commits - extend path beyond visible area
                                commits.len() as f32 + 10.0
                            }
                        } else {
                            point.y as f32
                        };

                        let px = GRAPH_PADDING + (point.x as f32 * COLUMN_WIDTH);
                        let mut py = GRAPH_PADDING + (y * ROW_HEIGHT);

                        match point.point_type {
                            PointType::MergeBack => py -= GAP * ROW_HEIGHT,
                            PointType::Fork | PointType::MergeTo => py += GAP * ROW_HEIGHT,
                            _ => {}
                        }

                        coordinates.push(Coordinate { x: px, y: py });
                    }

                    if !coordinates.is_empty() {
                        log::debug!("Added branch path with {} coordinates", coordinates.len());
                        branch_paths.push(BranchPath {
                            coordinates,
                            color: colors[path.color_idx % colors.len()],
                        });
                    }
                }
            }
        }

        // Convert partial paths to coordinates
        let mut partial_branch_paths = Vec::new();
        for partial_path in &graph_result.partial_paths {
            let mut coordinates = Vec::new();
            const GAP: f32 = 2.0 / 5.0;

            for &(x, y, typ) in &partial_path.points {
                let px = GRAPH_PADDING + (x as f32 * COLUMN_WIDTH);
                let mut py = GRAPH_PADDING + (y as f32 * ROW_HEIGHT);

                // Apply point type adjustments
                let point_type = PointType::from(typ);
                match point_type {
                    PointType::MergeBack => py -= GAP * ROW_HEIGHT,
                    PointType::Fork | PointType::MergeTo => py += GAP * ROW_HEIGHT,
                    _ => {}
                }

                coordinates.push(Coordinate { x: px, y: py });
            }

            if !coordinates.is_empty() {
                partial_branch_paths.push(BranchPath {
                    coordinates,
                    color: colors[partial_path.color_idx % colors.len()],
                });
            }
        }

        (positioned_commits, branch_paths, partial_branch_paths)
    }

    fn select_commit(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if index < self.positioned_commits.len() {
            self.selected_commit_index = Some(index);
            self.selected_commit_hash = Some(self.positioned_commits[index].oid.clone());
            self.load_commit_details(index, cx);
            cx.notify();
        }
    }

    fn load_commit_details(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.positioned_commits.len() {
            return;
        }

        let commit_oid = self.positioned_commits[index].oid.clone();

        if let Some(active_repository) = self.active_repository.clone() {
            let receiver: Task<Result<CommitDetails>> = active_repository.update(cx, |repo, cx| {
                let show = repo.show(commit_oid.clone());
                let diff = repo.load_commit_diff(commit_oid.clone());

                cx.spawn(async move |_, _| {
                    let commit_details = show.await??;
                    let commit_diff = diff.await??;

                    let files: Vec<CommitFileInfo> = commit_diff
                        .files
                        .iter()
                        .map(|file| {
                            let status = if file.old_text.is_none() {
                                FileStatus::Added
                            } else if file.new_text.is_none() {
                                FileStatus::Deleted
                            } else {
                                FileStatus::Modified
                            };
                            CommitFileInfo {
                                path: file.path.to_proto(),
                                status,
                            }
                        })
                        .collect();

                    let date = OffsetDateTime::from_unix_timestamp(commit_details.commit_timestamp)
                        .map(|timestamp| {
                            let now = OffsetDateTime::now_utc();
                            format_localized_timestamp(
                                timestamp,
                                now,
                                time::UtcOffset::UTC,
                                TimestampFormat::Absolute,
                            )
                        })
                        .unwrap_or_else(|_| "Unknown".to_string());

                    Ok(CommitDetails {
                        hash: commit_details.sha.to_string(),
                        author: commit_details.author_name.to_string(),
                        author_email: commit_details.author_email.to_string(),
                        date,
                        full_message: commit_details.message.to_string(),
                        files,
                    })
                })
            });

            cx.spawn(async move |this, cx| {
                if let Ok(details) = receiver.await {
                    this.update(cx, |this, cx| {
                        this.commit_details = Some(details.clone());
                        this.files_changed_section.update(cx, |section, _cx| {
                            section
                                .delegate_mut()
                                .set_commit_details(Some(details.clone()));
                            section
                                .delegate_mut()
                                .set_commit_hash(this.selected_commit_hash.clone());
                        });
                        cx.notify();
                    })
                    .ok();
                }
                Ok::<(), anyhow::Error>(())
            })
            .detach();
        }
    }

    fn check_load_more(&mut self, visible_range: Range<usize>, cx: &mut Context<Self>) {
        if visible_range.end
            >= self
                .positioned_commits
                .len()
                .saturating_sub(LOAD_MORE_THRESHOLD)
        {
            self.load_more_commits(cx);
        }
    }

    fn open_selected_file(
        &mut self,
        _: &OpenSelectedFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Get the files from the current commit details and find the selected file
        if let Some(_details) = &self.commit_details {
            if let Some(selected_file) = self.files_changed_section.read(cx).get_selected_file() {
                self.open_file_from_git_graph_panel(
                    selected_file.path.clone(),
                    selected_file.status,
                    window,
                    cx,
                );
            }
        }
    }

    fn open_file_from_git_graph_panel(
        &mut self,
        file_path: String,
        file_status: FileStatus,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.active_repository.clone() else {
            return;
        };

        if file_status == FileStatus::Deleted {
            return;
        }

        let project_path = repository.read_with(cx, |repo, cx| {
            let rel_path = RepoPath::from_std_path(Path::new(&file_path), PathStyle::local()).ok();
            if let Some(repo_path) = rel_path {
                repo.repo_path_to_project_path(&repo_path, cx)
            } else {
                None
            }
        });

        if let Some(path) = project_path {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace
                        .open_path_preview(path, None, false, false, true, window, cx)
                        .detach_and_prompt_err("Failed to open file", window, cx, |e, _, _| {
                            Some(format!("{e}"))
                        });
                })
                .ok();
        }
    }

    fn checkout_file_at_commit(
        &mut self,
        _: &CheckoutFileAtCommit,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let file = self
            .selected_file
            .clone()
            .or_else(|| self.files_changed_section.read(cx).get_selected_file());

        let Some(file) = file else {
            return;
        };
        let Some(commit_hash) = &self.selected_commit_hash else {
            return;
        };
        let Some(repository) = self.active_repository.clone() else {
            return;
        };

        let file_path = file.path;
        let commit_hash = commit_hash.clone();

        cx.spawn(async move |_this, cx| {
            let Some(repo_path) =
                RepoPath::from_std_path(Path::new(&file_path), PathStyle::local()).ok()
            else {
                return;
            };

            let receiver = repository.update(cx, |repo, cx| {
                repo.checkout_files(&commit_hash, vec![repo_path], cx)
            });
            if let Ok(receiver) = receiver {
                receiver.await.ok().and_then(|r| r.log_err());
            }
        })
        .detach();
    }

    fn open_diff(&mut self, _: &OpenDiff, window: &mut Window, cx: &mut Context<Self>) {
        let file = self
            .selected_file
            .clone()
            .or_else(|| self.files_changed_section.read(cx).get_selected_file());

        if let Some(file) = file {
            self.open_file_diff(file, window, cx);
        }
    }

    fn compare_with_current(
        &mut self,
        _: &CompareWithCurrent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let file = self
            .selected_file
            .clone()
            .or_else(|| self.files_changed_section.read(cx).get_selected_file());

        if let Some(file) = file {
            self.compare_file_with_current(file, window, cx);
        }
    }

    fn open_all_files_diff_from_event(&mut self, window: &mut Window, cx: &mut App) {
        let Some(commit_details) = &self.commit_details else {
            return;
        };

        let Some(repository) = self.active_repository.clone() else {
            return;
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        // Extract first line of commit message as subject
        let subject = commit_details
            .full_message
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        let commit_summary = CommitSummary {
            sha: commit_details.hash.clone().into(),
            subject: subject.into(),
            commit_timestamp: 0, // Not used by CommitView
            author_name: commit_details.author.clone().into(),
            has_parent: true,
        };

        CommitView::open(
            commit_summary,
            repository.downgrade(),
            workspace.downgrade(),
            window,
            cx,
        );
    }

    fn toggle_remote_branches(
        &mut self,
        _: &ToggleRemoteBranches,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_remote_branches = !self.show_remote_branches;
        self.load_commits(INITIAL_COMMITS, cx);
        cx.notify();
    }

    fn toggle_commit_order(
        &mut self,
        _: &ToggleCommitOrder,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.commit_order = match self.commit_order {
            CommitOrder::Topological => CommitOrder::Date,
            CommitOrder::Date => CommitOrder::Topological,
        };
        self.load_commits(INITIAL_COMMITS, cx);
        cx.notify();
    }

    fn open_file_diff(
        &mut self,
        file: CommitFileInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let Some(repository) = self.active_repository.clone() else {
            return;
        };

        let Some(commit_hash) = self.selected_commit_hash.clone() else {
            return;
        };

        let file_path = file.path;
        let project = self.project.clone();

        // Get the actual parent commit hash from positioned_commits
        let parent_commit_hash = if let Some(index) = self.selected_commit_index {
            // Check if there's a parent commit (index + 1 since commits are newest to oldest)
            if index + 1 < self.positioned_commits.len() {
                self.positioned_commits[index + 1].oid.clone()
            } else {
                // If no parent (initial commit), use a placeholder
                "00000000".to_string()
            }
        } else {
            "00000000".to_string()
        };

        // Spawn window task to load commit diff and create buffers
        window
            .spawn(cx, async move |cx| {
                // Load the full commit diff to get file contents
                let commit_diff = repository
                    .update(cx, |repo, _cx| repo.load_commit_diff(commit_hash.clone()))?
                    .await
                    .context("Failed to load commit diff")?;

                // Convert file_path String to RepoPath for comparison
                let file_repo_path =
                    RepoPath::from_std_path(Path::new(&file_path), PathStyle::local()).ok();

                // Find the file in the diff
                let commit_file = commit_diff?
                    .files
                    .into_iter()
                    .find(|f| {
                        if let Some(ref file_repo_path) = file_repo_path {
                            f.path == *file_repo_path
                        } else {
                            false
                        }
                    })
                    .context("File not found in commit diff")?;

                // Extract old/new text (handle added/deleted files)
                let old_text = commit_file.old_text.unwrap_or_default();
                let new_text = commit_file.new_text.unwrap_or_default();

                // Open GitDiffView in workspace
                workspace.update_in(cx, |workspace, window, cx| {
                    if let Some(task) = GitDiffView::open(
                        old_text,
                        new_text,
                        file_path,
                        parent_commit_hash,
                        commit_hash,
                        project.clone(),
                        workspace,
                        window,
                        cx,
                    ) {
                        task.detach();
                    }
                })?;

                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
    }

    fn compare_file_with_current(
        &mut self,
        file: CommitFileInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let Some(repository) = self.active_repository.clone() else {
            return;
        };

        let Some(commit_hash) = self.selected_commit_hash.clone() else {
            return;
        };

        let file_path = file.path;
        let project = self.project.clone();

        // Spawn window task to load commit file content and current working tree content
        window
            .spawn(cx, async move |cx| {
                // Load the commit diff to get the file content at that commit
                let commit_diff = repository
                    .update(cx, |repo, _cx| repo.load_commit_diff(commit_hash.clone()))?
                    .await
                    .context("Failed to load commit diff")?;

                // Convert file_path String to RepoPath for comparison
                let file_repo_path =
                    RepoPath::from_std_path(Path::new(&file_path), PathStyle::local()).ok();

                // Find the file in the commit
                let commit_file = commit_diff?
                    .files
                    .into_iter()
                    .find(|f| {
                        if let Some(ref file_repo_path) = file_repo_path {
                            f.path == *file_repo_path
                        } else {
                            false
                        }
                    })
                    .context("File not found in commit diff")?;

                // Get the file content from the commit (use new_text as this is the state after the commit)
                let commit_text = commit_file.new_text.unwrap_or_default();

                // Load the current working tree version of the file
                let worktree = project
                    .read_with(cx, |project, cx| project.worktrees(cx).next())?
                    .context("No worktree found")?;

                let current_text = if let Some(repo_path) =
                    RepoPath::from_std_path(Path::new(&file_path), PathStyle::local()).ok()
                {
                    worktree
                        .update(cx, |worktree, cx| worktree.load_file(&repo_path, cx))?
                        .await
                        .map(|loaded_file| loaded_file.text)
                        .unwrap_or_default()
                } else {
                    "".to_string()
                };

                // Open GitDiffView in workspace
                workspace.update_in(cx, |workspace, window, cx| {
                    if let Some(task) = GitDiffView::open(
                        commit_text,
                        current_text,
                        file_path,
                        commit_hash.clone(),
                        "Working Tree".to_string(),
                        project.clone(),
                        workspace,
                        window,
                        cx,
                    ) {
                        task.detach();
                    }
                })?;

                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
    }

    fn render_branch_filter_menu(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let show_remote = self.show_remote_branches;
        let label_text = if show_remote {
            "All branches"
        } else {
            "Local branches"
        };

        let left = ui::ButtonLike::new_rounded_left("branch-filter-left")
            .layer(ui::ElevationIndex::ModalSurface)
            .size(ui::ButtonSize::Compact)
            .child(
                h_flex()
                    .ml_neg_0p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall)),
            )
            .child(
                div()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .mr_0p5(),
            )
            .on_click(move |_, window, cx| {
                window.dispatch_action(Box::new(ToggleRemoteBranches), cx);
            });

        let right = PopoverMenu::new("branch-filter-menu")
            .trigger(
                ui::ButtonLike::new_rounded_right("branch-filter-right")
                    .layer(ui::ElevationIndex::ModalSurface)
                    .size(ui::ButtonSize::None)
                    .child(
                        div()
                            .px_1()
                            .child(Icon::new(IconName::ChevronDown).size(IconSize::XSmall)),
                    ),
            )
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, move |context_menu, _, _| {
                    context_menu.entry(
                        if show_remote {
                            "Local branches only"
                        } else {
                            "All branches"
                        },
                        Some(Box::new(ToggleRemoteBranches)),
                        move |window, cx| {
                            window.dispatch_action(Box::new(ToggleRemoteBranches), cx);
                        },
                    )
                }))
            })
            .anchor(Corner::TopRight)
            .into_any_element();

        SplitButton::new(left, right)
    }

    fn render_commit_order_menu(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let order = self.commit_order;
        let label_text = match order {
            CommitOrder::Topological => "Ancestor order",
            CommitOrder::Date => "Date order",
        };

        let left = ui::ButtonLike::new_rounded_left("commit-order-left")
            .layer(ui::ElevationIndex::ModalSurface)
            .size(ui::ButtonSize::Compact)
            .child(
                h_flex()
                    .ml_neg_0p5()
                    .child(Icon::new(IconName::ArrowRightLeft).size(IconSize::XSmall)),
            )
            .child(
                div()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .mr_0p5(),
            )
            .on_click(move |_, window, cx| {
                window.dispatch_action(Box::new(ToggleCommitOrder), cx);
            });

        let right = PopoverMenu::new("commit-order-menu")
            .trigger(
                ui::ButtonLike::new_rounded_right("commit-order-right")
                    .layer(ui::ElevationIndex::ModalSurface)
                    .size(ui::ButtonSize::None)
                    .child(
                        div()
                            .px_1()
                            .child(Icon::new(IconName::ChevronDown).size(IconSize::XSmall)),
                    ),
            )
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, move |context_menu, _, _| {
                    context_menu.entry(
                        match order {
                            CommitOrder::Topological => "Date order",
                            CommitOrder::Date => "Ancestor order",
                        },
                        Some(Box::new(ToggleCommitOrder)),
                        move |window, cx| {
                            window.dispatch_action(Box::new(ToggleCommitOrder), cx);
                        },
                    )
                }))
            })
            .anchor(Corner::TopRight)
            .into_any_element();

        SplitButton::new(left, right)
    }
}

impl Render for GitGraphPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Check for pending open all files diff request
        let pending_open_all = self
            .files_changed_section
            .read(cx)
            .delegate()
            .pending_open_all_files_diff;
        if pending_open_all {
            self.files_changed_section.update(cx, |section, _cx| {
                section.delegate_mut().pending_open_all_files_diff = false;
            });
            self.open_all_files_diff_from_event(window, cx);
        }

        // Check for pending context menu requests from the files section
        let pending_request = {
            let request = self
                .files_changed_section
                .read(cx)
                .delegate()
                .pending_context_menu_request;
            if request.is_some() {
                self.files_changed_section.update(cx, |section, _cx| {
                    section.delegate_mut().pending_context_menu_request.take()
                })
            } else {
                None
            }
        };

        if let Some((file_index, position)) = pending_request {
            // Get file info first (clone to avoid borrow issues)
            let files = self
                .files_changed_section
                .read(cx)
                .delegate()
                .files()
                .to_vec();

            if let Some(file) = files.get(file_index).cloned() {
                // Update the selected file index and file
                self.files_changed_section.update(cx, |section, _cx| {
                    section.delegate_mut().set_selected_index(Some(file_index));
                });
                self.selected_file = Some(file.clone());
                self.selected_file_index = Some(file_index);

                // Build context menu directly (inlined from delegate to avoid borrowing issues)
                let is_deleted = file.status == FileStatus::Deleted;

                let context_menu = ContextMenu::build(window, cx, |context_menu, _, cx_menu| {
                    let mut menu = context_menu.context(cx_menu.focus_handle());
                    if !is_deleted {
                        menu = menu.entry("Open Diff", None, move |window, cx| {
                            window.dispatch_action(Box::new(OpenDiff), cx);
                        });
                        menu = menu.entry("Compare with Current", None, move |window, cx| {
                            window.dispatch_action(Box::new(CompareWithCurrent), cx);
                        });
                        menu = menu.entry("Checkout File at Commit", None, move |window, cx| {
                            window.dispatch_action(Box::new(CheckoutFileAtCommit), cx);
                        });
                    }
                    if is_deleted {
                        menu = menu.entry("Restore File", None, move |window, cx| {
                            window.dispatch_action(
                                RestoreFile { skip_prompt: false }.boxed_clone(),
                                cx,
                            );
                        });
                    }
                    menu
                });

                self.deploy_file_context_menu_from_delegate(context_menu, position, window, cx);
            }
        }

        let context_menu = self.context_menu.as_ref().map(|(menu, position, _)| {
            anchored()
                .position(*position)
                .anchor(Corner::BottomLeft)
                .snap_to_window_with_margin(px(8.))
                .child(menu.clone())
        });

        let colors = cx.theme().colors();
        let panel_color = colors.panel_background;
        let border_color = colors.border;
        let element_selected = colors.element_selected;
        let element_hover = colors.element_hover;
        let text_muted = colors.text_muted;
        let text = colors.text;
        let text_disabled = colors.text_disabled;
        let editor_background = colors.editor_background;
        let _version_control_added = colors.version_control_added;
        let _version_control_modified = colors.version_control_modified;
        let _version_control_deleted = colors.version_control_deleted;

        v_flex()
            .id("git_graph_panel")
            .size_full()
            .bg(panel_color)
            .key_context("GitGraphPanel")
            .on_action(cx.listener(Self::open_selected_file))
            .on_action(cx.listener(Self::checkout_file_at_commit))
            .on_action(cx.listener(Self::open_diff))
            .on_action(cx.listener(Self::compare_with_current))
            .on_action(cx.listener(Self::toggle_remote_branches))
            .on_action(cx.listener(Self::toggle_commit_order))
            .child(
                // Header with dropdown menu
                h_flex()
                    .justify_end()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .h(ui::Tab::container_height(cx))
                    .border_b_1()
                    .border_color(border_color)
                    .child(self.render_commit_order_menu(cx))
                    .child(self.render_branch_filter_menu(cx))
            )
            .when(self.loading_commits && self.positioned_commits.is_empty(), |this| {
                this.child(
                    div()
                        .p_4()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(SpinnerLabel::new().size(LabelSize::Small))
                        .text_sm()
                        .text_color(Color::Muted.color(cx))
                        .child("Loading commits...")
                )
            })
            .when(!self.loading_commits && self.positioned_commits.is_empty(), |this| {
                this.child(div().p_4().child("No repository selected"))
            })
            .when(!self.positioned_commits.is_empty(), |this| {
                let graph_width = self.graph_width;
                let commits_for_graph = self.positioned_commits.clone();
                let commits_for_info = self.positioned_commits.clone();
                let selected_index = self.selected_commit_index;
                let commit_details = self.commit_details.clone();
                let commit_details_height = self.commit_details_height;

                this.child(
                    h_flex()
                        .flex_1()
                        .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                            // Handle graph width resizing
                            if this.is_resizing {
                                if let (Some(drag_start_x), Some(initial_width)) = (this.resize_drag_start_x, this.resize_initial_width) {
                                    let delta = event.position.x - drag_start_x;
                                    this.graph_width = (initial_width + delta).clamp(px(100.0), px(800.0));
                                    cx.notify();
                                }
                            }
                            // Handle commit details height resizing
                            if this.is_resizing_details {
                                if let (Some(drag_start_y), Some(initial_height)) = (this.resize_details_drag_start_y, this.resize_details_initial_height) {
                                    let delta = event.position.y - drag_start_y;
                                    this.commit_details_height = (initial_height + delta).clamp(px(100.0), px(600.0));
                                    cx.notify();
                                }
                            }
                        }))
                        .on_mouse_up(
                            MouseButton::Left,
                            cx.listener(|this, _event: &MouseUpEvent, _window, cx| {
                                // Handle graph width resize end
                                if this.is_resizing {
                                    this.is_resizing = false;
                                    this.resize_drag_start_x = None;
                                    this.resize_initial_width = None;
                                    cx.notify();
                                }
                                // Handle commit details height resize end
                                if this.is_resizing_details {
                                    this.is_resizing_details = false;
                                    this.resize_details_drag_start_y = None;
                                    this.resize_details_initial_height = None;
                                    cx.notify();
                                }
                            }),
                        )
                        .child(
                            div()
                                .flex_1()
                                .size_full()
                                .relative()
                                .child(
                                    div()
                                        .id("git-commits-scroll")
                                        .size_full()
                                        .child(
                                            uniform_list(
                                        "git-commits",
                                        commits_for_graph.len(),
                                        cx.processor(move |this: &mut Self, range: Range<usize>, _window, cx| {
                                            this.check_load_more(range.clone(), cx);

                                            let selected_idx = selected_index;
                                            let g_width = graph_width;

                                            commits_for_info
                                                .iter()
                                                .enumerate()
                                                .skip(range.start)
                                                .take(range.end - range.start)
                                                .map(|(absolute_index, positioned)| {
                                                    let is_selected = selected_idx == Some(absolute_index);
                                                    let row_bg = if is_selected {
                                                        element_selected
                                                    } else {
                                                        panel_color
                                                    };

                                                    div()
                                                        .id(("commit-row", absolute_index))
                                                        .h(ROW_HEIGHT)
                                                        .w_full()
                                                        .flex()
                                                        .bg(row_bg)
                                                        .hover(|style| style.bg(element_hover))
                                                        .on_click(cx.listener(
                                                            move |this, _event: &ClickEvent, window, cx| {
                                                                this.select_commit(absolute_index, window, cx);
                                                            },
                                                        ))
                                                        .child(div().w(g_width).h_full().flex_shrink_0().overflow_hidden())
                                                        .child(
                                                            div()
                                                                .flex_1()
                                                                .h_full()
                                                                .flex()
                                                                .items_center()
                                                                .px_4()
                                                                .min_w_0()
                                                                .child(
                                                                    div()
                                                                        .w_full()
                                                                        .flex()
                                                                        .gap_4()
                                                                        .child(
                                                                            div()
                                                                                .flex_1()
                                                                                .flex()
                                                                                .flex_col()
                                                                                .gap_1()
                                                                                .min_w_0()
                                                                                .child({
                                                                                    // Author name and branch badges
                                                                                    let mut author_row = div()
                                                                                        .flex()
                                                                                        .items_center()
                                                                                        .gap_2()
                                                                                        .min_w_0()
                                                                                        .overflow_hidden()
                                                                                        .child(
                                                                                            div()
                                                                                                .text_color(text_muted)
                                                                                                .text_sm()
                                                                                                .flex_shrink_0()
                                                                                                .child(positioned.author.clone())
                                                                                        );

                                                                                    // Add HEAD badge if this is the HEAD commit
                                                                                    if positioned.is_head {
                                                                                        let head_bg = gpui::hsla(0.6, 0.9, 0.5, 0.2); // Cyan-ish
                                                                                        let head_border = gpui::hsla(0.6, 0.9, 0.5, 1.0);
                                                                                        let effective_bg = theme::blend_with_background(head_bg, panel_color);
                                                                                        let white = gpui::hsla(0.0, 0.0, 1.0, 1.0);
                                                                                        let head_text_color = theme::get_accessible_text_color(white, effective_bg);

                                                                                        author_row = author_row.child(
                                                                                            div()
                                                                                                .px_2()
                                                                                                .py(px(2.0))
                                                                                                .rounded(px(4.0))
                                                                                                .bg(head_bg)
                                                                                                .border_1()
                                                                                                .border_color(head_border)
                                                                                                .text_color(head_text_color)
                                                                                                .text_xs()
                                                                                                .font_weight(gpui::FontWeight::BOLD)
                                                                                                .child("HEAD")
                                                                                        );
                                                                                    }

                                                                                    // Add branch badges if present
                                                                                    for branch in &positioned.branches {
                                                                                        if !branch.is_empty() {
                                                                                            // Use WCAG-compliant text color for badge (5:1 contrast for small text)
                                                                                            let badge_bg = positioned.color.opacity(0.2);
                                                                                            // Blend the semi-transparent badge with the background
                                                                                            let effective_bg = theme::blend_with_background(badge_bg, panel_color);
                                                                                            // Get white or black text that meets 5:1 contrast
                                                                                            let white = gpui::hsla(0.0, 0.0, 1.0, 1.0);
                                                                                            let badge_text_color = theme::get_accessible_text_color(white, effective_bg);

                                                                                            author_row = author_row.child(
                                                                                                div()
                                                                                                    .flex()
                                                                                                    .items_center()
                                                                                                    .gap_1()
                                                                                                    .px_2()
                                                                                                    .py(px(2.0))
                                                                                                    .rounded(px(4.0))
                                                                                                    .bg(badge_bg)
                                                                                                    .border_1()
                                                                                                    .border_color(positioned.color)
                                                                                                    .text_color(badge_text_color)
                                                                                                    .text_xs()
                                                                                                    .child(
                                                                                                        Icon::new(IconName::GitBranch)
                                                                                                            .size(IconSize::XSmall)
                                                                                                            .color(Color::Custom(badge_text_color))
                                                                                                    )
                                                                                                    .child(branch.clone())
                                                                                            );
                                                                                        }
                                                                                    }

                                                                                    // Render tags with a different icon and style
                                                                                    for tag in &positioned.tags {
                                                                                        if !tag.is_empty() {
                                                                                            // Tags use a gold/yellow color scheme
                                                                                            let tag_color = gpui::hsla(45.0 / 360.0, 0.7, 0.5, 1.0); // Gold color
                                                                                            let tag_bg = tag_color.opacity(0.15);
                                                                                            let effective_bg = theme::blend_with_background(tag_bg, panel_color);
                                                                                            let white = gpui::hsla(0.0, 0.0, 1.0, 1.0);
                                                                                            let tag_text_color = theme::get_accessible_text_color(white, effective_bg);

                                                                                            author_row = author_row.child(
                                                                                                div()
                                                                                                    .flex()
                                                                                                    .items_center()
                                                                                                    .gap_1()
                                                                                                    .px_2()
                                                                                                    .py(px(2.0))
                                                                                                    .rounded(px(4.0))
                                                                                                    .bg(tag_bg)
                                                                                                    .border_1()
                                                                                                    .border_color(tag_color)
                                                                                                    .text_color(tag_text_color)
                                                                                                    .text_xs()
                                                                                                    .child(
                                                                                                        Icon::new(IconName::Tag)
                                                                                                            .size(IconSize::XSmall)
                                                                                                            .color(Color::Custom(tag_text_color))
                                                                                                    )
                                                                                                    .child(tag.clone())
                                                                                            );
                                                                                        }
                                                                                    }

                                                                                    author_row
                                                                                })
                                                                                .child(
                                                                                    div()
                                                                                        .text_color(text)
                                                                                        .text_sm()
                                                                                        .overflow_hidden()
                                                                                        .whitespace_nowrap()
                                                                                        .child(positioned.message.clone()),
                                                                                ),
                                                                        )
                                                                        .child(
                                                                            div()
                                                                                .flex()
                                                                                .flex_col()
                                                                                .items_end()
                                                                                .gap_1()
                                                                                .flex_shrink_0()
                                                                                .child(
                                                                                    div()
                                                                                        .text_color(text_muted)
                                                                                        .text_xs()
                                                                                        .child(positioned.date.clone()),
                                                                                )
                                                                                .child(
                                                                                    div()
                                                                                        .text_color(text_disabled)
                                                                                        .text_xs()
                                                                                        .font_family("monospace")
                                                                                        .child(positioned.oid[..7.min(positioned.oid.len())].to_string()),
                                                                                ),
                                                                        ),
                                                                ),
                                                        )
                                                })
                                                .collect()
                                        }),
                                    )
                                    .size_full()
                                    .flex_grow()
                                    .with_decoration(GitGraphDecoration::new(
                                        self.positioned_commits.clone(),
                                        self.branch_paths.clone(),
                                        self.partial_paths.clone(),
                                        self.graph_scroll_x,
                                        self.graph_width,
                                    ))
                                    .track_scroll(self.scroll_handle.clone()))
                                )
                                .custom_scrollbars(
                                    ui::Scrollbars::new(ui::ScrollAxes::Vertical)
                                        .tracked_scroll_handle(self.scroll_handle.clone())
                                        .id("git-commits-list-scrollbar"),
                                    window,
                                    cx
                                )
                                .child(
                                    // Wider divider with transparent background and centered line for easier grabbing
                                    div()
                                        .absolute()
                                        .left(graph_width - px(4.0)) // Center the 8px divider on the graph edge
                                        .top_0()
                                        .bottom_0()
                                        .w(px(8.0)) // Make it 8px wide for easier clicking
                                        .cursor_col_resize()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                                                this.is_resizing = true;
                                                this.resize_drag_start_x = Some(event.position.x);
                                                this.resize_initial_width = Some(this.graph_width);
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            // Centered 1px line
                                            div()
                                                .absolute()
                                                .left(px(3.5)) // Center in the 8px parent
                                                .top_0()
                                                .bottom_0()
                                                .w(px(1.0))
                                                .bg(border_color)
                                        ),
                                )
                        )
                        .child(
                            div()
                                .id("commit-details-column")
                                .w(px(400.0))
                                .h_full()
                                .border_l_1()
                                .border_color(border_color)
                                .bg(editor_background)
                                .relative()
                                .child(
                                    // Commit details with explicit height
                                    div()
                                        .h(commit_details_height)
                                        .w_full()
                                        .child(
                                            self.commit_details_section.render(
                                                commit_details.as_ref(),
                                                commit_details_height,
                                                window,
                                                cx,
                                            )
                                        )
                                )
                                .child(
                                    // Wider horizontal divider with transparent background and centered line for easier grabbing
                                    div()
                                        .absolute()
                                        .top(commit_details_height - px(4.0)) // Center the 8px divider
                                        .w_full()
                                        .h(px(8.0)) // Make it 8px tall for easier clicking
                                        .cursor_row_resize()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                                                this.is_resizing_details = true;
                                                this.resize_details_drag_start_y = Some(event.position.y);
                                                this.resize_details_initial_height = Some(this.commit_details_height);
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            // Centered 1px line
                                            div()
                                                .absolute()
                                                .top(px(3.5)) // Center in the 8px parent
                                                .left_0()
                                                .right_0()
                                                .h(px(1.0))
                                                .bg(border_color)
                                        )
                                )
                                .child(
                                    // Files changed section - positioned below commit details
                                    div()
                                        .absolute()
                                        .top(commit_details_height + px(4.0))
                                        .bottom_0()
                                        .w_full()
                                        .child({
                                            // Update the delegate with current commit details before rendering
                                            self.files_changed_section.update(cx, |section, _cx| {
                                                section.delegate_mut().set_commit_details(self.commit_details.clone());
                                            });

                                            self.files_changed_section.clone()
                                        })
                                )
                        )
                )
            })
            .children(context_menu)
    }
}

impl Focusable for GitGraphPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for GitGraphPanel {}
impl EventEmitter<PanelEvent> for GitGraphPanel {}
impl EventEmitter<OpenSelectedFile> for GitGraphPanel {}
impl EventEmitter<OpenDiff> for GitGraphPanel {}
impl EventEmitter<CompareWithCurrent> for GitGraphPanel {}
impl EventEmitter<CheckoutFileAtCommit> for GitGraphPanel {}

impl Panel for GitGraphPanel {
    fn persistent_name() -> &'static str {
        "GitGraphPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(
            position,
            DockPosition::Bottom | DockPosition::Left | DockPosition::Right
        )
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.position = position;
    }

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.size.unwrap_or(px(320.0))
    }

    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.size = size;
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Git Graph")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleGitGraphPanel)
    }

    fn activation_priority(&self) -> u32 {
        1
    }

    fn set_active(&mut self, active: bool, _window: &mut Window, cx: &mut Context<Self>) {
        self.active = active;

        // Load commits when panel becomes active if we haven't loaded yet
        if active && self.positioned_commits.is_empty() && !self.loading_commits {
            if self.active_repository.is_some() {
                self.load_commits(INITIAL_COMMITS, cx);
            }
        }

        cx.notify();
    }
}

impl GitGraphPanel {
    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let workspace_handle = cx.entity();

        cx.new(|cx| {
            let mut panel =
                Self::initialize_panel(workspace, workspace_handle.downgrade(), window, cx);

            cx.subscribe_in(
                &git_store,
                window,
                move |this, _git_store, event, _window, cx| match event {
                    GitStoreEvent::ActiveRepositoryChanged(_) => {
                        this.active_repository = this.project.read(cx).active_repository(cx);
                        this.load_commits(INITIAL_COMMITS, cx);
                        cx.notify();
                    }
                    GitStoreEvent::RepositoryUpdated(
                        _,
                        RepositoryEvent::Updated { full_scan, .. },
                        true,
                    ) => {
                        if *full_scan {
                            this.load_commits(INITIAL_COMMITS, cx);
                            cx.notify();
                        }
                    }
                    _ => {}
                },
            )
            .detach();

            // Only load commits immediately if the panel is active
            // Otherwise, defer until it becomes active (set_active will trigger the load)
            if panel.active_repository.is_some() && panel.active {
                panel.load_commits(INITIAL_COMMITS, cx);
            }

            panel
        })
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            Self::new(workspace, window, cx)
        })
    }
}

pub fn register(workspace: &mut workspace::Workspace) {
    workspace.register_action(|workspace, _: &ToggleGitGraphPanel, window, cx| {
        workspace.toggle_panel_focus::<GitGraphPanel>(window, cx);
    });
}
