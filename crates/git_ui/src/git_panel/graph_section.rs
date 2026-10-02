use std::{cell::Cell, ops::Range, rc::Rc, slice, time::Duration};

use anyhow::Result;
use collections::HashMap;
use git::{
    Oid,
    repository::{LogOrder, LogSource},
};
use gpui::{
    Anchor, AnyElement, Bounds, ClickEvent, ContentMask, Entity, Pixels, ScrollStrategy, Stateful,
    Subscription, Task, UniformListScrollHandle, anchored, canvas, deferred, px, uniform_list,
};
use project::git_store::{CommitDataState, CommitDiff, GitGraphEvent, Repository, RepositoryEvent};
use ui::{Tooltip, WithScrollbar, prelude::*};

use super::{GitPanel, GitPanelMessageTooltip, GitPanelTab};
use crate::git_panel_settings::GitPanelSettings;
use crate::{
    commit_view::CommitView,
    git_graph::{
        ChangedFileEntry, CommitLine, GraphData, accent_colors_count, graph_lanes_width,
        lanes_between_commits, lanes_in_row, paint_graph_lanes, paint_lanes_between_commits,
    },
    git_status_icon,
};
use settings::Settings as _;
use workspace::dock::DockPosition;

const GRAPH_SECTION_LOG_ORDER: LogOrder = LogOrder::DateOrder;
const MAX_GRAPH_SECTION_LANES: usize = 8;
const COMMIT_DATA_PREFETCH_ROWS: usize = 50;
const MAX_CACHED_CHANGED_FILES: usize = 64;
const COMMIT_POPOVER_SHOW_DELAY: Duration = Duration::from_millis(500);
const COMMIT_POPOVER_HIDE_DELAY: Duration = Duration::from_millis(500);

pub(super) struct GraphSection {
    pub(super) is_expanded: bool,
    log_source: Option<LogSource>,
    needs_reload: bool,
    is_loading: bool,
    load_error: Option<SharedString>,
    graph_data: GraphData,
    scroll_handle: UniformListScrollHandle,
    expanded_commits: HashMap<Oid, ExpandedGraphCommit>,
    changed_files_cache: HashMap<Oid, Rc<[ChangedFileEntry]>>,
    selected_file: Option<(Oid, usize)>,
    commit_popover: Option<GraphCommitPopover>,
    _repository_subscriptions: Vec<Subscription>,
}

/// Commit details shown beside a hovered commit row. It is anchored to the panel's outer edge
/// so it opens over the editor instead of covering the panel.
struct GraphCommitPopover {
    sha: Oid,
    /// The hovered row's and the popover's bounds, recorded while they are painted. The
    /// tooltip content occludes the mouse, so hover events can't tell when it is hovered.
    row_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    popover_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    /// Set once the show delay has passed.
    tooltip: Option<Entity<GitPanelMessageTooltip>>,
    _delay_task: Task<()>,
}

struct ExpandedGraphCommit {
    commit_index: usize,
    files: ExpandedCommitFiles,
    _load_task: Option<Task<()>>,
}

enum ExpandedCommitFiles {
    Loading,
    Loaded(Rc<[ChangedFileEntry]>),
    Error(SharedString),
}

impl GraphSection {
    pub(super) fn new(is_expanded: bool, cx: &App) -> Self {
        Self {
            is_expanded,
            log_source: None,
            needs_reload: true,
            is_loading: false,
            load_error: None,
            graph_data: GraphData::new(accent_colors_count(cx.theme().accents())),
            scroll_handle: UniformListScrollHandle::new(),
            expanded_commits: HashMap::default(),
            changed_files_cache: HashMap::default(),
            selected_file: None,
            commit_popover: None,
            _repository_subscriptions: Vec::new(),
        }
    }

    /// The expanded commits' indices and file counts, ordered by commit index.
    fn expanded_rows(&self) -> Vec<(usize, usize)> {
        let mut expanded_rows: Vec<(usize, usize)> = self
            .expanded_commits
            .values()
            .map(|expanded| {
                let file_count = match &expanded.files {
                    ExpandedCommitFiles::Loaded(files) => files.len(),
                    ExpandedCommitFiles::Loading | ExpandedCommitFiles::Error(_) => 0,
                };
                (expanded.commit_index, file_count)
            })
            .collect();
        expanded_rows.sort_unstable();
        expanded_rows
    }

    fn row_count(&self) -> usize {
        let child_row_count: usize = self
            .expanded_rows()
            .iter()
            .filter(|(commit_index, _)| *commit_index < self.graph_data.commits.len())
            .map(|(_, file_count)| (*file_count).max(1))
            .sum();
        self.graph_data.commits.len() + child_row_count
    }

    fn expanded_commit(&self, commit_index: usize) -> Option<&ExpandedGraphCommit> {
        let sha = self.graph_data.commits.get(commit_index)?.data.sha;
        self.expanded_commits.get(&sha)
    }
}

impl GitPanel {
    pub(super) fn mark_graph_section_stale(&mut self, cx: &mut Context<Self>) {
        let section = &mut self.graph_section;
        section.graph_data.clear();
        section.needs_reload = true;
        section.log_source = None;
        section.is_loading = false;
        section.load_error = None;
        section.expanded_commits.clear();
        section.selected_file = None;
        section.commit_popover = None;
        cx.notify();
    }

    pub(super) fn reset_graph_section_for_repository_change(&mut self, cx: &mut Context<Self>) {
        self.graph_section._repository_subscriptions.clear();
        self.graph_section.changed_files_cache.clear();
        self.mark_graph_section_stale(cx);
    }

    pub(super) fn ensure_graph_section_loaded(&mut self, cx: &mut Context<Self>) {
        if !self.graph_section.needs_reload {
            return;
        }
        self.graph_section.needs_reload = false;
        let Some(repository) = self.active_repository.clone() else {
            return;
        };

        if self.graph_section._repository_subscriptions.is_empty() {
            self.graph_section._repository_subscriptions = vec![
                cx.subscribe(&repository, Self::on_graph_section_repository_event),
                cx.observe(&repository, |this, _, cx| {
                    if this.graph_section.is_expanded
                        && GitPanelSettings::get_global(cx).compact_graph
                    {
                        cx.notify();
                    }
                }),
            ];
        }

        self.graph_section.log_source = Self::commit_history_log_source(&repository, cx);
        self.append_graph_section_commits(&repository, cx);
    }

    fn append_graph_section_commits(
        &mut self,
        repository: &Entity<Repository>,
        cx: &mut Context<Self>,
    ) {
        let Some(log_source) = self.graph_section.log_source.clone() else {
            return;
        };
        let graph_data = &mut self.graph_section.graph_data;
        let (is_loading, load_error) = repository.update(cx, |repository, cx| {
            graph_data.append_commits_from_repository(
                repository,
                log_source,
                GRAPH_SECTION_LOG_ORDER,
                cx,
            )
        });
        self.graph_section.is_loading = is_loading;
        self.graph_section.load_error = load_error;
        cx.notify();
    }

    fn on_graph_section_repository_event(
        &mut self,
        repository: Entity<Repository>,
        event: &RepositoryEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            RepositoryEvent::GraphEvent((log_source, log_order), graph_event)
                if *log_order == GRAPH_SECTION_LOG_ORDER
                    && self.graph_section.log_source.as_ref() == Some(log_source) =>
            {
                match graph_event {
                    GitGraphEvent::CountUpdated(_) => {
                        self.append_graph_section_commits(&repository, cx);
                    }
                    GitGraphEvent::FullyLoaded => {
                        self.graph_section.is_loading = false;
                        cx.notify();
                    }
                    GitGraphEvent::LoadingError => {
                        self.graph_section.is_loading = false;
                        self.graph_section.load_error = repository
                            .read(cx)
                            .get_graph_data(log_source.clone(), *log_order)
                            .and_then(|data| data.error.clone());
                        cx.notify();
                    }
                }
            }
            RepositoryEvent::HeadChanged | RepositoryEvent::BranchListChanged => {
                // Events from the initial scan don't invalidate anything, unless the repository
                // had no HEAD commit to show when the section was loaded.
                if repository.read(cx).scan_id > 1 || self.graph_section.log_source.is_none() {
                    self.mark_graph_section_stale(cx);
                }
            }
            _ => {}
        }
    }

    pub(super) fn toggle_graph_section(&mut self, cx: &mut Context<Self>) {
        self.graph_section.is_expanded = !self.graph_section.is_expanded;
        self.graph_section.commit_popover = None;
        self.serialize(cx);
        cx.notify();
    }

    fn open_graph_commit(&self, commit_index: usize, window: &mut Window, cx: &mut App) {
        let Some(commit) = self.graph_section.graph_data.commits.get(commit_index) else {
            return;
        };
        let Some(repository) = self.active_repository.as_ref() else {
            return;
        };
        CommitView::open_without_header(
            commit.data.sha.to_string(),
            repository.downgrade(),
            self.workspace.clone(),
            None,
            window,
            cx,
        );
    }

    pub(super) fn render_graph_section(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        self.active_repository.as_ref()?;
        let is_expanded = self.graph_section.is_expanded;

        // Styled like the branch row of the repository footer.
        let single_repository = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .repositories()
            .len()
            == 1;
        let header = h_flex()
            .id("graph-section-header")
            .flex_none()
            .w_full()
            .px_2()
            .py_1p5()
            .gap_px()
            .cursor_pointer()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
            .child(
                Icon::new(if is_expanded {
                    IconName::ChevronDown
                } else {
                    IconName::ChevronRight
                })
                .size(IconSize::Small)
                .color(if single_repository {
                    Color::Disabled
                } else {
                    Color::Muted
                }),
            )
            .child(Label::new("Graph").size(LabelSize::Small))
            .on_click(cx.listener(|this, _, _, cx| this.toggle_graph_section(cx)));

        if !is_expanded {
            return Some(
                v_flex()
                    .w_full()
                    .flex_none()
                    .child(header)
                    .into_any_element(),
            );
        }

        let section = &self.graph_section;
        let body = if !section.graph_data.commits.is_empty() {
            let row_count = section.row_count();
            let scroll_handle = section.scroll_handle.clone();
            v_flex()
                .flex_1()
                .size_full()
                .overflow_hidden()
                .child(
                    uniform_list(
                        "graph-section-list",
                        row_count,
                        cx.processor(|this, range: Range<usize>, window, cx| {
                            this.render_graph_section_rows(range, window, cx)
                        }),
                    )
                    .size_full()
                    .track_scroll(&scroll_handle),
                )
                .vertical_scrollbar_for(&scroll_handle, window, cx)
                .into_any_element()
        } else if section.load_error.is_some() {
            Self::render_history_placeholder("Failed to load commit history").into_any_element()
        } else if section.is_loading || section.needs_reload {
            Self::render_history_placeholder("Loading Commit History…").into_any_element()
        } else {
            Self::render_history_placeholder("No commits yet").into_any_element()
        };

        Some(
            v_flex()
                .w_full()
                .flex_1()
                .min_h(rems(6.))
                .overflow_hidden()
                .child(header)
                .child(body)
                .into_any_element(),
        )
    }

    fn render_graph_section_rows(
        &mut self,
        range: Range<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let Some(repository) = self.active_repository.clone() else {
            return Vec::new();
        };
        let section = &self.graph_section;
        let commits = &section.graph_data.commits;
        let expanded_rows = section.expanded_rows();
        let rows: Vec<GraphSectionRow> = range
            .clone()
            .filter_map(|row_index| graph_section_row(row_index, commits.len(), &expanded_rows))
            .collect();
        let (Some(first_row), Some(last_row)) = (rows.first(), rows.last()) else {
            return Vec::new();
        };
        let commit_range = first_row.commit_index()..last_row.commit_index() + 1;

        let prefetch_range = commit_range.start.saturating_sub(COMMIT_DATA_PREFETCH_ROWS)
            ..(commit_range.end + COMMIT_DATA_PREFETCH_ROWS).min(commits.len());
        let subjects = repository.update(cx, |repository, cx| {
            for commit in &commits[prefetch_range] {
                repository.fetch_commit_data(commit.data.sha, false, cx);
            }
            commits[commit_range.clone()]
                .iter()
                .map(
                    |commit| match repository.fetch_commit_data(commit.data.sha, false, cx) {
                        CommitDataState::Loaded(data) => data.subject.clone(),
                        CommitDataState::Loading(_) => SharedString::from("Loading…"),
                    },
                )
                .collect::<Vec<_>>()
        });

        let row_height = self.list_item_height().to_pixels(window.rem_size());
        let lines = section.graph_data.lines_in_range(commit_range.clone());

        rows.into_iter()
            .enumerate()
            .map(|(offset, row)| {
                let row_index = range.start + offset;
                match row {
                    GraphSectionRow::Commit(commit_index) => {
                        let Some(entry) = commits.get(commit_index).cloned() else {
                            return div().h(row_height).into_any_element();
                        };
                        let commit_sha = entry.data.sha;
                        let subject = subjects
                            .get(commit_index - commit_range.start)
                            .cloned()
                            .unwrap_or_default();
                        let is_expanded = section.expanded_commit(commit_index).is_some();
                        let row_lines = lines_spanning_rows(&lines, commit_index..commit_index);
                        let gutter_width =
                            gutter_width(lanes_in_row(commit_index, Some(entry.lane), &row_lines));

                        let lanes = canvas(
                            |_, _, _| {},
                            move |bounds, _, window, cx| {
                                // Lines extend beyond their own row, so keep them within this row.
                                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                                    paint_graph_lanes(
                                        bounds,
                                        row_height,
                                        commit_index,
                                        px(0.),
                                        slice::from_ref(&entry),
                                        &row_lines,
                                        window,
                                        cx,
                                    );
                                });
                            },
                        )
                        .flex_none()
                        .w(gutter_width)
                        .h_full();

                        h_flex()
                            .id(("graph-section-commit", commit_index))
                            .h(row_height)
                            .w_full()
                            .pr_2()
                            .gap_1()
                            .overflow_hidden()
                            .cursor_pointer()
                            .relative()
                            .when_some(
                                section
                                    .commit_popover
                                    .as_ref()
                                    .filter(|popover| popover.sha == commit_sha)
                                    .map(|popover| popover.row_bounds.clone()),
                                |this, row_bounds| {
                                    this.child(
                                        canvas(
                                            move |bounds, _, _| row_bounds.set(Some(bounds)),
                                            |_, _, _, _| {},
                                        )
                                        .absolute()
                                        .size_full(),
                                    )
                                },
                            )
                            .on_hover(cx.listener(move |this, hovered, window, cx| {
                                this.hover_graph_commit(commit_sha, *hovered, window, cx)
                            }))
                            .map(|this| graph_row_background(this, is_expanded, cx))
                            .child(lanes)
                            .child(
                                div()
                                    .min_w_0()
                                    .flex_1()
                                    .child(Label::new(subject).size(LabelSize::Small).truncate()),
                            )
                            .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                                if event.click_count() > 1 {
                                    // The first click of a double-click already toggled the commit.
                                    this.toggle_graph_commit(commit_index, cx);
                                    this.open_graph_commit(commit_index, window, cx);
                                } else {
                                    this.toggle_graph_commit(commit_index, cx);
                                }
                            }))
                            .into_any_element()
                    }
                    GraphSectionRow::File {
                        commit_index,
                        file_index,
                    } => {
                        let file = section.expanded_commit(commit_index).and_then(|expanded| {
                            match &expanded.files {
                                ExpandedCommitFiles::Loaded(files) => files.get(file_index),
                                ExpandedCommitFiles::Loading | ExpandedCommitFiles::Error(_) => {
                                    None
                                }
                            }
                        });
                        let (Some(file), Some(commit)) = (file, commits.get(commit_index)) else {
                            return div().h(row_height).into_any_element();
                        };
                        let sha = commit.data.sha;
                        let is_selected = section.selected_file == Some((sha, file_index));
                        h_flex()
                            .id(("graph-section-file", row_index))
                            .h(row_height)
                            .w_full()
                            .pr_2()
                            .gap_1()
                            .overflow_hidden()
                            .cursor_pointer()
                            .map(|this| graph_row_background(this, is_selected, cx))
                            .child(lanes_between_commit_rows(commit_index, &lines))
                            .child(git_status_icon(file.status))
                            .child(
                                Label::new(file.file_name.clone())
                                    .size(LabelSize::Small)
                                    .truncate(),
                            )
                            .when(!file.dir_path.is_empty(), |this| {
                                this.child(
                                    Label::new(file.dir_path.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .truncate_start(),
                                )
                            })
                            .on_click({
                                let file = file.clone();
                                let repository = repository.downgrade();
                                cx.listener(move |this, _: &ClickEvent, window, cx| {
                                    this.graph_section.selected_file = Some((sha, file_index));
                                    CommitView::open_without_header(
                                        sha.to_string(),
                                        repository.clone(),
                                        this.workspace.clone(),
                                        Some(file.repo_path.clone()),
                                        window,
                                        cx,
                                    );
                                    cx.notify();
                                })
                            })
                            .into_any_element()
                    }
                    GraphSectionRow::FilesPlaceholder(commit_index) => {
                        let files = section
                            .expanded_commit(commit_index)
                            .map(|expanded| &expanded.files);
                        let message = match files {
                            Some(ExpandedCommitFiles::Error(_)) => "Failed to load changed files",
                            Some(ExpandedCommitFiles::Loaded(_)) => "No changed files",
                            Some(ExpandedCommitFiles::Loading) | None => "Loading…",
                        };
                        let error = match files {
                            Some(ExpandedCommitFiles::Error(error)) => Some(error.clone()),
                            _ => None,
                        };
                        h_flex()
                            .id(("graph-section-files-placeholder", row_index))
                            .when_some(error, |this, error| this.tooltip(Tooltip::text(error)))
                            .h(row_height)
                            .w_full()
                            .child(lanes_between_commit_rows(commit_index, &lines))
                            .child(
                                Label::new(message)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .into_any_element()
                    }
                }
            })
            .collect()
    }

    fn hover_graph_commit(
        &mut self,
        sha: Oid,
        hovered: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !hovered {
            // Moving to another row hovers the new row before unhovering the old one, so
            // only the popover's own row may hide it.
            if self
                .graph_section
                .commit_popover
                .as_ref()
                .is_some_and(|popover| popover.sha == sha)
            {
                self.hide_graph_commit_popover_after_delay(window, cx);
            }
            return;
        }
        if let Some(popover) = self
            .graph_section
            .commit_popover
            .as_mut()
            .filter(|popover| popover.sha == sha && popover.tooltip.is_some())
        {
            popover._delay_task = Task::ready(());
            return;
        }
        let Some(repository) = self.active_repository.clone() else {
            return;
        };
        let show_task = cx.spawn_in(window, async move |this, cx| {
            cx.background_executor()
                .timer(COMMIT_POPOVER_SHOW_DELAY)
                .await;
            this.update_in(cx, |this, window, cx| {
                let git_panel = cx.entity();
                if let Some(popover) = this
                    .graph_section
                    .commit_popover
                    .as_mut()
                    .filter(|popover| popover.sha == sha)
                {
                    popover.tooltip = Some(GitPanelMessageTooltip::new(
                        git_panel,
                        sha.to_string().into(),
                        repository,
                        window,
                        cx,
                    ));
                    cx.notify();
                }
            })
            .ok();
        });
        self.graph_section.commit_popover = Some(GraphCommitPopover {
            sha,
            row_bounds: Rc::new(Cell::new(None)),
            popover_bounds: Rc::new(Cell::new(None)),
            tooltip: None,
            _delay_task: show_task,
        });
        cx.notify();
    }

    fn hide_graph_commit_popover_after_delay(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(popover) = self.graph_section.commit_popover.as_mut() else {
            return;
        };
        popover._delay_task = cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(COMMIT_POPOVER_HIDE_DELAY)
                    .await;
                let still_hovered = this.update_in(cx, |this, window, cx| {
                    let Some(popover) = this.graph_section.commit_popover.as_ref() else {
                        return false;
                    };
                    let mouse_position = window.mouse_position();
                    let is_over = |bounds: &Rc<Cell<Option<Bounds<Pixels>>>>| {
                        bounds
                            .get()
                            .is_some_and(|bounds| bounds.contains(&mouse_position))
                    };
                    // The mouse position is stale once the pointer leaves the window.
                    if window.is_window_hovered()
                        && (is_over(&popover.row_bounds) || is_over(&popover.popover_bounds))
                    {
                        return true;
                    }
                    this.graph_section.commit_popover = None;
                    cx.notify();
                    false
                });
                if !matches!(still_hovered, Ok(true)) {
                    break;
                }
            }
        });
    }

    pub(super) fn render_graph_commit_popover(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !(self.active_tab == GitPanelTab::Changes
            && self.graph_section.is_expanded
            && GitPanelSettings::get_global(cx).compact_graph
            && !self.commit_editor_expanded)
        {
            return None;
        }
        let popover = self.graph_section.commit_popover.as_ref()?;
        let tooltip = popover.tooltip.clone()?;
        let row_bounds = popover.row_bounds.get()?;
        let popover_bounds = popover.popover_bounds.clone();
        // Open over the editor, on the side of the panel facing away from the window edge.
        let (position, anchor) = match GitPanelSettings::get_global(cx).dock {
            DockPosition::Left => (row_bounds.top_right(), Anchor::TopLeft),
            DockPosition::Right | DockPosition::Bottom => (row_bounds.origin, Anchor::TopRight),
        };
        Some(
            deferred(
                anchored()
                    .position(position)
                    .anchor(anchor)
                    .snap_to_window_with_margin(px(8.))
                    .child(
                        div()
                            .relative()
                            .occlude()
                            .child(
                                canvas(
                                    move |bounds, _, _| popover_bounds.set(Some(bounds)),
                                    |_, _, _, _| {},
                                )
                                .absolute()
                                .size_full(),
                            )
                            .child(tooltip),
                    ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    fn toggle_graph_commit(&mut self, commit_index: usize, cx: &mut Context<Self>) {
        let Some(sha) = self
            .graph_section
            .graph_data
            .commits
            .get(commit_index)
            .map(|commit| commit.data.sha)
        else {
            return;
        };
        if self.graph_section.expanded_commits.remove(&sha).is_some() {
            cx.notify();
            return;
        }

        let (files, load_task) =
            if let Some(files) = self.graph_section.changed_files_cache.get(&sha) {
                (ExpandedCommitFiles::Loaded(files.clone()), None)
            } else {
                let Some(repository) = self.active_repository.as_ref() else {
                    return;
                };
                let diff_task = repository.update(cx, |repository, cx| {
                    repository.load_commit_diff(sha.to_string(), false, cx)
                });
                let load_task = cx.spawn(async move |this, cx| {
                    let diff = diff_task.await;
                    this.update(cx, |this, cx| this.set_graph_commit_files(sha, diff, cx))
                        .ok();
                });
                (ExpandedCommitFiles::Loading, Some(load_task))
            };

        self.graph_section.expanded_commits.insert(
            sha,
            ExpandedGraphCommit {
                commit_index,
                files,
                _load_task: load_task,
            },
        );
        let commit_row = commit_index
            + self
                .graph_section
                .expanded_rows()
                .iter()
                .filter(|(expanded_index, _)| *expanded_index < commit_index)
                .map(|(_, file_count)| (*file_count).max(1))
                .sum::<usize>();
        self.graph_section
            .scroll_handle
            .scroll_to_item(commit_row + 1, ScrollStrategy::Nearest);
        cx.notify();
    }

    fn set_graph_commit_files(
        &mut self,
        sha: Oid,
        diff: Result<CommitDiff>,
        cx: &mut Context<Self>,
    ) {
        let section = &mut self.graph_section;
        let Some(expanded) = section.expanded_commits.get_mut(&sha) else {
            return;
        };
        expanded.files = match diff {
            Ok(diff) => {
                let files: Rc<[ChangedFileEntry]> = diff
                    .files
                    .iter()
                    .map(|file| ChangedFileEntry::from_commit_file(file, cx))
                    .collect();
                if section.changed_files_cache.len() >= MAX_CACHED_CHANGED_FILES {
                    section.changed_files_cache.clear();
                }
                section.changed_files_cache.insert(sha, files.clone());
                ExpandedCommitFiles::Loaded(files)
            }
            Err(error) => {
                log::error!("failed to load changed files for commit {sha}: {error:?}");
                ExpandedCommitFiles::Error(error.to_string().into())
            }
        };
        cx.notify();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GraphSectionRow {
    Commit(usize),
    File {
        commit_index: usize,
        file_index: usize,
    },
    FilesPlaceholder(usize),
}

impl GraphSectionRow {
    fn commit_index(self) -> usize {
        match self {
            Self::Commit(commit_index)
            | Self::File { commit_index, .. }
            | Self::FilesPlaceholder(commit_index) => commit_index,
        }
    }
}

/// Maps a list row to what it shows. `expanded` holds the expanded commits' indices and
/// numbers of changed files, ordered by commit index; an expanded commit with no files to
/// show yet still gets one placeholder row.
fn graph_section_row(
    row_index: usize,
    commit_count: usize,
    expanded: &[(usize, usize)],
) -> Option<GraphSectionRow> {
    let mut child_rows_above = 0;
    for &(expanded_index, file_count) in expanded {
        if expanded_index >= commit_count {
            continue;
        }
        let commit_row = expanded_index + child_rows_above;
        if row_index <= commit_row {
            break;
        }
        let child_row_count = file_count.max(1);
        if row_index <= commit_row + child_row_count {
            let file_index = row_index - commit_row - 1;
            return Some(if file_count == 0 {
                GraphSectionRow::FilesPlaceholder(expanded_index)
            } else {
                GraphSectionRow::File {
                    commit_index: expanded_index,
                    file_index,
                }
            });
        }
        child_rows_above += child_row_count;
    }
    let commit_index = row_index - child_rows_above;
    (commit_index < commit_count).then_some(GraphSectionRow::Commit(commit_index))
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use git::repository::{CommitFile, InitialGraphCommitData, repo_path};
    use gpui::{TestAppContext, VisualTestContext};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use smallvec::SmallVec;
    use theme::LoadThemes;
    use util::path;
    use workspace::MultiWorkspace;

    use super::*;
    use crate::git_panel::{
        ActivateChangesTab, ActivateHistoryTab, GitPanelTab, SerializedGitPanel,
    };

    fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(LoadThemes::JustBase, cx);
            language_model::init(cx);
            editor::init(cx);
            crate::init(cx);
        });
        set_compact_graph(true, cx);
    }

    fn set_compact_graph(enabled: bool, cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().compact_graph = Some(enabled);
                });
            });
        });
    }

    fn oid(digit: char) -> Oid {
        digit.to_string().repeat(40).parse().unwrap()
    }

    /// Seeds a detached HEAD at a merge commit whose parents fork from a common root,
    /// returning the commits newest first.
    fn seed_merge_history(fs: &FakeFs) -> Vec<Oid> {
        let (merge, left, right, root) = (oid('4'), oid('3'), oid('2'), oid('1'));
        let commit = |sha: Oid, parents: &[Oid]| {
            Arc::new(InitialGraphCommitData {
                sha,
                parents: SmallVec::from_slice(parents),
                ref_names: Vec::new(),
            })
        };
        fs.with_git_state(Path::new(path!("/root/project/.git")), false, |state| {
            state.current_branch_name = None;
            state.refs.insert("HEAD".into(), merge.to_string());
            state.graph_commits = vec![
                commit(merge, &[left, right]),
                commit(left, &[root]),
                commit(right, &[root]),
                commit(root, &[]),
            ];
            state.commit_files.insert(
                merge.to_string(),
                vec![
                    CommitFile {
                        path: repo_path("src/added.rs"),
                        old_content: None,
                        new_content: Some(b"added".to_vec()),
                        is_binary: false,
                    },
                    CommitFile {
                        path: repo_path("modified.rs"),
                        old_content: Some(b"old".to_vec()),
                        new_content: Some(b"new".to_vec()),
                        is_binary: false,
                    },
                ],
            );
        })
        .unwrap();
        vec![merge, left, right, root]
    }

    async fn changes_panel_for_project(
        fs: Arc<FakeFs>,
        serialized_panel: Option<SerializedGitPanel>,
        cx: &mut TestAppContext,
    ) -> (Entity<GitPanel>, VisualTestContext) {
        let project = Project::test(fs, [Path::new(path!("/root/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.read(|cx| {
            project
                .read(cx)
                .worktrees(cx)
                .next()
                .unwrap()
                .read(cx)
                .as_local()
                .unwrap()
                .scan_complete()
        })
        .await;
        cx.run_until_parked();

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel =
                GitPanel::new_with_serialized_panel(workspace, serialized_panel, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            workspace.open_panel::<GitPanel>(window, cx);
            panel
        });
        cx.run_until_parked();
        (panel, cx)
    }

    async fn project_with_merge_history(cx: &mut TestAppContext) -> (Arc<FakeFs>, Vec<Oid>) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "project": { ".git": {} } }))
            .await;
        let commits = seed_merge_history(&fs);
        (fs, commits)
    }

    fn loaded_shas(panel: &GitPanel) -> Vec<Oid> {
        panel
            .graph_section
            .graph_data
            .commits
            .iter()
            .map(|commit| commit.data.sha)
            .collect()
    }

    #[test]
    fn test_graph_section_row_mapping() {
        use GraphSectionRow::*;

        let rows = |commit_count: usize, expanded: &[(usize, usize)], row_count: usize| {
            (0..row_count)
                .map(|row_index| graph_section_row(row_index, commit_count, expanded))
                .collect::<Vec<_>>()
        };

        assert_eq!(
            rows(2, &[], 3),
            vec![Some(Commit(0)), Some(Commit(1)), None]
        );
        assert_eq!(
            rows(3, &[(1, 2)], 6),
            vec![
                Some(Commit(0)),
                Some(Commit(1)),
                Some(File {
                    commit_index: 1,
                    file_index: 0
                }),
                Some(File {
                    commit_index: 1,
                    file_index: 1
                }),
                Some(Commit(2)),
                None,
            ]
        );
        assert_eq!(
            rows(2, &[(1, 0)], 4),
            vec![
                Some(Commit(0)),
                Some(Commit(1)),
                Some(FilesPlaceholder(1)),
                None
            ]
        );
        assert_eq!(
            rows(3, &[(0, 1), (2, 0)], 6),
            vec![
                Some(Commit(0)),
                Some(File {
                    commit_index: 0,
                    file_index: 0
                }),
                Some(Commit(1)),
                Some(Commit(2)),
                Some(FilesPlaceholder(2)),
                None,
            ]
        );
        // An expanded index past the end (e.g. after the list shrank) is ignored.
        assert_eq!(rows(1, &[(5, 3)], 2), vec![Some(Commit(0)), None]);
    }

    #[gpui::test]
    async fn test_graph_section_disabled_skips_loading(cx: &mut TestAppContext) {
        init_test(cx);
        set_compact_graph(false, cx);
        let (fs, _) = project_with_merge_history(cx).await;

        let (panel, mut cx) = changes_panel_for_project(fs, None, cx).await;

        panel.update(&mut cx, |panel, _| {
            assert!(panel.graph_section.needs_reload);
            assert!(panel.graph_section.graph_data.commits.is_empty());
            assert!(panel.graph_section._repository_subscriptions.is_empty());
        });
    }

    #[gpui::test]
    async fn test_graph_section_loads_on_changes_tab(cx: &mut TestAppContext) {
        init_test(cx);
        let (fs, commits) = project_with_merge_history(cx).await;

        let (panel, mut cx) = changes_panel_for_project(fs, None, cx).await;

        panel.update(&mut cx, |panel, _| {
            assert_eq!(panel.active_tab, GitPanelTab::Changes);
            assert!(panel.graph_section.is_expanded);
            assert!(!panel.graph_section.needs_reload);
            assert!(!panel.graph_section.is_loading);
            assert_eq!(panel.graph_section.load_error, None);
            assert_eq!(loaded_shas(panel), commits);
            assert_eq!(panel.graph_section.graph_data.max_lanes, 2);
            assert_eq!(panel.graph_section.row_count(), commits.len());
        });
    }

    #[gpui::test]
    async fn test_graph_section_survives_tab_switch(cx: &mut TestAppContext) {
        init_test(cx);
        let (fs, commits) = project_with_merge_history(cx).await;
        let (panel, mut cx) = changes_panel_for_project(fs, None, cx).await;

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.activate_history_tab(&ActivateHistoryTab, window, cx);
        });
        cx.run_until_parked();
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.activate_changes_tab(&ActivateChangesTab, window, cx);
        });
        cx.run_until_parked();

        panel.update(&mut cx, |panel, _| {
            assert!(!panel.graph_section.needs_reload);
            assert!(!panel.graph_section._repository_subscriptions.is_empty());
            assert_eq!(loaded_shas(panel), commits);
        });
    }

    #[gpui::test]
    async fn test_graph_section_collapsed_skips_loading(cx: &mut TestAppContext) {
        init_test(cx);
        let (fs, commits) = project_with_merge_history(cx).await;
        let serialized_panel = SerializedGitPanel {
            graph_section_collapsed: true,
            ..Default::default()
        };

        let (panel, mut cx) = changes_panel_for_project(fs, Some(serialized_panel), cx).await;

        panel.update(&mut cx, |panel, _| {
            assert!(!panel.graph_section.is_expanded);
            assert!(panel.graph_section.needs_reload);
            assert!(panel.graph_section.graph_data.commits.is_empty());
        });

        panel.update(&mut cx, |panel, cx| panel.toggle_graph_section(cx));
        cx.run_until_parked();

        panel.update(&mut cx, |panel, _| {
            assert!(panel.graph_section.is_expanded);
            assert_eq!(loaded_shas(panel), commits);
        });
    }

    #[gpui::test]
    async fn test_graph_section_toggle_commit(cx: &mut TestAppContext) {
        init_test(cx);
        let (fs, commits) = project_with_merge_history(cx).await;
        let (panel, mut cx) = changes_panel_for_project(fs, None, cx).await;

        let expanded_files = |panel: &GitPanel, commit_index: usize| {
            panel
                .graph_section
                .expanded_commit(commit_index)
                .map(|expanded| match &expanded.files {
                    ExpandedCommitFiles::Loading => None,
                    ExpandedCommitFiles::Loaded(files) => Some(
                        files
                            .iter()
                            .map(|file| file.dir_path.to_string())
                            .collect::<Vec<_>>(),
                    ),
                    ExpandedCommitFiles::Error(error) => panic!("unexpected error: {error}"),
                })
        };

        panel.update(&mut cx, |panel, cx| {
            panel.toggle_graph_commit(0, cx);
            assert_eq!(expanded_files(panel, 0), Some(None));
            assert_eq!(panel.graph_section.row_count(), commits.len() + 1);
        });
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            assert_eq!(
                expanded_files(panel, 0),
                Some(Some(vec!["src".to_string(), String::new()]))
            );
            assert_eq!(panel.graph_section.row_count(), commits.len() + 2);
            assert_eq!(
                graph_section_row(3, commits.len(), &panel.graph_section.expanded_rows()),
                Some(GraphSectionRow::Commit(1))
            );

            panel.toggle_graph_commit(0, cx);
            assert!(panel.graph_section.expanded_commits.is_empty());
            assert_eq!(panel.graph_section.row_count(), commits.len());

            // Re-expanding uses the cached files instead of loading them again.
            panel.toggle_graph_commit(0, cx);
            assert_eq!(
                expanded_files(panel, 0),
                Some(Some(vec!["src".to_string(), String::new()]))
            );

            // Expanding another commit keeps the first one expanded.
            panel.toggle_graph_commit(1, cx);
            assert!(expanded_files(panel, 0).is_some());
            assert!(panel.graph_section.expanded_commit(1).is_some());
        });
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            assert_eq!(expanded_files(panel, 1), Some(Some(Vec::new())));
            assert_eq!(panel.graph_section.row_count(), commits.len() + 3);

            // A commit's changed files never change, so they outlive HEAD and branch changes.
            panel.mark_graph_section_stale(cx);
            assert!(panel.graph_section.expanded_commits.is_empty());
            assert!(
                panel
                    .graph_section
                    .changed_files_cache
                    .contains_key(&commits[0])
            );

            panel.reset_graph_section_for_repository_change(cx);
            assert!(panel.graph_section.changed_files_cache.is_empty());
        });
    }

    #[test]
    fn test_graph_section_collapsed_serialization() {
        let serialized = serde_json::to_string(&SerializedGitPanel {
            graph_section_collapsed: true,
            ..Default::default()
        })
        .unwrap();
        let deserialized: SerializedGitPanel = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.graph_section_collapsed);

        let legacy: SerializedGitPanel =
            serde_json::from_str(r#"{"signoff_enabled":true}"#).unwrap();
        assert!(!legacy.graph_section_collapsed);
    }
}

fn graph_row_background(row: Stateful<Div>, is_selected: bool, cx: &App) -> Stateful<Div> {
    // Matches the selected entry styling of the changes list.
    let info_color = cx.theme().status().info;
    if is_selected {
        row.bg(info_color.alpha(0.08))
            .hover(|style| style.bg(info_color.alpha(0.12)))
    } else {
        row.hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
    }
}

fn lines_spanning_rows(lines: &[Rc<CommitLine>], rows: Range<usize>) -> Vec<Rc<CommitLine>> {
    lines
        .iter()
        .filter(|line| line.spans_rows(&rows))
        .cloned()
        .collect()
}

fn gutter_width(lane_count: usize) -> Pixels {
    graph_lanes_width(lane_count.clamp(1, MAX_GRAPH_SECTION_LANES))
}

fn lanes_between_commit_rows(commit_index: usize, lines: &[Rc<CommitLine>]) -> impl IntoElement {
    let lines = lines_spanning_rows(lines, commit_index..commit_index + 1);
    let gutter_width = gutter_width(lanes_between_commits(commit_index, &lines));
    canvas(
        |_, _, _| {},
        move |bounds, _, window, cx| {
            window.with_content_mask(Some(ContentMask { bounds }), |window| {
                paint_lanes_between_commits(bounds, commit_index, &lines, window, cx);
            });
        },
    )
    .flex_none()
    .w(gutter_width)
    .h_full()
}
