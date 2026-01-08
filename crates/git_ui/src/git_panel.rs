use crate::askpass_modal::AskPassModal;
use crate::commit_modal::CommitModal;
use crate::commit_tooltip::CommitTooltip;
use crate::commit_view::CommitView;
use crate::project_diff::{self, Diff, ProjectDiff};
use crate::remote_output::{self, RemoteAction, SuccessMessage};
use crate::{branch_picker, picker_prompt, render_remote_button};
use crate::{
    file_history_view::FileHistoryView, git_panel_settings::GitPanelSettings, git_status_icon,
    repository_selector::RepositorySelector,
};
use agent_settings::AgentSettings;
use anyhow::Context as _;
use askpass::AskPassDelegate;
use cloud_llm_client::CompletionIntent;
use collections::{BTreeMap, HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::RewrapOptions;
use editor::{
    Direction, Editor, EditorElement, EditorMode, MultiBuffer, MultiBufferOffset,
    actions::ExpandAllDiffHunks,
};
use futures::StreamExt as _;
use git::commit::ParsedCommitMessage;
use git::repository::{
    Branch, CommitDetails, CommitOptions, CommitSummary, DiffType, FetchOptions, GitCommitter,
    PushOptions, Remote, RemoteCommandOutput, ResetMode, Upstream, UpstreamTracking,
    UpstreamTrackingStatus, get_git_committer,
};
use git::stash::GitStash;
use git::status::StageStatus;
use git::{Amend, Signoff, ToggleStaged, repository::RepoPath, status::FileStatus};
use git::{
    ExpandCommitEditor, GitHostingProviderRegistry, RestoreTrackedFiles, StageAll, StashAll,
    StashApply, StashPop, TrashUntrackedFiles, UnstageAll,
};
use gpui::{
    Action, AsyncApp, AsyncWindowContext, Bounds, ClickEvent, Corner, DismissEvent, Entity,
    EventEmitter, FocusHandle, Focusable, KeyContext, MouseButton, MouseDownEvent, Point,
    PromptLevel, ScrollStrategy, Subscription, Task, UniformListScrollHandle, WeakEntity, actions,
    anchored, deferred, point, size, uniform_list,
};
use itertools::Itertools;
use language::{Buffer, File};
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    Role, ZED_CLOUD_PROVIDER_ID,
};
use menu;
use multi_buffer::ExcerptInfo;
use notifications::status_toast::{StatusToast, ToastIcon};
use panel::{
    PanelHeader, panel_button, panel_editor_container, panel_editor_style, panel_filled_button,
    panel_icon_button,
};
use project::{
    Fs, Project, ProjectPath,
    git_store::{GitStoreEvent, Repository, RepositoryEvent, RepositoryId, pending_op},
    project_settings::{GitPathStyle, ProjectSettings},
};
use prompt_store::{BuiltInPrompt, PromptId, PromptStore, RULES_FILE_NAMES};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore, StatusStyle};
use std::future::Future;
use std::ops::Range;
use std::path::Path;
use std::{sync::Arc, time::Duration, usize};
use strum::{IntoEnumIterator, VariantNames};
use time::OffsetDateTime;
use ui::{
    ButtonLike, Checkbox, CommonAnimationExt, ContextMenu, ElevationIndex, IndentGuideColors,
    PopoverMenu, RenderedIndentGuide, ScrollAxes, Scrollbars, SplitButton, Tooltip, WithScrollbar,
    prelude::*,
};
use util::paths::PathStyle;
use util::{ResultExt, TryFutureExt, maybe, rel_path::RelPath};
use workspace::SERIALIZATION_THROTTLE_TIME;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{DetachAndPromptErr, ErrorMessagePrompt, NotificationId, NotifyResultExt},
};
use ztracing::instrument;
actions!(
    git_panel,
    [
        /// Closes the git panel.
        Close,
        /// Toggles focus on the git panel.
        ToggleFocus,
        /// Opens the git panel menu.
        OpenMenu,
        /// Focuses on the commit message editor.
        FocusEditor,
        /// Focuses on the changes list.
        FocusChanges,
        /// Select next git panel menu item, and show it in the diff view
        NextEntry,
        /// Select previous git panel menu item, and show it in the diff view
        PreviousEntry,
        /// Select first git panel menu item, and show it in the diff view
        FirstEntry,
        /// Select last git panel menu item, and show it in the diff view
        LastEntry,
        /// Toggles automatic co-author suggestions.
        ToggleFillCoAuthors,
        /// Toggles sorting entries by path vs status.
        ToggleSortByPath,
        /// Toggles showing entries in tree vs flat view.
        ToggleTreeView,
        /// Expands the selected entry to show its children.
        ExpandSelectedEntry,
        /// Collapses the selected entry to hide its children.
        CollapseSelectedEntry,
    ]
);

fn prompt<T>(
    msg: &str,
    detail: Option<&str>,
    window: &mut Window,
    cx: &mut App,
) -> Task<anyhow::Result<T>>
where
    T: IntoEnumIterator + VariantNames + 'static,
{
    let rx = window.prompt(PromptLevel::Info, msg, detail, T::VARIANTS, cx);
    cx.spawn(async move |_| Ok(T::iter().nth(rx.await?).unwrap()))
}

#[derive(strum::EnumIter, strum::VariantNames)]
#[strum(serialize_all = "title_case")]
enum TrashCancel {
    Trash,
    Cancel,
}

struct GitMenuState {
    has_tracked_changes: bool,
    has_staged_changes: bool,
    has_unstaged_changes: bool,
    has_new_changes: bool,
    sort_by_path: bool,
    has_stash_items: bool,
    tree_view: bool,
}

fn git_panel_context_menu(
    focus_handle: FocusHandle,
    state: GitMenuState,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ContextMenu> {
    ContextMenu::build(window, cx, move |context_menu, _, _| {
        context_menu
            .context(focus_handle)
            .action_disabled_when(
                !state.has_unstaged_changes,
                "Stage All",
                StageAll.boxed_clone(),
            )
            .action_disabled_when(
                !state.has_staged_changes,
                "Unstage All",
                UnstageAll.boxed_clone(),
            )
            .separator()
            .action_disabled_when(
                !(state.has_new_changes || state.has_tracked_changes),
                "Stash All",
                StashAll.boxed_clone(),
            )
            .action_disabled_when(!state.has_stash_items, "Stash Pop", StashPop.boxed_clone())
            .action("View Stash", zed_actions::git::ViewStash.boxed_clone())
            .separator()
            .action("Open Diff", project_diff::Diff.boxed_clone())
            .separator()
            .action_disabled_when(
                !state.has_tracked_changes,
                "Discard Tracked Changes",
                RestoreTrackedFiles.boxed_clone(),
            )
            .action_disabled_when(
                !state.has_new_changes,
                "Trash Untracked Files",
                TrashUntrackedFiles.boxed_clone(),
            )
            .separator()
            .entry(
                if state.tree_view {
                    "Flat View"
                } else {
                    "Tree View"
                },
                Some(Box::new(ToggleTreeView)),
                move |window, cx| window.dispatch_action(Box::new(ToggleTreeView), cx),
            )
            .when(!state.tree_view, |this| {
                this.entry(
                    if state.sort_by_path {
                        "Sort by Status"
                    } else {
                        "Sort by Path"
                    },
                    Some(Box::new(ToggleSortByPath)),
                    move |window, cx| window.dispatch_action(Box::new(ToggleSortByPath), cx),
                )
            })
    })
}

const GIT_PANEL_KEY: &str = "GitPanel";

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);
// TODO: We should revise this part. It seems the indentation width is not aligned with the one in project panel
const TREE_INDENT: f32 = 16.0;

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<GitPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &ExpandCommitEditor, window, cx| {
        CommitModal::toggle(workspace, None, window, cx)
    });
}

#[derive(Debug, Clone)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedGitPanel {
    width: Option<Pixels>,
    #[serde(default)]
    amend_pending: bool,
    #[serde(default)]
    signoff_enabled: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Section {
    Conflict,
    Tracked,
    New,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GitHeaderEntry {
    header: Section,
}

impl GitHeaderEntry {
    pub fn contains(&self, status_entry: &GitStatusEntry, repo: &Repository) -> bool {
        let this = &self.header;
        let status = status_entry.status;
        match this {
            Section::Conflict => {
                repo.had_conflict_on_last_merge_head_change(&status_entry.repo_path)
            }
            Section::Tracked => !status.is_created(),
            Section::New => status.is_created(),
        }
    }
    pub fn title(&self) -> &'static str {
        match self.header {
            Section::Conflict => "Conflicts",
            Section::Tracked => "Tracked",
            Section::New => "Untracked",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum GitListEntry {
    Status(GitStatusEntry),
    TreeStatus(GitTreeStatusEntry),
    Directory(GitTreeDirEntry),
    Header(GitHeaderEntry),
}

impl GitListEntry {
    fn status_entry(&self) -> Option<&GitStatusEntry> {
        match self {
            GitListEntry::Status(entry) => Some(entry),
            GitListEntry::TreeStatus(entry) => Some(&entry.entry),
            _ => None,
        }
    }

    fn directory_entry(&self) -> Option<&GitTreeDirEntry> {
        match self {
            GitListEntry::Directory(entry) => Some(entry),
            _ => None,
        }
    }
}

enum GitPanelViewMode {
    Flat,
    Tree(TreeViewState),
}

impl GitPanelViewMode {
    fn from_settings(cx: &App) -> Self {
        if GitPanelSettings::get_global(cx).tree_view {
            GitPanelViewMode::Tree(TreeViewState::default())
        } else {
            GitPanelViewMode::Flat
        }
    }

    fn tree_state(&self) -> Option<&TreeViewState> {
        match self {
            GitPanelViewMode::Tree(state) => Some(state),
            GitPanelViewMode::Flat => None,
        }
    }

    fn tree_state_mut(&mut self) -> Option<&mut TreeViewState> {
        match self {
            GitPanelViewMode::Tree(state) => Some(state),
            GitPanelViewMode::Flat => None,
        }
    }
}

#[derive(Default)]
struct TreeViewState {
    // Maps visible index to actual entry index.
    // Length equals the number of visible entries.
    // This is needed because some entries (like collapsed directories) may be hidden.
    logical_indices: Vec<usize>,
    expanded_dirs: HashMap<TreeKey, bool>,
    directory_descendants: HashMap<TreeKey, Vec<GitStatusEntry>>,
}

impl TreeViewState {
    fn build_tree_entries(
        &mut self,
        section: Section,
        mut entries: Vec<GitStatusEntry>,
        seen_directories: &mut HashSet<TreeKey>,
    ) -> Vec<(GitListEntry, bool)> {
        if entries.is_empty() {
            return Vec::new();
        }

        entries.sort_by(|a, b| a.repo_path.cmp(&b.repo_path));

        let mut root = TreeNode::default();
        for entry in entries {
            let components: Vec<&str> = entry.repo_path.components().collect();
            if components.is_empty() {
                root.files.push(entry);
                continue;
            }

            let mut current = &mut root;
            let mut current_path = String::new();

            for (ix, component) in components.iter().enumerate() {
                if ix == components.len() - 1 {
                    current.files.push(entry.clone());
                } else {
                    if !current_path.is_empty() {
                        current_path.push('/');
                    }
                    current_path.push_str(component);
                    let dir_path = RepoPath::new(&current_path)
                        .expect("repo path from status entry component");

                    let component = SharedString::from(component.to_string());

                    current = current
                        .children
                        .entry(component.clone())
                        .or_insert_with(|| TreeNode {
                            name: component,
                            path: Some(dir_path),
                            ..Default::default()
                        });
                }
            }
        }

        let (flattened, _) = self.flatten_tree(&root, section, 0, seen_directories);
        flattened
    }

    fn flatten_tree(
        &mut self,
        node: &TreeNode,
        section: Section,
        depth: usize,
        seen_directories: &mut HashSet<TreeKey>,
    ) -> (Vec<(GitListEntry, bool)>, Vec<GitStatusEntry>) {
        let mut all_statuses = Vec::new();
        let mut flattened = Vec::new();

        for child in node.children.values() {
            let (terminal, name) = Self::compact_directory_chain(child);
            let Some(path) = terminal.path.clone().or_else(|| child.path.clone()) else {
                continue;
            };
            let (child_flattened, mut child_statuses) =
                self.flatten_tree(terminal, section, depth + 1, seen_directories);
            let key = TreeKey { section, path };
            let expanded = *self.expanded_dirs.get(&key).unwrap_or(&true);
            self.expanded_dirs.entry(key.clone()).or_insert(true);
            seen_directories.insert(key.clone());

            self.directory_descendants
                .insert(key.clone(), child_statuses.clone());

            flattened.push((
                GitListEntry::Directory(GitTreeDirEntry {
                    key,
                    name,
                    depth,
                    expanded,
                }),
                true,
            ));

            if expanded {
                flattened.extend(child_flattened);
            } else {
                flattened.extend(child_flattened.into_iter().map(|(child, _)| (child, false)));
            }

            all_statuses.append(&mut child_statuses);
        }

        for file in &node.files {
            all_statuses.push(file.clone());
            flattened.push((
                GitListEntry::TreeStatus(GitTreeStatusEntry {
                    entry: file.clone(),
                    depth,
                }),
                true,
            ));
        }

        (flattened, all_statuses)
    }

    fn compact_directory_chain(mut node: &TreeNode) -> (&TreeNode, SharedString) {
        let mut parts = vec![node.name.clone()];
        while node.files.is_empty() && node.children.len() == 1 {
            let Some(child) = node.children.values().next() else {
                continue;
            };
            if child.path.is_none() {
                break;
            }
            parts.push(child.name.clone());
            node = child;
        }
        let name = parts.join("/");
        (node, SharedString::from(name))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GitTreeStatusEntry {
    entry: GitStatusEntry,
    depth: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct TreeKey {
    section: Section,
    path: RepoPath,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GitTreeDirEntry {
    key: TreeKey,
    name: SharedString,
    depth: usize,
    // staged_state: ToggleState,
    expanded: bool,
}

#[derive(Default)]
struct TreeNode {
    name: SharedString,
    path: Option<RepoPath>,
    children: BTreeMap<SharedString, TreeNode>,
    files: Vec<GitStatusEntry>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GitStatusEntry {
    pub(crate) repo_path: RepoPath,
    pub(crate) status: FileStatus,
    pub(crate) staging: StageStatus,
}

impl GitStatusEntry {
    fn display_name(&self, path_style: PathStyle) -> String {
        self.repo_path
            .file_name()
            .map(|name| name.to_owned())
            .unwrap_or_else(|| self.repo_path.display(path_style).to_string())
    }

    fn parent_dir(&self, path_style: PathStyle) -> Option<String> {
        self.repo_path
            .parent()
            .map(|parent| parent.display(path_style).to_string())
    }
}

struct TruncatedPatch {
    header: String,
    hunks: Vec<String>,
    hunks_to_keep: usize,
}

impl TruncatedPatch {
    fn from_unified_diff(patch_str: &str) -> Option<Self> {
        let lines: Vec<&str> = patch_str.lines().collect();
        if lines.len() < 2 {
            return None;
        }
        let header = format!("{}\n{}\n", lines[0], lines[1]);
        let mut hunks = Vec::new();
        let mut current_hunk = String::new();
        for line in &lines[2..] {
            if line.starts_with("@@") {
                if !current_hunk.is_empty() {
                    hunks.push(current_hunk);
                }
                current_hunk = format!("{}\n", line);
            } else if !current_hunk.is_empty() {
                current_hunk.push_str(line);
                current_hunk.push('\n');
            }
        }
        if !current_hunk.is_empty() {
            hunks.push(current_hunk);
        }
        if hunks.is_empty() {
            return None;
        }
        let hunks_to_keep = hunks.len();
        Some(TruncatedPatch {
            header,
            hunks,
            hunks_to_keep,
        })
    }
    fn calculate_size(&self) -> usize {
        let mut size = self.header.len();
        for (i, hunk) in self.hunks.iter().enumerate() {
            if i < self.hunks_to_keep {
                size += hunk.len();
            }
        }
        size
    }
    fn to_string(&self) -> String {
        let mut out = self.header.clone();
        for (i, hunk) in self.hunks.iter().enumerate() {
            if i < self.hunks_to_keep {
                out.push_str(hunk);
            }
        }
        let skipped_hunks = self.hunks.len() - self.hunks_to_keep;
        if skipped_hunks > 0 {
            out.push_str(&format!("[...skipped {} hunks...]\n", skipped_hunks));
        }
        out
    }
}

pub struct GitPanel {
    pub(crate) active_repository: Option<Entity<Repository>>,
    pub(crate) commit_editor: Entity<Editor>,
    conflicted_count: usize,
    conflicted_staged_count: usize,
    add_coauthors: bool,
    generate_commit_message_task: Option<Task<Option<()>>>,
    entries: Vec<GitListEntry>,
    view_mode: GitPanelViewMode,
    entries_indices: HashMap<RepoPath, usize>,
    single_staged_entry: Option<GitStatusEntry>,
    single_tracked_entry: Option<GitStatusEntry>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    new_count: usize,
    entry_count: usize,
    changes_count: usize,
    new_staged_count: usize,
    pending_commit: Option<Task<()>>,
    amend_pending: bool,
    original_commit_message: Option<String>,
    signoff_enabled: bool,
    pending_serialization: Task<()>,
    pub(crate) project: Entity<Project>,
    scroll_handle: UniformListScrollHandle,
    max_width_item_index: Option<usize>,
    selected_entry: Option<usize>,
    marked_entries: Vec<usize>,
    tracked_count: usize,
    tracked_staged_count: usize,
    update_visible_entries_task: Task<()>,
    width: Option<Pixels>,
    pub(crate) workspace: WeakEntity<Workspace>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    modal_open: bool,
    show_placeholders: bool,
    local_committer: Option<GitCommitter>,
    local_committer_task: Option<Task<()>>,
    bulk_staging: Option<BulkStaging>,
    stash_entries: GitStash,
    _settings_subscription: Subscription,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BulkStaging {
    repo_id: RepositoryId,
    anchor: RepoPath,
}

const MAX_PANEL_EDITOR_LINES: usize = 6;

pub(crate) fn commit_message_editor(
    commit_message_buffer: Entity<Buffer>,
    placeholder: Option<SharedString>,
    project: Entity<Project>,
    in_panel: bool,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Editor {
    let buffer = cx.new(|cx| MultiBuffer::singleton(commit_message_buffer, cx));
    let max_lines = if in_panel { MAX_PANEL_EDITOR_LINES } else { 18 };
    let mut commit_editor = Editor::new(
        EditorMode::AutoHeight {
            min_lines: max_lines,
            max_lines: Some(max_lines),
        },
        buffer,
        None,
        window,
        cx,
    );
    commit_editor.set_collaboration_hub(Box::new(project));
    commit_editor.set_use_autoclose(false);
    commit_editor.set_show_gutter(false, cx);
    commit_editor.set_use_modal_editing(true);
    commit_editor.set_show_wrap_guides(false, cx);
    commit_editor.set_show_indent_guides(false, cx);
    let placeholder = placeholder.unwrap_or("Enter commit message".into());
    commit_editor.set_placeholder_text(&placeholder, window, cx);
    commit_editor
}

impl GitPanel {
    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let app_state = workspace.app_state().clone();
        let fs = app_state.fs.clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);

        cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, Self::focus_in).detach();

            let mut was_sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;
            let mut was_tree_view = GitPanelSettings::get_global(cx).tree_view;
            cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
                let sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;
                let tree_view = GitPanelSettings::get_global(cx).tree_view;
                if tree_view != was_tree_view {
                    this.view_mode = GitPanelViewMode::from_settings(cx);
                }
                if sort_by_path != was_sort_by_path || tree_view != was_tree_view {
                    this.bulk_staging.take();
                    this.update_visible_entries(window, cx);
                }
                was_sort_by_path = sort_by_path;
                was_tree_view = tree_view;
            })
            .detach();

            // just to let us render a placeholder editor.
            // Once the active git repo is set, this buffer will be replaced.
            let temporary_buffer = cx.new(|cx| Buffer::local("", cx));
            let commit_editor = cx.new(|cx| {
                commit_message_editor(temporary_buffer, None, project.clone(), true, window, cx)
            });

            commit_editor.update(cx, |editor, cx| {
                editor.clear(window, cx);
            });

            let scroll_handle = UniformListScrollHandle::new();

            let mut was_ai_enabled = AgentSettings::get_global(cx).enabled(cx);
            let _settings_subscription = cx.observe_global::<SettingsStore>(move |_, cx| {
                let is_ai_enabled = AgentSettings::get_global(cx).enabled(cx);
                if was_ai_enabled != is_ai_enabled {
                    was_ai_enabled = is_ai_enabled;
                    cx.notify();
                }
            });

            cx.subscribe_in(
                &git_store,
                window,
                move |this, _git_store, event, window, cx| match event {
                    GitStoreEvent::ActiveRepositoryChanged(_) => {
                        this.active_repository = this.project.read(cx).active_repository(cx);
                        this.schedule_update(window, cx);
                    }
                    GitStoreEvent::RepositoryUpdated(
                        _,
                        RepositoryEvent::StatusesChanged
                        | RepositoryEvent::BranchChanged
                        | RepositoryEvent::MergeHeadsChanged,
                        true,
                    )
                    | GitStoreEvent::RepositoryAdded
                    | GitStoreEvent::RepositoryRemoved(_) => {
                        this.schedule_update(window, cx);
                    }
                    GitStoreEvent::IndexWriteError(error) => {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                workspace.show_error(error, cx);
                            })
                            .ok();
                    }
                    GitStoreEvent::RepositoryUpdated(_, _, _) => {}
                    GitStoreEvent::JobsUpdated | GitStoreEvent::ConflictsUpdated => {}
                },
            )
            .detach();

            let mut this = Self {
                active_repository,
                commit_editor,
                conflicted_count: 0,
                conflicted_staged_count: 0,
                add_coauthors: true,
                generate_commit_message_task: None,
                entries: Vec::new(),
                view_mode: GitPanelViewMode::from_settings(cx),
                entries_indices: HashMap::default(),
                focus_handle: cx.focus_handle(),
                fs,
                new_count: 0,
                new_staged_count: 0,
                changes_count: 0,
                pending_commit: None,
                amend_pending: false,
                original_commit_message: None,
                signoff_enabled: false,
                pending_serialization: Task::ready(()),
                single_staged_entry: None,
                single_tracked_entry: None,
                project,
                scroll_handle,
                max_width_item_index: None,
                selected_entry: None,
                marked_entries: Vec::new(),
                tracked_count: 0,
                tracked_staged_count: 0,
                update_visible_entries_task: Task::ready(()),
                width: None,
                show_placeholders: false,
                local_committer: None,
                local_committer_task: None,
                context_menu: None,
                workspace: workspace.weak_handle(),
                modal_open: false,
                entry_count: 0,
                bulk_staging: None,
                stash_entries: Default::default(),
                _settings_subscription,
            };

            this.schedule_update(window, cx);
            this
        })
    }

    pub fn entry_by_path(&self, path: &RepoPath) -> Option<usize> {
        self.entries_indices.get(path).copied()
    }

    pub fn select_entry_by_path(
        &mut self,
        path: ProjectPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(git_repo) = self.active_repository.as_ref() else {
            return;
        };

        let (repo_path, section) = {
            let repo = git_repo.read(cx);
            let Some(repo_path) = repo.project_path_to_repo_path(&path, cx) else {
                return;
            };

            let section = repo
                .status_for_path(&repo_path)
                .map(|status| status.status)
                .map(|status| {
                    if repo.had_conflict_on_last_merge_head_change(&repo_path) {
                        Section::Conflict
                    } else if status.is_created() {
                        Section::New
                    } else {
                        Section::Tracked
                    }
                });

            (repo_path, section)
        };

        let mut needs_rebuild = false;
        if let (Some(section), Some(tree_state)) = (section, self.view_mode.tree_state_mut()) {
            let mut current_dir = repo_path.parent();
            while let Some(dir) = current_dir {
                let key = TreeKey {
                    section,
                    path: RepoPath::from_rel_path(dir),
                };

                if tree_state.expanded_dirs.get(&key) == Some(&false) {
                    tree_state.expanded_dirs.insert(key, true);
                    needs_rebuild = true;
                }

                current_dir = dir.parent();
            }
        }

        if needs_rebuild {
            self.update_visible_entries(window, cx);
        }

        let Some(ix) = self.entry_by_path(&repo_path) else {
            return;
        };

        self.selected_entry = Some(ix);
        self.scroll_to_selected_entry(cx);
    }

    fn serialization_key(workspace: &Workspace) -> Option<String> {
        workspace
            .database_id()
            .map(|id| i64::from(id).to_string())
            .or(workspace.session_id())
            .map(|id| format!("{}-{:?}", GIT_PANEL_KEY, id))
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let amend_pending = self.amend_pending;
        let signoff_enabled = self.signoff_enabled;

        self.pending_serialization = cx.spawn(async move |git_panel, cx| {
            cx.background_executor()
                .timer(SERIALIZATION_THROTTLE_TIME)
                .await;
            let Some(serialization_key) = git_panel
                .update(cx, |git_panel, cx| {
                    git_panel
                        .workspace
                        .read_with(cx, |workspace, _| Self::serialization_key(workspace))
                        .ok()
                        .flatten()
                })
                .ok()
                .flatten()
            else {
                return;
            };
            cx.background_spawn(
                async move {
                    KEY_VALUE_STORE
                        .write_kvp(
                            serialization_key,
                            serde_json::to_string(&SerializedGitPanel {
                                width,
                                amend_pending,
                                signoff_enabled,
                            })?,
                        )
                        .await?;
                    anyhow::Ok(())
                }
                .log_err(),
            )
            .await;
        });
    }

    pub(crate) fn set_modal_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.modal_open = open;
        cx.notify();
    }

    fn dispatch_context(&self, window: &mut Window, cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");

        if window
            .focused(cx)
            .is_some_and(|focused| self.focus_handle == focused)
        {
            dispatch_context.add("menu");
            dispatch_context.add("ChangesList");
        }

        if self.commit_editor.read(cx).is_focused(window) {
            dispatch_context.add("CommitEditor");
        }

        dispatch_context
    }

    fn close_panel(&mut self, _: &Close, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PanelEvent::Close);
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.contains_focused(window, cx) {
            cx.emit(Event::Focus);
        }
    }

    fn scroll_to_selected_entry(&mut self, cx: &mut Context<Self>) {
        let Some(selected_entry) = self.selected_entry else {
            cx.notify();
            return;
        };

        let visible_index = match &self.view_mode {
            GitPanelViewMode::Flat => Some(selected_entry),
            GitPanelViewMode::Tree(state) => state
                .logical_indices
                .iter()
                .position(|&ix| ix == selected_entry),
        };

        if let Some(visible_index) = visible_index {
            self.scroll_handle
                .scroll_to_item(visible_index, ScrollStrategy::Center);
        }

        cx.notify();
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.get_selected_entry().cloned() else {
            return;
        };

        if let GitListEntry::Directory(dir_entry) = entry {
            if dir_entry.expanded {
                self.select_next(&menu::SelectNext, window, cx);
            } else {
                self.toggle_directory(&dir_entry.key, window, cx);
            }
        } else {
            self.select_next(&menu::SelectNext, window, cx);
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.get_selected_entry().cloned() else {
            return;
        };

        if let GitListEntry::Directory(dir_entry) = entry {
            if dir_entry.expanded {
                self.toggle_directory(&dir_entry.key, window, cx);
            } else {
                self.select_previous(&menu::SelectPrevious, window, cx);
            }
        } else {
            self.select_previous(&menu::SelectPrevious, window, cx);
        }
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let first_entry = match &self.view_mode {
            GitPanelViewMode::Flat => self
                .entries
                .iter()
                .position(|entry| entry.status_entry().is_some()),
            GitPanelViewMode::Tree(state) => {
                let index = self.entries.iter().position(|entry| {
                    entry.status_entry().is_some() || entry.directory_entry().is_some()
                });

                index.map(|index| state.logical_indices[index])
            }
        };

        if let Some(first_entry) = first_entry {
            self.selected_entry = Some(first_entry);
            self.scroll_to_selected_entry(cx);
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let item_count = self.entries.len();
        if item_count == 0 {
            return;
        }

        let Some(selected_entry) = self.selected_entry else {
            return;
        };

        let new_index = match &self.view_mode {
            GitPanelViewMode::Flat => selected_entry.saturating_sub(1),
            GitPanelViewMode::Tree(state) => {
                let Some(current_logical_index) = state
                    .logical_indices
                    .iter()
                    .position(|&i| i == selected_entry)
                else {
                    return;
                };

                state.logical_indices[current_logical_index.saturating_sub(1)]
            }
        };

        if selected_entry == 0 && new_index == 0 {
            return;
        }

        if matches!(
            self.entries.get(new_index.saturating_sub(1)),
            Some(GitListEntry::Header(..))
        ) && new_index == 0
        {
            return;
        }

        if matches!(self.entries.get(new_index), Some(GitListEntry::Header(..))) {
            self.selected_entry = Some(new_index.saturating_sub(1));
        } else {
            self.selected_entry = Some(new_index);
        }

        self.scroll_to_selected_entry(cx);
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let item_count = self.entries.len();
        if item_count == 0 {
            return;
        }

        let Some(selected_entry) = self.selected_entry else {
            return;
        };

        if selected_entry == item_count - 1 {
            return;
        }

        let new_index = match &self.view_mode {
            GitPanelViewMode::Flat => selected_entry.saturating_add(1),
            GitPanelViewMode::Tree(state) => {
                let Some(current_logical_index) = state
                    .logical_indices
                    .iter()
                    .position(|&i| i == selected_entry)
                else {
                    return;
                };

                state.logical_indices[current_logical_index.saturating_add(1)]
            }
        };

        if matches!(self.entries.get(new_index), Some(GitListEntry::Header(..))) {
            self.selected_entry = Some(new_index.saturating_add(1));
        } else {
            self.selected_entry = Some(new_index);
        }

        self.scroll_to_selected_entry(cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        if self.entries.last().is_some() {
            self.selected_entry = Some(self.entries.len() - 1);
            self.scroll_to_selected_entry(cx);
        }
    }

    /// Show diff view at selected entry, only if the diff view is open
    fn move_diff_to_entry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let workspace = self.workspace.upgrade()?;

            if let Some(project_diff) = workspace.read(cx).item_of_type::<ProjectDiff>(cx) {
                let entry = self.entries.get(self.selected_entry?)?.status_entry()?;

                project_diff.update(cx, |project_diff, cx| {
                    project_diff.move_to_entry(entry.clone(), window, cx);
                });
            }

            Some(())
        });
    }

    fn first_entry(&mut self, _: &FirstEntry, window: &mut Window, cx: &mut Context<Self>) {
        self.select_first(&menu::SelectFirst, window, cx);
        self.move_diff_to_entry(window, cx);
    }

    fn last_entry(&mut self, _: &LastEntry, window: &mut Window, cx: &mut Context<Self>) {
        self.select_last(&menu::SelectLast, window, cx);
        self.move_diff_to_entry(window, cx);
    }

    fn next_entry(&mut self, _: &NextEntry, window: &mut Window, cx: &mut Context<Self>) {
        self.select_next(&menu::SelectNext, window, cx);
        self.move_diff_to_entry(window, cx);
    }

    fn previous_entry(&mut self, _: &PreviousEntry, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&menu::SelectPrevious, window, cx);
        self.move_diff_to_entry(window, cx);
    }

    fn focus_editor(&mut self, _: &FocusEditor, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_editor.update(cx, |editor, cx| {
            window.focus(&editor.focus_handle(cx), cx);
        });
        cx.notify();
    }

    fn select_first_entry_if_none(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let have_entries = self
            .active_repository
            .as_ref()
            .is_some_and(|active_repository| active_repository.read(cx).status_summary().count > 0);
        if have_entries && self.selected_entry.is_none() {
            self.select_first(&menu::SelectFirst, window, cx);
        }
    }

    fn focus_changes_list(
        &mut self,
        _: &FocusChanges,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_handle.focus(window, cx);
        self.select_first_entry_if_none(window, cx);
    }

    fn get_selected_entry(&self) -> Option<&GitListEntry> {
        self.selected_entry.and_then(|i| self.entries.get(i))
    }

    #[instrument(skip_all)]
    fn open_diff(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let entry = self.entries.get(self.selected_entry?)?.status_entry()?;
            let workspace = self.workspace.upgrade()?;
            let git_repo = self.active_repository.as_ref()?;

            if let Some(project_diff) = workspace.read(cx).active_item_as::<ProjectDiff>(cx)
                && let Some(project_path) = project_diff.read(cx).active_path(cx)
                && Some(&entry.repo_path)
                    == git_repo
                        .read(cx)
                        .project_path_to_repo_path(&project_path, cx)
                        .as_ref()
            {
                project_diff.focus_handle(cx).focus(window, cx);
                project_diff.update(cx, |project_diff, cx| project_diff.autoscroll(cx));
                return None;
            };

            self.workspace
                .update(cx, |workspace, cx| {
                    ProjectDiff::deploy_at(workspace, Some(entry.clone()), window, cx);
                })
                .ok();
            self.focus_handle.focus(window, cx);

            Some(())
        });
    }

    fn file_history(&mut self, _: &git::FileHistory, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let entry = self.entries.get(self.selected_entry?)?.status_entry()?;
            let active_repo = self.active_repository.as_ref()?;
            let repo_path = entry.repo_path.clone();
            let git_store = self.project.read(cx).git_store();

            FileHistoryView::open(
                repo_path,
                git_store.downgrade(),
                active_repo.downgrade(),
                self.workspace.clone(),
                window,
                cx,
            );

            Some(())
        });
    }

    #[instrument(skip_all)]
    fn open_file(
        &mut self,
        _: &menu::SecondaryConfirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let entry = self.entries.get(self.selected_entry?)?.status_entry()?;
            let active_repo = self.active_repository.as_ref()?;
            let path = active_repo
                .read(cx)
                .repo_path_to_project_path(&entry.repo_path, cx)?;
            if entry.status.is_deleted() {
                return None;
            }

            let open_task = self
                .workspace
                .update(cx, |workspace, cx| {
                    workspace.open_path_preview(path, None, false, false, true, window, cx)
                })
                .ok()?;

            cx.spawn_in(window, async move |_, mut cx| {
                let item = open_task
                    .await
                    .notify_async_err(&mut cx)
                    .ok_or_else(|| anyhow::anyhow!("Failed to open file"))?;
                if let Some(active_editor) = item.downcast::<Editor>() {
                    if let Some(diff_task) =
                        active_editor.update(cx, |editor, _cx| editor.wait_for_diff_to_load())
                    {
                        diff_task.await;
                    }

                    cx.update(|window, cx| {
                        active_editor.update(cx, |editor, cx| {
                            editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);

                            let snapshot = editor.snapshot(window, cx);
                            editor.go_to_hunk_before_or_after_position(
                                &snapshot,
                                language::Point::new(0, 0),
                                Direction::Next,
                                window,
                                cx,
                            );
                        })
                    })
                    .log_err();
                }

                anyhow::Ok(())
            })
            .detach();

            Some(())
        });
    }

    fn revert_selected(
        &mut self,
        action: &git::RestoreFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let path_style = self.project.read(cx).path_style(cx);
        maybe!({
            let list_entry = self.entries.get(self.selected_entry?)?.clone();
            let entry = list_entry.status_entry()?.to_owned();
            let skip_prompt = action.skip_prompt || entry.status.is_created();

            let prompt = if skip_prompt {
                Task::ready(Ok(0))
            } else {
                let prompt = window.prompt(
                    PromptLevel::Warning,
                    &format!(
                        "Are you sure you want to discard changes to {}?",
                        entry
                            .repo_path
                            .file_name()
                            .unwrap_or(entry.repo_path.display(path_style).as_ref()),
                    ),
                    None,
                    &["Discard Changes", "Cancel"],
                    cx,
                );
                cx.background_spawn(prompt)
            };

            let this = cx.weak_entity();
            window
                .spawn(cx, async move |cx| {
                    if prompt.await? != 0 {
                        return anyhow::Ok(());
                    }

                    this.update_in(cx, |this, window, cx| {
                        this.revert_entry(&entry, window, cx);
                    })?;

                    Ok(())
                })
                .detach();
            Some(())
        });
    }

    fn add_to_gitignore(
        &mut self,
        _: &git::AddToGitignore,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let list_entry = self.entries.get(self.selected_entry?)?.clone();
            let entry = list_entry.status_entry()?.to_owned();

            if !entry.status.is_created() {
                return Some(());
            }

            let project = self.project.downgrade();
            let repo_path = entry.repo_path;
            let active_repository = self.active_repository.as_ref()?.downgrade();

            cx.spawn(async move |_, cx| {
                let file_path_str = repo_path.as_ref().display(PathStyle::Posix);

                let repo_root = active_repository.read_with(cx, |repository, _| {
                    repository.snapshot().work_directory_abs_path
                })?;

                let gitignore_abs_path = repo_root.join(".gitignore");

                let buffer: Entity<Buffer> = project
                    .update(cx, |project, cx| {
                        project.open_local_buffer(gitignore_abs_path, cx)
                    })?
                    .await?;

                let mut should_save = false;
                buffer.update(cx, |buffer, cx| {
                    let existing_content = buffer.text();

                    if existing_content
                        .lines()
                        .any(|line: &str| line.trim() == file_path_str)
                    {
                        return;
                    }

                    let insert_position = existing_content.len();
                    let new_entry = if existing_content.is_empty() {
                        format!("{}\n", file_path_str)
                    } else if existing_content.ends_with('\n') {
                        format!("{}\n", file_path_str)
                    } else {
                        format!("\n{}\n", file_path_str)
                    };

                    buffer.edit([(insert_position..insert_position, new_entry)], None, cx);
                    should_save = true;
                });

                if should_save {
                    project
                        .update(cx, |project, cx| project.save_buffer(buffer, cx))?
                        .await?;
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            Some(())
        });
    }

    fn revert_entry(
        &mut self,
        entry: &GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let active_repo = self.active_repository.clone()?;
            let path = active_repo
                .read(cx)
                .repo_path_to_project_path(&entry.repo_path, cx)?;
            let workspace = self.workspace.clone();

            if entry.status.staging().has_staged() {
                self.change_file_stage(false, vec![entry.clone()], cx);
            }
            let filename = path.path.file_name()?.to_string();

            if !entry.status.is_created() {
                self.perform_checkout(vec![entry.clone()], window, cx);
            } else {
                let prompt = prompt(&format!("Trash {}?", filename), None, window, cx);
                cx.spawn_in(window, async move |_, cx| {
                    match prompt.await? {
                        TrashCancel::Trash => {}
                        TrashCancel::Cancel => return Ok(()),
                    }
                    let task = workspace.update(cx, |workspace, cx| {
                        workspace
                            .project()
                            .update(cx, |project, cx| project.delete_file(path, true, cx))
                    })?;
                    if let Some(task) = task {
                        task.await?;
                    }
                    Ok(())
                })
                .detach_and_prompt_err(
                    "Failed to trash file",
                    window,
                    cx,
                    |e, _, _| Some(format!("{e}")),
                );
            }
            Some(())
        });
    }

    fn perform_checkout(
        &mut self,
        entries: Vec<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        let task = cx.spawn_in(window, async move |this, cx| {
            let tasks: Vec<_> = workspace.update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    entries
                        .iter()
                        .filter_map(|entry| {
                            let path = active_repository
                                .read(cx)
                                .repo_path_to_project_path(&entry.repo_path, cx)?;
                            Some(project.open_buffer(path, cx))
                        })
                        .collect()
                })
            })?;

            let buffers = futures::future::join_all(tasks).await;

            this.update_in(cx, |this, window, cx| {
                let task = active_repository.update(cx, |repo, cx| {
                    repo.checkout_files(
                        "HEAD",
                        entries
                            .into_iter()
                            .map(|entries| entries.repo_path)
                            .collect(),
                        cx,
                    )
                });
                this.update_visible_entries(window, cx);
                cx.notify();
                task
            })?
            .await?;

            let tasks: Vec<_> = cx.update(|_, cx| {
                buffers
                    .iter()
                    .filter_map(|buffer| {
                        buffer.as_ref().ok()?.update(cx, |buffer, cx| {
                            buffer.is_dirty().then(|| buffer.reload(cx))
                        })
                    })
                    .collect()
            })?;

            futures::future::join_all(tasks).await;

            Ok(())
        });

        cx.spawn_in(window, async move |this, cx| {
            let result = task.await;

            this.update_in(cx, |this, window, cx| {
                if let Err(err) = result {
                    this.update_visible_entries(window, cx);
                    this.show_error_toast("checkout", err, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    fn restore_tracked_files(
        &mut self,
        _: &RestoreTrackedFiles,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entries = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry().cloned())
            .filter(|status_entry| !status_entry.status.is_created())
            .collect::<Vec<_>>();

        match entries.len() {
            0 => return,
            1 => return self.revert_entry(&entries[0], window, cx),
            _ => {}
        }
        let mut details = entries
            .iter()
            .filter_map(|entry| entry.repo_path.as_ref().file_name())
            .map(|filename| filename.to_string())
            .take(5)
            .join("\n");
        if entries.len() > 5 {
            details.push_str(&format!("\nand {} more…", entries.len() - 5))
        }

        #[derive(strum::EnumIter, strum::VariantNames)]
        #[strum(serialize_all = "title_case")]
        enum RestoreCancel {
            RestoreTrackedFiles,
            Cancel,
        }
        let prompt = prompt(
            "Discard changes to these files?",
            Some(&details),
            window,
            cx,
        );
        cx.spawn_in(window, async move |this, cx| {
            if let Ok(RestoreCancel::RestoreTrackedFiles) = prompt.await {
                this.update_in(cx, |this, window, cx| {
                    this.perform_checkout(entries, window, cx);
                })
                .ok();
            }
        })
        .detach();
    }

    fn clean_all(&mut self, _: &TrashUntrackedFiles, window: &mut Window, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();
        let Some(active_repo) = self.active_repository.clone() else {
            return;
        };
        let to_delete = self
            .entries
            .iter()
            .filter_map(|entry| entry.status_entry())
            .filter(|status_entry| status_entry.status.is_created())
            .cloned()
            .collect::<Vec<_>>();

        match to_delete.len() {
            0 => return,
            1 => return self.revert_entry(&to_delete[0], window, cx),
            _ => {}
        };

        let mut details = to_delete
            .iter()
            .map(|entry| {
                entry
                    .repo_path
                    .as_ref()
                    .file_name()
                    .map(|f| f.to_string())
                    .unwrap_or_default()
            })
            .take(5)
            .join("\n");

        if to_delete.len() > 5 {
            details.push_str(&format!("\nand {} more…", to_delete.len() - 5))
        }

        let prompt = prompt("Trash these files?", Some(&details), window, cx);
        cx.spawn_in(window, async move |this, cx| {
            match prompt.await? {
                TrashCancel::Trash => {}
                TrashCancel::Cancel => return Ok(()),
            }
            let tasks = workspace.update(cx, |workspace, cx| {
                to_delete
                    .iter()
                    .filter_map(|entry| {
                        workspace.project().update(cx, |project, cx| {
                            let project_path = active_repo
                                .read(cx)
                                .repo_path_to_project_path(&entry.repo_path, cx)?;
                            project.delete_file(project_path, true, cx)
                        })
                    })
                    .collect::<Vec<_>>()
            })?;
            let to_unstage = to_delete
                .into_iter()
                .filter(|entry| !entry.status.staging().is_fully_unstaged())
                .collect();
            this.update(cx, |this, cx| this.change_file_stage(false, to_unstage, cx))?;
            for task in tasks {
                task.await?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to trash files", window, cx, |e, _, _| {
            Some(format!("{e}"))
        });
    }

    fn change_all_files_stage(&mut self, stage: bool, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        cx.spawn({
            async move |this, cx| {
                let result = this
                    .update(cx, |this, cx| {
                        let task = active_repository.update(cx, |repo, cx| {
                            if stage {
                                repo.stage_all(cx)
                            } else {
                                repo.unstage_all(cx)
                            }
                        });
                        this.update_counts(active_repository.read(cx));
                        cx.notify();
                        task
                    })?
                    .await;

                this.update(cx, |this, cx| {
                    if let Err(err) = result {
                        this.show_error_toast(if stage { "add" } else { "reset" }, err, cx);
                    }
                    cx.notify()
                })
            }
        })
        .detach();
    }

    fn stage_status_for_entry(entry: &GitStatusEntry, repo: &Repository) -> StageStatus {
        // Checking for current staged/unstaged file status is a chained operation:
        // 1. first, we check for any pending operation recorded in repository
        // 2. if there are no pending ops either running or finished, we then ask the repository
        //    for the most up-to-date file status read from disk - we do this since `entry` arg to this function `render_entry`
        //    is likely to be staled, and may lead to weird artifacts in the form of subsecond auto-uncheck/check on
        //    the checkbox's state (or flickering) which is undesirable.
        // 3. finally, if there is no info about this `entry` in the repo, we fall back to whatever status is encoded
        //    in `entry` arg.
        repo.pending_ops_for_path(&entry.repo_path)
            .map(|ops| {
                if ops.staging() || ops.staged() {
                    StageStatus::Staged
                } else {
                    StageStatus::Unstaged
                }
            })
            .or_else(|| {
                repo.status_for_path(&entry.repo_path)
                    .map(|status| status.status.staging())
            })
            .unwrap_or(entry.staging)
    }

    fn stage_status_for_directory(
        &self,
        entry: &GitTreeDirEntry,
        repo: &Repository,
    ) -> StageStatus {
        let GitPanelViewMode::Tree(tree_state) = &self.view_mode else {
            util::debug_panic!("We should never render a directory entry while in flat view mode");
            return StageStatus::Unstaged;
        };

        let Some(descendants) = tree_state.directory_descendants.get(&entry.key) else {
            return StageStatus::Unstaged;
        };

        let mut fully_staged_count = 0usize;
        let mut any_staged_or_partially_staged = false;

        for descendant in descendants {
            match GitPanel::stage_status_for_entry(descendant, repo) {
                StageStatus::Staged => {
                    fully_staged_count += 1;
                    any_staged_or_partially_staged = true;
                }
                StageStatus::PartiallyStaged => {
                    any_staged_or_partially_staged = true;
                }
                StageStatus::Unstaged => {}
            }
        }

        if descendants.is_empty() {
            StageStatus::Unstaged
        } else if fully_staged_count == descendants.len() {
            StageStatus::Staged
        } else if any_staged_or_partially_staged {
            StageStatus::PartiallyStaged
        } else {
            StageStatus::Unstaged
        }
    }

    pub fn stage_all(&mut self, _: &StageAll, _window: &mut Window, cx: &mut Context<Self>) {
        self.change_all_files_stage(true, cx);
    }

    pub fn unstage_all(&mut self, _: &UnstageAll, _window: &mut Window, cx: &mut Context<Self>) {
        self.change_all_files_stage(false, cx);
    }

    fn toggle_staged_for_entry(
        &mut self,
        entry: &GitListEntry,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        let mut set_anchor: Option<RepoPath> = None;
        let mut clear_anchor = None;

        let (stage, repo_paths) = {
            let repo = active_repository.read(cx);
            match entry {
                GitListEntry::Status(status_entry) => {
                    let repo_paths = vec![status_entry.clone()];
                    let stage = match GitPanel::stage_status_for_entry(status_entry, &repo) {
                        StageStatus::Staged => {
                            if let Some(op) = self.bulk_staging.clone()
                                && op.anchor == status_entry.repo_path
                            {
                                clear_anchor = Some(op.anchor);
                            }
                            false
                        }
                        StageStatus::Unstaged | StageStatus::PartiallyStaged => {
                            set_anchor = Some(status_entry.repo_path.clone());
                            true
                        }
                    };
                    (stage, repo_paths)
                }
                GitListEntry::TreeStatus(status_entry) => {
                    let repo_paths = vec![status_entry.entry.clone()];
                    let stage = match GitPanel::stage_status_for_entry(&status_entry.entry, &repo) {
                        StageStatus::Staged => {
                            if let Some(op) = self.bulk_staging.clone()
                                && op.anchor == status_entry.entry.repo_path
                            {
                                clear_anchor = Some(op.anchor);
                            }
                            false
                        }
                        StageStatus::Unstaged | StageStatus::PartiallyStaged => {
                            set_anchor = Some(status_entry.entry.repo_path.clone());
                            true
                        }
                    };
                    (stage, repo_paths)
                }
                GitListEntry::Header(section) => {
                    let goal_staged_state = !self.header_state(section.header).selected();
                    let entries = self
                        .entries
                        .iter()
                        .filter_map(|entry| entry.status_entry())
                        .filter(|status_entry| {
                            section.contains(status_entry, &repo)
                                && GitPanel::stage_status_for_entry(status_entry, &repo).as_bool()
                                    != Some(goal_staged_state)
                        })
                        .cloned()
                        .collect::<Vec<_>>();

                    (goal_staged_state, entries)
                }
                GitListEntry::Directory(entry) => {
                    let goal_staged_state = match self.stage_status_for_directory(entry, repo) {
                        StageStatus::Staged => StageStatus::Unstaged,
                        StageStatus::Unstaged | StageStatus::PartiallyStaged => StageStatus::Staged,
                    };
                    let goal_stage = goal_staged_state == StageStatus::Staged;

                    let entries = self
                        .view_mode
                        .tree_state()
                        .and_then(|state| state.directory_descendants.get(&entry.key))
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|status_entry| {
                            GitPanel::stage_status_for_entry(status_entry, &repo)
                                != goal_staged_state
                        })
                        .collect::<Vec<_>>();
                    (goal_stage, entries)
                }
            }
        };
        if let Some(anchor) = clear_anchor {
            if let Some(op) = self.bulk_staging.clone()
                && op.anchor == anchor
            {
                self.bulk_staging = None;
            }
        }
        if let Some(anchor) = set_anchor {
            self.set_bulk_staging_anchor(anchor, cx);
        }

        self.change_file_stage(stage, repo_paths, cx);
    }

    fn change_file_stage(
        &mut self,
        stage: bool,
        entries: Vec<GitStatusEntry>,
        cx: &mut Context<Self>,
    ) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        cx.spawn({
            async move |this, cx| {
                let result = this
                    .update(cx, |this, cx| {
                        let task = active_repository.update(cx, |repo, cx| {
                            let repo_paths = entries
                                .iter()
                                .map(|entry| entry.repo_path.clone())
                                .collect();
                            if stage {
                                repo.stage_entries(repo_paths, cx)
                            } else {
                                repo.unstage_entries(repo_paths, cx)
                            }
                        });
                        this.update_counts(active_repository.read(cx));
                        cx.notify();
                        task
                    })?
                    .await;

                this.update(cx, |this, cx| {
                    if let Err(err) = result {
                        this.show_error_toast(if stage { "add" } else { "reset" }, err, cx);
                    }
                    cx.notify();
                })
            }
        })
        .detach();
    }

    pub fn total_staged_count(&self) -> usize {
        self.tracked_staged_count + self.new_staged_count + self.conflicted_staged_count
    }

    pub fn stash_pop(&mut self, _: &StashPop, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        cx.spawn({
            async move |this, cx| {
                let stash_task = active_repository
                    .update(cx, |repo, cx| repo.stash_pop(None, cx))
                    .await;
                this.update(cx, |this, cx| {
                    stash_task
                        .map_err(|e| {
                            this.show_error_toast("stash pop", e, cx);
                        })
                        .ok();
                    cx.notify();
                })
            }
        })
        .detach();
    }

    pub fn stash_apply(&mut self, _: &StashApply, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        cx.spawn({
            async move |this, cx| {
                let stash_task = active_repository
                    .update(cx, |repo, cx| repo.stash_apply(None, cx))
                    .await;
                this.update(cx, |this, cx| {
                    stash_task
                        .map_err(|e| {
                            this.show_error_toast("stash apply", e, cx);
                        })
                        .ok();
                    cx.notify();
                })
            }
        })
        .detach();
    }

    pub fn stash_all(&mut self, _: &StashAll, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        cx.spawn({
            async move |this, cx| {
                let stash_task = active_repository
                    .update(cx, |repo, cx| repo.stash_all(cx))
                    .await;
                this.update(cx, |this, cx| {
                    stash_task
                        .map_err(|e| {
                            this.show_error_toast("stash", e, cx);
                        })
                        .ok();
                    cx.notify();
                })
            }
        })
        .detach();
    }

    pub fn commit_message_buffer(&self, cx: &App) -> Entity<Buffer> {
        self.commit_editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .unwrap()
    }

    fn toggle_staged_for_selected(
        &mut self,
        _: &git::ToggleStaged,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selected_entry) = self.get_selected_entry().cloned() {
            self.toggle_staged_for_entry(&selected_entry, window, cx);
        }
    }

    fn stage_range(&mut self, _: &git::StageRange, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(index) = self.selected_entry else {
            return;
        };
        self.stage_bulk(index, cx);
    }

    fn stage_selected(&mut self, _: &git::StageFile, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(selected_entry) = self.get_selected_entry() else {
            return;
        };
        let Some(status_entry) = selected_entry.status_entry() else {
            return;
        };
        if status_entry.staging != StageStatus::Staged {
            self.change_file_stage(true, vec![status_entry.clone()], cx);
        }
    }

    fn unstage_selected(
        &mut self,
        _: &git::UnstageFile,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(selected_entry) = self.get_selected_entry() else {
            return;
        };
        let Some(status_entry) = selected_entry.status_entry() else {
            return;
        };
        if status_entry.staging != StageStatus::Unstaged {
            self.change_file_stage(false, vec![status_entry.clone()], cx);
        }
    }

    fn on_commit(&mut self, _: &git::Commit, window: &mut Window, cx: &mut Context<Self>) {
        if self.commit(&self.commit_editor.focus_handle(cx), window, cx) {
            telemetry::event!("Git Committed", source = "Git Panel");
        }
    }

    /// Commits staged changes with the current commit message.
    ///
    /// Returns `true` if the commit was executed, `false` otherwise.
    pub(crate) fn commit(
        &mut self,
        commit_editor_focus_handle: &FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.amend_pending {
            return false;
        }

        if commit_editor_focus_handle.contains_focused(window, cx) {
            self.commit_changes(
                CommitOptions {
                    amend: false,
                    signoff: self.signoff_enabled,
                },
                window,
                cx,
            );
            true
        } else {
            cx.propagate();
            false
        }
    }

    fn on_amend(&mut self, _: &git::Amend, window: &mut Window, cx: &mut Context<Self>) {
        if self.amend(&self.commit_editor.focus_handle(cx), window, cx) {
            telemetry::event!("Git Amended", source = "Git Panel");
        }
    }

    /// Amends the most recent commit with staged changes and/or an updated commit message.
    ///
    /// Uses a two-stage workflow where the first invocation loads the commit
    /// message for editing, second invocation performs the amend. Returns
    /// `true` if the amend was executed, `false` otherwise.
    pub(crate) fn amend(
        &mut self,
        commit_editor_focus_handle: &FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if commit_editor_focus_handle.contains_focused(window, cx) {
            if self.head_commit(cx).is_some() {
                if !self.amend_pending {
                    self.set_amend_pending(true, cx);
                    self.load_last_commit_message(cx);

                    return false;
                } else {
                    self.commit_changes(
                        CommitOptions {
                            amend: true,
                            signoff: self.signoff_enabled,
                        },
                        window,
                        cx,
                    );

                    return true;
                }
            }
            return false;
        } else {
            cx.propagate();
            return false;
        }
    }
    pub fn head_commit(&self, cx: &App) -> Option<CommitDetails> {
        self.active_repository
            .as_ref()
            .and_then(|repo| repo.read(cx).head_commit.as_ref())
            .cloned()
    }

    pub fn load_last_commit_message(&mut self, cx: &mut Context<Self>) {
        let Some(head_commit) = self.head_commit(cx) else {
            return;
        };

        let recent_sha = head_commit.sha.to_string();
        let detail_task = self.load_commit_details(recent_sha, cx);
        cx.spawn(async move |this, cx| {
            if let Ok(message) = detail_task.await.map(|detail| detail.message) {
                this.update(cx, |this, cx| {
                    this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                        let start = buffer.anchor_before(0);
                        let end = buffer.anchor_after(buffer.len());
                        buffer.edit([(start..end, message)], None, cx);
                    });
                })
                .log_err();
            }
        })
        .detach();
    }

    fn custom_or_suggested_commit_message(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        let git_commit_language = self
            .commit_editor
            .read(cx)
            .language_at(MultiBufferOffset(0), cx);
        let message = self.commit_editor.read(cx).text(cx);
        if message.is_empty() {
            return self
                .suggest_commit_message(cx)
                .filter(|message| !message.trim().is_empty());
        } else if message.trim().is_empty() {
            return None;
        }
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(message, cx);
            buffer.set_language(git_commit_language, cx);
            buffer
        });
        let editor = cx.new(|cx| Editor::for_buffer(buffer, None, window, cx));
        let wrapped_message = editor.update(cx, |editor, cx| {
            editor.select_all(&Default::default(), window, cx);
            editor.rewrap_impl(
                RewrapOptions {
                    override_language_settings: false,
                    preserve_existing_whitespace: true,
                },
                cx,
            );
            editor.text(cx)
        });
        if wrapped_message.trim().is_empty() {
            return None;
        }
        Some(wrapped_message)
    }

    fn has_commit_message(&self, cx: &mut Context<Self>) -> bool {
        let text = self.commit_editor.read(cx).text(cx);
        if !text.trim().is_empty() {
            true
        } else if text.is_empty() {
            self.suggest_commit_message(cx)
                .is_some_and(|text| !text.trim().is_empty())
        } else {
            false
        }
    }

    pub(crate) fn commit_changes(
        &mut self,
        options: CommitOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        let error_spawn = |message, window: &mut Window, cx: &mut App| {
            let prompt = window.prompt(PromptLevel::Warning, message, None, &["Ok"], cx);
            cx.spawn(async move |_| {
                prompt.await.ok();
            })
            .detach();
        };

        if self.has_unstaged_conflicts() {
            error_spawn(
                "There are still conflicts. You must stage these before committing",
                window,
                cx,
            );
            return;
        }

        let askpass = self.askpass_delegate("git commit", window, cx);
        let commit_message = self.custom_or_suggested_commit_message(window, cx);

        let Some(mut message) = commit_message else {
            self.commit_editor
                .read(cx)
                .focus_handle(cx)
                .focus(window, cx);
            return;
        };

        if self.add_coauthors {
            self.fill_co_authors(&mut message, cx);
        }

        let task = if self.has_staged_changes() {
            // Repository serializes all git operations, so we can just send a commit immediately
            let commit_task = active_repository.update(cx, |repo, cx| {
                repo.commit(message.into(), None, options, askpass, cx)
            });
            cx.background_spawn(async move { commit_task.await? })
        } else {
            let changed_files = self
                .entries
                .iter()
                .filter_map(|entry| entry.status_entry())
                .filter(|status_entry| !status_entry.status.is_created())
                .map(|status_entry| status_entry.repo_path.clone())
                .collect::<Vec<_>>();

            if changed_files.is_empty() && !options.amend {
                error_spawn("No changes to commit", window, cx);
                return;
            }

            let stage_task =
                active_repository.update(cx, |repo, cx| repo.stage_entries(changed_files, cx));
            cx.spawn(async move |_, cx| {
                stage_task.await?;
                let commit_task = active_repository.update(cx, |repo, cx| {
                    repo.commit(message.into(), None, options, askpass, cx)
                });
                commit_task.await?
            })
        };
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = task.await;
            this.update_in(cx, |this, window, cx| {
                this.pending_commit.take();

                match result {
                    Ok(()) => {
                        if options.amend {
                            this.set_amend_pending(false, cx);
                        } else {
                            this.commit_editor
                                .update(cx, |editor, cx| editor.clear(window, cx));
                            this.original_commit_message = None;
                        }
                    }
                    Err(e) => this.show_error_toast("commit", e, cx),
                }
            })
            .ok();
        });

        self.pending_commit = Some(task);
    }

    pub(crate) fn uncommit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        telemetry::event!("Git Uncommitted");

        let confirmation = self.check_for_pushed_commits(window, cx);
        let prior_head = self.load_commit_details("HEAD".to_string(), cx);

        let task = cx.spawn_in(window, async move |this, cx| {
            let result = maybe!(async {
                if let Ok(true) = confirmation.await {
                    let prior_head = prior_head.await?;

                    repo.update(cx, |repo, cx| {
                        repo.reset("HEAD^".to_string(), ResetMode::Soft, cx)
                    })
                    .await??;

                    Ok(Some(prior_head))
                } else {
                    Ok(None)
                }
            })
            .await;

            this.update_in(cx, |this, window, cx| {
                this.pending_commit.take();
                match result {
                    Ok(None) => {}
                    Ok(Some(prior_commit)) => {
                        this.commit_editor.update(cx, |editor, cx| {
                            editor.set_text(prior_commit.message, window, cx)
                        });
                    }
                    Err(e) => this.show_error_toast("reset", e, cx),
                }
            })
            .ok();
        });

        self.pending_commit = Some(task);
    }

    fn check_for_pushed_commits(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = anyhow::Result<bool>> + use<> {
        let repo = self.active_repository.clone();
        let mut cx = window.to_async(cx);

        async move {
            let repo = repo.context("No active repository")?;

            let pushed_to: Vec<SharedString> = repo
                .update(&mut cx, |repo, _| repo.check_for_pushed_commits())
                .await??;

            if pushed_to.is_empty() {
                Ok(true)
            } else {
                #[derive(strum::EnumIter, strum::VariantNames)]
                #[strum(serialize_all = "title_case")]
                enum CancelUncommit {
                    Uncommit,
                    Cancel,
                }
                let detail = format!(
                    "This commit was already pushed to {}.",
                    pushed_to.into_iter().join(", ")
                );
                let result = cx
                    .update(|window, cx| prompt("Are you sure?", Some(&detail), window, cx))?
                    .await?;

                match result {
                    CancelUncommit::Cancel => Ok(false),
                    CancelUncommit::Uncommit => Ok(true),
                }
            }
        }
    }

    /// Suggests a commit message based on the changed files and their statuses
    pub fn suggest_commit_message(&self, cx: &App) -> Option<String> {
        if let Some(merge_message) = self
            .active_repository
            .as_ref()
            .and_then(|repo| repo.read(cx).merge.message.as_ref())
        {
            return Some(merge_message.to_string());
        }

        let git_status_entry = if let Some(staged_entry) = &self.single_staged_entry {
            Some(staged_entry)
        } else if self.total_staged_count() == 0
            && let Some(single_tracked_entry) = &self.single_tracked_entry
        {
            Some(single_tracked_entry)
        } else {
            None
        }?;

        let action_text = if git_status_entry.status.is_deleted() {
            Some("Delete")
        } else if git_status_entry.status.is_created() {
            Some("Create")
        } else if git_status_entry.status.is_modified() {
            Some("Update")
        } else {
            None
        }?;

        let file_name = git_status_entry
            .repo_path
            .file_name()
            .unwrap_or_default()
            .to_string();

        Some(format!("{} {}", action_text, file_name))
    }

    fn generate_commit_message_action(
        &mut self,
        _: &git::GenerateCommitMessage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.generate_commit_message(cx);
    }

    fn split_patch(patch: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_patch = String::new();

        for line in patch.lines() {
            if line.starts_with("---") && !current_patch.is_empty() {
                result.push(current_patch.trim_end_matches('\n').into());
                current_patch = String::new();
            }
            current_patch.push_str(line);
            current_patch.push('\n');
        }

        if !current_patch.is_empty() {
            result.push(current_patch.trim_end_matches('\n').into());
        }

        result
    }
    fn truncate_iteratively(patch: &str, max_bytes: usize) -> String {
        let mut current_size = patch.len();
        if current_size <= max_bytes {
            return patch.to_string();
        }
        let file_patches = Self::split_patch(patch);
        let mut file_infos: Vec<TruncatedPatch> = file_patches
            .iter()
            .filter_map(|patch| TruncatedPatch::from_unified_diff(patch))
            .collect();

        if file_infos.is_empty() {
            return patch.to_string();
        }

        current_size = file_infos.iter().map(|f| f.calculate_size()).sum::<usize>();
        while current_size > max_bytes {
            let file_idx = file_infos
                .iter()
                .enumerate()
                .filter(|(_, f)| f.hunks_to_keep > 1)
                .max_by_key(|(_, f)| f.hunks_to_keep)
                .map(|(idx, _)| idx);
            match file_idx {
                Some(idx) => {
                    let file = &mut file_infos[idx];
                    let size_before = file.calculate_size();
                    file.hunks_to_keep -= 1;
                    let size_after = file.calculate_size();
                    let saved = size_before.saturating_sub(size_after);
                    current_size = current_size.saturating_sub(saved);
                }
                None => {
                    break;
                }
            }
        }

        file_infos
            .iter()
            .map(|info| info.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn compress_commit_diff(diff_text: &str, max_bytes: usize) -> String {
        if diff_text.len() <= max_bytes {
            return diff_text.to_string();
        }

        let mut compressed = diff_text
            .lines()
            .map(|line| {
                if line.len() > 256 {
                    format!("{}...[truncated]\n", &line[..line.floor_char_boundary(256)])
                } else {
                    format!("{}\n", line)
                }
            })
            .collect::<Vec<_>>()
            .join("");

        if compressed.len() <= max_bytes {
            return compressed;
        }

        compressed = Self::truncate_iteratively(&compressed, max_bytes);

        compressed
    }

    async fn load_project_rules(
        project: &Entity<Project>,
        repo_work_dir: &Arc<Path>,
        cx: &mut AsyncApp,
    ) -> Option<String> {
        let rules_path = cx.update(|cx| {
            for worktree in project.read(cx).worktrees(cx) {
                let worktree_abs_path = worktree.read(cx).abs_path();
                if !worktree_abs_path.starts_with(&repo_work_dir) {
                    continue;
                }

                let worktree_snapshot = worktree.read(cx).snapshot();
                for rules_name in RULES_FILE_NAMES {
                    if let Ok(rel_path) = RelPath::unix(rules_name) {
                        if let Some(entry) = worktree_snapshot.entry_for_path(rel_path) {
                            if entry.is_file() {
                                return Some(ProjectPath {
                                    worktree_id: worktree.read(cx).id(),
                                    path: entry.path.clone(),
                                });
                            }
                        }
                    }
                }
            }
            None
        })?;

        let buffer = project
            .update(cx, |project, cx| project.open_buffer(rules_path, cx))
            .await
            .ok()?;

        let content = buffer
            .read_with(cx, |buffer, _| buffer.text())
            .trim()
            .to_string();

        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }

    async fn load_commit_message_prompt(
        is_using_legacy_zed_pro: bool,
        cx: &mut AsyncApp,
    ) -> String {
        // Remove this once we stop supporting legacy Zed Pro
        // In legacy Zed Pro, Git commit summary generation did not count as a
        // prompt. If the user changes the prompt, our classification will fail,
        // meaning that users will be charged for generating commit messages.
        if is_using_legacy_zed_pro {
            return BuiltInPrompt::CommitMessage.default_content().to_string();
        }

        let load = async {
            let store = cx.update(|cx| PromptStore::global(cx)).await.ok()?;
            store
                .update(cx, |s, cx| {
                    s.load(PromptId::BuiltIn(BuiltInPrompt::CommitMessage), cx)
                })
                .await
                .ok()
        };
        load.await
            .unwrap_or_else(|| BuiltInPrompt::CommitMessage.default_content().to_string())
    }

    /// Generates a commit message using an LLM.
    pub fn generate_commit_message(&mut self, cx: &mut Context<Self>) {
        if !self.can_commit() || !AgentSettings::get_global(cx).enabled(cx) {
            return;
        }

        let Some(ConfiguredModel { provider, model }) =
            LanguageModelRegistry::read_global(cx).commit_message_model()
        else {
            return;
        };

        let Some(repo) = self.active_repository.as_ref() else {
            return;
        };

        telemetry::event!("Git Commit Message Generated");

        let diff = repo.update(cx, |repo, cx| {
            if self.has_staged_changes() {
                repo.diff(DiffType::HeadToIndex, cx)
            } else {
                repo.diff(DiffType::HeadToWorktree, cx)
            }
        });

        let temperature = AgentSettings::temperature_for_model(&model, cx);
        let project = self.project.clone();
        let repo_work_dir = repo.read(cx).work_directory_abs_path.clone();

        // Remove this once we stop supporting legacy Zed Pro
        let is_using_legacy_zed_pro = provider.id() == ZED_CLOUD_PROVIDER_ID
            && self.workspace.upgrade().map_or(false, |workspace| {
                workspace.read(cx).user_store().read(cx).plan()
                    == Some(cloud_llm_client::Plan::V1(cloud_llm_client::PlanV1::ZedPro))
            });

        self.generate_commit_message_task = Some(cx.spawn(async move |this, mut cx| {
             async move {
                let _defer = cx.on_drop(&this, |this, _cx| {
                    this.generate_commit_message_task.take();
                });

                if let Some(task) = cx.update(|cx| {
                    if !provider.is_authenticated(cx) {
                        Some(provider.authenticate(cx))
                    } else {
                        None
                    }
                }) {
                    task.await.log_err();
                }

                let mut diff_text = match diff.await {
                    Ok(result) => match result {
                        Ok(text) => text,
                        Err(e) => {
                            Self::show_commit_message_error(&this, &e, cx);
                            return anyhow::Ok(());
                        }
                    },
                    Err(e) => {
                        Self::show_commit_message_error(&this, &e, cx);
                        return anyhow::Ok(());
                    }
                };

                const MAX_DIFF_BYTES: usize = 20_000;
                diff_text = Self::compress_commit_diff(&diff_text, MAX_DIFF_BYTES);

                let rules_content = Self::load_project_rules(&project, &repo_work_dir, &mut cx).await;

                let prompt = Self::load_commit_message_prompt(is_using_legacy_zed_pro, &mut cx).await;

                let subject = this.update(cx, |this, cx| {
                    this.commit_editor.read(cx).text(cx).lines().next().map(ToOwned::to_owned).unwrap_or_default()
                })?;

                let text_empty = subject.trim().is_empty();

                let rules_section = match &rules_content {
                    Some(rules) => format!(
                        "\n\nThe user has provided the following project rules that you should follow when writing the commit message:\n\
                        <project_rules>\n{rules}\n</project_rules>\n"
                    ),
                    None => String::new(),
                };

                let subject_section = if text_empty {
                    String::new()
                } else {
                    format!("\nHere is the user's subject line:\n{subject}")
                };

                let content = format!(
                    "{prompt}{rules_section}{subject_section}\nHere are the changes in this commit:\n{diff_text}"
                );

                let request = LanguageModelRequest {
                    thread_id: None,
                    prompt_id: None,
                    intent: Some(CompletionIntent::GenerateGitCommitMessage),
                    mode: None,
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![content.into()],
                        cache: false,
            reasoning_details: None,
                    }],
                    tools: Vec::new(),
                    tool_choice: None,
                    stop: Vec::new(),
                    temperature,
                    thinking_allowed: false,
                };

                let stream = model.stream_completion_text(request, cx);
                match stream.await {
                    Ok(mut messages) => {
                        if !text_empty {
                            this.update(cx, |this, cx| {
                                this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                                    let insert_position = buffer.anchor_before(buffer.len());
                                    buffer.edit([(insert_position..insert_position, "\n")], None, cx)
                                });
                            })?;
                        }

                        while let Some(message) = messages.stream.next().await {
                            match message {
                                Ok(text) => {
                                    this.update(cx, |this, cx| {
                                        this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                                            let insert_position = buffer.anchor_before(buffer.len());
                                            buffer.edit([(insert_position..insert_position, text)], None, cx);
                                        });
                                    })?;
                                }
                                Err(e) => {
                                    Self::show_commit_message_error(&this, &e, cx);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        Self::show_commit_message_error(&this, &e, cx);
                    }
                }

                anyhow::Ok(())
            }
            .log_err().await
        }));
    }

    fn get_fetch_options(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<FetchOptions>> {
        let repo = self.active_repository.clone();
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |_, cx| {
            let repo = repo?;
            let remotes = repo
                .update(cx, |repo, _| repo.get_remotes(None, false))
                .await
                .ok()?
                .log_err()?;

            let mut remotes: Vec<_> = remotes.into_iter().map(FetchOptions::Remote).collect();
            if remotes.len() > 1 {
                remotes.push(FetchOptions::All);
            }
            let selection = cx
                .update(|window, cx| {
                    picker_prompt::prompt(
                        "Pick which remote to fetch",
                        remotes.iter().map(|r| r.name()).collect(),
                        workspace,
                        window,
                        cx,
                    )
                })
                .ok()?
                .await?;
            remotes.get(selection).cloned()
        })
    }

    pub(crate) fn fetch(
        &mut self,
        is_fetch_all: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_push_and_pull(cx) {
            return;
        }

        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        telemetry::event!("Git Fetched");
        let askpass = self.askpass_delegate("git fetch", window, cx);
        let this = cx.weak_entity();

        let fetch_options = if is_fetch_all {
            Task::ready(Some(FetchOptions::All))
        } else {
            self.get_fetch_options(window, cx)
        };

        window
            .spawn(cx, async move |cx| {
                let Some(fetch_options) = fetch_options.await else {
                    return Ok(());
                };
                let fetch = repo.update(cx, |repo, cx| {
                    repo.fetch(fetch_options.clone(), askpass, cx)
                });

                let remote_message = fetch.await?;
                this.update(cx, |this, cx| {
                    let action = match fetch_options {
                        FetchOptions::All => RemoteAction::Fetch(None),
                        FetchOptions::Remote(remote) => RemoteAction::Fetch(Some(remote)),
                    };
                    match remote_message {
                        Ok(remote_message) => this.show_remote_output(action, remote_message, cx),
                        Err(e) => {
                            log::error!("Error while fetching {:?}", e);
                            this.show_error_toast(action.name(), e, cx)
                        }
                    }

                    anyhow::Ok(())
                })
                .ok();
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
    }

    pub(crate) fn git_clone(&mut self, repo: String, window: &mut Window, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();

        crate::clone::clone_and_open(
            repo.into(),
            workspace,
            window,
            cx,
            Arc::new(|_workspace: &mut workspace::Workspace, _window, _cx| {}),
        );
    }

    pub(crate) fn git_init(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let worktrees = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();

        let worktree = if worktrees.len() == 1 {
            Task::ready(Some(worktrees.first().unwrap().clone()))
        } else if worktrees.is_empty() {
            let result = window.prompt(
                PromptLevel::Warning,
                "Unable to initialize a git repository",
                Some("Open a directory first"),
                &["Ok"],
                cx,
            );
            cx.background_executor()
                .spawn(async move {
                    result.await.ok();
                })
                .detach();
            return;
        } else {
            let worktree_directories = worktrees
                .iter()
                .map(|worktree| worktree.read(cx).abs_path())
                .map(|worktree_abs_path| {
                    if let Ok(path) = worktree_abs_path.strip_prefix(util::paths::home_dir()) {
                        Path::new("~")
                            .join(path)
                            .to_string_lossy()
                            .to_string()
                            .into()
                    } else {
                        worktree_abs_path.to_string_lossy().into_owned().into()
                    }
                })
                .collect_vec();
            let prompt = picker_prompt::prompt(
                "Where would you like to initialize this git repository?",
                worktree_directories,
                self.workspace.clone(),
                window,
                cx,
            );

            cx.spawn(async move |_, _| prompt.await.map(|ix| worktrees[ix].clone()))
        };

        cx.spawn_in(window, async move |this, cx| {
            let worktree = match worktree.await {
                Some(worktree) => worktree,
                None => {
                    return;
                }
            };

            let Ok(result) = this.update(cx, |this, cx| {
                let fallback_branch_name = GitPanelSettings::get_global(cx)
                    .fallback_branch_name
                    .clone();
                this.project.read(cx).git_init(
                    worktree.read(cx).abs_path(),
                    fallback_branch_name,
                    cx,
                )
            }) else {
                return;
            };

            let result = result.await;

            this.update_in(cx, |this, _, cx| match result {
                Ok(()) => {}
                Err(e) => this.show_error_toast("init", e, cx),
            })
            .ok();
        })
        .detach();
    }

    pub(crate) fn pull(&mut self, rebase: bool, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_push_and_pull(cx) {
            return;
        }
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let Some(branch) = repo.read(cx).branch.as_ref() else {
            return;
        };
        telemetry::event!("Git Pulled");
        let branch = branch.clone();
        let remote = self.get_remote(false, false, window, cx);
        cx.spawn_in(window, async move |this, cx| {
            let remote = match remote.await {
                Ok(Some(remote)) => remote,
                Ok(None) => {
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Failed to get current remote: {}", e);
                    this.update(cx, |this, cx| this.show_error_toast("pull", e, cx))
                        .ok();
                    return Ok(());
                }
            };

            let askpass = this.update_in(cx, |this, window, cx| {
                this.askpass_delegate(format!("git pull {}", remote.name), window, cx)
            })?;

            let branch_name = branch
                .upstream
                .is_none()
                .then(|| branch.name().to_owned().into());

            let pull = repo.update(cx, |repo, cx| {
                repo.pull(branch_name, remote.name.clone(), rebase, askpass, cx)
            });

            let remote_message = pull.await?;

            let action = RemoteAction::Pull(remote);
            this.update(cx, |this, cx| match remote_message {
                Ok(remote_message) => this.show_remote_output(action, remote_message, cx),
                Err(e) => {
                    log::error!("Error while pulling {:?}", e);
                    this.show_error_toast(action.name(), e, cx)
                }
            })
            .ok();

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub(crate) fn push(
        &mut self,
        force_push: bool,
        select_remote: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_push_and_pull(cx) {
            return;
        }
        let Some(repo) = self.active_repository.clone() else {
            return;
        };
        let Some(branch) = repo.read(cx).branch.as_ref() else {
            return;
        };
        telemetry::event!("Git Pushed");
        let branch = branch.clone();

        let options = if force_push {
            Some(PushOptions::Force)
        } else {
            match branch.upstream {
                Some(Upstream {
                    tracking: UpstreamTracking::Gone,
                    ..
                })
                | None => Some(PushOptions::SetUpstream),
                _ => None,
            }
        };
        let remote = self.get_remote(select_remote, true, window, cx);

        cx.spawn_in(window, async move |this, cx| {
            let remote = match remote.await {
                Ok(Some(remote)) => remote,
                Ok(None) => {
                    return Ok(());
                }
                Err(e) => {
                    log::error!("Failed to get current remote: {}", e);
                    this.update(cx, |this, cx| this.show_error_toast("push", e, cx))
                        .ok();
                    return Ok(());
                }
            };

            let askpass_delegate = this.update_in(cx, |this, window, cx| {
                this.askpass_delegate(format!("git push {}", remote.name), window, cx)
            })?;

            let push = repo.update(cx, |repo, cx| {
                repo.push(
                    branch.name().to_owned().into(),
                    branch
                        .upstream
                        .as_ref()
                        .filter(|u| matches!(u.tracking, UpstreamTracking::Tracked(_)))
                        .and_then(|u| u.branch_name())
                        .unwrap_or_else(|| branch.name())
                        .to_owned()
                        .into(),
                    remote.name.clone(),
                    options,
                    askpass_delegate,
                    cx,
                )
            });

            let remote_output = push.await?;

            let action = RemoteAction::Push(branch.name().to_owned().into(), remote);
            this.update(cx, |this, cx| match remote_output {
                Ok(remote_message) => this.show_remote_output(action, remote_message, cx),
                Err(e) => {
                    log::error!("Error while pushing {:?}", e);
                    this.show_error_toast(action.name(), e, cx)
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn create_pull_request(&self, window: &mut Window, cx: &mut Context<Self>) {
        let result = (|| -> anyhow::Result<()> {
            let repo = self
                .active_repository
                .clone()
                .ok_or_else(|| anyhow::anyhow!("No active repository"))?;

            let (branch, remote_origin, remote_upstream) = {
                let repository = repo.read(cx);
                (
                    repository.branch.clone(),
                    repository.remote_origin_url.clone(),
                    repository.remote_upstream_url.clone(),
                )
            };

            let branch = branch.ok_or_else(|| anyhow::anyhow!("No active branch"))?;
            let source_branch = branch
                .upstream
                .as_ref()
                .filter(|upstream| matches!(upstream.tracking, UpstreamTracking::Tracked(_)))
                .and_then(|upstream| upstream.branch_name())
                .ok_or_else(|| anyhow::anyhow!("No remote configured for repository"))?;
            let source_branch = source_branch.to_string();

            let remote_url = branch
                .upstream
                .as_ref()
                .and_then(|upstream| match upstream.remote_name() {
                    Some("upstream") => remote_upstream.as_deref(),
                    Some(_) => remote_origin.as_deref(),
                    None => None,
                })
                .or(remote_origin.as_deref())
                .or(remote_upstream.as_deref())
                .ok_or_else(|| anyhow::anyhow!("No remote configured for repository"))?;
            let remote_url = remote_url.to_string();

            let provider_registry = GitHostingProviderRegistry::global(cx);
            let Some((provider, parsed_remote)) =
                git::parse_git_remote_url(provider_registry, &remote_url)
            else {
                return Err(anyhow::anyhow!("Unsupported remote URL: {}", remote_url));
            };

            let Some(url) = provider.build_create_pull_request_url(&parsed_remote, &source_branch)
            else {
                return Err(anyhow::anyhow!("Unable to construct pull request URL"));
            };

            cx.open_url(url.as_str());
            Ok(())
        })();

        if let Err(err) = result {
            log::error!("Error while creating pull request {:?}", err);
            cx.defer_in(window, |panel, _window, cx| {
                panel.show_error_toast("create pull request", err, cx);
            });
        }
    }

    fn askpass_delegate(
        &self,
        operation: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AskPassDelegate {
        let this = cx.weak_entity();
        let operation = operation.into();
        let window = window.window_handle();
        AskPassDelegate::new(&mut cx.to_async(), move |prompt, tx, cx| {
            window
                .update(cx, |_, window, cx| {
                    this.update(cx, |this, cx| {
                        this.workspace.update(cx, |workspace, cx| {
                            workspace.toggle_modal(window, cx, |window, cx| {
                                AskPassModal::new(operation.clone(), prompt.into(), tx, window, cx)
                            });
                        })
                    })
                })
                .ok();
        })
    }

    fn can_push_and_pull(&self, cx: &App) -> bool {
        !self.project.read(cx).is_via_collab()
    }

    fn get_remote(
        &mut self,
        always_select: bool,
        is_push: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = anyhow::Result<Option<Remote>>> + use<> {
        let repo = self.active_repository.clone();
        let workspace = self.workspace.clone();
        let mut cx = window.to_async(cx);

        async move {
            let repo = repo.context("No active repository")?;
            let current_remotes: Vec<Remote> = repo
                .update(&mut cx, |repo, _| {
                    let current_branch = if always_select {
                        None
                    } else {
                        let current_branch = repo.branch.as_ref().context("No active branch")?;
                        Some(current_branch.name().to_string())
                    };
                    anyhow::Ok(repo.get_remotes(current_branch, is_push))
                })?
                .await??;

            let current_remotes: Vec<_> = current_remotes
                .into_iter()
                .map(|remotes| remotes.name)
                .collect();
            let selection = cx
                .update(|window, cx| {
                    picker_prompt::prompt(
                        "Pick which remote to push to",
                        current_remotes.clone(),
                        workspace,
                        window,
                        cx,
                    )
                })?
                .await;

            Ok(selection.map(|selection| Remote {
                name: current_remotes[selection].clone(),
            }))
        }
    }

    pub fn load_local_committer(&mut self, cx: &Context<Self>) {
        if self.local_committer_task.is_none() {
            self.local_committer_task = Some(cx.spawn(async move |this, cx| {
                let committer = get_git_committer(cx).await;
                this.update(cx, |this, cx| {
                    this.local_committer = Some(committer);
                    cx.notify()
                })
                .ok();
            }));
        }
    }

    fn potential_co_authors(&self, cx: &App) -> Vec<(String, String)> {
        let mut new_co_authors = Vec::new();
        let project = self.project.read(cx);

        let Some(room) = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_call()?.read(cx).room().cloned())
        else {
            return Vec::default();
        };

        let room = room.read(cx);

        for (peer_id, collaborator) in project.collaborators() {
            if collaborator.is_host {
                continue;
            }

            let Some(participant) = room.remote_participant_for_peer_id(*peer_id) else {
                continue;
            };
            if !participant.can_write() {
                continue;
            }
            if let Some(email) = &collaborator.committer_email {
                let name = collaborator
                    .committer_name
                    .clone()
                    .or_else(|| participant.user.name.clone())
                    .unwrap_or_else(|| participant.user.github_login.clone().to_string());
                new_co_authors.push((name.clone(), email.clone()))
            }
        }
        if !project.is_local()
            && !project.is_read_only(cx)
            && let Some(local_committer) = self.local_committer(room, cx)
        {
            new_co_authors.push(local_committer);
        }
        new_co_authors
    }

    fn local_committer(&self, room: &call::Room, cx: &App) -> Option<(String, String)> {
        let user = room.local_participant_user(cx)?;
        let committer = self.local_committer.as_ref()?;
        let email = committer.email.clone()?;
        let name = committer
            .name
            .clone()
            .or_else(|| user.name.clone())
            .unwrap_or_else(|| user.github_login.clone().to_string());
        Some((name, email))
    }

    fn toggle_fill_co_authors(
        &mut self,
        _: &ToggleFillCoAuthors,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.add_coauthors = !self.add_coauthors;
        cx.notify();
    }

    fn toggle_sort_by_path(
        &mut self,
        _: &ToggleSortByPath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let current_setting = GitPanelSettings::get_global(cx).sort_by_path;
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            let fs = workspace.app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings.git_panel.get_or_insert_default().sort_by_path =
                        Some(!current_setting);
                });
            });
        }
    }

    fn toggle_tree_view(&mut self, _: &ToggleTreeView, _: &mut Window, cx: &mut Context<Self>) {
        let current_setting = GitPanelSettings::get_global(cx).tree_view;
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            let fs = workspace.app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings.git_panel.get_or_insert_default().tree_view = Some(!current_setting);
                });
            })
        }
    }

    fn toggle_directory(&mut self, key: &TreeKey, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(state) = self.view_mode.tree_state_mut() {
            let expanded = state.expanded_dirs.entry(key.clone()).or_insert(true);
            *expanded = !*expanded;
            self.update_visible_entries(window, cx);
        } else {
            util::debug_panic!("Attempted to toggle directory in flat Git Panel state");
        }
    }

    fn fill_co_authors(&mut self, message: &mut String, cx: &mut Context<Self>) {
        const CO_AUTHOR_PREFIX: &str = "Co-authored-by: ";

        let existing_text = message.to_ascii_lowercase();
        let lowercase_co_author_prefix = CO_AUTHOR_PREFIX.to_lowercase();
        let mut ends_with_co_authors = false;
        let existing_co_authors = existing_text
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.starts_with(&lowercase_co_author_prefix) {
                    ends_with_co_authors = true;
                    Some(line)
                } else {
                    ends_with_co_authors = false;
                    None
                }
            })
            .collect::<HashSet<_>>();

        let new_co_authors = self
            .potential_co_authors(cx)
            .into_iter()
            .filter(|(_, email)| {
                !existing_co_authors
                    .iter()
                    .any(|existing| existing.contains(email.as_str()))
            })
            .collect::<Vec<_>>();

        if new_co_authors.is_empty() {
            return;
        }

        if !ends_with_co_authors {
            message.push('\n');
        }
        for (name, email) in new_co_authors {
            message.push('\n');
            message.push_str(CO_AUTHOR_PREFIX);
            message.push_str(&name);
            message.push_str(" <");
            message.push_str(&email);
            message.push('>');
        }
        message.push('\n');
    }

    fn schedule_update(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let handle = cx.entity().downgrade();
        self.reopen_commit_buffer(window, cx);
        self.update_visible_entries_task = cx.spawn_in(window, async move |_, cx| {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            if let Some(git_panel) = handle.upgrade() {
                git_panel
                    .update_in(cx, |git_panel, window, cx| {
                        git_panel.update_visible_entries(window, cx);
                    })
                    .ok();
            }
        });
    }

    fn reopen_commit_buffer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repo) = self.active_repository.as_ref() else {
            return;
        };
        let load_buffer = active_repo.update(cx, |active_repo, cx| {
            let project = self.project.read(cx);
            active_repo.open_commit_buffer(
                Some(project.languages().clone()),
                project.buffer_store().clone(),
                cx,
            )
        });

        cx.spawn_in(window, async move |git_panel, cx| {
            let buffer = load_buffer.await?;
            git_panel.update_in(cx, |git_panel, window, cx| {
                if git_panel
                    .commit_editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .as_ref()
                    != Some(&buffer)
                {
                    git_panel.commit_editor = cx.new(|cx| {
                        commit_message_editor(
                            buffer,
                            git_panel.suggest_commit_message(cx).map(SharedString::from),
                            git_panel.project.clone(),
                            true,
                            window,
                            cx,
                        )
                    });
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn update_visible_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let path_style = self.project.read(cx).path_style(cx);
        let bulk_staging = self.bulk_staging.take();
        let last_staged_path_prev_index = bulk_staging
            .as_ref()
            .and_then(|op| self.entry_by_path(&op.anchor));

        self.entries.clear();
        self.entries_indices.clear();
        self.single_staged_entry.take();
        self.single_tracked_entry.take();
        self.conflicted_count = 0;
        self.conflicted_staged_count = 0;
        self.changes_count = 0;
        self.new_count = 0;
        self.tracked_count = 0;
        self.new_staged_count = 0;
        self.tracked_staged_count = 0;
        self.entry_count = 0;
        self.max_width_item_index = None;

        let sort_by_path = GitPanelSettings::get_global(cx).sort_by_path;
        let is_tree_view = matches!(self.view_mode, GitPanelViewMode::Tree(_));
        let group_by_status = is_tree_view || !sort_by_path;

        let mut changed_entries = Vec::new();
        let mut new_entries = Vec::new();
        let mut conflict_entries = Vec::new();
        let mut single_staged_entry = None;
        let mut staged_count = 0;
        let mut seen_directories = HashSet::default();
        let mut max_width_estimate = 0usize;
        let mut max_width_item_index = None;

        let Some(repo) = self.active_repository.as_ref() else {
            // Just clear entries if no repository is active.
            cx.notify();
            return;
        };

        let repo = repo.read(cx);

        self.stash_entries = repo.cached_stash();

        for entry in repo.cached_status() {
            self.changes_count += 1;
            let is_conflict = repo.had_conflict_on_last_merge_head_change(&entry.repo_path);
            let is_new = entry.status.is_created();
            let staging = entry.status.staging();

            if let Some(pending) = repo.pending_ops_for_path(&entry.repo_path)
                && pending
                    .ops
                    .iter()
                    .any(|op| op.git_status == pending_op::GitStatus::Reverted && op.finished())
            {
                continue;
            }

            let entry = GitStatusEntry {
                repo_path: entry.repo_path.clone(),
                status: entry.status,
                staging,
            };

            if staging.has_staged() {
                staged_count += 1;
                single_staged_entry = Some(entry.clone());
            }

            if group_by_status && is_conflict {
                conflict_entries.push(entry);
            } else if group_by_status && is_new {
                new_entries.push(entry);
            } else {
                changed_entries.push(entry);
            }
        }

        if conflict_entries.is_empty() {
            if staged_count == 1
                && let Some(entry) = single_staged_entry.as_ref()
            {
                if let Some(ops) = repo.pending_ops_for_path(&entry.repo_path) {
                    if ops.staged() {
                        self.single_staged_entry = single_staged_entry;
                    }
                } else {
                    self.single_staged_entry = single_staged_entry;
                }
            } else if repo.pending_ops_summary().item_summary.staging_count == 1
                && let Some(ops) = repo.pending_ops().find(|ops| ops.staging())
            {
                self.single_staged_entry =
                    repo.status_for_path(&ops.repo_path)
                        .map(|status| GitStatusEntry {
                            repo_path: ops.repo_path.clone(),
                            status: status.status,
                            staging: StageStatus::Staged,
                        });
            }
        }

        if conflict_entries.is_empty() && changed_entries.len() == 1 {
            self.single_tracked_entry = changed_entries.first().cloned();
        }

        let mut push_entry =
            |this: &mut Self,
             entry: GitListEntry,
             is_visible: bool,
             logical_indices: Option<&mut Vec<usize>>| {
                if let Some(estimate) =
                    this.width_estimate_for_list_entry(is_tree_view, &entry, path_style)
                {
                    if estimate > max_width_estimate {
                        max_width_estimate = estimate;
                        max_width_item_index = Some(this.entries.len());
                    }
                }

                if let Some(repo_path) = entry.status_entry().map(|status| status.repo_path.clone())
                {
                    this.entries_indices.insert(repo_path, this.entries.len());
                }

                if let (Some(indices), true) = (logical_indices, is_visible) {
                    indices.push(this.entries.len());
                }

                this.entries.push(entry);
            };

        macro_rules! take_section_entries {
            () => {
                [
                    (Section::Conflict, std::mem::take(&mut conflict_entries)),
                    (Section::Tracked, std::mem::take(&mut changed_entries)),
                    (Section::New, std::mem::take(&mut new_entries)),
                ]
            };
        }

        match &mut self.view_mode {
            GitPanelViewMode::Tree(tree_state) => {
                tree_state.logical_indices.clear();
                tree_state.directory_descendants.clear();

                // This is just to get around the borrow checker
                // because push_entry mutably borrows self
                let mut tree_state = std::mem::take(tree_state);

                for (section, entries) in take_section_entries!() {
                    if entries.is_empty() {
                        continue;
                    }

                    push_entry(
                        self,
                        GitListEntry::Header(GitHeaderEntry { header: section }),
                        true,
                        Some(&mut tree_state.logical_indices),
                    );

                    for (entry, is_visible) in
                        tree_state.build_tree_entries(section, entries, &mut seen_directories)
                    {
                        push_entry(
                            self,
                            entry,
                            is_visible,
                            Some(&mut tree_state.logical_indices),
                        );
                    }
                }

                tree_state
                    .expanded_dirs
                    .retain(|key, _| seen_directories.contains(key));
                self.view_mode = GitPanelViewMode::Tree(tree_state);
            }
            GitPanelViewMode::Flat => {
                for (section, entries) in take_section_entries!() {
                    if entries.is_empty() {
                        continue;
                    }

                    if section != Section::Tracked || !sort_by_path {
                        push_entry(
                            self,
                            GitListEntry::Header(GitHeaderEntry { header: section }),
                            true,
                            None,
                        );
                    }

                    for entry in entries {
                        push_entry(self, GitListEntry::Status(entry), true, None);
                    }
                }
            }
        }

        self.max_width_item_index = max_width_item_index;

        self.update_counts(repo);

        let bulk_staging_anchor_new_index = bulk_staging
            .as_ref()
            .filter(|op| op.repo_id == repo.id)
            .and_then(|op| self.entry_by_path(&op.anchor));
        if bulk_staging_anchor_new_index == last_staged_path_prev_index
            && let Some(index) = bulk_staging_anchor_new_index
            && let Some(entry) = self.entries.get(index)
            && let Some(entry) = entry.status_entry()
            && GitPanel::stage_status_for_entry(entry, &repo)
                .as_bool()
                .unwrap_or(false)
        {
            self.bulk_staging = bulk_staging;
        }

        self.select_first_entry_if_none(window, cx);

        let suggested_commit_message = self.suggest_commit_message(cx);
        let placeholder_text = suggested_commit_message.unwrap_or("Enter commit message".into());

        self.commit_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text(&placeholder_text, window, cx)
        });

        cx.notify();
    }

    fn header_state(&self, header_type: Section) -> ToggleState {
        let (staged_count, count) = match header_type {
            Section::New => (self.new_staged_count, self.new_count),
            Section::Tracked => (self.tracked_staged_count, self.tracked_count),
            Section::Conflict => (self.conflicted_staged_count, self.conflicted_count),
        };
        if staged_count == 0 {
            ToggleState::Unselected
        } else if count == staged_count {
            ToggleState::Selected
        } else {
            ToggleState::Indeterminate
        }
    }

    fn update_counts(&mut self, repo: &Repository) {
        self.show_placeholders = false;
        self.conflicted_count = 0;
        self.conflicted_staged_count = 0;
        self.new_count = 0;
        self.tracked_count = 0;
        self.new_staged_count = 0;
        self.tracked_staged_count = 0;
        self.entry_count = 0;

        for status_entry in self.entries.iter().filter_map(|entry| entry.status_entry()) {
            self.entry_count += 1;
            let is_staging_or_staged = GitPanel::stage_status_for_entry(status_entry, repo)
                .as_bool()
                .unwrap_or(true);

            if repo.had_conflict_on_last_merge_head_change(&status_entry.repo_path) {
                self.conflicted_count += 1;
                if is_staging_or_staged {
                    self.conflicted_staged_count += 1;
                }
            } else if status_entry.status.is_created() {
                self.new_count += 1;
                if is_staging_or_staged {
                    self.new_staged_count += 1;
                }
            } else {
                self.tracked_count += 1;
                if is_staging_or_staged {
                    self.tracked_staged_count += 1;
                }
            }
        }
    }

    pub(crate) fn has_staged_changes(&self) -> bool {
        self.tracked_staged_count > 0
            || self.new_staged_count > 0
            || self.conflicted_staged_count > 0
    }

    pub(crate) fn has_unstaged_changes(&self) -> bool {
        self.tracked_count > self.tracked_staged_count
            || self.new_count > self.new_staged_count
            || self.conflicted_count > self.conflicted_staged_count
    }

    fn has_tracked_changes(&self) -> bool {
        self.tracked_count > 0
    }

    pub fn has_unstaged_conflicts(&self) -> bool {
        self.conflicted_count > 0 && self.conflicted_count != self.conflicted_staged_count
    }

    fn show_error_toast(&self, action: impl Into<SharedString>, e: anyhow::Error, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        show_error_toast(workspace, action, e, cx)
    }

    fn show_commit_message_error<E>(weak_this: &WeakEntity<Self>, err: &E, cx: &mut AsyncApp)
    where
        E: std::fmt::Debug + std::fmt::Display,
    {
        if let Ok(Some(workspace)) = weak_this.update(cx, |this, _cx| this.workspace.upgrade()) {
            let _ = workspace.update(cx, |workspace, cx| {
                struct CommitMessageError;
                let notification_id = NotificationId::unique::<CommitMessageError>();
                workspace.show_notification(notification_id, cx, |cx| {
                    cx.new(|cx| {
                        ErrorMessagePrompt::new(
                            format!("Failed to generate commit message: {err}"),
                            cx,
                        )
                    })
                });
            });
        }
    }

    fn show_remote_output(
        &mut self,
        action: RemoteAction,
        info: RemoteCommandOutput,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        workspace.update(cx, |workspace, cx| {
            let SuccessMessage { message, style } = remote_output::format_output(&action, info);
            let workspace_weak = cx.weak_entity();
            let operation = action.name();

            let status_toast = StatusToast::new(message, cx, move |this, _cx| {
                use remote_output::SuccessStyle::*;
                match style {
                    Toast => this.icon(ToastIcon::new(IconName::GitBranchAlt).color(Color::Muted)),
                    ToastWithLog { output } => this
                        .icon(ToastIcon::new(IconName::GitBranchAlt).color(Color::Muted))
                        .action("View Log", move |window, cx| {
                            let output = output.clone();
                            let output =
                                format!("stdout:\n{}\nstderr:\n{}", output.stdout, output.stderr);
                            workspace_weak
                                .update(cx, move |workspace, cx| {
                                    open_output(operation, workspace, &output, window, cx)
                                })
                                .ok();
                        }),
                    PushPrLink { text, link } => this
                        .icon(ToastIcon::new(IconName::GitBranchAlt).color(Color::Muted))
                        .action(text, move |_, cx| cx.open_url(&link)),
                }
                .dismiss_button(true)
            });
            workspace.toggle_status_toast(status_toast, cx)
        });
    }

    pub fn can_commit(&self) -> bool {
        (self.has_staged_changes() || self.has_tracked_changes()) && !self.has_unstaged_conflicts()
    }

    pub fn can_stage_all(&self) -> bool {
        self.has_unstaged_changes()
    }

    pub fn can_unstage_all(&self) -> bool {
        self.has_staged_changes()
    }

    fn status_width_estimate(
        tree_view: bool,
        entry: &GitStatusEntry,
        path_style: PathStyle,
        depth: usize,
    ) -> usize {
        if tree_view {
            Self::item_width_estimate(0, entry.display_name(path_style).len(), depth)
        } else {
            Self::item_width_estimate(
                entry.parent_dir(path_style).map(|s| s.len()).unwrap_or(0),
                entry.display_name(path_style).len(),
                0,
            )
        }
    }

    fn width_estimate_for_list_entry(
        &self,
        tree_view: bool,
        entry: &GitListEntry,
        path_style: PathStyle,
    ) -> Option<usize> {
        match entry {
            GitListEntry::Status(status) => Some(Self::status_width_estimate(
                tree_view, status, path_style, 0,
            )),
            GitListEntry::TreeStatus(status) => Some(Self::status_width_estimate(
                tree_view,
                &status.entry,
                path_style,
                status.depth,
            )),
            GitListEntry::Directory(dir) => {
                Some(Self::item_width_estimate(0, dir.name.len(), dir.depth))
            }
            GitListEntry::Header(_) => None,
        }
    }

    fn item_width_estimate(path: usize, file_name: usize, depth: usize) -> usize {
        path + file_name + depth * 2
    }

    fn render_overflow_menu(&self, id: impl Into<ElementId>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let has_tracked_changes = self.has_tracked_changes();
        let has_staged_changes = self.has_staged_changes();
        let has_unstaged_changes = self.has_unstaged_changes();
        let has_new_changes = self.new_count > 0;
        let has_stash_items = self.stash_entries.entries.len() > 0;

        PopoverMenu::new(id.into())
            .trigger(
                IconButton::new("overflow-menu-trigger", IconName::Ellipsis)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
            )
            .menu(move |window, cx| {
                Some(git_panel_context_menu(
                    focus_handle.clone(),
                    GitMenuState {
                        has_tracked_changes,
                        has_staged_changes,
                        has_unstaged_changes,
                        has_new_changes,
                        sort_by_path: GitPanelSettings::get_global(cx).sort_by_path,
                        has_stash_items,
                        tree_view: GitPanelSettings::get_global(cx).tree_view,
                    },
                    window,
                    cx,
                ))
            })
            .anchor(Corner::TopRight)
    }

    pub(crate) fn render_generate_commit_message_button(
        &self,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        if !agent_settings::AgentSettings::get_global(cx).enabled(cx)
            || LanguageModelRegistry::read_global(cx)
                .commit_message_model()
                .is_none()
        {
            return None;
        }

        if self.generate_commit_message_task.is_some() {
            return Some(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::XSmall)
                            .color(Color::Info)
                            .with_rotate_animation(2),
                    )
                    .child(
                        Label::new("Generating Commit...")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            );
        }

        let can_commit = self.can_commit();
        let editor_focus_handle = self.commit_editor.focus_handle(cx);
        Some(
            IconButton::new("generate-commit-message", IconName::AiEdit)
                .shape(ui::IconButtonShape::Square)
                .icon_color(Color::Muted)
                .tooltip(move |_window, cx| {
                    if can_commit {
                        Tooltip::for_action_in(
                            "Generate Commit Message",
                            &git::GenerateCommitMessage,
                            &editor_focus_handle,
                            cx,
                        )
                    } else {
                        Tooltip::simple("No changes to commit", cx)
                    }
                })
                .disabled(!can_commit)
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    this.generate_commit_message(cx);
                }))
                .into_any_element(),
        )
    }

    pub(crate) fn render_co_authors(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let potential_co_authors = self.potential_co_authors(cx);

        let (tooltip_label, icon) = if self.add_coauthors {
            ("Remove co-authored-by", IconName::Person)
        } else {
            ("Add co-authored-by", IconName::UserCheck)
        };

        if potential_co_authors.is_empty() {
            None
        } else {
            Some(
                IconButton::new("co-authors", icon)
                    .shape(ui::IconButtonShape::Square)
                    .icon_color(Color::Disabled)
                    .selected_icon_color(Color::Selected)
                    .toggle_state(self.add_coauthors)
                    .tooltip(move |_, cx| {
                        let title = format!(
                            "{}:{}{}",
                            tooltip_label,
                            if potential_co_authors.len() == 1 {
                                ""
                            } else {
                                "\n"
                            },
                            potential_co_authors
                                .iter()
                                .map(|(name, email)| format!(" {} <{}>", name, email))
                                .join("\n")
                        );
                        Tooltip::simple(title, cx)
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.add_coauthors = !this.add_coauthors;
                        cx.notify();
                    }))
                    .into_any_element(),
            )
        }
    }

    fn render_git_commit_menu(
        &self,
        id: impl Into<ElementId>,
        keybinding_target: Option<FocusHandle>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        PopoverMenu::new(id.into())
            .trigger(
                ui::ButtonLike::new_rounded_right("commit-split-button-right")
                    .layer(ui::ElevationIndex::ModalSurface)
                    .size(ButtonSize::None)
                    .child(
                        h_flex()
                            .px_1()
                            .h_full()
                            .justify_center()
                            .border_l_1()
                            .border_color(cx.theme().colors().border)
                            .child(Icon::new(IconName::ChevronDown).size(IconSize::XSmall)),
                    ),
            )
            .menu({
                let git_panel = cx.entity();
                let has_previous_commit = self.head_commit(cx).is_some();
                let amend = self.amend_pending();
                let signoff = self.signoff_enabled;

                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |context_menu, _, _| {
                        context_menu
                            .when_some(keybinding_target.clone(), |el, keybinding_target| {
                                el.context(keybinding_target)
                            })
                            .when(has_previous_commit, |this| {
                                this.toggleable_entry(
                                    "Amend",
                                    amend,
                                    IconPosition::Start,
                                    Some(Box::new(Amend)),
                                    {
                                        let git_panel = git_panel.downgrade();
                                        move |_, cx| {
                                            git_panel
                                                .update(cx, |git_panel, cx| {
                                                    git_panel.toggle_amend_pending(cx);
                                                })
                                                .ok();
                                        }
                                    },
                                )
                            })
                            .toggleable_entry(
                                "Signoff",
                                signoff,
                                IconPosition::Start,
                                Some(Box::new(Signoff)),
                                move |window, cx| window.dispatch_action(Box::new(Signoff), cx),
                            )
                    }))
                }
            })
            .anchor(Corner::TopRight)
    }

    pub fn configure_commit_button(&self, cx: &mut Context<Self>) -> (bool, &'static str) {
        if self.has_unstaged_conflicts() {
            (false, "You must resolve conflicts before committing")
        } else if !self.has_staged_changes() && !self.has_tracked_changes() && !self.amend_pending {
            (false, "No changes to commit")
        } else if self.pending_commit.is_some() {
            (false, "Commit in progress")
        } else if !self.has_commit_message(cx) {
            (false, "No commit message")
        } else if !self.has_write_access(cx) {
            (false, "You do not have write access to this project")
        } else {
            (true, self.commit_button_title())
        }
    }

    pub fn commit_button_title(&self) -> &'static str {
        if self.amend_pending {
            if self.has_staged_changes() {
                "Amend"
            } else if self.has_tracked_changes() {
                "Amend Tracked"
            } else {
                "Amend"
            }
        } else if self.has_staged_changes() {
            "Commit"
        } else {
            "Commit Tracked"
        }
    }

    fn expand_commit_editor(
        &mut self,
        _: &git::ExpandCommitEditor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        window.defer(cx, move |window, cx| {
            workspace
                .update(cx, |workspace, cx| {
                    CommitModal::toggle(workspace, None, window, cx)
                })
                .ok();
        })
    }

    fn render_panel_header(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        self.active_repository.as_ref()?;

        let (text, action, stage, tooltip) =
            if self.total_staged_count() == self.entry_count && self.entry_count > 0 {
                ("Unstage All", UnstageAll.boxed_clone(), false, "git reset")
            } else {
                ("Stage All", StageAll.boxed_clone(), true, "git add --all")
            };

        let change_string = match self.changes_count {
            0 => "No Changes".to_string(),
            1 => "1 Change".to_string(),
            count => format!("{} Changes", count),
        };

        Some(
            self.panel_header_container(window, cx)
                .px_2()
                .justify_between()
                .child(
                    panel_button(change_string)
                        .color(Color::Muted)
                        .tooltip(Tooltip::for_action_title_in(
                            "Open Diff",
                            &Diff,
                            &self.focus_handle,
                        ))
                        .on_click(|_, _, cx| {
                            cx.defer(|cx| {
                                cx.dispatch_action(&Diff);
                            })
                        }),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(self.render_overflow_menu("overflow_menu"))
                        .child(
                            panel_filled_button(text)
                                .tooltip(Tooltip::for_action_title_in(
                                    tooltip,
                                    action.as_ref(),
                                    &self.focus_handle,
                                ))
                                .disabled(self.entry_count == 0)
                                .on_click({
                                    let git_panel = cx.weak_entity();
                                    move |_, _, cx| {
                                        git_panel
                                            .update(cx, |git_panel, cx| {
                                                git_panel.change_all_files_stage(stage, cx);
                                            })
                                            .ok();
                                    }
                                }),
                        ),
                ),
        )
    }

    pub(crate) fn render_remote_button(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let branch = self.active_repository.as_ref()?.read(cx).branch.clone();
        if !self.can_push_and_pull(cx) {
            return None;
        }
        Some(
            h_flex()
                .gap_1()
                .flex_shrink_0()
                .when_some(branch, |this, branch| {
                    let focus_handle = Some(self.focus_handle(cx));

                    this.children(render_remote_button(
                        "remote-button",
                        &branch,
                        focus_handle,
                        true,
                    ))
                })
                .into_any_element(),
        )
    }

    pub fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let active_repository = self.active_repository.clone()?;
        let panel_editor_style = panel_editor_style(true, window, cx);
        let enable_coauthors = self.render_co_authors(cx);

        let editor_focus_handle = self.commit_editor.focus_handle(cx);
        let expand_tooltip_focus_handle = editor_focus_handle;

        let branch = active_repository.read(cx).branch.clone();
        let head_commit = active_repository.read(cx).head_commit.clone();

        let footer_size = px(32.);
        let gap = px(9.0);
        let max_height = panel_editor_style
            .text
            .line_height_in_pixels(window.rem_size())
            * MAX_PANEL_EDITOR_LINES
            + gap;

        let git_panel = cx.entity();
        let display_name = SharedString::from(Arc::from(
            active_repository
                .read(cx)
                .display_name()
                .trim_end_matches("/"),
        ));
        let editor_is_long = self.commit_editor.update(cx, |editor, cx| {
            editor.max_point(cx).row().0 >= MAX_PANEL_EDITOR_LINES as u32
        });

        let footer = v_flex()
            .child(PanelRepoFooter::new(
                display_name,
                branch,
                head_commit,
                Some(git_panel),
            ))
            .child(
                panel_editor_container(window, cx)
                    .id("commit-editor-container")
                    .relative()
                    .w_full()
                    .h(max_height + footer_size)
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .cursor_text()
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        window.focus(&this.commit_editor.focus_handle(cx), cx);
                    }))
                    .child(
                        h_flex()
                            .id("commit-footer")
                            .border_t_1()
                            .when(editor_is_long, |el| {
                                el.border_color(cx.theme().colors().border_variant)
                            })
                            .absolute()
                            .bottom_0()
                            .left_0()
                            .w_full()
                            .px_2()
                            .h(footer_size)
                            .flex_none()
                            .justify_between()
                            .child(
                                self.render_generate_commit_message_button(cx)
                                    .unwrap_or_else(|| div().into_any_element()),
                            )
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .children(enable_coauthors)
                                    .child(self.render_commit_button(cx)),
                            ),
                    )
                    .child(
                        div()
                            .pr_2p5()
                            .on_action(|&editor::actions::MoveUp, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_action(|&editor::actions::MoveDown, _, cx| {
                                cx.stop_propagation();
                            })
                            .child(EditorElement::new(&self.commit_editor, panel_editor_style)),
                    )
                    .child(
                        h_flex()
                            .absolute()
                            .top_2()
                            .right_2()
                            .opacity(0.5)
                            .hover(|this| this.opacity(1.0))
                            .child(
                                panel_icon_button("expand-commit-editor", IconName::Maximize)
                                    .icon_size(IconSize::Small)
                                    .size(ui::ButtonSize::Default)
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            "Open Commit Modal",
                                            &git::ExpandCommitEditor,
                                            &expand_tooltip_focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener({
                                        move |_, _, window, cx| {
                                            window.dispatch_action(
                                                git::ExpandCommitEditor.boxed_clone(),
                                                cx,
                                            )
                                        }
                                    })),
                            ),
                    ),
            );

        Some(footer)
    }

    fn render_commit_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (can_commit, tooltip) = self.configure_commit_button(cx);
        let title = self.commit_button_title();
        let commit_tooltip_focus_handle = self.commit_editor.focus_handle(cx);
        let amend = self.amend_pending();
        let signoff = self.signoff_enabled;

        let label_color = if self.pending_commit.is_some() {
            Color::Disabled
        } else {
            Color::Default
        };

        div()
            .id("commit-wrapper")
            .on_hover(cx.listener(move |this, hovered, _, cx| {
                this.show_placeholders =
                    *hovered && !this.has_staged_changes() && !this.has_unstaged_conflicts();
                cx.notify()
            }))
            .child(SplitButton::new(
                ButtonLike::new_rounded_left(ElementId::Name(
                    format!("split-button-left-{}", title).into(),
                ))
                .layer(ElevationIndex::ModalSurface)
                .size(ButtonSize::Compact)
                .child(
                    Label::new(title)
                        .size(LabelSize::Small)
                        .color(label_color)
                        .mr_0p5(),
                )
                .on_click({
                    let git_panel = cx.weak_entity();
                    move |_, window, cx| {
                        telemetry::event!("Git Committed", source = "Git Panel");
                        git_panel
                            .update(cx, |git_panel, cx| {
                                git_panel.commit_changes(
                                    CommitOptions { amend, signoff },
                                    window,
                                    cx,
                                );
                            })
                            .ok();
                    }
                })
                .disabled(!can_commit || self.modal_open)
                .tooltip({
                    let handle = commit_tooltip_focus_handle.clone();
                    move |_window, cx| {
                        if can_commit {
                            Tooltip::with_meta_in(
                                tooltip,
                                Some(if amend { &git::Amend } else { &git::Commit }),
                                format!(
                                    "git commit{}{}",
                                    if amend { " --amend" } else { "" },
                                    if signoff { " --signoff" } else { "" }
                                ),
                                &handle.clone(),
                                cx,
                            )
                        } else {
                            Tooltip::simple(tooltip, cx)
                        }
                    }
                }),
                self.render_git_commit_menu(
                    ElementId::Name(format!("split-button-right-{}", title).into()),
                    Some(commit_tooltip_focus_handle),
                    cx,
                )
                .into_any_element(),
            ))
    }

    fn render_pending_amend(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .py_1p5()
            .px_2()
            .gap_1p5()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border.opacity(0.8))
            .child(
                div()
                    .flex_grow()
                    .overflow_hidden()
                    .max_w(relative(0.85))
                    .child(
                        Label::new("This will update your most recent commit.")
                            .size(LabelSize::Small)
                            .truncate(),
                    ),
            )
            .child(
                panel_button("Cancel")
                    .size(ButtonSize::Default)
                    .on_click(cx.listener(|this, _, _, cx| this.set_amend_pending(false, cx))),
            )
    }

    fn render_previous_commit(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let active_repository = self.active_repository.as_ref()?;
        let branch = active_repository.read(cx).branch.as_ref()?;
        let commit = branch.most_recent_commit.as_ref()?.clone();
        let workspace = self.workspace.clone();
        let this = cx.entity();

        Some(
            h_flex()
                .p_1p5()
                .gap_1p5()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border.opacity(0.8))
                .child(
                    div()
                        .id("commit-msg-hover")
                        .px_1()
                        .cursor_pointer()
                        .line_clamp(1)
                        .rounded_sm()
                        .hover(|s| s.bg(cx.theme().colors().element_hover))
                        .child(
                            Label::new(commit.subject.clone())
                                .size(LabelSize::Small)
                                .truncate(),
                        )
                        .on_click({
                            let commit = commit.clone();
                            let repo = active_repository.downgrade();
                            move |_, window, cx| {
                                CommitView::open(
                                    commit.sha.to_string(),
                                    repo.clone(),
                                    workspace.clone(),
                                    None,
                                    None,
                                    window,
                                    cx,
                                );
                            }
                        })
                        .hoverable_tooltip({
                            let repo = active_repository.clone();
                            move |window, cx| {
                                GitPanelMessageTooltip::new(
                                    this.clone(),
                                    commit.sha.clone(),
                                    repo.clone(),
                                    window,
                                    cx,
                                )
                                .into()
                            }
                        }),
                )
                .when(commit.has_parent, |this| {
                    let has_unstaged = self.has_unstaged_changes();
                    this.pr_2().child(
                        panel_icon_button("undo", IconName::Undo)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Muted)
                            .tooltip(move |_window, cx| {
                                Tooltip::with_meta(
                                    "Uncommit",
                                    Some(&git::Uncommit),
                                    if has_unstaged {
                                        "git reset HEAD^ --soft"
                                    } else {
                                        "git reset HEAD^"
                                    },
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|this, _, window, cx| this.uncommit(window, cx))),
                    )
                }),
        )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().h_full().flex_grow().justify_center().child(
            v_flex()
                .gap_2()
                .child(h_flex().w_full().justify_around().child(
                    if self.active_repository.is_some() {
                        "No changes to commit"
                    } else {
                        "No Git repositories"
                    },
                ))
                .children({
                    let worktree_count = self.project.read(cx).visible_worktrees(cx).count();
                    (worktree_count > 0 && self.active_repository.is_none()).then(|| {
                        h_flex().w_full().justify_around().child(
                            panel_filled_button("Initialize Repository")
                                .tooltip(Tooltip::for_action_title_in(
                                    "git init",
                                    &git::Init,
                                    &self.focus_handle,
                                ))
                                .on_click(move |_, _, cx| {
                                    cx.defer(move |cx| {
                                        cx.dispatch_action(&git::Init);
                                    })
                                }),
                        )
                    })
                })
                .text_ui_sm(cx)
                .mx_auto()
                .text_color(Color::Placeholder.color(cx)),
        )
    }

    fn render_buffer_header_controls(
        &self,
        entity: &Entity<Self>,
        file: &Arc<dyn File>,
        _: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let repo = self.active_repository.as_ref()?.read(cx);
        let project_path = (file.worktree_id(cx), file.path().clone()).into();
        let repo_path = repo.project_path_to_repo_path(&project_path, cx)?;
        let ix = self.entry_by_path(&repo_path)?;
        let entry = self.entries.get(ix)?;

        let is_staging_or_staged = repo
            .pending_ops_for_path(&repo_path)
            .map(|ops| ops.staging() || ops.staged())
            .or_else(|| {
                repo.status_for_path(&repo_path)
                    .and_then(|status| status.status.staging().as_bool())
            })
            .or_else(|| {
                entry
                    .status_entry()
                    .and_then(|entry| entry.staging.as_bool())
            });

        let checkbox = Checkbox::new("stage-file", is_staging_or_staged.into())
            .disabled(!self.has_write_access(cx))
            .fill()
            .elevation(ElevationIndex::Surface)
            .on_click({
                let entry = entry.clone();
                let git_panel = entity.downgrade();
                move |_, window, cx| {
                    git_panel
                        .update(cx, |this, cx| {
                            this.toggle_staged_for_entry(&entry, window, cx);
                            cx.stop_propagation();
                        })
                        .ok();
                }
            });
        Some(
            h_flex()
                .id("start-slot")
                .text_lg()
                .child(checkbox)
                .on_mouse_down(MouseButton::Left, |_, _, cx| {
                    // prevent the list item active state triggering when toggling checkbox
                    cx.stop_propagation();
                })
                .into_any_element(),
        )
    }

    fn render_entries(
        &self,
        has_write_access: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let (is_tree_view, entry_count) = match &self.view_mode {
            GitPanelViewMode::Tree(state) => (true, state.logical_indices.len()),
            GitPanelViewMode::Flat => (false, self.entries.len()),
        };

        v_flex()
            .flex_1()
            .size_full()
            .overflow_hidden()
            .relative()
            .child(
                h_flex()
                    .flex_1()
                    .size_full()
                    .relative()
                    .overflow_hidden()
                    .child(
                        uniform_list(
                            "entries",
                            entry_count,
                            cx.processor(move |this, range: Range<usize>, window, cx| {
                                let mut items = Vec::with_capacity(range.end - range.start);

                                for ix in range.into_iter().map(|ix| match &this.view_mode {
                                    GitPanelViewMode::Tree(state) => state.logical_indices[ix],
                                    GitPanelViewMode::Flat => ix,
                                }) {
                                    match &this.entries.get(ix) {
                                        Some(GitListEntry::Status(entry)) => {
                                            items.push(this.render_status_entry(
                                                ix,
                                                entry,
                                                0,
                                                has_write_access,
                                                window,
                                                cx,
                                            ));
                                        }
                                        Some(GitListEntry::TreeStatus(entry)) => {
                                            items.push(this.render_status_entry(
                                                ix,
                                                &entry.entry,
                                                entry.depth,
                                                has_write_access,
                                                window,
                                                cx,
                                            ));
                                        }
                                        Some(GitListEntry::Directory(entry)) => {
                                            items.push(this.render_directory_entry(
                                                ix,
                                                entry,
                                                has_write_access,
                                                window,
                                                cx,
                                            ));
                                        }
                                        Some(GitListEntry::Header(header)) => {
                                            items.push(this.render_list_header(
                                                ix,
                                                header,
                                                has_write_access,
                                                window,
                                                cx,
                                            ));
                                        }
                                        None => {}
                                    }
                                }

                                items
                            }),
                        )
                        .when(is_tree_view, |list| {
                            let indent_size = px(TREE_INDENT);
                            list.with_decoration(
                                ui::indent_guides(indent_size, IndentGuideColors::panel(cx))
                                    .with_compute_indents_fn(
                                        cx.entity(),
                                        |this, range, _window, _cx| {
                                            range
                                                .map(|ix| match this.entries.get(ix) {
                                                    Some(GitListEntry::Directory(dir)) => dir.depth,
                                                    Some(GitListEntry::TreeStatus(status)) => {
                                                        status.depth
                                                    }
                                                    _ => 0,
                                                })
                                                .collect()
                                        },
                                    )
                                    .with_render_fn(cx.entity(), |_, params, _, _| {
                                        // Magic number to align the tree item is 3 here
                                        // because we're using 12px as the left-side padding
                                        // and 3 makes the alignment work with the bounding box of the icon
                                        let left_offset = px(TREE_INDENT + 3_f32);
                                        let indent_size = params.indent_size;
                                        let item_height = params.item_height;

                                        params
                                            .indent_guides
                                            .into_iter()
                                            .map(|layout| {
                                                let bounds = Bounds::new(
                                                    point(
                                                        layout.offset.x * indent_size + left_offset,
                                                        layout.offset.y * item_height,
                                                    ),
                                                    size(px(1.), layout.length * item_height),
                                                );
                                                RenderedIndentGuide {
                                                    bounds,
                                                    layout,
                                                    is_active: false,
                                                    hitbox: None,
                                                }
                                            })
                                            .collect()
                                    }),
                            )
                        })
                        .size_full()
                        .flex_grow()
                        .with_width_from_item(self.max_width_item_index)
                        .track_scroll(&self.scroll_handle),
                    )
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                            this.deploy_panel_context_menu(event.position, window, cx)
                        }),
                    )
                    .custom_scrollbars(
                        Scrollbars::for_settings::<GitPanelSettings>()
                            .tracked_scroll_handle(&self.scroll_handle)
                            .with_track_along(
                                ScrollAxes::Horizontal,
                                cx.theme().colors().panel_background,
                            ),
                        window,
                        cx,
                    ),
            )
    }

    fn entry_label(&self, label: impl Into<SharedString>, color: Color) -> Label {
        Label::new(label.into()).color(color)
    }

    fn list_item_height(&self) -> Rems {
        rems(1.75)
    }

    fn render_list_header(
        &self,
        ix: usize,
        header: &GitHeaderEntry,
        _: bool,
        _: &Window,
        _: &Context<Self>,
    ) -> AnyElement {
        let id: ElementId = ElementId::Name(format!("header_{}", ix).into());

        h_flex()
            .id(id)
            .h(self.list_item_height())
            .w_full()
            .items_end()
            .px_3()
            .pb_1()
            .child(
                Label::new(header.title())
                    .color(Color::Muted)
                    .size(LabelSize::Small)
                    .line_height_style(LineHeightStyle::UiLabel)
                    .single_line(),
            )
            .into_any_element()
    }

    pub fn load_commit_details(
        &self,
        sha: String,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<CommitDetails>> {
        let Some(repo) = self.active_repository.clone() else {
            return Task::ready(Err(anyhow::anyhow!("no active repo")));
        };
        repo.update(cx, |repo, cx| {
            let show = repo.show(sha);
            cx.spawn(async move |_, _| show.await?)
        })
    }

    fn deploy_entry_context_menu(
        &mut self,
        position: Point<Pixels>,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.entries.get(ix).and_then(|e| e.status_entry()) else {
            return;
        };
        let stage_title = if entry.status.staging().is_fully_staged() {
            "Unstage File"
        } else {
            "Stage File"
        };
        let restore_title = if entry.status.is_created() {
            "Trash File"
        } else {
            "Discard Changes"
        };
        let context_menu = ContextMenu::build(window, cx, |context_menu, _, _| {
            let is_created = entry.status.is_created();
            context_menu
                .context(self.focus_handle.clone())
                .action(stage_title, ToggleStaged.boxed_clone())
                .action(restore_title, git::RestoreFile::default().boxed_clone())
                .action_disabled_when(
                    !is_created,
                    "Add to .gitignore",
                    git::AddToGitignore.boxed_clone(),
                )
                .separator()
                .action("Open Diff", menu::Confirm.boxed_clone())
                .action("Open File", menu::SecondaryConfirm.boxed_clone())
                .separator()
                .action_disabled_when(is_created, "View File History", Box::new(git::FileHistory))
        });
        self.selected_entry = Some(ix);
        self.set_context_menu(context_menu, position, window, cx);
    }

    fn deploy_panel_context_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_menu = git_panel_context_menu(
            self.focus_handle.clone(),
            GitMenuState {
                has_tracked_changes: self.has_tracked_changes(),
                has_staged_changes: self.has_staged_changes(),
                has_unstaged_changes: self.has_unstaged_changes(),
                has_new_changes: self.new_count > 0,
                sort_by_path: GitPanelSettings::get_global(cx).sort_by_path,
                has_stash_items: self.stash_entries.entries.len() > 0,
                tree_view: GitPanelSettings::get_global(cx).tree_view,
            },
            window,
            cx,
        );
        self.set_context_menu(context_menu, position, window, cx);
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
                this.context_menu.take();
                cx.notify();
            },
        );
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn render_status_entry(
        &self,
        ix: usize,
        entry: &GitStatusEntry,
        depth: usize,
        has_write_access: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let tree_view = GitPanelSettings::get_global(cx).tree_view;
        let path_style = self.project.read(cx).path_style(cx);
        let git_path_style = ProjectSettings::get_global(cx).git.path_style;
        let display_name = entry.display_name(path_style);

        let selected = self.selected_entry == Some(ix);
        let marked = self.marked_entries.contains(&ix);
        let status_style = GitPanelSettings::get_global(cx).status_style;
        let status = entry.status;

        let has_conflict = status.is_conflicted();
        let is_modified = status.is_modified();
        let is_deleted = status.is_deleted();
        let is_created = status.is_created();

        let label_color = if status_style == StatusStyle::LabelColor {
            if has_conflict {
                Color::VersionControlConflict
            } else if is_created {
                Color::VersionControlAdded
            } else if is_modified {
                Color::VersionControlModified
            } else if is_deleted {
                // We don't want a bunch of red labels in the list
                Color::Disabled
            } else {
                Color::VersionControlAdded
            }
        } else {
            Color::Default
        };

        let path_color = if status.is_deleted() {
            Color::Disabled
        } else {
            Color::Muted
        };

        let id: ElementId = ElementId::Name(format!("entry_{}_{}", display_name, ix).into());
        let checkbox_wrapper_id: ElementId =
            ElementId::Name(format!("entry_{}_{}_checkbox_wrapper", display_name, ix).into());
        let checkbox_id: ElementId =
            ElementId::Name(format!("entry_{}_{}_checkbox", display_name, ix).into());

        let active_repo = self
            .project
            .read(cx)
            .active_repository(cx)
            .expect("active repository must be set");
        let repo = active_repo.read(cx);
        let stage_status = GitPanel::stage_status_for_entry(entry, &repo);
        let mut is_staged: ToggleState = match stage_status {
            StageStatus::Staged => ToggleState::Selected,
            StageStatus::Unstaged => ToggleState::Unselected,
            StageStatus::PartiallyStaged => ToggleState::Indeterminate,
        };
        if self.show_placeholders && !self.has_staged_changes() && !entry.status.is_created() {
            is_staged = ToggleState::Selected;
        }

        let handle = cx.weak_entity();

        let selected_bg_alpha = 0.08;
        let marked_bg_alpha = 0.12;
        let state_opacity_step = 0.04;

        let info_color = cx.theme().status().info;

        let base_bg = match (selected, marked) {
            (true, true) => info_color.alpha(selected_bg_alpha + marked_bg_alpha),
            (true, false) => info_color.alpha(selected_bg_alpha),
            (false, true) => info_color.alpha(marked_bg_alpha),
            _ => cx.theme().colors().ghost_element_background,
        };

        let (hover_bg, active_bg) = if selected {
            (
                info_color.alpha(selected_bg_alpha + state_opacity_step),
                info_color.alpha(selected_bg_alpha + state_opacity_step * 2.0),
            )
        } else {
            (
                cx.theme().colors().ghost_element_hover,
                cx.theme().colors().ghost_element_active,
            )
        };

        let name_row = h_flex()
            .min_w_0()
            .flex_1()
            .gap_1()
            .child(git_status_icon(status))
            .map(|this| {
                if tree_view {
                    this.pl(px(depth as f32 * TREE_INDENT)).child(
                        self.entry_label(display_name, label_color)
                            .when(status.is_deleted(), Label::strikethrough)
                            .truncate(),
                    )
                } else {
                    this.child(self.path_formatted(
                        entry.parent_dir(path_style),
                        path_color,
                        display_name,
                        label_color,
                        path_style,
                        git_path_style,
                        status.is_deleted(),
                    ))
                }
            });

        h_flex()
            .id(id)
            .h(self.list_item_height())
            .w_full()
            .pl_3()
            .pr_1()
            .gap_1p5()
            .border_1()
            .border_r_2()
            .when(selected && self.focus_handle.is_focused(window), |el| {
                el.border_color(cx.theme().colors().panel_focused_border)
            })
            .bg(base_bg)
            .hover(|s| s.bg(hover_bg))
            .active(|s| s.bg(active_bg))
            .child(name_row)
            .child(
                div()
                    .id(checkbox_wrapper_id)
                    .flex_none()
                    .occlude()
                    .cursor_pointer()
                    .child(
                        Checkbox::new(checkbox_id, is_staged)
                            .disabled(!has_write_access)
                            .fill()
                            .elevation(ElevationIndex::Surface)
                            .on_click_ext({
                                let entry = entry.clone();
                                let this = cx.weak_entity();
                                move |_, click, window, cx| {
                                    this.update(cx, |this, cx| {
                                        if !has_write_access {
                                            return;
                                        }
                                        if click.modifiers().shift {
                                            this.stage_bulk(ix, cx);
                                        } else {
                                            let list_entry =
                                                if GitPanelSettings::get_global(cx).tree_view {
                                                    GitListEntry::TreeStatus(GitTreeStatusEntry {
                                                        entry: entry.clone(),
                                                        depth,
                                                    })
                                                } else {
                                                    GitListEntry::Status(entry.clone())
                                                };
                                            this.toggle_staged_for_entry(&list_entry, window, cx);
                                        }
                                        cx.stop_propagation();
                                    })
                                    .ok();
                                }
                            })
                            .tooltip(move |_window, cx| {
                                let action = match stage_status {
                                    StageStatus::Staged => "Unstage",
                                    StageStatus::Unstaged | StageStatus::PartiallyStaged => "Stage",
                                };
                                let tooltip_name = action.to_string();

                                Tooltip::for_action(tooltip_name, &ToggleStaged, cx)
                            }),
                    ),
            )
            .on_click({
                cx.listener(move |this, event: &ClickEvent, window, cx| {
                    this.selected_entry = Some(ix);
                    cx.notify();
                    if event.modifiers().secondary() {
                        this.open_file(&Default::default(), window, cx) // here?
                    } else {
                        this.open_diff(&Default::default(), window, cx); // here?
                        this.focus_handle.focus(window, cx);
                    }
                })
            })
            .on_mouse_down(
                MouseButton::Right,
                move |event: &MouseDownEvent, window, cx| {
                    // why isn't this happening automatically? we are passing MouseButton::Right to `on_mouse_down`?
                    if event.button != MouseButton::Right {
                        return;
                    }

                    let Some(this) = handle.upgrade() else {
                        return;
                    };
                    this.update(cx, |this, cx| {
                        this.deploy_entry_context_menu(event.position, ix, window, cx);
                    });
                    cx.stop_propagation();
                },
            )
            .into_any_element()
    }

    fn render_directory_entry(
        &self,
        ix: usize,
        entry: &GitTreeDirEntry,
        has_write_access: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        // TODO: Have not yet plugin the self.marked_entries. Not sure when and why we need that
        let selected = self.selected_entry == Some(ix);
        let label_color = Color::Muted;

        let id: ElementId = ElementId::Name(format!("dir_{}_{}", entry.name, ix).into());
        let checkbox_id: ElementId =
            ElementId::Name(format!("dir_checkbox_{}_{}", entry.name, ix).into());
        let checkbox_wrapper_id: ElementId =
            ElementId::Name(format!("dir_checkbox_wrapper_{}_{}", entry.name, ix).into());

        let selected_bg_alpha = 0.08;
        let state_opacity_step = 0.04;

        let info_color = cx.theme().status().info;
        let colors = cx.theme().colors();

        let (base_bg, hover_bg, active_bg) = if selected {
            (
                info_color.alpha(selected_bg_alpha),
                info_color.alpha(selected_bg_alpha + state_opacity_step),
                info_color.alpha(selected_bg_alpha + state_opacity_step * 2.0),
            )
        } else {
            (
                colors.ghost_element_background,
                colors.ghost_element_hover,
                colors.ghost_element_active,
            )
        };

        let folder_icon = if entry.expanded {
            IconName::FolderOpen
        } else {
            IconName::Folder
        };

        let stage_status = if let Some(repo) = &self.active_repository {
            self.stage_status_for_directory(entry, repo.read(cx))
        } else {
            util::debug_panic!(
                "Won't have entries to render without an active repository in Git Panel"
            );
            StageStatus::PartiallyStaged
        };

        let toggle_state: ToggleState = match stage_status {
            StageStatus::Staged => ToggleState::Selected,
            StageStatus::Unstaged => ToggleState::Unselected,
            StageStatus::PartiallyStaged => ToggleState::Indeterminate,
        };

        let name_row = h_flex()
            .min_w_0()
            .gap_1()
            .pl(px(entry.depth as f32 * TREE_INDENT))
            .child(
                Icon::new(folder_icon)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .child(self.entry_label(entry.name.clone(), label_color).truncate());

        h_flex()
            .id(id)
            .h(self.list_item_height())
            .min_w_0()
            .w_full()
            .pl_3()
            .pr_1()
            .gap_1p5()
            .justify_between()
            .border_1()
            .border_r_2()
            .when(selected && self.focus_handle.is_focused(window), |el| {
                el.border_color(cx.theme().colors().panel_focused_border)
            })
            .bg(base_bg)
            .hover(|s| s.bg(hover_bg))
            .active(|s| s.bg(active_bg))
            .child(name_row)
            .child(
                div()
                    .id(checkbox_wrapper_id)
                    .flex_none()
                    .occlude()
                    .cursor_pointer()
                    .child(
                        Checkbox::new(checkbox_id, toggle_state)
                            .disabled(!has_write_access)
                            .fill()
                            .elevation(ElevationIndex::Surface)
                            .on_click({
                                let entry = entry.clone();
                                let this = cx.weak_entity();
                                move |_, window, cx| {
                                    this.update(cx, |this, cx| {
                                        if !has_write_access {
                                            return;
                                        }
                                        this.toggle_staged_for_entry(
                                            &GitListEntry::Directory(entry.clone()),
                                            window,
                                            cx,
                                        );
                                        cx.stop_propagation();
                                    })
                                    .ok();
                                }
                            })
                            .tooltip(move |_window, cx| {
                                let action = match stage_status {
                                    StageStatus::Staged => "Unstage",
                                    StageStatus::Unstaged | StageStatus::PartiallyStaged => "Stage",
                                };
                                Tooltip::simple(format!("{action} folder"), cx)
                            }),
                    ),
            )
            .on_click({
                let key = entry.key.clone();
                cx.listener(move |this, _event: &ClickEvent, window, cx| {
                    this.selected_entry = Some(ix);
                    this.toggle_directory(&key, window, cx);
                })
            })
            .into_any_element()
    }

    fn path_formatted(
        &self,
        directory: Option<String>,
        path_color: Color,
        file_name: String,
        label_color: Color,
        path_style: PathStyle,
        git_path_style: GitPathStyle,
        strikethrough: bool,
    ) -> Div {
        let file_name_first = git_path_style == GitPathStyle::FileNameFirst;
        let file_path_first = git_path_style == GitPathStyle::FilePathFirst;

        let file_name = format!("{} ", file_name);

        h_flex()
            .min_w_0()
            .overflow_hidden()
            .when(file_path_first, |this| this.flex_row_reverse())
            .child(
                div().flex_none().child(
                    self.entry_label(file_name, label_color)
                        .when(strikethrough, Label::strikethrough),
                ),
            )
            .when_some(directory, |this, dir| {
                let path_name = if file_name_first {
                    dir
                } else {
                    format!("{dir}{}", path_style.primary_separator())
                };

                this.child(
                    self.entry_label(path_name, path_color)
                        .truncate_start()
                        .when(strikethrough, Label::strikethrough),
                )
            })
    }

    fn has_write_access(&self, cx: &App) -> bool {
        !self.project.read(cx).is_read_only(cx)
    }

    pub fn amend_pending(&self) -> bool {
        self.amend_pending
    }

    /// Sets the pending amend state, ensuring that the original commit message
    /// is either saved, when `value` is `true` and there's no pending amend, or
    /// restored, when `value` is `false` and there's a pending amend.
    pub fn set_amend_pending(&mut self, value: bool, cx: &mut Context<Self>) {
        if value && !self.amend_pending {
            let current_message = self.commit_message_buffer(cx).read(cx).text();
            self.original_commit_message = if current_message.trim().is_empty() {
                None
            } else {
                Some(current_message)
            };
        } else if !value && self.amend_pending {
            let message = self.original_commit_message.take().unwrap_or_default();
            self.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, message)], None, cx);
            });
        }

        self.amend_pending = value;
        self.serialize(cx);
        cx.notify();
    }

    pub fn signoff_enabled(&self) -> bool {
        self.signoff_enabled
    }

    pub fn set_signoff_enabled(&mut self, value: bool, cx: &mut Context<Self>) {
        self.signoff_enabled = value;
        self.serialize(cx);
        cx.notify();
    }

    pub fn toggle_signoff_enabled(
        &mut self,
        _: &Signoff,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_signoff_enabled(!self.signoff_enabled, cx);
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        let serialized_panel = match workspace
            .read_with(&cx, |workspace, _| Self::serialization_key(workspace))
            .ok()
            .flatten()
        {
            Some(serialization_key) => cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(&serialization_key) })
                .await
                .context("loading git panel")
                .log_err()
                .flatten()
                .map(|panel| serde_json::from_str::<SerializedGitPanel>(&panel))
                .transpose()
                .log_err()
                .flatten(),
            None => None,
        };

        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = GitPanel::new(workspace, window, cx);

            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width;
                    panel.amend_pending = serialized_panel.amend_pending;
                    panel.signoff_enabled = serialized_panel.signoff_enabled;
                    cx.notify();
                })
            }

            panel
        })
    }

    fn stage_bulk(&mut self, mut index: usize, cx: &mut Context<'_, Self>) {
        let Some(op) = self.bulk_staging.as_ref() else {
            return;
        };
        let Some(mut anchor_index) = self.entry_by_path(&op.anchor) else {
            return;
        };
        if let Some(entry) = self.entries.get(index)
            && let Some(entry) = entry.status_entry()
        {
            self.set_bulk_staging_anchor(entry.repo_path.clone(), cx);
        }
        if index < anchor_index {
            std::mem::swap(&mut index, &mut anchor_index);
        }
        let entries = self
            .entries
            .get(anchor_index..=index)
            .unwrap_or_default()
            .iter()
            .filter_map(|entry| entry.status_entry().cloned())
            .collect::<Vec<_>>();
        self.change_file_stage(true, entries, cx);
    }

    fn set_bulk_staging_anchor(&mut self, path: RepoPath, cx: &mut Context<'_, GitPanel>) {
        let Some(repo) = self.active_repository.as_ref() else {
            return;
        };
        self.bulk_staging = Some(BulkStaging {
            repo_id: repo.read(cx).id,
            anchor: path,
        });
    }

    pub(crate) fn toggle_amend_pending(&mut self, cx: &mut Context<Self>) {
        self.set_amend_pending(!self.amend_pending, cx);
        if self.amend_pending {
            self.load_last_commit_message(cx);
        }
    }
}

impl Render for GitPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project = self.project.read(cx);
        let has_entries = !self.entries.is_empty();
        let room = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_call()?.read(cx).room().cloned());

        let has_write_access = self.has_write_access(cx);

        let has_co_authors = room.is_some_and(|room| {
            self.load_local_committer(cx);
            let room = room.read(cx);
            room.remote_participants()
                .values()
                .any(|remote_participant| remote_participant.can_write())
        });

        v_flex()
            .id("git_panel")
            .key_context(self.dispatch_context(window, cx))
            .track_focus(&self.focus_handle)
            .when(has_write_access && !project.is_read_only(cx), |this| {
                this.on_action(cx.listener(Self::toggle_staged_for_selected))
                    .on_action(cx.listener(Self::stage_range))
                    .on_action(cx.listener(GitPanel::on_commit))
                    .on_action(cx.listener(GitPanel::on_amend))
                    .on_action(cx.listener(GitPanel::toggle_signoff_enabled))
                    .on_action(cx.listener(Self::stage_all))
                    .on_action(cx.listener(Self::unstage_all))
                    .on_action(cx.listener(Self::stage_selected))
                    .on_action(cx.listener(Self::unstage_selected))
                    .on_action(cx.listener(Self::restore_tracked_files))
                    .on_action(cx.listener(Self::revert_selected))
                    .on_action(cx.listener(Self::add_to_gitignore))
                    .on_action(cx.listener(Self::clean_all))
                    .on_action(cx.listener(Self::generate_commit_message_action))
                    .on_action(cx.listener(Self::stash_all))
                    .on_action(cx.listener(Self::stash_pop))
            })
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::first_entry))
            .on_action(cx.listener(Self::next_entry))
            .on_action(cx.listener(Self::previous_entry))
            .on_action(cx.listener(Self::last_entry))
            .on_action(cx.listener(Self::close_panel))
            .on_action(cx.listener(Self::open_diff))
            .on_action(cx.listener(Self::open_file))
            .on_action(cx.listener(Self::file_history))
            .on_action(cx.listener(Self::focus_changes_list))
            .on_action(cx.listener(Self::focus_editor))
            .on_action(cx.listener(Self::expand_commit_editor))
            .when(has_write_access && has_co_authors, |git_panel| {
                git_panel.on_action(cx.listener(Self::toggle_fill_co_authors))
            })
            .on_action(cx.listener(Self::toggle_sort_by_path))
            .on_action(cx.listener(Self::toggle_tree_view))
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().colors().panel_background)
            .child(
                v_flex()
                    .size_full()
                    .children(self.render_panel_header(window, cx))
                    .map(|this| {
                        if has_entries {
                            this.child(self.render_entries(has_write_access, window, cx))
                        } else {
                            this.child(self.render_empty_state(cx).into_any_element())
                        }
                    })
                    .children(self.render_footer(window, cx))
                    .when(self.amend_pending, |this| {
                        this.child(self.render_pending_amend(cx))
                    })
                    .when(!self.amend_pending, |this| {
                        this.children(self.render_previous_commit(cx))
                    })
                    .into_any_element(),
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
    }
}

impl Focusable for GitPanel {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        if self.entries.is_empty() {
            self.commit_editor.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl EventEmitter<Event> for GitPanel {}

impl EventEmitter<PanelEvent> for GitPanel {}

pub(crate) struct GitPanelAddon {
    pub(crate) workspace: WeakEntity<Workspace>,
}

impl editor::Addon for GitPanelAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn render_buffer_header_controls(
        &self,
        excerpt_info: &ExcerptInfo,
        window: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let file = excerpt_info.buffer.file()?;
        let git_panel = self.workspace.upgrade()?.read(cx).panel::<GitPanel>(cx)?;

        git_panel
            .read(cx)
            .render_buffer_header_controls(&git_panel, file, window, cx)
    }
}

impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "GitPanel"
    }

    fn panel_key() -> &'static str {
        GIT_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        GitPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.git_panel.get_or_insert_default().dock = Some(position.into())
        });
    }

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| GitPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranchAlt).filter(|_| GitPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        2
    }
}

impl PanelHeader for GitPanel {}

struct GitPanelMessageTooltip {
    commit_tooltip: Option<Entity<CommitTooltip>>,
}

impl GitPanelMessageTooltip {
    fn new(
        git_panel: Entity<GitPanel>,
        sha: SharedString,
        repository: Entity<Repository>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let remote_url = repository.read(cx).default_remote_url();
        cx.new(|cx| {
            cx.spawn_in(window, async move |this, cx| {
                let (details, workspace) = git_panel.update(cx, |git_panel, cx| {
                    (
                        git_panel.load_commit_details(sha.to_string(), cx),
                        git_panel.workspace.clone(),
                    )
                });
                let details = details.await?;
                let provider_registry = cx
                    .update(|_, app| GitHostingProviderRegistry::default_global(app))
                    .ok();

                let commit_details = crate::commit_tooltip::CommitDetails {
                    sha: details.sha.clone(),
                    author_name: details.author_name.clone(),
                    author_email: details.author_email.clone(),
                    commit_time: OffsetDateTime::from_unix_timestamp(details.commit_timestamp)?,
                    message: Some(ParsedCommitMessage::parse(
                        details.sha.to_string(),
                        details.message.to_string(),
                        remote_url.as_deref(),
                        provider_registry,
                    )),
                };

                this.update(cx, |this: &mut GitPanelMessageTooltip, cx| {
                    this.commit_tooltip = Some(cx.new(move |cx| {
                        CommitTooltip::new(commit_details, repository, workspace, cx)
                    }));
                    cx.notify();
                })
            })
            .detach();

            Self {
                commit_tooltip: None,
            }
        })
    }
}

impl Render for GitPanelMessageTooltip {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(commit_tooltip) = &self.commit_tooltip {
            commit_tooltip.clone().into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct PanelRepoFooter {
    active_repository: SharedString,
    branch: Option<Branch>,
    head_commit: Option<CommitDetails>,

    // Getting a GitPanel in previews will be difficult.
    //
    // For now just take an option here, and we won't bind handlers to buttons in previews.
    git_panel: Option<Entity<GitPanel>>,
}

impl PanelRepoFooter {
    pub fn new(
        active_repository: SharedString,
        branch: Option<Branch>,
        head_commit: Option<CommitDetails>,
        git_panel: Option<Entity<GitPanel>>,
    ) -> Self {
        Self {
            active_repository,
            branch,
            head_commit,
            git_panel,
        }
    }

    pub fn new_preview(active_repository: SharedString, branch: Option<Branch>) -> Self {
        Self {
            active_repository,
            branch,
            head_commit: None,
            git_panel: None,
        }
    }
}

impl RenderOnce for PanelRepoFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let project = self
            .git_panel
            .as_ref()
            .map(|panel| panel.read(cx).project.clone());

        let (workspace, repo) = self
            .git_panel
            .as_ref()
            .map(|panel| {
                let panel = panel.read(cx);
                (panel.workspace.clone(), panel.active_repository.clone())
            })
            .unzip();

        let single_repo = project
            .as_ref()
            .map(|project| project.read(cx).git_store().read(cx).repositories().len() == 1)
            .unwrap_or(true);

        const MAX_BRANCH_LEN: usize = 16;
        const MAX_REPO_LEN: usize = 16;
        const LABEL_CHARACTER_BUDGET: usize = MAX_BRANCH_LEN + MAX_REPO_LEN;
        const MAX_SHORT_SHA_LEN: usize = 8;
        let branch_name = self
            .branch
            .as_ref()
            .map(|branch| branch.name().to_owned())
            .or_else(|| {
                self.head_commit.as_ref().map(|commit| {
                    commit
                        .sha
                        .chars()
                        .take(MAX_SHORT_SHA_LEN)
                        .collect::<String>()
                })
            })
            .unwrap_or_else(|| " (no branch)".to_owned());
        let show_separator = self.branch.is_some() || self.head_commit.is_some();

        let active_repo_name = self.active_repository.clone();

        let branch_actual_len = branch_name.len();
        let repo_actual_len = active_repo_name.len();

        // ideally, show the whole branch and repo names but
        // when we can't, use a budget to allocate space between the two
        let (repo_display_len, branch_display_len) =
            if branch_actual_len + repo_actual_len <= LABEL_CHARACTER_BUDGET {
                (repo_actual_len, branch_actual_len)
            } else if branch_actual_len <= MAX_BRANCH_LEN {
                let repo_space = (LABEL_CHARACTER_BUDGET - branch_actual_len).min(MAX_REPO_LEN);
                (repo_space, branch_actual_len)
            } else if repo_actual_len <= MAX_REPO_LEN {
                let branch_space = (LABEL_CHARACTER_BUDGET - repo_actual_len).min(MAX_BRANCH_LEN);
                (repo_actual_len, branch_space)
            } else {
                (MAX_REPO_LEN, MAX_BRANCH_LEN)
            };

        let truncated_repo_name = if repo_actual_len <= repo_display_len {
            active_repo_name.to_string()
        } else {
            util::truncate_and_trailoff(active_repo_name.trim_ascii(), repo_display_len)
        };

        let truncated_branch_name = if branch_actual_len <= branch_display_len {
            branch_name
        } else {
            util::truncate_and_trailoff(branch_name.trim_ascii(), branch_display_len)
        };

        let repo_selector_trigger = Button::new("repo-selector", truncated_repo_name)
            .size(ButtonSize::None)
            .label_size(LabelSize::Small)
            .color(Color::Muted);

        let repo_selector = PopoverMenu::new("repository-switcher")
            .menu({
                let project = project;
                move |window, cx| {
                    let project = project.clone()?;
                    Some(cx.new(|cx| RepositorySelector::new(project, rems(16.), window, cx)))
                }
            })
            .trigger_with_tooltip(
                repo_selector_trigger.disabled(single_repo).truncate(true),
                Tooltip::text("Switch Active Repository"),
            )
            .anchor(Corner::BottomLeft)
            .into_any_element();

        let branch_selector_button = Button::new("branch-selector", truncated_branch_name)
            .size(ButtonSize::None)
            .label_size(LabelSize::Small)
            .truncate(true)
            .on_click(|_, window, cx| {
                window.dispatch_action(zed_actions::git::Switch.boxed_clone(), cx);
            });

        let branch_selector = PopoverMenu::new("popover-button")
            .menu(move |window, cx| {
                let workspace = workspace.clone()?;
                let repo = repo.clone().flatten();
                Some(branch_picker::popover(workspace, false, repo, window, cx))
            })
            .trigger_with_tooltip(
                branch_selector_button,
                Tooltip::for_action_title("Switch Branch", &zed_actions::git::Switch),
            )
            .anchor(Corner::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            });

        h_flex()
            .h(px(36.))
            .w_full()
            .px_2()
            .justify_between()
            .gap_1()
            .child(
                h_flex()
                    .flex_1()
                    .overflow_hidden()
                    .gap_px()
                    .child(
                        Icon::new(IconName::GitBranchAlt)
                            .size(IconSize::Small)
                            .color(if single_repo {
                                Color::Disabled
                            } else {
                                Color::Muted
                            }),
                    )
                    .child(repo_selector)
                    .when(show_separator, |this| {
                        this.child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().colors().icon_muted.opacity(0.5))
                                .child("/"),
                        )
                    })
                    .child(branch_selector),
            )
            .children(if let Some(git_panel) = self.git_panel {
                git_panel.update(cx, |git_panel, cx| git_panel.render_remote_button(cx))
            } else {
                None
            })
    }
}

impl Component for PanelRepoFooter {
    fn scope() -> ComponentScope {
        ComponentScope::VersionControl
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let unknown_upstream = None;
        let no_remote_upstream = Some(UpstreamTracking::Gone);
        let ahead_of_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 2,
                behind: 0,
            }
            .into(),
        );
        let behind_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 0,
                behind: 2,
            }
            .into(),
        );
        let ahead_and_behind_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 3,
                behind: 1,
            }
            .into(),
        );

        let not_ahead_or_behind_upstream = Some(
            UpstreamTrackingStatus {
                ahead: 0,
                behind: 0,
            }
            .into(),
        );

        fn branch(upstream: Option<UpstreamTracking>) -> Branch {
            Branch {
                is_head: true,
                ref_name: "some-branch".into(),
                upstream: upstream.map(|tracking| Upstream {
                    ref_name: "origin/some-branch".into(),
                    tracking,
                }),
                most_recent_commit: Some(CommitSummary {
                    sha: "abc123".into(),
                    subject: "Modify stuff".into(),
                    commit_timestamp: 1710932954,
                    author_name: "John Doe".into(),
                    has_parent: true,
                }),
            }
        }

        fn custom(branch_name: &str, upstream: Option<UpstreamTracking>) -> Branch {
            Branch {
                is_head: true,
                ref_name: branch_name.to_string().into(),
                upstream: upstream.map(|tracking| Upstream {
                    ref_name: format!("zed/{}", branch_name).into(),
                    tracking,
                }),
                most_recent_commit: Some(CommitSummary {
                    sha: "abc123".into(),
                    subject: "Modify stuff".into(),
                    commit_timestamp: 1710932954,
                    author_name: "John Doe".into(),
                    has_parent: true,
                }),
            }
        }

        fn active_repository(id: usize) -> SharedString {
            format!("repo-{}", id).into()
        }

        let example_width = px(340.);
        Some(
            v_flex()
                .gap_6()
                .w_full()
                .flex_none()
                .children(vec![
                    example_group_with_title(
                        "Action Button States",
                        vec![
                            single_example(
                                "No Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(active_repository(1), None))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Remote status unknown",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(2),
                                        Some(branch(unknown_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "No Remote Upstream",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(3),
                                        Some(branch(no_remote_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Not Ahead or Behind",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(4),
                                        Some(branch(not_ahead_or_behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Behind remote",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(5),
                                        Some(branch(behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Ahead of remote",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(6),
                                        Some(branch(ahead_of_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Ahead and behind remote",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        active_repository(7),
                                        Some(branch(ahead_and_behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                        ],
                    )
                    .grow()
                    .vertical(),
                ])
                .children(vec![
                    example_group_with_title(
                        "Labels",
                        vec![
                            single_example(
                                "Short Branch & Repo",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed"),
                                        Some(custom("main", behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Long Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed"),
                                        Some(custom(
                                            "redesign-and-update-git-ui-list-entry-style",
                                            behind_upstream,
                                        )),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Long Repo",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed-industries-community-examples"),
                                        Some(custom("gpui", ahead_of_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Long Repo & Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed-industries-community-examples"),
                                        Some(custom(
                                            "redesign-and-update-git-ui-list-entry-style",
                                            behind_upstream,
                                        )),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Uppercase Repo",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("LICENSES"),
                                        Some(custom("main", ahead_of_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Uppercase Branch",
                                div()
                                    .w(example_width)
                                    .overflow_hidden()
                                    .child(PanelRepoFooter::new_preview(
                                        SharedString::from("zed"),
                                        Some(custom("update-README", behind_upstream)),
                                    ))
                                    .into_any_element(),
                            ),
                        ],
                    )
                    .grow()
                    .vertical(),
                ])
                .into_any_element(),
        )
    }
}

fn open_output(
    operation: impl Into<SharedString>,
    workspace: &mut Workspace,
    output: &str,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let operation = operation.into();
    let buffer = cx.new(|cx| Buffer::local(output, cx));
    buffer.update(cx, |buffer, cx| {
        buffer.set_capability(language::Capability::ReadOnly, cx);
    });
    let editor = cx.new(|cx| {
        let mut editor = Editor::for_buffer(buffer, None, window, cx);
        editor.buffer().update(cx, |buffer, cx| {
            buffer.set_title(format!("Output from git {operation}"), cx);
        });
        editor.set_read_only(true);
        editor
    });

    workspace.add_item_to_center(Box::new(editor), window, cx);
}

pub(crate) fn show_error_toast(
    workspace: Entity<Workspace>,
    action: impl Into<SharedString>,
    e: anyhow::Error,
    cx: &mut App,
) {
    let action = action.into();
    let message = e.to_string().trim().to_string();
    if message
        .matches(git::repository::REMOTE_CANCELLED_BY_USER)
        .next()
        .is_some()
    { // Hide the cancelled by user message
    } else {
        workspace.update(cx, |workspace, cx| {
            let workspace_weak = cx.weak_entity();
            let toast = StatusToast::new(format!("git {} failed", action), cx, |this, _cx| {
                this.icon(ToastIcon::new(IconName::XCircle).color(Color::Error))
                    .action("View Log", move |window, cx| {
                        let message = message.clone();
                        let action = action.clone();
                        workspace_weak
                            .update(cx, move |workspace, cx| {
                                open_output(action, workspace, &message, window, cx)
                            })
                            .ok();
                    })
            });
            workspace.toggle_status_toast(toast, cx)
        });
    }
}

#[cfg(test)]
mod tests {
    use git::{
        repository::repo_path,
        status::{StatusCode, UnmergedStatus, UnmergedStatusCode},
    };
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext};
    use indoc::indoc;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use theme::LoadThemes;
    use util::path;
    use util::rel_path::rel_path;

    use super::*;

    fn init_test(cx: &mut gpui::TestAppContext) {
        zlog::init_test();

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    async fn test_entry_worktree_paths(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "zed": {
                    ".git": {},
                    "crates": {
                        "gpui": {
                            "gpui.rs": "fn main() {}"
                        },
                        "util": {
                            "util.rs": "fn do_it() {}"
                        }
                    }
                },
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/zed/.git")),
            &[
                ("crates/gpui/gpui.rs", StatusCode::Modified.worktree()),
                ("crates/util/util.rs", StatusCode::Modified.worktree()),
            ],
        );

        let project =
            Project::test(fs.clone(), [path!("/root/zed/crates/gpui").as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

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

        cx.executor().run_until_parked();

        let panel = workspace.update(cx, GitPanel::new).unwrap();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        pretty_assertions::assert_eq!(
            entries,
            [
                GitListEntry::Header(GitHeaderEntry {
                    header: Section::Tracked
                }),
                GitListEntry::Status(GitStatusEntry {
                    repo_path: repo_path("crates/gpui/gpui.rs"),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                }),
                GitListEntry::Status(GitStatusEntry {
                    repo_path: repo_path("crates/util/util.rs"),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                },),
            ],
        );

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;
        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        pretty_assertions::assert_eq!(
            entries,
            [
                GitListEntry::Header(GitHeaderEntry {
                    header: Section::Tracked
                }),
                GitListEntry::Status(GitStatusEntry {
                    repo_path: repo_path("crates/gpui/gpui.rs"),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                }),
                GitListEntry::Status(GitStatusEntry {
                    repo_path: repo_path("crates/util/util.rs"),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                },),
            ],
        );
    }

    #[gpui::test]
    async fn test_bulk_staging(cx: &mut TestAppContext) {
        use GitListEntry::*;

        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}",
                        "lib.rs": "pub fn hello() {}",
                        "utils.rs": "pub fn util() {}"
                    },
                    "tests": {
                        "test.rs": "fn test() {}"
                    },
                    "new_file.txt": "new content",
                    "another_new.rs": "// new file",
                    "conflict.txt": "conflicted content"
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project/.git")),
            &[
                ("src/main.rs", StatusCode::Modified.worktree()),
                ("src/lib.rs", StatusCode::Modified.worktree()),
                ("tests/test.rs", StatusCode::Modified.worktree()),
                ("new_file.txt", FileStatus::Untracked),
                ("another_new.rs", FileStatus::Untracked),
                ("src/utils.rs", FileStatus::Untracked),
                (
                    "conflict.txt",
                    UnmergedStatus {
                        first_head: UnmergedStatusCode::Updated,
                        second_head: UnmergedStatusCode::Updated,
                    }
                    .into(),
                ),
            ],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/root/project"))], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

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

        cx.executor().run_until_parked();

        let panel = workspace.update(cx, GitPanel::new).unwrap();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        #[rustfmt::skip]
        pretty_assertions::assert_matches!(
            entries.as_slice(),
            &[
                Header(GitHeaderEntry { header: Section::Conflict }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Header(GitHeaderEntry { header: Section::Tracked }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Header(GitHeaderEntry { header: Section::New }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
            ],
        );

        let second_status_entry = entries[3].clone();
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&second_status_entry, window, cx);
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.selected_entry = Some(7);
            panel.stage_range(&git::StageRange, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        #[rustfmt::skip]
        pretty_assertions::assert_matches!(
            entries.as_slice(),
            &[
                Header(GitHeaderEntry { header: Section::Conflict }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Header(GitHeaderEntry { header: Section::Tracked }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Header(GitHeaderEntry { header: Section::New }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
            ],
        );

        let third_status_entry = entries[4].clone();
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&third_status_entry, window, cx);
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.selected_entry = Some(9);
            panel.stage_range(&git::StageRange, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        #[rustfmt::skip]
        pretty_assertions::assert_matches!(
            entries.as_slice(),
            &[
                Header(GitHeaderEntry { header: Section::Conflict }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Header(GitHeaderEntry { header: Section::Tracked }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Header(GitHeaderEntry { header: Section::New }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
            ],
        );
    }

    #[gpui::test]
    async fn test_bulk_staging_with_sort_by_paths(cx: &mut TestAppContext) {
        use GitListEntry::*;

        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}",
                        "lib.rs": "pub fn hello() {}",
                        "utils.rs": "pub fn util() {}"
                    },
                    "tests": {
                        "test.rs": "fn test() {}"
                    },
                    "new_file.txt": "new content",
                    "another_new.rs": "// new file",
                    "conflict.txt": "conflicted content"
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project/.git")),
            &[
                ("src/main.rs", StatusCode::Modified.worktree()),
                ("src/lib.rs", StatusCode::Modified.worktree()),
                ("tests/test.rs", StatusCode::Modified.worktree()),
                ("new_file.txt", FileStatus::Untracked),
                ("another_new.rs", FileStatus::Untracked),
                ("src/utils.rs", FileStatus::Untracked),
                (
                    "conflict.txt",
                    UnmergedStatus {
                        first_head: UnmergedStatusCode::Updated,
                        second_head: UnmergedStatusCode::Updated,
                    }
                    .into(),
                ),
            ],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/root/project"))], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

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

        cx.executor().run_until_parked();

        let panel = workspace.update(cx, GitPanel::new).unwrap();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        #[rustfmt::skip]
        pretty_assertions::assert_matches!(
            entries.as_slice(),
            &[
                Header(GitHeaderEntry { header: Section::Conflict }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Header(GitHeaderEntry { header: Section::Tracked }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Header(GitHeaderEntry { header: Section::New }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
            ],
        );

        assert_entry_paths(
            &entries,
            &[
                None,
                Some("conflict.txt"),
                None,
                Some("src/lib.rs"),
                Some("src/main.rs"),
                Some("tests/test.rs"),
                None,
                Some("another_new.rs"),
                Some("new_file.txt"),
                Some("src/utils.rs"),
            ],
        );

        let second_status_entry = entries[3].clone();
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&second_status_entry, window, cx);
        });

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().sort_by_path = Some(true);
                })
            });
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.selected_entry = Some(7);
            panel.stage_range(&git::StageRange, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        #[rustfmt::skip]
        pretty_assertions::assert_matches!(
            entries.as_slice(),
            &[
                Status(GitStatusEntry { status: FileStatus::Untracked, staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Unmerged(..), staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Untracked, staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Tracked(..), staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { status: FileStatus::Tracked(..), staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Untracked, staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Tracked(..), staging: StageStatus::Unstaged, .. }),
            ],
        );

        assert_entry_paths(
            &entries,
            &[
                Some("another_new.rs"),
                Some("conflict.txt"),
                Some("new_file.txt"),
                Some("src/lib.rs"),
                Some("src/main.rs"),
                Some("src/utils.rs"),
                Some("tests/test.rs"),
            ],
        );

        let third_status_entry = entries[4].clone();
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&third_status_entry, window, cx);
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.selected_entry = Some(9);
            panel.stage_range(&git::StageRange, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());
        #[rustfmt::skip]
        pretty_assertions::assert_matches!(
            entries.as_slice(),
            &[
                Status(GitStatusEntry { status: FileStatus::Untracked, staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Unmerged(..), staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Untracked, staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Tracked(..), staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { status: FileStatus::Tracked(..), staging: StageStatus::Staged, .. }),
                Status(GitStatusEntry { status: FileStatus::Untracked, staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { status: FileStatus::Tracked(..), staging: StageStatus::Unstaged, .. }),
            ],
        );

        assert_entry_paths(
            &entries,
            &[
                Some("another_new.rs"),
                Some("conflict.txt"),
                Some("new_file.txt"),
                Some("src/lib.rs"),
                Some("src/main.rs"),
                Some("src/utils.rs"),
                Some("tests/test.rs"),
            ],
        );
    }

    #[gpui::test]
    async fn test_amend_commit_message_handling(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project/.git")),
            &[("src/main.rs", StatusCode::Modified.worktree())],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/root/project"))], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        let panel = workspace.update(cx, GitPanel::new).unwrap();

        // Test: User has commit message, enables amend (saves message), then disables (restores message)
        panel.update(cx, |panel, cx| {
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, "Initial commit message")], None, cx);
            });

            panel.set_amend_pending(true, cx);
            assert!(panel.original_commit_message.is_some());

            panel.set_amend_pending(false, cx);
            let current_message = panel.commit_message_buffer(cx).read(cx).text();
            assert_eq!(current_message, "Initial commit message");
            assert!(panel.original_commit_message.is_none());
        });

        // Test: User has empty commit message, enables amend, then disables (clears message)
        panel.update(cx, |panel, cx| {
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, "")], None, cx);
            });

            panel.set_amend_pending(true, cx);
            assert!(panel.original_commit_message.is_none());

            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, "Previous commit message")], None, cx);
            });

            panel.set_amend_pending(false, cx);
            let current_message = panel.commit_message_buffer(cx).read(cx).text();
            assert_eq!(current_message, "");
        });
    }

    #[gpui::test]
    async fn test_amend(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project/.git")),
            &[("src/main.rs", StatusCode::Modified.worktree())],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/root/project"))], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        // Wait for the project scanning to finish so that `head_commit(cx)` is
        // actually set, otherwise no head commit would be available from which
        // to fetch the latest commit message from.
        cx.executor().run_until_parked();

        let panel = workspace.update(cx, GitPanel::new).unwrap();
        panel.read_with(cx, |panel, cx| {
            assert!(panel.active_repository.is_some());
            assert!(panel.head_commit(cx).is_some());
        });

        panel.update_in(cx, |panel, window, cx| {
            // Update the commit editor's message to ensure that its contents
            // are later restored, after amending is finished.
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                buffer.set_text("refactor: update main.rs", cx);
            });

            // Start amending the previous commit.
            panel.focus_editor(&Default::default(), window, cx);
            panel.on_amend(&Amend, window, cx);
        });

        // Since `GitPanel.amend` attempts to fetch the latest commit message in
        // a background task, we need to wait for it to complete before being
        // able to assert that the commit message editor's state has been
        // updated.
        cx.run_until_parked();

        panel.update_in(cx, |panel, window, cx| {
            assert_eq!(
                panel.commit_message_buffer(cx).read(cx).text(),
                "initial commit"
            );
            assert_eq!(
                panel.original_commit_message,
                Some("refactor: update main.rs".to_string())
            );

            // Finish amending the previous commit.
            panel.focus_editor(&Default::default(), window, cx);
            panel.on_amend(&Amend, window, cx);
        });

        // Since the actual commit logic is run in a background task, we need to
        // await its completion to actually ensure that the commit message
        // editor's contents are set to the original message and haven't been
        // cleared.
        cx.run_until_parked();

        panel.update_in(cx, |panel, _window, cx| {
            // After amending, the commit editor's message should be restored to
            // the original message.
            assert_eq!(
                panel.commit_message_buffer(cx).read(cx).text(),
                "refactor: update main.rs"
            );
            assert!(panel.original_commit_message.is_none());
        });
    }

    #[gpui::test]
    async fn test_open_diff(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "tracked": "tracked\n",
                "untracked": "\n",
            }),
        )
        .await;

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("tracked", "old tracked\n".into())],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, GitPanel::new).unwrap();

        // Enable the `sort_by_path` setting and wait for entries to be updated,
        // as there should no longer be separators between Tracked and Untracked
        // files.
        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().sort_by_path = Some(true);
                })
            });
        });

        cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        })
        .await;

        // Confirm that `Open Diff` still works for the untracked file, updating
        // the Project Diff's active path.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_entry = Some(1);
            panel.open_diff(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();

        let _ = workspace.update(cx, |workspace, _window, cx| {
            let active_path = workspace
                .item_of_type::<ProjectDiff>(cx)
                .expect("ProjectDiff should exist")
                .read(cx)
                .active_path(cx)
                .expect("active_path should exist");

            assert_eq!(active_path.path, rel_path("untracked").into_arc());
        });
    }

    #[gpui::test]
    async fn test_tree_view_reveals_collapsed_parent_on_select_entry_by_path(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "src": {
                    "a": {
                        "foo.rs": "fn foo() {}",
                    },
                    "b": {
                        "bar.rs": "fn bar() {}",
                    },
                },
            }),
        )
        .await;

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("src/a/foo.rs", StatusCode::Modified.worktree()),
                ("src/b/bar.rs", StatusCode::Modified.worktree()),
            ],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

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

        cx.executor().run_until_parked();

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().tree_view = Some(true);
                })
            });
        });

        let panel = workspace.update(cx, GitPanel::new).unwrap();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let src_key = panel.read_with(cx, |panel, _| {
            panel
                .entries
                .iter()
                .find_map(|entry| match entry {
                    GitListEntry::Directory(dir) if dir.key.path == repo_path("src") => {
                        Some(dir.key.clone())
                    }
                    _ => None,
                })
                .expect("src directory should exist in tree view")
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_directory(&src_key, window, cx);
        });

        panel.read_with(cx, |panel, _| {
            let state = panel
                .view_mode
                .tree_state()
                .expect("tree view state should exist");
            assert_eq!(state.expanded_dirs.get(&src_key).copied(), Some(false));
        });

        let worktree_id =
            cx.read(|cx| project.read(cx).worktrees(cx).next().unwrap().read(cx).id());
        let project_path = ProjectPath {
            worktree_id,
            path: RelPath::unix("src/a/foo.rs").unwrap().into_arc(),
        };

        panel.update_in(cx, |panel, window, cx| {
            panel.select_entry_by_path(project_path, window, cx);
        });

        panel.read_with(cx, |panel, _| {
            let state = panel
                .view_mode
                .tree_state()
                .expect("tree view state should exist");
            assert_eq!(state.expanded_dirs.get(&src_key).copied(), Some(true));

            let selected_ix = panel.selected_entry.expect("selection should be set");
            assert!(state.logical_indices.contains(&selected_ix));

            let selected_entry = panel
                .entries
                .get(selected_ix)
                .and_then(|entry| entry.status_entry())
                .expect("selected entry should be a status entry");
            assert_eq!(selected_entry.repo_path, repo_path("src/a/foo.rs"));
        });
    }

    fn assert_entry_paths(entries: &[GitListEntry], expected_paths: &[Option<&str>]) {
        assert_eq!(entries.len(), expected_paths.len());
        for (entry, expected_path) in entries.iter().zip(expected_paths) {
            assert_eq!(
                entry.status_entry().map(|status| status
                    .repo_path
                    .as_ref()
                    .as_std_path()
                    .to_string_lossy()
                    .to_string()),
                expected_path.map(|s| s.to_string())
            );
        }
    }

    #[test]
    fn test_compress_diff_no_truncation() {
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,2 @@
            -old
            +new
        "};
        let result = GitPanel::compress_commit_diff(diff, 1000);
        assert_eq!(result, diff);
    }

    #[test]
    fn test_compress_diff_truncate_long_lines() {
        let long_line = "🦀".repeat(300);
        let diff = indoc::formatdoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,3 @@
             context
            +{}
             more context
        ", long_line};
        let result = GitPanel::compress_commit_diff(&diff, 100);
        assert!(result.contains("...[truncated]"));
        assert!(result.len() < diff.len());
    }

    #[test]
    fn test_compress_diff_truncate_hunks() {
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,2 @@
             context
            -old1
            +new1
            @@ -5,2 +5,2 @@
             context 2
            -old2
            +new2
            @@ -10,2 +10,2 @@
             context 3
            -old3
            +new3
        "};
        let result = GitPanel::compress_commit_diff(diff, 100);
        let expected = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,2 @@
             context
            -old1
            +new1
            [...skipped 2 hunks...]
        "};
        assert_eq!(result, expected);
    }

    #[gpui::test]
    async fn test_suggest_commit_message(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "tracked": "tracked\n",
                "untracked": "\n",
            }),
        )
        .await;

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("tracked", "old tracked\n".into())],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, GitPanel::new).unwrap();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let entries = panel.read_with(cx, |panel, _| panel.entries.clone());

        // GitPanel
        // - Tracked:
        // - [] tracked
        // - Untracked
        // - [] untracked
        //
        // The commit message should now read:
        // "Update tracked"
        let message = panel.update(cx, |panel, cx| panel.suggest_commit_message(cx));
        assert_eq!(message, Some("Update tracked".to_string()));

        let first_status_entry = entries[1].clone();
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&first_status_entry, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        // GitPanel
        // - Tracked:
        // - [x] tracked
        // - Untracked
        // - [] untracked
        //
        // The commit message should still read:
        // "Update tracked"
        let message = panel.update(cx, |panel, cx| panel.suggest_commit_message(cx));
        assert_eq!(message, Some("Update tracked".to_string()));

        let second_status_entry = entries[3].clone();
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&second_status_entry, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        // GitPanel
        // - Tracked:
        // - [x] tracked
        // - Untracked
        // - [x] untracked
        //
        // The commit message should now read:
        // "Enter commit message"
        // (which means we should see None returned).
        let message = panel.update(cx, |panel, cx| panel.suggest_commit_message(cx));
        assert!(message.is_none());

        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&first_status_entry, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        // GitPanel
        // - Tracked:
        // - [] tracked
        // - Untracked
        // - [x] untracked
        //
        // The commit message should now read:
        // "Update untracked"
        let message = panel.update(cx, |panel, cx| panel.suggest_commit_message(cx));
        assert_eq!(message, Some("Create untracked".to_string()));

        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(&second_status_entry, window, cx);
        });

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

        cx.executor().run_until_parked();

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        // GitPanel
        // - Tracked:
        // - [] tracked
        // - Untracked
        // - [] untracked
        //
        // The commit message should now read:
        // "Update tracked"
        let message = panel.update(cx, |panel, cx| panel.suggest_commit_message(cx));
        assert_eq!(message, Some("Update tracked".to_string()));
    }
}
