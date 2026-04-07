use askpass::{AskPassDelegate, EncryptedPassword};
use collections::{BTreeMap, HashMap, HashSet, IndexSet};
use editor::Editor;
use futures::channel::oneshot;
use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, Oid, ParsedGitRemote,
    parse_git_remote_url,
    repository::{
        Branch, CommitDiff, CommitFile, DropCommitSupport, GraphLogOptions, InitialGraphCommitData,
        LogOrder, LogSource, PushMode, PushOptions, Remote, RepoPath, ResetMode, SearchCommitArgs,
        UpstreamTracking,
    },
    status::{FileStatus, StatusCode, TrackedStatus},
};
use git_ui::{
    commit_tooltip::CommitAvatar, commit_view::CommitView, git_status_icon, picker_prompt,
};
use gpui::{
    Action, AnyElement, App, Bounds, ClickEvent, ClipboardItem, Corner, DefiniteLength,
    DismissEvent, DragMoveEvent, ElementId, Empty, Entity, EventEmitter, FocusHandle, Focusable,
    FontWeight, Hsla, MouseButton, MouseDownEvent, PathBuilder, Pixels, Point, PromptLevel,
    ScrollStrategy, ScrollWheelEvent, SharedString, Subscription, Task, TextStyleRefinement,
    UniformListScrollHandle, WeakEntity, Window, actions, anchored, deferred, point, prelude::*,
    px, uniform_list,
};
use language::line_diff;
use menu::{Cancel, Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use picker::{Picker, PickerDelegate, PickerEditorPosition, popover_menu::PickerPopoverMenu};
use project::git_store::{
    CommitDataState, GitGraphEvent, GitStore, GitStoreEvent, GraphDataResponse, Repository,
    RepositoryEvent, RepositoryId,
};
use search::{
    SearchOption, SearchOptions, SearchSource, SelectNextMatch, SelectPreviousMatch,
    ToggleCaseSensitive, buffer_search,
};
use settings::Settings;
use smallvec::{SmallVec, smallvec};
use std::{
    cell::Cell,
    ops::Range,
    rc::Rc,
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};
use theme::AccentColors;
use theme_settings::ThemeSettings;
use time::{OffsetDateTime, UtcOffset, format_description::BorrowedFormatItem};
use ui::{
    Button, ButtonLike, ButtonStyle, Checkbox, Chip, Color, ColumnWidthConfig,
    CommonAnimationExt as _, ContextMenu, DiffStat, Divider, DropdownMenu, DropdownStyle,
    HeaderResizeInfo, HighlightedLabel, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle,
    RedistributableColumnsState, ScrollableHandle, Table, TableInteractionState,
    TableRenderContext, TableResizeBehavior, ToggleState, Tooltip, WithScrollbar,
    bind_redistributable_columns, prelude::*, render_redistributable_columns_resize_handles,
    render_table_header, table_row::TableRow,
};
use ui_input::ErasedEditor;
use workspace::{
    ModalView, Workspace,
    item::{Item, ItemEvent, TabTooltipContent},
    notifications::DetachAndPromptErr,
};
use zeroize::Zeroize;

const COMMIT_CIRCLE_RADIUS: Pixels = px(3.5);
const COMMIT_CIRCLE_STROKE_WIDTH: Pixels = px(1.5);
const LANE_WIDTH: Pixels = px(16.0);
const LEFT_PADDING: Pixels = px(12.0);
const LINE_WIDTH: Pixels = px(1.5);
const RESIZE_HANDLE_WIDTH: f32 = 8.0;
const COPIED_STATE_DURATION: Duration = Duration::from_secs(2);

struct CopiedState {
    copied_at: Option<Instant>,
}

impl CopiedState {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { copied_at: None }
    }

    fn is_copied(&self) -> bool {
        self.copied_at
            .map(|t| t.elapsed() < COPIED_STATE_DURATION)
            .unwrap_or(false)
    }

    fn mark_copied(&mut self) {
        self.copied_at = Some(Instant::now());
    }
}

struct DraggedSplitHandle;

#[derive(Clone)]
struct ChangedFileEntry {
    status: FileStatus,
    file_name: SharedString,
    dir_path: SharedString,
    repo_path: RepoPath,
}

impl ChangedFileEntry {
    fn from_commit_file(file: &CommitFile, _cx: &App) -> Self {
        let file_name: SharedString = file
            .path
            .file_name()
            .map(|n| n.to_string())
            .unwrap_or_default()
            .into();
        let dir_path: SharedString = file
            .path
            .parent()
            .map(|p| p.as_unix_str().to_string())
            .unwrap_or_default()
            .into();

        let status_code = match (&file.old_text, &file.new_text) {
            (None, Some(_)) => StatusCode::Added,
            (Some(_), None) => StatusCode::Deleted,
            _ => StatusCode::Modified,
        };

        let status = FileStatus::Tracked(TrackedStatus {
            index_status: status_code,
            worktree_status: StatusCode::Unmodified,
        });

        Self {
            status,
            file_name,
            dir_path,
            repo_path: file.path.clone(),
        }
    }

    fn open_in_commit_view(
        &self,
        commit_sha: &SharedString,
        repository: &WeakEntity<Repository>,
        workspace: &WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        CommitView::open(
            commit_sha.to_string(),
            repository.clone(),
            workspace.clone(),
            None,
            Some(self.repo_path.clone()),
            window,
            cx,
        );
    }

    fn render(
        &self,
        ix: usize,
        commit_sha: SharedString,
        repository: WeakEntity<Repository>,
        workspace: WeakEntity<Workspace>,
        _cx: &App,
    ) -> AnyElement {
        let file_name = self.file_name.clone();
        let dir_path = self.dir_path.clone();

        div()
            .w_full()
            .child(
                ButtonLike::new(("changed-file", ix))
                    .child(
                        h_flex()
                            .min_w_0()
                            .w_full()
                            .gap_1()
                            .overflow_hidden()
                            .child(git_status_icon(self.status))
                            .child(
                                Label::new(file_name.clone())
                                    .size(LabelSize::Small)
                                    .truncate(),
                            )
                            .when(!dir_path.is_empty(), |this| {
                                this.child(
                                    Label::new(dir_path.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .truncate_start(),
                                )
                            }),
                    )
                    .tooltip({
                        let meta = if dir_path.is_empty() {
                            file_name
                        } else {
                            format!("{}/{}", dir_path, file_name).into()
                        };
                        move |_, cx| Tooltip::with_meta("View Changes", None, meta.clone(), cx)
                    })
                    .on_click({
                        let entry = self.clone();
                        move |_, window, cx| {
                            entry.open_in_commit_view(
                                &commit_sha,
                                &repository,
                                &workspace,
                                window,
                                cx,
                            );
                        }
                    }),
            )
            .into_any_element()
    }
}

enum QueryState {
    Pending(SharedString),
    Confirmed((SharedString, Task<()>)),
    Empty,
}

impl QueryState {
    fn next_state(&mut self) {
        match self {
            Self::Confirmed((query, _)) => *self = Self::Pending(std::mem::take(query)),
            _ => {}
        };
    }
}

struct SearchState {
    case_sensitive: bool,
    editor: Entity<Editor>,
    state: QueryState,
    pub matches: IndexSet<Oid>,
    pub selected_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GraphSettings {
    show_stashes: bool,
    show_tags: bool,
    show_remote_branches: bool,
    include_reflog_commits: bool,
    first_parent_only: bool,
}

impl Default for GraphSettings {
    fn default() -> Self {
        Self {
            show_stashes: true,
            show_tags: true,
            show_remote_branches: true,
            include_reflog_commits: false,
            first_parent_only: false,
        }
    }
}

impl From<GraphSettings> for GraphLogOptions {
    fn from(settings: GraphSettings) -> Self {
        Self {
            show_stashes: settings.show_stashes,
            show_tags: settings.show_tags,
            include_reflog_commits: settings.include_reflog_commits,
            first_parent_only: settings.first_parent_only,
        }
    }
}

#[derive(Default)]
struct SettingsDropdownState {
    handle: PopoverMenuHandle<ContextMenu>,
    settings: GraphSettings,
}

#[derive(Clone)]
struct BranchInfo {
    ref_name: SharedString,
    is_head: bool,
    is_remote: bool,
    is_selected: bool,
}

#[derive(Default)]
struct BranchFilterState {
    handle: PopoverMenuHandle<Picker<BranchFilterPickerDelegate>>,
    available_branches: Vec<BranchInfo>,
    selected_branches: HashSet<SharedString>,
    query: SharedString,
    branches_loaded: bool,
    is_loading: bool,
}

#[derive(Clone)]
enum BranchFilterEntry {
    Branch(SharedString),
}

struct BranchFilterPickerDelegate {
    graph: Option<WeakEntity<GitGraph>>,
    query: String,
    matches: Vec<BranchFilterEntry>,
    selected_index: usize,
}

impl BranchFilterPickerDelegate {
    fn new() -> Self {
        Self {
            graph: None,
            query: String::new(),
            matches: Vec::new(),
            selected_index: 0,
        }
    }

    fn recompute_matches(&mut self, cx: &App) {
        let matches = self
            .graph
            .as_ref()
            .and_then(|graph| graph.upgrade())
            .map_or_else(Vec::new, |graph| {
                graph.read_with(cx, |graph, _| graph.branch_filter_entries(&self.query))
            });
        self.set_matches(matches);
    }

    fn set_matches(&mut self, matches: Vec<BranchFilterEntry>) {
        self.matches = matches;
        if self.matches.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = self.selected_index.min(self.matches.len() - 1);
        }
    }
}

#[derive(Clone)]
struct SelectedCommitInfo {
    index: usize,
    sha: SharedString,
    subject: Option<SharedString>,
}

#[derive(Clone, Debug)]
enum RefNameKind {
    Branch(SharedString),
    Tag(SharedString),
    Stash(SharedString),
}

impl RefNameKind {
    fn classify(ref_name: &SharedString) -> Option<Self> {
        let name = ref_name.as_ref();
        if name == "refs/stash"
            || name == "stash"
            || name.starts_with("stash@{")
            || name.contains("refs/stash")
        {
            Some(RefNameKind::Stash(ref_name.clone()))
        } else if name.starts_with("tag: ") || name.starts_with("refs/tags/") {
            Some(RefNameKind::Tag(ref_name.clone()))
        } else {
            Some(RefNameKind::Branch(ref_name.clone()))
        }
    }

    fn display_name(&self) -> SharedString {
        match self {
            RefNameKind::Branch(name) => {
                let n = name.as_ref();
                if let Some(stripped) = n.strip_prefix("HEAD -> ") {
                    stripped.to_string().into()
                } else {
                    name.clone()
                }
            }
            RefNameKind::Tag(name) => {
                let n = name.as_ref();
                if let Some(stripped) = n.strip_prefix("tag: ") {
                    stripped.to_string().into()
                } else if let Some(stripped) = n.strip_prefix("refs/tags/") {
                    stripped.to_string().into()
                } else {
                    name.clone()
                }
            }
            RefNameKind::Stash(name) => name.clone(),
        }
    }

    fn stash_index(&self) -> Option<usize> {
        match self {
            RefNameKind::Stash(name) => {
                let n = name.as_ref();
                if let Some(start) = n.find("stash@{") {
                    let rest = &n[start + 7..];
                    rest.strip_suffix('}')?.parse::<usize>().ok()
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }

    fn branch_lookup_name(&self) -> Option<SharedString> {
        match self {
            RefNameKind::Branch(name) => {
                let n = name.as_ref();
                Some(n.strip_prefix("HEAD -> ").unwrap_or(n).to_string().into())
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
struct CommitContextMenuState {
    row_index: usize,
    drop_support: DropCommitSupport,
}

#[derive(Clone, Copy)]
enum ResetPromptMode {
    Soft,
    Mixed,
    Hard,
}

impl ResetPromptMode {
    const ALL: [Self; 3] = [Self::Soft, Self::Mixed, Self::Hard];

    fn to_reset_mode(self) -> ResetMode {
        match self {
            Self::Soft => ResetMode::Soft,
            Self::Mixed => ResetMode::Mixed,
            Self::Hard => ResetMode::Hard,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Soft => "Soft",
            Self::Mixed => "Mixed",
            Self::Hard => "Hard",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BranchPushTarget {
    branch: Branch,
    remote: Remote,
    remote_branch_name: SharedString,
    options: Option<PushOptions>,
}

#[derive(Clone, Debug)]
struct PushBranchDialogState {
    branch: Branch,
    available_remotes: Vec<SharedString>,
    selected_remote: SharedString,
    set_upstream: bool,
    push_mode: PushMode,
}

pub struct SplitState {
    left_ratio: f32,
    visible_left_ratio: f32,
}

impl PushBranchDialogState {
    fn new(branch: Branch, available_remotes: Vec<SharedString>) -> anyhow::Result<Self> {
        let selected_remote = Self::default_remote_name(&branch, &available_remotes)?;
        let set_upstream = Self::default_set_upstream(&branch, selected_remote.as_ref());

        Ok(Self {
            branch,
            available_remotes,
            selected_remote,
            set_upstream,
            push_mode: PushMode::Normal,
        })
    }

    fn default_remote_name(
        branch: &Branch,
        available_remotes: &[SharedString],
    ) -> anyhow::Result<SharedString> {
        if let Some(remote_name) = Self::tracked_upstream_remote_name(branch)
            && let Some(remote) = available_remotes
                .iter()
                .find(|remote| remote.as_ref() == remote_name)
        {
            return Ok(remote.clone());
        }

        available_remotes
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No remote configured for repository"))
    }

    fn tracked_upstream_remote_name(branch: &Branch) -> Option<&str> {
        branch
            .upstream
            .as_ref()
            .filter(|upstream| matches!(upstream.tracking, UpstreamTracking::Tracked(_)))
            .and_then(|upstream| upstream.remote_name())
    }

    fn tracked_upstream_branch_name(branch: &Branch) -> Option<&str> {
        branch
            .upstream
            .as_ref()
            .filter(|upstream| matches!(upstream.tracking, UpstreamTracking::Tracked(_)))
            .and_then(|upstream| upstream.branch_name())
    }

    fn default_set_upstream(branch: &Branch, selected_remote: &str) -> bool {
        Self::tracked_upstream_remote_name(branch) != Some(selected_remote)
    }

    fn select_remote(&mut self, remote_name: SharedString) {
        self.selected_remote = remote_name;
        self.set_upstream = Self::default_set_upstream(&self.branch, self.selected_remote.as_ref());
    }

    fn push_target(&self) -> BranchPushTarget {
        let remote_branch_name = if Self::tracked_upstream_remote_name(&self.branch)
            == Some(self.selected_remote.as_ref())
        {
            Self::tracked_upstream_branch_name(&self.branch)
                .unwrap_or_else(|| self.branch.name())
                .to_string()
                .into()
        } else {
            self.branch.name().to_string().into()
        };

        let options = match (self.set_upstream, self.push_mode) {
            (false, PushMode::Normal) => None,
            (set_upstream, push_mode) => Some(PushOptions {
                set_upstream,
                push_mode,
            }),
        };

        BranchPushTarget {
            branch: self.branch.clone(),
            remote: Remote {
                name: self.selected_remote.clone(),
            },
            remote_branch_name,
            options,
        }
    }
}

impl SplitState {
    pub fn new() -> Self {
        Self {
            left_ratio: 1.0,
            visible_left_ratio: 1.0,
        }
    }

    pub fn right_ratio(&self) -> f32 {
        1.0 - self.visible_left_ratio
    }

    fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedSplitHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;
        let bounds_width = bounds.right() - bounds.left();

        let min_ratio = 0.1;
        let max_ratio = 0.9;

        let new_ratio = (drag_position.x - bounds.left()) / bounds_width;
        self.visible_left_ratio = new_ratio.clamp(min_ratio, max_ratio);
    }

    fn commit_ratio(&mut self) {
        self.left_ratio = self.visible_left_ratio;
    }

    fn on_double_click(&mut self) {
        self.left_ratio = 1.0;
        self.visible_left_ratio = 1.0;
    }
}

actions!(
    git_graph,
    [
        /// Opens the commit view for the selected commit.
        OpenCommitView,
        /// Focuses the search field.
        FocusSearch,
        ToggleBranchFilter,
        ToggleSettingsDropdown,
        ToggleShowStashes,
        ToggleShowTags,
        ToggleShowRemoteBranches,
        ToggleReflogCommits,
        ToggleFirstParentOnly,
        AddTag,
        CreateBranchAtCommit,
        CheckoutCommit,
        CherryPickCommit,
        RevertCommit,
        DropCommit,
        MergeCommit,
        RebaseOntoCommit,
        ResetCommit,
        CopyCommitHash,
        CopyCommitSubject,
    ]
);

fn timestamp_format() -> &'static [BorrowedFormatItem<'static>] {
    static FORMAT: OnceLock<Vec<BorrowedFormatItem<'static>>> = OnceLock::new();
    FORMAT.get_or_init(|| {
        time::format_description::parse("[day] [month repr:short] [year] [hour]:[minute]")
            .unwrap_or_default()
    })
}

fn format_timestamp(timestamp: i64) -> String {
    let Ok(datetime) = OffsetDateTime::from_unix_timestamp(timestamp) else {
        return "Unknown".to_string();
    };

    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local_datetime = datetime.to_offset(local_offset);

    local_datetime
        .format(timestamp_format())
        .unwrap_or_default()
}

fn accent_colors_count(accents: &AccentColors) -> usize {
    accents.0.len()
}

#[derive(Copy, Clone, Debug)]
struct BranchColor(u8);

#[derive(Debug)]
enum LaneState {
    Empty,
    Active {
        child: Oid,
        parent: Oid,
        color: Option<BranchColor>,
        starting_row: usize,
        starting_col: usize,
        destination_column: Option<usize>,
        segments: SmallVec<[CommitLineSegment; 1]>,
    },
}

impl LaneState {
    fn to_commit_lines(
        &mut self,
        ending_row: usize,
        lane_column: usize,
        parent_column: usize,
        parent_color: BranchColor,
    ) -> Option<CommitLine> {
        let state = std::mem::replace(self, LaneState::Empty);

        match state {
            LaneState::Active {
                #[cfg_attr(not(test), allow(unused_variables))]
                parent,
                #[cfg_attr(not(test), allow(unused_variables))]
                child,
                color,
                starting_row,
                starting_col,
                destination_column,
                mut segments,
            } => {
                let final_destination = destination_column.unwrap_or(parent_column);
                let final_color = color.unwrap_or(parent_color);

                Some(CommitLine {
                    #[cfg(test)]
                    child,
                    #[cfg(test)]
                    parent,
                    child_column: starting_col,
                    full_interval: starting_row..ending_row,
                    color_idx: final_color.0 as usize,
                    segments: {
                        match segments.last_mut() {
                            Some(CommitLineSegment::Straight { to_row })
                                if *to_row == usize::MAX =>
                            {
                                if final_destination != lane_column {
                                    *to_row = ending_row - 1;

                                    let curved_line = CommitLineSegment::Curve {
                                        to_column: final_destination,
                                        on_row: ending_row,
                                        curve_kind: CurveKind::Checkout,
                                    };

                                    if *to_row == starting_row {
                                        let last_index = segments.len() - 1;
                                        segments[last_index] = curved_line;
                                    } else {
                                        segments.push(curved_line);
                                    }
                                } else {
                                    *to_row = ending_row;
                                }
                            }
                            Some(CommitLineSegment::Curve {
                                on_row,
                                to_column,
                                curve_kind,
                            }) if *on_row == usize::MAX => {
                                if *to_column == usize::MAX {
                                    *to_column = final_destination;
                                }
                                if matches!(curve_kind, CurveKind::Merge) {
                                    *on_row = starting_row + 1;
                                    if *on_row < ending_row {
                                        if *to_column != final_destination {
                                            segments.push(CommitLineSegment::Straight {
                                                to_row: ending_row - 1,
                                            });
                                            segments.push(CommitLineSegment::Curve {
                                                to_column: final_destination,
                                                on_row: ending_row,
                                                curve_kind: CurveKind::Checkout,
                                            });
                                        } else {
                                            segments.push(CommitLineSegment::Straight {
                                                to_row: ending_row,
                                            });
                                        }
                                    } else if *to_column != final_destination {
                                        segments.push(CommitLineSegment::Curve {
                                            to_column: final_destination,
                                            on_row: ending_row,
                                            curve_kind: CurveKind::Checkout,
                                        });
                                    }
                                } else {
                                    *on_row = ending_row;
                                    if *to_column != final_destination {
                                        segments.push(CommitLineSegment::Straight {
                                            to_row: ending_row,
                                        });
                                        segments.push(CommitLineSegment::Curve {
                                            to_column: final_destination,
                                            on_row: ending_row,
                                            curve_kind: CurveKind::Checkout,
                                        });
                                    }
                                }
                            }
                            Some(CommitLineSegment::Curve {
                                on_row, to_column, ..
                            }) => {
                                if *on_row < ending_row {
                                    if *to_column != final_destination {
                                        segments.push(CommitLineSegment::Straight {
                                            to_row: ending_row - 1,
                                        });
                                        segments.push(CommitLineSegment::Curve {
                                            to_column: final_destination,
                                            on_row: ending_row,
                                            curve_kind: CurveKind::Checkout,
                                        });
                                    } else {
                                        segments.push(CommitLineSegment::Straight {
                                            to_row: ending_row,
                                        });
                                    }
                                } else if *to_column != final_destination {
                                    segments.push(CommitLineSegment::Curve {
                                        to_column: final_destination,
                                        on_row: ending_row,
                                        curve_kind: CurveKind::Checkout,
                                    });
                                }
                            }
                            _ => {}
                        }

                        segments
                    },
                })
            }
            LaneState::Empty => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            LaneState::Empty => true,
            LaneState::Active { .. } => false,
        }
    }
}

struct CommitEntry {
    data: Arc<InitialGraphCommitData>,
    lane: usize,
    color_idx: usize,
}

type ActiveLaneIdx = usize;

enum AllCommitCount {
    NotLoaded,
    Loaded(usize),
}

#[derive(Debug)]
enum CurveKind {
    Merge,
    Checkout,
}

#[derive(Debug)]
enum CommitLineSegment {
    Straight {
        to_row: usize,
    },
    Curve {
        to_column: usize,
        on_row: usize,
        curve_kind: CurveKind,
    },
}

#[derive(Debug)]
struct CommitLine {
    #[cfg(test)]
    child: Oid,
    #[cfg(test)]
    parent: Oid,
    child_column: usize,
    full_interval: Range<usize>,
    color_idx: usize,
    segments: SmallVec<[CommitLineSegment; 1]>,
}

impl CommitLine {
    fn get_first_visible_segment_idx(&self, first_visible_row: usize) -> Option<(usize, usize)> {
        if first_visible_row > self.full_interval.end {
            return None;
        } else if first_visible_row <= self.full_interval.start {
            return Some((0, self.child_column));
        }

        let mut current_column = self.child_column;

        for (idx, segment) in self.segments.iter().enumerate() {
            match segment {
                CommitLineSegment::Straight { to_row } => {
                    if *to_row >= first_visible_row {
                        return Some((idx, current_column));
                    }
                }
                CommitLineSegment::Curve {
                    to_column, on_row, ..
                } => {
                    if *on_row >= first_visible_row {
                        return Some((idx, current_column));
                    }
                    current_column = *to_column;
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CommitLineKey {
    child: Oid,
    parent: Oid,
}

struct GraphData {
    lane_states: SmallVec<[LaneState; 8]>,
    lane_colors: HashMap<ActiveLaneIdx, BranchColor>,
    parent_to_lanes: HashMap<Oid, SmallVec<[usize; 1]>>,
    next_color: BranchColor,
    accent_colors_count: usize,
    commits: Vec<Rc<CommitEntry>>,
    max_commit_count: AllCommitCount,
    max_lanes: usize,
    lines: Vec<Rc<CommitLine>>,
    active_commit_lines: HashMap<CommitLineKey, usize>,
    active_commit_lines_by_parent: HashMap<Oid, SmallVec<[usize; 1]>>,
}

impl GraphData {
    fn new(accent_colors_count: usize) -> Self {
        GraphData {
            lane_states: SmallVec::default(),
            lane_colors: HashMap::default(),
            parent_to_lanes: HashMap::default(),
            next_color: BranchColor(0),
            accent_colors_count,
            commits: Vec::default(),
            max_commit_count: AllCommitCount::NotLoaded,
            max_lanes: 0,
            lines: Vec::default(),
            active_commit_lines: HashMap::default(),
            active_commit_lines_by_parent: HashMap::default(),
        }
    }

    fn clear(&mut self) {
        self.lane_states.clear();
        self.lane_colors.clear();
        self.parent_to_lanes.clear();
        self.commits.clear();
        self.lines.clear();
        self.active_commit_lines.clear();
        self.active_commit_lines_by_parent.clear();
        self.next_color = BranchColor(0);
        self.max_commit_count = AllCommitCount::NotLoaded;
        self.max_lanes = 0;
    }

    fn first_empty_lane_idx(&mut self) -> ActiveLaneIdx {
        self.lane_states
            .iter()
            .position(LaneState::is_empty)
            .unwrap_or_else(|| {
                self.lane_states.push(LaneState::Empty);
                self.lane_states.len() - 1
            })
    }

    fn get_lane_color(&mut self, lane_idx: ActiveLaneIdx) -> BranchColor {
        let accent_colors_count = self.accent_colors_count;
        *self.lane_colors.entry(lane_idx).or_insert_with(|| {
            let color_idx = self.next_color;
            self.next_color = BranchColor((self.next_color.0 + 1) % accent_colors_count as u8);
            color_idx
        })
    }

    fn add_commits(&mut self, commits: &[Arc<InitialGraphCommitData>]) {
        self.commits.reserve(commits.len());
        self.lines.reserve(commits.len() / 2);

        for commit in commits.iter() {
            let commit_row = self.commits.len();

            let commit_lane = self
                .parent_to_lanes
                .get(&commit.sha)
                .and_then(|lanes| lanes.first().copied());

            let commit_lane = commit_lane.unwrap_or_else(|| self.first_empty_lane_idx());

            let commit_color = self.get_lane_color(commit_lane);

            if let Some(lanes) = self.parent_to_lanes.remove(&commit.sha) {
                for lane_column in lanes {
                    let state = &mut self.lane_states[lane_column];

                    if let LaneState::Active {
                        starting_row,
                        segments,
                        ..
                    } = state
                    {
                        if let Some(CommitLineSegment::Curve {
                            to_column,
                            curve_kind: CurveKind::Merge,
                            ..
                        }) = segments.first_mut()
                        {
                            let curve_row = *starting_row + 1;
                            let would_overlap =
                                if lane_column != commit_lane && curve_row < commit_row {
                                    self.commits[curve_row..commit_row]
                                        .iter()
                                        .any(|c| c.lane == commit_lane)
                                } else {
                                    false
                                };

                            if would_overlap {
                                *to_column = lane_column;
                            }
                        }
                    }

                    if let Some(commit_line) =
                        state.to_commit_lines(commit_row, lane_column, commit_lane, commit_color)
                    {
                        self.lines.push(Rc::new(commit_line));
                    }
                }
            }

            commit
                .parents
                .iter()
                .enumerate()
                .for_each(|(parent_idx, parent)| {
                    if parent_idx == 0 {
                        self.lane_states[commit_lane] = LaneState::Active {
                            parent: *parent,
                            child: commit.sha,
                            color: Some(commit_color),
                            starting_col: commit_lane,
                            starting_row: commit_row,
                            destination_column: None,
                            segments: smallvec![CommitLineSegment::Straight { to_row: usize::MAX }],
                        };

                        self.parent_to_lanes
                            .entry(*parent)
                            .or_default()
                            .push(commit_lane);
                    } else {
                        let new_lane = self.first_empty_lane_idx();

                        self.lane_states[new_lane] = LaneState::Active {
                            parent: *parent,
                            child: commit.sha,
                            color: None,
                            starting_col: commit_lane,
                            starting_row: commit_row,
                            destination_column: None,
                            segments: smallvec![CommitLineSegment::Curve {
                                to_column: usize::MAX,
                                on_row: usize::MAX,
                                curve_kind: CurveKind::Merge,
                            },],
                        };

                        self.parent_to_lanes
                            .entry(*parent)
                            .or_default()
                            .push(new_lane);
                    }
                });

            self.max_lanes = self.max_lanes.max(self.lane_states.len());

            self.commits.push(Rc::new(CommitEntry {
                data: commit.clone(),
                lane: commit_lane,
                color_idx: commit_color.0 as usize,
            }));
        }

        self.max_commit_count = AllCommitCount::Loaded(self.commits.len());
    }
}

pub fn init(cx: &mut App) {
    workspace::register_serializable_item::<GitGraph>(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action_renderer(|div, workspace, _, cx| {
            div.when(
                workspace.project().read(cx).active_repository(cx).is_some(),
                |div| {
                    let workspace = workspace.weak_handle();

                    div.on_action({
                        let workspace = workspace.clone();
                        move |_: &git_ui::git_panel::Open, window, cx| {
                            workspace
                                .update(cx, |workspace, cx| {
                                    let Some(repo) =
                                        workspace.project().read(cx).active_repository(cx)
                                    else {
                                        return;
                                    };
                                    let selected_repo_id = repo.read(cx).id;

                                    let existing = workspace
                                        .items_of_type::<GitGraph>(cx)
                                        .find(|graph| graph.read(cx).repo_id == selected_repo_id);
                                    if let Some(existing) = existing {
                                        workspace.activate_item(&existing, true, true, window, cx);
                                        return;
                                    }

                                    let git_store =
                                        workspace.project().read(cx).git_store().clone();
                                    let workspace_handle = workspace.weak_handle();
                                    let git_graph = cx.new(|cx| {
                                        GitGraph::new(
                                            selected_repo_id,
                                            git_store,
                                            workspace_handle,
                                            window,
                                            cx,
                                        )
                                    });
                                    workspace.add_item_to_active_pane(
                                        Box::new(git_graph),
                                        None,
                                        true,
                                        window,
                                        cx,
                                    );
                                })
                                .ok();
                        }
                    })
                    .on_action(
                        move |action: &git_ui::git_panel::OpenAtCommit, window, cx| {
                            let sha = action.sha.clone();
                            workspace
                                .update(cx, |workspace, cx| {
                                    let Some(repo) =
                                        workspace.project().read(cx).active_repository(cx)
                                    else {
                                        return;
                                    };
                                    let selected_repo_id = repo.read(cx).id;

                                    let existing = workspace
                                        .items_of_type::<GitGraph>(cx)
                                        .find(|graph| graph.read(cx).repo_id == selected_repo_id);
                                    if let Some(existing) = existing {
                                        existing.update(cx, |graph, cx| {
                                            graph.select_commit_by_sha(sha.as_str(), cx);
                                        });
                                        workspace.activate_item(&existing, true, true, window, cx);
                                        return;
                                    }

                                    let git_store =
                                        workspace.project().read(cx).git_store().clone();
                                    let workspace_handle = workspace.weak_handle();
                                    let git_graph = cx.new(|cx| {
                                        let mut graph = GitGraph::new(
                                            selected_repo_id,
                                            git_store,
                                            workspace_handle,
                                            window,
                                            cx,
                                        );
                                        graph.select_commit_by_sha(sha.as_str(), cx);
                                        graph
                                    });
                                    workspace.add_item_to_active_pane(
                                        Box::new(git_graph),
                                        None,
                                        true,
                                        window,
                                        cx,
                                    );
                                })
                                .ok();
                        },
                    )
                },
            )
        });
    })
    .detach();
}

fn lane_center_x(bounds: Bounds<Pixels>, lane: f32) -> Pixels {
    bounds.origin.x + LEFT_PADDING + lane * LANE_WIDTH + LANE_WIDTH / 2.0
}

fn to_row_center(
    to_row: usize,
    row_height: Pixels,
    scroll_offset: Pixels,
    bounds: Bounds<Pixels>,
) -> Pixels {
    bounds.origin.y + to_row as f32 * row_height + row_height / 2.0 - scroll_offset
}

fn draw_commit_circle(center_x: Pixels, center_y: Pixels, color: Hsla, window: &mut Window) {
    let radius = COMMIT_CIRCLE_RADIUS;

    let mut builder = PathBuilder::fill();

    // Start at the rightmost point of the circle
    builder.move_to(point(center_x + radius, center_y));

    // Draw the circle using two arc_to calls (top half, then bottom half)
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center_x - radius, center_y),
    );
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center_x + radius, center_y),
    );
    builder.close();

    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

fn compute_diff_stats(diff: &CommitDiff) -> (usize, usize) {
    diff.files.iter().fold((0, 0), |(added, removed), file| {
        let old_text = file.old_text.as_deref().unwrap_or("");
        let new_text = file.new_text.as_deref().unwrap_or("");
        let hunks = line_diff(old_text, new_text);
        hunks
            .iter()
            .fold((added, removed), |(a, r), (old_range, new_range)| {
                (
                    a + (new_range.end - new_range.start) as usize,
                    r + (old_range.end - old_range.start) as usize,
                )
            })
    })
}

pub struct GitGraph {
    focus_handle: FocusHandle,
    search_state: SearchState,
    branch_filter_picker: Entity<Picker<BranchFilterPickerDelegate>>,
    branch_filter_state: BranchFilterState,
    settings_dropdown_state: SettingsDropdownState,
    graph_data: GraphData,
    git_store: Entity<GitStore>,
    workspace: WeakEntity<Workspace>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    commit_context_menu_state: Option<CommitContextMenuState>,
    row_height: Pixels,
    table_interaction_state: Entity<TableInteractionState>,
    column_widths: Entity<RedistributableColumnsState>,
    selected_entry_idx: Option<usize>,
    hovered_entry_idx: Option<usize>,
    graph_canvas_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    log_source: LogSource,
    log_order: LogOrder,
    selected_commit_diff: Option<CommitDiff>,
    selected_commit_diff_stats: Option<(usize, usize)>,
    _commit_diff_task: Option<Task<()>>,
    commit_details_split_state: Entity<SplitState>,
    repo_id: RepositoryId,
    changed_files_scroll_handle: UniformListScrollHandle,
    pending_select_sha: Option<Oid>,
}

impl GitGraph {
    fn invalidate_state(&mut self, cx: &mut Context<Self>) {
        self.graph_data.clear();
        self.search_state.matches.clear();
        self.search_state.selected_index = None;
        self.search_state.state.next_state();
        cx.notify();
    }

    fn reload_graph(&mut self, cx: &mut Context<Self>) {
        self.context_menu = None;
        self.commit_context_menu_state = None;
        self.selected_entry_idx = None;
        self.hovered_entry_idx = None;
        self.selected_commit_diff = None;
        self.selected_commit_diff_stats = None;
        self._commit_diff_task = None;
        self.pending_select_sha = None;
        self.invalidate_state(cx);
    }

    fn branch_display_name(ref_name: &str) -> SharedString {
        ref_name
            .strip_prefix("refs/heads/")
            .or_else(|| ref_name.strip_prefix("refs/remotes/"))
            .unwrap_or(ref_name)
            .to_string()
            .into()
    }

    fn eligible_branch_infos(&self) -> Vec<&BranchInfo> {
        let show_remote = self.settings_dropdown_state.settings.show_remote_branches;
        self.branch_filter_state
            .available_branches
            .iter()
            .filter(|branch| show_remote || !branch.is_remote)
            .collect()
    }

    fn matching_branch_infos(&self, query: &str) -> Vec<&BranchInfo> {
        let query = query.to_lowercase();
        self.eligible_branch_infos()
            .into_iter()
            .filter(|branch| {
                query.is_empty()
                    || Self::branch_display_name(branch.ref_name.as_ref())
                        .to_lowercase()
                        .contains(&query)
            })
            .collect()
    }

    fn branch_filter_entries(&self, query: &str) -> Vec<BranchFilterEntry> {
        self.matching_branch_infos(query)
            .into_iter()
            .map(|branch| BranchFilterEntry::Branch(branch.ref_name.clone()))
            .collect()
    }

    fn all_branches_selection_state(&self) -> ToggleState {
        if !self.branch_filter_state.branches_loaded {
            return ToggleState::Selected;
        }

        let eligible_branches = self.eligible_branch_infos();
        if eligible_branches.is_empty() {
            return ToggleState::Unselected;
        }

        let selected_count = eligible_branches
            .iter()
            .filter(|branch| {
                self.branch_filter_state
                    .selected_branches
                    .contains(&branch.ref_name)
            })
            .count();

        if selected_count == 0 {
            ToggleState::Unselected
        } else if selected_count == eligible_branches.len() {
            ToggleState::Selected
        } else {
            ToggleState::Indeterminate
        }
    }

    fn all_branches_selected(&self) -> bool {
        matches!(self.all_branches_selection_state(), ToggleState::Selected)
    }

    fn set_all_branch_selection(&mut self, selected: bool, cx: &mut Context<Self>) {
        if selected {
            self.branch_filter_state.selected_branches = self
                .eligible_branch_infos()
                .into_iter()
                .map(|branch| branch.ref_name.clone())
                .collect();
        } else {
            self.branch_filter_state.selected_branches.clear();
        }
        self.sync_branch_filter_selection();
        self.apply_branch_filter_source(cx);
        self.refresh_branch_filter_picker(cx);
    }

    fn build_available_branches(branches: Vec<Branch>) -> Vec<BranchInfo> {
        let mut available_branches = branches
            .into_iter()
            .map(|branch| BranchInfo {
                ref_name: branch.ref_name.clone(),
                is_head: branch.is_head,
                is_remote: branch.is_remote(),
                is_selected: false,
            })
            .collect::<Vec<_>>();
        available_branches.sort_by(|left, right| {
            right.is_head.cmp(&left.is_head).then_with(|| {
                left.is_remote.cmp(&right.is_remote).then_with(|| {
                    Self::branch_display_name(left.ref_name.as_ref())
                        .as_ref()
                        .cmp(Self::branch_display_name(right.ref_name.as_ref()).as_ref())
                })
            })
        });
        available_branches
    }

    fn eligible_ref_names(
        branches: &[BranchInfo],
        show_remote_branches: bool,
    ) -> HashSet<SharedString> {
        branches
            .iter()
            .filter(|branch| show_remote_branches || !branch.is_remote)
            .map(|branch| branch.ref_name.clone())
            .collect()
    }

    fn reconcile_branch_selection(
        branches_were_loaded: bool,
        previously_all_eligible_selected: bool,
        previous_selection: &HashSet<SharedString>,
        eligible_ref_names: &HashSet<SharedString>,
    ) -> HashSet<SharedString> {
        if !branches_were_loaded || previously_all_eligible_selected {
            eligible_ref_names.clone()
        } else {
            previous_selection
                .iter()
                .filter(|branch| eligible_ref_names.contains(*branch))
                .cloned()
                .collect()
        }
    }

    fn apply_loaded_available_branches(
        &mut self,
        available_branches: Vec<BranchInfo>,
        branches_were_loaded: bool,
        previously_all_eligible_selected: bool,
        previous_selection: HashSet<SharedString>,
    ) {
        self.branch_filter_state.available_branches = available_branches;
        self.branch_filter_state.branches_loaded = true;
        self.branch_filter_state.is_loading = false;
        self.reconcile_available_branch_selection(
            branches_were_loaded,
            previously_all_eligible_selected,
            &previous_selection,
        );
    }

    fn reconcile_available_branch_selection(
        &mut self,
        branches_were_loaded: bool,
        previously_all_eligible_selected: bool,
        previous_selection: &HashSet<SharedString>,
    ) {
        let eligible_ref_names = Self::eligible_ref_names(
            &self.branch_filter_state.available_branches,
            self.settings_dropdown_state.settings.show_remote_branches,
        );
        self.branch_filter_state.selected_branches = Self::reconcile_branch_selection(
            branches_were_loaded,
            previously_all_eligible_selected,
            previous_selection,
            &eligible_ref_names,
        );
        self.sync_branch_filter_selection();
    }

    fn reconcile_loaded_branch_selection(&mut self, previously_all_eligible_selected: bool) {
        let previous_selection = self.branch_filter_state.selected_branches.clone();
        self.reconcile_available_branch_selection(
            true,
            previously_all_eligible_selected,
            &previous_selection,
        );
    }

    fn sync_branch_filter_selection(&mut self) {
        for branch in &mut self.branch_filter_state.available_branches {
            branch.is_selected = self
                .branch_filter_state
                .selected_branches
                .contains(&branch.ref_name);
        }
    }

    fn branch_filter_source_for_state(
        branches_loaded: bool,
        show_remote_branches: bool,
        available_branches: &[BranchInfo],
        selected_branches: &HashSet<SharedString>,
    ) -> LogSource {
        if !branches_loaded {
            return LogSource::All;
        }

        let eligible_branches = available_branches
            .iter()
            .filter(|branch| show_remote_branches || !branch.is_remote)
            .collect::<Vec<_>>();

        let mut selected_eligible_branches = eligible_branches
            .iter()
            .filter(|branch| selected_branches.contains(&branch.ref_name))
            .map(|branch| branch.ref_name.clone())
            .collect::<Vec<_>>();

        if selected_eligible_branches.is_empty() {
            return LogSource::Branches(Vec::new());
        }

        selected_eligible_branches
            .sort_unstable_by(|left, right| left.as_ref().cmp(right.as_ref()));

        if show_remote_branches && selected_eligible_branches.len() == eligible_branches.len() {
            LogSource::All
        } else {
            LogSource::Branches(selected_eligible_branches)
        }
    }

    fn branch_filter_source(&self) -> LogSource {
        Self::branch_filter_source_for_state(
            self.branch_filter_state.branches_loaded,
            self.settings_dropdown_state.settings.show_remote_branches,
            &self.branch_filter_state.available_branches,
            &self.branch_filter_state.selected_branches,
        )
    }

    fn apply_branch_filter_source(&mut self, cx: &mut Context<Self>) {
        let new_log_source = self
            .branch_filter_source()
            .with_graph_options(self.settings_dropdown_state.settings.into());
        if new_log_source != self.log_source {
            self.log_source = new_log_source;
            self.reload_graph(cx);
        } else {
            cx.notify();
        }
    }

    fn load_available_branches(
        &mut self,
        previously_all_eligible_selected: Option<bool>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.branch_filter_state.is_loading {
            return;
        }

        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        let branches_were_loaded = self.branch_filter_state.branches_loaded;
        let previous_selection = self.branch_filter_state.selected_branches.clone();
        let previously_all_eligible_selected =
            previously_all_eligible_selected.unwrap_or_else(|| self.all_branches_selected());
        self.branch_filter_state.is_loading = true;
        self.refresh_branch_filter_picker(cx);
        let receiver = repository.update(cx, |repository, _| repository.branches());

        cx.spawn_in(window, async move |this, cx| {
            let branches = receiver
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;

            this.update_in(cx, |this, _window, cx| {
                let available_branches = Self::build_available_branches(branches);
                this.apply_loaded_available_branches(
                    available_branches,
                    branches_were_loaded,
                    previously_all_eligible_selected,
                    previous_selection.clone(),
                );
                this.apply_branch_filter_source(cx);
                this.refresh_branch_filter_picker(cx);
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to load branches", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn refresh_branch_filter_picker(&mut self, cx: &mut Context<Self>) {
        let matches = self.branch_filter_entries(self.branch_filter_state.query.as_ref());
        self.branch_filter_picker.update(cx, |picker, cx| {
            picker.delegate.set_matches(matches);
            cx.notify();
        });
    }

    fn toggle_branch_selection(
        &mut self,
        ref_name: SharedString,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .branch_filter_state
            .selected_branches
            .contains(&ref_name)
        {
            self.branch_filter_state.selected_branches.remove(&ref_name);
        } else {
            self.branch_filter_state.selected_branches.insert(ref_name);
        }
        self.sync_branch_filter_selection();
        self.apply_branch_filter_source(cx);
        self.refresh_branch_filter_picker(cx);
    }

    fn open_branch_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.branch_filter_state.branches_loaded {
            self.load_available_branches(None, window, cx);
        }

        if self.branch_filter_state.handle.is_deployed() {
            self.branch_filter_state.handle.hide(cx);
        } else {
            self.branch_filter_state.handle.show(window, cx);
            self.branch_filter_picker.focus_handle(cx).focus(window, cx);
        }
    }

    fn row_height(cx: &App) -> Pixels {
        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        font_size + px(12.0)
    }

    fn graph_canvas_content_width(&self) -> Pixels {
        (LANE_WIDTH * self.graph_data.max_lanes.max(6) as f32) + LEFT_PADDING * 2.0
    }

    fn preview_column_fractions(&self, window: &Window, cx: &App) -> [f32; 5] {
        let fractions = self
            .column_widths
            .read(cx)
            .preview_fractions(window.rem_size());
        [
            fractions[0],
            fractions[1],
            fractions[2],
            fractions[3],
            fractions[4],
        ]
    }

    fn table_column_width_config(&self, window: &Window, cx: &App) -> ColumnWidthConfig {
        let [_, description, date, author, commit] = self.preview_column_fractions(window, cx);
        let table_total = description + date + author + commit;

        let widths = if table_total > 0.0 {
            vec![
                DefiniteLength::Fraction(description / table_total),
                DefiniteLength::Fraction(date / table_total),
                DefiniteLength::Fraction(author / table_total),
                DefiniteLength::Fraction(commit / table_total),
            ]
        } else {
            vec![
                DefiniteLength::Fraction(0.25),
                DefiniteLength::Fraction(0.25),
                DefiniteLength::Fraction(0.25),
                DefiniteLength::Fraction(0.25),
            ]
        };

        ColumnWidthConfig::explicit(widths)
    }

    fn graph_viewport_width(&self, window: &Window, cx: &App) -> Pixels {
        self.column_widths
            .read(cx)
            .preview_column_width(0, window)
            .unwrap_or_else(|| self.graph_canvas_content_width())
    }

    pub fn new(
        repo_id: RepositoryId,
        git_store: Entity<GitStore>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();

        let accent_colors = cx.theme().accents();
        let graph = GraphData::new(accent_colors_count(accent_colors));
        let settings_dropdown_state = SettingsDropdownState::default();
        let branch_filter_state = BranchFilterState::default();
        let branch_filter_picker = cx.new(|picker_cx| {
            Picker::uniform_list(BranchFilterPickerDelegate::new(), window, picker_cx)
                .show_scrollbar(true)
                .width(px(260.))
        });
        let log_source =
            LogSource::default().with_graph_options(settings_dropdown_state.settings.into());
        let log_order = LogOrder::default();

        cx.subscribe(&git_store, |this, _, event, cx| match event {
            GitStoreEvent::RepositoryUpdated(updated_repo_id, repo_event, _) => {
                if this.repo_id == *updated_repo_id {
                    if let Some(repository) = this.get_repository(cx) {
                        this.on_repository_event(repository, repo_event, cx);
                    }
                }
            }
            _ => {}
        })
        .detach();

        let search_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search commits…", window, cx);
            editor
        });

        let table_interaction_state = cx.new(|cx| TableInteractionState::new(cx));
        let column_widths = cx.new(|_cx| {
            RedistributableColumnsState::new(
                5,
                vec![
                    DefiniteLength::Fraction(0.14),
                    DefiniteLength::Fraction(0.6192),
                    DefiniteLength::Fraction(0.1032),
                    DefiniteLength::Fraction(0.086),
                    DefiniteLength::Fraction(0.0516),
                ],
                vec![
                    TableResizeBehavior::Resizable,
                    TableResizeBehavior::Resizable,
                    TableResizeBehavior::Resizable,
                    TableResizeBehavior::Resizable,
                    TableResizeBehavior::Resizable,
                ],
            )
        });
        let mut row_height = Self::row_height(cx);

        cx.observe_global_in::<settings::SettingsStore>(window, move |this, _window, cx| {
            let new_row_height = Self::row_height(cx);
            if new_row_height != row_height {
                this.row_height = new_row_height;
                this.table_interaction_state.update(cx, |state, _cx| {
                    state.scroll_handle.0.borrow_mut().last_item_size = None;
                });
                row_height = new_row_height;
                cx.notify();
            }
        })
        .detach();

        let mut this = GitGraph {
            focus_handle,
            git_store,
            search_state: SearchState {
                case_sensitive: false,
                editor: search_editor,
                matches: IndexSet::default(),
                selected_index: None,
                state: QueryState::Empty,
            },
            branch_filter_picker,
            branch_filter_state,
            settings_dropdown_state,
            workspace,
            graph_data: graph,
            _commit_diff_task: None,
            context_menu: None,
            commit_context_menu_state: None,
            row_height,
            table_interaction_state,
            column_widths,
            selected_entry_idx: None,
            hovered_entry_idx: None,
            graph_canvas_bounds: Rc::new(Cell::new(None)),
            selected_commit_diff: None,
            selected_commit_diff_stats: None,
            log_source,
            log_order,
            commit_details_split_state: cx.new(|_cx| SplitState::new()),
            repo_id,
            changed_files_scroll_handle: UniformListScrollHandle::new(),
            pending_select_sha: None,
        };

        let weak_graph = cx.weak_entity();
        this.branch_filter_picker.update(cx, |picker, _| {
            picker.delegate.graph = Some(weak_graph);
        });
        this.fetch_initial_graph_data(cx);
        this
    }

    fn on_repository_event(
        &mut self,
        repository: Entity<Repository>,
        event: &RepositoryEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            RepositoryEvent::GraphEvent((source, order), event)
                if source == &self.log_source && order == &self.log_order =>
            {
                match event {
                    GitGraphEvent::FullyLoaded => {
                        if let Some(pending_sha_index) =
                            self.pending_select_sha.take().and_then(|oid| {
                                repository
                                    .read(cx)
                                    .get_graph_data(source.clone(), *order)
                                    .and_then(|data| data.commit_oid_to_index.get(&oid).copied())
                            })
                        {
                            self.select_entry(pending_sha_index, ScrollStrategy::Nearest, cx);
                        }
                    }
                    GitGraphEvent::LoadingError => {
                        // todo(git_graph): Wire this up with the UI
                    }
                    GitGraphEvent::CountUpdated(commit_count) => {
                        let old_count = self.graph_data.commits.len();

                        if let Some(pending_selection_index) =
                            repository.update(cx, |repository, cx| {
                                let GraphDataResponse {
                                    commits,
                                    is_loading,
                                    error: _,
                                } = repository.graph_data(
                                    source.clone(),
                                    *order,
                                    old_count..*commit_count,
                                    cx,
                                );
                                self.graph_data.add_commits(commits);

                                let pending_sha_index = self.pending_select_sha.and_then(|oid| {
                                    repository.get_graph_data(source.clone(), *order).and_then(
                                        |data| data.commit_oid_to_index.get(&oid).copied(),
                                    )
                                });

                                if !is_loading && pending_sha_index.is_none() {
                                    self.pending_select_sha.take();
                                }

                                pending_sha_index
                            })
                        {
                            self.select_entry(pending_selection_index, ScrollStrategy::Nearest, cx);
                            self.pending_select_sha.take();
                        }

                        cx.notify();
                    }
                }
            }
            RepositoryEvent::BranchChanged => {
                self.pending_select_sha = None;
                if self.branch_filter_state.branches_loaded {
                    let previous_selection = self.branch_filter_state.selected_branches.clone();
                    let previously_all_eligible_selected = self.all_branches_selected();
                    let receiver = repository.update(cx, |repository, _| repository.branches());
                    cx.spawn(async move |this, cx| {
                        if let Ok(Ok(branches)) = receiver.await {
                            let _ = this.update(cx, |this, cx| {
                                let available_branches = Self::build_available_branches(branches);
                                this.apply_loaded_available_branches(
                                    available_branches,
                                    true,
                                    previously_all_eligible_selected,
                                    previous_selection.clone(),
                                );
                                this.apply_branch_filter_source(cx);
                                this.refresh_branch_filter_picker(cx);
                            });
                        }
                        anyhow::Ok(())
                    })
                    .detach();
                }
                // Only invalidate if we scanned atleast once,
                // meaning we are not inside the initial repo loading state
                // NOTE: this fixes an loading performance regression
                if repository.read(cx).scan_id > 1 {
                    self.invalidate_state(cx);
                }
            }
            RepositoryEvent::GraphEvent(_, _) => {}
            _ => {}
        }
    }

    fn fetch_initial_graph_data(&mut self, cx: &mut App) {
        if let Some(repository) = self.get_repository(cx) {
            repository.update(cx, |repository, cx| {
                let commits = repository
                    .graph_data(self.log_source.clone(), self.log_order, 0..usize::MAX, cx)
                    .commits;
                self.graph_data.add_commits(commits);
            });
        }
    }

    fn get_repository(&self, cx: &App) -> Option<Entity<Repository>> {
        let git_store = self.git_store.read(cx);
        git_store.repositories().get(&self.repo_id).cloned()
    }

    fn selected_commit_info(&self, cx: &App) -> Option<SelectedCommitInfo> {
        let index = self.selected_entry_idx?;
        let commit = self.graph_data.commits.get(index)?;
        let repository = self.get_repository(cx)?;
        let subject = match repository.read(cx).commit_data_state(commit.data.sha) {
            Some(CommitDataState::Loaded(data)) => Some(data.subject.clone()),
            _ => None,
        };

        Some(SelectedCommitInfo {
            index,
            sha: commit.data.sha.to_string().into(),
            subject,
        })
    }

    fn prompt_confirmation(
        &self,
        level: PromptLevel,
        message: impl Into<SharedString>,
        detail: Option<SharedString>,
        confirm_label: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<bool>> {
        let message = message.into();
        let detail = detail.map(|detail| detail.to_string());
        let answer = window.prompt(
            level,
            message.as_ref(),
            detail.as_deref(),
            &[confirm_label, "Cancel"],
            cx,
        );

        cx.spawn(async move |_, _| Ok(answer.await? == 0))
    }

    fn prompt_reset_mode(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<ResetPromptMode>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(None);
        };

        let options = ResetPromptMode::ALL
            .into_iter()
            .map(|mode| SharedString::from(mode.label()))
            .collect::<Vec<_>>();
        let workspace = workspace.downgrade();
        let picker = picker_prompt::prompt("Select reset mode…", options, workspace, window, cx);

        window.spawn(cx, async move |_| {
            picker
                .await
                .and_then(|index| ResetPromptMode::ALL.get(index).copied())
        })
    }

    fn set_context_menu(
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
                this.context_menu = None;
                this.commit_context_menu_state = None;
                cx.notify();
            },
        );

        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn deploy_commit_context_menu(
        &mut self,
        position: Point<Pixels>,
        row_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_entry(row_index, ScrollStrategy::Nearest, cx);

        let Some(commit) = self.graph_data.commits.get(row_index) else {
            return;
        };
        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        let sha = commit.data.sha.to_string();
        let receiver = repository.update(cx, |repository, _| repository.drop_commit_support(sha));

        cx.spawn_in(window, async move |this, cx| {
            let drop_support = receiver
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))
                .and_then(|result| result)
                .unwrap_or_else(|error| DropCommitSupport {
                    can_drop: false,
                    reason: Some(SharedString::from(error.to_string())),
                });

            let _ = this.update_in(cx, |this, window, cx| {
                this.commit_context_menu_state = Some(CommitContextMenuState {
                    row_index,
                    drop_support,
                });
                if let Some(context_menu) = this.build_commit_context_menu(window, cx) {
                    this.set_context_menu(context_menu, position, window, cx);
                }
            });
        })
        .detach();
    }

    fn build_commit_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let selected_commit = self.selected_commit_info(cx)?;
        if self
            .commit_context_menu_state
            .as_ref()
            .is_none_or(|context_state| context_state.row_index != selected_commit.index)
        {
            return None;
        }

        let copy_subject_disabled = selected_commit.subject.is_none();
        let focus_handle = self.focus_handle.clone();

        Some(ContextMenu::build(window, cx, |context_menu, _, _| {
            context_menu
                .context(focus_handle)
                .action("Create Tag...", AddTag.boxed_clone())
                .action("Create Branch...", CreateBranchAtCommit.boxed_clone())
                .separator()
                .action("Checkout Commit...", CheckoutCommit.boxed_clone())
                .action("Cherry-Pick Commit...", CherryPickCommit.boxed_clone())
                .action("Revert Commit...", RevertCommit.boxed_clone())
                .action(
                    "Reset Current Branch to This Commit...",
                    ResetCommit.boxed_clone(),
                )
                .separator()
                .action("Copy Commit Hash", CopyCommitHash.boxed_clone())
                .action_disabled_when(
                    copy_subject_disabled,
                    "Copy Commit Subject",
                    CopyCommitSubject.boxed_clone(),
                )
        }))
    }

    fn deploy_ref_context_menu(
        &mut self,
        position: Point<Pixels>,
        row_index: usize,
        ref_kind: RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_entry(row_index, ScrollStrategy::Nearest, cx);

        let context_menu = match &ref_kind {
            RefNameKind::Branch(_) => {
                self.deploy_branch_context_menu(position, row_index, ref_kind, window, cx);
                return;
            }
            RefNameKind::Tag(_) => self.build_tag_context_menu(&ref_kind, window, cx),
            RefNameKind::Stash(_) => self.build_stash_context_menu(&ref_kind, window, cx),
        };

        if let Some(context_menu) = context_menu {
            self.set_context_menu(context_menu, position, window, cx);
        }
    }

    fn deploy_branch_context_menu(
        &mut self,
        position: Point<Pixels>,
        _row_index: usize,
        ref_kind: RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let branch_task = self.resolve_branch(ref_kind, repository, cx);

        cx.spawn_in(window, async move |this, cx| {
            let branch = branch_task.await?;
            this.update_in(cx, |this, window, cx| {
                if let Some(context_menu) = this.build_branch_context_menu(branch, window, cx) {
                    this.set_context_menu(context_menu, position, window, cx);
                }
            })?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to open branch menu", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn build_branch_context_menu(
        &self,
        branch: Branch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let branch_name: SharedString = branch.name().to_string().into();
        let focus_handle = self.focus_handle.clone();
        let weak = cx.weak_entity();
        let is_remote = branch.is_remote();

        Some(ContextMenu::build(window, cx, {
            let branch_name_for_checkout = branch_name.clone();
            let branch_name_for_copy = branch_name.clone();
            let branch_name_for_rename = branch_name.clone();
            let branch_name_for_delete = branch_name.clone();
            let branch_name_for_push = branch_name;
            move |context_menu, _, _| {
                let context_menu =
                    context_menu
                        .context(focus_handle)
                        .entry("Checkout Branch", None, {
                            let branch_name = branch_name_for_checkout.clone();
                            let weak = weak.clone();
                            move |window, cx| {
                                if let Some(entity) = weak.upgrade() {
                                    entity.update(cx, |this, cx| {
                                        this.checkout_branch(branch_name.to_string(), window, cx);
                                    });
                                }
                            }
                        });

                let context_menu = if is_remote {
                    context_menu
                } else {
                    context_menu.entry("Rename Branch...", None, {
                        let branch_name = branch_name_for_rename.clone();
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.rename_branch(branch_name.to_string(), window, cx);
                                });
                            }
                        }
                    })
                };

                let context_menu = context_menu.entry(
                    if is_remote {
                        "Delete Remote-Tracking Branch..."
                    } else {
                        "Delete Branch..."
                    },
                    None,
                    {
                        let branch_name = branch_name_for_delete.clone();
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.delete_branch(
                                        branch_name.to_string(),
                                        is_remote,
                                        window,
                                        cx,
                                    );
                                });
                            }
                        }
                    },
                );

                let context_menu = if is_remote {
                    context_menu
                } else {
                    context_menu.entry("Push Branch...", None, {
                        let branch_name = branch_name_for_push;
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.push_branch(branch_name.to_string(), window, cx);
                                });
                            }
                        }
                    })
                };

                context_menu
                    .separator()
                    .entry("Merge Branch into Current Branch...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.merge_selected_commit(window, cx);
                                });
                            }
                        }
                    })
                    .entry("Rebase Current Branch onto Branch...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.rebase_selected_commit(window, cx);
                                });
                            }
                        }
                    })
                    .separator()
                    .action("Copy Branch HEAD Hash", CopyCommitHash.boxed_clone())
                    .entry("Copy Branch Name", None, {
                        let name = branch_name_for_copy;
                        move |_window, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(name.to_string()));
                        }
                    })
            }
        }))
    }

    fn build_tag_context_menu(
        &self,
        ref_kind: &RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let tag_name = ref_kind.display_name();
        let focus_handle = self.focus_handle.clone();
        let weak = cx.weak_entity();

        Some(ContextMenu::build(window, cx, {
            let tag_name_for_delete = tag_name.clone();
            let tag_name_for_copy = tag_name.clone();
            let tag_name_for_push = tag_name;
            move |context_menu, _, _| {
                context_menu
                    .context(focus_handle)
                    .action("Checkout Tag...", CheckoutCommit.boxed_clone())
                    .separator()
                    .entry("Delete Tag...", None, {
                        let tag_name = tag_name_for_delete.clone();
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.delete_tag(tag_name.to_string(), window, cx);
                                });
                            }
                        }
                    })
                    .entry("Push Tag", None, {
                        let tag_name = tag_name_for_push;
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.push_tag(tag_name.to_string(), window, cx);
                                });
                            }
                        }
                    })
                    .entry("Create Branch from Tag...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.show_create_branch_from_tag_modal(window, cx);
                                });
                            }
                        }
                    })
                    .separator()
                    .action("Copy Tagged Commit Hash", CopyCommitHash.boxed_clone())
                    .entry("Copy Tag Name", None, {
                        let name = tag_name_for_copy;
                        move |_window, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(name.to_string()));
                        }
                    })
            }
        }))
    }

    fn build_stash_context_menu(
        &self,
        ref_kind: &RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let stash_name = ref_kind.display_name();
        let stash_index = ref_kind.stash_index();
        let focus_handle = self.focus_handle.clone();
        let weak = cx.weak_entity();

        Some(ContextMenu::build(window, cx, {
            let stash_name_for_copy = stash_name.clone();
            let stash_name_for_branch = stash_name;
            let render_entry = |icon: IconName, label: &'static str| {
                move |_window: &mut Window, _cx: &mut App| {
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                        .child(Label::new(label))
                        .into_any_element()
                }
            };
            move |context_menu, _, _| {
                context_menu
                    .context(focus_handle)
                    .custom_entry(render_entry(IconName::ArrowDown, "Apply Stash"), {
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.apply_stash(stash_index, window, cx);
                                });
                            }
                        }
                    })
                    .custom_entry(render_entry(IconName::ArrowUp, "Pop Stash..."), {
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.pop_stash(stash_index, window, cx);
                                });
                            }
                        }
                    })
                    .custom_entry(render_entry(IconName::Trash, "Drop Stash..."), {
                        let weak = weak.clone();
                        move |window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.drop_stash(stash_index, window, cx);
                                });
                            }
                        }
                    })
                    .separator()
                    .custom_entry(
                        render_entry(IconName::GitBranchPlus, "Create Branch from Stash..."),
                        {
                            let weak = weak.clone();
                            move |window, cx| {
                                if let Some(entity) = weak.upgrade() {
                                    entity.update(cx, |this, cx| {
                                        this.show_create_branch_from_stash_modal(
                                            stash_name_for_branch.to_string(),
                                            window,
                                            cx,
                                        );
                                    });
                                }
                            }
                        },
                    )
                    .separator()
                    .custom_entry(render_entry(IconName::Copy, "Copy Stash Commit Hash"), {
                        let weak = weak.clone();
                        move |_window, cx| {
                            if let Some(entity) = weak.upgrade() {
                                entity.update(cx, |this, cx| {
                                    this.copy_selected_commit_hash(cx);
                                });
                            }
                        }
                    })
                    .entry("Copy Stash Name", None, {
                        let name = stash_name_for_copy;
                        move |_window, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(name.to_string()));
                        }
                    })
            }
        }))
    }

    fn run_git_operation(
        &mut self,
        operation: Task<anyhow::Result<()>>,
        error_message: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_menu = None;
        self.commit_context_menu_state = None;

        cx.spawn(async move |this, cx| {
            operation.await?;

            this.update(cx, |this, cx| {
                this.reload_graph(cx);
            })
            .ok();

            Ok(())
        })
        .detach_and_prompt_err(error_message, window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn show_add_tag_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };
        let workspace = self.workspace.clone();
        let graph = cx.weak_entity();

        if let Some(workspace) = workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    AddTagModal::new(graph, repository, commit.sha.clone(), window, cx)
                });
            });
        }
    }

    fn show_create_branch_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_create_branch_modal_with_options(None, "Create Branch".into(), window, cx);
    }

    fn show_create_branch_from_stash_modal(
        &mut self,
        stash_name: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let suggested_name = if let Some(index) = stash_name
            .strip_prefix("stash@{")
            .and_then(|rest| rest.strip_suffix('}'))
        {
            format!("stash-{}", index)
        } else {
            "stash-branch".to_string()
        };

        self.show_create_branch_modal_with_options(
            Some(suggested_name),
            "Create Branch from Stash".into(),
            window,
            cx,
        );
    }

    fn show_create_branch_from_tag_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_create_branch_modal_with_options(
            None,
            "Create Branch from Tag".into(),
            window,
            cx,
        );
    }

    fn show_create_branch_modal_with_options(
        &mut self,
        initial_name: Option<String>,
        title: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };
        let workspace = self.workspace.clone();
        let graph = cx.weak_entity();

        if let Some(workspace) = workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    CreateBranchAtCommitModal::new(
                        graph,
                        repository,
                        commit.sha.clone(),
                        initial_name,
                        title,
                        window,
                        cx,
                    )
                });
            });
        }
    }

    fn is_visible_ref_name(&self, ref_name: &str) -> bool {
        if !Self::ref_name_matches_remote_visibility(
            self.settings_dropdown_state.settings.show_remote_branches,
            &self.branch_filter_state.available_branches,
            ref_name,
        ) {
            return false;
        }

        if !self.settings_dropdown_state.settings.show_tags
            && (ref_name.starts_with("tag: ") || ref_name.starts_with("refs/tags/"))
        {
            return false;
        }

        if !self.settings_dropdown_state.settings.show_stashes
            && (ref_name == "refs/stash"
                || ref_name == "stash"
                || ref_name.starts_with("stash@{")
                || ref_name.contains("refs/stash"))
        {
            return false;
        }

        true
    }

    fn ref_name_matches_remote_visibility(
        show_remote_branches: bool,
        available_branches: &[BranchInfo],
        ref_name: &str,
    ) -> bool {
        if show_remote_branches {
            return true;
        }

        let ref_name = ref_name.strip_prefix("HEAD -> ").unwrap_or(ref_name);
        if ref_name.starts_with("refs/remotes/") {
            return false;
        }

        !available_branches.iter().any(|branch| {
            branch.is_remote
                && Self::branch_display_name(branch.ref_name.as_ref()).as_ref() == ref_name
        })
    }

    fn visible_ref_names(&self, ref_names: &[SharedString]) -> Vec<SharedString> {
        ref_names
            .iter()
            .filter(|name| self.is_visible_ref_name(name))
            .cloned()
            .collect()
    }

    fn update_graph_settings(
        &mut self,
        update: impl FnOnce(&mut GraphSettings),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let previously_all_eligible_selected = self.all_branches_selected();
        let old_show_remote = self.settings_dropdown_state.settings.show_remote_branches;
        let mut settings = self.settings_dropdown_state.settings;
        update(&mut settings);

        if settings == self.settings_dropdown_state.settings {
            return;
        }

        let new_show_remote = settings.show_remote_branches;
        self.settings_dropdown_state.settings = settings;

        if old_show_remote != new_show_remote {
            if self.branch_filter_state.branches_loaded {
                self.reconcile_loaded_branch_selection(previously_all_eligible_selected);
                self.apply_branch_filter_source(cx);
                self.refresh_branch_filter_picker(cx);
            } else {
                self.load_available_branches(Some(previously_all_eligible_selected), window, cx);
            }
        } else {
            // Other settings changed → update log_source and reload immediately
            self.refresh_branch_filter_picker(cx);
            self.log_source = self.log_source.clone().with_graph_options(settings.into());
            self.reload_graph(cx);
        }
    }

    fn render_chip(&self, name: &SharedString, accent_color: gpui::Hsla) -> impl IntoElement {
        let is_head = name.clone().starts_with("HEAD");
        let multiplier = if is_head { 4.0 } else { 1.0 };
        let is_tag = name.clone().starts_with("tag:");
        let is_stash = name.clone().starts_with("stash@{")
            || name.clone() == "stash"
            || name.clone().starts_with("refs/stash");

        // Extract display name: strip "tag: " or "HEAD -> " prefixes
        let display_name = if is_tag {
            name.trim_start_matches("tag: ").to_string().into()
        } else if name.clone().starts_with("HEAD -> ") {
            name.trim_start_matches("HEAD -> ").to_string().into()
        } else {
            name.clone()
        };

        let mut chip = Chip::new(display_name)
            .label_size(LabelSize::Small)
            .bg_color(accent_color.opacity(0.1 * multiplier))
            .border_color(accent_color.opacity(0.5 * multiplier))
            .leading_icon(if is_tag {
                Icon::new(IconName::GitTag)
            } else if is_stash {
                Icon::new(IconName::GitStash)
            } else {
                Icon::new(IconName::GitBranch)
            });

        if is_head {
            chip = chip.weight(FontWeight::SEMIBOLD);
        }

        chip
    }

    fn render_interactive_chip(
        &self,
        name: &SharedString,
        accent_color: gpui::Hsla,
        row_index: usize,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let ref_kind = RefNameKind::classify(name);
        let weak = cx.weak_entity();
        let chip_id = ElementId::Name(format!("ref-chip-{}-{}", row_index, name.as_ref()).into());

        div()
            .id(chip_id)
            .child(self.render_chip(name, accent_color))
            .when_some(ref_kind, move |this, ref_kind| {
                this.on_mouse_down(
                    MouseButton::Right,
                    move |event: &MouseDownEvent, window, cx| {
                        if event.button != MouseButton::Right {
                            return;
                        }
                        let ref_kind = ref_kind.clone();
                        if let Some(entity) = weak.upgrade() {
                            entity.update(cx, |this, cx| {
                                this.deploy_ref_context_menu(
                                    event.position,
                                    row_index,
                                    ref_kind,
                                    window,
                                    cx,
                                );
                            });
                        }
                        cx.stop_propagation();
                    },
                )
            })
    }

    fn render_table_rows(
        &mut self,
        range: Range<usize>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Vec<AnyElement>> {
        let repository = self.get_repository(cx);

        let row_height = self.row_height;

        // We fetch data outside the visible viewport to avoid loading entries when
        // users scroll through the git graph
        if let Some(repository) = repository.as_ref() {
            const FETCH_RANGE: usize = 100;
            repository.update(cx, |repository, cx| {
                self.graph_data.commits[range.start.saturating_sub(FETCH_RANGE)
                    ..(range.end + FETCH_RANGE)
                        .min(self.graph_data.commits.len().saturating_sub(1))]
                    .iter()
                    .for_each(|commit| {
                        repository.fetch_commit_data(commit.data.sha, cx);
                    });
            });
        }

        range
            .map(|idx| {
                let Some((commit, repository)) =
                    self.graph_data.commits.get(idx).zip(repository.as_ref())
                else {
                    return vec![
                        div().h(row_height).into_any_element(),
                        div().h(row_height).into_any_element(),
                        div().h(row_height).into_any_element(),
                        div().h(row_height).into_any_element(),
                    ];
                };

                let data = repository.update(cx, |repository, cx| {
                    repository.fetch_commit_data(commit.data.sha, cx).clone()
                });

                let short_sha = commit.data.sha.display_short();
                let mut formatted_time = String::new();
                let subject: SharedString;
                let author_name: SharedString;

                if let CommitDataState::Loaded(data) = data {
                    subject = data.subject.clone();
                    author_name = data.author_name.clone();
                    formatted_time = format_timestamp(data.commit_timestamp);
                } else {
                    subject = "Loading…".into();
                    author_name = "".into();
                }

                let accent_colors = cx.theme().accents();
                let accent_color = accent_colors
                    .0
                    .get(commit.color_idx)
                    .copied()
                    .unwrap_or_else(|| accent_colors.0.first().copied().unwrap_or_default());
                let visible_ref_names = self.visible_ref_names(&commit.data.ref_names);

                let is_selected = self.selected_entry_idx == Some(idx);
                let is_matched = self.search_state.matches.contains(&commit.data.sha);
                let column_label = |label: SharedString| {
                    Label::new(label)
                        .when(!is_selected, |c| c.color(Color::Muted))
                        .truncate()
                        .into_any_element()
                };

                let subject_label = if is_matched {
                    let query = match &self.search_state.state {
                        QueryState::Confirmed((query, _)) => Some(query.clone()),
                        _ => None,
                    };
                    let highlight_ranges = query
                        .and_then(|q| {
                            let ranges = if self.search_state.case_sensitive {
                                subject
                                    .match_indices(q.as_str())
                                    .map(|(start, matched)| start..start + matched.len())
                                    .collect::<Vec<_>>()
                            } else {
                                let q = q.to_lowercase();
                                let subject_lower = subject.to_lowercase();

                                subject_lower
                                    .match_indices(&q)
                                    .filter_map(|(start, matched)| {
                                        let end = start + matched.len();
                                        subject.is_char_boundary(start).then_some(()).and_then(
                                            |_| subject.is_char_boundary(end).then_some(start..end),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                            };

                            (!ranges.is_empty()).then_some(ranges)
                        })
                        .unwrap_or_default();
                    HighlightedLabel::from_ranges(subject, highlight_ranges)
                        .when(!is_selected, |c| c.color(Color::Muted))
                        .truncate()
                        .into_any_element()
                } else {
                    column_label(subject)
                };

                vec![
                    div()
                        .id(ElementId::NamedInteger("commit-subject".into(), idx as u64))
                        .overflow_hidden()
                        .child(
                            h_flex()
                                .gap_2()
                                .overflow_hidden()
                                .children((!visible_ref_names.is_empty()).then(|| {
                                    h_flex().gap_1().children(visible_ref_names.iter().map(
                                        |name| {
                                            self.render_interactive_chip(
                                                name,
                                                accent_color,
                                                idx,
                                                cx,
                                            )
                                            .into_any_element()
                                        },
                                    ))
                                }))
                                .child(subject_label),
                        )
                        .into_any_element(),
                    column_label(formatted_time.into()),
                    column_label(author_name),
                    column_label(short_sha.into()),
                ]
            })
            .collect()
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        self.selected_entry_idx = None;
        self.selected_commit_diff = None;
        self.selected_commit_diff_stats = None;
        cx.notify();
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        self.select_entry(0, ScrollStrategy::Nearest, cx);
    }

    fn select_prev(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected_entry_idx) = &self.selected_entry_idx {
            self.select_entry(
                selected_entry_idx.saturating_sub(1),
                ScrollStrategy::Nearest,
                cx,
            );
        } else {
            self.select_first(&SelectFirst, window, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(selected_entry_idx) = &self.selected_entry_idx {
            self.select_entry(
                selected_entry_idx
                    .saturating_add(1)
                    .min(self.graph_data.commits.len().saturating_sub(1)),
                ScrollStrategy::Nearest,
                cx,
            );
        } else {
            self.select_prev(&SelectPrevious, window, cx);
        }
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        self.select_entry(
            self.graph_data.commits.len().saturating_sub(1),
            ScrollStrategy::Nearest,
            cx,
        );
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.open_selected_commit_view(window, cx);
    }

    fn search(&mut self, query: SharedString, cx: &mut Context<Self>) {
        let Some(repo) = self.get_repository(cx) else {
            return;
        };

        self.search_state.matches.clear();
        self.search_state.selected_index = None;
        self.search_state.editor.update(cx, |editor, _cx| {
            editor.set_text_style_refinement(Default::default());
        });

        if query.as_str().is_empty() {
            self.search_state.state = QueryState::Empty;
            cx.notify();
            return;
        }

        let (request_tx, request_rx) = smol::channel::unbounded::<Oid>();

        repo.update(cx, |repo, cx| {
            repo.search_commits(
                self.log_source.clone(),
                SearchCommitArgs {
                    query: query.clone(),
                    case_sensitive: self.search_state.case_sensitive,
                },
                request_tx,
                cx,
            );
        });

        let search_task = cx.spawn(async move |this, cx| {
            while let Ok(first_oid) = request_rx.recv().await {
                let mut pending_oids = vec![first_oid];
                while let Ok(oid) = request_rx.try_recv() {
                    pending_oids.push(oid);
                }

                this.update(cx, |this, cx| {
                    if this.search_state.selected_index.is_none() {
                        this.search_state.selected_index = Some(0);
                        this.select_commit_by_sha(first_oid, cx);
                    }

                    this.search_state.matches.extend(pending_oids);
                    cx.notify();
                })
                .ok();
            }

            this.update(cx, |this, cx| {
                if this.search_state.matches.is_empty() {
                    this.search_state.editor.update(cx, |editor, cx| {
                        editor.set_text_style_refinement(TextStyleRefinement {
                            color: Some(Color::Error.color(cx)),
                            ..Default::default()
                        });
                    });
                }
            })
            .ok();
        });

        self.search_state.state = QueryState::Confirmed((query, search_task));
    }

    fn confirm_search(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let query = self.search_state.editor.read(cx).text(cx).into();
        self.search(query, cx);
    }

    fn select_entry(
        &mut self,
        idx: usize,
        scroll_strategy: ScrollStrategy,
        cx: &mut Context<Self>,
    ) {
        if self.selected_entry_idx == Some(idx) {
            return;
        }

        self.selected_entry_idx = Some(idx);
        self.selected_commit_diff = None;
        self.selected_commit_diff_stats = None;
        self.changed_files_scroll_handle
            .scroll_to_item(0, ScrollStrategy::Top);
        self.table_interaction_state.update(cx, |state, cx| {
            state.scroll_handle.scroll_to_item(idx, scroll_strategy);
            cx.notify();
        });

        let Some(commit) = self.graph_data.commits.get(idx) else {
            return;
        };

        let sha = commit.data.sha.to_string();

        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        let diff_receiver = repository.update(cx, |repo, _| repo.load_commit_diff(sha));

        self._commit_diff_task = Some(cx.spawn(async move |this, cx| {
            if let Ok(Ok(diff)) = diff_receiver.await {
                this.update(cx, |this, cx| {
                    let stats = compute_diff_stats(&diff);
                    this.selected_commit_diff = Some(diff);
                    this.selected_commit_diff_stats = Some(stats);
                    cx.notify();
                })
                .ok();
            }
        }));

        cx.notify();
    }

    fn select_previous_match(&mut self, cx: &mut Context<Self>) {
        if self.search_state.matches.is_empty() {
            return;
        }

        let mut prev_selection = self.search_state.selected_index.unwrap_or_default();

        if prev_selection == 0 {
            prev_selection = self.search_state.matches.len() - 1;
        } else {
            prev_selection -= 1;
        }

        let Some(&oid) = self.search_state.matches.get_index(prev_selection) else {
            return;
        };

        self.search_state.selected_index = Some(prev_selection);
        self.select_commit_by_sha(oid, cx);
    }

    fn select_next_match(&mut self, cx: &mut Context<Self>) {
        if self.search_state.matches.is_empty() {
            return;
        }

        let mut next_selection = self
            .search_state
            .selected_index
            .map(|index| index + 1)
            .unwrap_or_default();

        if next_selection >= self.search_state.matches.len() {
            next_selection = 0;
        }

        let Some(&oid) = self.search_state.matches.get_index(next_selection) else {
            return;
        };

        self.search_state.selected_index = Some(next_selection);
        self.select_commit_by_sha(oid, cx);
    }

    pub fn set_repo_id(&mut self, repo_id: RepositoryId, cx: &mut Context<Self>) {
        if repo_id != self.repo_id
            && self
                .git_store
                .read(cx)
                .repositories()
                .contains_key(&repo_id)
        {
            self.repo_id = repo_id;
            self.invalidate_state(cx);
        }
    }

    pub fn select_commit_by_sha(&mut self, sha: impl TryInto<Oid>, cx: &mut Context<Self>) {
        fn inner(this: &mut GitGraph, oid: Oid, cx: &mut Context<GitGraph>) {
            let Some(selected_repository) = this.get_repository(cx) else {
                return;
            };

            let Some(index) = selected_repository
                .read(cx)
                .get_graph_data(this.log_source.clone(), this.log_order)
                .and_then(|data| data.commit_oid_to_index.get(&oid))
                .copied()
            else {
                return;
            };

            this.select_entry(index, ScrollStrategy::Center, cx);
        }

        if let Ok(oid) = sha.try_into() {
            inner(self, oid, cx);
        }
    }

    fn open_selected_commit_view(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(selected_entry_index) = self.selected_entry_idx else {
            return;
        };

        self.open_commit_view(selected_entry_index, window, cx);
    }

    fn open_commit_view(
        &mut self,
        entry_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(commit_entry) = self.graph_data.commits.get(entry_index) else {
            return;
        };

        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        CommitView::open(
            commit_entry.data.sha.to_string(),
            repository.downgrade(),
            self.workspace.clone(),
            None,
            None,
            window,
            cx,
        );
    }

    fn copy_selected_commit_hash(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };

        cx.write_to_clipboard(ClipboardItem::new_string(commit.sha.to_string()));
    }

    fn copy_selected_commit_subject(&mut self, cx: &mut Context<Self>) {
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };
        let Some(subject) = commit.subject else {
            return;
        };

        cx.write_to_clipboard(ClipboardItem::new_string(subject.to_string()));
    }

    fn askpass_delegate(
        &self,
        operation: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AskPassDelegate {
        let workspace = self.workspace.clone();
        let operation = operation.into();
        let window = window.window_handle();
        AskPassDelegate::new(&mut cx.to_async(), move |prompt, tx, cx| {
            window
                .update(cx, |_, window, cx| {
                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            workspace.toggle_modal(window, cx, |window, cx| {
                                GitGraphAskPassModal::new(
                                    operation.clone(),
                                    prompt.into(),
                                    tx,
                                    window,
                                    cx,
                                )
                            });
                        });
                    }
                })
                .ok();
        })
    }

    fn resolve_branch(
        &self,
        ref_kind: RefNameKind,
        repository: Entity<Repository>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Branch>> {
        let branch_name = ref_kind
            .branch_lookup_name()
            .unwrap_or_else(|| ref_kind.display_name());
        let receiver = repository.update(cx, |repository, _| repository.branches());

        cx.spawn(async move |_, _| {
            let branches = receiver
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
            branches
                .into_iter()
                .find(|branch| branch.name() == branch_name.as_ref())
                .ok_or_else(|| anyhow::anyhow!("Branch '{}' not found", branch_name))
        })
    }

    fn checkout_branch(
        &mut self,
        branch_name: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        self.context_menu = None;
        self.commit_context_menu_state = None;

        let task = cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| repository.change_branch(branch_name))
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
            Ok(())
        });
        self.run_git_operation(task, "Failed to checkout branch", window, cx);
    }

    fn push_branch(&mut self, branch_name: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let branches_receiver = repository.update(cx, |repository, _| repository.branches());
        let remotes_receiver = repository.update(cx, |repository, _| {
            repository.get_remotes(Some(branch_name.clone()), true)
        });

        self.context_menu = None;
        self.commit_context_menu_state = None;

        cx.spawn_in(window, async move |this, cx| {
            let branches = branches_receiver
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
            let remotes = remotes_receiver
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
            let branch = branches
                .into_iter()
                .find(|branch| branch.name() == branch_name.as_str())
                .ok_or_else(|| anyhow::anyhow!("Branch '{}' not found", branch_name))?;
            if branch.is_remote() {
                anyhow::bail!("Cannot push a remote-tracking branch");
            }

            let dialog_state = PushBranchDialogState::new(
                branch,
                remotes.into_iter().map(|remote| remote.name).collect(),
            )?;

            let _ = this.update_in(cx, |this, window, cx| {
                let Some(workspace) = this.workspace.upgrade() else {
                    return anyhow::Ok(());
                };
                let graph = cx.weak_entity();
                workspace.update(cx, |workspace, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        PushBranchModal::new(graph.clone(), dialog_state.clone(), window, cx)
                    });
                });
                anyhow::Ok(())
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to push branch", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn perform_push_branch(
        &mut self,
        target: BranchPushTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let branch_name: SharedString = target.branch.name().to_string().into();
        let remote_branch_name = target.remote_branch_name.clone();
        let remote_name = target.remote.name.clone();
        let options = target.options;
        let askpass = self.askpass_delegate(format!("git push {}", remote_name), window, cx);

        let task = cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, cx| {
                    repository.push(
                        branch_name,
                        remote_branch_name,
                        remote_name,
                        options,
                        askpass,
                        cx,
                    )
                })
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
            Ok(())
        });
        self.run_git_operation(task, "Failed to push branch", window, cx);
    }

    fn push_tag(&mut self, tag_name: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(remote_name) = repository
            .read(cx)
            .remote_upstream_url
            .as_ref()
            .map(|_| SharedString::from("upstream"))
            .or_else(|| {
                repository
                    .read(cx)
                    .remote_origin_url
                    .as_ref()
                    .map(|_| SharedString::from("origin"))
            })
        else {
            let prompt = window.prompt(
                PromptLevel::Warning,
                "No remote configured for repository",
                None,
                &["Ok"],
                cx,
            );
            cx.spawn(async move |_, _| {
                prompt.await.ok();
                anyhow::Ok(())
            })
            .detach();
            return;
        };

        let askpass = self.askpass_delegate(format!("git push {}", remote_name), window, cx);

        let task = cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, cx| {
                    repository.push_tag(tag_name.into(), remote_name, askpass, cx)
                })
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
            Ok(())
        });
        self.run_git_operation(task, "Failed to push tag", window, cx);
    }

    fn checkout_selected_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };

        let confirm = self.prompt_confirmation(
            PromptLevel::Warning,
            format!("Checkout {} in detached HEAD state?", commit.sha),
            Some("This will detach HEAD at the selected commit.".into()),
            "Checkout",
            window,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                let sha = commit.sha.to_string();
                let repository = repository.clone();
                let task = cx.spawn(async move |_, cx| {
                    repository
                        .update(cx, |repository, _| repository.checkout_commit(sha))
                        .await
                        .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
                    Ok(())
                });
                this.run_git_operation(task, "Failed to checkout commit", window, cx);
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to checkout commit", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn cherry_pick_selected_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };
        let workspace = self.workspace.clone();
        let graph = cx.weak_entity();

        if let Some(workspace) = workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    CherryPickModal::new(graph, repository, commit.sha.clone(), window, cx)
                });
            });
        }
    }

    fn revert_selected_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let graph = cx.weak_entity();

        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                RevertCommitModal::new(graph, repository, commit.sha.clone(), window, cx)
            });
        });
    }

    fn drop_selected_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };
        let Some(context_state) = self.commit_context_menu_state.as_ref() else {
            return;
        };
        if !context_state.drop_support.can_drop {
            return;
        }

        let confirm = self.prompt_confirmation(
            PromptLevel::Warning,
            format!("Drop commit {}?", commit.sha),
            Some("This rewrites history on the current branch.".into()),
            "Drop Commit",
            window,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                let sha = commit.sha.to_string();
                let repository = repository.clone();
                let task = cx.spawn(async move |_, cx| {
                    repository
                        .update(cx, |repository, _| repository.drop_commit(sha))
                        .await
                        .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
                    Ok(())
                });
                this.run_git_operation(task, "Failed to drop commit", window, cx);
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to drop commit", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn merge_selected_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };

        let confirm = self.prompt_confirmation(
            PromptLevel::Warning,
            format!("Merge commit {} into the current branch?", commit.sha),
            None,
            "Merge",
            window,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                let sha = commit.sha.to_string();
                let repository = repository.clone();
                let task = cx.spawn(async move |_, cx| {
                    repository
                        .update(cx, |repository, _| repository.merge_commit(sha))
                        .await
                        .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
                    Ok(())
                });
                this.run_git_operation(task, "Failed to merge commit", window, cx);
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to merge commit", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn rebase_selected_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };

        let confirm = self.prompt_confirmation(
            PromptLevel::Warning,
            format!("Rebase the current branch onto {}?", commit.sha),
            None,
            "Rebase",
            window,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                let sha = commit.sha.to_string();
                let repository = repository.clone();
                let task = cx.spawn(async move |_, cx| {
                    repository
                        .update(cx, |repository, _| repository.rebase_onto(sha))
                        .await
                        .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
                    Ok(())
                });
                this.run_git_operation(task, "Failed to rebase current branch", window, cx);
            })?;

            Ok(())
        })
        .detach_and_prompt_err(
            "Failed to rebase current branch",
            window,
            cx,
            |error, _, _| Some(error.to_string()),
        );
    }

    fn reset_selected_commit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(commit) = self.selected_commit_info(cx) else {
            return;
        };

        let reset_mode_prompt = self.prompt_reset_mode(window, cx);

        cx.spawn_in(window, async move |this, cx| {
            let Some(prompt_mode) = reset_mode_prompt.await else {
                return Ok(());
            };

            let confirm = this.update_in(cx, |this, window, cx| {
                let detail = match prompt_mode {
                    ResetPromptMode::Hard => {
                        Some("Hard reset will discard working tree and index changes.".into())
                    }
                    ResetPromptMode::Soft => {
                        Some("Soft reset moves the branch pointer and keeps changes staged.".into())
                    }
                    ResetPromptMode::Mixed => Some(
                        "Mixed reset moves the branch pointer and keeps changes unstaged.".into(),
                    ),
                };
                this.prompt_confirmation(
                    PromptLevel::Warning,
                    format!(
                        "Reset the current branch to {} using {} mode?",
                        commit.sha,
                        prompt_mode.label()
                    ),
                    detail,
                    "Reset",
                    window,
                    cx,
                )
            })?;

            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                let sha = commit.sha.to_string();
                let repository = repository.clone();
                let task = cx.spawn(async move |_, cx| {
                    repository
                        .update(cx, |repository, cx| {
                            repository.reset(sha, prompt_mode.to_reset_mode(), cx)
                        })
                        .await
                        .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
                    Ok(())
                });
                this.run_git_operation(task, "Failed to reset current branch", window, cx);
            })?;

            Ok(())
        })
        .detach_and_prompt_err(
            "Failed to reset current branch",
            window,
            cx,
            |error, _, _| Some(error.to_string()),
        );
    }

    fn delete_tag(&mut self, tag_name: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        let confirm = self.prompt_confirmation(
            PromptLevel::Warning,
            format!("Delete tag '{}'?", tag_name),
            None,
            "Delete",
            window,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                let tag_name = tag_name.clone();
                let repository = repository.clone();
                let task = cx.spawn(async move |_, cx| {
                    repository
                        .update(cx, |repository, _| repository.delete_tag(tag_name))
                        .await
                        .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
                    Ok(())
                });
                this.run_git_operation(task, "Failed to delete tag", window, cx);
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to delete tag", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn delete_branch(
        &mut self,
        branch_name: String,
        is_remote: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let graph = cx.weak_entity();

        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                DeleteBranchModal::new(graph, branch_name, is_remote, window, cx)
            });
        });
    }

    fn perform_delete_branch(
        &mut self,
        branch_name: String,
        is_remote: bool,
        force_delete: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        let task = cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.delete_branch(is_remote, branch_name, force_delete)
                })
                .await
                .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;
            Ok(())
        });
        self.run_git_operation(task, "Failed to delete branch", window, cx);
    }

    fn rename_branch(&mut self, branch_name: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let graph = cx.weak_entity();

        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                RenameBranchModal::new(branch_name, repository, graph, window, cx)
            });
        });
    }

    fn apply_stash(
        &mut self,
        stash_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        self.context_menu = None;
        self.commit_context_menu_state = None;

        let task = repository.update(cx, |repository, cx| repository.stash_apply(stash_index, cx));

        cx.spawn(async move |this, cx| {
            task.await?;

            this.update(cx, |this, cx| {
                this.reload_graph(cx);
            })
            .ok();

            Ok(())
        })
        .detach_and_prompt_err("Failed to apply stash", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn pop_stash(
        &mut self,
        stash_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        let confirm = self.prompt_confirmation(
            PromptLevel::Warning,
            "Pop stash? This will apply and remove the stash entry.",
            None,
            "Pop",
            window,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                this.context_menu = None;
                this.commit_context_menu_state = None;

                let task =
                    repository.update(cx, |repository, cx| repository.stash_pop(stash_index, cx));

                cx.spawn(async move |this, cx| {
                    task.await?;

                    this.update(cx, |this, cx| {
                        this.reload_graph(cx);
                    })
                    .ok();

                    Ok(())
                })
                .detach_and_prompt_err(
                    "Failed to pop stash",
                    window,
                    cx,
                    |error, _, _| Some(error.to_string()),
                );
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to pop stash", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn drop_stash(
        &mut self,
        stash_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.get_repository(cx) else {
            return;
        };

        let confirm = self.prompt_confirmation(
            PromptLevel::Warning,
            "Drop stash? This will permanently remove the stash entry.",
            None,
            "Drop",
            window,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            if !confirm.await? {
                return Ok(());
            }

            this.update_in(cx, |this, _window, cx| {
                this.context_menu = None;
                this.commit_context_menu_state = None;

                let receiver =
                    repository.update(cx, |repository, cx| repository.stash_drop(stash_index, cx));

                cx.spawn(async move |this, cx| {
                    receiver
                        .await
                        .map_err(|_| anyhow::anyhow!("Operation was canceled"))??;

                    this.update(cx, |this, cx| {
                        this.reload_graph(cx);
                    })
                    .ok();

                    Ok::<(), anyhow::Error>(())
                })
                .detach();
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to drop stash", window, cx, |error, _, _| {
            Some(error.to_string())
        });
    }

    fn get_remote(
        &self,
        repository: &Repository,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<GitRemote> {
        let remote_url = repository.default_remote_url()?;
        let provider_registry = GitHostingProviderRegistry::default_global(cx);
        let (provider, parsed) = parse_git_remote_url(provider_registry, &remote_url)?;
        Some(GitRemote {
            host: provider,
            owner: parsed.owner.into(),
            repo: parsed.repo.into(),
        })
    }

    fn render_settings_button(&self, _cx: &mut Context<Self>) -> PopoverMenu<ContextMenu> {
        let settings = self.settings_dropdown_state.settings;

        let render_setting = |id_suffix: &'static str, label: &'static str, enabled: bool| {
            move |_window: &mut Window, _cx: &mut App| {
                Checkbox::new(
                    format!("git-graph-settings-checkbox-{id_suffix}"),
                    if enabled {
                        ToggleState::Selected
                    } else {
                        ToggleState::Unselected
                    },
                )
                .label(label)
                .label_size(LabelSize::Small)
                .label_color(Color::Default)
                .visualization_only(true)
                .into_any_element()
            }
        };

        PopoverMenu::new("git-graph-settings")
            .trigger_with_tooltip(
                IconButton::new("toggle-git-graph-settings", IconName::Settings)
                    .shape(ui::IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(self.settings_dropdown_state.handle.is_deployed()),
                Tooltip::text("Git Graph Settings"),
            )
            .anchor(Corner::TopRight)
            .with_handle(self.settings_dropdown_state.handle.clone())
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, move |menu, _window, _cx| {
                    menu.custom_entry(
                        render_setting("show-stashes", "Show Stashes", settings.show_stashes),
                        move |window, cx| {
                            window.dispatch_action(Box::new(ToggleShowStashes), cx);
                        },
                    )
                    .custom_entry(
                        render_setting("show-tags", "Show Tags", settings.show_tags),
                        move |window, cx| {
                            window.dispatch_action(Box::new(ToggleShowTags), cx);
                        },
                    )
                    .custom_entry(
                        render_setting(
                            "show-remote-branches",
                            "Show Remote Branches",
                            settings.show_remote_branches,
                        ),
                        move |window, cx| {
                            window.dispatch_action(Box::new(ToggleShowRemoteBranches), cx);
                        },
                    )
                    .custom_entry(
                        render_setting(
                            "include-reflog-commits",
                            "Include commits only mentioned by reflogs",
                            settings.include_reflog_commits,
                        ),
                        move |window, cx| {
                            window.dispatch_action(Box::new(ToggleReflogCommits), cx);
                        },
                    )
                    .custom_entry(
                        render_setting(
                            "first-parent-only",
                            "Only follow the first parent of commits",
                            settings.first_parent_only,
                        ),
                        move |window, cx| {
                            window.dispatch_action(Box::new(ToggleFirstParentOnly), cx);
                        },
                    )
                }))
            })
    }

    fn render_branch_filter_button(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let eligible_branches = self.eligible_branch_infos();
        let selected_eligible = eligible_branches
            .iter()
            .filter(|branch| {
                self.branch_filter_state
                    .selected_branches
                    .contains(&branch.ref_name)
            })
            .count();

        let label = if !self.branch_filter_state.branches_loaded || self.all_branches_selected() {
            "Branch".to_string()
        } else {
            if selected_eligible == 1 {
                eligible_branches
                    .iter()
                    .find(|branch| {
                        self.branch_filter_state
                            .selected_branches
                            .contains(&branch.ref_name)
                    })
                    .map(|branch| Self::branch_display_name(branch.ref_name.as_ref()).to_string())
                    .unwrap_or_else(|| "Branch".to_string())
            } else {
                format!("{selected_eligible} branches")
            }
        };

        let weak = cx.weak_entity();

        PickerPopoverMenu::new(
            self.branch_filter_picker.clone(),
            ButtonLike::new("toggle-git-graph-branch-filter")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .on_mouse_down(MouseButton::Left, {
                            move |_, window, cx| {
                                if let Some(entity) = weak.upgrade() {
                                    entity.update(cx, |this, cx| {
                                        if !this.branch_filter_state.branches_loaded {
                                            this.load_available_branches(None, window, cx);
                                        }
                                    });
                                }
                            }
                        })
                        .child(Label::new(label))
                        .child(Icon::new(IconName::ChevronDown).size(IconSize::Small)),
                ),
            Tooltip::text("Filter branches"),
            Corner::TopRight,
            cx,
        )
        .with_handle(self.branch_filter_state.handle.clone())
        .render(window, cx)
        .into_any_element()
    }

    fn render_search_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let color = cx.theme().colors();
        let query_focus_handle = self.search_state.editor.focus_handle(cx);
        let search_options = {
            let mut options = SearchOptions::NONE;
            options.set(
                SearchOptions::CASE_SENSITIVE,
                self.search_state.case_sensitive,
            );
            options
        };

        h_flex()
            .w_full()
            .p_1p5()
            .gap_1p5()
            .border_b_1()
            .border_color(color.border_variant)
            .child(
                h_flex()
                    .h_8()
                    .flex_1()
                    .min_w_0()
                    .px_1p5()
                    .gap_1()
                    .border_1()
                    .border_color(color.border)
                    .rounded_md()
                    .bg(color.toolbar_background)
                    .on_action(cx.listener(Self::confirm_search))
                    .child(self.search_state.editor.clone())
                    .child(SearchOption::CaseSensitive.as_button(
                        search_options,
                        SearchSource::Buffer,
                        query_focus_handle,
                    )),
            )
            .child(self.render_branch_filter_button(window, cx))
            .child(self.render_settings_button(cx))
            .child(
                h_flex()
                    .min_w_64()
                    .gap_1()
                    .child({
                        let focus_handle = self.focus_handle.clone();
                        IconButton::new("git-graph-search-prev", IconName::ChevronLeft)
                            .shape(ui::IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .tooltip(move |_, cx| {
                                Tooltip::for_action_in(
                                    "Select Previous Match",
                                    &SelectPreviousMatch,
                                    &focus_handle,
                                    cx,
                                )
                            })
                            .map(|this| {
                                if self.search_state.matches.is_empty() {
                                    this.disabled(true)
                                } else {
                                    this.disabled(false).on_click(cx.listener(|this, _, _, cx| {
                                        this.select_previous_match(cx);
                                    }))
                                }
                            })
                    })
                    .child({
                        let focus_handle = self.focus_handle.clone();
                        IconButton::new("git-graph-search-next", IconName::ChevronRight)
                            .shape(ui::IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .tooltip(move |_, cx| {
                                Tooltip::for_action_in(
                                    "Select Next Match",
                                    &SelectNextMatch,
                                    &focus_handle,
                                    cx,
                                )
                            })
                            .map(|this| {
                                if self.search_state.matches.is_empty() {
                                    this.disabled(true)
                                } else {
                                    this.disabled(false).on_click(cx.listener(|this, _, _, cx| {
                                        this.select_next_match(cx);
                                    }))
                                }
                            })
                    })
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(
                                Label::new(format!(
                                    "{}/{}",
                                    self.search_state
                                        .selected_index
                                        .map(|index| index + 1)
                                        .unwrap_or(0),
                                    self.search_state.matches.len()
                                ))
                                .size(LabelSize::Small)
                                .when(self.search_state.matches.is_empty(), |this| {
                                    this.color(Color::Disabled)
                                }),
                            )
                            .when(
                                matches!(
                                    &self.search_state.state,
                                    QueryState::Confirmed((_, task)) if !task.is_ready()
                                ),
                                |this| {
                                    this.child(
                                        Icon::new(IconName::ArrowCircle)
                                            .color(Color::Accent)
                                            .size(IconSize::Small)
                                            .with_rotate_animation(2)
                                            .into_any_element(),
                                    )
                                },
                            ),
                    ),
            )
    }

    fn render_loading_spinner(&self, cx: &App) -> AnyElement {
        let rems = TextSize::Large.rems(cx);
        Icon::new(IconName::LoadCircle)
            .size(IconSize::Custom(rems))
            .color(Color::Accent)
            .with_rotate_animation(3)
            .into_any_element()
    }

    fn render_commit_detail_panel(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(selected_idx) = self.selected_entry_idx else {
            return Empty.into_any_element();
        };

        let Some(commit_entry) = self.graph_data.commits.get(selected_idx) else {
            return Empty.into_any_element();
        };

        let Some(repository) = self.get_repository(cx) else {
            return Empty.into_any_element();
        };

        let data = repository.update(cx, |repository, cx| {
            repository
                .fetch_commit_data(commit_entry.data.sha, cx)
                .clone()
        });

        let full_sha: SharedString = commit_entry.data.sha.to_string().into();
        let ref_names = self.visible_ref_names(&commit_entry.data.ref_names);

        let accent_colors = cx.theme().accents();
        let accent_color = accent_colors
            .0
            .get(commit_entry.color_idx)
            .copied()
            .unwrap_or_else(|| accent_colors.0.first().copied().unwrap_or_default());

        // todo(git graph): We should use the full commit message here
        let (author_name, author_email, commit_timestamp, commit_message) = match &data {
            CommitDataState::Loaded(data) => (
                data.author_name.clone(),
                data.author_email.clone(),
                Some(data.commit_timestamp),
                data.subject.clone(),
            ),
            CommitDataState::Loading => ("Loading…".into(), "".into(), None, "Loading…".into()),
        };

        let date_string = commit_timestamp
            .and_then(|ts| OffsetDateTime::from_unix_timestamp(ts).ok())
            .map(|datetime| {
                let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
                let local_datetime = datetime.to_offset(local_offset);
                let format =
                    time::format_description::parse("[month repr:short] [day], [year]").ok();
                format
                    .and_then(|f| local_datetime.format(&f).ok())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let remote = repository.update(cx, |repo, cx| self.get_remote(repo, window, cx));

        let avatar = {
            let author_email_for_avatar = if author_email.is_empty() {
                None
            } else {
                Some(author_email.clone())
            };

            CommitAvatar::new(&full_sha, author_email_for_avatar, remote.as_ref())
                .size(px(40.))
                .render(window, cx)
        };

        let changed_files_count = self
            .selected_commit_diff
            .as_ref()
            .map(|diff| diff.files.len())
            .unwrap_or(0);

        let (total_lines_added, total_lines_removed) =
            self.selected_commit_diff_stats.unwrap_or((0, 0));

        let sorted_file_entries: Rc<Vec<ChangedFileEntry>> = Rc::new(
            self.selected_commit_diff
                .as_ref()
                .map(|diff| {
                    let mut files: Vec<_> = diff.files.iter().collect();
                    files.sort_by_key(|file| file.status());
                    files
                        .into_iter()
                        .map(|file| ChangedFileEntry::from_commit_file(file, cx))
                        .collect()
                })
                .unwrap_or_default(),
        );

        v_flex()
            .min_w(px(300.))
            .h_full()
            .bg(cx.theme().colors().surface_background)
            .flex_basis(DefiniteLength::Fraction(
                self.commit_details_split_state.read(cx).right_ratio(),
            ))
            .child(
                v_flex()
                    .relative()
                    .w_full()
                    .p_2()
                    .gap_2()
                    .child(
                        div().absolute().top_2().right_2().child(
                            IconButton::new("close-detail", IconName::Close)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.selected_entry_idx = None;
                                    this.selected_commit_diff = None;
                                    this.selected_commit_diff_stats = None;
                                    this._commit_diff_task = None;
                                    cx.notify();
                                })),
                        ),
                    )
                    .child(
                        v_flex()
                            .py_1()
                            .w_full()
                            .items_center()
                            .gap_1()
                            .child(avatar)
                            .child(
                                v_flex()
                                    .items_center()
                                    .child(Label::new(author_name))
                                    .child(
                                        Label::new(date_string)
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    ),
                            ),
                    )
                    .children((!ref_names.is_empty()).then(|| {
                        h_flex().gap_1().flex_wrap().justify_center().children(
                            ref_names
                                .iter()
                                .map(|name| self.render_chip(name, accent_color)),
                        )
                    }))
                    .child(
                        v_flex()
                            .ml_neg_1()
                            .gap_1p5()
                            .when(!author_email.is_empty(), |this| {
                                let copied_state: Entity<CopiedState> = window.use_keyed_state(
                                    "author-email-copy",
                                    cx,
                                    CopiedState::new,
                                );
                                let is_copied = copied_state.read(cx).is_copied();

                                let (icon, icon_color, tooltip_label) = if is_copied {
                                    (IconName::Check, Color::Success, "Email Copied!")
                                } else {
                                    (IconName::Envelope, Color::Muted, "Copy Email")
                                };

                                let copy_email = author_email.clone();
                                let author_email_for_tooltip = author_email.clone();

                                this.child(
                                    Button::new("author-email-copy", author_email.clone())
                                        .start_icon(
                                            Icon::new(icon).size(IconSize::Small).color(icon_color),
                                        )
                                        .label_size(LabelSize::Small)
                                        .truncate(true)
                                        .color(Color::Muted)
                                        .tooltip(move |_, cx| {
                                            Tooltip::with_meta(
                                                tooltip_label,
                                                None,
                                                author_email_for_tooltip.clone(),
                                                cx,
                                            )
                                        })
                                        .on_click(move |_, _, cx| {
                                            copied_state.update(cx, |state, _cx| {
                                                state.mark_copied();
                                            });
                                            cx.write_to_clipboard(ClipboardItem::new_string(
                                                copy_email.to_string(),
                                            ));
                                            let state_id = copied_state.entity_id();
                                            cx.spawn(async move |cx| {
                                                cx.background_executor()
                                                    .timer(COPIED_STATE_DURATION)
                                                    .await;
                                                cx.update(|cx| {
                                                    cx.notify(state_id);
                                                })
                                            })
                                            .detach();
                                        }),
                                )
                            })
                            .child({
                                let copy_sha = full_sha.clone();
                                let copied_state: Entity<CopiedState> =
                                    window.use_keyed_state("sha-copy", cx, CopiedState::new);
                                let is_copied = copied_state.read(cx).is_copied();

                                let (icon, icon_color, tooltip_label) = if is_copied {
                                    (IconName::Check, Color::Success, "Commit SHA Copied!")
                                } else {
                                    (IconName::Hash, Color::Muted, "Copy Commit SHA")
                                };

                                Button::new("sha-button", &full_sha)
                                    .start_icon(
                                        Icon::new(icon).size(IconSize::Small).color(icon_color),
                                    )
                                    .label_size(LabelSize::Small)
                                    .truncate(true)
                                    .color(Color::Muted)
                                    .tooltip({
                                        let full_sha = full_sha.clone();
                                        move |_, cx| {
                                            Tooltip::with_meta(
                                                tooltip_label,
                                                None,
                                                full_sha.clone(),
                                                cx,
                                            )
                                        }
                                    })
                                    .on_click(move |_, _, cx| {
                                        copied_state.update(cx, |state, _cx| {
                                            state.mark_copied();
                                        });
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            copy_sha.to_string(),
                                        ));
                                        let state_id = copied_state.entity_id();
                                        cx.spawn(async move |cx| {
                                            cx.background_executor()
                                                .timer(COPIED_STATE_DURATION)
                                                .await;
                                            cx.update(|cx| {
                                                cx.notify(state_id);
                                            })
                                        })
                                        .detach();
                                    })
                            })
                            .when_some(remote.clone(), |this, remote| {
                                let provider_name = remote.host.name();
                                let icon = match provider_name.as_str() {
                                    "GitHub" => IconName::Github,
                                    _ => IconName::Link,
                                };
                                let parsed_remote = ParsedGitRemote {
                                    owner: remote.owner.as_ref().into(),
                                    repo: remote.repo.as_ref().into(),
                                };
                                let params = BuildCommitPermalinkParams {
                                    sha: full_sha.as_ref(),
                                };
                                let url = remote
                                    .host
                                    .build_commit_permalink(&parsed_remote, params)
                                    .to_string();

                                this.child(
                                    Button::new(
                                        "view-on-provider",
                                        format!("View on {}", provider_name),
                                    )
                                    .start_icon(
                                        Icon::new(icon).size(IconSize::Small).color(Color::Muted),
                                    )
                                    .label_size(LabelSize::Small)
                                    .truncate(true)
                                    .color(Color::Muted)
                                    .on_click(
                                        move |_, _, cx| {
                                            cx.open_url(&url);
                                        },
                                    ),
                                )
                            }),
                    ),
            )
            .child(Divider::horizontal())
            .child(div().p_2().child(Label::new(commit_message)))
            .child(Divider::horizontal())
            .child(
                v_flex()
                    .min_w_0()
                    .p_2()
                    .flex_1()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Label::new(format!("{} Changed Files", changed_files_count))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(DiffStat::new(
                                "commit-diff-stat",
                                total_lines_added,
                                total_lines_removed,
                            )),
                    )
                    .child(
                        div()
                            .id("changed-files-container")
                            .flex_1()
                            .min_h_0()
                            .child({
                                let entries = sorted_file_entries;
                                let entry_count = entries.len();
                                let commit_sha = full_sha.clone();
                                let repository = repository.downgrade();
                                let workspace = self.workspace.clone();
                                uniform_list(
                                    "changed-files-list",
                                    entry_count,
                                    move |range, _window, cx| {
                                        range
                                            .map(|ix| {
                                                entries[ix].render(
                                                    ix,
                                                    commit_sha.clone(),
                                                    repository.clone(),
                                                    workspace.clone(),
                                                    cx,
                                                )
                                            })
                                            .collect()
                                    },
                                )
                                .size_full()
                                .ml_neg_1()
                                .track_scroll(&self.changed_files_scroll_handle)
                            })
                            .vertical_scrollbar_for(&self.changed_files_scroll_handle, window, cx),
                    ),
            )
            .child(Divider::horizontal())
            .child(
                h_flex().p_1p5().w_full().child(
                    Button::new("view-commit", "View Commit")
                        .full_width()
                        .style(ButtonStyle::Outlined)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.open_selected_commit_view(window, cx);
                        })),
                ),
            )
            .into_any_element()
    }

    pub fn render_graph(&self, window: &Window, cx: &mut Context<GitGraph>) -> impl IntoElement {
        let row_height = self.row_height;
        let table_state = self.table_interaction_state.read(cx);
        let viewport_height = table_state
            .scroll_handle
            .0
            .borrow()
            .last_item_size
            .map(|size| size.item.height)
            .unwrap_or(px(600.0));
        let loaded_commit_count = self.graph_data.commits.len();

        let content_height = row_height * loaded_commit_count;
        let max_scroll = (content_height - viewport_height).max(px(0.));
        let scroll_offset_y = (-table_state.scroll_offset().y).clamp(px(0.), max_scroll);

        let first_visible_row = (scroll_offset_y / row_height).floor() as usize;
        let vertical_scroll_offset = scroll_offset_y - (first_visible_row as f32 * row_height);

        let graph_viewport_width = self.graph_viewport_width(window, cx);
        let graph_width = if self.graph_canvas_content_width() > graph_viewport_width {
            self.graph_canvas_content_width()
        } else {
            graph_viewport_width
        };
        let last_visible_row =
            first_visible_row + (viewport_height / row_height).ceil() as usize + 1;

        let viewport_range = first_visible_row.min(loaded_commit_count.saturating_sub(1))
            ..(last_visible_row).min(loaded_commit_count);
        let rows = self.graph_data.commits[viewport_range.clone()].to_vec();
        let commit_lines: Vec<_> = self
            .graph_data
            .lines
            .iter()
            .filter(|line| {
                line.full_interval.start <= viewport_range.end
                    && line.full_interval.end >= viewport_range.start
            })
            .cloned()
            .collect();

        let mut lines: BTreeMap<usize, Vec<_>> = BTreeMap::new();

        let hovered_entry_idx = self.hovered_entry_idx;
        let selected_entry_idx = self.selected_entry_idx;
        let is_focused = self.focus_handle.is_focused(window);
        let graph_canvas_bounds = self.graph_canvas_bounds.clone();

        gpui::canvas(
            move |_bounds, _window, _cx| {},
            move |bounds: Bounds<Pixels>, _: (), window: &mut Window, cx: &mut App| {
                graph_canvas_bounds.set(Some(bounds));

                window.paint_layer(bounds, |window| {
                    let accent_colors = cx.theme().accents();

                    let hover_bg = cx.theme().colors().element_hover.opacity(0.6);
                    let selected_bg = if is_focused {
                        cx.theme().colors().element_selected
                    } else {
                        cx.theme().colors().element_hover
                    };

                    for visible_row_idx in 0..rows.len() {
                        let absolute_row_idx = first_visible_row + visible_row_idx;
                        let is_hovered = hovered_entry_idx == Some(absolute_row_idx);
                        let is_selected = selected_entry_idx == Some(absolute_row_idx);

                        if is_hovered || is_selected {
                            let row_y = bounds.origin.y + visible_row_idx as f32 * row_height
                                - vertical_scroll_offset;

                            let row_bounds = Bounds::new(
                                point(bounds.origin.x, row_y),
                                gpui::Size {
                                    width: bounds.size.width,
                                    height: row_height,
                                },
                            );

                            let bg_color = if is_selected { selected_bg } else { hover_bg };
                            window.paint_quad(gpui::fill(row_bounds, bg_color));
                        }
                    }

                    for (row_idx, row) in rows.into_iter().enumerate() {
                        let row_color = accent_colors.color_for_index(row.color_idx as u32);
                        let row_y_center =
                            bounds.origin.y + row_idx as f32 * row_height + row_height / 2.0
                                - vertical_scroll_offset;

                        let commit_x = lane_center_x(bounds, row.lane as f32);

                        draw_commit_circle(commit_x, row_y_center, row_color, window);
                    }

                    for line in commit_lines {
                        let Some((start_segment_idx, start_column)) =
                            line.get_first_visible_segment_idx(first_visible_row)
                        else {
                            continue;
                        };

                        let line_x = lane_center_x(bounds, start_column as f32);

                        let start_row = line.full_interval.start as i32 - first_visible_row as i32;

                        let from_y =
                            bounds.origin.y + start_row as f32 * row_height + row_height / 2.0
                                - vertical_scroll_offset
                                + COMMIT_CIRCLE_RADIUS;

                        let mut current_row = from_y;
                        let mut current_column = line_x;

                        let mut builder = PathBuilder::stroke(LINE_WIDTH);
                        builder.move_to(point(line_x, from_y));

                        let segments = &line.segments[start_segment_idx..];
                        let desired_curve_height = row_height / 3.0;
                        let desired_curve_width = LANE_WIDTH / 3.0;

                        for (segment_idx, segment) in segments.iter().enumerate() {
                            let is_last = segment_idx + 1 == segments.len();

                            match segment {
                                CommitLineSegment::Straight { to_row } => {
                                    let mut dest_row = to_row_center(
                                        to_row - first_visible_row,
                                        row_height,
                                        vertical_scroll_offset,
                                        bounds,
                                    );
                                    if is_last {
                                        dest_row -= COMMIT_CIRCLE_RADIUS;
                                    }

                                    let dest_point = point(current_column, dest_row);

                                    current_row = dest_point.y;
                                    builder.line_to(dest_point);
                                    builder.move_to(dest_point);
                                }
                                CommitLineSegment::Curve {
                                    to_column,
                                    on_row,
                                    curve_kind,
                                } => {
                                    let mut to_column = lane_center_x(bounds, *to_column as f32);

                                    let mut to_row = to_row_center(
                                        *on_row - first_visible_row,
                                        row_height,
                                        vertical_scroll_offset,
                                        bounds,
                                    );

                                    // This means that this branch was a checkout
                                    let going_right = to_column > current_column;
                                    let column_shift = if going_right {
                                        COMMIT_CIRCLE_RADIUS + COMMIT_CIRCLE_STROKE_WIDTH
                                    } else {
                                        -COMMIT_CIRCLE_RADIUS - COMMIT_CIRCLE_STROKE_WIDTH
                                    };

                                    match curve_kind {
                                        CurveKind::Checkout => {
                                            if is_last {
                                                to_column -= column_shift;
                                            }

                                            let available_curve_width =
                                                (to_column - current_column).abs();
                                            let available_curve_height =
                                                (to_row - current_row).abs();
                                            let curve_width =
                                                desired_curve_width.min(available_curve_width);
                                            let curve_height =
                                                desired_curve_height.min(available_curve_height);
                                            let signed_curve_width = if going_right {
                                                curve_width
                                            } else {
                                                -curve_width
                                            };
                                            let curve_start =
                                                point(current_column, to_row - curve_height);
                                            let curve_end =
                                                point(current_column + signed_curve_width, to_row);
                                            let curve_control = point(current_column, to_row);

                                            builder.move_to(point(current_column, current_row));
                                            builder.line_to(curve_start);
                                            builder.move_to(curve_start);
                                            builder.curve_to(curve_end, curve_control);
                                            builder.move_to(curve_end);
                                            builder.line_to(point(to_column, to_row));
                                        }
                                        CurveKind::Merge => {
                                            if is_last {
                                                to_row -= COMMIT_CIRCLE_RADIUS;
                                            }

                                            let merge_start = point(
                                                current_column + column_shift,
                                                current_row - COMMIT_CIRCLE_RADIUS,
                                            );
                                            let available_curve_width =
                                                (to_column - merge_start.x).abs();
                                            let available_curve_height =
                                                (to_row - merge_start.y).abs();
                                            let curve_width =
                                                desired_curve_width.min(available_curve_width);
                                            let curve_height =
                                                desired_curve_height.min(available_curve_height);
                                            let signed_curve_width = if going_right {
                                                curve_width
                                            } else {
                                                -curve_width
                                            };
                                            let curve_start = point(
                                                to_column - signed_curve_width,
                                                merge_start.y,
                                            );
                                            let curve_end =
                                                point(to_column, merge_start.y + curve_height);
                                            let curve_control = point(to_column, merge_start.y);

                                            builder.move_to(merge_start);
                                            builder.line_to(curve_start);
                                            builder.move_to(curve_start);
                                            builder.curve_to(curve_end, curve_control);
                                            builder.move_to(curve_end);
                                            builder.line_to(point(to_column, to_row));
                                        }
                                    }
                                    current_row = to_row;
                                    current_column = to_column;
                                    builder.move_to(point(current_column, current_row));
                                }
                            }
                        }

                        builder.close();
                        lines.entry(line.color_idx).or_default().push(builder);
                    }

                    for (color_idx, builders) in lines {
                        let line_color = accent_colors.color_for_index(color_idx as u32);

                        for builder in builders {
                            if let Ok(path) = builder.build() {
                                // we paint each color on it's own layer to stop overlapping lines
                                // of different colors changing the color of a line
                                window.paint_layer(bounds, |window| {
                                    window.paint_path(path, line_color);
                                });
                            }
                        }
                    }
                })
            },
        )
        .w(graph_width)
        .h_full()
    }

    fn row_at_position(&self, position_y: Pixels, cx: &Context<Self>) -> Option<usize> {
        let canvas_bounds = self.graph_canvas_bounds.get()?;
        let table_state = self.table_interaction_state.read(cx);
        let scroll_offset_y = -table_state.scroll_offset().y;

        let local_y = position_y - canvas_bounds.origin.y;

        if local_y >= px(0.) && local_y < canvas_bounds.size.height {
            let row_in_viewport = (local_y / self.row_height).floor() as usize;
            let scroll_rows = (scroll_offset_y / self.row_height).floor() as usize;
            let absolute_row = scroll_rows + row_in_viewport;

            if absolute_row < self.graph_data.commits.len() {
                return Some(absolute_row);
            }
        }

        None
    }

    fn handle_graph_mouse_move(
        &mut self,
        event: &gpui::MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(row) = self.row_at_position(event.position.y, cx) {
            if self.hovered_entry_idx != Some(row) {
                self.hovered_entry_idx = Some(row);
                cx.notify();
            }
        } else if self.hovered_entry_idx.is_some() {
            self.hovered_entry_idx = None;
            cx.notify();
        }
    }

    fn handle_graph_click(
        &mut self,
        event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(row) = self.row_at_position(event.position().y, cx) {
            self.select_entry(row, ScrollStrategy::Nearest, cx);
            if event.click_count() >= 2 {
                self.open_commit_view(row, window, cx);
            }
        }
    }

    fn handle_graph_secondary_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.button != MouseButton::Right {
            return;
        }

        if let Some(row) = self.row_at_position(event.position.y, cx) {
            self.deploy_commit_context_menu(event.position, row, window, cx);
            cx.stop_propagation();
        }
    }

    fn handle_graph_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let line_height = window.line_height();
        let delta = event.delta.pixel_delta(line_height);

        let table_state = self.table_interaction_state.read(cx);
        let current_offset = table_state.scroll_offset();

        let viewport_height = table_state.scroll_handle.viewport().size.height;

        let commit_count = match self.graph_data.max_commit_count {
            AllCommitCount::Loaded(count) => count,
            AllCommitCount::NotLoaded => self.graph_data.commits.len(),
        };
        let content_height = self.row_height * commit_count;
        let max_vertical_scroll = (viewport_height - content_height).min(px(0.));

        let new_y = (current_offset.y + delta.y).clamp(max_vertical_scroll, px(0.));
        let new_offset = Point::new(current_offset.x, new_y);

        if new_offset != current_offset {
            table_state.set_scroll_offset(new_offset);
            cx.notify();
        }
    }

    fn render_commit_view_resize_handle(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .id("commit-view-split-resize-container")
            .relative()
            .h_full()
            .flex_shrink_0()
            .w(px(1.))
            .bg(cx.theme().colors().border_variant)
            .child(
                div()
                    .id("commit-view-split-resize-handle")
                    .absolute()
                    .left(px(-RESIZE_HANDLE_WIDTH / 2.0))
                    .w(px(RESIZE_HANDLE_WIDTH))
                    .h_full()
                    .cursor_col_resize()
                    .block_mouse_except_scroll()
                    .on_click(cx.listener(|this, event: &ClickEvent, _window, cx| {
                        if event.click_count() >= 2 {
                            this.commit_details_split_state.update(cx, |state, _| {
                                state.on_double_click();
                            });
                        }
                        cx.stop_propagation();
                    }))
                    .on_drag(DraggedSplitHandle, |_, _, _, cx| cx.new(|_| gpui::Empty)),
            )
            .into_any_element()
    }
}

struct CreateBranchAtCommitModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    title: SharedString,
    editor: Entity<Editor>,
    checkout_after_create: bool,
}

impl CreateBranchAtCommitModal {
    fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        initial_name: Option<String>,
        title: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if let Some(initial_name) = initial_name.clone() {
                editor.set_text(initial_name, window, cx);
            } else {
                editor.set_placeholder_text("Enter branch name…", window, cx);
            }
            editor
        });

        Self {
            graph,
            repository,
            commit_sha,
            title,
            editor,
            checkout_after_create: false,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let branch_name = self.editor.read(cx).text(cx).trim().replace(' ', "-");
        if branch_name.is_empty() {
            return;
        }

        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let commit_sha = self.commit_sha.to_string();
        let checkout_after_create = self.checkout_after_create;

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.create_branch_at(commit_sha, branch_name.clone())
                })
                .await??;

            if checkout_after_create {
                repository
                    .update(cx, |repository, _| repository.change_branch(branch_name))
                    .await??;
            }

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to create branch", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for CreateBranchAtCommitModal {}
impl ModalView for CreateBranchAtCommitModal {}
impl Focusable for CreateBranchAtCommitModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for CreateBranchAtCommitModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CreateBranchAtCommitModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(self.title.clone())),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(self.editor.clone())
                    .child(
                        Checkbox::new(
                            "create-branch-checkout-after-create",
                            if self.checkout_after_create {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Checkout after create")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(
                            |this: &mut CreateBranchAtCommitModal, _, _window, cx| {
                                this.checkout_after_create = !this.checkout_after_create;
                                cx.notify();
                            },
                        )),
                    ),
            )
    }
}

struct CherryPickModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    record_origin: bool,
    no_commit: bool,
    focus_handle: FocusHandle,
}

impl CherryPickModal {
    fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            repository,
            commit_sha,
            record_origin: false,
            no_commit: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let sha = self.commit_sha.to_string();
        let record_origin = self.record_origin;
        let no_commit = self.no_commit;

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.cherry_pick(sha, record_origin, no_commit)
                })
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err(
            "Failed to cherry-pick commit",
            window,
            cx,
            |error, _, _| Some(error.to_string()),
        );

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for CherryPickModal {}
impl ModalView for CherryPickModal {}
impl Focusable for CherryPickModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CherryPickModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CherryPickModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitCommit).size(IconSize::XSmall))
                    .child(Label::new(format!("Cherry Pick {}", self.commit_sha))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_2()
                    .gap_1()
                    .child(
                        Checkbox::new(
                            "cherry-pick-record-origin",
                            if self.record_origin {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Record origin (-x)")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.record_origin = !this.record_origin;
                            cx.notify();
                        })),
                    )
                    .child(
                        Checkbox::new(
                            "cherry-pick-no-commit",
                            if self.no_commit {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("No commit (--no-commit)")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.no_commit = !this.no_commit;
                            cx.notify();
                        })),
                    ),
            )
            .child(
                h_flex()
                    .px_3()
                    .pb_3()
                    .gap_2()
                    .justify_end()
                    .child(
                        Button::new("cherry-pick-cancel", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.cancel(&Cancel, window, cx);
                            })),
                    )
                    .child(
                        Button::new("cherry-pick-confirm", "Cherry Pick")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&Confirm, window, cx);
                            })),
                    ),
            )
    }
}

struct AddTagModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    name_editor: Entity<Editor>,
    message_editor: Entity<Editor>,
}

impl AddTagModal {
    fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Enter tag name…", window, cx);
            editor
        });
        let message_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Optional tag message…", window, cx);
            editor
        });

        Self {
            graph,
            repository,
            commit_sha,
            name_editor,
            message_editor,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let tag_name = self.name_editor.read(cx).text(cx).trim().to_string();
        if tag_name.is_empty() {
            return;
        }

        let tag_message = self.message_editor.read(cx).text(cx).trim().to_string();
        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let commit_sha = self.commit_sha.to_string();

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.create_tag(
                        commit_sha,
                        tag_name,
                        (!tag_message.is_empty()).then_some(tag_message),
                    )
                })
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to add tag", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for AddTagModal {}
impl ModalView for AddTagModal {}
impl Focusable for AddTagModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.name_editor.focus_handle(cx)
    }
}

impl Render for AddTagModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AddTagModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitCommit).size(IconSize::XSmall))
                    .child(Label::new(format!("Create Tag at {}", self.commit_sha))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(self.name_editor.clone())
                    .child(self.message_editor.clone()),
            )
    }
}

struct RenameBranchModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    branch_name: SharedString,
    editor: Entity<Editor>,
}

impl RenameBranchModal {
    fn new(
        branch_name: String,
        repository: Entity<Repository>,
        graph: WeakEntity<GitGraph>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(branch_name.clone(), window, cx);
            editor
        });
        Self {
            graph,
            repository,
            branch_name: branch_name.into(),
            editor,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let new_name = self.editor.read(cx).text(cx);
        if new_name.is_empty() || new_name == self.branch_name.as_ref() {
            cx.emit(DismissEvent);
            return;
        }

        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let old_name = self.branch_name.to_string();

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.rename_branch(old_name.clone(), new_name.clone())
                })
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to rename branch", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for RenameBranchModal {}
impl ModalView for RenameBranchModal {}
impl Focusable for RenameBranchModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for RenameBranchModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RenameBranchModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(format!("Rename Branch ({})", self.branch_name))),
            )
            .child(div().px_3().pb_3().w_full().child(self.editor.clone()))
    }
}

struct PushBranchModal {
    graph: WeakEntity<GitGraph>,
    state: PushBranchDialogState,
    focus_handle: FocusHandle,
}

impl PushBranchModal {
    fn new(
        graph: WeakEntity<GitGraph>,
        state: PushBranchDialogState,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            state,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let target = self.state.push_target();
        if let Some(graph) = self.graph.upgrade() {
            graph.update(cx, |graph, cx| {
                graph.perform_push_branch(target, window, cx);
            });
        }

        cx.emit(DismissEvent);
    }

    fn render_remote_dropdown(&self, window: &mut Window, cx: &mut Context<Self>) -> DropdownMenu {
        let weak = cx.weak_entity();
        let remotes = self.state.available_remotes.clone();
        let menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for remote_name in remotes.clone() {
                let weak = weak.clone();
                menu = menu.entry(remote_name.clone(), None, move |_window, cx| {
                    if let Some(entity) = weak.upgrade() {
                        entity.update(cx, |this, cx| {
                            this.state.select_remote(remote_name.clone());
                            cx.notify();
                        });
                    }
                });
            }
            menu
        });

        DropdownMenu::new(
            "push-branch-remote-dropdown",
            self.state.selected_remote.clone(),
            menu,
        )
        .style(DropdownStyle::Outlined)
        .full_width(true)
    }

    fn render_push_mode_option(
        &self,
        id: &'static str,
        label: &'static str,
        push_mode: PushMode,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Checkbox::new(
            id,
            if self.state.push_mode == push_mode {
                ToggleState::Selected
            } else {
                ToggleState::Unselected
            },
        )
        .label(label)
        .label_size(LabelSize::Small)
        .on_click(
            cx.listener(move |this: &mut PushBranchModal, _, _window, cx| {
                this.state.push_mode = push_mode;
                cx.notify();
            }),
        )
    }
}

impl EventEmitter<DismissEvent> for PushBranchModal {}
impl ModalView for PushBranchModal {}
impl Focusable for PushBranchModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PushBranchModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("PushBranchModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(36.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(format!(
                        "Push Branch ({})",
                        self.state.branch.name()
                    ))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_3()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Push to Remote(s):").size(LabelSize::Small))
                            .child(self.render_remote_dropdown(window, cx)),
                    )
                    .child(
                        Checkbox::new(
                            "push-branch-set-upstream",
                            if self.state.set_upstream {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Set Upstream")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(
                            |this: &mut PushBranchModal, _, _window, cx| {
                                this.state.set_upstream = !this.state.set_upstream;
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Push Mode:").size(LabelSize::Small))
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(self.render_push_mode_option(
                                        "push-branch-mode-normal",
                                        "Normal",
                                        PushMode::Normal,
                                        cx,
                                    ))
                                    .child(self.render_push_mode_option(
                                        "push-branch-mode-force-with-lease",
                                        "Force With Lease",
                                        PushMode::ForceWithLease,
                                        cx,
                                    ))
                                    .child(self.render_push_mode_option(
                                        "push-branch-mode-force",
                                        "Force",
                                        PushMode::Force,
                                        cx,
                                    )),
                            ),
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("push-branch-cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(
                                        |this: &mut PushBranchModal, _, window, cx| {
                                            this.cancel(&Cancel, window, cx);
                                        },
                                    )),
                            )
                            .child(
                                Button::new("push-branch-confirm", "Push")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(
                                        |this: &mut PushBranchModal, _, window, cx| {
                                            this.confirm(&Confirm, window, cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }
}

struct DeleteBranchModal {
    graph: WeakEntity<GitGraph>,
    branch_name: SharedString,
    is_remote: bool,
    force_delete: bool,
    focus_handle: FocusHandle,
}

impl DeleteBranchModal {
    fn new(
        graph: WeakEntity<GitGraph>,
        branch_name: String,
        is_remote: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            branch_name: branch_name.into(),
            is_remote,
            force_delete: false,
            focus_handle: _cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let graph = self.graph.clone();
        let branch_name = self.branch_name.to_string();
        let is_remote = self.is_remote;
        let force_delete = self.force_delete;
        if let Some(graph) = graph.upgrade() {
            graph.update(cx, |graph, cx| {
                graph.perform_delete_branch(branch_name, is_remote, force_delete, window, cx);
            });
        }

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for DeleteBranchModal {}
impl ModalView for DeleteBranchModal {}
impl Focusable for DeleteBranchModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DeleteBranchModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DeleteBranchModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::Trash).size(IconSize::XSmall))
                    .child(Label::new(if self.is_remote {
                        format!("Delete Remote-Tracking Branch ({})", self.branch_name)
                    } else {
                        format!("Delete Branch ({})", self.branch_name)
                    })),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(Label::new("This cannot be undone."))
                    .when(!self.is_remote, |this| {
                        this.child(
                            Checkbox::new(
                                "delete-branch-force-delete",
                                if self.force_delete {
                                    ToggleState::Selected
                                } else {
                                    ToggleState::Unselected
                                },
                            )
                            .label("Force delete")
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(
                                |this: &mut DeleteBranchModal, _, _window, cx| {
                                    this.force_delete = !this.force_delete;
                                    cx.notify();
                                },
                            )),
                        )
                    })
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("delete-branch-cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(
                                        |this: &mut DeleteBranchModal, _, window, cx| {
                                            this.cancel(&Cancel, window, cx);
                                        },
                                    )),
                            )
                            .child(
                                Button::new("delete-branch-confirm", "Delete")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(
                                        |this: &mut DeleteBranchModal, _, window, cx| {
                                            this.confirm(&Confirm, window, cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }
}

struct RevertCommitModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    no_commit: bool,
    focus_handle: FocusHandle,
}

impl RevertCommitModal {
    fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            repository,
            commit_sha,
            no_commit: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let sha = self.commit_sha.to_string();
        let no_commit = self.no_commit;

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| repository.revert_commit(sha, no_commit))
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to revert commit", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for RevertCommitModal {}
impl ModalView for RevertCommitModal {}
impl Focusable for RevertCommitModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RevertCommitModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RevertCommitModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitCommit).size(IconSize::XSmall))
                    .child(Label::new(format!("Revert Commit {}", self.commit_sha))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(
                        Checkbox::new(
                            "revert-commit-no-commit",
                            if self.no_commit {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Do not commit (--no-commit)")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(
                            |this: &mut RevertCommitModal, _, _window, cx| {
                                this.no_commit = !this.no_commit;
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("revert-commit-cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(
                                        |this: &mut RevertCommitModal, _, window, cx| {
                                            this.cancel(&Cancel, window, cx);
                                        },
                                    )),
                            )
                            .child(
                                Button::new("revert-commit-confirm", "Revert")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(
                                        |this: &mut RevertCommitModal, _, window, cx| {
                                            this.confirm(&Confirm, window, cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }
}

struct GitGraphAskPassModal {
    operation: SharedString,
    prompt: SharedString,
    editor: Entity<Editor>,
    tx: Option<oneshot::Sender<EncryptedPassword>>,
}

impl GitGraphAskPassModal {
    fn new(
        operation: SharedString,
        prompt: SharedString,
        tx: oneshot::Sender<EncryptedPassword>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if prompt.contains("yes/no") || prompt.contains("Username") {
                editor.set_masked(false, cx);
            } else {
                editor.set_masked(true, cx);
            }
            editor
        });

        Self {
            operation,
            prompt,
            editor,
            tx: Some(tx),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tx) = self.tx.take() {
            let mut text = self.editor.update(cx, |editor, cx| {
                let text = editor.text(cx);
                editor.clear(window, cx);
                text
            });
            if let Ok(password) = EncryptedPassword::try_from(text.as_ref()) {
                let _ = tx.send(password);
            }
            text.zeroize();
        }

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for GitGraphAskPassModal {}
impl ModalView for GitGraphAskPassModal {}
impl Focusable for GitGraphAskPassModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for GitGraphAskPassModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitGraphAskPassModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(self.operation.clone())),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(Label::new(self.prompt.clone()))
                    .child(self.editor.clone()),
            )
    }
}

impl PickerDelegate for BranchFilterPickerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Filter branches…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, cx: &mut App) -> Option<SharedString> {
        self.graph
            .as_ref()
            .and_then(|graph| graph.upgrade())
            .map(|graph| {
                graph.read_with(cx, |graph, _| {
                    if graph.branch_filter_state.is_loading {
                        SharedString::new_static("Loading branches…")
                    } else {
                        SharedString::new_static("No matching branches")
                    }
                })
            })
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = index;
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if let Some(graph) = self.graph.as_ref().and_then(|graph| graph.upgrade()) {
            let query_for_graph: SharedString = query.clone().into();
            graph.update(cx, |graph, _| {
                graph.branch_filter_state.query = query_for_graph;
            });
        }
        self.query = query;
        self.recompute_matches(cx);
        Task::ready(())
    }

    fn should_dismiss(&self) -> bool {
        true
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(BranchFilterEntry::Branch(ref_name)) =
            self.matches.get(self.selected_index).cloned()
        else {
            return;
        };

        if let Some(graph) = self.graph.as_ref().and_then(|graph| graph.upgrade()) {
            window.defer(cx, move |window, cx| {
                graph.update(cx, |graph, cx| {
                    graph.toggle_branch_selection(ref_name, window, cx);
                });
            });
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(graph) = self.graph.as_ref().and_then(|graph| graph.upgrade()) {
            let weak_graph = graph.downgrade();
            window.defer(cx, move |_window, cx| {
                if let Some(graph) = weak_graph.upgrade() {
                    graph.update(cx, |this, cx| {
                        this.branch_filter_state.handle.hide(cx);
                    });
                }
            });
        }
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::Start
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let editor = editor
            .as_any()
            .downcast_ref::<Entity<Editor>>()
            .expect("branch filter picker should render an editor");

        h_flex()
            .overflow_hidden()
            .flex_none()
            .h_9()
            .px_2p5()
            .child(editor.clone())
    }

    fn render_header(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let graph = self.graph.as_ref().and_then(|graph| graph.upgrade())?;
        let (is_loading, all_selection_state) = graph.read_with(cx, |graph, _| {
            (
                graph.branch_filter_state.is_loading,
                graph.all_branches_selection_state(),
            )
        });

        if is_loading {
            return None;
        }

        let weak_graph = self.graph.clone();
        Some(
            v_flex()
                .child(
                    ListItem::new("git-graph-branch-filter-all")
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .child(
                            Checkbox::new(
                                "git-graph-branch-filter-all-checkbox",
                                all_selection_state,
                            )
                            .label("Select All")
                            .label_size(LabelSize::Small)
                            .label_color(Color::Default)
                            .visualization_only(true),
                        )
                        .on_click(move |_, window, cx| {
                            if let Some(graph) =
                                weak_graph.as_ref().and_then(|graph| graph.upgrade())
                            {
                                graph.update(cx, |graph, cx| {
                                    graph.set_all_branch_selection(
                                        !graph.all_branches_selected(),
                                        cx,
                                    );
                                });
                            }
                            window.prevent_default();
                        }),
                )
                .child(Divider::horizontal())
                .into_any_element(),
        )
    }

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let BranchFilterEntry::Branch(ref_name) = self.matches.get(index)?.clone();
        let graph = self.graph.as_ref().and_then(|graph| graph.upgrade())?;
        let (label, is_selected) = graph.read_with(cx, |graph, _| {
            let branch = graph
                .branch_filter_state
                .available_branches
                .iter()
                .find(|branch| branch.ref_name == ref_name)?;
            Some((
                GitGraph::branch_display_name(branch.ref_name.as_ref()),
                branch.is_selected,
            ))
        })?;

        Some(
            ListItem::new(("git-graph-branch-filter-entry", index))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    Checkbox::new(
                        ("git-graph-branch-filter-checkbox", index),
                        ToggleState::from(is_selected),
                    )
                    .label(label)
                    .label_size(LabelSize::Small)
                    .label_color(Color::Default)
                    .visualization_only(true),
                ),
        )
    }
}

impl Render for GitGraph {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // This happens when we changed branches, we should refresh our search as well
        if let QueryState::Pending(query) = &mut self.search_state.state {
            let query = std::mem::take(query);
            self.search_state.state = QueryState::Empty;
            self.search(query, cx);
        }
        let (commit_count, is_loading) = match self.graph_data.max_commit_count {
            AllCommitCount::Loaded(count) => (count, true),
            AllCommitCount::NotLoaded => {
                let (commit_count, is_loading) = if let Some(repository) = self.get_repository(cx) {
                    repository.update(cx, |repository, cx| {
                        // Start loading the graph data if we haven't started already
                        let GraphDataResponse {
                            commits,
                            is_loading,
                            error: _,
                        } = repository.graph_data(
                            self.log_source.clone(),
                            self.log_order,
                            0..usize::MAX,
                            cx,
                        );
                        self.graph_data.add_commits(&commits);
                        (commits.len(), is_loading)
                    })
                } else {
                    (0, false)
                };

                (commit_count, is_loading)
            }
        };

        let content = if commit_count == 0 {
            let message = if is_loading {
                "Loading"
            } else {
                "No commits found"
            };
            let label = Label::new(message)
                .color(Color::Muted)
                .size(LabelSize::Large);
            div()
                .size_full()
                .h_flex()
                .gap_1()
                .items_center()
                .justify_center()
                .child(label)
                .when(is_loading, |this| {
                    this.child(self.render_loading_spinner(cx))
                })
        } else {
            let header_resize_info = HeaderResizeInfo::from_state(&self.column_widths, cx);
            let header_context = TableRenderContext::for_column_widths(
                Some(self.column_widths.read(cx).widths_to_render()),
                true,
            );
            let [
                graph_fraction,
                description_fraction,
                date_fraction,
                author_fraction,
                commit_fraction,
            ] = self.preview_column_fractions(window, cx);
            let table_fraction =
                description_fraction + date_fraction + author_fraction + commit_fraction;
            let table_width_config = self.table_column_width_config(window, cx);

            h_flex()
                .size_full()
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .size_full()
                        .flex()
                        .flex_col()
                        .child(render_table_header(
                            TableRow::from_vec(
                                vec![
                                    Label::new("Graph")
                                        .color(Color::Muted)
                                        .truncate()
                                        .into_any_element(),
                                    Label::new("Description")
                                        .color(Color::Muted)
                                        .into_any_element(),
                                    Label::new("Date").color(Color::Muted).into_any_element(),
                                    Label::new("Author").color(Color::Muted).into_any_element(),
                                    Label::new("Commit").color(Color::Muted).into_any_element(),
                                ],
                                5,
                            ),
                            header_context,
                            Some(header_resize_info),
                            Some(self.column_widths.entity_id()),
                            cx,
                        ))
                        .child({
                            let row_height = self.row_height;
                            let selected_entry_idx = self.selected_entry_idx;
                            let hovered_entry_idx = self.hovered_entry_idx;
                            let weak_self = cx.weak_entity();
                            let focus_handle = self.focus_handle.clone();

                            bind_redistributable_columns(
                                div()
                                    .relative()
                                    .flex_1()
                                    .w_full()
                                    .overflow_hidden()
                                    .child(
                                        h_flex()
                                            .size_full()
                                            .child(
                                                div()
                                                    .w(DefiniteLength::Fraction(graph_fraction))
                                                    .h_full()
                                                    .min_w_0()
                                                    .overflow_hidden()
                                                    .child(
                                                        div()
                                                            .id("graph-canvas")
                                                            .size_full()
                                                            .overflow_hidden()
                                                            .child(
                                                                div()
                                                                    .size_full()
                                                                    .child(self.render_graph(window, cx)),
                                                            )
                                                            .on_scroll_wheel(
                                                                cx.listener(Self::handle_graph_scroll),
                                                            )
                                                            .on_mouse_move(
                                                                cx.listener(Self::handle_graph_mouse_move),
                                                            )
                                                            .on_click(cx.listener(Self::handle_graph_click))
                                                            .on_mouse_down(
                                                                MouseButton::Right,
                                                                cx.listener(
                                                                    Self::handle_graph_secondary_mouse_down,
                                                                ),
                                                            )
                                                            .on_hover(cx.listener(
                                                                |this, &is_hovered: &bool, _, cx| {
                                                                    if !is_hovered
                                                                        && this.hovered_entry_idx.is_some()
                                                                    {
                                                                        this.hovered_entry_idx = None;
                                                                        cx.notify();
                                                                    }
                                                                },
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .w(DefiniteLength::Fraction(table_fraction))
                                                    .h_full()
                                                    .min_w_0()
                                                    .child(
                                                        Table::new(4)
                                                            .interactable(&self.table_interaction_state)
                                                            .hide_row_borders()
                                                            .hide_row_hover()
                                                            .width_config(table_width_config)
                                                            .map_row(move |(index, row), window, cx| {
                                                                let is_selected =
                                                                    selected_entry_idx == Some(index);
                                                                let is_hovered =
                                                                    hovered_entry_idx == Some(index);
                                                                let is_focused =
                                                                    focus_handle.is_focused(window);
                                                                let weak = weak_self.clone();
                                                                let weak_for_hover = weak.clone();
                                                                let weak_for_click = weak.clone();
                                                                let weak_for_context_menu = weak;

                                                                let hover_bg = cx
                                                                    .theme()
                                                                    .colors()
                                                                    .element_hover
                                                                    .opacity(0.6);
                                                                let selected_bg = if is_focused {
                                                                    cx.theme().colors().element_selected
                                                                } else {
                                                                    cx.theme().colors().element_hover
                                                                };

                                                                row.h(row_height)
                                                                    .when(is_selected, |row| row.bg(selected_bg))
                                                                    .when(
                                                                        is_hovered && !is_selected,
                                                                        |row| row.bg(hover_bg),
                                                                    )
                                                                    .on_hover(move |&is_hovered, _, cx| {
                                                                        weak_for_hover
                                                                            .update(cx, |this, cx| {
                                                                                if is_hovered {
                                                                                    if this.hovered_entry_idx
                                                                                        != Some(index)
                                                                                    {
                                                                                        this.hovered_entry_idx =
                                                                                            Some(index);
                                                                                        cx.notify();
                                                                                    }
                                                                                } else if this
                                                                                    .hovered_entry_idx
                                                                                    == Some(index)
                                                                                {
                                                                                    this.hovered_entry_idx =
                                                                                        None;
                                                                                    cx.notify();
                                                                                }
                                                                            })
                                                                            .ok();
                                                                    })
                                                                    .on_click(move |event, window, cx| {
                                                                        let click_count = event.click_count();
                                                                        weak_for_click.update(cx, |this, cx| {
                                                                            this.select_entry(
                                                                                index,
                                                                                ScrollStrategy::Center,
                                                                                cx,
                                                                            );
                                                                            if click_count >= 2 {
                                                                                this.open_commit_view(
                                                                                    index,
                                                                                    window,
                                                                                    cx,
                                                                                );
                                                                            }
                                                                        })
                                                                        .ok();
                                                                    })
                                                                    .on_mouse_down(
                                                                        MouseButton::Right,
                                                                        move |event: &MouseDownEvent, window, cx| {
                                                                            if event.button != MouseButton::Right {
                                                                                return;
                                                                            }

                                                                            let Some(this) = weak_for_context_menu.upgrade() else {
                                                                                return;
                                                                            };
                                                                            this.update(cx, |this, cx| {
                                                                                this.deploy_commit_context_menu(
                                                                                    event.position,
                                                                                    index,
                                                                                    window,
                                                                                    cx,
                                                                                );
                                                                            });
                                                                            cx.stop_propagation();
                                                                        },
                                                                    )
                                                                    .into_any_element()
                                                            })
                                                            .uniform_list(
                                                                "git-graph-commits",
                                                                commit_count,
                                                                cx.processor(Self::render_table_rows),
                                                            ),
                                                    ),
                                            ),
                                    )
                                    .child(render_redistributable_columns_resize_handles(
                                        &self.column_widths,
                                        window,
                                        cx,
                                    )),
                                self.column_widths.clone(),
                            )
                        }),
                )
                .on_drag_move::<DraggedSplitHandle>(cx.listener(|this, event, window, cx| {
                    this.commit_details_split_state.update(cx, |state, cx| {
                        state.on_drag_move(event, window, cx);
                    });
                }))
                .on_drop::<DraggedSplitHandle>(cx.listener(|this, _event, _window, cx| {
                    this.commit_details_split_state.update(cx, |state, _cx| {
                        state.commit_ratio();
                    });
                }))
                .when(self.selected_entry_idx.is_some(), |this| {
                    this.child(self.render_commit_view_resize_handle(window, cx))
                        .child(self.render_commit_detail_panel(window, cx))
                })
        };

        div()
            .key_context("GitGraph")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(|this, _: &OpenCommitView, window, cx| {
                this.open_selected_commit_view(window, cx);
            }))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|this, _: &FocusSearch, window, cx| {
                this.search_state
                    .editor
                    .update(cx, |editor, cx| editor.focus_handle(cx).focus(window, cx));
            }))
            .on_action(cx.listener(|this, _: &ToggleBranchFilter, window, cx| {
                this.open_branch_filter(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleSettingsDropdown, window, cx| {
                this.settings_dropdown_state.handle.toggle(window, cx);
            }))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(|this, _: &SelectNextMatch, _window, cx| {
                this.select_next_match(cx);
            }))
            .on_action(cx.listener(|this, _: &SelectPreviousMatch, _window, cx| {
                this.select_previous_match(cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleCaseSensitive, _window, cx| {
                this.search_state.case_sensitive = !this.search_state.case_sensitive;
                this.search_state.state.next_state();
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleShowStashes, window, cx| {
                this.update_graph_settings(
                    |settings| settings.show_stashes = !settings.show_stashes,
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &ToggleShowTags, window, cx| {
                this.update_graph_settings(
                    |settings| settings.show_tags = !settings.show_tags,
                    window,
                    cx,
                );
            }))
            .on_action(
                cx.listener(|this, _: &ToggleShowRemoteBranches, window, cx| {
                    this.update_graph_settings(
                        |settings| settings.show_remote_branches = !settings.show_remote_branches,
                        window,
                        cx,
                    );
                }),
            )
            .on_action(cx.listener(|this, _: &ToggleReflogCommits, window, cx| {
                this.update_graph_settings(
                    |settings| {
                        settings.include_reflog_commits = !settings.include_reflog_commits;
                    },
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &ToggleFirstParentOnly, window, cx| {
                this.update_graph_settings(
                    |settings| settings.first_parent_only = !settings.first_parent_only,
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &AddTag, window, cx| {
                this.show_add_tag_modal(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CreateBranchAtCommit, window, cx| {
                this.show_create_branch_modal(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CheckoutCommit, window, cx| {
                this.checkout_selected_commit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CherryPickCommit, window, cx| {
                this.cherry_pick_selected_commit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &RevertCommit, window, cx| {
                this.revert_selected_commit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &DropCommit, window, cx| {
                this.drop_selected_commit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &MergeCommit, window, cx| {
                this.merge_selected_commit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &RebaseOntoCommit, window, cx| {
                this.rebase_selected_commit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ResetCommit, window, cx| {
                this.reset_selected_commit(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CopyCommitHash, _window, cx| {
                this.copy_selected_commit_hash(cx);
            }))
            .on_action(cx.listener(|this, _: &CopyCommitSubject, _window, cx| {
                this.copy_selected_commit_subject(cx);
            }))
            .child(
                v_flex()
                    .size_full()
                    .child(self.render_search_bar(window, cx))
                    .child(div().flex_1().child(content)),
            )
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
            .on_action(cx.listener(|_, _: &buffer_search::Deploy, window, cx| {
                window.dispatch_action(Box::new(FocusSearch), cx);
                cx.stop_propagation();
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

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitGraph))
    }

    fn tab_tooltip_content(&self, cx: &App) -> Option<TabTooltipContent> {
        let repo_name = self.get_repository(cx).and_then(|repo| {
            repo.read(cx)
                .work_directory_abs_path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        });

        Some(TabTooltipContent::Custom(Box::new(Tooltip::element({
            move |_, _| {
                v_flex()
                    .child(Label::new("Git Graph"))
                    .when_some(repo_name.clone(), |this, name| {
                        this.child(Label::new(name).color(Color::Muted).size(LabelSize::Small))
                    })
                    .into_any_element()
            }
        }))))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.get_repository(cx)
            .and_then(|repo| {
                repo.read(cx)
                    .work_directory_abs_path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
            })
            .map_or_else(|| "Git Graph".into(), |name| SharedString::from(name))
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }
}

impl workspace::SerializableItem for GitGraph {
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
            &persistence::GitGraphsDb::global(cx),
            cx,
        )
    }

    fn deserialize(
        project: Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        let db = persistence::GitGraphsDb::global(cx);
        let Some(repo_work_path) = db.get_git_graph(item_id, workspace_id).ok().flatten() else {
            return Task::ready(Err(anyhow::anyhow!("No git graph to deserialize")));
        };

        let window_handle = window.window_handle();
        let project = project.read(cx);
        let git_store = project.git_store().clone();
        let wait = project.wait_for_initial_scan(cx);

        cx.spawn(async move |cx| {
            wait.await;

            cx.update_window(window_handle, |_, window, cx| {
                let path = repo_work_path.as_path();

                let repositories = git_store.read(cx).repositories();
                let repo_id = repositories.iter().find_map(|(&repo_id, repo)| {
                    if repo.read(cx).snapshot().work_directory_abs_path.as_ref() == path {
                        Some(repo_id)
                    } else {
                        None
                    }
                });

                let Some(repo_id) = repo_id else {
                    return Err(anyhow::anyhow!("Repository not found for path: {:?}", path));
                };

                Ok(cx.new(|cx| GitGraph::new(repo_id, git_store, workspace, window, cx)))
            })?
        })
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
        let repo = self.get_repository(cx)?;
        let repo_working_path = repo
            .read(cx)
            .snapshot()
            .work_directory_abs_path
            .to_string_lossy()
            .to_string();

        let db = persistence::GitGraphsDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_git_graph(item_id, workspace_id, repo_working_path)
                .await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        event == &ItemEvent::UpdateTab
    }
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::WorkspaceDb;

    pub struct GitGraphsDb(ThreadSafeConnection);

    impl Domain for GitGraphsDb {
        const NAME: &str = stringify!(GitGraphsDb);

        const MIGRATIONS: &[&str] = &[
            sql!(
                CREATE TABLE git_graphs (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,
                    is_open INTEGER DEFAULT FALSE,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
            ),
            sql!(
                ALTER TABLE git_graphs ADD COLUMN repo_working_path TEXT;
            ),
        ];
    }

    db::static_connection!(GitGraphsDb, [WorkspaceDb]);

    impl GitGraphsDb {
        query! {
            pub async fn save_git_graph(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId,
                repo_working_path: String
            ) -> Result<()> {
                INSERT OR REPLACE INTO git_graphs(item_id, workspace_id, repo_working_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_git_graph(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<Option<PathBuf>> {
                SELECT repo_working_path
                FROM git_graphs
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result, bail};
    use collections::{HashMap, HashSet};
    use fs::FakeFs;
    use git::Oid;
    use git::repository::InitialGraphCommitData;
    use gpui::TestAppContext;
    use project::Project;
    use project::git_store::{GitStoreEvent, RepositoryEvent};
    use rand::prelude::*;
    use serde_json::json;
    use settings::SettingsStore;
    use smallvec::{SmallVec, smallvec};
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
    }

    fn branch_ref_set(branches: &[&str]) -> HashSet<SharedString> {
        branches
            .iter()
            .map(|branch| SharedString::from((*branch).to_string()))
            .collect()
    }

    fn branch_ref_vec(branches: &[&str]) -> Vec<SharedString> {
        branches
            .iter()
            .map(|branch| SharedString::from((*branch).to_string()))
            .collect()
    }

    fn branch_infos(branches: &[&str]) -> Vec<BranchInfo> {
        branches
            .iter()
            .map(|branch| BranchInfo {
                ref_name: SharedString::from((*branch).to_string()),
                is_head: false,
                is_remote: branch.starts_with("refs/remotes/"),
                is_selected: false,
            })
            .collect()
    }

    fn test_branch(branch_name: &str, upstream: Option<git::repository::Upstream>) -> Branch {
        Branch {
            is_head: false,
            ref_name: SharedString::from(format!("refs/heads/{branch_name}")),
            upstream,
            most_recent_commit: None,
        }
    }

    fn visible_ref_names_for_test(
        settings: GraphSettings,
        available_branches: &[BranchInfo],
        ref_names: &[&str],
    ) -> Vec<SharedString> {
        ref_names
            .iter()
            .filter(|ref_name| {
                let ref_name = **ref_name;
                GitGraph::ref_name_matches_remote_visibility(
                    settings.show_remote_branches,
                    available_branches,
                    ref_name,
                ) && (settings.show_tags
                    || !(ref_name.starts_with("tag: ") || ref_name.starts_with("refs/tags/")))
                    && (settings.show_stashes
                        || !(ref_name == "refs/stash"
                            || ref_name == "stash"
                            || ref_name.starts_with("stash@{")
                            || ref_name.contains("refs/stash")))
            })
            .map(|ref_name| SharedString::from((*ref_name).to_string()))
            .collect()
    }

    async fn setup_git_graph_with_branches(
        branches: &[&str],
        cx: &mut TestAppContext,
    ) -> (
        Arc<FakeFs>,
        Entity<Project>,
        Entity<Repository>,
        Entity<GitGraph>,
        gpui::AnyWindowHandle,
    ) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            Path::new("/project"),
            json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;

        if !branches.is_empty() {
            fs.insert_branches(Path::new("/project/.git"), branches);
        }

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        cx.run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project
                .active_repository(cx)
                .expect("should have a repository")
        });

        let (multi_workspace, window_cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });
        let workspace_weak =
            multi_workspace.read_with(&*window_cx, |multi, _| multi.workspace().downgrade());
        let window_handle = window_cx.window_handle();
        let git_graph = window_cx.new_window_entity(|window, cx| {
            GitGraph::new(
                repository.read(cx).id,
                project.read(cx).git_store().clone(),
                workspace_weak,
                window,
                cx,
            )
        });
        window_cx.run_until_parked();

        (fs, project, repository, git_graph, window_handle)
    }

    #[test]
    fn test_reconcile_branch_selection_initial_load_selects_all_eligible_refs() {
        let previous_selection = HashSet::default();
        let eligible_ref_names = branch_ref_set(&["refs/heads/main", "refs/heads/feature"]);

        let reconciled = GitGraph::reconcile_branch_selection(
            false,
            false,
            &previous_selection,
            &eligible_ref_names,
        );

        assert_eq!(reconciled, eligible_ref_names);
    }

    #[test]
    fn test_reconcile_branch_selection_extends_to_new_remote_refs_when_all_were_selected() {
        let previous_selection = branch_ref_set(&["refs/heads/main", "refs/heads/feature"]);
        let eligible_ref_names = branch_ref_set(&[
            "refs/heads/main",
            "refs/heads/feature",
            "refs/remotes/origin/main",
            "refs/remotes/origin/feature",
        ]);

        let reconciled = GitGraph::reconcile_branch_selection(
            true,
            true,
            &previous_selection,
            &eligible_ref_names,
        );

        assert_eq!(reconciled, eligible_ref_names);
    }

    #[test]
    fn test_reconcile_branch_selection_preserves_subset_when_not_all_were_selected() {
        let previous_selection = branch_ref_set(&["refs/heads/main"]);
        let eligible_ref_names = branch_ref_set(&[
            "refs/heads/main",
            "refs/heads/feature",
            "refs/remotes/origin/main",
        ]);

        let reconciled = GitGraph::reconcile_branch_selection(
            true,
            false,
            &previous_selection,
            &eligible_ref_names,
        );

        assert_eq!(reconciled, branch_ref_set(&["refs/heads/main"]));
    }

    #[test]
    fn test_reconcile_branch_selection_drops_remote_refs_when_they_become_ineligible() {
        let previous_selection = branch_ref_set(&[
            "refs/heads/main",
            "refs/remotes/origin/main",
            "refs/remotes/origin/feature",
        ]);
        let eligible_ref_names = branch_ref_set(&["refs/heads/main", "refs/heads/feature"]);

        let reconciled = GitGraph::reconcile_branch_selection(
            true,
            false,
            &previous_selection,
            &eligible_ref_names,
        );

        assert_eq!(reconciled, branch_ref_set(&["refs/heads/main"]));
    }

    #[test]
    fn test_reconcile_branch_selection_auto_selects_new_branch_when_all_eligible_were_selected() {
        let previous_selection = branch_ref_set(&["refs/heads/main", "refs/heads/feature"]);
        let eligible_ref_names =
            branch_ref_set(&["refs/heads/main", "refs/heads/feature", "refs/heads/bugfix"]);

        let reconciled = GitGraph::reconcile_branch_selection(
            true,
            true,
            &previous_selection,
            &eligible_ref_names,
        );

        assert_eq!(reconciled, eligible_ref_names);
    }

    #[test]
    fn test_branch_filter_source_uses_explicit_local_refs_when_remote_branches_are_hidden() {
        let available_branches = branch_infos(&[
            "refs/heads/main",
            "refs/heads/feature",
            "refs/remotes/origin/main",
            "refs/remotes/origin/feature",
        ]);
        let selected_branches = branch_ref_set(&["refs/heads/main", "refs/heads/feature"]);

        let source = GitGraph::branch_filter_source_for_state(
            true,
            false,
            &available_branches,
            &selected_branches,
        );

        assert_eq!(
            source,
            LogSource::Branches(branch_ref_vec(&["refs/heads/feature", "refs/heads/main"]))
        );
    }

    #[test]
    fn test_branch_filter_source_uses_all_only_when_all_visible_remote_refs_are_selected() {
        let available_branches = branch_infos(&[
            "refs/heads/main",
            "refs/heads/feature",
            "refs/remotes/origin/main",
            "refs/remotes/origin/feature",
        ]);
        let selected_branches = branch_ref_set(&[
            "refs/heads/main",
            "refs/heads/feature",
            "refs/remotes/origin/main",
            "refs/remotes/origin/feature",
        ]);

        let source = GitGraph::branch_filter_source_for_state(
            true,
            true,
            &available_branches,
            &selected_branches,
        );

        assert_eq!(source, LogSource::All);
    }

    #[test]
    fn test_branch_filter_source_returns_empty_branch_list_when_no_refs_are_selected() {
        let available_branches = branch_infos(&["refs/heads/main", "refs/remotes/origin/main"]);
        let selected_branches = HashSet::default();

        let source = GitGraph::branch_filter_source_for_state(
            true,
            false,
            &available_branches,
            &selected_branches,
        );

        assert_eq!(source, LogSource::Branches(Vec::new()));
    }

    #[test]
    fn test_ref_name_matches_remote_visibility_hides_remote_refs_when_setting_is_disabled() {
        let available_branches = branch_infos(&["refs/heads/main", "refs/remotes/origin/main"]);

        assert!(!GitGraph::ref_name_matches_remote_visibility(
            false,
            &available_branches,
            "refs/remotes/origin/main"
        ));
        assert!(!GitGraph::ref_name_matches_remote_visibility(
            false,
            &available_branches,
            "HEAD -> refs/remotes/origin/main"
        ));
        assert!(!GitGraph::ref_name_matches_remote_visibility(
            false,
            &available_branches,
            "origin/main"
        ));
        assert!(GitGraph::ref_name_matches_remote_visibility(
            false,
            &available_branches,
            "refs/heads/main"
        ));
        assert!(GitGraph::ref_name_matches_remote_visibility(
            true,
            &available_branches,
            "refs/remotes/origin/main"
        ));
    }

    #[test]
    fn test_visible_ref_names_drops_only_remote_refs_when_remote_setting_is_disabled() {
        let mut settings = GraphSettings::default();
        settings.show_remote_branches = false;
        let available_branches = branch_infos(&["refs/heads/main", "refs/remotes/origin/main"]);

        let visible_ref_names = visible_ref_names_for_test(
            settings,
            &available_branches,
            &["main", "origin/main", "tag: v1.0.0", "stash@{0}"],
        );

        assert_eq!(
            visible_ref_names,
            vec![
                SharedString::from("main".to_string()),
                SharedString::from("tag: v1.0.0".to_string()),
                SharedString::from("stash@{0}".to_string()),
            ]
        );
    }

    #[test]
    fn test_push_branch_dialog_state_defaults_to_tracked_upstream_remote() {
        let branch = test_branch(
            "feature",
            Some(git::repository::Upstream {
                ref_name: "refs/remotes/origin/feature".into(),
                tracking: git::repository::UpstreamTracking::Tracked(
                    git::repository::UpstreamTrackingStatus {
                        ahead: 1,
                        behind: 0,
                    },
                ),
            }),
        );

        let state = PushBranchDialogState::new(
            branch,
            vec![SharedString::from("origin"), SharedString::from("upstream")],
        )
        .unwrap();

        assert_eq!(state.selected_remote, SharedString::from("origin"));
        assert!(!state.set_upstream);
        assert_eq!(state.push_mode, PushMode::Normal);
    }

    #[test]
    fn test_push_branch_dialog_state_defaults_to_set_upstream_for_unpublished_branch() {
        let branch = test_branch("feature", None);

        let state = PushBranchDialogState::new(
            branch,
            vec![SharedString::from("upstream"), SharedString::from("origin")],
        )
        .unwrap();

        assert_eq!(state.selected_remote, SharedString::from("upstream"));
        assert!(state.set_upstream);
        assert_eq!(state.push_mode, PushMode::Normal);
    }

    #[test]
    fn test_push_branch_dialog_state_switching_remote_enables_set_upstream() {
        let branch = test_branch(
            "feature",
            Some(git::repository::Upstream {
                ref_name: "refs/remotes/origin/feature".into(),
                tracking: git::repository::UpstreamTracking::Tracked(
                    git::repository::UpstreamTrackingStatus {
                        ahead: 0,
                        behind: 0,
                    },
                ),
            }),
        );

        let mut state = PushBranchDialogState::new(
            branch,
            vec![SharedString::from("origin"), SharedString::from("upstream")],
        )
        .unwrap();
        state.set_upstream = false;
        state.select_remote(SharedString::from("upstream"));

        assert_eq!(state.selected_remote, SharedString::from("upstream"));
        assert!(state.set_upstream);
    }

    #[test]
    fn test_push_branch_dialog_state_builds_push_target_with_selected_mode() {
        let branch = test_branch(
            "feature",
            Some(git::repository::Upstream {
                ref_name: "refs/remotes/origin/review/feature".into(),
                tracking: git::repository::UpstreamTracking::Tracked(
                    git::repository::UpstreamTrackingStatus {
                        ahead: 2,
                        behind: 0,
                    },
                ),
            }),
        );

        let mut state =
            PushBranchDialogState::new(branch.clone(), vec![SharedString::from("origin")]).unwrap();
        state.set_upstream = true;
        state.push_mode = PushMode::Force;

        let target = state.push_target();

        assert_eq!(target.branch, branch);
        assert_eq!(target.remote.name, SharedString::from("origin"));
        assert_eq!(
            target.remote_branch_name,
            SharedString::from("review/feature")
        );
        assert_eq!(
            target.options,
            Some(PushOptions {
                set_upstream: true,
                push_mode: PushMode::Force,
            })
        );
    }

    #[gpui::test]
    async fn test_show_remote_branches_updates_branch_filter_selection_and_log_source_immediately(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (_fs, _project, _repository, git_graph, window_handle) = setup_git_graph_with_branches(
            &["main", "feature", "origin/main", "origin/feature"],
            cx,
        )
        .await;

        let mut window_cx = gpui::VisualTestContext::from_window(window_handle, cx);
        window_cx.update_window_entity(&git_graph, |graph, window, cx| {
            graph.load_available_branches(None, window, cx);
        });
        window_cx.run_until_parked();

        git_graph.read_with(cx, |graph, cx| {
            assert!(graph.settings_dropdown_state.settings.show_remote_branches);
            assert_eq!(graph.eligible_branch_infos().len(), 4);
            assert_eq!(
                graph.branch_filter_state.selected_branches,
                branch_ref_set(&[
                    "refs/heads/main",
                    "refs/heads/feature",
                    "refs/remotes/origin/main",
                    "refs/remotes/origin/feature",
                ])
            );
            assert_eq!(graph.branch_filter_source(), LogSource::All);
            assert_eq!(
                graph.branch_filter_picker.read(cx).delegate.match_count(),
                4
            );
        });

        window_cx.update_window_entity(&git_graph, |graph, window, cx| {
            graph.update_graph_settings(
                |settings| settings.show_remote_branches = false,
                window,
                cx,
            );
        });

        git_graph.read_with(cx, |graph, cx| {
            assert!(!graph.settings_dropdown_state.settings.show_remote_branches);
            assert_eq!(graph.eligible_branch_infos().len(), 2);
            assert_eq!(
                graph.branch_filter_state.selected_branches,
                branch_ref_set(&["refs/heads/main", "refs/heads/feature"])
            );
            assert_eq!(
                graph.branch_filter_source(),
                LogSource::Branches(branch_ref_vec(&["refs/heads/feature", "refs/heads/main"]))
            );
            assert_eq!(
                graph.branch_filter_picker.read(cx).delegate.match_count(),
                2
            );
            let visible_ref_names = graph.visible_ref_names(&[
                SharedString::from("main".to_string()),
                SharedString::from("origin/main".to_string()),
                SharedString::from("tag: v1.0.0".to_string()),
            ]);
            assert!(
                visible_ref_names
                    .iter()
                    .all(|name| name.as_ref() != "origin/main")
            );
            assert_eq!(
                visible_ref_names,
                vec![
                    SharedString::from("main".to_string()),
                    SharedString::from("tag: v1.0.0".to_string()),
                ]
            );
        });

        window_cx.update_window_entity(&git_graph, |graph, window, cx| {
            graph.update_graph_settings(
                |settings| settings.show_remote_branches = true,
                window,
                cx,
            );
        });

        git_graph.read_with(cx, |graph, cx| {
            assert!(graph.settings_dropdown_state.settings.show_remote_branches);
            assert_eq!(graph.eligible_branch_infos().len(), 4);
            assert_eq!(
                graph.branch_filter_state.selected_branches,
                branch_ref_set(&[
                    "refs/heads/main",
                    "refs/heads/feature",
                    "refs/remotes/origin/main",
                    "refs/remotes/origin/feature",
                ])
            );
            assert_eq!(graph.branch_filter_source(), LogSource::All);
            assert_eq!(
                graph.branch_filter_picker.read(cx).delegate.match_count(),
                4
            );
        });
    }

    #[gpui::test]
    async fn test_branch_filter_picker_delegate_exposes_more_than_two_hundred_branches(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (_fs, _project, _repository, git_graph, _window_handle) =
            setup_git_graph_with_branches(&["main"], cx).await;

        git_graph.update(cx, |graph, cx| {
            graph.branch_filter_state.available_branches = (0..250)
                .map(|index| {
                    let ref_name = SharedString::from(format!("refs/heads/branch-{index:03}"));
                    BranchInfo {
                        ref_name,
                        is_head: index == 0,
                        is_remote: false,
                        is_selected: true,
                    }
                })
                .collect();
            graph.branch_filter_state.selected_branches = graph
                .branch_filter_state
                .available_branches
                .iter()
                .map(|branch| branch.ref_name.clone())
                .collect();
            graph.branch_filter_state.branches_loaded = true;
            graph.refresh_branch_filter_picker(cx);
        });

        let match_count = git_graph.read_with(cx, |graph, cx| {
            graph.branch_filter_picker.read(cx).delegate.match_count()
        });
        assert_eq!(match_count, 250);
    }

    /// Generates a random commit DAG suitable for testing git graph rendering.
    ///
    /// The commits are ordered newest-first (like git log output), so:
    /// - Index 0 = most recent commit (HEAD)
    /// - Last index = oldest commit (root, has no parents)
    /// - Parents of commit at index I must have index > I
    ///
    /// When `adversarial` is true, generates complex topologies with many branches
    /// and octopus merges. Otherwise generates more realistic linear histories
    /// with occasional branches.
    fn generate_random_commit_dag(
        rng: &mut StdRng,
        num_commits: usize,
        adversarial: bool,
    ) -> Vec<Arc<InitialGraphCommitData>> {
        if num_commits == 0 {
            return Vec::new();
        }

        let mut commits: Vec<Arc<InitialGraphCommitData>> = Vec::with_capacity(num_commits);
        let oids: Vec<Oid> = (0..num_commits).map(|_| Oid::random(rng)).collect();

        for i in 0..num_commits {
            let sha = oids[i];

            let parents = if i == num_commits - 1 {
                smallvec![]
            } else {
                generate_parents_from_oids(rng, &oids, i, num_commits, adversarial)
            };

            let ref_names = if i == 0 {
                vec!["HEAD".into(), "main".into()]
            } else if adversarial && rng.random_bool(0.1) {
                vec![format!("branch-{}", i).into()]
            } else {
                Vec::new()
            };

            commits.push(Arc::new(InitialGraphCommitData {
                sha,
                parents,
                ref_names,
            }));
        }

        commits
    }

    fn generate_parents_from_oids(
        rng: &mut StdRng,
        oids: &[Oid],
        current_idx: usize,
        num_commits: usize,
        adversarial: bool,
    ) -> SmallVec<[Oid; 1]> {
        let remaining = num_commits - current_idx - 1;
        if remaining == 0 {
            return smallvec![];
        }

        if adversarial {
            let merge_chance = 0.4;
            let octopus_chance = 0.15;

            if remaining >= 3 && rng.random_bool(octopus_chance) {
                let num_parents = rng.random_range(3..=remaining.min(5));
                let mut parent_indices: Vec<usize> = (current_idx + 1..num_commits).collect();
                parent_indices.shuffle(rng);
                parent_indices
                    .into_iter()
                    .take(num_parents)
                    .map(|idx| oids[idx])
                    .collect()
            } else if remaining >= 2 && rng.random_bool(merge_chance) {
                let mut parent_indices: Vec<usize> = (current_idx + 1..num_commits).collect();
                parent_indices.shuffle(rng);
                parent_indices
                    .into_iter()
                    .take(2)
                    .map(|idx| oids[idx])
                    .collect()
            } else {
                let parent_idx = rng.random_range(current_idx + 1..num_commits);
                smallvec![oids[parent_idx]]
            }
        } else {
            let merge_chance = 0.15;
            let skip_chance = 0.1;

            if remaining >= 2 && rng.random_bool(merge_chance) {
                let first_parent = current_idx + 1;
                let second_parent = rng.random_range(current_idx + 2..num_commits);
                smallvec![oids[first_parent], oids[second_parent]]
            } else if rng.random_bool(skip_chance) && remaining >= 2 {
                let skip = rng.random_range(1..remaining.min(3));
                smallvec![oids[current_idx + 1 + skip]]
            } else {
                smallvec![oids[current_idx + 1]]
            }
        }
    }

    fn build_oid_to_row_map(graph: &GraphData) -> HashMap<Oid, usize> {
        graph
            .commits
            .iter()
            .enumerate()
            .map(|(idx, entry)| (entry.data.sha, idx))
            .collect()
    }

    fn verify_commit_order(
        graph: &GraphData,
        commits: &[Arc<InitialGraphCommitData>],
    ) -> Result<()> {
        if graph.commits.len() != commits.len() {
            bail!(
                "Commit count mismatch: graph has {} commits, expected {}",
                graph.commits.len(),
                commits.len()
            );
        }

        for (idx, (graph_commit, expected_commit)) in
            graph.commits.iter().zip(commits.iter()).enumerate()
        {
            if graph_commit.data.sha != expected_commit.sha {
                bail!(
                    "Commit order mismatch at index {}: graph has {:?}, expected {:?}",
                    idx,
                    graph_commit.data.sha,
                    expected_commit.sha
                );
            }
        }

        Ok(())
    }

    fn verify_line_endpoints(graph: &GraphData, oid_to_row: &HashMap<Oid, usize>) -> Result<()> {
        for line in &graph.lines {
            let child_row = *oid_to_row
                .get(&line.child)
                .context("Line references non-existent child commit")?;

            let parent_row = *oid_to_row
                .get(&line.parent)
                .context("Line references non-existent parent commit")?;

            if child_row >= parent_row {
                bail!(
                    "child_row ({}) must be < parent_row ({})",
                    child_row,
                    parent_row
                );
            }

            if line.full_interval.start != child_row {
                bail!(
                    "full_interval.start ({}) != child_row ({})",
                    line.full_interval.start,
                    child_row
                );
            }

            if line.full_interval.end != parent_row {
                bail!(
                    "full_interval.end ({}) != parent_row ({})",
                    line.full_interval.end,
                    parent_row
                );
            }

            if let Some(last_segment) = line.segments.last() {
                let segment_end_row = match last_segment {
                    CommitLineSegment::Straight { to_row } => *to_row,
                    CommitLineSegment::Curve { on_row, .. } => *on_row,
                };

                if segment_end_row != line.full_interval.end {
                    bail!(
                        "last segment ends at row {} but full_interval.end is {}",
                        segment_end_row,
                        line.full_interval.end
                    );
                }
            }
        }

        Ok(())
    }

    fn verify_column_correctness(
        graph: &GraphData,
        oid_to_row: &HashMap<Oid, usize>,
    ) -> Result<()> {
        for line in &graph.lines {
            let child_row = *oid_to_row
                .get(&line.child)
                .context("Line references non-existent child commit")?;

            let parent_row = *oid_to_row
                .get(&line.parent)
                .context("Line references non-existent parent commit")?;

            let child_lane = graph.commits[child_row].lane;
            if line.child_column != child_lane {
                bail!(
                    "child_column ({}) != child's lane ({})",
                    line.child_column,
                    child_lane
                );
            }

            let mut current_column = line.child_column;
            for segment in &line.segments {
                if let CommitLineSegment::Curve { to_column, .. } = segment {
                    current_column = *to_column;
                }
            }

            let parent_lane = graph.commits[parent_row].lane;
            if current_column != parent_lane {
                bail!(
                    "ending column ({}) != parent's lane ({})",
                    current_column,
                    parent_lane
                );
            }
        }

        Ok(())
    }

    fn verify_segment_continuity(graph: &GraphData) -> Result<()> {
        for line in &graph.lines {
            if line.segments.is_empty() {
                bail!("Line has no segments");
            }

            let mut current_row = line.full_interval.start;

            for (idx, segment) in line.segments.iter().enumerate() {
                let segment_end_row = match segment {
                    CommitLineSegment::Straight { to_row } => *to_row,
                    CommitLineSegment::Curve { on_row, .. } => *on_row,
                };

                if segment_end_row < current_row {
                    bail!(
                        "segment {} ends at row {} which is before current row {}",
                        idx,
                        segment_end_row,
                        current_row
                    );
                }

                current_row = segment_end_row;
            }
        }

        Ok(())
    }

    fn verify_line_overlaps(graph: &GraphData) -> Result<()> {
        for line in &graph.lines {
            let child_row = line.full_interval.start;

            let mut current_column = line.child_column;
            let mut current_row = child_row;

            for segment in &line.segments {
                match segment {
                    CommitLineSegment::Straight { to_row } => {
                        for row in (current_row + 1)..*to_row {
                            if row < graph.commits.len() {
                                let commit_at_row = &graph.commits[row];
                                if commit_at_row.lane == current_column {
                                    bail!(
                                        "straight segment from row {} to {} in column {} passes through commit {:?} at row {}",
                                        current_row,
                                        to_row,
                                        current_column,
                                        commit_at_row.data.sha,
                                        row
                                    );
                                }
                            }
                        }
                        current_row = *to_row;
                    }
                    CommitLineSegment::Curve {
                        to_column, on_row, ..
                    } => {
                        current_column = *to_column;
                        current_row = *on_row;
                    }
                }
            }
        }

        Ok(())
    }

    fn verify_coverage(graph: &GraphData) -> Result<()> {
        let mut expected_edges: HashSet<(Oid, Oid)> = HashSet::default();
        for entry in &graph.commits {
            for parent in &entry.data.parents {
                expected_edges.insert((entry.data.sha, *parent));
            }
        }

        let mut found_edges: HashSet<(Oid, Oid)> = HashSet::default();
        for line in &graph.lines {
            let edge = (line.child, line.parent);

            if !found_edges.insert(edge) {
                bail!(
                    "Duplicate line found for edge {:?} -> {:?}",
                    line.child,
                    line.parent
                );
            }

            if !expected_edges.contains(&edge) {
                bail!(
                    "Orphan line found: {:?} -> {:?} is not in the commit graph",
                    line.child,
                    line.parent
                );
            }
        }

        for (child, parent) in &expected_edges {
            if !found_edges.contains(&(*child, *parent)) {
                bail!("Missing line for edge {:?} -> {:?}", child, parent);
            }
        }

        assert_eq!(
            expected_edges.symmetric_difference(&found_edges).count(),
            0,
            "The symmetric difference should be zero"
        );

        Ok(())
    }

    fn verify_merge_line_optimality(
        graph: &GraphData,
        oid_to_row: &HashMap<Oid, usize>,
    ) -> Result<()> {
        for line in &graph.lines {
            let first_segment = line.segments.first();
            let is_merge_line = matches!(
                first_segment,
                Some(CommitLineSegment::Curve {
                    curve_kind: CurveKind::Merge,
                    ..
                })
            );

            if !is_merge_line {
                continue;
            }

            let child_row = *oid_to_row
                .get(&line.child)
                .context("Line references non-existent child commit")?;

            let parent_row = *oid_to_row
                .get(&line.parent)
                .context("Line references non-existent parent commit")?;

            let parent_lane = graph.commits[parent_row].lane;

            let Some(CommitLineSegment::Curve { to_column, .. }) = first_segment else {
                continue;
            };

            let curves_directly_to_parent = *to_column == parent_lane;

            if !curves_directly_to_parent {
                continue;
            }

            let curve_row = child_row + 1;
            let has_commits_in_path = graph.commits[curve_row..parent_row]
                .iter()
                .any(|c| c.lane == parent_lane);

            if has_commits_in_path {
                bail!(
                    "Merge line from {:?} to {:?} curves directly to parent lane {} but there are commits in that lane between rows {} and {}",
                    line.child,
                    line.parent,
                    parent_lane,
                    curve_row,
                    parent_row
                );
            }

            let curve_ends_at_parent = curve_row == parent_row;

            if curve_ends_at_parent {
                if line.segments.len() != 1 {
                    bail!(
                        "Merge line from {:?} to {:?} curves directly to parent (curve_row == parent_row), but has {} segments instead of 1 [MergeCurve]",
                        line.child,
                        line.parent,
                        line.segments.len()
                    );
                }
            } else {
                if line.segments.len() != 2 {
                    bail!(
                        "Merge line from {:?} to {:?} curves directly to parent lane without overlap, but has {} segments instead of 2 [MergeCurve, Straight]",
                        line.child,
                        line.parent,
                        line.segments.len()
                    );
                }

                let is_straight_segment = matches!(
                    line.segments.get(1),
                    Some(CommitLineSegment::Straight { .. })
                );

                if !is_straight_segment {
                    bail!(
                        "Merge line from {:?} to {:?} curves directly to parent lane without overlap, but second segment is not a Straight segment",
                        line.child,
                        line.parent
                    );
                }
            }
        }

        Ok(())
    }

    fn verify_all_invariants(
        graph: &GraphData,
        commits: &[Arc<InitialGraphCommitData>],
    ) -> Result<()> {
        let oid_to_row = build_oid_to_row_map(graph);

        verify_commit_order(graph, commits).context("commit order")?;
        verify_line_endpoints(graph, &oid_to_row).context("line endpoints")?;
        verify_column_correctness(graph, &oid_to_row).context("column correctness")?;
        verify_segment_continuity(graph).context("segment continuity")?;
        verify_merge_line_optimality(graph, &oid_to_row).context("merge line optimality")?;
        verify_coverage(graph).context("coverage")?;
        verify_line_overlaps(graph).context("line overlaps")?;
        Ok(())
    }

    #[test]
    fn test_git_graph_merge_commits() {
        let mut rng = StdRng::seed_from_u64(42);

        let oid1 = Oid::random(&mut rng);
        let oid2 = Oid::random(&mut rng);
        let oid3 = Oid::random(&mut rng);
        let oid4 = Oid::random(&mut rng);

        let commits = vec![
            Arc::new(InitialGraphCommitData {
                sha: oid1,
                parents: smallvec![oid2, oid3],
                ref_names: vec!["HEAD".into()],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid2,
                parents: smallvec![oid4],
                ref_names: vec![],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid3,
                parents: smallvec![oid4],
                ref_names: vec![],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid4,
                parents: smallvec![],
                ref_names: vec![],
            }),
        ];

        let mut graph_data = GraphData::new(8);
        graph_data.add_commits(&commits);

        if let Err(error) = verify_all_invariants(&graph_data, &commits) {
            panic!("Graph invariant violation for merge commits:\n{}", error);
        }
    }

    #[test]
    fn test_git_graph_linear_commits() {
        let mut rng = StdRng::seed_from_u64(42);

        let oid1 = Oid::random(&mut rng);
        let oid2 = Oid::random(&mut rng);
        let oid3 = Oid::random(&mut rng);

        let commits = vec![
            Arc::new(InitialGraphCommitData {
                sha: oid1,
                parents: smallvec![oid2],
                ref_names: vec!["HEAD".into()],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid2,
                parents: smallvec![oid3],
                ref_names: vec![],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid3,
                parents: smallvec![],
                ref_names: vec![],
            }),
        ];

        let mut graph_data = GraphData::new(8);
        graph_data.add_commits(&commits);

        if let Err(error) = verify_all_invariants(&graph_data, &commits) {
            panic!("Graph invariant violation for linear commits:\n{}", error);
        }
    }

    #[test]
    fn test_git_graph_random_commits() {
        for seed in 0..100 {
            let mut rng = StdRng::seed_from_u64(seed);

            let adversarial = rng.random_bool(0.2);
            let num_commits = if adversarial {
                rng.random_range(10..100)
            } else {
                rng.random_range(5..50)
            };

            let commits = generate_random_commit_dag(&mut rng, num_commits, adversarial);

            assert_eq!(
                num_commits,
                commits.len(),
                "seed={}: Generate random commit dag didn't generate the correct amount of commits",
                seed
            );

            let mut graph_data = GraphData::new(8);
            graph_data.add_commits(&commits);

            if let Err(error) = verify_all_invariants(&graph_data, &commits) {
                panic!(
                    "Graph invariant violation (seed={}, adversarial={}, num_commits={}):\n{:#}",
                    seed, adversarial, num_commits, error
                );
            }
        }
    }

    // The full integration test has less iterations because it's significantly slower
    // than the random commit test
    #[gpui::test(iterations = 10)]
    async fn test_git_graph_random_integration(mut rng: StdRng, cx: &mut TestAppContext) {
        init_test(cx);

        let adversarial = rng.random_bool(0.2);
        let num_commits = if adversarial {
            rng.random_range(10..100)
        } else {
            rng.random_range(5..50)
        };

        let commits = generate_random_commit_dag(&mut rng, num_commits, adversarial);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            Path::new("/project"),
            json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;

        fs.set_graph_commits(Path::new("/project/.git"), commits.clone());

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        cx.run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project
                .active_repository(cx)
                .expect("should have a repository")
        });

        repository.update(cx, |repo, cx| {
            repo.graph_data(
                crate::LogSource::default(),
                crate::LogOrder::default(),
                0..usize::MAX,
                cx,
            );
        });
        cx.run_until_parked();

        let graph_commits: Vec<Arc<InitialGraphCommitData>> = repository.update(cx, |repo, cx| {
            repo.graph_data(
                crate::LogSource::default(),
                crate::LogOrder::default(),
                0..usize::MAX,
                cx,
            )
            .commits
            .to_vec()
        });

        let mut graph_data = GraphData::new(8);
        graph_data.add_commits(&graph_commits);

        if let Err(error) = verify_all_invariants(&graph_data, &commits) {
            panic!(
                "Graph invariant violation (adversarial={}, num_commits={}):\n{:#}",
                adversarial, num_commits, error
            );
        }
    }

    #[gpui::test]
    async fn test_initial_graph_data_not_cleared_on_initial_loading(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            Path::new("/project"),
            json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;

        let mut rng = StdRng::seed_from_u64(42);
        let commits = generate_random_commit_dag(&mut rng, 10, false);
        fs.set_graph_commits(Path::new("/project/.git"), commits.clone());

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        let observed_repository_events = Arc::new(Mutex::new(Vec::new()));
        project.update(cx, |project, cx| {
            let observed_repository_events = observed_repository_events.clone();
            cx.subscribe(project.git_store(), move |_, _, event, _| {
                if let GitStoreEvent::RepositoryUpdated(_, repository_event, true) = event {
                    observed_repository_events
                        .lock()
                        .expect("repository event mutex should be available")
                        .push(repository_event.clone());
                }
            })
            .detach();
        });

        let repository = project.read_with(cx, |project, cx| {
            project
                .active_repository(cx)
                .expect("should have a repository")
        });

        repository.update(cx, |repo, cx| {
            repo.graph_data(
                crate::LogSource::default(),
                crate::LogOrder::default(),
                0..usize::MAX,
                cx,
            );
        });

        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        cx.run_until_parked();

        let observed_repository_events = observed_repository_events
            .lock()
            .expect("repository event mutex should be available");
        assert!(
            observed_repository_events
                .iter()
                .any(|event| matches!(event, RepositoryEvent::BranchChanged)),
            "initial repository scan should emit BranchChanged"
        );
        let commit_count_after = repository.read_with(cx, |repo, _| {
            repo.get_graph_data(crate::LogSource::default(), crate::LogOrder::default())
                .map(|data| data.commit_data.len())
                .unwrap()
        });
        assert_eq!(
            commits.len(),
            commit_count_after,
            "initial_graph_data should remain populated after events emitted by initial repository scan"
        );
    }

    #[gpui::test]
    async fn test_graph_data_repopulated_from_cache_after_repo_switch(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            Path::new("/project_a"),
            json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;
        fs.insert_tree(
            Path::new("/project_b"),
            json!({
                ".git": {},
                "other.txt": "content",
            }),
        )
        .await;

        let mut rng = StdRng::seed_from_u64(42);
        let commits = generate_random_commit_dag(&mut rng, 10, false);
        fs.set_graph_commits(Path::new("/project_a/.git"), commits.clone());

        let project = Project::test(
            fs.clone(),
            [Path::new("/project_a"), Path::new("/project_b")],
            cx,
        )
        .await;
        cx.run_until_parked();

        let (first_repository, second_repository) = project.read_with(cx, |project, cx| {
            let mut first_repository = None;
            let mut second_repository = None;

            for repository in project.repositories(cx).values() {
                let work_directory_abs_path = &repository.read(cx).work_directory_abs_path;
                if work_directory_abs_path.as_ref() == Path::new("/project_a") {
                    first_repository = Some(repository.clone());
                } else if work_directory_abs_path.as_ref() == Path::new("/project_b") {
                    second_repository = Some(repository.clone());
                }
            }

            (
                first_repository.expect("should have repository for /project_a"),
                second_repository.expect("should have repository for /project_b"),
            )
        });
        first_repository.update(cx, |repository, cx| repository.set_as_active_repository(cx));
        cx.run_until_parked();

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });

        let workspace_weak =
            multi_workspace.read_with(&*cx, |multi, _| multi.workspace().downgrade());
        let git_graph = cx.new_window_entity(|window, cx| {
            GitGraph::new(
                first_repository.read(cx).id,
                project.read(cx).git_store().clone(),
                workspace_weak,
                window,
                cx,
            )
        });
        cx.run_until_parked();

        // Verify initial graph data is loaded
        let initial_commit_count =
            git_graph.read_with(&*cx, |graph, _| graph.graph_data.commits.len());
        assert!(
            initial_commit_count > 0,
            "graph data should have been loaded, got 0 commits"
        );

        git_graph.update(cx, |graph, cx| {
            graph.set_repo_id(second_repository.read(cx).id, cx)
        });
        cx.run_until_parked();

        let commit_count_after_clear =
            git_graph.read_with(&*cx, |graph, _| graph.graph_data.commits.len());
        assert_eq!(
            commit_count_after_clear, 0,
            "graph_data should be cleared after switching away"
        );

        git_graph.update(cx, |graph, cx| {
            graph.set_repo_id(first_repository.read(cx).id, cx)
        });
        cx.run_until_parked();

        cx.draw(
            point(px(0.), px(0.)),
            gpui::size(px(1200.), px(800.)),
            |_, _| git_graph.clone().into_any_element(),
        );
        cx.run_until_parked();

        let commit_count_after_switch_back =
            git_graph.read_with(&*cx, |graph, _| graph.graph_data.commits.len());
        assert_eq!(
            initial_commit_count, commit_count_after_switch_back,
            "graph_data should be repopulated from cache after switching back to the same repo"
        );
    }
}
