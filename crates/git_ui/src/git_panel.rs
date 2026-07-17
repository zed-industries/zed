use crate::askpass_modal::AskPassModal;
use crate::commit_context_menu::{
    CommitContextMenuData, CommitContextMenuSource, commit_context_menu,
};
use crate::commit_modal::CommitModal;
use crate::commit_tooltip::{CommitAvatar, CommitTooltip};
use crate::commit_view::CommitView;
use crate::git_panel_settings::GitPanelScrollbarAccessor;
use crate::project_diff::{DeployBranchDiff, Diff, ProjectDiff};
use crate::remote_output::{self, RemoteAction, SuccessMessage};
use crate::solo_diff_view::SoloDiffView;
use crate::staged_diff::StagedDiff;
use crate::unstaged_diff::UnstagedDiff;
use crate::{branch_picker, picker_prompt, render_remote_button};
use crate::{
    git_panel_settings::GitPanelSettings, git_status_icon, repository_selector::RepositorySelector,
};
use agent_settings::{AgentSettings, UserAgentsMd};
use anyhow::Context as _;
use askpass::AskPassDelegate;
use client::zed_urls;
use collections::{BTreeMap, HashMap, HashSet};
use db::kvp::KeyValueStore;
use editor::{Editor, EditorElement, EditorMode, MultiBuffer, MultiBufferOffset, SizingBehavior};
use editor::{EditorStyle, RewrapOptions};
use file_icons::FileIcons;
use futures::StreamExt as _;
use futures::channel::oneshot::Canceled;
use git::Oid;
use git::commit::ParsedCommitMessage;
use git::repository::{
    Branch, CommitData, CommitDetails, CommitOptions, CommitSummary, DiffType, FetchOptions,
    GitCommitTemplate, GitCommitter, InitialGraphCommitData, LogOrder, LogSource, PushOptions,
    Remote, RemoteCommandOutput, ResetMode, Upstream, UpstreamTracking, UpstreamTrackingStatus,
    get_git_committer,
};
use git::stash::GitStash;
use git::status::{DiffStat, StageStatus};
use git::{Amend, Commit, Signoff, ToggleStaged, repository::RepoPath, status::FileStatus};
use git::{
    ExpandCommitEditor, GitHostingProviderRegistry, GitRemote, RestoreTrackedFiles, StageAll,
    StashAll, StashApply, StashPop, ToggleFillCommitEditor, TrashUntrackedFiles, UnstageAll,
    ViewFile, parse_git_remote_url,
};
use gpui::{
    AbsoluteLength, Action, Anchor, AnyElement, AsyncApp, AsyncWindowContext, ClickEvent,
    DismissEvent, Empty, Entity, EventEmitter, FocusHandle, Focusable, KeyContext, MouseButton,
    MouseDownEvent, Pixels, Point, PromptLevel, ScrollStrategy, Subscription, Task, TaskExt,
    TextStyle, UniformListScrollHandle, WeakEntity, actions, anchored, deferred, uniform_list,
};
use itertools::Itertools;
use language::{Buffer, BufferEvent, File};
use language_model::{
    CompletionIntent, ConfiguredModel, Event as LanguageModelEvent, LanguageModelRegistry,
    LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use menu;
use multi_buffer::ExcerptBoundaryInfo;
use notifications::status_toast::StatusToast;
use panel::PanelHeader;
use project::git_store::GitAccess;
use project::{
    Fs, Project, ProjectPath,
    git_store::{
        CommitDataState, GitStoreEvent, Repository, RepositoryEvent, RepositoryId, pending_op,
    },
    project_settings::{GitPathStyle, ProjectSettings},
};
use prompt_store::RULES_FILE_NAMES;
use proto::RpcError;
use serde::{Deserialize, Serialize};
use settings::{
    GitPanelClickBehavior, GitPanelGroupBy, GitPanelSortBy, Settings, SettingsStore, StatusStyle,
    update_settings_file,
};
use smallvec::SmallVec;
use std::cell::Cell;
use std::future::Future;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::{sync::Arc, time::Duration, usize};
use strum::{IntoEnumIterator, VariantNames};
use theme_settings::ThemeSettings;
use time::OffsetDateTime;
use ui::{
    ButtonLike, Checkbox, Chip, ContextMenu, ContextMenuEntry, Divider, ElevationIndex,
    IndentGuideColors, KeyBinding, PopoverMenu, PopoverMenuHandle, ProjectEmptyState, ScrollAxes,
    Scrollbars, SplitButton, Tab, TintColor, Tooltip, WithScrollbar, prelude::*,
};
use util::paths::PathStyle;
use util::{ResultExt, TryFutureExt, markdown::MarkdownInlineCode, maybe, rel_path::RelPath};
use workspace::SERIALIZATION_THROTTLE_TIME;
use workspace::{
    Item, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{DetachAndPromptErr, NotificationId, NotifyTaskExt},
};
use zed_actions::{
    DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize, git_panel::ToggleFocus,
};

const GIT_PANEL_KEY: &str = "GitPanel";
const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);
// TODO: We should revise this part. It seems the indentation width is not aligned with the one in project panel
const TREE_INDENT: f32 = 16.0;
// Repository rows include both a disclosure and a folder icon, while section and
// content rows have fewer leading controls. These small offsets keep each child
// row's first visible element clearly to the right of its parent without making
// the dense panel excessively wide.
const SECTION_ROW_INDENT_OFFSET: f32 = 8.0;
const CONTENT_ROW_INDENT_OFFSET: f32 = 16.0;
const MAX_HISTORY_TAG_CHIPS: usize = 3;
// Horizontal offset that aligns the tree indent guides with the row icon column.
const INDENT_GUIDE_LEFT_OFFSET: gpui::Pixels = gpui::px(19.);

actions!(
    git_panel,
    [
        /// Closes the git panel.
        Close,
        /// Toggles the git panel.
        Toggle,
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
        /// Sorts entries by path.
        SetSortByPath,
        /// Sorts entries by name.
        SetSortByName,
        /// Disables grouping entries by status.
        SetGroupByNone,
        /// Groups entries by status.
        SetGroupByStatus,
        /// Groups entries by staging state.
        SetGroupByStaging,
        /// Toggles showing entries in tree vs flat view.
        ToggleTreeView,
        /// Shows changes only from the active repository.
        ShowCurrentRepository,
        /// Shows changes from all repositories in the current project.
        ShowAllRepositories,
        /// Expands the selected entry to show its children.
        ExpandSelectedEntry,
        /// Collapses the selected entry to hide its children.
        CollapseSelectedEntry,
        /// View unstaged changes
        ViewUnstagedChanges,
        /// View staged changes
        ViewStagedChanges,
        /// Activates the Changes tab.
        ActivateChangesTab,
        /// Activates the History tab.
        ActivateHistoryTab,
    ]
);

actions!(
    dev,
    [
        /// Shows the current git job queue debug state for the active repository.
        ShowGitJobQueue,
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

#[derive(Clone, Copy)]
struct GitPanelViewOptionsMenuState {
    sort_by: GitPanelSortBy,
    group_by: GitPanelGroupBy,
    tree_view: bool,
    show_all_repositories: bool,
}

fn git_panel_context_menu(
    has_tracked_changes: bool,
    has_staged_changes: bool,
    has_unstaged_changes: bool,
    has_new_changes: bool,
    has_stash_items: bool,
    focus_handle: FocusHandle,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ContextMenu> {
    let show_all_repositories = GitPanelSettings::get_global(cx).show_all_repositories;
    let stage_all_label = if show_all_repositories {
        "Stage All in Active Repository"
    } else {
        "Stage All"
    };
    let unstage_all_label = if show_all_repositories {
        "Unstage All in Active Repository"
    } else {
        "Unstage All"
    };
    let restore_all_label = if show_all_repositories {
        "Restore All in Active Repository"
    } else {
        "Restore All Changes"
    };
    let stash_all_label = if show_all_repositories {
        "Stash Active Repository"
    } else {
        "Stash All"
    };
    let discard_tracked_label = if show_all_repositories {
        "Discard Tracked Changes in Active Repository"
    } else {
        "Discard Tracked Changes"
    };
    let trash_untracked_label = if show_all_repositories {
        "Trash Untracked Files in Active Repository"
    } else {
        "Trash Untracked Files"
    };
    ContextMenu::build(window, cx, |context_menu, _, _| {
        context_menu
            .context(focus_handle.clone())
            .action_disabled_when(
                !has_unstaged_changes,
                stage_all_label,
                StageAll.boxed_clone(),
            )
            .action_disabled_when(
                !has_staged_changes,
                unstage_all_label,
                UnstageAll.boxed_clone(),
            )
            .action_disabled_when(
                !has_tracked_changes,
                restore_all_label,
                RestoreTrackedFiles.boxed_clone(),
            )
            .separator()
            .action_disabled_when(
                !(has_new_changes || has_tracked_changes),
                stash_all_label,
                StashAll.boxed_clone(),
            )
            .action_disabled_when(!has_stash_items, "Stash Pop", StashPop.boxed_clone())
            .action("View Stash", zed_actions::git::ViewStash.boxed_clone())
            .separator()
            .action_disabled_when(
                !has_tracked_changes,
                discard_tracked_label,
                RestoreTrackedFiles.boxed_clone(),
            )
            .action_disabled_when(
                !has_new_changes,
                trash_untracked_label,
                TrashUntrackedFiles.boxed_clone(),
            )
    })
}

fn git_panel_view_options_menu(
    focus_handle: FocusHandle,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ContextMenu> {
    let view_options_menu_state = Rc::new(Cell::new(GitPanelViewOptionsMenuState {
        sort_by: GitPanelSettings::get_global(cx).sort_by,
        group_by: GitPanelSettings::get_global(cx).group_by,
        tree_view: GitPanelSettings::get_global(cx).tree_view,
        show_all_repositories: GitPanelSettings::get_global(cx).show_all_repositories,
    }));

    ContextMenu::build_persistent(window, cx, move |context_menu, _, _| {
        let state = view_options_menu_state.get();

        context_menu
            .context(focus_handle.clone())
            .header("Repository Scope")
            .item({
                let view_options_menu_state = view_options_menu_state.clone();
                ContextMenuEntry::new("Current Repository")
                    .toggle(IconPosition::End, !state.show_all_repositories)
                    .handler(move |window, cx| {
                        if state.show_all_repositories {
                            view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                show_all_repositories: false,
                                ..state
                            });
                            window.dispatch_action(Box::new(ShowCurrentRepository), cx);
                        }
                    })
            })
            .item({
                let view_options_menu_state = view_options_menu_state.clone();
                ContextMenuEntry::new("All Repositories")
                    .toggle(IconPosition::End, state.show_all_repositories)
                    .handler(move |window, cx| {
                        if !state.show_all_repositories {
                            view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                show_all_repositories: true,
                                ..state
                            });
                            window.dispatch_action(Box::new(ShowAllRepositories), cx);
                        }
                    })
            })
            .separator()
            .header("View")
            .item({
                let view_options_menu_state = view_options_menu_state.clone();
                ContextMenuEntry::new("List")
                    .toggle(IconPosition::End, !state.tree_view)
                    .handler(move |window, cx| {
                        if state.tree_view {
                            view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                tree_view: false,
                                ..state
                            });
                            window.dispatch_action(Box::new(ToggleTreeView), cx);
                        }
                    })
            })
            .item({
                let view_options_menu_state = view_options_menu_state.clone();
                ContextMenuEntry::new("Tree")
                    .toggle(IconPosition::End, state.tree_view)
                    .handler(move |window, cx| {
                        if !state.tree_view {
                            view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                tree_view: true,
                                ..state
                            });
                            window.dispatch_action(Box::new(ToggleTreeView), cx);
                        }
                    })
            })
            .when(!state.tree_view, |this| {
                this.separator()
                    .header("Sort By")
                    .item({
                        let view_options_menu_state = view_options_menu_state.clone();
                        ContextMenuEntry::new("Path")
                            .toggle(IconPosition::End, state.sort_by == GitPanelSortBy::Path)
                            .handler(move |window, cx| {
                                if !state.tree_view {
                                    view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                        sort_by: GitPanelSortBy::Path,
                                        ..state
                                    });
                                    window.dispatch_action(Box::new(SetSortByPath), cx);
                                }
                            })
                    })
                    .item({
                        let view_options_menu_state = view_options_menu_state.clone();
                        ContextMenuEntry::new("Name")
                            .toggle(IconPosition::End, state.sort_by == GitPanelSortBy::Name)
                            .handler(move |window, cx| {
                                if !state.tree_view {
                                    view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                        sort_by: GitPanelSortBy::Name,
                                        ..state
                                    });
                                    window.dispatch_action(Box::new(SetSortByName), cx);
                                }
                            })
                    })
            })
            .separator()
            .header("Group By")
            .item({
                let view_options_menu_state = view_options_menu_state.clone();
                ContextMenuEntry::new("None")
                    .toggle(IconPosition::End, state.group_by == GitPanelGroupBy::None)
                    .handler(move |window, cx| {
                        if state.group_by != GitPanelGroupBy::None {
                            view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                group_by: GitPanelGroupBy::None,
                                ..state
                            });
                            window.dispatch_action(Box::new(SetGroupByNone), cx);
                        }
                    })
            })
            .item({
                let view_options_menu_state = view_options_menu_state.clone();
                ContextMenuEntry::new("Tracked & Untracked")
                    .toggle(IconPosition::End, state.group_by == GitPanelGroupBy::Status)
                    .handler(move |window, cx| {
                        if state.group_by != GitPanelGroupBy::Status {
                            view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                group_by: GitPanelGroupBy::Status,
                                ..state
                            });
                            window.dispatch_action(Box::new(SetGroupByStatus), cx);
                        }
                    })
            })
            .item({
                let view_options_menu_state = view_options_menu_state.clone();
                ContextMenuEntry::new("Staged & Unstaged")
                    .toggle(
                        IconPosition::End,
                        state.group_by == GitPanelGroupBy::Staging,
                    )
                    .handler(move |window, cx| {
                        if state.group_by != GitPanelGroupBy::Staging {
                            view_options_menu_state.set(GitPanelViewOptionsMenuState {
                                group_by: GitPanelGroupBy::Staging,
                                ..state
                            });
                            window.dispatch_action(Box::new(SetGroupByStaging), cx);
                        }
                    })
            })
    })
}

// We only allow a single remote operation at a time to avoid concurrent
// credential prompts and competing ref/working-tree updates.
#[derive(Clone, Copy)]
pub(crate) enum RemoteOperationKind {
    Fetch,
    Pull,
    Push,
}

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<GitPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &Toggle, window, cx| {
        if !workspace.toggle_panel_focus::<GitPanel>(window, cx) {
            workspace.close_panel::<GitPanel>(window, cx);
        }
    });
    workspace.register_action(|workspace, _: &ExpandCommitEditor, window, cx| {
        CommitModal::toggle(workspace, None, window, cx)
    });
    workspace.register_action(|workspace, _: &ToggleFillCommitEditor, window, cx| {
        if let Some(panel) = workspace.panel::<GitPanel>(cx) {
            panel.update(cx, |panel, cx| {
                panel.toggle_fill_commit_editor(&Default::default(), window, cx)
            });
        }
    });
    workspace.register_action(|workspace, _: &git::Init, window, cx| {
        if let Some(panel) = workspace.panel::<GitPanel>(cx) {
            panel.update(cx, |panel, cx| panel.git_init(window, cx));
        }
    });
    workspace.register_action(|workspace, _: &ShowGitJobQueue, window, cx| {
        if let Some(panel) = workspace.panel::<GitPanel>(cx) {
            panel.update(cx, |panel, cx| {
                panel.show_git_job_queue(window, cx);
            });
        }
    });
}

#[derive(Debug, Clone)]
pub enum Event {
    Focus,
}

#[derive(Default, Serialize, Deserialize)]
struct SerializedGitPanel {
    #[serde(default)]
    signoff_enabled: bool,
    #[serde(default)]
    commit_messages: BTreeMap<String, SerializedCommitMessage>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct SerializedCommitMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    original_message: Option<String>,
    #[serde(default)]
    amend_pending: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GitPanelTab {
    Changes,
    History,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CommitHistory {
    Loading,
    /// A non-empty list can still grow on later fetches.
    /// An empty list means the repository has no commits.
    Loaded(Rc<[CommitHistoryEntry]>),
    Error(SharedString),
}

fn commit_history_from_response(
    entries: Rc<[CommitHistoryEntry]>,
    is_loading: bool,
    error: Option<SharedString>,
) -> CommitHistory {
    if !entries.is_empty() {
        CommitHistory::Loaded(entries)
    } else if let Some(error) = error {
        CommitHistory::Error(error)
    } else if is_loading {
        CommitHistory::Loading
    } else {
        CommitHistory::Loaded(Rc::from([]))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Section {
    Conflict,
    Tracked,
    New,
    Staged,
    Unstaged,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ChangeKey {
    repository_id: RepositoryId,
    repo_path: RepoPath,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GitRepositoryKind {
    Primary,
    Submodule,
    Repository,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GitRepositoryHeaderEntry {
    repository_id: RepositoryId,
    display_name: SharedString,
    work_directory: SharedString,
    branch_label: SharedString,
    kind: GitRepositoryKind,
    parent_display_name: Option<SharedString>,
    change_count: usize,
    is_active: bool,
    expanded: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GitProjectRepositoriesEntry {
    repository_count: usize,
    change_count: usize,
    expanded: bool,
    contains_active_repository: bool,
}

fn root_repository_id(
    repository_id: RepositoryId,
    parent_by_repository_id: &HashMap<RepositoryId, RepositoryId>,
) -> RepositoryId {
    let mut root_id = repository_id;
    while let Some(parent_id) = parent_by_repository_id.get(&root_id) {
        if *parent_id == root_id {
            break;
        }
        root_id = *parent_id;
    }
    root_id
}

fn repository_depth_below_root(
    repository_id: RepositoryId,
    root_id: RepositoryId,
    parent_by_repository_id: &HashMap<RepositoryId, RepositoryId>,
) -> Option<usize> {
    let mut current_id = repository_id;
    let mut depth = 0;

    while current_id != root_id {
        current_id = *parent_by_repository_id.get(&current_id)?;
        depth += 1;

        // The map is built from strictly nested work directories, but keep a
        // defensive bound here so malformed repository metadata cannot loop.
        if depth > parent_by_repository_id.len() {
            return None;
        }
    }

    (depth > 0).then_some(depth)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GitHeaderEntry {
    header: Section,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ProjectedChangeEntry {
    section: Section,
    index: usize,
}

/// What clicking a staging control should do.
///
/// In the "staged & unstaged" grouping, a partially staged file appears in both the
/// "Staged" and "Unstaged" sections at once, so a row's meaning comes from
/// the section it is rendered in rather than from the file's own state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StageIntent {
    Stage,
    Unstage,
    Toggle,
}

impl StageIntent {
    fn for_section(section: Section) -> Self {
        match section {
            Section::Staged => StageIntent::Unstage,
            Section::Unstaged => StageIntent::Stage,
            _ => StageIntent::Toggle,
        }
    }

    /// Resolves to a concrete direction (`true` = stage), consulting the
    /// current stage status only when no section dictates one.
    fn resolve_with(self, stage_status: impl FnOnce() -> StageStatus) -> bool {
        match self {
            StageIntent::Stage => true,
            StageIntent::Unstage => false,
            StageIntent::Toggle => match stage_status() {
                StageStatus::Staged => false,
                StageStatus::Unstaged | StageStatus::PartiallyStaged => true,
            },
        }
    }

    fn checkbox_state(self, entry_state: impl FnOnce() -> ToggleState) -> ToggleState {
        match self {
            StageIntent::Stage => ToggleState::Unselected,
            StageIntent::Unstage => ToggleState::Selected,
            StageIntent::Toggle => entry_state(),
        }
    }

    fn label(self, stage_status: impl FnOnce() -> StageStatus) -> &'static str {
        if self.resolve_with(stage_status) {
            "Stage"
        } else {
            "Unstage"
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DiffTarget {
    Uncommitted,
    Staged,
    Unstaged,
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
            // Conflicted files render only under the Conflict section, so the
            // Staged/Unstaged bulk operations must not sweep them up: "Unstage
            // All" would silently un-resolve conflicts, and "Stage All" would
            // silently mark them resolved.
            Section::Staged => {
                !repo.had_conflict_on_last_merge_head_change(&status_entry.repo_path)
                    && GitPanel::stage_status_for_entry(status_entry, repo).has_staged()
            }
            Section::Unstaged => {
                !repo.had_conflict_on_last_merge_head_change(&status_entry.repo_path)
                    && GitPanel::stage_status_for_entry(status_entry, repo).has_unstaged()
            }
        }
    }
    pub fn title(&self) -> &'static str {
        match self.header {
            Section::Conflict => "Conflicts",
            Section::Tracked => "Tracked",
            Section::New => "Untracked",
            Section::Staged => "Staged",
            Section::Unstaged => "Unstaged",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum GitListEntry {
    RepositoryHeader(GitRepositoryHeaderEntry),
    ProjectRepositoriesHeader(GitProjectRepositoriesEntry),
    Status(GitStatusEntry),
    TreeStatus(GitTreeStatusEntry),
    Directory(GitTreeDirEntry),
    Header(GitHeaderEntry),
    EmptySection(Section),
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

    /// Returns the indentation contributed by directories within a section.
    fn tree_depth(&self) -> usize {
        match self {
            GitListEntry::Directory(dir) => dir.depth,
            GitListEntry::TreeStatus(status) => status.depth,
            _ => 0,
        }
    }

    /// Returns the indentation contributed by the repository hierarchy itself.
    fn repository_hierarchy_depth(&self) -> usize {
        match self {
            GitListEntry::RepositoryHeader(_) | GitListEntry::ProjectRepositoriesHeader(_) => 0,
            GitListEntry::Header(_) => 1,
            GitListEntry::Status(_)
            | GitListEntry::TreeStatus(_)
            | GitListEntry::Directory(_)
            | GitListEntry::EmptySection(_) => 2,
        }
    }

    fn is_selectable(&self) -> bool {
        matches!(
            self,
            GitListEntry::RepositoryHeader(_)
                | GitListEntry::ProjectRepositoriesHeader(_)
                | GitListEntry::Status(_)
                | GitListEntry::TreeStatus(_)
                | GitListEntry::Directory(_)
                | GitListEntry::Header(_)
        )
    }

    fn is_stageable(&self) -> bool {
        matches!(
            self,
            GitListEntry::Status(_) | GitListEntry::TreeStatus(_) | GitListEntry::Directory(_)
        )
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
    expanded_dirs: HashMap<TreeKey, bool>,
    directory_descendants: HashMap<TreeKey, Vec<GitStatusEntry>>,
}

impl TreeViewState {
    fn build_tree_entries(
        &mut self,
        repository_id: RepositoryId,
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

        let (flattened, _) = self.flatten_tree(&root, repository_id, section, 0, seen_directories);
        flattened
    }

    fn flatten_tree(
        &mut self,
        node: &TreeNode,
        repository_id: RepositoryId,
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
            let (child_flattened, mut child_statuses) = self.flatten_tree(
                terminal,
                repository_id,
                section,
                depth + 1,
                seen_directories,
            );
            let key = TreeKey {
                repository_id,
                section,
                path,
            };
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
    repository_id: RepositoryId,
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
    pub(crate) diff_stat: Option<DiffStat>,
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
    active_repository_id: Option<RepositoryId>,
    pub(crate) commit_editor: Entity<Editor>,
    /// Whether the commit editor should fill the vertical height of the panel.
    commit_editor_expanded: bool,
    conflicted_count: usize,
    conflicted_staged_count: usize,
    add_coauthors: bool,
    generate_commit_message_task: Option<Task<Option<()>>>,
    entries: Vec<GitListEntry>,
    /// Maps visible list indices to the corresponding model indices in `entries`.
    /// Collapsed directories and project repositories remain in the model so
    /// repository-scoped staging and commit state stay correct.
    visible_entry_indices: Vec<usize>,
    /// Repository identity for each entry at the same index in `entries`.
    /// Repository ids are resolved against this panel's current GitStore and
    /// are never persisted across workspace/worktree switches.
    entry_repository_ids: Vec<RepositoryId>,
    repository_entry_ranges: HashMap<RepositoryId, Range<usize>>,
    project_repositories_expanded: bool,
    project_repository_depths: HashMap<RepositoryId, usize>,
    collapsed_repositories: HashSet<RepositoryId>,
    collapsed_sections: HashSet<(RepositoryId, Section)>,
    view_mode: GitPanelViewMode,
    tree_expanded_dirs: HashMap<TreeKey, bool>,
    projected_entries_by_path: HashMap<ChangeKey, SmallVec<[ProjectedChangeEntry; 2]>>,
    single_staged_entry: Option<GitStatusEntry>,
    single_tracked_entry: Option<GitStatusEntry>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    new_count: usize,
    entry_count: usize,
    changes_count: usize,
    active_changes_count: usize,
    diff_stat_total: DiffStat,
    new_staged_count: usize,
    pending_commit: Option<Task<()>>,
    pending_remote_operation: Option<RemoteOperationKind>,
    amend_pending: bool,
    original_commit_message: Option<String>,
    pending_commit_message_restores: BTreeMap<String, SerializedCommitMessage>,
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
    reopen_commit_buffer_task: Task<()>,
    pub(crate) workspace: WeakEntity<Workspace>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    modal_open: bool,
    show_placeholders: bool,
    // Only read to compute collaborative co-authors, which requires the `call` feature.
    #[cfg_attr(not(feature = "call"), allow(dead_code))]
    local_committer: Option<GitCommitter>,
    local_committer_task: Option<Task<()>>,
    commit_template: Option<GitCommitTemplate>,
    bulk_staging: Option<BulkStaging>,
    stash_entries: GitStash,
    active_tab: GitPanelTab,
    commit_history_scroll_handle: UniformListScrollHandle,
    commit_history: CommitHistory,
    focused_history_entry: Option<usize>,
    history_keyboard_nav: bool,
    _commit_message_buffer_subscription: Option<Subscription>,
    _repo_subscriptions: Vec<Subscription>,
    _settings_subscription: Subscription,
    git_access: Option<GitAccess>,
    commit_menu_handle: PopoverMenuHandle<ContextMenu>,
    changes_actions_menu_handle: PopoverMenuHandle<ContextMenu>,
    remote_action_menu_handle: PopoverMenuHandle<ContextMenu>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BulkStaging {
    repo_id: RepositoryId,
    anchor: RepoPath,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CommitHistoryEntry {
    sha: Oid,
    tag_names: Vec<SharedString>,
}

impl From<&Arc<InitialGraphCommitData>> for CommitHistoryEntry {
    fn from(commit: &Arc<InitialGraphCommitData>) -> Self {
        Self {
            sha: commit.sha,
            tag_names: commit
                .tag_names()
                .into_iter()
                .map(|tag_name| SharedString::from(tag_name.to_string()))
                .collect(),
        }
    }
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
    // Only the test-support constructors call this thin wrapper now; production
    // registration goes through `new_with_serialized_panel` directly. Gate it to
    // the same cfg as `new_test` so the non-test lib build doesn't see it as dead.
    #[cfg(any(test, feature = "test-support"))]
    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        Self::new_with_serialized_panel(workspace, None, window, cx)
    }

    fn new_with_serialized_panel(
        workspace: &mut Workspace,
        serialized_panel: Option<SerializedGitPanel>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let app_state = workspace.app_state().clone();
        let fs = app_state.fs.clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);
        let active_repository_id = active_repository
            .as_ref()
            .map(|repository| repository.read(cx).id);
        let signoff_enabled = serialized_panel
            .as_ref()
            .is_some_and(|panel| panel.signoff_enabled);
        let active_work_directory_abs_path = active_repository.as_ref().map(|repository| {
            repository
                .read(cx)
                .work_directory_abs_path
                .to_string_lossy()
                .into_owned()
        });
        let active_draft = serialized_panel.as_ref().and_then(|panel| {
            let path = active_work_directory_abs_path.as_ref()?;
            panel.commit_messages.get(path)
        });
        // Seed the placeholder editor with the restored draft when the active
        // repository already matches the serialized one, so the message is
        // present immediately on restart instead of only after the commit
        // buffer finishes loading in `reopen_commit_buffer`. Sourced from the
        // serialized draft rather than a live buffer snapshot and scoped to the
        // matching repository, so it neither replays cleared text nor leaks a
        // draft across repositories. `reopen_commit_buffer` still performs the
        // one-shot restore into the loaded buffer; applying the same draft
        // there is idempotent.
        let amend_pending = active_draft.is_some_and(|draft| draft.amend_pending);
        let original_commit_message = active_draft.and_then(|draft| draft.original_message.clone());
        let initial_commit_message = active_draft
            .and_then(|draft| draft.message.clone())
            .unwrap_or_default();
        let pending_commit_message_restores = serialized_panel
            .map(|panel| panel.commit_messages)
            .unwrap_or_default();

        cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, Self::focus_in).detach();

            let mut was_sort_by = GitPanelSettings::get_global(cx).sort_by;
            let mut was_group_by = GitPanelSettings::get_global(cx).group_by;
            let mut was_tree_view = GitPanelSettings::get_global(cx).tree_view;
            let mut was_show_all_repositories =
                GitPanelSettings::get_global(cx).show_all_repositories;
            let mut was_file_icons = GitPanelSettings::get_global(cx).file_icons;
            let mut was_folder_icons = GitPanelSettings::get_global(cx).folder_icons;
            let mut was_diff_stats = GitPanelSettings::get_global(cx).diff_stats;
            cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
                let settings = GitPanelSettings::get_global(cx);
                let sort_by = settings.sort_by;
                let group_by = settings.group_by;
                let tree_view = settings.tree_view;
                let show_all_repositories = settings.show_all_repositories;
                let file_icons = settings.file_icons;
                let folder_icons = settings.folder_icons;
                let diff_stats = settings.diff_stats;
                if tree_view != was_tree_view {
                    match (&mut this.view_mode, tree_view) {
                        (GitPanelViewMode::Tree(state), false) => {
                            this.tree_expanded_dirs = state.expanded_dirs.clone();
                            this.view_mode = GitPanelViewMode::Flat;
                        }
                        (GitPanelViewMode::Flat, true) => {
                            this.view_mode = GitPanelViewMode::Tree(TreeViewState {
                                expanded_dirs: this.tree_expanded_dirs.clone(),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    }
                }

                let mut update_entries = false;
                if sort_by != was_sort_by
                    || group_by != was_group_by
                    || tree_view != was_tree_view
                    || show_all_repositories != was_show_all_repositories
                {
                    this.bulk_staging.take();
                    update_entries = true;
                }
                if (diff_stats != was_diff_stats) || update_entries {
                    this.update_visible_entries(window, cx);
                }
                if file_icons != was_file_icons || folder_icons != was_folder_icons {
                    cx.notify();
                }
                was_sort_by = sort_by;
                was_group_by = group_by;
                was_tree_view = tree_view;
                was_show_all_repositories = show_all_repositories;
                was_file_icons = file_icons;
                was_folder_icons = folder_icons;
                was_diff_stats = diff_stats;
            })
            .detach();

            cx.observe_global::<FileIcons>(|_, cx| {
                cx.notify();
            })
            .detach();

            // just to let us render a placeholder editor.
            // Once the active git repo is set, this buffer will be replaced.
            let temporary_buffer = cx.new(|cx| Buffer::local(initial_commit_message, cx));
            let commit_editor = cx.new(|cx| {
                commit_message_editor(temporary_buffer, None, project.clone(), true, window, cx)
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

            let registry = LanguageModelRegistry::global(cx);
            cx.subscribe(&registry, |_, _, event, cx| match event {
                LanguageModelEvent::CommitMessageModelChanged
                | LanguageModelEvent::DefaultModelChanged
                | LanguageModelEvent::ProviderStateChanged(_)
                | LanguageModelEvent::AddedProvider(_)
                | LanguageModelEvent::RemovedProvider(_)
                | LanguageModelEvent::ProvidersChanged => {
                    cx.notify();
                }
                _ => {}
            })
            .detach();

            cx.subscribe_in(
                &git_store,
                window,
                move |this, _git_store, event, window, cx| match event {
                    GitStoreEvent::RepositoryUpdated(
                        _,
                        RepositoryEvent::StatusesChanged | RepositoryEvent::HeadChanged,
                        is_active,
                    ) if *is_active || GitPanelSettings::get_global(cx).show_all_repositories => {
                        this.schedule_update(window, cx);
                    }
                    GitStoreEvent::RepositoryAdded
                    | GitStoreEvent::RepositoryRemoved(_)
                    | GitStoreEvent::ActiveRepositoryChanged(_) => {
                        this.schedule_update(window, cx);
                    }
                    GitStoreEvent::RepositoryUpdated(
                        _,
                        RepositoryEvent::GitDirectoryChanged,
                        true,
                    )
                    | GitStoreEvent::GlobalConfigurationUpdated => {
                        this.git_access = None;
                        this.schedule_update(window, cx);
                    }
                    GitStoreEvent::IndexWriteError(error) => {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                workspace.show_error(format!("{error}"), cx);
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
                active_repository_id,
                commit_editor,
                commit_editor_expanded: false,
                conflicted_count: 0,
                conflicted_staged_count: 0,
                add_coauthors: true,
                generate_commit_message_task: None,
                entries: Vec::new(),
                visible_entry_indices: Vec::new(),
                entry_repository_ids: Vec::new(),
                repository_entry_ranges: HashMap::default(),
                project_repositories_expanded: true,
                project_repository_depths: HashMap::default(),
                collapsed_repositories: HashSet::default(),
                collapsed_sections: HashSet::default(),
                view_mode: GitPanelViewMode::from_settings(cx),
                tree_expanded_dirs: HashMap::default(),
                projected_entries_by_path: HashMap::default(),
                focus_handle: cx.focus_handle(),
                fs,
                new_count: 0,
                new_staged_count: 0,
                changes_count: 0,
                active_changes_count: 0,
                diff_stat_total: DiffStat::default(),
                pending_commit: None,
                pending_remote_operation: None,
                amend_pending,
                original_commit_message,
                pending_commit_message_restores,
                signoff_enabled,
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
                reopen_commit_buffer_task: Task::ready(()),
                show_placeholders: false,
                local_committer: None,
                local_committer_task: None,
                commit_template: None,
                context_menu: None,
                workspace: workspace.weak_handle(),
                modal_open: false,
                entry_count: 0,
                bulk_staging: None,
                stash_entries: Default::default(),
                active_tab: GitPanelTab::Changes,
                commit_history_scroll_handle: UniformListScrollHandle::new(),
                commit_history: CommitHistory::Loading,
                focused_history_entry: None,
                history_keyboard_nav: false,
                _commit_message_buffer_subscription: None,
                _repo_subscriptions: Vec::new(),
                _settings_subscription,
                git_access: None,
                commit_menu_handle: PopoverMenuHandle::default(),
                changes_actions_menu_handle: PopoverMenuHandle::default(),
                remote_action_menu_handle: PopoverMenuHandle::default(),
            };

            this.schedule_update(window, cx);
            this
        })
    }

    pub fn entry_by_path(&self, path: &RepoPath) -> Option<usize> {
        let repository_id = self.active_repository_id?;
        self.entry_by_change_key(&ChangeKey {
            repository_id,
            repo_path: path.clone(),
        })
    }

    fn entry_by_change_key(&self, key: &ChangeKey) -> Option<usize> {
        self.projected_entries_by_path
            .get(key)?
            .first()
            .map(|entry| entry.index)
    }

    #[cfg(test)]
    fn entry_by_path_in_section(&self, path: &RepoPath, section: Section) -> Option<usize> {
        let repository_id = self.active_repository_id?;
        self.entry_by_change_key_in_section(
            &ChangeKey {
                repository_id,
                repo_path: path.clone(),
            },
            section,
        )
    }

    fn entry_by_change_key_in_section(&self, key: &ChangeKey, section: Section) -> Option<usize> {
        self.projected_entries_by_path
            .get(key)?
            .iter()
            .find(|entry| entry.section == section)
            .map(|entry| entry.index)
    }

    fn repository_id_for_entry_index(&self, ix: usize) -> Option<RepositoryId> {
        debug_assert_eq!(self.entries.len(), self.entry_repository_ids.len());
        self.entry_repository_ids.get(ix).copied()
    }

    fn repository_for_id(
        &self,
        repository_id: RepositoryId,
        cx: &App,
    ) -> Option<Entity<Repository>> {
        let git_store = self.project.read(cx).git_store().clone();
        git_store
            .read(cx)
            .repositories()
            .get(&repository_id)
            .cloned()
    }

    fn activate_repository(&self, repository_id: RepositoryId, cx: &mut App) {
        if let Some(repository) = self.repository_for_id(repository_id, cx) {
            repository.update(cx, |repository, cx| repository.set_as_active_repository(cx));
        }
    }

    fn repository_for_entry_index(&self, ix: usize, cx: &App) -> Option<Entity<Repository>> {
        self.repository_for_id(self.repository_id_for_entry_index(ix)?, cx)
    }

    fn repository_local_entry_index(
        &self,
        ix: usize,
        repository_id: RepositoryId,
    ) -> Option<usize> {
        (self.repository_id_for_entry_index(ix) == Some(repository_id)).then(|| {
            self.entry_repository_ids[..=ix]
                .iter()
                .filter(|id| **id == repository_id)
                .count()
                - 1
        })
    }

    fn is_entry_visible(&self, ix: usize) -> bool {
        self.visible_entry_indices.binary_search(&ix).is_ok()
    }

    fn project_repository_depth(&self, repository_id: RepositoryId) -> usize {
        self.project_repository_depths
            .get(&repository_id)
            .copied()
            .unwrap_or(0)
    }

    fn visual_depth_for_entry(&self, ix: usize) -> usize {
        let Some(entry) = self.entries.get(ix) else {
            return 0;
        };
        let project_repository_depth = self
            .repository_id_for_entry_index(ix)
            .map(|repository_id| self.project_repository_depth(repository_id))
            .unwrap_or(0);

        project_repository_depth + entry.repository_hierarchy_depth() + entry.tree_depth()
    }

    pub fn select_entry_by_path(
        &mut self,
        path: ProjectPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let repository_and_path = if GitPanelSettings::get_global(cx).show_all_repositories {
            self.project
                .read(cx)
                .git_store()
                .read(cx)
                .repository_and_path_for_project_path(&path, cx)
        } else {
            self.active_repository.as_ref().and_then(|repository| {
                let repo_path = repository.read(cx).project_path_to_repo_path(&path, cx)?;
                Some((repository.clone(), repo_path))
            })
        };
        let Some((git_repo, repo_path)) = repository_and_path else {
            return;
        };

        let (repository_id, repo_path, default_section) = {
            let repo = git_repo.read(cx);
            let section = repo
                .status_for_path(&repo_path)
                .map(|status| status.status)
                .map(|status| {
                    if GitPanelSettings::get_global(cx).group_by == GitPanelGroupBy::Staging {
                        if repo.had_conflict_on_last_merge_head_change(&repo_path) {
                            Section::Conflict
                        } else if status.staging().has_staged() {
                            Section::Staged
                        } else {
                            Section::Unstaged
                        }
                    } else if repo.had_conflict_on_last_merge_head_change(&repo_path) {
                        Section::Conflict
                    } else if status.is_created() {
                        Section::New
                    } else {
                        Section::Tracked
                    }
                });

            (repo.id, repo_path, section)
        };
        let selected_section = self.selected_entry.and_then(|index| {
            let selected_entry = self.entries.get(index)?.status_entry()?;
            if self.repository_id_for_entry_index(index) == Some(repository_id)
                && selected_entry.repo_path == repo_path
            {
                self.section_for_entry_index(index)
            } else {
                None
            }
        });
        let section = selected_section.or(default_section);

        let mut needs_rebuild = false;
        if !self.project_repositories_expanded
            && self.project_repository_depths.contains_key(&repository_id)
        {
            self.project_repositories_expanded = true;
            needs_rebuild = true;
        }
        if self.collapsed_repositories.remove(&repository_id) {
            needs_rebuild = true;
        }
        if let Some(section) = section
            && self.collapsed_sections.remove(&(repository_id, section))
        {
            needs_rebuild = true;
        }
        if let (Some(section), Some(tree_state)) = (section, self.view_mode.tree_state_mut()) {
            let mut current_dir = repo_path.parent();
            while let Some(dir) = current_dir {
                let key = TreeKey {
                    repository_id,
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

        let change_key = ChangeKey {
            repository_id,
            repo_path,
        };
        let Some(ix) = section
            .and_then(|section| self.entry_by_change_key_in_section(&change_key, section))
            .or_else(|| self.entry_by_change_key(&change_key))
        else {
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
        let signoff_enabled = self.signoff_enabled;
        let commit_messages = self.serialized_commit_messages(cx);
        let kvp = KeyValueStore::global(cx);

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
                    kvp.write_kvp(
                        serialization_key,
                        serde_json::to_string(&SerializedGitPanel {
                            signoff_enabled,
                            commit_messages,
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

    fn serialized_commit_messages(&self, cx: &App) -> BTreeMap<String, SerializedCommitMessage> {
        let active_work_directory_abs_path = self.active_repository.as_ref().map(|repository| {
            repository
                .read(cx)
                .work_directory_abs_path
                .to_string_lossy()
                .into_owned()
        });
        let git_store = self.project.read(cx).git_store().clone();
        let mut commit_messages = self.pending_commit_message_restores.clone();
        for repository in git_store.read(cx).repositories().values() {
            let repository = repository.read(cx);
            let work_directory_abs_path = repository
                .work_directory_abs_path
                .to_string_lossy()
                .into_owned();
            if active_work_directory_abs_path.as_deref() == Some(work_directory_abs_path.as_str()) {
                continue;
            }
            if let Some(buffer) = repository.commit_message_buffer() {
                let text = buffer.read(cx).text();
                if text.trim().is_empty() {
                    commit_messages.remove(&work_directory_abs_path);
                } else {
                    commit_messages.insert(
                        work_directory_abs_path,
                        SerializedCommitMessage {
                            message: Some(text),
                            original_message: None,
                            amend_pending: false,
                        },
                    );
                }
            }
        }
        if let Some(work_directory_abs_path) = active_work_directory_abs_path {
            let text = self.commit_message_buffer(cx).read(cx).text();
            let message = (!text.trim().is_empty()).then_some(text);
            let original_message = self.original_commit_message.clone();
            let amend_pending = self.amend_pending;
            if message.is_some() || original_message.is_some() || amend_pending {
                commit_messages.insert(
                    work_directory_abs_path,
                    SerializedCommitMessage {
                        message,
                        original_message,
                        amend_pending,
                    },
                );
            } else {
                commit_messages.remove(&work_directory_abs_path);
            }
        }
        commit_messages
    }

    pub(crate) fn set_modal_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.modal_open = open;
        cx.notify();
    }

    fn dispatch_context(&self, window: &mut Window, cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");

        if self.commit_editor.read(cx).is_focused(window) {
            dispatch_context.add("CommitEditor");
        } else if self.focus_handle.contains_focused(window, cx) {
            dispatch_context.add("menu");
            dispatch_context.add("ChangesList");
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
        if self.active_tab == GitPanelTab::History && self.focused_history_entry.is_some() {
            self.history_keyboard_nav = true;
            cx.notify();
        }
    }

    fn scroll_to_selected_entry(&mut self, cx: &mut Context<Self>) {
        let Some(selected_entry) = self.selected_entry else {
            cx.notify();
            return;
        };

        let visible_index = self
            .visible_entry_indices
            .iter()
            .position(|&ix| ix == selected_entry);

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
        let Some(selected_index) = self.selected_entry else {
            return;
        };
        let Some(entry) = self.entries.get(selected_index).cloned() else {
            return;
        };

        match entry {
            GitListEntry::ProjectRepositoriesHeader(entry) => {
                if entry.expanded {
                    self.select_next(&menu::SelectNext, window, cx);
                } else {
                    self.toggle_project_repositories(window, cx);
                }
            }
            GitListEntry::RepositoryHeader(entry) => {
                if entry.change_count == 0 || entry.expanded {
                    self.select_next(&menu::SelectNext, window, cx);
                } else {
                    self.toggle_repository(entry.repository_id, window, cx);
                }
            }
            GitListEntry::Header(entry) => {
                let Some(repository_id) = self.repository_id_for_entry_index(selected_index) else {
                    return;
                };
                if self
                    .collapsed_sections
                    .contains(&(repository_id, entry.header))
                {
                    self.toggle_section(repository_id, entry.header, window, cx);
                } else {
                    self.select_next(&menu::SelectNext, window, cx);
                }
            }
            GitListEntry::Directory(dir_entry) if dir_entry.expanded => {
                self.select_next(&menu::SelectNext, window, cx);
            }
            GitListEntry::Directory(dir_entry) => {
                self.toggle_directory(&dir_entry.key, window, cx);
            }
            _ => self.select_next(&menu::SelectNext, window, cx),
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(selected_index) = self.selected_entry else {
            return;
        };
        let Some(entry) = self.entries.get(selected_index).cloned() else {
            return;
        };

        match entry {
            GitListEntry::ProjectRepositoriesHeader(entry) if entry.expanded => {
                self.toggle_project_repositories(window, cx);
            }
            GitListEntry::RepositoryHeader(entry) if entry.change_count > 0 && entry.expanded => {
                self.toggle_repository(entry.repository_id, window, cx);
            }
            GitListEntry::Header(entry) => {
                let Some(repository_id) = self.repository_id_for_entry_index(selected_index) else {
                    return;
                };
                if self
                    .collapsed_sections
                    .contains(&(repository_id, entry.header))
                {
                    self.select_previous(&menu::SelectPrevious, window, cx);
                } else {
                    self.toggle_section(repository_id, entry.header, window, cx);
                }
            }
            GitListEntry::Directory(dir_entry) if dir_entry.expanded => {
                self.toggle_directory(&dir_entry.key, window, cx);
            }
            _ => self.select_previous(&menu::SelectPrevious, window, cx),
        }
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let first_entry = self.visible_entry_indices.iter().copied().find(|&ix| {
            self.entries
                .get(ix)
                .is_some_and(GitListEntry::is_selectable)
        });

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
        if self.active_tab == GitPanelTab::History {
            self.select_previous_history_entry(cx);
            return;
        }

        if self.visible_entry_indices.is_empty() {
            return;
        }

        let Some(selected_entry) = self.selected_entry else {
            return;
        };

        let Some(current_position) = self
            .visible_entry_indices
            .iter()
            .position(|&ix| ix == selected_entry)
        else {
            return;
        };
        let candidate = self.visible_entry_indices[..current_position]
            .iter()
            .rev()
            .copied()
            .find(|&ix| {
                self.entries
                    .get(ix)
                    .is_some_and(GitListEntry::is_selectable)
            });

        let Some(candidate) = candidate else {
            return;
        };

        self.selected_entry = Some(candidate);
        self.scroll_to_selected_entry(cx);
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        if self.active_tab == GitPanelTab::History {
            self.select_next_history_entry(cx);
            return;
        }

        if self.visible_entry_indices.is_empty() {
            return;
        }

        let Some(selected_entry) = self.selected_entry else {
            return;
        };

        let Some(current_position) = self
            .visible_entry_indices
            .iter()
            .position(|&ix| ix == selected_entry)
        else {
            return;
        };
        let candidate = self.visible_entry_indices[current_position + 1..]
            .iter()
            .copied()
            .find(|&ix| {
                self.entries
                    .get(ix)
                    .is_some_and(GitListEntry::is_selectable)
            });

        let Some(candidate) = candidate else {
            return;
        };

        self.selected_entry = Some(candidate);
        self.scroll_to_selected_entry(cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let last_entry = self
            .visible_entry_indices
            .iter()
            .rev()
            .copied()
            .find(|&ix| {
                self.entries
                    .get(ix)
                    .is_some_and(GitListEntry::is_selectable)
            });

        if let Some(last_entry) = last_entry {
            self.selected_entry = Some(last_entry);
            self.scroll_to_selected_entry(cx);
        }
    }

    /// Show diff view at selected entry, only if the diff view is open
    fn move_diff_to_entry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let workspace = self.workspace.upgrade()?;
            let selected_index = self.selected_entry?;
            let entry = self.entries.get(selected_index)?.status_entry()?.clone();
            let repository = self.repository_for_entry_index(selected_index, cx)?;
            let target =
                Self::diff_target_for_section(self.section_for_entry_index(selected_index));

            match target {
                DiffTarget::Staged => {
                    if let Some(staged_diff) = workspace.read(cx).item_of_type::<StagedDiff>(cx) {
                        staged_diff.update(cx, |staged_diff, cx| {
                            staged_diff.move_to_entry_in_repository(repository, entry, window, cx);
                        });
                    }
                }
                DiffTarget::Unstaged => {
                    if let Some(unstaged_diff) = workspace.read(cx).item_of_type::<UnstagedDiff>(cx)
                    {
                        unstaged_diff.update(cx, |unstaged_diff, cx| {
                            unstaged_diff
                                .move_to_entry_in_repository(repository, entry, window, cx);
                        });
                    }
                }
                DiffTarget::Uncommitted => {
                    if let Some(project_diff) = workspace.read(cx).item_of_type::<ProjectDiff>(cx) {
                        project_diff.update(cx, |project_diff, cx| {
                            project_diff.move_to_entry_in_repository(repository, entry, window, cx);
                        });
                    }
                }
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
        let have_entries = self.visible_entry_indices.iter().any(|&ix| {
            self.entries
                .get(ix)
                .is_some_and(GitListEntry::is_selectable)
        });
        if have_entries && self.selected_entry.is_none() {
            self.select_first(&menu::SelectFirst, window, cx);
        }
    }

    fn select_last_entry_if_out_of_bounds(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_entry
            && !self.is_entry_visible(idx)
        {
            self.select_last(&menu::SelectLast, window, cx);
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

    fn change_entries_by_path(&self) -> impl Iterator<Item = &GitStatusEntry> {
        let active_repository_id = self.active_repository_id;
        // A grouping can project one changed file into multiple list rows.
        self.entries
            .iter()
            .enumerate()
            .filter_map(move |(ix, entry)| {
                (self.repository_id_for_entry_index(ix) == active_repository_id)
                    .then(|| entry.status_entry())
                    .flatten()
            })
            .unique_by(|entry| entry.repo_path.clone())
    }

    fn change_entries_for_repository(
        &self,
        repository_id: RepositoryId,
    ) -> impl Iterator<Item = &GitStatusEntry> {
        let range = self
            .repository_entry_ranges
            .get(&repository_id)
            .cloned()
            .unwrap_or(0..0);
        self.entries[range]
            .iter()
            .filter_map(GitListEntry::status_entry)
            .unique_by(|entry| entry.repo_path.clone())
    }

    fn open_diff(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_tab == GitPanelTab::History {
            self.open_selected_history_commit(window, cx);
            return;
        }
        let Some(selected_index) = self.selected_entry else {
            return;
        };
        let Some(selected_entry) = self.entries.get(selected_index).cloned() else {
            return;
        };
        match selected_entry {
            GitListEntry::ProjectRepositoriesHeader(_) => {
                self.toggle_project_repositories(window, cx);
                return;
            }
            GitListEntry::RepositoryHeader(entry) => {
                self.activate_repository(entry.repository_id, cx);
                return;
            }
            GitListEntry::Header(entry) => {
                let Some(repository_id) = self.repository_id_for_entry_index(selected_index) else {
                    return;
                };
                let intent = self.stage_intent_for_entry_index(selected_index);
                self.toggle_staged_for_entry_in_repository(
                    &GitListEntry::Header(entry),
                    repository_id,
                    intent,
                    window,
                    cx,
                );
                return;
            }
            GitListEntry::Directory(entry) => {
                self.toggle_directory(&entry.key, window, cx);
                return;
            }
            GitListEntry::Status(_)
            | GitListEntry::TreeStatus(_)
            | GitListEntry::EmptySection(_) => {}
        }
        maybe!({
            let entry = self.entries.get(selected_index)?.status_entry()?;
            let workspace = self.workspace.upgrade()?;
            let repository = self.repository_for_entry_index(selected_index, cx)?;
            let repository_id = repository.read(cx).id;
            let target =
                Self::diff_target_for_section(self.section_for_entry_index(selected_index));

            if target == DiffTarget::Uncommitted
                && let Some(project_diff) = workspace.read(cx).active_item_as::<ProjectDiff>(cx)
                && project_diff
                    .read(cx)
                    .repo(cx)
                    .is_some_and(|repo| repo.read(cx).id == repository_id)
                && let Some(project_path) = project_diff.read(cx).active_project_path(cx)
                && Some(&entry.repo_path)
                    == repository
                        .read(cx)
                        .project_path_to_repo_path(&project_path, cx)
                        .as_ref()
            {
                project_diff.focus_handle(cx).focus(window, cx);
                project_diff.update(cx, |project_diff, cx| project_diff.autoscroll(cx));
                return None;
            };

            self.workspace
                .update(cx, |workspace, cx| match target {
                    DiffTarget::Uncommitted => {
                        ProjectDiff::deploy_at_in_repository(
                            workspace,
                            repository,
                            Some(entry.clone()),
                            window,
                            cx,
                        );
                    }
                    DiffTarget::Staged => {
                        StagedDiff::deploy_at_in_repository(
                            workspace,
                            repository,
                            Some(entry.clone()),
                            window,
                            cx,
                        );
                    }
                    DiffTarget::Unstaged => {
                        UnstagedDiff::deploy_at_in_repository(
                            workspace,
                            repository,
                            Some(entry.clone()),
                            window,
                            cx,
                        );
                    }
                })
                .ok();
            self.focus_handle.focus(window, cx);

            Some(())
        });
    }

    fn open_solo_diff(
        &mut self,
        _: &menu::SecondaryConfirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let selected_index = self.selected_entry?;
            let entry = self.entries.get(selected_index)?.status_entry()?.clone();
            let repository = self.repository_for_entry_index(selected_index, cx)?;

            SoloDiffView::open_or_focus(entry, repository, self.workspace.clone(), window, cx)
                .detach_and_notify_err(self.workspace.clone(), window, cx);

            Some(())
        });
    }

    fn view_file(&mut self, _: &ViewFile, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let selected_index = self.selected_entry?;
            let entry = self.entries.get(selected_index)?.status_entry()?;
            let project_path = self
                .repository_for_entry_index(selected_index, cx)?
                .read(cx)
                .repo_path_to_project_path(&entry.repo_path, cx)?;

            self.workspace
                .update(cx, |workspace, cx| {
                    workspace
                        .open_path_preview(project_path, None, false, false, true, window, cx)
                        .detach_and_log_err(cx);
                })
                .ok()?;

            Some(())
        });
    }

    fn open_selected_entry_on_click(
        &mut self,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entry_primary_click_action =
            GitPanelSettings::get_global(cx).entry_primary_click_action;
        let action = match (entry_primary_click_action, secondary) {
            (GitPanelClickBehavior::ProjectDiff, false) => GitPanelClickBehavior::ProjectDiff,
            (GitPanelClickBehavior::ProjectDiff, true) => GitPanelClickBehavior::FileDiff,
            (GitPanelClickBehavior::FileDiff, false) => GitPanelClickBehavior::FileDiff,
            (GitPanelClickBehavior::FileDiff, true) => GitPanelClickBehavior::ProjectDiff,
            (GitPanelClickBehavior::ViewFile, false) => GitPanelClickBehavior::ViewFile,
            (GitPanelClickBehavior::ViewFile, true) => GitPanelClickBehavior::ProjectDiff,
        };
        match action {
            GitPanelClickBehavior::ProjectDiff => {
                self.open_diff(&Default::default(), window, cx);
                self.focus_handle.focus(window, cx);
            }
            GitPanelClickBehavior::FileDiff => {
                self.open_solo_diff(&Default::default(), window, cx);
            }
            GitPanelClickBehavior::ViewFile => {
                self.view_file(&Default::default(), window, cx);
            }
        }
    }

    fn revert_selected(
        &mut self,
        action: &git::RestoreFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let path_style = self.project.read(cx).path_style(cx);
        maybe!({
            let selected_index = self.selected_entry?;
            let list_entry = self.entries.get(selected_index)?.clone();
            let entry = list_entry.status_entry()?.to_owned();
            let repository = self.repository_for_entry_index(selected_index, cx)?;
            let skip_prompt = action.skip_prompt || entry.status.is_created();

            let prompt = if skip_prompt {
                Task::ready(Ok(0))
            } else {
                let prompt = window.prompt(
                    PromptLevel::Warning,
                    &format!(
                        "Are you sure you want to discard changes to {}?",
                        MarkdownInlineCode(
                            entry
                                .repo_path
                                .file_name()
                                .unwrap_or(entry.repo_path.display(path_style).as_ref())
                        ),
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
                        this.revert_entry_in_repository(&entry, repository, window, cx);
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
            let selected_index = self.selected_entry?;
            let list_entry = self.entries.get(selected_index)?.clone();
            let entry = list_entry.status_entry()?.to_owned();

            if !entry.status.is_created() {
                return Some(());
            }

            let repository = self.repository_for_entry_index(selected_index, cx)?;
            let workspace = self.workspace.clone();
            let repo_path = entry.repo_path;

            let receiver =
                repository.update(cx, |repo, _| repo.add_path_to_gitignore(&repo_path, false));

            cx.spawn(async move |_, cx| {
                if let Err(e) = receiver.await? {
                    if let Some(workspace) = workspace.upgrade() {
                        cx.update(|cx| {
                            show_error_toast(workspace, "add to .gitignore", e, cx);
                        });
                    }
                }
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            Some(())
        });
    }

    fn add_to_git_info_exclude(
        &mut self,
        _: &git::AddToGitInfoExclude,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let selected_index = self.selected_entry?;
            let list_entry = self.entries.get(selected_index)?.clone();
            let entry = list_entry.status_entry()?.to_owned();

            if !entry.status.is_created() {
                return Some(());
            }

            let repository = self.repository_for_entry_index(selected_index, cx)?;
            let workspace = self.workspace.clone();
            let repo_path = entry.repo_path;

            let receiver = repository.update(cx, |repo, _| {
                repo.add_path_to_git_info_exclude(&repo_path, false)
            });

            cx.spawn(async move |_, cx| {
                if let Err(e) = receiver.await? {
                    if let Some(workspace) = workspace.upgrade() {
                        cx.update(|cx| {
                            show_error_toast(workspace, "add to .git/info/exclude", e, cx);
                        });
                    }
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
        let Some(repository) = self.active_repository.clone() else {
            return;
        };
        self.revert_entry_in_repository(entry, repository, window, cx);
    }

    fn revert_entry_in_repository(
        &mut self,
        entry: &GitStatusEntry,
        repository: Entity<Repository>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let path = repository
                .read(cx)
                .repo_path_to_project_path(&entry.repo_path, cx)?;
            let workspace = self.workspace.clone();

            if entry.status.staging().has_staged() {
                self.change_file_stage_for_repository(
                    repository.clone(),
                    false,
                    vec![entry.clone()],
                    cx,
                );
            }
            let filename = path.path.file_name()?.to_string();

            if !entry.status.is_created() {
                self.perform_checkout_in_repository(repository, vec![entry.clone()], window, cx);
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
                            .update(cx, |project, cx| project.trash_file(path, cx))
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
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };
        self.perform_checkout_in_repository(active_repository, entries, window, cx);
    }

    fn perform_checkout_in_repository(
        &mut self,
        repository: Entity<Repository>,
        entries: Vec<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();

        let task = cx.spawn_in(window, async move |this, cx| {
            let tasks: Vec<_> = workspace.update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    entries
                        .iter()
                        .filter_map(|entry| {
                            let path = repository
                                .read(cx)
                                .repo_path_to_project_path(&entry.repo_path, cx)?;
                            Some(project.open_buffer(path, cx))
                        })
                        .collect()
                })
            })?;

            let buffers = futures::future::join_all(tasks).await;

            this.update_in(cx, |this, window, cx| {
                let task = repository.update(cx, |repo, cx| {
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
            .change_entries_by_path()
            .filter(|status_entry| !status_entry.status.is_created())
            .cloned()
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
            .change_entries_by_path()
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
                            project.trash_file(project_path, cx)
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
                    .update(cx, |_this, cx| {
                        active_repository.update(cx, |repo, cx| {
                            if stage {
                                repo.stage_all(cx)
                            } else {
                                repo.unstage_all(cx)
                            }
                        })
                    })?
                    .await;

                this.update(cx, |this, cx| {
                    if let Err(err) = result {
                        this.show_error_toast(if stage { "add" } else { "reset" }, err, cx);
                    }
                    this.update_counts(active_repository.read(cx));
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
            .and_then(|ops| {
                // In case the last operation in the list of pending operations
                // failed, we can't assume the stage status for this entry and
                // need to fallback to the actual state in the repo.
                if ops.last_op_errored() {
                    return None;
                }

                if ops.staging() || ops.staged() {
                    Some(StageStatus::Staged)
                } else {
                    Some(StageStatus::Unstaged)
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

        let show_placeholders = self.show_placeholders
            && self.active_repository_id == Some(entry.key.repository_id)
            && !self.has_staged_changes();
        let mut fully_staged_count = 0usize;
        let mut any_staged_or_partially_staged = false;

        for descendant in descendants {
            if show_placeholders && !descendant.status.is_created() {
                fully_staged_count += 1;
                any_staged_or_partially_staged = true;
            } else {
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

    #[cfg(test)]
    fn toggle_staged_for_entry(
        &mut self,
        entry: &GitListEntry,
        intent: StageIntent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository_id) = self.active_repository_id else {
            return;
        };
        self.toggle_staged_for_entry_in_repository(entry, repository_id, intent, window, cx);
    }

    fn toggle_staged_for_entry_in_repository(
        &mut self,
        entry: &GitListEntry,
        repository_id: RepositoryId,
        intent: StageIntent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.repository_for_id(repository_id, cx) else {
            return;
        };
        let mut set_anchor: Option<RepoPath> = None;
        let mut clear_anchor = None;

        let (stage, repo_paths) = {
            let repo = repository.read(cx);
            match entry {
                GitListEntry::Status(status_entry)
                | GitListEntry::TreeStatus(GitTreeStatusEntry {
                    entry: status_entry,
                    ..
                }) => {
                    let repo_paths = vec![status_entry.clone()];
                    let stage = intent
                        .resolve_with(|| GitPanel::stage_status_for_entry(status_entry, &repo));

                    if stage {
                        set_anchor = Some(status_entry.repo_path.clone());
                    } else if let Some(op) = self.bulk_staging.clone()
                        && op.repo_id == repository_id
                        && op.anchor == status_entry.repo_path
                    {
                        clear_anchor = Some(op.anchor);
                    }
                    (stage, repo_paths)
                }
                GitListEntry::Header(section) => {
                    let goal_staged_state = match intent {
                        StageIntent::Stage => true,
                        StageIntent::Unstage => false,
                        StageIntent::Toggle => !self.header_state(section.header, repo).selected(),
                    };
                    let entries = self
                        .change_entries_for_repository(repository_id)
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
                    let goal_stage =
                        intent.resolve_with(|| self.stage_status_for_directory(entry, repo));
                    let goal_staged_state = if goal_stage {
                        StageStatus::Staged
                    } else {
                        StageStatus::Unstaged
                    };

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
                GitListEntry::RepositoryHeader(_)
                | GitListEntry::ProjectRepositoriesHeader(_)
                | GitListEntry::EmptySection(_) => return,
            }
        };
        if let Some(anchor) = clear_anchor {
            if let Some(op) = self.bulk_staging.clone()
                && op.repo_id == repository_id
                && op.anchor == anchor
            {
                self.bulk_staging = None;
            }
        }
        if let Some(anchor) = set_anchor {
            self.set_bulk_staging_anchor_for_repository(repository_id, anchor);
        }

        self.change_file_stage_for_repository(repository, stage, repo_paths, cx);
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
        self.change_file_stage_for_repository(active_repository, stage, entries, cx);
    }

    fn change_file_stage_for_repository(
        &mut self,
        repository: Entity<Repository>,
        stage: bool,
        entries: Vec<GitStatusEntry>,
        cx: &mut Context<Self>,
    ) {
        cx.spawn({
            async move |this, cx| {
                let result = this
                    .update(cx, |this, cx| {
                        let task = repository.update(cx, |repo, cx| {
                            let repo_paths = entries
                                .iter()
                                .map(|entry| entry.repo_path.clone())
                                .unique()
                                .collect();
                            if stage {
                                repo.stage_entries(repo_paths, cx)
                            } else {
                                repo.unstage_entries(repo_paths, cx)
                            }
                        });
                        if this.active_repository_id == Some(repository.read(cx).id) {
                            this.update_counts(repository.read(cx));
                        }
                        cx.notify();
                        task
                    })?
                    .await;

                this.update(cx, |this, cx| {
                    if let Err(err) = result {
                        this.show_error_toast(if stage { "add" } else { "reset" }, err, cx);
                        if this.active_repository_id == Some(repository.read(cx).id) {
                            this.update_counts(repository.read(cx));
                        }
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
        let Some(selected_index) = self.selected_entry else {
            return;
        };
        let Some(selected_entry) = self.entries.get(selected_index).cloned() else {
            return;
        };

        match &selected_entry {
            GitListEntry::ProjectRepositoriesHeader(_) => {
                self.toggle_project_repositories(window, cx);
                return;
            }
            GitListEntry::RepositoryHeader(entry) => {
                self.activate_repository(entry.repository_id, cx);
                return;
            }
            _ => {}
        }

        if self.is_resolved_conflict(selected_index, cx) {
            return;
        }

        let intent = self.stage_intent_for_entry_index(selected_index);
        let Some(repository_id) = self.repository_id_for_entry_index(selected_index) else {
            return;
        };
        self.toggle_staged_for_entry_in_repository(
            &selected_entry,
            repository_id,
            intent,
            window,
            cx,
        );
    }

    fn stage_range(&mut self, _: &git::StageRange, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(index) = self.selected_entry else {
            return;
        };
        let stage = self.stage_intent_for_entry_index(index) != StageIntent::Unstage;
        self.stage_bulk(index, stage, cx);
    }

    fn stage_selected(&mut self, _: &git::StageFile, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(selected_index) = self.selected_entry else {
            return;
        };
        let Some(selected_entry) = self.entries.get(selected_index) else {
            return;
        };
        let Some(status_entry) = selected_entry.status_entry() else {
            return;
        };
        if status_entry.staging != StageStatus::Staged {
            let Some(repository) = self.repository_for_entry_index(selected_index, cx) else {
                return;
            };
            self.change_file_stage_for_repository(repository, true, vec![status_entry.clone()], cx);
        }
    }

    fn unstage_selected(
        &mut self,
        _: &git::UnstageFile,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(selected_index) = self.selected_entry else {
            return;
        };
        let Some(selected_entry) = self.entries.get(selected_index) else {
            return;
        };
        let Some(status_entry) = selected_entry.status_entry() else {
            return;
        };
        if status_entry.staging != StageStatus::Unstaged {
            let Some(repository) = self.repository_for_entry_index(selected_index, cx) else {
                return;
            };
            self.change_file_stage_for_repository(
                repository,
                false,
                vec![status_entry.clone()],
                cx,
            );
        }
    }

    fn on_commit(&mut self, _: &Commit, window: &mut Window, cx: &mut Context<Self>) {
        let is_amend = self.amend_pending;
        if self.commit(&self.commit_editor.focus_handle(cx), window, cx) {
            if is_amend {
                telemetry::event!("Git Amended", source = "Git Panel");
            } else {
                telemetry::event!("Git Committed", source = "Git Panel");
            }
        }
    }

    /// Commits staged changes with the current commit message.
    /// When `amend_pending` is true, performs an amend commit instead.
    ///
    /// Returns `true` if the commit was executed, `false` otherwise.
    pub(crate) fn commit(
        &mut self,
        commit_editor_focus_handle: &FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if commit_editor_focus_handle.contains_focused(window, cx) {
            self.commit_changes(
                CommitOptions {
                    amend: self.amend_pending,
                    signoff: self.signoff_enabled,
                    allow_empty: false,
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

    fn on_amend(&mut self, _: &Amend, window: &mut Window, cx: &mut Context<Self>) {
        if self.amend(&self.commit_editor.focus_handle(cx), window, cx) {
            telemetry::event!("Git Amended", source = "Git Panel");
        }
    }

    /// Enters the amend state on first invocation, loading the last commit
    /// message for editing. On second invocation, performs the amend commit
    /// by delegating to [`Self::commit`]. Returns `true` if a commit was
    /// executed.
    pub(crate) fn amend(
        &mut self,
        commit_editor_focus_handle: &FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if commit_editor_focus_handle.contains_focused(window, cx) {
            if self.head_commit(cx).is_some() {
                if !self.amend_pending {
                    self.toggle_amend_pending(cx);
                } else {
                    return self.commit(commit_editor_focus_handle, window, cx);
                }
            }
            false
        } else {
            cx.propagate();
            false
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
            editor.rewrap(
                RewrapOptions {
                    override_language_settings: false,
                    preserve_existing_whitespace: true,
                    line_length: None,
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
            let prompt = window.prompt(PromptLevel::Warning, message, None, &["OK"], cx);
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
                .change_entries_by_path()
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
                            this.serialize(cx);
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
            .concat();

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
                    if let Ok(rel_path) = RelPath::from_unix_str(rules_name) {
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

    fn build_commit_message_prompt(
        prompt: &str,
        user_agents_md: Option<&str>,
        rules_content: Option<&str>,
        instructions: Option<&str>,
        subject: &str,
        diff_text: &str,
    ) -> String {
        let user_agents_md_section = match user_agents_md {
            Some(user_agents_md) => format!(
                "\n\nThe user has provided the following rules that you should follow when writing the commit message. Project-specific rules may override these instructions when they conflict:\n\
                <rules>\n{user_agents_md}\n</rules>\n"
            ),
            None => String::new(),
        };

        let rules_section = match rules_content {
            Some(rules) => format!(
                "\n\nThe user has provided the following rules specific to this project that you should follow when writing the commit message:\n\
                <project_rules>\n{rules}\n</project_rules>\n"
            ),
            None => String::new(),
        };

        let instructions_section = match instructions {
            Some(instructions) if !instructions.trim().is_empty() => format!(
                "\n\nThe user has provided the following instructions for writing commit messages that you should follow:\n\
                <commit_message_instructions>\n{instructions}\n</commit_message_instructions>\n"
            ),
            _ => String::new(),
        };

        let subject_section = if subject.trim().is_empty() {
            String::new()
        } else {
            format!("\nHere is the user's subject line:\n{subject}")
        };

        format!(
            "{prompt}{user_agents_md_section}{rules_section}{instructions_section}{subject_section}\nHere are the changes in this commit:\n{diff_text}"
        )
    }

    /// Generates a commit message using an LLM.
    pub fn generate_commit_message(&mut self, cx: &mut Context<Self>) {
        if !self.can_commit() || !AgentSettings::get_global(cx).enabled(cx) {
            return;
        }

        let Some(ConfiguredModel { provider, model }) =
            LanguageModelRegistry::read_global(cx).commit_message_model(cx)
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

        let include_project_rules =
            AgentSettings::get_global(cx).commit_message_include_project_rules;

        let instructions = AgentSettings::get_global(cx)
            .commit_message_instructions
            .clone();
        let project = self.project.clone();
        let repo_work_dir = repo.read(cx).work_directory_abs_path.clone();

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

                let rules_content = if include_project_rules {
                    Self::load_project_rules(&project, &repo_work_dir, &mut cx).await
                } else {
                    None
                };
                let user_agents_md = if include_project_rules {
                    cx.update(|cx| {
                        UserAgentsMd::global(cx)
                            .and_then(|user_agents_md| user_agents_md.content().cloned())
                    })
                } else {
                    None
                };

                let prompt = include_str!("../src/commit_message_prompt.txt");

                let subject = this.update(cx, |this, cx| {
                    this.commit_editor
                        .read(cx)
                        .text(cx)
                        .lines()
                        .next()
                        .map(ToOwned::to_owned)
                        .unwrap_or_default()
                })?;

                let text_empty = subject.trim().is_empty();

                let content = Self::build_commit_message_prompt(
                    &prompt,
                    user_agents_md.as_deref(),
                    rules_content.as_deref(),
                    instructions.as_deref(),
                    &subject,
                    &diff_text,
                );

                let request = LanguageModelRequest {
                    thread_id: None,
                    prompt_id: None,
                    intent: Some(CompletionIntent::GenerateGitCommitMessage),
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
                    thinking_effort: None,
                    speed: None,
                    compact_at_tokens: None,
                };

                let stream = model.stream_completion_text(request, cx);
                match stream.await {
                    Ok(mut messages) => {
                        if !text_empty {
                            this.update(cx, |this, cx| {
                                this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                                    let insert_position = buffer.anchor_before(buffer.len());
                                    buffer.edit(
                                        [(insert_position..insert_position, "\n")],
                                        None,
                                        cx,
                                    )
                                });
                            })?;
                        }

                        while let Some(message) = messages.stream.next().await {
                            match message {
                                Ok(text) => {
                                    this.update(cx, |this, cx| {
                                        this.commit_message_buffer(cx).update(cx, |buffer, cx| {
                                            let insert_position =
                                                buffer.anchor_before(buffer.len());
                                            buffer.edit(
                                                [(insert_position..insert_position, text)],
                                                None,
                                                cx,
                                            );
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
            .log_err()
            .await
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
        if !self.start_remote_operation(RemoteOperationKind::Fetch, cx) {
            return;
        }

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
                let _clear_pending_remote_operation = cx.on_drop(&this, |this, cx| {
                    this.clear_remote_operation(cx);
                });

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
                &["OK"],
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
        let Some(branch) = repo.read(cx).branch.clone() else {
            return;
        };
        if !self.start_remote_operation(RemoteOperationKind::Pull, cx) {
            return;
        }

        telemetry::event!("Git Pulled");
        let remote = self.get_remote(false, false, window, cx);
        cx.spawn_in(window, async move |this, cx| {
            let _clear_pending_remote_operation = cx.on_drop(&this, |this, cx| {
                this.clear_remote_operation(cx);
            });

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
        let Some(branch) = repo.read(cx).branch.clone() else {
            return;
        };
        if !self.start_remote_operation(RemoteOperationKind::Push, cx) {
            return;
        }

        telemetry::event!("Git Pushed");

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
            let _clear_pending_remote_operation = cx.on_drop(&this, |this, cx| {
                this.clear_remote_operation(cx);
            });

            let remote = match remote.await {
                Ok(Some(remote)) => remote,
                Ok(None) => {
                    this.update(cx, |this, cx| {
                        this.show_error_toast(
                            "push",
                            anyhow::anyhow!("No remote available to push to. Add a remote to be able to publish changes."),
                            cx,
                        )
                    })
                    .ok();
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

    /// Updates git's configuration, adding the directory of the current
    /// worktree to the `safe.directory` config, ensuring that, even if the user
    /// that's running the application is not the owner of `.git/`, it can still
    /// read the repository's contents.
    fn add_safe_directory(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_repository) = &self.active_repository else {
            return;
        };

        let path = active_repository.update(cx, |repository, _cx| {
            repository.snapshot().work_directory_abs_path
        });

        if let Some(path_str) = path.to_str() {
            let path_arg = String::from(path_str);
            let args = vec![
                String::from("--global"),
                String::from("--add"),
                String::from("safe.directory"),
                path_arg,
            ];

            self.project
                .read(cx)
                .git_config(path, args, cx)
                .detach_and_log_err(cx);
        }
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
        let workspace = self.workspace.clone();
        let operation = operation.into();
        let window = window.window_handle();
        AskPassDelegate::new(&mut cx.to_async(), move |prompt, tx, cx| {
            window
                .update(cx, |_, window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            AskPassModal::new(operation.clone(), prompt.into(), tx, window, cx)
                        });
                    })
                })
                .ok();
        })
    }

    fn can_push_and_pull(&self, cx: &App) -> bool {
        !self.project.read(cx).is_via_collab()
    }

    fn start_remote_operation(
        &mut self,
        kind: RemoteOperationKind,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.pending_remote_operation.is_some() {
            return false;
        }

        self.pending_remote_operation = Some(kind);
        cx.notify();
        true
    }

    fn clear_remote_operation(&mut self, cx: &mut Context<Self>) {
        self.pending_remote_operation.take();
        cx.notify();
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

    #[cfg(not(feature = "call"))]
    fn potential_co_authors(&self, _cx: &App) -> Vec<(String, String)> {
        Vec::new()
    }

    #[cfg(feature = "call")]
    fn potential_co_authors(&self, cx: &App) -> Vec<(String, String)> {
        let mut new_co_authors = Vec::new();
        let project = self.project.read(cx);

        let Some(room) =
            call::ActiveCall::try_global(cx).and_then(|call| call.read(cx).room().cloned())
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
                    .unwrap_or_else(|| participant.user.username.clone().to_string());
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

    #[cfg(feature = "call")]
    fn local_committer(&self, room: &call::Room, cx: &App) -> Option<(String, String)> {
        let user = room.local_participant_user(cx)?;
        let committer = self.local_committer.as_ref()?;
        let email = committer.email.clone()?;
        let name = committer
            .name
            .clone()
            .or_else(|| user.name.clone())
            .unwrap_or_else(|| user.username.clone().to_string());
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

    fn set_sort_by_path(&mut self, _: &SetSortByPath, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            let fs = workspace.app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings.git_panel.get_or_insert_default().sort_by = Some(GitPanelSortBy::Path);
                });
            });
        }
    }

    fn set_sort_by_name(&mut self, _: &SetSortByName, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            let fs = workspace.app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings.git_panel.get_or_insert_default().sort_by = Some(GitPanelSortBy::Name);
                });
            });
        }
    }

    fn set_group_by_none(&mut self, _: &SetGroupByNone, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            let fs = workspace.app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::None);
                });
            });
        }
    }

    fn set_group_by_status(
        &mut self,
        _: &SetGroupByStatus,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            let fs = workspace.app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Status);
                });
            });
        }
    }

    fn view_staged_changes(
        &mut self,
        _: &ViewStagedChanges,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected = self.selected_entry.and_then(|index| {
            Some((
                self.entries
                    .get(index)
                    .and_then(|entry| entry.status_entry())
                    .cloned(),
                self.repository_for_entry_index(index, cx)?,
            ))
        });
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                if let Some((entry, repository)) = selected {
                    StagedDiff::deploy_at_in_repository(workspace, repository, entry, window, cx);
                } else {
                    StagedDiff::deploy_at(workspace, None, window, cx);
                }
            });
        }
    }

    fn view_unstaged_changes(
        &mut self,
        _: &ViewUnstagedChanges,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected = self.selected_entry.and_then(|index| {
            Some((
                self.entries
                    .get(index)
                    .and_then(|entry| entry.status_entry())
                    .cloned(),
                self.repository_for_entry_index(index, cx)?,
            ))
        });
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                if let Some((entry, repository)) = selected {
                    UnstagedDiff::deploy_at_in_repository(workspace, repository, entry, window, cx);
                } else {
                    UnstagedDiff::deploy_at(workspace, None, window, cx);
                }
            });
        }
    }

    fn set_group_by_staging(
        &mut self,
        _: &SetGroupByStaging,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            let fs = workspace.app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Staging);
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

    fn show_current_repository(
        &mut self,
        _: &ShowCurrentRepository,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_show_all_repositories(false, cx);
    }

    fn show_all_repositories(
        &mut self,
        _: &ShowAllRepositories,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_show_all_repositories(true, cx);
    }

    fn set_show_all_repositories(&mut self, show_all: bool, cx: &mut Context<Self>) {
        if GitPanelSettings::get_global(cx).show_all_repositories == show_all {
            return;
        }
        if let Some(workspace) = self.workspace.upgrade() {
            let fs = workspace.read(cx).app_state().fs.clone();
            cx.update_global::<SettingsStore, _>(|store, _cx| {
                store.update_settings_file(fs, move |settings, _cx| {
                    settings
                        .git_panel
                        .get_or_insert_default()
                        .show_all_repositories = Some(show_all);
                });
            });
        }
    }

    pub(crate) fn increase_font_size(
        &mut self,
        action: &IncreaseBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_font_size_action(action.persist, px(1.0), cx);
    }

    pub(crate) fn decrease_font_size(
        &mut self,
        action: &DecreaseBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_font_size_action(action.persist, px(-1.0), cx);
    }

    fn handle_font_size_action(&mut self, persist: bool, delta: Pixels, cx: &mut Context<Self>) {
        if persist {
            update_settings_file(self.fs.clone(), cx, move |settings, cx| {
                let git_commit_buffer_font_size =
                    ThemeSettings::get_global(cx).git_commit_buffer_font_size(cx) + delta;

                let _ = settings.theme.git_commit_buffer_font_size.insert(
                    f32::from(theme_settings::clamp_font_size(git_commit_buffer_font_size)).into(),
                );
            });
        } else {
            theme_settings::adjust_git_commit_buffer_font_size(cx, |size| size + delta);
        }
    }

    pub(crate) fn reset_font_size(
        &mut self,
        action: &ResetBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if action.persist {
            update_settings_file(self.fs.clone(), cx, move |settings, _| {
                settings.theme.git_commit_buffer_font_size = None;
            });
        } else {
            theme_settings::reset_git_commit_buffer_font_size(cx);
        }
    }

    fn toggle_directory(&mut self, key: &TreeKey, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(state) = self.view_mode.tree_state_mut() {
            let expanded = state.expanded_dirs.entry(key.clone()).or_insert(true);
            *expanded = !*expanded;
            self.tree_expanded_dirs = state.expanded_dirs.clone();
            self.update_visible_entries(window, cx);
        } else {
            util::debug_panic!("Attempted to toggle directory in flat Git Panel state");
        }
    }

    fn toggle_project_repositories(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.project_repositories_expanded = !self.project_repositories_expanded;
        self.update_visible_entries(window, cx);
    }

    fn toggle_repository(
        &mut self,
        repository_id: RepositoryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.collapsed_repositories.insert(repository_id) {
            self.collapsed_repositories.remove(&repository_id);
        }
        self.update_visible_entries(window, cx);
    }

    fn toggle_section(
        &mut self,
        repository_id: RepositoryId,
        section: Section,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = (repository_id, section);
        if !self.collapsed_sections.insert(key) {
            self.collapsed_sections.remove(&key);
        }
        self.update_visible_entries(window, cx);
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
        let new_active_repository = self.project.read(cx).active_repository(cx);
        let new_active_repository_id = new_active_repository
            .as_ref()
            .map(|repository| repository.read(cx).id);
        let active_repository_changed = self.active_repository.as_ref().map(Entity::entity_id)
            != new_active_repository.as_ref().map(Entity::entity_id);
        if active_repository_changed {
            if self.amend_pending {
                // Leaving a repository with a pending amend: undo it so the amend
                // state doesn't carry over to the newly active repository. The
                // commit editor still holds the previous repository's buffer here
                // (`reopen_commit_buffer` swaps it asynchronously below), so this
                // restores the pre-amend draft into that repository's buffer.
                self.set_amend_pending(false, cx);
            }
            self.git_access = None;
            self._repo_subscriptions.clear();
            if self.active_tab == GitPanelTab::History {
                self.set_commit_history(CommitHistory::Loading, cx);
            }
        }
        self.active_repository = new_active_repository;
        self.active_repository_id = new_active_repository_id;
        self.reopen_commit_buffer(window, cx);
        self.preload_commit_history(cx);
        if self.active_tab == GitPanelTab::History {
            self.load_commit_history(cx);
        }
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
            self.reopen_commit_buffer_task = Task::ready(());
            return;
        };
        let active_repository_abs_path = active_repo
            .read(cx)
            .work_directory_abs_path
            .to_string_lossy()
            .into_owned();
        let load_buffer = active_repo.update(cx, |active_repo, cx| {
            let project = self.project.read(cx);
            active_repo.open_commit_buffer(
                Some(project.languages().clone()),
                project.buffer_store().clone(),
                cx,
            )
        });
        let load_template = self.load_commit_template(cx);

        self.reopen_commit_buffer_task = cx.spawn_in(window, async move |git_panel, cx| {
            let result = async {
                let buffer = load_buffer.await?;
                let template = load_template.await?;

                git_panel.update_in(cx, move |git_panel, window, cx| {
                    git_panel.commit_template = template;
                    let restored_commit_message = git_panel
                        .pending_commit_message_restores
                        .remove(&active_repository_abs_path);
                    if let Some(restored_commit_message) = restored_commit_message {
                        git_panel.amend_pending = restored_commit_message.amend_pending;
                        git_panel.original_commit_message =
                            restored_commit_message.original_message;
                        cx.notify();
                        if let Some(message) = restored_commit_message.message
                            && buffer.read(cx).text().trim().is_empty()
                        {
                            buffer.update(cx, |buffer, cx| {
                                let start = buffer.anchor_before(0);
                                let end = buffer.anchor_after(buffer.len());
                                buffer.edit([(start..end, message)], None, cx);
                            });
                        }
                    }
                    if buffer.read(cx).text().trim().is_empty() {
                        let template_text = git_panel
                            .commit_template
                            .as_ref()
                            .map(|t| t.template.clone())
                            .unwrap_or_default();
                        if !template_text.is_empty() {
                            buffer.update(cx, |buffer, cx| {
                                let start = buffer.anchor_before(0);
                                let end = buffer.anchor_after(buffer.len());
                                buffer.edit([(start..end, template_text)], None, cx);
                            });
                        }
                    }

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
                                buffer.clone(),
                                git_panel.suggest_commit_message(cx).map(SharedString::from),
                                git_panel.project.clone(),
                                true,
                                window,
                                cx,
                            )
                        });
                    }

                    git_panel._commit_message_buffer_subscription =
                        Some(cx.subscribe(&buffer, |this, _, event, cx| {
                            if matches!(event, BufferEvent::Edited { .. }) {
                                this.serialize(cx);
                            }
                        }));
                })?;
                anyhow::Ok(())
            }
            .await;
            result.log_err();
        });
    }

    fn update_visible_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let path_style = self.project.read(cx).path_style(cx);
        let selected_change = self.selected_entry.and_then(|index| {
            let entry = self.entries.get(index)?.status_entry()?;
            Some((
                ChangeKey {
                    repository_id: self.repository_id_for_entry_index(index)?,
                    repo_path: entry.repo_path.clone(),
                },
                self.section_for_entry_index(index),
            ))
        });
        let selected_directory = self.selected_entry.and_then(|index| {
            self.entries
                .get(index)?
                .directory_entry()
                .map(|entry| entry.key.clone())
        });
        let selected_repository = self.selected_entry.and_then(|index| {
            let GitListEntry::RepositoryHeader(entry) = self.entries.get(index)? else {
                return None;
            };
            Some(entry.repository_id)
        });
        let selected_project_repositories = self.selected_entry.is_some_and(|index| {
            matches!(
                self.entries.get(index),
                Some(GitListEntry::ProjectRepositoriesHeader(_))
            )
        });
        let selected_section = self.selected_entry.and_then(|index| {
            let GitListEntry::Header(entry) = self.entries.get(index)? else {
                return None;
            };
            Some((self.repository_id_for_entry_index(index)?, entry.header))
        });
        let had_selected_identity = selected_change.is_some()
            || selected_directory.is_some()
            || selected_repository.is_some()
            || selected_project_repositories
            || selected_section.is_some();
        let bulk_staging = self.bulk_staging.take();
        let last_staged_path_prev_index = bulk_staging.as_ref().and_then(|op| {
            self.entry_by_change_key(&ChangeKey {
                repository_id: op.repo_id,
                repo_path: op.anchor.clone(),
            })
            .and_then(|ix| self.repository_local_entry_index(ix, op.repo_id))
        });

        self.entries.clear();
        self.visible_entry_indices.clear();
        self.entry_repository_ids.clear();
        self.repository_entry_ranges.clear();
        self.project_repository_depths.clear();
        self.projected_entries_by_path.clear();
        self.single_staged_entry.take();
        self.single_tracked_entry.take();
        self.conflicted_count = 0;
        self.conflicted_staged_count = 0;
        self.changes_count = 0;
        self.active_changes_count = 0;
        self.diff_stat_total = DiffStat::default();
        self.new_count = 0;
        self.tracked_count = 0;
        self.new_staged_count = 0;
        self.tracked_staged_count = 0;
        self.entry_count = 0;
        self.max_width_item_index = None;

        let settings = GitPanelSettings::get_global(cx);
        let sort_by = settings.sort_by;
        let group_by = settings.group_by;
        let group_by_file_status = group_by == GitPanelGroupBy::Status;
        let group_by_staging_state = group_by == GitPanelGroupBy::Staging;
        let show_all_repositories = settings.show_all_repositories;

        if let Some(active_repo) = self.active_repository.as_ref() {
            if self.git_access.is_none() {
                let access = active_repo.update(cx, |active_repo, cx| active_repo.access(cx));

                cx.spawn_in(window, async move |git_panel, cx| {
                    // When the user does not own the `.git` folder, the
                    // `GitStore.spawn_local_git_worker` will fail to create the
                    // receiver for Git jobs, so this access check will be
                    // cancelled.
                    //
                    // We assume `GitAccess::No` on cancellation. I believe this is
                    // imprecise, other failures could also cause cancellation, but
                    // the consequence is just showing the "unsafe repo" UI, which
                    // seems acceptable for this edge case.
                    let access = match access.await {
                        Ok(access) => access,
                        Err(Canceled) => GitAccess::No,
                    };

                    git_panel.update(cx, |this, _cx| {
                        this.git_access = Some(access);
                    })
                })
                .detach_and_log_err(cx);
            }
        }

        let Some(active_repository) = self.active_repository.clone() else {
            // Just clear entries if no repository is active.
            cx.notify();
            return;
        };
        let active_repository_id = active_repository.read(cx).id;
        self.active_repository_id = Some(active_repository_id);
        self.stash_entries = active_repository.read(cx).cached_stash();

        let git_store = self.project.read(cx).git_store().clone();
        let mut repositories = if show_all_repositories {
            git_store
                .read(cx)
                .repositories()
                .values()
                .cloned()
                .collect::<Vec<_>>()
        } else {
            vec![active_repository.clone()]
        };
        let root_repositories = if show_all_repositories {
            crate::worktree_service::classify_worktrees(self.project.read(cx), cx).0
        } else {
            Vec::new()
        };
        let primary_repository_id = root_repositories
            .first()
            .map(|repository| repository.read(cx).id);
        let repository_by_id = repositories
            .iter()
            .map(|repository| (repository.read(cx).id, repository.clone()))
            .collect::<HashMap<_, _>>();
        self.collapsed_repositories
            .retain(|repository_id| repository_by_id.contains_key(repository_id));
        self.collapsed_sections
            .retain(|(repository_id, _)| repository_by_id.contains_key(repository_id));
        let repository_snapshots = repositories
            .iter()
            .map(|repository| {
                let repository = repository.read(cx);
                (repository.id, repository.snapshot())
            })
            .collect::<HashMap<_, _>>();
        let mut parent_by_repository_id = HashMap::default();

        if show_all_repositories {
            let repository_id_by_work_directory = repository_snapshots
                .iter()
                .map(|(repository_id, snapshot)| {
                    (snapshot.work_directory_abs_path.as_ref(), *repository_id)
                })
                .collect::<HashMap<&Path, RepositoryId>>();

            for (child_id, child) in &repository_snapshots {
                let mut ancestor = child.work_directory_abs_path.parent();
                while let Some(ancestor_path) = ancestor {
                    if let Some(parent_id) = repository_id_by_work_directory.get(ancestor_path) {
                        let parent = &repository_snapshots[parent_id];
                        if child.modern_submodule_path_in(parent).is_some() {
                            parent_by_repository_id.insert(*child_id, *parent_id);
                            break;
                        }
                    }
                    ancestor = ancestor_path.parent();
                }
            }

            // If a submodule itself is the opened project root, it is the primary
            // repository for this workspace rather than a child of an invisible
            // superproject.
            if let Some(primary_repository_id) = primary_repository_id {
                parent_by_repository_id.remove(&primary_repository_id);
            }

            let root_by_repository_id: HashMap<RepositoryId, RepositoryId> = repository_snapshots
                .keys()
                .map(|repository_id| {
                    (
                        *repository_id,
                        root_repository_id(*repository_id, &parent_by_repository_id),
                    )
                })
                .collect();

            let root_rank = root_repositories
                .iter()
                .enumerate()
                .map(|(rank, repository)| {
                    let repository_id = repository.read(cx).id;
                    (root_by_repository_id[&repository_id], rank)
                })
                .collect::<HashMap<_, _>>();

            repositories.sort_by(|left, right| {
                let left_id = left.read(cx).id;
                let right_id = right.read(cx).id;
                let left_root_id = root_by_repository_id[&left_id];
                let right_root_id = root_by_repository_id[&right_id];
                let left_snapshot = &repository_snapshots[&left_id];
                let right_snapshot = &repository_snapshots[&right_id];
                let left_root_snapshot = &repository_snapshots[&left_root_id];
                let right_root_snapshot = &repository_snapshots[&right_root_id];
                let left_relative_path = left_snapshot
                    .work_directory_abs_path
                    .strip_prefix(left_root_snapshot.work_directory_abs_path.as_ref())
                    .unwrap_or(left_snapshot.work_directory_abs_path.as_ref());
                let right_relative_path = right_snapshot
                    .work_directory_abs_path
                    .strip_prefix(right_root_snapshot.work_directory_abs_path.as_ref())
                    .unwrap_or(right_snapshot.work_directory_abs_path.as_ref());

                (Some(left_root_id) != primary_repository_id)
                    .cmp(&(Some(right_root_id) != primary_repository_id))
                    .then_with(|| {
                        root_rank
                            .get(&left_root_id)
                            .copied()
                            .unwrap_or(usize::MAX)
                            .cmp(&root_rank.get(&right_root_id).copied().unwrap_or(usize::MAX))
                    })
                    .then_with(|| {
                        left_root_snapshot
                            .work_directory_abs_path
                            .cmp(&right_root_snapshot.work_directory_abs_path)
                    })
                    .then_with(|| left_relative_path.cmp(right_relative_path))
                    .then_with(|| left_id.cmp(&right_id))
            });
        }

        if let Some(primary_repository_id) = primary_repository_id {
            for repository_id in repository_by_id.keys().copied() {
                if let Some(depth) = repository_depth_below_root(
                    repository_id,
                    primary_repository_id,
                    &parent_by_repository_id,
                ) {
                    self.project_repository_depths.insert(repository_id, depth);
                }
            }
        }

        let project_repository_count = self.project_repository_depths.len();
        let project_repositories_contain_active = self
            .project_repository_depths
            .contains_key(&active_repository_id);

        let mut seen_directories = HashSet::default();
        let mut max_width_estimate = 0usize;
        let mut max_width_item_index = None;
        let mut tree_state = match std::mem::replace(&mut self.view_mode, GitPanelViewMode::Flat) {
            GitPanelViewMode::Flat => None,
            GitPanelViewMode::Tree(mut state) => {
                state.directory_descendants.clear();
                Some(state)
            }
        };
        let is_tree_view = tree_state.is_some();

        let mut push_entry = |this: &mut Self,
                              repository_id: RepositoryId,
                              entry: GitListEntry,
                              section: Section,
                              is_visible: bool| {
            if is_visible
                && let Some(estimate) =
                    this.width_estimate_for_list_entry(is_tree_view, &entry, path_style)
            {
                // Tree depth is already included by `width_estimate_for_list_entry`;
                // account here for the repository and section hierarchy around it.
                let estimate = estimate
                    + (this.project_repository_depth(repository_id)
                        + entry.repository_hierarchy_depth())
                    .saturating_mul(2);
                if estimate > max_width_estimate {
                    max_width_estimate = estimate;
                    max_width_item_index = Some(this.visible_entry_indices.len());
                }
            }

            if let Some(repo_path) = entry.status_entry().map(|status| status.repo_path.clone()) {
                this.projected_entries_by_path
                    .entry(ChangeKey {
                        repository_id,
                        repo_path,
                    })
                    .or_default()
                    .push(ProjectedChangeEntry {
                        section,
                        index: this.entries.len(),
                    });
            }

            if is_visible {
                this.visible_entry_indices.push(this.entries.len());
            }

            this.entry_repository_ids.push(repository_id);
            this.entries.push(entry);
        };

        let mut project_repositories_header_index = None;
        let mut project_repositories_change_count = 0;
        for repository in repositories {
            let repo = repository.read(cx);
            let repository_id = repo.id;
            let project_repository_depth = self.project_repository_depth(repository_id);
            let is_project_repository = project_repository_depth > 0;
            let repository_is_visible =
                !is_project_repository || self.project_repositories_expanded;
            let repository_contents_are_visible =
                repository_is_visible && !self.collapsed_repositories.contains(&repository_id);

            if is_project_repository && project_repositories_header_index.is_none() {
                let Some(primary_repository_id) = primary_repository_id else {
                    unreachable!("project repositories require a primary repository")
                };
                project_repositories_header_index = Some(self.entries.len());
                push_entry(
                    self,
                    primary_repository_id,
                    GitListEntry::ProjectRepositoriesHeader(GitProjectRepositoriesEntry {
                        repository_count: project_repository_count,
                        change_count: 0,
                        expanded: self.project_repositories_expanded,
                        contains_active_repository: project_repositories_contain_active,
                    }),
                    Section::Tracked,
                    true,
                );
            }
            let repository_entries_start = self.entries.len();
            let mut changed_entries = Vec::new();
            let mut new_entries = Vec::new();
            let mut conflict_entries = Vec::new();
            let mut staged_entries = Vec::new();
            let mut unstaged_entries = Vec::new();
            let mut tracked_entries = Vec::new();
            let mut single_staged_entry = None;
            let mut staged_count = 0;
            let mut repository_change_count = 0;

            for entry in repo.cached_status() {
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
                repository_change_count += 1;

                let entry = GitStatusEntry {
                    repo_path: entry.repo_path.clone(),
                    status: entry.status,
                    staging,
                    diff_stat: entry.diff_stat,
                };

                if !is_conflict && !is_new {
                    tracked_entries.push(entry.clone());
                }

                if staging.has_staged() {
                    staged_count += 1;
                    single_staged_entry = Some(entry.clone());
                }

                if group_by_staging_state && is_conflict {
                    conflict_entries.push(entry);
                } else if group_by_staging_state {
                    if staging.has_staged() {
                        staged_entries.push(entry.clone());
                    }
                    if staging.has_unstaged() {
                        unstaged_entries.push(entry);
                    }
                } else if group_by_file_status && is_conflict {
                    conflict_entries.push(entry);
                } else if group_by_file_status && is_new {
                    new_entries.push(entry);
                } else {
                    changed_entries.push(entry);
                }
            }

            self.changes_count += repository_change_count;
            if is_project_repository {
                project_repositories_change_count += repository_change_count;
            }
            if repository_id == active_repository_id {
                self.active_changes_count = repository_change_count;
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
                                    diff_stat: status.diff_stat,
                                });
                    }
                }

                if tracked_entries.len() == 1 {
                    self.single_tracked_entry = tracked_entries.pop();
                }
            }

            if !is_tree_view {
                let sort_entries = |entries: &mut Vec<GitStatusEntry>| match sort_by {
                    GitPanelSortBy::Path => entries.sort_by(|a, b| a.repo_path.cmp(&b.repo_path)),
                    GitPanelSortBy::Name => entries.sort_by(|a, b| {
                        a.repo_path
                            .file_name()
                            .cmp(&b.repo_path.file_name())
                            .then_with(|| a.repo_path.cmp(&b.repo_path))
                    }),
                };

                sort_entries(&mut conflict_entries);
                sort_entries(&mut changed_entries);
                sort_entries(&mut new_entries);
                sort_entries(&mut staged_entries);
                sort_entries(&mut unstaged_entries);
            }

            let section_entries = if group_by_staging_state {
                vec![
                    (Section::Conflict, conflict_entries),
                    (Section::Staged, staged_entries),
                    (Section::Unstaged, unstaged_entries),
                ]
            } else {
                vec![
                    (Section::Conflict, conflict_entries),
                    (Section::Tracked, changed_entries),
                    (Section::New, new_entries),
                ]
            };

            let has_any_section_entries = section_entries
                .iter()
                .any(|(_, entries)| !entries.is_empty());

            if show_all_repositories {
                const MAX_SHORT_SHA_LEN: usize = 8;
                let kind = if Some(repository_id) == primary_repository_id {
                    GitRepositoryKind::Primary
                } else if parent_by_repository_id.contains_key(&repository_id) {
                    GitRepositoryKind::Submodule
                } else {
                    GitRepositoryKind::Repository
                };
                let parent_display_name = parent_by_repository_id
                    .get(&repository_id)
                    .and_then(|parent_id| repository_by_id.get(parent_id))
                    .map(|parent| parent.read(cx).display_name());
                let branch_label = repo
                    .branch
                    .as_ref()
                    .map(|branch| branch.name().to_owned())
                    .or_else(|| {
                        repo.head_commit.as_ref().map(|commit| {
                            commit
                                .sha
                                .chars()
                                .take(MAX_SHORT_SHA_LEN)
                                .collect::<String>()
                        })
                    })
                    .unwrap_or_else(|| "(no branch)".to_owned());
                push_entry(
                    self,
                    repository_id,
                    GitListEntry::RepositoryHeader(GitRepositoryHeaderEntry {
                        repository_id,
                        display_name: repo.display_name(),
                        work_directory: repo
                            .work_directory_abs_path
                            .to_string_lossy()
                            .into_owned()
                            .into(),
                        branch_label: branch_label.into(),
                        kind,
                        parent_display_name,
                        change_count: repository_change_count,
                        is_active: repository_id == active_repository_id,
                        expanded: !self.collapsed_repositories.contains(&repository_id),
                    }),
                    Section::Tracked,
                    repository_is_visible,
                );
            }

            if !has_any_section_entries {
                if self.entries.len() > repository_entries_start {
                    self.repository_entry_ranges
                        .insert(repository_id, repository_entries_start..self.entries.len());
                }
                continue;
            }

            let show_when_empty = |section: Section| {
                group_by_staging_state
                    && has_any_section_entries
                    && matches!(section, Section::Staged | Section::Unstaged)
            };

            for (section, entries) in section_entries {
                if entries.is_empty() && !show_when_empty(section) {
                    continue;
                }

                let section_has_header =
                    section != Section::Tracked || group_by != GitPanelGroupBy::None;
                let section_is_visible = repository_contents_are_visible
                    && (!section_has_header
                        || !self.collapsed_sections.contains(&(repository_id, section)));

                if section_has_header {
                    push_entry(
                        self,
                        repository_id,
                        GitListEntry::Header(GitHeaderEntry { header: section }),
                        section,
                        repository_contents_are_visible,
                    );
                }

                if entries.is_empty() {
                    push_entry(
                        self,
                        repository_id,
                        GitListEntry::EmptySection(section),
                        section,
                        section_is_visible,
                    );
                    continue;
                }

                if let Some(state) = tree_state.as_mut() {
                    let tree_entries = state.build_tree_entries(
                        repository_id,
                        section,
                        entries,
                        &mut seen_directories,
                    );
                    for (entry, is_visible) in tree_entries {
                        push_entry(
                            self,
                            repository_id,
                            entry,
                            section,
                            section_is_visible && is_visible,
                        );
                    }
                } else {
                    for entry in entries {
                        push_entry(
                            self,
                            repository_id,
                            GitListEntry::Status(entry),
                            section,
                            section_is_visible,
                        );
                    }
                }
            }

            if self.entries.len() > repository_entries_start {
                self.repository_entry_ranges
                    .insert(repository_id, repository_entries_start..self.entries.len());
            }
        }

        if let Some(header_index) = project_repositories_header_index
            && let Some(GitListEntry::ProjectRepositoriesHeader(header)) =
                self.entries.get_mut(header_index)
        {
            header.change_count = project_repositories_change_count;
        }

        if let Some(mut state) = tree_state {
            state
                .expanded_dirs
                .retain(|key, _| seen_directories.contains(key));
            self.tree_expanded_dirs = state.expanded_dirs.clone();
            self.view_mode = GitPanelViewMode::Tree(state);
        }

        self.max_width_item_index = max_width_item_index;

        self.update_counts(active_repository.read(cx));

        let bulk_staging_anchor_new_global_index = bulk_staging.as_ref().and_then(|op| {
            self.entry_by_change_key(&ChangeKey {
                repository_id: op.repo_id,
                repo_path: op.anchor.clone(),
            })
        });
        let bulk_staging_anchor_new_index = bulk_staging
            .as_ref()
            .zip(bulk_staging_anchor_new_global_index)
            .and_then(|(op, ix)| self.repository_local_entry_index(ix, op.repo_id));
        if bulk_staging_anchor_new_index == last_staged_path_prev_index
            && let Some(index) = bulk_staging_anchor_new_global_index
            && let Some(entry) = self.entries.get(index)
            && let Some(entry) = entry.status_entry()
            && let Some(repository) = bulk_staging
                .as_ref()
                .and_then(|op| self.repository_for_id(op.repo_id, cx))
            && GitPanel::stage_status_for_entry(entry, repository.read(cx))
                .as_bool()
                .unwrap_or(false)
        {
            self.bulk_staging = bulk_staging;
        }

        let restored_selected_entry = selected_change
            .and_then(|(key, section)| {
                section
                    .and_then(|section| self.entry_by_change_key_in_section(&key, section))
                    .or_else(|| self.entry_by_change_key(&key))
            })
            .or_else(|| {
                selected_directory.and_then(|key| {
                    self.entries.iter().position(|entry| {
                        entry
                            .directory_entry()
                            .is_some_and(|directory| directory.key == key)
                    })
                })
            })
            .or_else(|| {
                selected_repository.and_then(|repository_id| {
                    self.entries.iter().position(|entry| {
                        matches!(
                            entry,
                            GitListEntry::RepositoryHeader(header)
                                if header.repository_id == repository_id
                        )
                    })
                })
            })
            .or_else(|| {
                selected_project_repositories
                    .then(|| {
                        self.entries.iter().position(|entry| {
                            matches!(entry, GitListEntry::ProjectRepositoriesHeader(_))
                        })
                    })
                    .flatten()
            })
            .or_else(|| {
                selected_section.and_then(|(repository_id, section)| {
                    self.entries.iter().enumerate().position(|(index, entry)| {
                        self.repository_id_for_entry_index(index) == Some(repository_id)
                            && matches!(
                                entry,
                                GitListEntry::Header(header) if header.header == section
                            )
                    })
                })
            })
            .filter(|&ix| self.is_entry_visible(ix));
        if had_selected_identity && restored_selected_entry.is_none() && self.context_menu.is_some()
        {
            self.context_menu.take();
            self.focus_handle.focus(window, cx);
        }
        self.selected_entry = restored_selected_entry;
        self.select_first_entry_if_none(window, cx);
        self.select_last_entry_if_out_of_bounds(window, cx);

        let suggested_commit_message = self.suggest_commit_message(cx);
        let placeholder_text = suggested_commit_message.unwrap_or("Enter commit message".into());

        self.commit_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text(&placeholder_text, window, cx)
        });

        cx.notify();
    }

    fn header_state(&self, header_type: Section, repo: &Repository) -> ToggleState {
        let header = GitHeaderEntry {
            header: header_type,
        };
        let mut count = 0;
        let mut staged_count = 0;
        for entry in self
            .change_entries_for_repository(repo.id)
            .filter(|entry| header.contains(entry, repo))
        {
            count += 1;
            if GitPanel::stage_status_for_entry(entry, repo).has_staged() {
                staged_count += 1;
            }
        }
        if staged_count == 0 {
            ToggleState::Unselected
        } else if count == staged_count {
            ToggleState::Selected
        } else {
            ToggleState::Indeterminate
        }
    }

    fn section_for_entry_index(&self, ix: usize) -> Option<Section> {
        for entry in self.entries.get(..=ix)?.iter().rev() {
            match entry {
                GitListEntry::Header(header) => return Some(header.header),
                GitListEntry::RepositoryHeader(_) | GitListEntry::ProjectRepositoriesHeader(_) => {
                    return None;
                }
                _ => {}
            }
        }
        None
    }

    fn stage_intent_for_entry_index(&self, ix: usize) -> StageIntent {
        self.section_for_entry_index(ix)
            .map_or(StageIntent::Toggle, StageIntent::for_section)
    }

    // A conflict that has been marked resolved (fully staged) is locked
    // against toggling: unstaging would rebuild the index entry from HEAD,
    // silently discarding the unmerged (base/ours/theirs) stages — a
    // round-trip git can't actually perform. The explicit git::UnstageFile
    // action remains as an escape hatch.
    fn is_resolved_conflict(&self, ix: usize, cx: &App) -> bool {
        if self.section_for_entry_index(ix) != Some(Section::Conflict) {
            return false;
        }
        let Some(entry) = self.entries.get(ix) else {
            return false;
        };
        let Some(repo) = self.repository_for_entry_index(ix, cx) else {
            return false;
        };
        let repo = repo.read(cx);
        match entry {
            GitListEntry::Directory(directory) => {
                self.stage_status_for_directory(directory, repo) == StageStatus::Staged
            }
            entry => entry.status_entry().is_some_and(|status_entry| {
                GitPanel::stage_status_for_entry(status_entry, repo) == StageStatus::Staged
            }),
        }
    }

    fn diff_target_for_section(section: Option<Section>) -> DiffTarget {
        match section {
            Some(Section::Staged) => DiffTarget::Staged,
            Some(Section::Unstaged) => DiffTarget::Unstaged,
            _ => DiffTarget::Uncommitted,
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
        self.diff_stat_total = DiffStat::default();

        let change_entries = self.change_entries_by_path().cloned().collect::<Vec<_>>();
        for status_entry in change_entries {
            self.entry_count += 1;
            if let Some(diff_stat) = status_entry.diff_stat {
                self.diff_stat_total.added =
                    self.diff_stat_total.added.saturating_add(diff_stat.added);
                self.diff_stat_total.deleted = self
                    .diff_stat_total
                    .deleted
                    .saturating_add(diff_stat.deleted);
            }

            let stage_status = GitPanel::stage_status_for_entry(&status_entry, repo);

            if repo.had_conflict_on_last_merge_head_change(&status_entry.repo_path) {
                self.conflicted_count += 1;
                if stage_status.has_staged() {
                    self.conflicted_staged_count += 1;
                }
            } else if status_entry.status.is_created() {
                self.new_count += 1;
                if stage_status.has_staged() {
                    self.new_staged_count += 1;
                }
            } else {
                self.tracked_count += 1;
                if stage_status.has_staged() {
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
        self.change_entries_by_path()
            .any(|entry| entry.staging.has_unstaged())
    }

    fn primary_changes_action_stages(&self) -> bool {
        self.entry_count == 0 || self.has_unstaged_changes()
    }

    fn has_tracked_changes(&self) -> bool {
        self.tracked_count > 0
    }

    pub fn has_unstaged_conflicts(&self) -> bool {
        self.change_entries_by_path()
            .any(|entry| entry.status.is_conflicted() && entry.staging.has_unstaged())
    }

    fn show_error_toast(&self, action: impl Into<SharedString>, e: anyhow::Error, cx: &mut App) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        show_error_toast(workspace, action, e, cx)
    }

    fn show_git_job_queue(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repo) = self.active_repository.as_ref() else {
            let workspace = self.workspace.clone();
            cx.defer(move |cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        struct GitJobQueueToast;
                        workspace.show_toast(
                            workspace::Toast::new(
                                NotificationId::unique::<GitJobQueueToast>(),
                                "No active repository",
                            )
                            .autohide(),
                            cx,
                        );
                    });
                }
            });
            return;
        };

        let repo_path = repo.read(cx).work_directory_abs_path.display().to_string();
        let queue_value = repo.read(cx).job_debug_queue().to_debug_value();
        let title = format!("Git Job Queue: {repo_path}");

        let json_language = self.project.read(cx).languages().language_for_name("JSON");
        let project = self.project.clone();
        let workspace = self.workspace.clone();

        window
            .spawn(cx, async move |cx| {
                let json_language = json_language.await.ok();

                // Best-effort: gather runtime diagnostics off the main thread.
                // Any failure inside `gather` is logged and produces an empty
                // section; this `.await` itself cannot meaningfully fail and
                // must never prevent us from showing the queue dump.
                let diagnostics = cx
                    .background_spawn(crate::git_runtime_diagnostics::gather())
                    .await;

                let mut combined = queue_value;
                if let serde_json::Value::Object(ref mut map) = combined
                    && let serde_json::Value::Object(diag_map) = diagnostics
                    && !diag_map.is_empty()
                {
                    map.insert(
                        "runtime_diagnostics".into(),
                        serde_json::Value::Object(diag_map),
                    );
                }

                let text = serde_json::to_string_pretty(&combined).unwrap_or_default();

                let buffer = project
                    .update(cx, |project, cx| {
                        project.create_buffer(json_language, false, cx)
                    })
                    .await?;

                buffer.update(cx, |buffer, cx| {
                    buffer.set_text(text, cx);
                    buffer.set_capability(language::Capability::ReadWrite, cx);
                });

                workspace.update_in(cx, |workspace, window, cx| {
                    let buffer =
                        cx.new(|cx| MultiBuffer::singleton(buffer, cx).with_title(title.clone()));

                    workspace.add_item_to_active_pane(
                        Box::new(cx.new(|cx| {
                            let mut editor =
                                Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                            editor.set_breadcrumb_header(title);
                            editor.disable_mouse_wheel_zoom();
                            editor
                        })),
                        None,
                        true,
                        window,
                        cx,
                    );
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
    }

    fn show_commit_message_error<E>(weak_this: &WeakEntity<Self>, err: &E, cx: &mut AsyncApp)
    where
        E: std::fmt::Debug + std::fmt::Display,
    {
        if let Ok(Some(workspace)) = weak_this.update(cx, |this, _cx| this.workspace.upgrade()) {
            let _ = workspace.update(cx, |workspace, cx| {
                workspace.show_error(format!("Failed to generate commit message: {err}"), cx);
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

        let is_push = matches!(action, RemoteAction::Push(_, _));

        workspace.update(cx, |workspace, cx| {
            let SuccessMessage { message, style } = remote_output::format_output(&action, info);
            let workspace_weak = cx.weak_entity();
            let operation = action.name();

            let status_toast = StatusToast::new(message, cx, move |this, _cx| {
                use remote_output::SuccessStyle::*;
                let this = this.icon(
                    Icon::new(IconName::GitBranch)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                );
                match (style, is_push) {
                    (PushPrLink { label, url }, _) => {
                        this.action(label, move |_window, cx| cx.open_url(&url))
                    }
                    (Toast | ToastWithLog { .. }, true) => {
                        // If we were not able to parse a valid URL from the
                        // output of a push command, we'll simply dispatch the
                        // generic `CreatePullRequest` action when the toast
                        // button is pressed.
                        this.action("Create Pull Request", move |window, cx| {
                            window
                                .dispatch_action(Box::new(zed_actions::git::CreatePullRequest), cx);
                        })
                    }
                    (Toast, false) => this,
                    (ToastWithLog { output }, false) => {
                        this.action("View Log", move |window, cx| {
                            let output = output.clone();
                            let output =
                                format!("stdout:\n{}\nstderr:\n{}", output.stdout, output.stderr);
                            workspace_weak
                                .update(cx, move |workspace, cx| {
                                    open_output(operation, workspace, &output, window, cx)
                                })
                                .ok();
                        })
                    }
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

    /// Computes tree indentation depths for visible entries in the given range.
    /// Used by indent guides to render vertical connector lines in tree view.
    fn compute_visible_depths(&self, range: Range<usize>) -> SmallVec<[usize; 64]> {
        range
            .map(|ix| {
                self.visible_entry_indices
                    .get(ix)
                    .and_then(|&entry_ix| self.entries.get(entry_ix))
                    .map_or(0, |_| {
                        self.visual_depth_for_entry(self.visible_entry_indices[ix])
                    })
            })
            .collect()
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
            GitListEntry::RepositoryHeader(header) => Some(Self::item_width_estimate(
                0,
                header.display_name.len()
                    + header.branch_label.len().min(12)
                    + if header.change_count == 0 {
                        0
                    } else {
                        header.change_count.to_string().len()
                    },
                0,
            )),
            GitListEntry::ProjectRepositoriesHeader(_)
            | GitListEntry::Header(_)
            | GitListEntry::EmptySection(_) => None,
        }
    }

    fn item_width_estimate(path: usize, file_name: usize, depth: usize) -> usize {
        path + file_name + depth * 2
    }

    fn render_view_options_menu(&self, id: impl Into<ElementId>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();

        PopoverMenu::new(id.into())
            .trigger_with_tooltip(
                IconButton::new("view-options-menu-trigger", IconName::Filter)
                    .icon_size(IconSize::Small),
                Tooltip::text("View Options"),
            )
            .menu(move |window, cx| {
                Some(git_panel_view_options_menu(
                    focus_handle.clone(),
                    window,
                    cx,
                ))
            })
            .anchor(Anchor::TopRight)
    }

    pub(crate) fn render_generate_commit_message_button(
        &self,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        if !agent_settings::AgentSettings::get_global(cx).enabled(cx) {
            return None;
        }

        if self.generate_commit_message_task.is_some() {
            return Some(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("cancel-generate-commit-message", IconName::Stop)
                            .icon_color(Color::Error)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Tinted(TintColor::Error))
                            .tooltip(Tooltip::text("Cancel Commit Message Generation"))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.generate_commit_message_task.take();
                                cx.notify();
                            })),
                    )
                    .child(
                        Label::new("Generating Commit…")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            );
        }

        let model_registry = LanguageModelRegistry::read_global(cx);
        let has_commit_model_configuration_error = model_registry
            .configuration_error(model_registry.commit_message_model(cx), cx)
            .is_some();
        let can_commit = self.can_commit();

        let editor_focus_handle = self.commit_editor.focus_handle(cx);

        let button = IconButton::new("generate-commit-message", IconName::AiEdit)
            .shape(ui::IconButtonShape::Square)
            .icon_color(if has_commit_model_configuration_error {
                Color::Disabled
            } else {
                Color::Muted
            })
            .disabled(!can_commit || has_commit_model_configuration_error)
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.generate_commit_message(cx);
            }));

        let button = if can_commit && has_commit_model_configuration_error {
            button.hoverable_tooltip(move |_window, cx| {
                cx.new(|_| GenerateCommitMessageConfigurationTooltip).into()
            })
        } else {
            button.tooltip(move |_window, cx| {
                if !can_commit {
                    Tooltip::simple("No Changes to Commit", cx)
                } else {
                    Tooltip::for_action_in(
                        "Generate Commit Message",
                        &git::GenerateCommitMessage,
                        &editor_focus_handle,
                        cx,
                    )
                }
            })
        };

        Some(button.into_any_element())
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
        let menu_open = self.commit_menu_handle.is_deployed();

        PopoverMenu::new(id.into())
            .trigger(crate::render_split_button_chevron_trigger(
                "commit-split-button-right",
                menu_open,
            ))
            .with_handle(self.commit_menu_handle.clone())
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
            .anchor(Anchor::TopRight)
            .offset(gpui::Point {
                x: px(0.),
                y: px(2.),
            })
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

    fn toggle_fill_commit_editor(
        &mut self,
        _: &ToggleFillCommitEditor,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.commit_editor_expanded = !self.commit_editor_expanded;
        self.commit_editor.update(cx, |editor, _cx| {
            if self.commit_editor_expanded {
                editor.set_mode(EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sizing_behavior: SizingBehavior::ExcludeOverscrollMargin,
                })
            } else {
                editor.set_mode(EditorMode::AutoHeight {
                    min_lines: MAX_PANEL_EDITOR_LINES,
                    max_lines: Some(MAX_PANEL_EDITOR_LINES),
                })
            }
        });

        cx.notify();
    }

    fn expand_commit_editor(
        &mut self,
        _: &ExpandCommitEditor,
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

    fn render_git_changes_actions_menu(
        &self,
        id: impl Into<ElementId>,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_tracked_changes = self.has_tracked_changes();
        let has_staged_changes = self.has_staged_changes();
        let has_unstaged_changes = self.has_unstaged_changes();
        let has_new_changes = self.new_count > 0;
        let has_stash_items = self.stash_entries.entries.len() > 0;

        let focus_handle = self.focus_handle.clone();
        let menu_open = self.changes_actions_menu_handle.is_deployed();

        PopoverMenu::new(id.into())
            .trigger(crate::render_split_button_chevron_trigger(
                "changes-actions-split-button-right",
                menu_open,
            ))
            .with_handle(self.changes_actions_menu_handle.clone())
            .menu(move |window, cx| {
                Some(git_panel_context_menu(
                    has_tracked_changes,
                    has_staged_changes,
                    has_unstaged_changes,
                    has_new_changes,
                    has_stash_items,
                    focus_handle.clone(),
                    window,
                    cx,
                ))
            })
            .anchor(Anchor::TopRight)
            .offset(gpui::Point {
                x: px(0.),
                y: px(2.),
            })
    }

    fn render_git_changes_actions_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let show_all_repositories = GitPanelSettings::get_global(cx).show_all_repositories;
        let (text, action, stage, tooltip) = if self.primary_changes_action_stages() {
            (
                if show_all_repositories {
                    "Stage Active"
                } else {
                    "Stage All"
                },
                StageAll.boxed_clone(),
                true,
                if show_all_repositories {
                    "Stage All in Active Repository"
                } else {
                    "git add --all"
                },
            )
        } else {
            (
                if show_all_repositories {
                    "Unstage Active"
                } else {
                    "Unstage All"
                },
                UnstageAll.boxed_clone(),
                false,
                if show_all_repositories {
                    "Unstage All in Active Repository"
                } else {
                    "git reset"
                },
            )
        };

        SplitButton::new(
            ButtonLike::new_rounded_left("git-changes-actions-split-button-left")
                .layer(ElevationIndex::ModalSurface)
                .size(ButtonSize::Compact)
                .child(Label::new(text).size(LabelSize::Small).mr_0p5())
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
            self.render_git_changes_actions_menu("git-changes-actions-split-button-menu", cx)
                .into_any_element(),
        )
    }

    fn render_changes_header(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if matches!(self.git_access, Some(GitAccess::No)) {
            return None;
        }

        self.active_repository.as_ref()?;

        let diff_stat_total = self.diff_stat_total;
        let show_all_repositories = GitPanelSettings::get_global(cx).show_all_repositories;
        let diff_label = if show_all_repositories {
            "View Active Diff"
        } else {
            "View Diff"
        };

        Some(
            h_flex()
                .min_h(Tab::container_height(cx))
                .w_full()
                .pl_1()
                .pr_2()
                .flex_none()
                .flex_wrap()
                .gap_1()
                .justify_between()
                .child(
                    ButtonLike::new("diff-button")
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Icon::new(IconName::Diff)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    Label::new(diff_label)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .when(
                                    GitPanelSettings::get_global(cx).diff_stats
                                        && diff_stat_total != DiffStat::default(),
                                    |this| {
                                        this.child(ui::DiffStat::new(
                                            "changes-diff-stat-total",
                                            diff_stat_total.added as usize,
                                            diff_stat_total.deleted as usize,
                                        ))
                                    },
                                ),
                        )
                        .tooltip(Tooltip::for_action_title_in(
                            diff_label,
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
                        .child(self.render_view_options_menu("view_options_menu"))
                        .child(self.render_git_changes_actions_button(cx)),
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
                        self.pending_remote_operation,
                        self.remote_action_menu_handle.clone(),
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
        let settings = ThemeSettings::get_global(cx);
        let panel_editor_style =
            git_commit_editor_style(settings.git_commit_buffer_font_size(cx), cx);
        let enable_coauthors = self.render_co_authors(cx);
        let editor_focus_handle = self.commit_editor.focus_handle(cx);
        let branch = active_repository.read(cx).branch.clone();
        let head_commit = active_repository.read(cx).head_commit.clone();

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

        let max_title_length = GitPanelSettings::get_global(cx).commit_title_max_length;
        let title_exceeds_limit = if max_title_length > 0 {
            self.commit_editor
                .read(cx)
                .text(cx)
                .lines()
                .next()
                .is_some_and(|title| commit_title_exceeds_limit(title, max_title_length))
        } else {
            false
        };

        let vertical_buttons = v_flex()
            .h_full()
            .gap_px()
            .p_1p5()
            .opacity(0.6)
            .hover(|s| s.opacity(1.0))
            .child(
                IconButton::new("expand-commit-editor", IconName::MaximizeAlt)
                    .icon_size(IconSize::Small)
                    .tooltip({
                        move |_window, cx| {
                            Tooltip::for_action_in(
                                "Open Commit Modal",
                                &git::ExpandCommitEditor,
                                &editor_focus_handle,
                                cx,
                            )
                        }
                    })
                    .on_click(cx.listener({
                        move |_, _, window, cx| {
                            window.dispatch_action(git::ExpandCommitEditor.boxed_clone(), cx)
                        }
                    })),
            )
            .child({
                let (icon, label) = if self.commit_editor_expanded {
                    (IconName::Minimize, "Collapse Commit Editor")
                } else {
                    (IconName::Maximize, "Expand Commit Editor")
                };
                let focus_handle = self.focus_handle.clone();

                IconButton::new("fill-commit-editor", icon)
                    .icon_size(IconSize::Small)
                    .tooltip({
                        move |_window, cx| {
                            Tooltip::for_action_in(
                                label,
                                &git::ToggleFillCommitEditor,
                                &focus_handle,
                                cx,
                            )
                        }
                    })
                    .on_click(cx.listener({
                        move |_, _, window, cx| {
                            window.dispatch_action(git::ToggleFillCommitEditor.boxed_clone(), cx)
                        }
                    }))
            });

        let footer = v_flex()
            .when(self.commit_editor_expanded, |this| this.flex_1().min_h_0())
            .child(PanelRepoFooter::new(
                display_name,
                branch,
                head_commit,
                Some(git_panel),
            ))
            .when(title_exceeds_limit, |this| {
                this.child(
                    h_flex()
                        .px_2()
                        .py_1()
                        .gap_1()
                        .border_t_1()
                        .border_color(cx.theme().status().warning_border)
                        .bg(cx.theme().status().warning_background.opacity(0.5))
                        .child(
                            Icon::new(IconName::Warning)
                                .size(IconSize::XSmall)
                                .color(Color::Warning),
                        )
                        .child(
                            Label::new(format!(
                                "Commit message title exceeds {max_title_length}-character limit."
                            ))
                            .size(LabelSize::Small),
                        ),
                )
            })
            .child(
                panel_editor_container(window, cx)
                    .id("commit-editor-container")
                    .w_full()
                    .when(self.commit_editor_expanded, |this| this.flex_1().min_h_0())
                    .border_t_1()
                    .border_color(if title_exceeds_limit {
                        cx.theme().status().warning_border
                    } else {
                        cx.theme().colors().border
                    })
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        window.focus(&this.commit_editor.focus_handle(cx), cx);
                    }))
                    .child(
                        h_flex()
                            .size_full()
                            .child(
                                div()
                                    .pt_2()
                                    .px_2()
                                    .h_full()
                                    .flex_grow_1()
                                    .cursor_text()
                                    .on_action(|&zed_actions::editor::MoveUp, _, cx| {
                                        cx.stop_propagation();
                                    })
                                    .on_action(|&zed_actions::editor::MoveDown, _, cx| {
                                        cx.stop_propagation();
                                    })
                                    .child(EditorElement::new(
                                        &self.commit_editor,
                                        panel_editor_style,
                                    )),
                            )
                            .child(vertical_buttons),
                    )
                    .child(
                        h_flex()
                            .id("commit-footer")
                            .w_full()
                            .p_1p5()
                            .border_t_1()
                            .when(editor_is_long, |el| {
                                el.border_color(cx.theme().colors().border_variant)
                            })
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

        h_flex()
            .id("commit-wrapper")
            .on_hover(cx.listener(move |this, hovered, _, cx| {
                this.show_placeholders =
                    *hovered && !this.has_staged_changes() && !this.has_unstaged_conflicts();
                cx.notify()
            }))
            .child(SplitButton::new(
                ButtonLike::new_rounded_left(format!("split-button-left-{}", title))
                    .layer(ElevationIndex::ModalSurface)
                    .size(ButtonSize::Compact)
                    .disabled(!can_commit || self.modal_open)
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
                                        CommitOptions {
                                            amend,
                                            signoff,
                                            allow_empty: false,
                                        },
                                        window,
                                        cx,
                                    );
                                })
                                .ok();
                        }
                    })
                    .tooltip({
                        let handle = commit_tooltip_focus_handle.clone();
                        move |_window, cx| {
                            if can_commit {
                                Tooltip::with_meta_in(
                                    tooltip,
                                    Some(&git::Commit),
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
                    .flex_grow_1()
                    .overflow_hidden()
                    .max_w(relative(0.85))
                    .child(
                        Label::new("This will update your most recent commit.")
                            .size(LabelSize::Small)
                            .truncate(),
                    ),
            )
            .child(
                Button::new("cancel", "Cancel")
                    .label_size(LabelSize::Small)
                    .layer(ElevationIndex::ModalSurface)
                    .on_click(cx.listener(|this, _, _, cx| this.set_amend_pending(false, cx))),
            )
    }

    fn render_previous_commit(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
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
                        .cursor_pointer()
                        .px_1()
                        .rounded_sm()
                        .line_clamp(1)
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
                .child(
                    h_flex()
                        .gap_0p5()
                        .when(commit.has_parent, |this| {
                            let has_unstaged = self.has_unstaged_changes();
                            this.child(
                                IconButton::new("undo", IconName::Undo)
                                    .icon_size(IconSize::Small)
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
                                    .on_click(
                                        cx.listener(|this, _, window, cx| {
                                            this.uncommit(window, cx)
                                        }),
                                    ),
                            )
                        })
                        .child(
                            IconButton::new("git-graph-button", IconName::GitGraph)
                                .icon_size(IconSize::Small)
                                .tooltip(|_window, cx| {
                                    Tooltip::for_action(
                                        "Open Git Graph",
                                        &crate::git_graph::Open,
                                        cx,
                                    )
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(crate::git_graph::Open.boxed_clone(), cx)
                                }),
                        ),
                ),
        )
    }

    fn render_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_tab = self.active_tab;

        let focus_handle = self.focus_handle.clone();
        let tab = |id: ElementId,
                   active: bool,
                   show_changes: bool,
                   label: SharedString,
                   set_active_tab: GitPanelTab,
                   tooltip_action: Box<dyn Action>| {
            let focus_handle = focus_handle.clone();

            h_flex()
                .cursor_pointer()
                .id(id)
                .h_full()
                .py_1()
                .gap_1()
                .flex_1()
                .justify_center()
                .hover(|s| s.bg(cx.theme().colors().element_hover))
                .border_b_1()
                .when(!active, |s| {
                    s.bg(cx.theme().colors().editor_background.opacity(0.6))
                        .border_color(cx.theme().colors().border.opacity(0.6))
                })
                .child(Label::new(label.clone()).when(!active, |this| this.color(Color::Muted)))
                .when(show_changes && self.changes_count > 0, |this| {
                    this.child(
                        Label::new(format!("({})", self.changes_count))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                })
                .tooltip(Tooltip::for_action_title_in(
                    format!("Toggle {} Tab", label),
                    tooltip_action.as_ref(),
                    &focus_handle,
                ))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.set_active_tab(set_active_tab, window, cx)
                }))
        };

        h_flex()
            .relative()
            .h(Tab::container_height(cx))
            .w_full()
            .child(tab(
                ElementId::Name("changes-tab".into()),
                active_tab == GitPanelTab::Changes,
                true,
                "Changes".into(),
                GitPanelTab::Changes,
                ActivateChangesTab.boxed_clone(),
            ))
            .child(
                Divider::vertical()
                    .color(ui::DividerColor::BorderFaded)
                    .h_full(),
            )
            .child(tab(
                ElementId::Name("history-tab".into()),
                active_tab != GitPanelTab::Changes,
                false,
                "History".into(),
                GitPanelTab::History,
                ActivateHistoryTab.boxed_clone(),
            ))
    }

    fn render_history_tab(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().flex_1().size_full().overflow_hidden().map(|this| {
            let has_repo = self.active_repository.is_some();
            match &self.commit_history {
                _ if !has_repo => {
                    this.child(Self::render_history_placeholder("No repository found"))
                }
                CommitHistory::Error(_) => this.child(Self::render_history_placeholder(
                    "Failed to load commit history",
                )),
                CommitHistory::Loading => {
                    this.child(Self::render_history_placeholder("Loading Commit History…"))
                }
                CommitHistory::Loaded(entries) if entries.is_empty() => {
                    this.child(Self::render_history_placeholder("No commits yet"))
                }
                CommitHistory::Loaded(_) => match self.render_commit_history(window, cx) {
                    Some(history) => this.child(history),
                    None => this.child(Self::render_history_placeholder("Failed to load commits")),
                },
            }
        })
    }

    fn render_history_placeholder(message: &'static str) -> impl IntoElement {
        h_flex()
            .flex_1()
            .justify_center()
            .child(Label::new(message).color(Color::Muted))
    }

    fn commit_history_entries(&self) -> &[CommitHistoryEntry] {
        match &self.commit_history {
            CommitHistory::Loaded(entries) => entries,
            CommitHistory::Loading | CommitHistory::Error(_) => &[],
        }
    }

    fn select_next_history_entry(&mut self, cx: &mut Context<Self>) {
        let count = self.commit_history_entries().len();
        if count == 0 {
            return;
        }
        let new_index = match self.focused_history_entry {
            None => 0,
            Some(i) => (i + 1).min(count - 1),
        };
        self.focused_history_entry = Some(new_index);
        self.history_keyboard_nav = true;
        self.commit_history_scroll_handle
            .scroll_to_item(new_index, ScrollStrategy::Top);
        cx.notify();
    }

    fn select_previous_history_entry(&mut self, cx: &mut Context<Self>) {
        let count = self.commit_history_entries().len();
        if count == 0 {
            return;
        }
        let new_index = match self.focused_history_entry {
            None => 0,
            Some(i) => i.saturating_sub(1),
        };
        self.focused_history_entry = Some(new_index);
        self.history_keyboard_nav = true;
        self.commit_history_scroll_handle
            .scroll_to_item(new_index, ScrollStrategy::Top);
        cx.notify();
    }

    fn open_selected_history_commit(&self, window: &mut Window, cx: &mut App) {
        let Some(index) = self.focused_history_entry else {
            return;
        };
        let Some(entry) = self.commit_history_entries().get(index) else {
            return;
        };
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };
        CommitView::open(
            entry.sha.to_string(),
            active_repository.downgrade(),
            self.workspace.clone(),
            None,
            None,
            window,
            cx,
        );
    }

    fn deploy_history_context_menu(
        &mut self,
        position: Point<Pixels>,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(commit) = self.commit_history_entries().get(index).cloned() else {
            return;
        };
        let Some(repository) = self.active_repository.as_ref() else {
            return;
        };
        let context_menu = commit_context_menu(
            CommitContextMenuData {
                sha: commit.sha,
                tag_names: commit.tag_names,
            },
            CommitContextMenuSource::GitPanel,
            None,
            self.focus_handle.clone(),
            Some(repository.downgrade()),
            self.workspace.clone(),
            window,
            cx,
        );
        self.focused_history_entry = Some(index);
        self.history_keyboard_nav = false;
        self.set_context_menu(context_menu, position, window, cx);
    }

    fn activate_changes_tab(
        &mut self,
        _: &ActivateChangesTab,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_active_tab(GitPanelTab::Changes, window, cx);
    }

    fn activate_history_tab(
        &mut self,
        _: &ActivateHistoryTab,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_active_tab(GitPanelTab::History, window, cx);
    }

    fn set_active_tab(&mut self, tab: GitPanelTab, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_tab == tab {
            return;
        }
        self.active_tab = tab;
        match tab {
            GitPanelTab::History => {
                self.focus_handle.focus(window, cx);
                self.load_commit_history(cx);
            }
            GitPanelTab::Changes => {
                self.focus_handle.focus(window, cx);
                self.set_commit_history(CommitHistory::Loading, cx);
                self._repo_subscriptions.clear();
            }
        }
        cx.notify();
    }

    fn preload_commit_history(&mut self, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.as_ref() else {
            return;
        };

        let Some(log_source) = Self::commit_history_log_source(active_repository, cx) else {
            return;
        };
        let log_order = LogOrder::DateOrder;

        // Kick off the git log fetch so data is ready when the user switches to History.
        // graph_data() is idempotent — if already loading/loaded, this is a no-op.
        active_repository.update(cx, |repository, cx| {
            repository.graph_data(log_source, log_order, 0..0, cx);
        });
    }

    fn load_commit_history(&mut self, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        if self._repo_subscriptions.is_empty() {
            self._repo_subscriptions.push(cx.subscribe(
                &active_repository,
                |this, _repo, event, cx| {
                    if let RepositoryEvent::GraphEvent(_, _) = event {
                        if this.active_tab == GitPanelTab::History {
                            this.fetch_commit_history_entries(cx);
                        }
                    }
                },
            ));
            self._repo_subscriptions
                .push(cx.observe(&active_repository, |_this, _repo, cx| {
                    cx.notify();
                }));
        }

        self.fetch_commit_history_entries(cx);
    }

    fn fetch_commit_history_entries(&mut self, cx: &mut Context<Self>) {
        let Some(active_repository) = self.active_repository.clone() else {
            return;
        };

        let Some(log_source) = Self::commit_history_log_source(&active_repository, cx) else {
            // No HEAD commit at all (unborn/empty repository).
            self.set_commit_history(CommitHistory::Loaded(Rc::from([])), cx);
            return;
        };
        let log_order = LogOrder::DateOrder;

        let (entries, is_loading, error) = active_repository.update(cx, |repository, cx| {
            let response = repository.graph_data(log_source, log_order, 0..usize::MAX, cx);
            let entries: Rc<[CommitHistoryEntry]> = response
                .commits
                .iter()
                .map(CommitHistoryEntry::from)
                .collect();
            (entries, response.is_loading, response.error)
        });

        self.set_commit_history(commit_history_from_response(entries, is_loading, error), cx);
    }

    fn set_commit_history(&mut self, commit_history: CommitHistory, cx: &mut Context<Self>) {
        let changed = self.commit_history != commit_history;
        self.commit_history = commit_history;
        // Keep the focused entry within range as the history grows or clears.
        let count = self.commit_history_entries().len();
        let focused = self.focused_history_entry.unwrap_or(0);
        self.focused_history_entry = (count > 0).then(|| focused.min(count - 1));
        if changed {
            cx.notify();
        }
    }

    fn commit_history_log_source(
        active_repository: &Entity<Repository>,
        cx: &App,
    ) -> Option<LogSource> {
        let repository = active_repository.read(cx);
        let head_commit = repository.head_commit.as_ref()?;
        if let Some(branch) = repository.branch.as_ref() {
            Some(LogSource::Branch(branch.name().to_string().into()))
        } else {
            Some(LogSource::Sha(head_commit.sha.as_ref().parse().ok()?))
        }
    }

    fn git_remote(&self, cx: &mut App) -> Option<GitRemote> {
        let repo = self.active_repository.as_ref()?;
        let remote_url = repo.read(cx).default_remote_url()?;
        let provider_registry = GitHostingProviderRegistry::default_global(cx);
        let (provider, parsed) = parse_git_remote_url(provider_registry, &remote_url)?;
        Some(GitRemote {
            host: provider,
            owner: parsed.owner.into(),
            repo: parsed.repo.into(),
        })
    }

    fn render_commit_history(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let CommitHistory::Loaded(entries) = &self.commit_history else {
            return None;
        };
        let entries = entries.clone();
        let active_repository = self.active_repository.as_ref()?;
        let workspace = self.workspace.clone();
        let repo_weak = active_repository.downgrade();
        let item_count = entries.len();
        let commit_history_scroll_handle = self.commit_history_scroll_handle.clone();
        let remote = self.git_remote(cx);

        let focused_history_entry = self.focused_history_entry;
        let is_panel_focused = self.focus_handle.is_focused(window);
        let show_focus_border = self.history_keyboard_nav;
        let has_context_menu = self.context_menu.is_some();

        let ahead_count = active_repository
            .read(cx)
            .branch
            .as_ref()
            .and_then(|b| b.upstream.as_ref())
            .and_then(|u| u.tracking.status())
            .map(|s| s.ahead as usize)
            .unwrap_or(0);

        Some(
            v_flex()
                .flex_1()
                .size_full()
                .overflow_hidden()
                .child(
                    uniform_list("commit_history_list", item_count, {
                        let workspace = workspace;
                        let repo_weak = repo_weak;
                        let git_panel = cx.weak_entity();
                        move |range, window, cx| {
                            let local_offset = time::UtcOffset::current_local_offset()
                                .unwrap_or(time::UtcOffset::UTC);
                            let now = time::OffsetDateTime::now_utc();

                            let visible_data: Vec<Option<Arc<CommitData>>> = repo_weak
                                .update(cx, |repository, cx| {
                                    entries[range.clone()]
                                        .iter()
                                        .map(|entry| {
                                            match repository.fetch_commit_data(entry.sha, false, cx)
                                            {
                                                CommitDataState::Loaded(data) => Some(data.clone()),
                                                CommitDataState::Loading(_) => None,
                                            }
                                        })
                                        .collect()
                                })
                                .unwrap_or_default();

                            entries[range.clone()]
                                .iter()
                                .zip(visible_data)
                                .enumerate()
                                .map(|(ix, (entry, data))| {
                                    let index = range.start + ix;
                                    let sha_string = entry.sha.to_string();
                                    let sha_shared: SharedString = sha_string.clone().into();
                                    let short_sha: SharedString =
                                        sha_string[..7.min(sha_string.len())].to_string().into();
                                    let tag_names = entry.tag_names.clone();

                                    let (subject, author_name, author_email, timestamp): (
                                        SharedString,
                                        SharedString,
                                        Option<SharedString>,
                                        Option<i64>,
                                    ) = match &data {
                                        Some(data) => (
                                            data.subject.clone(),
                                            data.author_name.clone(),
                                            Some(data.author_email.clone()),
                                            Some(data.commit_timestamp),
                                        ),
                                        None => ("Loading…".into(), "".into(), None, None),
                                    };

                                    let relative_time: SharedString = timestamp
                                        .and_then(|ts| {
                                            time::OffsetDateTime::from_unix_timestamp(ts).ok()
                                        })
                                        .map(|dt| {
                                            time_format::format_localized_timestamp(
                                                dt,
                                                now,
                                                local_offset,
                                                time_format::TimestampFormat::Relative,
                                            )
                                            .into()
                                        })
                                        .unwrap_or_else(|| "".into());

                                    let avatar = CommitAvatar::new(
                                        &sha_shared,
                                        author_email,
                                        remote.as_ref(),
                                    )
                                    .size(px(14.))
                                    .render(window, cx);

                                    let is_unpushed = index < ahead_count;
                                    let is_focused = focused_history_entry == Some(index);
                                    let workspace = workspace.clone();
                                    let repo = repo_weak.clone();
                                    let sha_for_click = sha_string;

                                    let dot_separator = || {
                                        Label::new("•")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .alpha(0.5)
                                    };

                                    v_flex()
                                        .id(("commit-history-item", index))
                                        .cursor_pointer()
                                        .w_full()
                                        .py_1()
                                        .px_2()
                                        .gap_0p5()
                                        .border_1()
                                        .border_color(gpui::transparent_black())
                                        .when(
                                            is_focused && is_panel_focused && show_focus_border,
                                            |this| {
                                                this.border_color(
                                                    cx.theme().colors().panel_focused_border,
                                                )
                                            },
                                        )
                                        .hover(|s| s.bg(cx.theme().colors().element_hover))
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .w_full()
                                                .min_w_0()
                                                .child(Label::new(subject).truncate())
                                                .children((!tag_names.is_empty()).then(|| {
                                                    let hidden_tag_count = tag_names
                                                        .len()
                                                        .saturating_sub(MAX_HISTORY_TAG_CHIPS);
                                                    h_flex()
                                                        .gap_1()
                                                        .min_w_0()
                                                        .children(
                                                            tag_names
                                                                .iter()
                                                                .take(MAX_HISTORY_TAG_CHIPS)
                                                                .map(|tag_name| {
                                                                    let tag_name = tag_name.clone();
                                                                    Chip::new(tag_name.clone())
                                                                        .truncate()
                                                                        .when(
                                                                            !has_context_menu,
                                                                            |chip| {
                                                                                chip.tooltip(
                                                                                    Tooltip::text(
                                                                                        tag_name,
                                                                                    ),
                                                                                )
                                                                            },
                                                                        )
                                                                }),
                                                        )
                                                        .when(hidden_tag_count > 0, |this| {
                                                            let hidden_tag_names = tag_names
                                                                [MAX_HISTORY_TAG_CHIPS..]
                                                                .join(", ");
                                                            this.child(
                                                                Chip::new(format!(
                                                                    "+{hidden_tag_count}"
                                                                ))
                                                                .when(!has_context_menu, |chip| {
                                                                    chip.tooltip(Tooltip::text(
                                                                        hidden_tag_names,
                                                                    ))
                                                                }),
                                                            )
                                                        })
                                                }))
                                                .when(is_unpushed, |this| {
                                                    this.child(
                                                        Icon::new(IconName::ArrowUp)
                                                            .size(IconSize::XSmall),
                                                    )
                                                }),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1p5()
                                                .child(avatar)
                                                .when(!author_name.is_empty(), |this| {
                                                    this.child(
                                                        Label::new(author_name)
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted),
                                                    )
                                                    .child(dot_separator())
                                                })
                                                .when(!relative_time.is_empty(), |this| {
                                                    this.child(
                                                        Label::new(relative_time)
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted),
                                                    )
                                                    .child(dot_separator())
                                                })
                                                .child(
                                                    Label::new(short_sha.clone())
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                ),
                                        )
                                        .when(!has_context_menu, |this| {
                                            this.tooltip(move |_, cx| {
                                                Tooltip::with_meta(
                                                    "View Commit",
                                                    None,
                                                    short_sha.clone(),
                                                    cx,
                                                )
                                            })
                                        })
                                        .on_mouse_down(gpui::MouseButton::Left, {
                                            let git_panel = git_panel.clone();
                                            move |_, _, cx| {
                                                git_panel
                                                    .update(cx, |panel, cx| {
                                                        panel.focused_history_entry = Some(index);
                                                        panel.history_keyboard_nav = false;
                                                        cx.notify();
                                                    })
                                                    .ok();
                                            }
                                        })
                                        .on_mouse_down(MouseButton::Right, {
                                            let git_panel = git_panel.clone();
                                            move |event, window, cx| {
                                                git_panel
                                                    .update(cx, |panel, cx| {
                                                        panel.deploy_history_context_menu(
                                                            event.position,
                                                            index,
                                                            window,
                                                            cx,
                                                        );
                                                    })
                                                    .ok();
                                                cx.stop_propagation();
                                            }
                                        })
                                        .on_click(move |_, window, cx| {
                                            CommitView::open(
                                                sha_for_click.clone(),
                                                repo.clone(),
                                                workspace.clone(),
                                                None,
                                                None,
                                                window,
                                                cx,
                                            );
                                        })
                                        .into_any_element()
                                })
                                .collect()
                        }
                    })
                    .size_full()
                    .track_scroll(&commit_history_scroll_handle),
                )
                .vertical_scrollbar_for(&commit_history_scroll_handle, window, cx),
        )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match (self.git_access, &self.active_repository) {
            (Some(GitAccess::No), Some(repository)) => self.render_unsafe_repo_ui(repository, cx),
            (_, None) => self.render_uninitialized_ui(cx),
            (_, Some(_)) => self.render_no_changes_ui(cx),
        };

        v_flex()
            .gap_1p5()
            .flex_1()
            .items_center()
            .justify_center()
            .child(content)
    }

    fn render_no_changes_ui(&self, cx: &Context<Self>) -> AnyElement {
        let show_branch_diff = self.active_changes_count == 0 && !self.is_on_main_branch(cx);

        v_flex()
            .gap_1()
            .items_center()
            .child(Label::new("No changes to commit").color(Color::Muted))
            .when(show_branch_diff, |this| {
                this.child(
                    Button::new("view_branch_diff", "View Branch Diff")
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Outlined)
                        .on_click(move |_, _, cx| {
                            cx.defer(move |cx| {
                                cx.dispatch_action(&DeployBranchDiff);
                            })
                        }),
                )
            })
            .into_any_element()
    }

    fn render_unsafe_repo_ui(
        &self,
        active_repository: &Entity<Repository>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let directory = active_repository.update(cx, |repository, _cx| {
            repository.snapshot().work_directory_abs_path
        });

        let message = format!(
            "Detected dubious ownership in repository at {}. \
            This happens when the .git/ directory is not owned by the current user. \
            If you want to learn more about safe directories, visit git's documentation.",
            directory.display()
        );

        v_flex()
                .px_4()
                .gap_1()
                .child(Label::new(message).color(Color::Muted))
                .child(
                    h_flex()
                        .flex_wrap()
                        .gap_1()
                        .child(
                            Button::new("trust_directory", "Trust Directory")
                            .label_size(LabelSize::Small)
                            .layer(ElevationIndex::ModalSurface)
                            .style(ButtonStyle::Filled)
                            .tooltip(Tooltip::text(
                                format!("git config --global --add safe.directory {}", directory.display())
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| {
                                    this.add_safe_directory(window, cx);
                                })
                            )
                    )
                    .child(
                        Button::new("learn_more", "Learn More")
                            .label_size(LabelSize::Small)
                            .style(ButtonStyle::Outlined)
                            .end_icon(Icon::new(IconName::ArrowUpRight).size(IconSize::Small).color(Color::Muted))
                            .on_click(move |_, _, cx| cx.open_url("https://git-scm.com/docs/git-config#Documentation/git-config.txt-safedirectory"))
                    )
                )
                .into_any_element()
    }

    fn render_uninitialized_ui(&self, cx: &mut Context<Self>) -> AnyElement {
        let worktree_count = self.project.read(cx).visible_worktrees(cx).count();
        if worktree_count > 0 && self.active_repository.is_none() {
            v_flex()
                .gap_1()
                .items_center()
                .child(Label::new("No Git Repositories").color(Color::Muted))
                .child(
                    Button::new("initialize_repository", "Initialize Repository")
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Outlined)
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
                .into_any_element()
        } else if worktree_count == 0 {
            let focus_handle = self.focus_handle.clone();
            ProjectEmptyState::new(
                "Git Panel",
                focus_handle.clone(),
                KeyBinding::for_action_in(&workspace::Open::default(), &focus_handle, cx),
            )
            .on_open_project(|_, window, cx| {
                telemetry::event!("Git Panel Add Project Clicked");
                window.dispatch_action(workspace::Open::default().boxed_clone(), cx);
            })
            .on_clone_repo(|_, window, cx| {
                telemetry::event!("Git Panel Clone Repo Clicked");
                window.dispatch_action(git::Clone.boxed_clone(), cx);
            })
            .into_any_element()
        } else {
            Empty.into_any_element()
        }
    }

    fn is_on_main_branch(&self, cx: &Context<Self>) -> bool {
        let Some(repo) = self.active_repository.as_ref() else {
            return false;
        };

        let Some(branch) = repo.read(cx).branch.as_ref() else {
            return false;
        };

        let branch_name = branch.name();
        matches!(branch_name, "main" | "master")
    }

    fn render_buffer_header_controls(
        &self,
        entity: &Entity<Self>,
        file: &Arc<dyn File>,
        _: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let project_path = (file.worktree_id(cx), file.path().clone()).into();
        let git_store = self.project.read(cx).git_store().clone();
        let (repository, repo_path, ix) = git_store
            .read(cx)
            .repositories()
            .values()
            .filter_map(|repository| {
                let repo = repository.read(cx);
                let repo_path = repo.project_path_to_repo_path(&project_path, cx)?;
                let ix = self.entry_by_change_key(&ChangeKey {
                    repository_id: repo.id,
                    repo_path: repo_path.clone(),
                })?;
                Some((
                    repository.clone(),
                    repo_path,
                    ix,
                    repo.work_directory_abs_path.components().count(),
                ))
            })
            .max_by_key(|(_, _, _, depth)| *depth)
            .map(|(repository, repo_path, ix, _)| (repository, repo_path, ix))?;
        let repo = repository.read(cx);
        let repository_id = repo.id;
        let entry = self.entries.get(ix)?;

        let is_staging_or_staged = repo
            .pending_ops_for_path(&repo_path)
            .map(|ops| !ops.last_op_errored() && (ops.staging() || ops.staged()))
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
                            this.toggle_staged_for_entry_in_repository(
                                &entry,
                                repository_id,
                                StageIntent::Toggle,
                                window,
                                cx,
                            );
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
        let is_tree_view = matches!(&self.view_mode, GitPanelViewMode::Tree(_));
        let show_indent_guides = is_tree_view || !self.project_repository_depths.is_empty();
        let entry_count = self.visible_entry_indices.len();
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

                                for ix in range.into_iter().map(|ix| this.visible_entry_indices[ix])
                                {
                                    match &this.entries.get(ix) {
                                        Some(GitListEntry::RepositoryHeader(entry)) => {
                                            items.push(
                                                this.render_repository_header(
                                                    ix, entry, window, cx,
                                                ),
                                            );
                                        }
                                        Some(GitListEntry::ProjectRepositoriesHeader(entry)) => {
                                            items.push(this.render_project_repositories_header(
                                                ix, entry, window, cx,
                                            ));
                                        }
                                        Some(GitListEntry::Status(entry)) => {
                                            let Some(repo) =
                                                this.repository_for_entry_index(ix, cx)
                                            else {
                                                items.push(
                                                    div()
                                                        .h(this.list_item_height())
                                                        .into_any_element(),
                                                );
                                                continue;
                                            };
                                            items.push(this.render_status_entry(
                                                ix,
                                                entry,
                                                0,
                                                has_write_access,
                                                repo.read(cx),
                                                window,
                                                cx,
                                            ));
                                        }
                                        Some(GitListEntry::TreeStatus(entry)) => {
                                            let Some(repo) =
                                                this.repository_for_entry_index(ix, cx)
                                            else {
                                                items.push(
                                                    div()
                                                        .h(this.list_item_height())
                                                        .into_any_element(),
                                                );
                                                continue;
                                            };
                                            items.push(this.render_status_entry(
                                                ix,
                                                &entry.entry,
                                                entry.depth,
                                                has_write_access,
                                                repo.read(cx),
                                                window,
                                                cx,
                                            ));
                                        }
                                        Some(GitListEntry::Directory(entry)) => {
                                            let Some(repo) =
                                                this.repository_for_entry_index(ix, cx)
                                            else {
                                                items.push(
                                                    div()
                                                        .h(this.list_item_height())
                                                        .into_any_element(),
                                                );
                                                continue;
                                            };
                                            items.push(this.render_directory_entry(
                                                ix,
                                                entry,
                                                has_write_access,
                                                repo.read(cx),
                                                window,
                                                cx,
                                            ));
                                        }
                                        Some(GitListEntry::Header(header)) => {
                                            let Some(repo) =
                                                this.repository_for_entry_index(ix, cx)
                                            else {
                                                items.push(
                                                    div()
                                                        .h(this.list_item_height())
                                                        .into_any_element(),
                                                );
                                                continue;
                                            };
                                            items.push(this.render_list_header(
                                                ix,
                                                header,
                                                has_write_access,
                                                repo.read(cx),
                                                window,
                                                cx,
                                            ));
                                        }
                                        Some(GitListEntry::EmptySection(section)) => {
                                            items.push(this.render_empty_section(ix, *section));
                                        }
                                        None => {}
                                    }
                                }

                                items
                            }),
                        )
                        .when(show_indent_guides, |list| {
                            list.with_decoration(
                                ui::indent_guides(px(TREE_INDENT), IndentGuideColors::panel(cx))
                                    .with_left_offset(INDENT_GUIDE_LEFT_OFFSET)
                                    .with_compute_indents_fn(
                                        cx.entity(),
                                        |this, range, _window, _cx| {
                                            this.compute_visible_depths(range)
                                        },
                                    ),
                            )
                        })
                        .group("entries")
                        .size_full()
                        .flex_grow_1()
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
                        Scrollbars::for_settings::<GitPanelScrollbarAccessor>()
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
        Label::new(label.into()).single_line().color(color)
    }

    fn list_item_height(&self) -> Rems {
        rems(1.75)
    }

    fn render_project_repositories_header(
        &self,
        ix: usize,
        entry: &GitProjectRepositoriesEntry,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let expanded = entry.expanded;
        let selected = self.selected_entry == Some(ix);
        let action = if expanded { "Collapse" } else { "Expand" };
        let repository_count = entry.repository_count;
        let tooltip = format!(
            "{action} {repository_count} project repositories ({} changes)",
            entry.change_count
        );
        let weak = cx.weak_entity();

        let button = ButtonLike::new("project-repositories-header")
            .full_width()
            .height(self.list_item_height().into())
            .style(ButtonStyle::Transparent)
            .aria_label(format!("{action} {repository_count} project repositories"))
            .aria_expanded(expanded)
            .when(entry.contains_active_repository, |button| {
                button.aria_description("Contains the active repository")
            })
            .tab_index(0isize)
            .tooltip(Tooltip::text(tooltip))
            .on_click(move |_, window, cx| {
                weak.update(cx, |this, cx| {
                    this.selected_entry = Some(ix);
                    this.toggle_project_repositories(window, cx);
                    cx.stop_propagation();
                })
                .ok();
            })
            .child(
                h_flex()
                    .h(self.list_item_height())
                    .min_w_0()
                    .w_full()
                    .pl_2p5()
                    .gap_1()
                    .child(
                        Icon::new(if expanded {
                            IconName::ChevronDown
                        } else {
                            IconName::ChevronRight
                        })
                        .size(IconSize::Small)
                        .color(Color::Muted),
                    )
                    .child(
                        Icon::new(IconName::FolderInclude)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        h_flex()
                            .min_w_0()
                            .flex_1()
                            .gap_1()
                            .child(
                                Label::new("Project repositories")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .truncate(),
                            )
                            .child(
                                Label::new(repository_count.to_string())
                                    .size(LabelSize::XSmall)
                                    .color(Color::Placeholder),
                            ),
                    )
                    .child(
                        h_flex()
                            .min_w(Checkbox::container_size())
                            .flex_shrink_0()
                            .justify_center()
                            .when(entry.change_count > 0, |this| {
                                this.child(
                                    Label::new(entry.change_count.to_string())
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                )
                            }),
                    ),
            );

        h_flex()
            .h(self.list_item_height())
            .min_w_0()
            .w_full()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .when(
                entry.contains_active_repository && !expanded && !selected,
                |this| this.bg(cx.theme().colors().ghost_element_selected),
            )
            .when(selected, |this| {
                this.bg(cx.theme().colors().ghost_element_selected)
            })
            .when(selected && self.focus_handle.is_focused(window), |this| {
                this.border_1()
                    .border_r_2()
                    .border_color(cx.theme().colors().panel_focused_border)
            })
            .child(button)
            .into_any_element()
    }

    fn render_repository_header(
        &self,
        ix: usize,
        entry: &GitRepositoryHeaderEntry,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let repository_id = entry.repository_id;
        let is_active = entry.is_active;
        let selected = self.selected_entry == Some(ix);
        let repository_tooltip = if is_active {
            format!("Active repository: {}", entry.work_directory)
        } else {
            format!("Use as active repository: {}", entry.work_directory)
        };
        let branch_tooltip = if entry.kind == GitRepositoryKind::Submodule {
            if let Some(parent) = entry.parent_display_name.as_ref() {
                format!(
                    "Switch branch in submodule {}. Its commit is recorded by {}.",
                    entry.display_name, parent
                )
            } else {
                format!("Switch branch in submodule {}", entry.display_name)
            }
        } else {
            format!("Switch branch in {}", entry.display_name)
        };
        let submodule_description = entry.parent_display_name.as_ref().map(|parent| {
            format!(
                "Submodule of {parent}. Switching its commit may update the parent repository status."
            )
        });
        let activate_panel = cx.weak_entity();
        let collapse_panel = cx.weak_entity();
        let branch_panel = cx.weak_entity();
        let project_depth = self.project_repository_depth(repository_id);
        let repository_expanded = entry.expanded;
        let repository_has_changes = entry.change_count > 0;
        let disclosure_action = if repository_expanded {
            "Collapse"
        } else {
            "Expand"
        };
        let disclosure_tooltip = if repository_has_changes {
            format!("{disclosure_action} changes in {}", entry.display_name)
        } else {
            format!("No changes in {}", entry.display_name)
        };

        let repository_disclosure = IconButton::new(
            ("repository-disclosure", repository_id.0),
            if repository_expanded {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            },
        )
        .size(ButtonSize::None)
        .style(ButtonStyle::Transparent)
        .icon_size(IconSize::Small)
        .icon_color(Color::Muted)
        .aria_label(format!(
            "{disclosure_action} changes in {}",
            entry.display_name
        ))
        .aria_expanded(repository_expanded)
        .tab_index(0isize)
        .disabled(!repository_has_changes)
        .tooltip(Tooltip::text(disclosure_tooltip))
        .on_click(move |_, window, cx| {
            collapse_panel
                .update(cx, |this, cx| {
                    this.selected_entry = Some(ix);
                    this.toggle_repository(repository_id, window, cx);
                    cx.stop_propagation();
                })
                .ok();
        });

        let repository_button =
            ButtonLike::new(("repository-header", repository_id.0))
                .full_width()
                .height(self.list_item_height().into())
                .style(ButtonStyle::Transparent)
                .aria_label(if is_active {
                    format!("Active repository: {}", entry.display_name)
                } else {
                    format!("Use {} as the active repository", entry.display_name)
                })
                .when_some(submodule_description.clone(), |button, description| {
                    button.aria_description(description)
                })
                .tab_index(0isize)
                .tooltip(Tooltip::text(repository_tooltip))
                .on_click(move |_, _, cx| {
                    activate_panel
                        .update(cx, |this, cx| {
                            this.selected_entry = Some(ix);
                            this.activate_repository(repository_id, cx);
                            cx.notify();
                        })
                        .ok();
                })
                .child(
                    h_flex()
                        .h(self.list_item_height())
                        .min_w_0()
                        .w_full()
                        .text_left()
                        .gap_1()
                        .child(Icon::new(IconName::Folder).size(IconSize::Small).color(
                            if is_active {
                                Color::Accent
                            } else {
                                Color::Muted
                            },
                        ))
                        .child(
                            div().min_w_0().flex_1().text_left().child(
                                Label::new(entry.display_name.clone())
                                    .size(LabelSize::Small)
                                    .truncate(),
                            ),
                        ),
                );

        let branch_button = Button::new(
            ("repository-branch-trigger", repository_id.0),
            entry.branch_label.clone(),
        )
        .size(ButtonSize::None)
        .style(ButtonStyle::Transparent)
        .label_size(LabelSize::XSmall)
        .color(Color::Muted)
        .start_icon(
            Icon::new(IconName::GitBranch)
                .size(IconSize::XSmall)
                .color(Color::Muted),
        )
        .full_width()
        .truncate(true)
        .aria_label(format!("Switch branch in {}", entry.display_name))
        .aria_value(entry.branch_label.clone())
        .when_some(submodule_description, |button, description| {
            button.aria_description(description)
        })
        .tab_index(0isize);

        let branch_selector = PopoverMenu::new(("repository-branch-picker", repository_id.0))
            .menu(move |window, cx| {
                let panel = branch_panel.upgrade()?;
                let (workspace, repository) = panel.read_with(cx, |panel, cx| {
                    (
                        panel.workspace.clone(),
                        panel.repository_for_id(repository_id, cx),
                    )
                });
                let repository = repository?;

                Some(branch_picker::popover(
                    workspace,
                    false,
                    Some(repository),
                    window,
                    cx,
                ))
            })
            .trigger_with_tooltip(branch_button, Tooltip::text(branch_tooltip))
            .full_width(true)
            .anchor(Anchor::BottomLeft);

        h_flex()
            .h(self.list_item_height())
            .min_w_0()
            .w_full()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .when(is_active && !selected, |this| {
                this.bg(cx.theme().colors().ghost_element_selected)
            })
            .when(selected, |this| {
                this.bg(cx.theme().colors().ghost_element_selected)
            })
            .when(selected && self.focus_handle.is_focused(window), |this| {
                this.border_1()
                    .border_r_2()
                    .border_color(cx.theme().colors().panel_focused_border)
            })
            .pl_2p5()
            .gap_1()
            .child(
                div()
                    .w(px(project_depth as f32 * TREE_INDENT))
                    .flex_shrink_0(),
            )
            .child(repository_disclosure)
            .child(
                div()
                    .flex_1()
                    .min_w(rems(3.5))
                    .overflow_hidden()
                    .child(repository_button),
            )
            .child(
                div()
                    .w(rems(4.25))
                    .min_w(rems(1.5))
                    .flex_shrink_1()
                    .overflow_hidden()
                    .child(branch_selector),
            )
            .child(
                h_flex()
                    .min_w(Checkbox::container_size())
                    .flex_shrink_0()
                    .justify_center()
                    .when(entry.change_count > 0, |this| {
                        this.child(
                            Label::new(entry.change_count.to_string())
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                    }),
            )
            .pr_1()
            .into_any_element()
    }

    fn render_list_header(
        &self,
        ix: usize,
        header: &GitHeaderEntry,
        has_write_access: bool,
        repo: &Repository,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id: ElementId = ElementId::Name(format!("header_{}", ix).into());
        let checkbox_id: ElementId = ElementId::Name(format!("header_{}_checkbox", ix).into());
        let group_name: SharedString = format!("header_{}", ix).into();
        let section = header.header;
        let repository_id = repo.id;
        let selected = self.selected_entry == Some(ix);
        let visual_depth = self.project_repository_depth(repository_id) + 1;
        let weak = cx.weak_entity();
        let collapse_weak = cx.weak_entity();
        let stage_intent = StageIntent::for_section(section);
        let toggle_state = stage_intent.checkbox_state(|| self.header_state(header.header, repo));

        let all_conflicts_resolved = section == Section::Conflict
            && self
                .change_entries_for_repository(repo.id)
                .filter(|entry| header.contains(entry, repo))
                .all(|entry| GitPanel::stage_status_for_entry(entry, repo) == StageStatus::Staged);

        let section_is_empty = !self
            .entries
            .get(ix + 1)
            .is_some_and(GitListEntry::is_stageable);
        let section_expanded = !self.collapsed_sections.contains(&(repository_id, section));
        let disclosure_action = if section_expanded {
            "Collapse"
        } else {
            "Expand"
        };
        let repository_name = repo.display_name();

        h_flex()
            .id(id)
            .group(group_name)
            .h(self.list_item_height())
            .w_full()
            .pl_2p5()
            .pr_1()
            .gap_2()
            .justify_between()
            .when(!section_is_empty && !all_conflicts_resolved, |this| {
                this.cursor_pointer()
                    .hover(|s| s.bg(cx.theme().colors().ghost_element_hover))
            })
            .border_1()
            .border_r_2()
            .when(selected, |this| {
                this.bg(cx.theme().colors().ghost_element_selected)
            })
            .when(selected && self.focus_handle.is_focused(window), |this| {
                this.border_color(cx.theme().colors().panel_focused_border)
            })
            .child(
                h_flex()
                    .pl(px(
                        visual_depth as f32 * TREE_INDENT + SECTION_ROW_INDENT_OFFSET
                    ))
                    .gap_1()
                    .child(
                        IconButton::new(
                            ("section-disclosure", ix),
                            if section_expanded {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            },
                        )
                        .size(ButtonSize::None)
                        .style(ButtonStyle::Transparent)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .aria_label(format!(
                            "{disclosure_action} {} in {repository_name}",
                            header.title()
                        ))
                        .aria_expanded(section_expanded)
                        .tab_index(0isize)
                        .disabled(section_is_empty)
                        .tooltip(Tooltip::text(format!(
                            "{disclosure_action} {} in {repository_name}",
                            header.title()
                        )))
                        .on_click(move |_, window, cx| {
                            collapse_weak
                                .update(cx, |this, cx| {
                                    this.selected_entry = Some(ix);
                                    this.toggle_section(repository_id, section, window, cx);
                                    cx.stop_propagation();
                                })
                                .ok();
                        }),
                    )
                    .child(
                        Label::new(header.title())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
            )
            .child(if section_is_empty {
                gpui::Empty.into_any_element()
            } else {
                let checkbox = Checkbox::new(checkbox_id, toggle_state)
                    .disabled(!has_write_access || all_conflicts_resolved)
                    .fill()
                    .elevation(ElevationIndex::Surface);
                let tooltip_label = if all_conflicts_resolved {
                    Some("All conflicts marked as resolved")
                } else {
                    match stage_intent {
                        StageIntent::Stage => Some("Stage All"),
                        StageIntent::Unstage => Some("Unstage All"),
                        StageIntent::Toggle => None,
                    }
                };
                if let Some(label) = tooltip_label {
                    checkbox
                        .tooltip(move |_window, cx| Tooltip::simple(label, cx))
                        .into_any_element()
                } else {
                    checkbox.into_any_element()
                }
            })
            .on_click(move |_, window, cx| {
                weak.update(cx, |this, cx| {
                    this.selected_entry = Some(ix);
                    if !has_write_access || section_is_empty || all_conflicts_resolved {
                        cx.notify();
                        return;
                    }
                    this.toggle_staged_for_entry_in_repository(
                        &GitListEntry::Header(GitHeaderEntry { header: section }),
                        repository_id,
                        stage_intent,
                        window,
                        cx,
                    );
                    cx.stop_propagation();
                })
                .ok();
            })
            .into_any_element()
    }

    fn render_empty_section(&self, ix: usize, section: Section) -> AnyElement {
        let message = match section {
            Section::Staged => "No staged changes yet",
            Section::Unstaged => "No unstaged changes",
            _ => "No changes",
        };
        h_flex()
            .h(self.list_item_height())
            .w_full()
            .pl_2p5()
            .pr_1()
            .opacity(0.8)
            .child(
                h_flex()
                    .pl(px(self.visual_depth_for_entry(ix) as f32 * TREE_INDENT
                        + CONTENT_ROW_INDENT_OFFSET))
                    .child(
                        Label::new(message)
                            .color(Color::Placeholder)
                            .size(LabelSize::Small),
                    ),
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
        let stage_intent = self.stage_intent_for_entry_index(ix);
        let Some(entry) = self.entries.get(ix).and_then(|e| e.status_entry()) else {
            return;
        };
        // Resolve against the pending-op-aware status (like the checkboxes do)
        // so the menu label can't lag behind a just-clicked checkbox.
        let repo = self.repository_for_entry_index(ix, cx);
        let repo = repo.as_ref().map(|repo| repo.read(cx));
        let stage_title = if stage_intent.resolve_with(|| match repo {
            Some(repo) => GitPanel::stage_status_for_entry(entry, repo),
            None => entry.status.staging(),
        }) {
            "Stage File"
        } else {
            "Unstage File"
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
                .separator()
                .action("Unstaged Changes", ViewUnstagedChanges.boxed_clone())
                .action("Staged Changes", ViewStagedChanges.boxed_clone())
                .separator()
                .action_disabled_when(
                    !is_created,
                    "Add to .gitignore",
                    git::AddToGitignore.boxed_clone(),
                )
                .action_disabled_when(
                    !is_created,
                    "Add to .git/info/exclude",
                    git::AddToGitInfoExclude.boxed_clone(),
                )
                .separator()
                .action("Open Diff", menu::Confirm.boxed_clone())
                .action("Open File Diff", menu::SecondaryConfirm.boxed_clone())
                .action("View File", ViewFile.boxed_clone())
                .when(!is_created, |context_menu| {
                    context_menu
                        .separator()
                        .action("View File History", Box::new(git::FileHistory))
                })
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
        let has_tracked_changes = self.has_tracked_changes();
        let has_staged_changes = self.has_staged_changes();
        let has_unstaged_changes = self.has_unstaged_changes();
        let has_new_changes = self.new_count > 0;
        let has_stash_items = self.stash_entries.entries.len() > 0;

        let context_menu = git_panel_context_menu(
            has_tracked_changes,
            has_staged_changes,
            has_unstaged_changes,
            has_new_changes,
            has_stash_items,
            self.focus_handle.clone(),
            window,
            cx,
        );
        self.set_context_menu(context_menu, position, window, cx);
    }

    fn set_context_menu(
        &mut self,
        context_menu: Entity<ContextMenu>,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&context_menu.focus_handle(cx), cx);

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
        repo: &Repository,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let settings = GitPanelSettings::get_global(cx);
        let repository_id = repo.id;
        let visual_depth = depth + self.project_repository_depth(repository_id) + 2;
        let tree_view = settings.tree_view;
        let path_style = self.project.read(cx).path_style(cx);
        let git_path_style = ProjectSettings::get_global(cx).git.path_style;
        let display_name = entry.display_name(path_style);

        let selected = self.selected_entry == Some(ix);
        let marked = self.marked_entries.contains(&ix);
        let status_style = settings.status_style;
        let status = entry.status;
        let file_icon = if settings.file_icons {
            FileIcons::get_icon(entry.repo_path.as_std_path(), cx)
        } else {
            None
        };

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

        let stage_status = GitPanel::stage_status_for_entry(entry, &repo);
        let stage_intent = self.stage_intent_for_entry_index(ix);
        let resolved_conflict = self.is_resolved_conflict(ix, cx);
        let toggle_state = stage_intent.checkbox_state(|| {
            if self.show_placeholders
                && self.active_repository_id == Some(repository_id)
                && !self.has_staged_changes()
                && !entry.status.is_created()
            {
                ToggleState::Selected
            } else {
                match stage_status {
                    StageStatus::Staged => ToggleState::Selected,
                    StageStatus::Unstaged => ToggleState::Unselected,
                    StageStatus::PartiallyStaged => ToggleState::Indeterminate,
                }
            }
        });

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
            .when(settings.file_icons, |this| {
                this.child(
                    file_icon
                        .map(|file_icon| {
                            Icon::from_path(file_icon)
                                .size(IconSize::Small)
                                .color(Color::Muted)
                        })
                        .unwrap_or_else(|| {
                            Icon::new(IconName::File)
                                .size(IconSize::Small)
                                .color(Color::Muted)
                        }),
                )
            })
            .when(status_style != StatusStyle::LabelColor, |el| {
                el.child(git_status_icon(status))
            })
            .map(|this| {
                if tree_view {
                    this.pl(px(
                        visual_depth as f32 * TREE_INDENT + CONTENT_ROW_INDENT_OFFSET
                    ))
                    .child(
                        self.entry_label(display_name, label_color)
                            .when(status.is_deleted(), Label::strikethrough)
                            .truncate(),
                    )
                } else {
                    this.pl(px(
                        visual_depth as f32 * TREE_INDENT + CONTENT_ROW_INDENT_OFFSET
                    ))
                    .child(self.path_formatted(
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

        let id_for_diff_stat = id.clone();

        h_flex()
            .id(id)
            .h(self.list_item_height())
            .w_full()
            .pl_2p5()
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
            .when(GitPanelSettings::get_global(cx).diff_stats, |el| {
                el.when_some(entry.diff_stat, move |this, stat| {
                    let id = format!("diff-stat-{}", id_for_diff_stat);
                    this.child(ui::DiffStat::new(
                        id,
                        stat.added as usize,
                        stat.deleted as usize,
                    ))
                })
            })
            .child(
                div()
                    .id(checkbox_wrapper_id)
                    .flex_none()
                    .occlude()
                    .cursor_pointer()
                    .child(
                        Checkbox::new(checkbox_id, toggle_state)
                            .fill()
                            .elevation(ElevationIndex::Surface)
                            .disabled(!has_write_access || resolved_conflict)
                            .on_click_ext({
                                let entry = entry.clone();
                                let this = cx.weak_entity();
                                move |_, click, window, cx| {
                                    this.update(cx, |this, cx| {
                                        if !has_write_access || resolved_conflict {
                                            return;
                                        }
                                        if click.modifiers().shift {
                                            this.stage_bulk(
                                                ix,
                                                stage_intent != StageIntent::Unstage,
                                                cx,
                                            );
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
                                            this.toggle_staged_for_entry_in_repository(
                                                &list_entry,
                                                repository_id,
                                                stage_intent,
                                                window,
                                                cx,
                                            );
                                        }
                                        cx.stop_propagation();
                                    })
                                    .ok();
                                }
                            })
                            .tooltip(move |_window, cx| {
                                if resolved_conflict {
                                    Tooltip::simple("Conflict marked as resolved", cx)
                                } else {
                                    let action = stage_intent.label(|| stage_status);
                                    Tooltip::for_action(action, &ToggleStaged, cx)
                                }
                            }),
                    ),
            )
            .on_click({
                cx.listener(move |this, event: &ClickEvent, window, cx| {
                    this.selected_entry = Some(ix);
                    cx.notify();
                    this.open_selected_entry_on_click(event.modifiers().secondary(), window, cx);
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
        repo: &Repository,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        // TODO: Have not yet plugged in self.marked_entries. Not sure when and why we need that
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

        let settings = GitPanelSettings::get_global(cx);
        let folder_icon = if settings.folder_icons {
            FileIcons::get_folder_icon(entry.expanded, entry.key.path.as_std_path(), cx)
        } else {
            FileIcons::get_chevron_icon(entry.expanded, cx)
        };
        let fallback_folder_icon = if settings.folder_icons {
            if entry.expanded {
                IconName::FolderOpen
            } else {
                IconName::Folder
            }
        } else {
            if entry.expanded {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            }
        };

        let repository_id = repo.id;
        let visual_depth = entry.depth + self.project_repository_depth(repository_id) + 2;
        let stage_status = self.stage_status_for_directory(entry, repo);

        let stage_intent = StageIntent::for_section(entry.key.section);
        let resolved_conflict = self.is_resolved_conflict(ix, cx);
        let toggle_state = stage_intent.checkbox_state(|| match stage_status {
            StageStatus::Staged => ToggleState::Selected,
            StageStatus::Unstaged => ToggleState::Unselected,
            StageStatus::PartiallyStaged => ToggleState::Indeterminate,
        });

        let name_row = h_flex()
            .min_w_0()
            .gap_1()
            .pl(px(
                visual_depth as f32 * TREE_INDENT + CONTENT_ROW_INDENT_OFFSET
            ))
            .child(
                folder_icon
                    .map(|folder_icon| {
                        Icon::from_path(folder_icon)
                            .size(IconSize::Small)
                            .color(Color::Muted)
                    })
                    .unwrap_or_else(|| {
                        Icon::new(fallback_folder_icon)
                            .size(IconSize::Small)
                            .color(Color::Muted)
                    }),
            )
            .child(self.entry_label(entry.name.clone(), label_color).truncate());

        h_flex()
            .id(id)
            .h(self.list_item_height())
            .min_w_0()
            .w_full()
            .pl_2p5()
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
                            .disabled(!has_write_access || resolved_conflict)
                            .fill()
                            .elevation(ElevationIndex::Surface)
                            .on_click({
                                let entry = entry.clone();
                                let this = cx.weak_entity();
                                move |_, window, cx| {
                                    this.update(cx, |this, cx| {
                                        if !has_write_access || resolved_conflict {
                                            return;
                                        }
                                        this.toggle_staged_for_entry_in_repository(
                                            &GitListEntry::Directory(entry.clone()),
                                            repository_id,
                                            stage_intent,
                                            window,
                                            cx,
                                        );
                                        cx.stop_propagation();
                                    })
                                    .ok();
                                }
                            })
                            .tooltip(move |_window, cx| {
                                if resolved_conflict {
                                    Tooltip::simple("Conflicts marked as resolved", cx)
                                } else {
                                    let action = stage_intent.label(|| stage_status);
                                    Tooltip::simple(format!("{action} Folder"), cx)
                                }
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

    pub fn load_commit_template(
        &self,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Option<GitCommitTemplate>>> {
        let Some(repo) = self.active_repository.clone() else {
            return Task::ready(Err(anyhow::anyhow!("no active repo")));
        };
        repo.update(cx, |repo, cx| {
            let rx = repo.load_commit_template_text();
            cx.spawn(async move |_, _| rx.await?)
        })
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
            .read_with(&cx, |workspace, cx| {
                Self::serialization_key(workspace).map(|key| (key, KeyValueStore::global(cx)))
            })
            .ok()
            .flatten()
        {
            Some((serialization_key, kvp)) => cx
                .background_spawn(async move { kvp.read_kvp(&serialization_key) })
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
            GitPanel::new_with_serialized_panel(workspace, serialized_panel, window, cx)
        })
    }

    fn stage_bulk(&mut self, mut index: usize, stage: bool, cx: &mut Context<'_, Self>) {
        let Some(op) = self.bulk_staging.clone() else {
            return;
        };
        if self.repository_id_for_entry_index(index) != Some(op.repo_id) {
            return;
        }
        let Some(mut anchor_index) = self.entry_by_change_key(&ChangeKey {
            repository_id: op.repo_id,
            repo_path: op.anchor.clone(),
        }) else {
            return;
        };
        // Only a staged anchor survives the next entries refresh, so there's no
        // point re-anchoring on the entry we're about to unstage.
        if stage
            && let Some(entry) = self.entries.get(index)
            && let Some(entry) = entry.status_entry()
        {
            self.set_bulk_staging_anchor_for_repository(op.repo_id, entry.repo_path.clone());
        }
        if index < anchor_index {
            std::mem::swap(&mut index, &mut anchor_index);
        }
        let Some(repository) = self.repository_for_id(op.repo_id, cx) else {
            return;
        };
        let entries = {
            let repo = repository.read(cx);
            // Conflicts only change staging via their own explicit controls; a
            // range sweep must neither mark them resolved nor un-resolve them.
            (anchor_index..=index)
                .filter(|ix| self.repository_id_for_entry_index(*ix) == Some(op.repo_id))
                .filter_map(|ix| self.entries.get(ix)?.status_entry().cloned())
                .filter(|entry| !repo.had_conflict_on_last_merge_head_change(&entry.repo_path))
                .collect::<Vec<_>>()
        };
        self.change_file_stage_for_repository(repository, stage, entries, cx);
    }

    #[cfg(test)]
    fn set_bulk_staging_anchor(&mut self, path: RepoPath, cx: &mut Context<'_, GitPanel>) {
        let Some(repo) = self.active_repository.as_ref() else {
            return;
        };
        self.set_bulk_staging_anchor_for_repository(repo.read(cx).id, path);
    }

    fn set_bulk_staging_anchor_for_repository(
        &mut self,
        repository_id: RepositoryId,
        path: RepoPath,
    ) {
        self.bulk_staging = Some(BulkStaging {
            repo_id: repository_id,
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

struct GenerateCommitMessageConfigurationTooltip;

impl Render for GenerateCommitMessageConfigurationTooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui::tooltip_container(cx, |container, _cx| {
            container
                .gap_1p5()
                .child(Label::new(
                    "Configure an LLM provider to generate commit messages.",
                ))
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Button::new("configure-commit-message-provider", "Configure Provider")
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ModalSurface)
                                .label_size(LabelSize::Small)
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(
                                        zed_actions::OpenSettingsAt {
                                            path: "llm_providers".to_string(),
                                            target: None,
                                        }
                                        .boxed_clone(),
                                        cx,
                                    );
                                }),
                        )
                        .child(
                            Button::new("llm-provider-docs", "See Docs")
                                .style(ButtonStyle::OutlinedGhost)
                                .end_icon(
                                    Icon::new(IconName::ArrowUpRight)
                                        .color(Color::Muted)
                                        .size(IconSize::Small),
                                )
                                .label_size(LabelSize::Small)
                                .on_click(move |_, _, cx| {
                                    cx.open_url(&zed_urls::llm_provider_docs(cx))
                                }),
                        ),
                )
        })
    }
}

impl GitPanel {
    pub fn selected_file_history_target(&self, cx: &App) -> Option<(Entity<Repository>, RepoPath)> {
        let selected_index = self.selected_entry?;
        let entry = self.entries.get(selected_index)?.status_entry()?;
        let repository = self.repository_for_entry_index(selected_index, cx)?;
        if entry.status.is_created() {
            return None;
        }
        Some((repository, entry.repo_path.clone()))
    }
}

#[cfg(any(test, feature = "test-support"))]
impl GitPanel {
    pub fn new_test(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        Self::new(workspace, window, cx)
    }

    pub fn active_repository(&self) -> Option<&Entity<Repository>> {
        self.active_repository.as_ref()
    }
}

impl Render for GitPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project = self.project.read(cx);
        let has_entries = !self.entries.is_empty();
        let has_write_access = self.has_write_access(cx);

        #[cfg(feature = "call")]
        let has_co_authors = self
            .workspace
            .upgrade()
            .and_then(|_workspace| {
                call::ActiveCall::try_global(cx).and_then(|call| call.read(cx).room().cloned())
            })
            .is_some_and(|room| {
                self.load_local_committer(cx);
                let room = room.read(cx);
                room.remote_participants()
                    .values()
                    .any(|remote_participant| remote_participant.can_write())
            });
        #[cfg(not(feature = "call"))]
        let has_co_authors = false;

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
                    .on_action(cx.listener(Self::add_to_git_info_exclude))
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
            .on_action(cx.listener(Self::open_solo_diff))
            .on_action(cx.listener(Self::view_file))
            .on_action(cx.listener(Self::view_unstaged_changes))
            .on_action(cx.listener(Self::view_staged_changes))
            .on_action(cx.listener(Self::focus_changes_list))
            .on_action(cx.listener(Self::focus_editor))
            .on_action(cx.listener(Self::expand_commit_editor))
            .when(has_write_access && has_co_authors, |git_panel| {
                git_panel.on_action(cx.listener(Self::toggle_fill_co_authors))
            })
            .on_action(cx.listener(Self::set_sort_by_path))
            .on_action(cx.listener(Self::set_sort_by_name))
            .on_action(cx.listener(Self::set_group_by_none))
            .on_action(cx.listener(Self::set_group_by_status))
            .on_action(cx.listener(Self::set_group_by_staging))
            .on_action(cx.listener(Self::toggle_tree_view))
            .on_action(cx.listener(Self::show_current_repository))
            .on_action(cx.listener(Self::show_all_repositories))
            .on_action(cx.listener(Self::increase_font_size))
            .on_action(cx.listener(Self::decrease_font_size))
            .on_action(cx.listener(Self::reset_font_size))
            .on_action(cx.listener(Self::activate_changes_tab))
            .on_action(cx.listener(Self::activate_history_tab))
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().colors().panel_background)
            .child(
                v_flex()
                    .size_full()
                    .when(!self.commit_editor_expanded, |this| {
                        this.child(self.render_tab_bar(cx))
                    })
                    .map(|this| match self.active_tab {
                        GitPanelTab::Changes => this
                            .children(self.render_changes_header(window, cx))
                            .when(!self.commit_editor_expanded, |this| {
                                this.map(|this| {
                                    if has_entries {
                                        this.child(self.render_entries(
                                            has_write_access,
                                            window,
                                            cx,
                                        ))
                                    } else {
                                        this.child(self.render_empty_state(cx).into_any_element())
                                    }
                                })
                            })
                            .children(self.render_footer(window, cx))
                            .when(self.amend_pending, |this| {
                                this.child(self.render_pending_amend(cx))
                            })
                            .when(!self.amend_pending, |this| {
                                this.children(self.render_previous_commit(window, cx))
                            }),
                        GitPanelTab::History => this.child(self.render_history_tab(window, cx)),
                    })
                    .into_any_element(),
            )
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Anchor::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl Focusable for GitPanel {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        if self.entries.is_empty() || self.commit_editor_expanded {
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
        _excerpt_info: &ExcerptBoundaryInfo,
        buffer: &language::BufferSnapshot,
        window: &Window,
        cx: &App,
    ) -> Option<AnyElement> {
        let file = buffer.file()?;
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

    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        GitPanelSettings::get_global(cx).default_width
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch).filter(|_| GitPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn icon_label(&self, _: &Window, cx: &App) -> Option<String> {
        if !GitPanelSettings::get_global(cx).show_count_badge {
            return None;
        }
        let total = self.changes_count;
        (total > 0).then(|| total.to_string())
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &Window, cx: &App) -> bool {
        GitPanelSettings::get_global(cx).starts_open
    }

    fn activation_priority(&self) -> u32 {
        3
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.git_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

impl PanelHeader for GitPanel {}

pub fn panel_editor_container(_window: &mut Window, cx: &mut App) -> Div {
    v_flex()
        .size_full()
        .bg(cx.theme().colors().editor_background)
}

pub(crate) fn git_commit_editor_style(font_size: gpui::Pixels, cx: &App) -> EditorStyle {
    let settings = ThemeSettings::get_global(cx);

    EditorStyle {
        background: cx.theme().colors().editor_background,
        local_player: cx.theme().players().local(),
        text: TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.buffer_font.family.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: AbsoluteLength::from(font_size),
            font_weight: settings.buffer_font.weight,
            line_height: (font_size * settings.buffer_line_height.value()).into(),
            ..Default::default()
        },
        syntax: cx.theme().syntax().clone(),
        ..Default::default()
    }
}

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
                    tag_names: Vec::new(),
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

        let repo_selector = PopoverMenu::new("repository-switcher")
            .menu({
                let project = project;
                move |window, cx| {
                    let project = project.clone()?;
                    Some(cx.new(|cx| RepositorySelector::new(project, rems(20.), window, cx)))
                }
            })
            .trigger_with_tooltip(
                Button::new("repo-selector", active_repo_name)
                    .size(ButtonSize::None)
                    .label_size(LabelSize::Small)
                    .truncate(true),
                move |_, cx| {
                    if single_repo {
                        cx.new(|_| Empty).into()
                    } else {
                        Tooltip::simple("Switch Active Repository", cx)
                    }
                },
            )
            .anchor(Anchor::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .into_any_element();

        let branch_selector_button = Button::new("branch-selector", branch_name)
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
            .anchor(Anchor::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            });

        h_flex()
            .w_full()
            .px_2()
            .py_1p5()
            .justify_between()
            .gap_1()
            .child(
                h_flex()
                    .flex_1()
                    .overflow_hidden()
                    .gap_px()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::Small).color(
                        if single_repo {
                            Color::Disabled
                        } else {
                            Color::Muted
                        },
                    ))
                    .when(!single_repo, |this| {
                        this.child(div().child(repo_selector).min_w_0()).when(
                            show_separator,
                            |this| {
                                this.child(Label::new("/").size(LabelSize::Small).color(
                                    Color::Custom(cx.theme().colors().text_muted.opacity(0.4)),
                                ))
                            },
                        )
                    })
                    .child(div().child(branch_selector).min_w_0()),
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

    fn description() -> &'static str {
        "The footer shown at the bottom of the git panel."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
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
            .into_any_element()
    }
}

pub(crate) fn open_output(
    operation: impl Into<SharedString>,
    workspace: &mut Workspace,
    output: &str,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let operation = operation.into();

    let plain_text = terminal::strip_ansi_text(output.as_bytes());

    let buffer = cx.new(|cx| Buffer::local(plain_text.as_str(), cx));
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
    let message = format_git_error_toast_message(&e);
    if message
        .matches(git::repository::REMOTE_CANCELLED_BY_USER)
        .next()
        .is_some()
    { // Hide the cancelled by user message
    } else {
        cx.defer(move |cx| {
            workspace.update(cx, |workspace, cx| {
                let workspace_weak = cx.weak_entity();
                let toast = StatusToast::new(format!("git {} failed", action), cx, |this, _cx| {
                    this.icon(
                        Icon::new(IconName::XCircle)
                            .size(IconSize::Small)
                            .color(Color::Error),
                    )
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
        });
    }
}

fn rpc_error_raw_message_from_chain(error: &anyhow::Error) -> Option<&str> {
    error
        .chain()
        .find_map(|cause| cause.downcast_ref::<RpcError>().map(RpcError::raw_message))
}

fn format_git_error_toast_message(error: &anyhow::Error) -> String {
    if let Some(message) = rpc_error_raw_message_from_chain(error) {
        message.trim().to_string()
    } else {
        error.to_string().trim().to_string()
    }
}

pub(crate) fn commit_title_exceeds_limit(title: &str, max_length: usize) -> bool {
    max_length > 0 && title.chars().count() > max_length
}

#[cfg(test)]
mod tests {
    use git::{
        repository::repo_path,
        status::{StatusCode, TrackedStatus, UnmergedStatus, UnmergedStatusCode},
    };
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext, px};
    use indoc::indoc;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use theme::LoadThemes;
    use util::path;
    use util::rel_path::rel_path;

    use workspace::MultiWorkspace;

    use super::*;

    fn init_test(cx: &mut gpui::TestAppContext) {
        zlog::init_test();

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(LoadThemes::JustBase, cx);
            language_model::init(cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    #[test]
    fn test_tree_view_directory_expansion_is_scoped_to_section() {
        let entry = |path, status| GitStatusEntry {
            repo_path: repo_path(path),
            status,
            staging: StageStatus::Unstaged,
            diff_stat: None,
        };
        let mut state = TreeViewState::default();
        let mut seen_directories = HashSet::default();
        let repository_id = RepositoryId(1);

        state.build_tree_entries(
            repository_id,
            Section::Tracked,
            vec![entry("src/tracked.rs", StatusCode::Modified.worktree())],
            &mut seen_directories,
        );
        state.build_tree_entries(
            repository_id,
            Section::New,
            vec![entry("src/new.rs", FileStatus::Untracked)],
            &mut seen_directories,
        );

        let tracked_key = TreeKey {
            repository_id,
            section: Section::Tracked,
            path: repo_path("src"),
        };
        let new_key = TreeKey {
            repository_id,
            section: Section::New,
            path: repo_path("src"),
        };
        state.expanded_dirs.insert(tracked_key.clone(), false);

        let tracked_entries = state.build_tree_entries(
            repository_id,
            Section::Tracked,
            vec![entry("src/tracked.rs", StatusCode::Modified.worktree())],
            &mut seen_directories,
        );
        let new_entries = state.build_tree_entries(
            repository_id,
            Section::New,
            vec![entry("src/new.rs", FileStatus::Untracked)],
            &mut seen_directories,
        );

        assert_eq!(state.expanded_dirs.get(&tracked_key), Some(&false));
        assert_eq!(state.expanded_dirs.get(&new_key), Some(&true));
        assert!(matches!(
            tracked_entries.first(),
            Some((GitListEntry::Directory(entry), _)) if !entry.expanded
        ));
        assert!(matches!(
            new_entries.first(),
            Some((GitListEntry::Directory(entry), _)) if entry.expanded
        ));
    }

    fn register_git_commit_language(project: &Entity<Project>, cx: &mut VisualTestContext) {
        project.read_with(cx, |project, _| {
            project.languages().add(Arc::new(language::Language::new(
                language::LanguageConfig {
                    name: "Git Commit".into(),
                    ..Default::default()
                },
                None,
            )));
        });
    }

    fn entry_index_for_repo_path(panel: &GitPanel, repo_path: &RepoPath) -> Option<usize> {
        panel.entries.iter().position(|entry| {
            entry
                .status_entry()
                .is_some_and(|entry| &entry.repo_path == repo_path)
        })
    }

    async fn await_git_panel_entries(panel: &Entity<GitPanel>, cx: &mut VisualTestContext) {
        let handle = cx.update_window_entity(panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;
    }

    fn assert_editor_opened_with_path(
        workspace: &Entity<Workspace>,
        expected_path: &Path,
        cx: &mut VisualTestContext,
    ) {
        workspace.update_in(cx, |workspace, _window, cx| {
            let editor = workspace
                .item_of_type::<editor::Editor>(cx)
                .expect("Editor should exist after View File");
            let file_path = editor
                .read(cx)
                .active_buffer(cx)
                .expect("Buffer should have an active buffer")
                .read(cx)
                .file()
                .cloned()
                .expect("Buffer should have a file");
            assert_eq!(file_path.path().as_ref().as_std_path(), expected_path);
        });
    }

    async fn setup_git_panel_with_changes(
        cx: &mut TestAppContext,
        tree: serde_json::Value,
        status_entries: &[(&str, git::status::StatusCode)],
    ) -> (
        Entity<Project>,
        Entity<Workspace>,
        Entity<GitPanel>,
        VisualTestContext,
    ) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/project"), tree).await;

        if !status_entries.is_empty() {
            fs.set_status_for_repo(
                path!("/project/.git").as_ref(),
                &status_entries
                    .iter()
                    .map(|(path, status)| (*path, status.worktree()))
                    .collect::<Vec<_>>(),
            );
        }

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
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

        cx.executor().run_until_parked();

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        (project, workspace, panel, cx)
    }

    #[gpui::test]
    async fn test_view_file_tracked(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "tracked": "tracked\n",
            }),
        )
        .await;

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("tracked", "old tracked\n".into())],
        );

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
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

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let entry_index = panel
            .read_with(&cx, |panel, _| {
                entry_index_for_repo_path(panel, &repo_path("tracked"))
            })
            .expect("tracked file should exist in the changes list");

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(entry_index);
            panel.view_file(&ViewFile, window, cx);
        });
        cx.run_until_parked();

        assert_editor_opened_with_path(&workspace, Path::new("tracked"), &mut cx);
    }

    #[gpui::test]
    async fn test_view_file_untracked(cx: &mut TestAppContext) {
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

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
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

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().sort_by = Some(GitPanelSortBy::Path);
                })
            });
        });

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let entry_index = panel
            .read_with(&cx, |panel, _| {
                entry_index_for_repo_path(panel, &repo_path("untracked"))
            })
            .expect("untracked file should exist in the changes list");

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(entry_index);
            panel.view_file(&ViewFile, window, cx);
        });
        cx.run_until_parked();

        assert_editor_opened_with_path(&workspace, Path::new("untracked"), &mut cx);
    }

    #[gpui::test]
    async fn test_view_file_tree_view(cx: &mut TestAppContext) {
        init_test(cx);

        let (_project, workspace, panel, mut cx) = setup_git_panel_with_changes(
            cx,
            json!({
                ".git": {},
                "src": {
                    "a": {
                        "foo.rs": "fn foo() {}",
                    },
                },
            }),
            &[("src/a/foo.rs", StatusCode::Modified)],
        )
        .await;

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().tree_view = Some(true);
                })
            });
        });
        await_git_panel_entries(&panel, &mut cx).await;

        let entry_index = panel
            .read_with(&cx, |panel, _| {
                entry_index_for_repo_path(panel, &repo_path("src/a/foo.rs"))
            })
            .expect("foo.rs should exist in the tree view changes list");

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(entry_index);
            panel.view_file(&ViewFile, window, cx);
        });
        cx.run_until_parked();

        assert_editor_opened_with_path(&workspace, Path::new("src/a/foo.rs"), &mut cx);
    }

    async fn history_panel_for_project(
        fs: Arc<FakeFs>,
        cx: &mut TestAppContext,
    ) -> Entity<GitPanel> {
        let project = Project::test(fs, [Path::new(path!("/root/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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

        let panel = workspace.update_in(cx, GitPanel::new);
        panel.update_in(cx, |panel, window, cx| {
            panel.activate_history_tab(&ActivateHistoryTab, window, cx);
        });
        cx.run_until_parked();
        panel
    }

    async fn wait_for_commit_history_to_settle(panel: &Entity<GitPanel>, cx: &mut TestAppContext) {
        cx.condition(panel, |panel, _| {
            !matches!(panel.commit_history, CommitHistory::Loading)
        })
        .await;
    }

    #[test]
    fn test_format_git_error_toast_message_prefers_raw_rpc_message() {
        let rpc_error = RpcError::from_proto(
            &proto::Error {
                message:
                    "Your local changes to the following files would be overwritten by merge\n"
                        .to_string(),
                code: proto::ErrorCode::Internal as i32,
                tags: Default::default(),
            },
            "Pull",
        );

        let message = format_git_error_toast_message(&rpc_error);
        assert_eq!(
            message,
            "Your local changes to the following files would be overwritten by merge"
        );
    }

    #[test]
    fn test_format_git_error_toast_message_prefers_raw_rpc_message_when_wrapped() {
        let rpc_error = RpcError::from_proto(
            &proto::Error {
                message:
                    "Your local changes to the following files would be overwritten by merge\n"
                        .to_string(),
                code: proto::ErrorCode::Internal as i32,
                tags: Default::default(),
            },
            "Pull",
        );
        let wrapped = rpc_error.context("sending pull request");

        let message = format_git_error_toast_message(&wrapped);
        assert_eq!(
            message,
            "Your local changes to the following files would be overwritten by merge"
        );
    }

    #[gpui::test]
    async fn test_history_tab_stops_loading_for_unborn_branch(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "project": { ".git": {} } }))
            .await;

        let dot_git = Path::new(path!("/root/project/.git"));
        fs.set_branch_name(dot_git, Some("main"));
        fs.with_git_state(dot_git, false, |state| {
            state.refs.remove("HEAD");
        })
        .unwrap();

        let panel = history_panel_for_project(fs.clone(), cx).await;

        wait_for_commit_history_to_settle(&panel, cx).await;
        panel.read_with(cx, |panel, _| {
            assert_eq!(panel.commit_history, CommitHistory::Loaded(Rc::from([])));
        });
    }

    #[gpui::test]
    async fn test_history_tab_loads_detached_head(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "project": { ".git": {} } }))
            .await;

        let dot_git = Path::new(path!("/root/project/.git"));
        let sha: Oid = "0123456789012345678901234567890123456789".parse().unwrap();
        fs.with_git_state(dot_git, false, |state| {
            state.current_branch_name = None;
            state.refs.insert("HEAD".into(), sha.to_string());
            state.graph_commits = vec![Arc::new(git::repository::InitialGraphCommitData {
                sha,
                parents: SmallVec::new(),
                ref_names: Vec::new(),
            })];
        })
        .unwrap();

        let panel = history_panel_for_project(fs.clone(), cx).await;

        wait_for_commit_history_to_settle(&panel, cx).await;
        panel.read_with(cx, |panel, _| {
            assert_eq!(
                panel.commit_history,
                CommitHistory::Loaded(Rc::from([CommitHistoryEntry {
                    sha,
                    tag_names: Vec::new(),
                }]))
            );
        });
    }

    #[gpui::test]
    async fn test_history_tab_surfaces_load_error(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "project": { ".git": {} } }))
            .await;

        let dot_git = Path::new(path!("/root/project/.git"));
        let sha: Oid = "0123456789012345678901234567890123456789".parse().unwrap();
        fs.with_git_state(dot_git, false, |state| {
            state.current_branch_name = None;
            state.refs.insert("HEAD".into(), sha.to_string());
            state.graph_commits = vec![Arc::new(git::repository::InitialGraphCommitData {
                sha,
                parents: SmallVec::new(),
                ref_names: Vec::new(),
            })];
        })
        .unwrap();
        fs.set_graph_error(dot_git, Some("simulated git log failure".into()));

        let panel = history_panel_for_project(fs.clone(), cx).await;

        wait_for_commit_history_to_settle(&panel, cx).await;
        panel.read_with(cx, |panel, _| {
            assert!(matches!(panel.commit_history, CommitHistory::Error(_)));
        });
    }

    #[gpui::test]
    async fn test_history_tab_without_repository(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree("/root", json!({ "project": {} })).await;

        let panel = history_panel_for_project(fs.clone(), cx).await;

        panel.read_with(cx, |panel, _| {
            assert_eq!(panel.commit_history, CommitHistory::Loading);
        });
    }

    #[test]
    fn test_commit_history_from_response() {
        let sha: Oid = "0123456789012345678901234567890123456789".parse().unwrap();
        let error = SharedString::from("git log failed");
        let entries: Rc<[CommitHistoryEntry]> = Rc::from([CommitHistoryEntry {
            sha,
            tag_names: Vec::new(),
        }]);
        let no_entries: Rc<[CommitHistoryEntry]> = Rc::from([]);

        // Commits win even while the fetch task still reports `is_loading`.
        assert_eq!(
            commit_history_from_response(entries.clone(), true, None),
            CommitHistory::Loaded(entries.clone())
        );
        assert_eq!(
            commit_history_from_response(entries.clone(), false, None),
            CommitHistory::Loaded(entries.clone())
        );
        // Commits also take precedence over a concurrently reported error.
        assert_eq!(
            commit_history_from_response(entries.clone(), true, Some(error.clone())),
            CommitHistory::Loaded(entries)
        );

        // With no commits a terminal error beats the loading state.
        assert_eq!(
            commit_history_from_response(no_entries.clone(), true, Some(error.clone())),
            CommitHistory::Error(error.clone())
        );
        assert_eq!(
            commit_history_from_response(no_entries.clone(), false, Some(error.clone())),
            CommitHistory::Error(error)
        );

        // When no commits and no error, loading vs. finished-empty hinges on `is_loading`.
        assert_eq!(
            commit_history_from_response(no_entries.clone(), true, None),
            CommitHistory::Loading
        );
        assert_eq!(
            commit_history_from_response(no_entries.clone(), false, None),
            CommitHistory::Loaded(no_entries)
        );
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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

        let panel = workspace.update_in(cx, GitPanel::new);

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
                    diff_stat: Some(DiffStat {
                        added: 1,
                        deleted: 1,
                    }),
                }),
                GitListEntry::Status(GitStatusEntry {
                    repo_path: repo_path("crates/util/util.rs"),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                    diff_stat: Some(DiffStat {
                        added: 1,
                        deleted: 1,
                    }),
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
                    diff_stat: Some(DiffStat {
                        added: 1,
                        deleted: 1,
                    }),
                }),
                GitListEntry::Status(GitStatusEntry {
                    repo_path: repo_path("crates/util/util.rs"),
                    status: StatusCode::Modified.worktree(),
                    staging: StageStatus::Unstaged,
                    diff_stat: Some(DiffStat {
                        added: 1,
                        deleted: 1,
                    }),
                },),
            ],
        );
    }

    #[gpui::test]
    async fn test_discard_prompt_escapes_markdown_in_file_name(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project": {
                    ".git": {},
                    "__somefile__": "modified\n",
                },
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project/.git")),
            &[("__somefile__", StatusCode::Modified.worktree())],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/root/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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

        let panel = workspace.update_in(cx, GitPanel::new);

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        panel.update_in(cx, |panel, window, cx| {
            panel.selected_entry = Some(1);
            panel.revert_selected(&git::RestoreFile::default(), window, cx);
        });

        let (message, _detail) = cx
            .pending_prompt()
            .expect("discard should show a confirmation prompt");

        assert_eq!(
            message,
            "Are you sure you want to discard changes to `__somefile__`?"
        );
    }

    #[gpui::test]
    async fn test_group_by_staging_section_membership_and_order(cx: &mut TestAppContext) {
        use GitListEntry::*;

        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "conflict.rs": "conflicted content",
                "new.rs": "new content",
                "partial.rs": "partial content",
                "partial_new.rs": "partial new content",
                "staged.rs": "staged content",
                "unstaged.rs": "unstaged content",
            }),
        )
        .await;

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                (
                    "conflict.rs",
                    UnmergedStatus {
                        first_head: UnmergedStatusCode::Updated,
                        second_head: UnmergedStatusCode::Updated,
                    }
                    .into(),
                ),
                ("new.rs", FileStatus::Untracked),
                (
                    "partial.rs",
                    TrackedStatus {
                        index_status: StatusCode::Modified,
                        worktree_status: StatusCode::Modified,
                    }
                    .into(),
                ),
                (
                    "partial_new.rs",
                    TrackedStatus {
                        index_status: StatusCode::Added,
                        worktree_status: StatusCode::Modified,
                    }
                    .into(),
                ),
                ("staged.rs", FileStatus::index(StatusCode::Modified)),
                ("unstaged.rs", StatusCode::Modified.worktree()),
            ],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Staging);
                })
            });
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

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let entries = panel.read_with(&mut cx, |panel, _| {
            assert_eq!(panel.entry_count, 6);
            assert_eq!(
                panel
                    .change_entries_by_path()
                    .filter(|entry| entry.status.is_created())
                    .map(|entry| &*entry.repo_path)
                    .sorted()
                    .collect::<Vec<_>>(),
                [rel_path("new.rs"), rel_path("partial_new.rs")]
            );

            let partial_path = repo_path("partial.rs");
            let projections = panel
                .projected_entries_by_path
                .get(&ChangeKey {
                    repository_id: panel.active_repository_id.unwrap(),
                    repo_path: partial_path,
                })
                .expect("partially staged entry should have projections");
            assert_eq!(
                projections.as_slice(),
                &[
                    ProjectedChangeEntry {
                        section: Section::Staged,
                        index: 3,
                    },
                    ProjectedChangeEntry {
                        section: Section::Unstaged,
                        index: 8,
                    },
                ]
            );
            assert_eq!(
                panel.stage_intent_for_entry_index(projections[0].index),
                StageIntent::Unstage
            );
            assert_eq!(
                panel.stage_intent_for_entry_index(projections[1].index),
                StageIntent::Stage
            );
            panel.entries.clone()
        });

        #[rustfmt::skip]
        pretty_assertions::assert_matches!(
            entries.as_slice(),
            &[
                Header(GitHeaderEntry { header: Section::Conflict }),
                Status(GitStatusEntry { status: FileStatus::Unmerged(..), staging: StageStatus::Unstaged, .. }),
                Header(GitHeaderEntry { header: Section::Staged }),
                Status(GitStatusEntry { staging: StageStatus::PartiallyStaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::PartiallyStaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Staged, .. }),
                Header(GitHeaderEntry { header: Section::Unstaged }),
                Status(GitStatusEntry { status: FileStatus::Untracked, staging: StageStatus::Unstaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::PartiallyStaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::PartiallyStaged, .. }),
                Status(GitStatusEntry { staging: StageStatus::Unstaged, .. }),
            ],
        );
        assert_entry_paths(
            &entries,
            &[
                None,
                Some("conflict.rs"),
                None,
                Some("partial.rs"),
                Some("partial_new.rs"),
                Some("staged.rs"),
                None,
                Some("new.rs"),
                Some("partial.rs"),
                Some("partial_new.rs"),
                Some("unstaged.rs"),
            ],
        );

        let worktree_id =
            cx.read(|cx| project.read(cx).worktrees(cx).next().unwrap().read(cx).id());
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.select_entry_by_path(
                ProjectPath {
                    worktree_id,
                    path: rel_path("partial.rs").into_arc(),
                },
                window,
                cx,
            );
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(
                panel.selected_entry,
                panel.entry_by_path_in_section(&repo_path("partial.rs"), Section::Staged)
            );
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry =
                panel.entry_by_path_in_section(&repo_path("partial.rs"), Section::Unstaged);
            panel.select_entry_by_path(
                ProjectPath {
                    worktree_id,
                    path: rel_path("partial.rs").into_arc(),
                },
                window,
                cx,
            );
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(
                panel.selected_entry,
                panel.entry_by_path_in_section(&repo_path("partial.rs"), Section::Unstaged)
            );
        });

        panel.update_in(&mut cx, |panel, _window, _cx| {
            panel.selected_entry =
                panel.entry_by_path_in_section(&repo_path("partial.rs"), Section::Staged);
        });

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                (
                    "conflict.rs",
                    UnmergedStatus {
                        first_head: UnmergedStatusCode::Updated,
                        second_head: UnmergedStatusCode::Updated,
                    }
                    .into(),
                ),
                ("new.rs", FileStatus::Untracked),
                ("partial.rs", StatusCode::Modified.worktree()),
                (
                    "partial_new.rs",
                    TrackedStatus {
                        index_status: StatusCode::Added,
                        worktree_status: StatusCode::Modified,
                    }
                    .into(),
                ),
                ("staged.rs", FileStatus::index(StatusCode::Modified)),
                ("unstaged.rs", StatusCode::Modified.worktree()),
            ],
        );
        cx.run_until_parked();
        await_git_panel_entries(&panel, &mut cx).await;

        panel.read_with(&cx, |panel, _| {
            let selected_entry = panel
                .selected_entry
                .and_then(|index| panel.entries.get(index))
                .and_then(GitListEntry::status_entry)
                .expect("selected change should remain selected");
            assert_eq!(selected_entry.repo_path, repo_path("partial.rs"));
            assert_eq!(
                panel.selected_entry,
                panel.entry_by_path_in_section(&repo_path("partial.rs"), Section::Unstaged)
            );
        });
    }

    #[gpui::test]
    async fn test_staging_conflict_mark_resolved_transition(cx: &mut TestAppContext) {
        use GitListEntry::*;

        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "conflict.rs": "<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n",
            }),
        )
        .await;

        let unresolved_status = FileStatus::Unmerged(UnmergedStatus {
            first_head: UnmergedStatusCode::Updated,
            second_head: UnmergedStatusCode::Updated,
        });
        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[("conflict.rs", unresolved_status)],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Staging);
                })
            });
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

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let conflict_entry = panel.read_with(&cx, |panel, _| {
            pretty_assertions::assert_matches!(
                panel.entries.as_slice(),
                &[
                    Header(GitHeaderEntry {
                        header: Section::Conflict
                    }),
                    Status(GitStatusEntry {
                        status: FileStatus::Unmerged(..),
                        ..
                    }),
                    Header(GitHeaderEntry {
                        header: Section::Staged
                    }),
                    EmptySection(Section::Staged),
                    Header(GitHeaderEntry {
                        header: Section::Unstaged
                    }),
                    EmptySection(Section::Unstaged),
                ],
            );
            panel
                .entries
                .get(1)
                .and_then(GitListEntry::status_entry)
                .cloned()
                .expect("conflict entry should exist")
        });

        panel.update_in(&mut cx, |panel, _window, cx| {
            panel.change_file_stage(true, vec![conflict_entry.clone()], cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _| {
            assert!(matches!(
                panel.entries.as_slice(),
                [
                    Header(GitHeaderEntry {
                        header: Section::Conflict
                    }),
                    Status(GitStatusEntry {
                        status: FileStatus::Unmerged(..),
                        ..
                    }),
                    Header(GitHeaderEntry {
                        header: Section::Staged
                    }),
                    EmptySection(Section::Staged),
                    Header(GitHeaderEntry {
                        header: Section::Unstaged
                    }),
                    EmptySection(Section::Unstaged),
                ]
            ));
        });

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[("conflict.rs", FileStatus::index(StatusCode::Modified))],
        );
        cx.run_until_parked();
        await_git_panel_entries(&panel, &mut cx).await;

        panel.read_with(&cx, |panel, _| {
            pretty_assertions::assert_matches!(
                panel.entries.as_slice(),
                &[
                    Header(GitHeaderEntry {
                        header: Section::Staged
                    }),
                    Status(GitStatusEntry {
                        staging: StageStatus::Staged,
                        ..
                    }),
                    Header(GitHeaderEntry {
                        header: Section::Unstaged
                    }),
                    EmptySection(Section::Unstaged),
                ],
            );
            assert_eq!(panel.entry_count, 1);
        });
    }

    #[gpui::test]
    async fn test_resolved_conflict_is_locked_against_unstaging(cx: &mut TestAppContext) {
        use GitListEntry::*;

        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "conflict.rs": "<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n",
                "staged.rs": "staged content",
                "unstaged.rs": "unstaged content",
            }),
        )
        .await;

        let unresolved_status = FileStatus::Unmerged(UnmergedStatus {
            first_head: UnmergedStatusCode::Updated,
            second_head: UnmergedStatusCode::Updated,
        });
        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("conflict.rs", unresolved_status),
                ("staged.rs", FileStatus::index(StatusCode::Modified)),
                ("unstaged.rs", StatusCode::Modified.worktree()),
            ],
        );
        // With MERGE_HEAD present (an in-progress merge), a resolved conflict
        // keeps rendering under the Conflict section instead of moving to Staged.
        fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
            state.refs.insert("MERGE_HEAD".into(), "merge-sha".into());
        })
        .unwrap();

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Staging);
                })
            });
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

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        fn stage_status_of(
            panel: &Entity<GitPanel>,
            cx: &VisualTestContext,
            path: &str,
        ) -> StageStatus {
            panel.read_with(cx, |panel, cx| {
                let repo = panel
                    .active_repository
                    .as_ref()
                    .expect("active repository should exist")
                    .read(cx);
                let entry = panel
                    .change_entries_by_path()
                    .find(|entry| entry.repo_path == repo_path(path))
                    .expect("entry should exist")
                    .clone();
                GitPanel::stage_status_for_entry(&entry, repo)
            })
        }

        let conflict_entry = panel.read_with(&cx, |panel, _| {
            pretty_assertions::assert_matches!(
                panel.entries.as_slice(),
                &[
                    Header(GitHeaderEntry {
                        header: Section::Conflict
                    }),
                    Status(GitStatusEntry {
                        status: FileStatus::Unmerged(..),
                        ..
                    }),
                    Header(GitHeaderEntry {
                        header: Section::Staged
                    }),
                    Status(GitStatusEntry {
                        staging: StageStatus::Staged,
                        ..
                    }),
                    Header(GitHeaderEntry {
                        header: Section::Unstaged
                    }),
                    Status(GitStatusEntry {
                        staging: StageStatus::Unstaged,
                        ..
                    }),
                ],
            );
            panel
                .entries
                .get(1)
                .and_then(GitListEntry::status_entry)
                .cloned()
                .expect("conflict entry should exist")
        });

        // Resolve the conflict: simulate what `git add` does to the status.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(
                &GitListEntry::Status(conflict_entry.clone()),
                StageIntent::Toggle,
                window,
                cx,
            );
        });
        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("conflict.rs", FileStatus::index(StatusCode::Modified)),
                ("staged.rs", FileStatus::index(StatusCode::Modified)),
                ("unstaged.rs", StatusCode::Modified.worktree()),
            ],
        );
        cx.run_until_parked();
        await_git_panel_entries(&panel, &mut cx).await;

        // The resolved conflict stays in the Conflict section, staged.
        panel.read_with(&cx, |panel, cx| {
            assert_eq!(
                panel.section_for_entry_index(
                    panel
                        .entry_by_path(&repo_path("conflict.rs"))
                        .expect("conflict entry should exist")
                ),
                Some(Section::Conflict)
            );
            assert!(
                panel.is_resolved_conflict(
                    panel.entry_by_path(&repo_path("conflict.rs")).unwrap(),
                    cx
                )
            );
        });
        assert_eq!(
            stage_status_of(&panel, &cx, "conflict.rs"),
            StageStatus::Staged
        );

        // The keyboard toggle must not unstage a resolved conflict.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = panel.entry_by_path(&repo_path("conflict.rs"));
            panel.toggle_staged_for_selected(&ToggleStaged, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            stage_status_of(&panel, &cx, "conflict.rs"),
            StageStatus::Staged
        );

        // "Unstage All" on the Staged header must skip resolved conflicts while
        // still unstaging regular staged files.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_staged_for_entry(
                &GitListEntry::Header(GitHeaderEntry {
                    header: Section::Staged,
                }),
                StageIntent::Unstage,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            stage_status_of(&panel, &cx, "staged.rs"),
            StageStatus::Unstaged
        );
        assert_eq!(
            stage_status_of(&panel, &cx, "conflict.rs"),
            StageStatus::Staged
        );

        // A shift-click range sweep anchored at the conflict must skip it too.
        let staged_entry = panel.read_with(&cx, |panel, _| {
            panel
                .change_entries_by_path()
                .find(|entry| entry.repo_path == repo_path("staged.rs"))
                .cloned()
                .expect("staged entry should exist")
        });
        panel.update_in(&mut cx, |panel, _window, cx| {
            panel.change_file_stage(true, vec![staged_entry], cx);
        });
        cx.run_until_parked();
        await_git_panel_entries(&panel, &mut cx).await;

        panel.update_in(&mut cx, |panel, _window, cx| {
            panel.set_bulk_staging_anchor(repo_path("conflict.rs"), cx);
            let last_index = panel.entries.len() - 1;
            panel.stage_bulk(last_index, false, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            stage_status_of(&panel, &cx, "staged.rs"),
            StageStatus::Unstaged
        );
        assert_eq!(
            stage_status_of(&panel, &cx, "conflict.rs"),
            StageStatus::Staged
        );
    }

    #[gpui::test]
    async fn test_group_by_staging_primary_action_stages_partially_staged_files(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "partial.rs": "partial content",
                "staged.rs": "staged content",
            }),
        )
        .await;

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                (
                    "partial.rs",
                    TrackedStatus {
                        index_status: StatusCode::Modified,
                        worktree_status: StatusCode::Modified,
                    }
                    .into(),
                ),
                ("staged.rs", FileStatus::index(StatusCode::Modified)),
            ],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Staging);
                })
            });
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

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        panel.read_with(&mut cx, |panel, _| {
            assert_eq!(panel.entry_count, 2);
            assert_eq!(panel.total_staged_count(), panel.entry_count);
            assert!(panel.has_unstaged_changes());
            assert!(panel.primary_changes_action_stages());
        });
    }

    #[gpui::test]
    async fn test_group_by_staging_open_diff_uses_section_diff(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "partial.rs": "partial content",
            }),
        )
        .await;

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[(
                "partial.rs",
                TrackedStatus {
                    index_status: StatusCode::Modified,
                    worktree_status: StatusCode::Modified,
                }
                .into(),
            )],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Staging);
                })
            });
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

        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry =
                panel.entry_by_path_in_section(&repo_path("partial.rs"), Section::Staged);
            panel.open_diff(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();

        workspace.read_with(&cx, |workspace, cx| {
            assert!(workspace.active_item_as::<StagedDiff>(cx).is_some());
            assert_eq!(workspace.items_of_type::<StagedDiff>(cx).count(), 1);
            assert_eq!(workspace.items_of_type::<UnstagedDiff>(cx).count(), 0);
            assert_eq!(workspace.items_of_type::<ProjectDiff>(cx).count(), 0);
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.open_solo_diff(&menu::SecondaryConfirm, window, cx);
        });
        cx.run_until_parked();

        workspace.read_with(&cx, |workspace, cx| {
            assert!(workspace.active_item_as::<SoloDiffView>(cx).is_some());
            assert_eq!(workspace.items_of_type::<StagedDiff>(cx).count(), 1);
            assert_eq!(workspace.items_of_type::<SoloDiffView>(cx).count(), 1);
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry =
                panel.entry_by_path_in_section(&repo_path("partial.rs"), Section::Unstaged);
            panel.open_diff(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();

        workspace.read_with(&cx, |workspace, cx| {
            assert!(workspace.active_item_as::<UnstagedDiff>(cx).is_some());
            assert_eq!(workspace.items_of_type::<StagedDiff>(cx).count(), 1);
            assert_eq!(workspace.items_of_type::<UnstagedDiff>(cx).count(), 1);
            assert_eq!(workspace.items_of_type::<ProjectDiff>(cx).count(), 0);
        });
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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

        let panel = workspace.update_in(cx, GitPanel::new);

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
            panel.toggle_staged_for_entry(&second_status_entry, StageIntent::Toggle, window, cx);
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
            panel.toggle_staged_for_entry(&third_status_entry, StageIntent::Toggle, window, cx);
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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

        let panel = workspace.update_in(cx, GitPanel::new);

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
            panel.toggle_staged_for_entry(&second_status_entry, StageIntent::Toggle, window, cx);
        });

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::None);
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
            panel.toggle_staged_for_entry(&third_status_entry, StageIntent::Toggle, window, cx);
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        let panel = workspace.update_in(cx, GitPanel::new);

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
    async fn test_commit_message_restored_after_reconnect(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project-a": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                },
                "project-b": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project-a/.git")),
            &[("src/main.rs", StatusCode::Modified.worktree())],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/project-b/.git")),
            &[("src/main.rs", StatusCode::Modified.worktree())],
        );

        let project = Project::test(
            fs.clone(),
            [
                Path::new(path!("/root/project-a")),
                Path::new(path!("/root/project-b")),
            ],
            cx,
        )
        .await;
        let (repository_a, repository_b) = project.read_with(cx, |project, cx| {
            let git_store = project.git_store().clone();
            let mut repository_a = None;
            let mut repository_b = None;
            for repository in git_store.read(cx).repositories().values() {
                let work_directory_abs_path = &repository.read(cx).work_directory_abs_path;
                if work_directory_abs_path.as_ref() == Path::new(path!("/root/project-a")) {
                    repository_a = Some(repository.clone());
                } else if work_directory_abs_path.as_ref() == Path::new(path!("/root/project-b")) {
                    repository_b = Some(repository.clone());
                }
            }
            (
                repository_a.expect("should have repository for project-a"),
                repository_b.expect("should have repository for project-b"),
            )
        });
        repository_a.update(cx, |repository, cx| repository.set_as_active_repository(cx));

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        register_git_commit_language(&project, cx);
        let panel = workspace.update_in(cx, GitPanel::new);
        cx.run_until_parked();

        let message_a = "Restore repository A message";
        panel.update(cx, |panel, cx| {
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, message_a)], None, cx);
            });
        });

        repository_b.update(cx, |repository, cx| repository.set_as_active_repository(cx));
        cx.run_until_parked();

        let message_b = "Restore repository B message";
        let serialized_panel = panel.update(cx, |panel, cx| {
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, message_b)], None, cx);
            });

            SerializedGitPanel {
                signoff_enabled: false,
                commit_messages: panel.serialized_commit_messages(cx),
            }
        });

        for repository in [&repository_a, &repository_b] {
            let buffer = repository.read_with(cx, |repository, _| {
                repository
                    .commit_message_buffer()
                    .expect("repository commit message buffer should be open")
                    .clone()
            });
            buffer.update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, "")], None, cx);
            });
        }

        let restored_panel = workspace.update_in(cx, |workspace, window, cx| {
            GitPanel::new_with_serialized_panel(workspace, Some(serialized_panel), window, cx)
        });
        cx.run_until_parked();

        restored_panel.read_with(cx, |panel, cx| {
            assert_eq!(panel.commit_message_buffer(cx).read(cx).text(), message_b);
        });

        repository_a.update(cx, |repository, cx| repository.set_as_active_repository(cx));
        cx.run_until_parked();

        restored_panel.read_with(cx, |panel, cx| {
            assert_eq!(panel.commit_message_buffer(cx).read(cx).text(), message_a);
        });

        restored_panel.update(cx, |panel, cx| {
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, "")], None, cx);
            });
        });

        let mismatched_serialized_panel = SerializedGitPanel {
            signoff_enabled: false,
            commit_messages: BTreeMap::from_iter([(
                path!("/root/other-project").to_string(),
                SerializedCommitMessage {
                    message: Some(message_a.to_string()),
                    original_message: None,
                    ..Default::default()
                },
            )]),
        };
        let mismatched_panel = workspace.update_in(cx, |workspace, window, cx| {
            GitPanel::new_with_serialized_panel(
                workspace,
                Some(mismatched_serialized_panel),
                window,
                cx,
            )
        });
        cx.run_until_parked();

        mismatched_panel.read_with(cx, |panel, cx| {
            // The draft is not restored because the serialized work directory
            // does not match the active repository, so it cannot leak across
            // repositories.
            assert_eq!(panel.commit_message_buffer(cx).read(cx).text(), "");
        });
    }

    #[gpui::test]
    async fn test_show_all_repositories_preserves_repository_identity_when_staging(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project-a": {
                    ".git": {},
                    "src": { "main.rs": "fn main() {}" }
                },
                "project-b": {
                    ".git": {},
                    "src": { "main.rs": "fn main() {}" },
                    "scratch.txt": "new"
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project-a/.git")),
            &[("src/main.rs", StatusCode::Modified.worktree())],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/project-b/.git")),
            &[
                ("src/main.rs", StatusCode::Modified.worktree()),
                ("scratch.txt", FileStatus::Untracked),
            ],
        );

        let project = Project::test(
            fs.clone(),
            [
                Path::new(path!("/root/project-a")),
                Path::new(path!("/root/project-b")),
            ],
            cx,
        )
        .await;
        let (repository_a, repository_b) = project.read_with(cx, |project, cx| {
            let git_store = project.git_store().read(cx);
            let repository_for = |path: &Path| {
                git_store
                    .repositories()
                    .values()
                    .find(|repository| repository.read(cx).work_directory_abs_path.as_ref() == path)
                    .cloned()
                    .unwrap()
            };
            (
                repository_for(Path::new(path!("/root/project-a"))),
                repository_for(Path::new(path!("/root/project-b"))),
            )
        });
        let repository_a_id = repository_a.read_with(cx, |repository, _| repository.id);
        let repository_b_id = repository_b.read_with(cx, |repository, _| repository.id);
        repository_a.update(cx, |repository, cx| repository.set_as_active_repository(cx));

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    let git_panel = settings.git_panel.get_or_insert_default();
                    git_panel.show_all_repositories = Some(true);
                    git_panel.group_by = Some(GitPanelGroupBy::Status);
                    git_panel.sort_by = Some(GitPanelSortBy::Path);
                    git_panel.tree_view = Some(false);
                })
            });
        });

        cx.run_until_parked();
        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let path = repo_path("src/main.rs");
        let untracked_path = repo_path("scratch.txt");
        let (
            repository_b_entry_index,
            repository_b_header_index,
            repository_b_tracked_header_index,
            repository_b_untracked_header_index,
        ) = panel.read_with(&cx, |panel, _| {
            assert_eq!(
                panel
                    .entries
                    .iter()
                    .filter(|entry| matches!(entry, GitListEntry::RepositoryHeader(_)))
                    .count(),
                2
            );

            let row_repository_ids = panel
                .entries
                .iter()
                .enumerate()
                .filter_map(|(ix, entry)| {
                    entry
                        .status_entry()
                        .filter(|entry| entry.repo_path == path)
                        .map(|_| panel.entry_repository_ids[ix])
                })
                .collect::<Vec<_>>();
            assert_eq!(row_repository_ids.len(), 2);
            assert!(row_repository_ids.contains(&repository_a_id));
            assert!(row_repository_ids.contains(&repository_b_id));

            let key_a = ChangeKey {
                repository_id: repository_a_id,
                repo_path: path.clone(),
            };
            let key_b = ChangeKey {
                repository_id: repository_b_id,
                repo_path: path.clone(),
            };
            assert!(panel.projected_entries_by_path.contains_key(&key_a));
            assert!(panel.projected_entries_by_path.contains_key(&key_b));

            let active_index = panel.entry_by_path(&path).unwrap();
            assert_eq!(panel.entry_repository_ids[active_index], repository_a_id);
            let repository_b_entry_index = panel.entry_by_change_key(&key_b).unwrap();
            let repository_b_header_index = panel
                .entries
                .iter()
                .position(|entry| {
                    matches!(
                        entry,
                        GitListEntry::RepositoryHeader(header)
                            if header.repository_id == repository_b_id
                    )
                })
                .unwrap();
            let section_header_index = |section| {
                panel
                    .entries
                    .iter()
                    .enumerate()
                    .position(|(index, entry)| {
                        panel.repository_id_for_entry_index(index) == Some(repository_b_id)
                            && matches!(
                                entry,
                                GitListEntry::Header(header) if header.header == section
                            )
                    })
                    .unwrap()
            };

            (
                repository_b_entry_index,
                repository_b_header_index,
                section_header_index(Section::Tracked),
                section_header_index(Section::New),
            )
        });

        // Repository and section rows participate in the same arrow-key model
        // as files and preserve selection while their children are collapsed.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(repository_b_header_index);
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(panel.selected_entry, Some(repository_b_header_index));
            assert!(panel.collapsed_repositories.contains(&repository_b_id));
            assert!(!panel.is_entry_visible(repository_b_entry_index));
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
            panel.select_next(&menu::SelectNext, window, cx);
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(
                panel.selected_entry,
                Some(repository_b_tracked_header_index)
            );
            assert!(!panel.collapsed_repositories.contains(&repository_b_id));
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(
                panel.selected_entry,
                Some(repository_b_tracked_header_index)
            );
            assert!(
                panel
                    .collapsed_sections
                    .contains(&(repository_b_id, Section::Tracked))
            );
            assert!(!panel.is_entry_visible(repository_b_entry_index));
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });

        // Space uses the existing ToggleStaged action for both section headers,
        // routed to the repository represented by the selected row.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(repository_b_tracked_header_index);
            panel.toggle_staged_for_selected(&git::ToggleStaged, window, cx);
        });
        cx.run_until_parked();
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(repository_b_untracked_header_index);
            panel.toggle_staged_for_selected(&git::ToggleStaged, window, cx);
        });
        cx.run_until_parked();

        let a_was_staged = fs
            .with_git_state(Path::new(path!("/root/project-a/.git")), false, |state| {
                state.index_contents.get(&path) != state.head_contents.get(&path)
            })
            .unwrap();
        let b_was_staged = fs
            .with_git_state(Path::new(path!("/root/project-b/.git")), false, |state| {
                state.index_contents.get(&path) != state.head_contents.get(&path)
            })
            .unwrap();
        let b_untracked_was_staged = fs
            .with_git_state(Path::new(path!("/root/project-b/.git")), false, |state| {
                state.index_contents.contains_key(&untracked_path)
            })
            .unwrap();
        assert!(!a_was_staged);
        assert!(b_was_staged);
        assert!(b_untracked_was_staged);

        for section in [Section::Tracked, Section::New] {
            let section_header_index = panel.read_with(&cx, |panel, _| {
                panel
                    .entries
                    .iter()
                    .enumerate()
                    .position(|(index, entry)| {
                        panel.repository_id_for_entry_index(index) == Some(repository_b_id)
                            && matches!(
                                entry,
                                GitListEntry::Header(header) if header.header == section
                            )
                    })
                    .unwrap()
            });
            panel.update_in(&mut cx, |panel, window, cx| {
                panel.selected_entry = Some(section_header_index);
                panel.toggle_staged_for_selected(&git::ToggleStaged, window, cx);
            });
            cx.run_until_parked();
        }
        fs.with_git_state(Path::new(path!("/root/project-b/.git")), false, |state| {
            assert_eq!(
                state.index_contents.get(&path),
                state.head_contents.get(&path)
            );
            assert!(!state.index_contents.contains_key(&untracked_path));
        })
        .unwrap();

        let repository_b_entry_index = panel.read_with(&cx, |panel, _| {
            panel
                .entry_by_change_key(&ChangeKey {
                    repository_id: repository_b_id,
                    repo_path: path.clone(),
                })
                .unwrap()
        });
        panel.update_in(&mut cx, |panel, _, _| {
            panel.selected_entry = Some(repository_b_entry_index);
        });
        let history_repository_id = panel.read_with(&cx, |panel, cx| {
            panel
                .selected_file_history_target(cx)
                .map(|(repository, _)| repository.read(cx).id)
        });
        assert_eq!(history_repository_id, Some(repository_b_id));

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.open_diff(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();
        workspace.read_with(&cx, |workspace, cx| {
            let project_diff = workspace
                .active_item_as::<ProjectDiff>(cx)
                .expect("project diff should open for repository B");
            assert_eq!(
                project_diff
                    .read(cx)
                    .repo(cx)
                    .map(|repository| repository.read(cx).id),
                Some(repository_b_id)
            );
        });

        let active_repository_id = project.read_with(&cx, |project, cx| {
            project
                .active_repository(cx)
                .map(|repository| repository.read(cx).id)
        });
        assert_eq!(active_repository_id, Some(repository_a_id));
        panel.read_with(&cx, |panel, _| {
            assert_eq!(panel.active_repository_id, Some(repository_a_id));
        });

        // Space and Enter on a repository row are keyboard equivalents to
        // clicking the repository name: they change the active repository
        // without reordering the all-repositories list.
        let repository_b_header_index = panel.read_with(&cx, |panel, _| {
            panel
                .entries
                .iter()
                .position(|entry| {
                    matches!(
                        entry,
                        GitListEntry::RepositoryHeader(header)
                            if header.repository_id == repository_b_id
                    )
                })
                .unwrap()
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(repository_b_header_index);
            panel.toggle_staged_for_selected(&git::ToggleStaged, window, cx);
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        assert_eq!(
            project.read_with(&cx, |project, cx| {
                project
                    .active_repository(cx)
                    .map(|repository| repository.read(cx).id)
            }),
            Some(repository_b_id)
        );
        panel.read_with(&cx, |panel, _| {
            assert_eq!(panel.active_repository_id, Some(repository_b_id));
            assert!(matches!(
                panel
                    .selected_entry
                    .and_then(|index| panel.entries.get(index)),
                Some(GitListEntry::RepositoryHeader(header))
                    if header.repository_id == repository_b_id
            ));
        });

        repository_a.update(&mut cx, |repository, cx| {
            repository.set_as_active_repository(cx)
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        let repository_b_header_index = panel.read_with(&cx, |panel, _| {
            panel
                .entries
                .iter()
                .position(|entry| {
                    matches!(
                        entry,
                        GitListEntry::RepositoryHeader(header)
                            if header.repository_id == repository_b_id
                    )
                })
                .unwrap()
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = Some(repository_b_header_index);
            panel.open_diff(&menu::Confirm, window, cx);
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        assert_eq!(
            project.read_with(&cx, |project, cx| {
                project
                    .active_repository(cx)
                    .map(|repository| repository.read(cx).id)
            }),
            Some(repository_b_id)
        );
        panel.read_with(&cx, |panel, _| {
            assert_eq!(panel.active_repository_id, Some(repository_b_id));
            assert!(matches!(
                panel
                    .selected_entry
                    .and_then(|index| panel.entries.get(index)),
                Some(GitListEntry::RepositoryHeader(header))
                    if header.repository_id == repository_b_id
            ));
        });
    }

    #[gpui::test]
    async fn test_non_active_repository_branch_switch_preserves_active_repository_and_header(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project-a": { ".git": {}, "a.txt": "a" },
                "project-b": { ".git": {}, "b.txt": "b" }
            }),
        )
        .await;
        for dot_git in [path!("/root/project-a/.git"), path!("/root/project-b/.git")] {
            fs.insert_branches(Path::new(dot_git), &["main", "feature"]);
            fs.set_status_for_repo(Path::new(dot_git), &[]);
        }

        let project = Project::test(
            fs.clone(),
            [
                Path::new(path!("/root/project-a")),
                Path::new(path!("/root/project-b")),
            ],
            cx,
        )
        .await;
        let (repository_a, repository_b) = project.read_with(cx, |project, cx| {
            let repositories = project.git_store().read(cx).repositories();
            let repository_for = |path: &Path| {
                repositories
                    .values()
                    .find(|repository| repository.read(cx).work_directory_abs_path.as_ref() == path)
                    .cloned()
                    .unwrap()
            };
            (
                repository_for(Path::new(path!("/root/project-a"))),
                repository_for(Path::new(path!("/root/project-b"))),
            )
        });
        let repository_a_id = repository_a.read_with(cx, |repository, _| repository.id);
        let repository_b_id = repository_b.read_with(cx, |repository, _| repository.id);
        repository_a.update(cx, |repository, cx| repository.set_as_active_repository(cx));

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);
        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .git_panel
                        .get_or_insert_default()
                        .show_all_repositories = Some(true);
                })
            });
        });
        cx.run_until_parked();
        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let change_branch = panel.update_in(&mut cx, |panel, _window, cx| {
            panel
                .repository_for_id(repository_b_id, cx)
                .unwrap()
                .update(cx, |repository, _| {
                    repository.change_branch("feature".to_owned())
                })
        });
        change_branch.await.unwrap().unwrap();
        assert_eq!(
            fs.with_git_state(Path::new(path!("/root/project-b/.git")), false, |state| {
                state.current_branch_name.clone()
            })
            .unwrap()
            .as_deref(),
            Some("feature")
        );
        // FakeGitRepository has no real HEAD file for the watcher to observe,
        // so mirror the HeadChanged event emitted by a real checkout.
        let feature_branch = repository_b.read_with(&cx, |repository, _| {
            repository
                .branch_list
                .iter()
                .find(|branch| branch.name() == "feature")
                .cloned()
                .unwrap()
        });
        panel.update_in(&mut cx, |_panel, _window, cx| {
            repository_b.update(cx, |repository, cx| {
                repository.set_branch_for_test(Some(feature_branch), cx)
            });
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();

        project.read_with(&cx, |project, cx| {
            assert_eq!(
                project
                    .active_repository(cx)
                    .map(|repository| repository.read(cx).id),
                Some(repository_a_id)
            );
        });
        panel.read_with(&cx, |panel, _| {
            let repository_b_header = panel
                .entries
                .iter()
                .find_map(|entry| match entry {
                    GitListEntry::RepositoryHeader(header)
                        if header.repository_id == repository_b_id =>
                    {
                        Some(header)
                    }
                    _ => None,
                })
                .unwrap();
            assert_eq!(repository_b_header.branch_label.as_ref(), "feature");
            assert_eq!(panel.active_repository_id, Some(repository_a_id));
        });
    }

    #[gpui::test]
    async fn test_all_repository_headers_group_primary_submodule_and_nested_repository(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root/super",
            json!({
                ".git": {
                    "modules": {
                        "modules": {
                            "child": {
                                "HEAD": "",
                                "config": ""
                            },
                            "second": {
                                "HEAD": "",
                                "config": ""
                            }
                        }
                    }
                },
                "README.md": "primary",
                "scratch.txt": "untracked in primary",
                "modules": {
                    "child": {
                        ".git": "gitdir: ../../.git/modules/modules/child\n",
                        "src": {
                            "nested": {
                                "child.txt": "child"
                            }
                        }
                    },
                    "second": {
                        ".git": "gitdir: ../../.git/modules/modules/second\n",
                        "second.txt": "second"
                    }
                },
                "tools": {
                    ".git": {},
                    "tool.txt": "tool"
                }
            }),
        )
        .await;
        fs.set_status_for_repo(
            Path::new(path!("/root/super/.git")),
            &[
                ("README.md", StatusCode::Modified.worktree()),
                ("scratch.txt", FileStatus::Untracked),
            ],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/super/modules/child/.git")),
            &[("src/nested/child.txt", StatusCode::Modified.worktree())],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/super/modules/second/.git")),
            &[("second.txt", FileStatus::Untracked)],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/super/tools/.git")),
            &[("tool.txt", StatusCode::Modified.worktree())],
        );

        let project = Project::test(fs, [Path::new(path!("/root/super"))], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let (primary_repository, submodule_repository, second_submodule, nested_repository) =
            project.read_with(cx, |project, cx| {
                let repositories = project.git_store().read(cx).repositories();
                let repository_for = |path: &Path| {
                    repositories
                        .values()
                        .find(|repository| {
                            repository.read(cx).work_directory_abs_path.as_ref() == path
                        })
                        .cloned()
                        .unwrap_or_else(|| panic!("missing repository at {}", path.display()))
                };
                (
                    repository_for(Path::new(path!("/root/super"))),
                    repository_for(Path::new(path!("/root/super/modules/child"))),
                    repository_for(Path::new(path!("/root/super/modules/second"))),
                    repository_for(Path::new(path!("/root/super/tools"))),
                )
            });
        let primary_repository_id = primary_repository.read_with(cx, |repository, _| repository.id);
        let submodule_repository_id =
            submodule_repository.read_with(cx, |repository, _| repository.id);
        let second_submodule_id = second_submodule.read_with(cx, |repository, _| repository.id);
        let nested_repository_id = nested_repository.read_with(cx, |repository, _| repository.id);

        // Active repository is intentionally not the project root. Primary order
        // follows the visible worktree and must remain stable as users work in a
        // nested repository.
        nested_repository.update(cx, |repository, cx| repository.set_as_active_repository(cx));

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    let git_panel = settings.git_panel.get_or_insert_default();
                    git_panel.show_all_repositories = Some(true);
                    git_panel.group_by = Some(GitPanelGroupBy::Status);
                    git_panel.tree_view = Some(false);
                })
            });
        });

        cx.run_until_parked();
        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let (expanded_visible_indices, changes_count) = panel.read_with(&cx, |panel, _| {
            let headers = panel
                .entries
                .iter()
                .filter_map(|entry| match entry {
                    GitListEntry::RepositoryHeader(header) => Some(header),
                    _ => None,
                })
                .collect::<Vec<_>>();

            assert_eq!(headers.len(), 4);
            assert_eq!(
                headers
                    .iter()
                    .map(|header| header.repository_id)
                    .collect::<Vec<_>>(),
                [
                    primary_repository_id,
                    submodule_repository_id,
                    second_submodule_id,
                    nested_repository_id
                ]
            );
            assert_eq!(headers[0].kind, GitRepositoryKind::Primary);
            assert_eq!(headers[1].kind, GitRepositoryKind::Submodule);
            assert_eq!(headers[2].kind, GitRepositoryKind::Submodule);
            assert_eq!(headers[3].kind, GitRepositoryKind::Repository);
            assert_eq!(headers[1].parent_display_name.as_deref(), Some("super"));
            assert_eq!(headers[2].parent_display_name.as_deref(), Some("super"));
            assert!(headers[3].is_active);
            assert!(!headers[0].is_active);

            let primary_untracked_header_ix = panel
                .entries
                .iter()
                .enumerate()
                .position(|(ix, entry)| {
                    panel.entry_repository_ids[ix] == primary_repository_id
                        && matches!(
                            entry,
                            GitListEntry::Header(GitHeaderEntry {
                                header: Section::New
                            })
                        )
                })
                .expect("primary Untracked header should exist");
            let primary_untracked_entry_ix = panel
                .entries
                .iter()
                .enumerate()
                .position(|(ix, entry)| {
                    panel.entry_repository_ids[ix] == primary_repository_id
                        && entry
                            .status_entry()
                            .is_some_and(|entry| entry.repo_path == repo_path("scratch.txt"))
                })
                .expect("primary untracked entry should exist");
            let project_repositories_header_ix = panel
                .entries
                .iter()
                .position(|entry| matches!(entry, GitListEntry::ProjectRepositoriesHeader(_)))
                .expect("project repositories header should exist");
            let first_submodule_header_ix = panel
                .entries
                .iter()
                .position(|entry| {
                    matches!(
                        entry,
                        GitListEntry::RepositoryHeader(header)
                            if header.repository_id == submodule_repository_id
                    )
                })
                .unwrap();
            let second_submodule_header_ix = panel
                .entries
                .iter()
                .position(|entry| {
                    matches!(
                        entry,
                        GitListEntry::RepositoryHeader(header)
                            if header.repository_id == second_submodule_id
                    )
                })
                .unwrap();
            let nested_repository_header_ix = panel
                .entries
                .iter()
                .position(|entry| {
                    matches!(
                        entry,
                        GitListEntry::RepositoryHeader(header)
                            if header.repository_id == nested_repository_id
                    )
                })
                .unwrap();

            assert!(primary_untracked_header_ix < primary_untracked_entry_ix);
            assert!(primary_untracked_entry_ix < project_repositories_header_ix);
            assert!(project_repositories_header_ix < first_submodule_header_ix);
            assert!(first_submodule_header_ix < second_submodule_header_ix);
            assert!(second_submodule_header_ix < nested_repository_header_ix);
            assert!(
                panel.repository_entry_ranges[&primary_repository_id]
                    .contains(&primary_untracked_entry_ix)
            );
            assert!(
                !panel.repository_entry_ranges[&primary_repository_id]
                    .contains(&project_repositories_header_ix)
            );
            assert_eq!(panel.project_repository_depth(submodule_repository_id), 1);
            assert_eq!(panel.project_repository_depth(second_submodule_id), 1);
            assert_eq!(panel.project_repository_depth(nested_repository_id), 0);

            let group = match &panel.entries[project_repositories_header_ix] {
                GitListEntry::ProjectRepositoriesHeader(group) => group,
                _ => unreachable!(),
            };
            assert_eq!(group.repository_count, 2);
            assert_eq!(group.change_count, 2);
            assert!(group.expanded);
            assert!(!group.contains_active_repository);
            assert_eq!(panel.changes_count, 5);

            (panel.visible_entry_indices.clone(), panel.changes_count)
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_section(primary_repository_id, Section::New, window, cx)
        });
        panel.read_with(&cx, |panel, _| {
            let untracked_header_ix = panel
                .entries
                .iter()
                .enumerate()
                .position(|(ix, entry)| {
                    panel.entry_repository_ids[ix] == primary_repository_id
                        && matches!(
                            entry,
                            GitListEntry::Header(GitHeaderEntry {
                                header: Section::New
                            })
                        )
                })
                .unwrap();
            let untracked_entry_ix = panel
                .entry_by_change_key(&ChangeKey {
                    repository_id: primary_repository_id,
                    repo_path: repo_path("scratch.txt"),
                })
                .unwrap();
            assert!(panel.is_entry_visible(untracked_header_ix));
            assert!(!panel.is_entry_visible(untracked_entry_ix));
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_section(primary_repository_id, Section::New, window, cx)
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_repository(submodule_repository_id, window, cx)
        });
        panel.read_with(&cx, |panel, _| {
            let submodule_header_ix = panel
                .entries
                .iter()
                .position(|entry| {
                    matches!(
                        entry,
                        GitListEntry::RepositoryHeader(header)
                            if header.repository_id == submodule_repository_id
                    )
                })
                .unwrap();
            let submodule_entry_ix = panel
                .entry_by_change_key(&ChangeKey {
                    repository_id: submodule_repository_id,
                    repo_path: repo_path("src/nested/child.txt"),
                })
                .unwrap();
            assert!(panel.is_entry_visible(submodule_header_ix));
            assert!(!panel.is_entry_visible(submodule_entry_ix));
            assert!(matches!(
                panel.entries[submodule_header_ix],
                GitListEntry::RepositoryHeader(GitRepositoryHeaderEntry {
                    expanded: false,
                    ..
                })
            ));
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_repository(submodule_repository_id, window, cx)
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(panel.visible_entry_indices, expanded_visible_indices);
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.selected_entry = panel
                .entries
                .iter()
                .position(|entry| matches!(entry, GitListEntry::ProjectRepositoriesHeader(_)));
            panel.toggle_staged_for_selected(&git::ToggleStaged, window, cx);
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(panel.changes_count, changes_count);
            let visible_repository_ids = panel
                .visible_entry_indices
                .iter()
                .filter_map(|&ix| match &panel.entries[ix] {
                    GitListEntry::RepositoryHeader(header) => Some(header.repository_id),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                visible_repository_ids,
                [primary_repository_id, nested_repository_id]
            );

            for repository_id in [submodule_repository_id, second_submodule_id] {
                let header_ix = panel
                    .entries
                    .iter()
                    .position(|entry| {
                        matches!(
                            entry,
                            GitListEntry::RepositoryHeader(header)
                                if header.repository_id == repository_id
                        )
                    })
                    .expect("collapsed repository should remain in the model");
                assert!(!panel.is_entry_visible(header_ix));
            }

            let primary_untracked_ix = panel
                .entries
                .iter()
                .enumerate()
                .position(|(ix, entry)| {
                    panel.entry_repository_ids[ix] == primary_repository_id
                        && entry
                            .status_entry()
                            .is_some_and(|entry| entry.repo_path == repo_path("scratch.txt"))
                })
                .unwrap();
            assert!(panel.is_entry_visible(primary_untracked_ix));
            assert!(panel.entries.iter().any(|entry| {
                matches!(
                    entry,
                    GitListEntry::ProjectRepositoriesHeader(group) if !group.expanded
                )
            }));
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_staged_for_selected(&git::ToggleStaged, window, cx);
        });
        panel.read_with(&cx, |panel, _| {
            assert_eq!(panel.visible_entry_indices, expanded_visible_indices);
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.view_mode = GitPanelViewMode::Tree(TreeViewState::default());
            panel.update_visible_entries(window, cx);
            panel.toggle_project_repositories(window, cx);
        });
        panel.read_with(&cx, |panel, _| {
            assert!(matches!(panel.view_mode, GitPanelViewMode::Tree(_)));
            assert!(!panel.project_repositories_expanded);
            let primary_untracked_ix = panel
                .entries
                .iter()
                .enumerate()
                .position(|(ix, entry)| {
                    panel.entry_repository_ids[ix] == primary_repository_id
                        && entry
                            .status_entry()
                            .is_some_and(|entry| entry.repo_path == repo_path("scratch.txt"))
                })
                .unwrap();
            assert!(panel.is_entry_visible(primary_untracked_ix));
            let submodule_directory_ix = panel
                .entries
                .iter()
                .enumerate()
                .position(|(ix, entry)| {
                    panel.entry_repository_ids[ix] == submodule_repository_id
                        && matches!(
                            entry,
                            GitListEntry::Directory(directory)
                                if directory.key.path == repo_path("src/nested")
                        )
                })
                .expect("submodule directories should be built in tree view");
            assert!(!panel.is_entry_visible(submodule_directory_ix));

            let repository_header_ix = |repository_id| {
                panel
                    .entries
                    .iter()
                    .position(|entry| {
                        matches!(
                            entry,
                            GitListEntry::RepositoryHeader(header)
                                if header.repository_id == repository_id
                        )
                    })
                    .unwrap()
            };
            let section_header_ix = |repository_id, section| {
                panel
                    .entries
                    .iter()
                    .enumerate()
                    .position(|(ix, entry)| {
                        panel.entry_repository_ids[ix] == repository_id
                            && matches!(
                                entry,
                                GitListEntry::Header(GitHeaderEntry { header })
                                    if *header == section
                            )
                    })
                    .unwrap()
            };
            let project_repositories_header_ix = panel
                .entries
                .iter()
                .position(|entry| matches!(entry, GitListEntry::ProjectRepositoriesHeader(_)))
                .unwrap();
            let submodule_child_ix = panel
                .entry_by_change_key(&ChangeKey {
                    repository_id: submodule_repository_id,
                    repo_path: repo_path("src/nested/child.txt"),
                })
                .unwrap();

            assert_eq!(
                panel.visual_depth_for_entry(repository_header_ix(primary_repository_id)),
                0
            );
            assert_eq!(
                panel
                    .visual_depth_for_entry(section_header_ix(primary_repository_id, Section::New)),
                1
            );
            assert_eq!(panel.visual_depth_for_entry(primary_untracked_ix), 2);
            assert_eq!(
                panel.visual_depth_for_entry(project_repositories_header_ix),
                0
            );
            assert_eq!(
                panel.visual_depth_for_entry(repository_header_ix(submodule_repository_id)),
                1
            );
            assert_eq!(
                panel.visual_depth_for_entry(section_header_ix(
                    submodule_repository_id,
                    Section::Tracked
                )),
                2
            );
            assert_eq!(panel.visual_depth_for_entry(submodule_directory_ix), 3);
            assert_eq!(panel.visual_depth_for_entry(submodule_child_ix), 4);
            assert_eq!(
                panel.visual_depth_for_entry(repository_header_ix(nested_repository_id)),
                0
            );
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_repository(submodule_repository_id, window, cx);
            panel.toggle_section(submodule_repository_id, Section::Tracked, window, cx);
        });

        let worktree_id = project.read_with(&cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.select_entry_by_path(
                ProjectPath {
                    worktree_id,
                    path: RelPath::from_unix_str("modules/child/src/nested/child.txt")
                        .unwrap()
                        .into_arc(),
                },
                window,
                cx,
            );
        });
        panel.read_with(&cx, |panel, _| {
            assert!(panel.project_repositories_expanded);
            assert!(
                !panel
                    .collapsed_repositories
                    .contains(&submodule_repository_id)
            );
            assert!(
                !panel
                    .collapsed_sections
                    .contains(&(submodule_repository_id, Section::Tracked))
            );
            let selected_ix = panel
                .selected_entry
                .expect("child entry should be selected");
            assert!(panel.is_entry_visible(selected_ix));
            assert_eq!(
                panel.repository_id_for_entry_index(selected_ix),
                Some(submodule_repository_id)
            );
            let path = repo_path("src/nested");
            let directory_ix = panel
                .entries
                .iter()
                .enumerate()
                .position(|(ix, entry)| {
                    panel.entry_repository_ids[ix] == submodule_repository_id
                        && matches!(
                            entry,
                            GitListEntry::Directory(directory) if directory.key.path == path
                        )
                })
                .expect("submodule directory should remain in the tree");
            assert!(panel.is_entry_visible(directory_ix));
        });
    }

    #[gpui::test]
    async fn test_all_repository_order_does_not_follow_active_repo_without_root_repository(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "a": { ".git": {}, "a.txt": "a" },
                "b": { ".git": {}, "b.txt": "b" }
            }),
        )
        .await;
        for dot_git in [path!("/root/a/.git"), path!("/root/b/.git")] {
            fs.set_status_for_repo(Path::new(dot_git), &[]);
        }

        let project = Project::test(fs, [Path::new(path!("/root"))], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let (repository_a, repository_b) = project.read_with(cx, |project, cx| {
            let repositories = project.git_store().read(cx).repositories();
            let repository_for = |path: &Path| {
                repositories
                    .values()
                    .find(|repository| repository.read(cx).work_directory_abs_path.as_ref() == path)
                    .cloned()
                    .unwrap()
            };
            (
                repository_for(Path::new(path!("/root/a"))),
                repository_for(Path::new(path!("/root/b"))),
            )
        });
        let repository_a_id = repository_a.read_with(cx, |repository, _| repository.id);
        let repository_b_id = repository_b.read_with(cx, |repository, _| repository.id);
        repository_b.update(cx, |repository, cx| repository.set_as_active_repository(cx));

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);
        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .git_panel
                        .get_or_insert_default()
                        .show_all_repositories = Some(true);
                })
            });
        });
        cx.run_until_parked();
        let panel = workspace.update_in(&mut cx, GitPanel::new);
        await_git_panel_entries(&panel, &mut cx).await;

        let assert_stable_headers = |panel: &GitPanel, active_repository_id| {
            let headers = panel
                .entries
                .iter()
                .filter_map(|entry| match entry {
                    GitListEntry::RepositoryHeader(header) => Some(header),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                headers
                    .iter()
                    .map(|header| header.repository_id)
                    .collect::<Vec<_>>(),
                [repository_a_id, repository_b_id]
            );
            assert!(
                headers
                    .iter()
                    .all(|header| header.kind == GitRepositoryKind::Repository)
            );
            assert!(
                headers
                    .iter()
                    .find(|header| header.repository_id == active_repository_id)
                    .unwrap()
                    .is_active
            );
            assert!(
                !panel
                    .entries
                    .iter()
                    .any(|entry| matches!(entry, GitListEntry::ProjectRepositoriesHeader(_)))
            );
        };
        repository_b.update(&mut cx, |repository, cx| {
            repository.set_as_active_repository(cx)
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        panel.read_with(&cx, |panel, _| {
            assert_stable_headers(panel, repository_b_id)
        });

        repository_a.update(&mut cx, |repository, cx| {
            repository.set_as_active_repository(cx)
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        panel.read_with(&cx, |panel, _| {
            assert_stable_headers(panel, repository_a_id)
        });
    }

    #[gpui::test]
    async fn test_amend_state_is_per_repository(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({
                "project-a": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                },
                "project-b": {
                    ".git": {},
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/project-a/.git")),
            &[("src/main.rs", StatusCode::Modified.worktree())],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/project-b/.git")),
            &[("src/main.rs", StatusCode::Modified.worktree())],
        );

        let project = Project::test(
            fs.clone(),
            [
                Path::new(path!("/root/project-a")),
                Path::new(path!("/root/project-b")),
            ],
            cx,
        )
        .await;
        let (repository_a, repository_b) = project.read_with(cx, |project, cx| {
            let git_store = project.git_store().clone();
            let mut repository_a = None;
            let mut repository_b = None;
            for repository in git_store.read(cx).repositories().values() {
                let work_directory_abs_path = &repository.read(cx).work_directory_abs_path;
                if work_directory_abs_path.as_ref() == Path::new(path!("/root/project-a")) {
                    repository_a = Some(repository.clone());
                } else if work_directory_abs_path.as_ref() == Path::new(path!("/root/project-b")) {
                    repository_b = Some(repository.clone());
                }
            }
            (
                repository_a.expect("should have repository for project-a"),
                repository_b.expect("should have repository for project-b"),
            )
        });
        repository_a.update(cx, |repository, cx| repository.set_as_active_repository(cx));

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        register_git_commit_language(&project, cx);
        let panel = workspace.update_in(cx, GitPanel::new);
        cx.run_until_parked();

        // Enter an amend on repository A, then simulate the amend flow loading
        // the last commit message into the editor.
        panel.update(cx, |panel, cx| {
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, "Draft for A")], None, cx);
            });
            panel.set_amend_pending(true, cx);
            panel.commit_message_buffer(cx).update(cx, |buffer, cx| {
                let start = buffer.anchor_before(0);
                let end = buffer.anchor_after(buffer.len());
                buffer.edit([(start..end, "Amended message")], None, cx);
            });
            assert!(panel.amend_pending());
        });

        // Switching the active repository away exits the amend state instead of
        // carrying it over to repository B.
        repository_b.update(cx, |repository, cx| repository.set_as_active_repository(cx));
        cx.run_until_parked();

        panel.update(cx, |panel, cx| {
            assert!(!panel.amend_pending());
            // Only the active repository may serialize a pending amend, and we
            // just left repository A's amend, so nothing is left pending.
            let serialized = panel.serialized_commit_messages(cx);
            assert!(serialized.values().all(|message| !message.amend_pending));
        });

        // Repository A's pre-amend draft is restored, discarding the amend edit.
        let buffer_a = repository_a.read_with(cx, |repository, _| {
            repository
                .commit_message_buffer()
                .expect("repository commit message buffer should be open")
                .clone()
        });
        buffer_a.read_with(cx, |buffer, _| {
            assert_eq!(buffer.text(), "Draft for A");
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

        // Wait for the project scanning to finish so that `head_commit(cx)` is
        // actually set, otherwise no head commit would be available from which
        // to fetch the latest commit message from.
        cx.executor().run_until_parked();

        let panel = workspace.update_in(cx, GitPanel::new);
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        let panel = workspace.update_in(cx, GitPanel::new);

        // Disable status grouping and wait for entries to be updated,
        // as there should no longer be separators between Tracked and Untracked
        // files.
        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::None);
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

        workspace.update_in(cx, |workspace, _window, cx| {
            let active_path = workspace
                .item_of_type::<ProjectDiff>(cx)
                .expect("ProjectDiff should exist")
                .read(cx)
                .active_project_path(cx)
                .expect("active_project_path should exist");

            assert_eq!(active_path.path, rel_path("untracked").into_arc());
        });
    }

    #[gpui::test]
    async fn test_remote_operation_serialization(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        let panel = workspace.update_in(cx, GitPanel::new);

        panel.update(cx, |panel, cx| {
            // The first remote operation starts and records its kind, which the
            // button uses to render an "in progress" tooltip.
            assert!(panel.start_remote_operation(RemoteOperationKind::Fetch, cx));
            assert!(matches!(
                panel.pending_remote_operation,
                Some(RemoteOperationKind::Fetch)
            ));

            // A second remote operation is refused while one is pending, even a
            // different kind: we serialize all remote ops.
            assert!(!panel.start_remote_operation(RemoteOperationKind::Push, cx));

            // Clearing the pending operation re-opens the gate.
            panel.clear_remote_operation(cx);
            assert!(panel.pending_remote_operation.is_none());
            assert!(panel.start_remote_operation(RemoteOperationKind::Pull, cx));
        });
    }

    #[gpui::test]
    async fn test_tree_view_without_status_grouping_combines_statuses(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "src": {
                    "main.rs": "fn main() {}",
                    "utils.rs": "pub fn util() {}",
                },
                "tests": {
                    "main_test.rs": "#[test] fn test_main() {}",
                },
            }),
        )
        .await;

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("src/main.rs", StatusCode::Modified.worktree()),
                ("src/utils.rs", FileStatus::Untracked),
                ("tests/main_test.rs", StatusCode::Modified.worktree()),
            ],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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
                    let git_panel = settings.git_panel.get_or_insert_default();
                    git_panel.tree_view = Some(true);
                    git_panel.group_by = Some(GitPanelGroupBy::None);
                })
            });
        });

        let panel = workspace.update_in(cx, GitPanel::new);
        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });

        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        panel.read_with(cx, |panel, _| {
            assert!(
                panel
                    .entries
                    .iter()
                    .all(|entry| !matches!(entry, GitListEntry::Header(_))),
                "status headers should not be shown when grouping is disabled",
            );

            let tree_state = panel
                .view_mode
                .tree_state()
                .expect("tree view state should exist");
            let src_key = panel
                .entries
                .iter()
                .find_map(|entry| match entry {
                    GitListEntry::Directory(dir) if dir.key.path == repo_path("src") => {
                        Some(&dir.key)
                    }
                    _ => None,
                })
                .expect("src directory should exist in tree view");
            let src_descendants = tree_state
                .directory_descendants
                .get(src_key)
                .expect("src descendants should be tracked");

            assert!(
                src_descendants
                    .iter()
                    .any(|entry| entry.repo_path == repo_path("src/main.rs"))
            );
            assert!(
                src_descendants
                    .iter()
                    .any(|entry| entry.repo_path == repo_path("src/utils.rs"))
            );
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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

        let panel = workspace.update_in(cx, GitPanel::new);

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
            path: RelPath::from_unix_str("src/a/foo.rs").unwrap().into_arc(),
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
            assert!(panel.visible_entry_indices.contains(&selected_ix));

            let selected_entry = panel
                .entries
                .get(selected_ix)
                .and_then(|entry| entry.status_entry())
                .expect("selected entry should be a status entry");
            assert_eq!(selected_entry.repo_path, repo_path("src/a/foo.rs"));
        });
    }

    #[gpui::test]
    async fn test_tree_view_select_next_reaches_section_header_after_collapsed_directory(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "bar": {
                    "bar1.py": "print('bar1')",
                    "bar2.py": "print('bar2')",
                },
                "foo": {
                    "foo1.py": "print('foo1')",
                    "foo2.py": "print('foo2')",
                },
                "foobar.py": "print('foobar')",
            }),
        )
        .await;

        fs.set_status_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("bar/bar1.py", StatusCode::Modified.worktree()),
                ("bar/bar2.py", StatusCode::Modified.worktree()),
                ("foo/foo1.py", StatusCode::Modified.worktree()),
                ("foo/foo2.py", StatusCode::Modified.worktree()),
                ("foobar.py", FileStatus::Untracked),
            ],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);

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

        let panel = workspace.update_in(cx, GitPanel::new);
        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });

        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        let foo_key = panel.read_with(cx, |panel, _| {
            panel
                .entries
                .iter()
                .find_map(|entry| match entry {
                    GitListEntry::Directory(dir) if dir.key.path == repo_path("foo") => {
                        Some(dir.key.clone())
                    }
                    _ => None,
                })
                .expect("foo directory should exist in tree view")
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_directory(&foo_key, window, cx);
        });

        let foo_idx = panel.read_with(cx, |panel, _| {
            let state = panel
                .view_mode
                .tree_state()
                .expect("tree view state should exist");
            assert_eq!(state.expanded_dirs.get(&foo_key).copied(), Some(false));

            let foo_idx = panel
                .entries
                .iter()
                .enumerate()
                .find_map(|(index, entry)| match entry {
                    GitListEntry::Directory(dir) if dir.key.path == repo_path("foo") => Some(index),
                    _ => None,
                })
                .expect("foo directory should exist in tree view");

            let foo_logical_idx = panel
                .visible_entry_indices
                .iter()
                .position(|&index| index == foo_idx)
                .expect("foo directory should be visible");
            let next_logical_idx = panel.visible_entry_indices[foo_logical_idx + 1];
            assert!(matches!(
                panel.entries.get(next_logical_idx),
                Some(GitListEntry::Header(GitHeaderEntry {
                    header: Section::New
                }))
            ));

            foo_idx
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.selected_entry = Some(foo_idx);
            panel.select_next(&menu::SelectNext, window, cx);
        });

        panel.read_with(cx, |panel, _| {
            let selected_idx = panel.selected_entry.expect("selection should be set");
            assert!(matches!(
                panel.entries.get(selected_idx),
                Some(GitListEntry::Header(GitHeaderEntry {
                    header: Section::New
                }))
            ));
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&menu::SelectNext, window, cx);
        });
        panel.read_with(cx, |panel, _| {
            let selected_idx = panel.selected_entry.expect("selection should be set");
            let selected_entry = panel
                .entries
                .get(selected_idx)
                .and_then(|entry| entry.status_entry())
                .expect("the file after the section header should be selected");
            assert_eq!(selected_entry.repo_path, repo_path("foobar.py"));
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

    #[test]
    fn test_commit_message_prompt_includes_user_agents_md_before_project_rules() {
        let prompt = GitPanel::build_commit_message_prompt(
            "Write a commit message.",
            Some("Use terse commit messages."),
            Some("Use the git_ui prefix."),
            Some("Follow the configured commit message format."),
            "Update generated message",
            "diff --git a/file b/file",
        );

        assert!(prompt.contains("Use terse commit messages."));
        assert!(prompt.contains("Use the git_ui prefix."));
        assert!(prompt.contains("Follow the configured commit message format."));
        assert!(prompt.contains("Update generated message"));
        assert!(prompt.contains("diff --git a/file b/file"));

        let user_agents_md_index = prompt.find("<rules>").unwrap();
        let project_rules_index = prompt.find("<project_rules>").unwrap();
        let instructions_index = prompt.find("<commit_message_instructions>").unwrap();
        assert!(user_agents_md_index < project_rules_index);
        assert!(project_rules_index < instructions_index);
    }

    #[test]
    fn test_commit_message_prompt_omits_blank_instructions() {
        let prompt = GitPanel::build_commit_message_prompt(
            "Write a commit message.",
            None,
            None,
            Some("   \n  "),
            "",
            "diff --git a/file b/file",
        );

        assert!(!prompt.contains("<commit_message_instructions>"));
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
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        let panel = workspace.update_in(cx, GitPanel::new);

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
            panel.toggle_staged_for_entry(&first_status_entry, StageIntent::Toggle, window, cx);
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
            panel.toggle_staged_for_entry(&second_status_entry, StageIntent::Toggle, window, cx);
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
            panel.toggle_staged_for_entry(&first_status_entry, StageIntent::Toggle, window, cx);
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
            panel.toggle_staged_for_entry(&second_status_entry, StageIntent::Toggle, window, cx);
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

        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().group_by =
                        Some(GitPanelGroupBy::Staging);
                });
            });
        });
        await_git_panel_entries(&panel, cx).await;

        let message = panel.update(cx, |panel, cx| panel.suggest_commit_message(cx));
        assert_eq!(message, Some("Update tracked".to_string()));
    }

    #[test]
    fn test_git_output_handler_strips_ansi_codes() {
        let cases = [
            ("no escape codes here\n", "no escape codes here\n"),
            ("\x1b[31mhello\x1b[0m", "hello"),
            ("\x1b[1;32mfoo\x1b[0m bar", "foo bar"),
            ("progress 10%\rprogress 100%\n", "progress 100%\n"),
        ];

        for (input, expected) in cases {
            assert_eq!(terminal::strip_ansi_text(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_commit_title_exceeds_limit() {
        // ASCII only
        let within_ascii = "abcde";
        let exceeds_ascii = "abcdef";
        assert!(!commit_title_exceeds_limit(within_ascii, 5));
        assert!(commit_title_exceeds_limit(exceeds_ascii, 5));

        // Multi-byte characters are counted as grapheme clusters
        let within_japanese = "あいうえお"; // 5 chars, 15 bytes
        let exceeds_japanese = "あいうえおか"; // 6 chars, 18 bytes
        assert!(!commit_title_exceeds_limit(within_japanese, 5));
        assert!(commit_title_exceeds_limit(exceeds_japanese, 5));

        // Mixed ASCII + multi-byte
        let within_mixed = "abcあ";
        let exceeds_mixed = "abcああ";
        assert!(!commit_title_exceeds_limit(within_mixed, 4));
        assert!(commit_title_exceeds_limit(exceeds_mixed, 4));

        // Emoji counts as one character each
        let within_emoji = "🚀";
        let exceeds_emoji = "🚀🚀";
        assert!(!commit_title_exceeds_limit(within_emoji, 1));
        assert!(commit_title_exceeds_limit(exceeds_emoji, 1));

        // A max_length of 0 disables the limit check
        assert!(!commit_title_exceeds_limit(
            "anything goes when disabled",
            0
        ));
        assert!(!commit_title_exceeds_limit("", 0));

        // Empty title never exceeds a positive limit
        assert!(!commit_title_exceeds_limit("", 72));
    }

    #[gpui::test]
    async fn test_dispatch_context_with_focus_states(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "tracked": "tracked\n",
            }),
        )
        .await;

        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[("tracked", "old tracked\n".into())],
        );

        let project = Project::test(fs.clone(), [Path::new(path!("/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        let panel = workspace.update_in(cx, GitPanel::new);

        let handle = cx.update_window_entity(&panel, |panel, _, _| {
            std::mem::replace(&mut panel.update_visible_entries_task, Task::ready(()))
        });
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        handle.await;

        // Case 1: Focus the commit editor — should have "CommitEditor" but NOT "menu"/"ChangesList"
        panel.update_in(cx, |panel, window, cx| {
            panel.focus_editor(&FocusEditor, window, cx);
            let editor_is_focused = panel.commit_editor.read(cx).is_focused(window);
            assert!(
                editor_is_focused,
                "commit editor should be focused after focus_editor action"
            );
            let context = panel.dispatch_context(window, cx);
            assert!(
                context.contains("GitPanel"),
                "should always have GitPanel context"
            );
            assert!(
                context.contains("CommitEditor"),
                "should have CommitEditor context when commit editor is focused"
            );
            assert!(
                !context.contains("menu"),
                "should not have menu context when commit editor is focused"
            );
            assert!(
                !context.contains("ChangesList"),
                "should not have ChangesList context when commit editor is focused"
            );
        });

        // Case 2: Focus the panel's focus handle directly — should have "menu" and "ChangesList".
        // We force a draw via simulate_resize to ensure the dispatch tree is populated,
        // since contains_focused() depends on the rendered dispatch tree.
        panel.update_in(cx, |panel, window, cx| {
            panel.focus_handle.focus(window, cx);
        });
        cx.simulate_resize(gpui::size(px(800.), px(600.)));

        panel.update_in(cx, |panel, window, cx| {
            let context = panel.dispatch_context(window, cx);
            assert!(
                context.contains("GitPanel"),
                "should always have GitPanel context"
            );
            assert!(
                context.contains("menu"),
                "should have menu context when changes list is focused"
            );
            assert!(
                context.contains("ChangesList"),
                "should have ChangesList context when changes list is focused"
            );
            assert!(
                !context.contains("CommitEditor"),
                "should not have CommitEditor context when changes list is focused"
            );
        });

        // Case 3: Switch back to commit editor and verify context switches correctly
        panel.update_in(cx, |panel, window, cx| {
            panel.focus_editor(&FocusEditor, window, cx);
        });

        panel.update_in(cx, |panel, window, cx| {
            let context = panel.dispatch_context(window, cx);
            assert!(
                context.contains("CommitEditor"),
                "should have CommitEditor after switching focus back to editor"
            );
            assert!(
                !context.contains("menu"),
                "should not have menu after switching focus back to editor"
            );
        });

        // Case 4: Re-focus changes list and verify it transitions back correctly
        panel.update_in(cx, |panel, window, cx| {
            panel.focus_handle.focus(window, cx);
        });
        cx.simulate_resize(gpui::size(px(800.), px(600.)));

        panel.update_in(cx, |panel, window, cx| {
            assert!(
                panel.focus_handle.contains_focused(window, cx),
                "panel focus handle should report contains_focused when directly focused"
            );
            let context = panel.dispatch_context(window, cx);
            assert!(
                context.contains("menu"),
                "should have menu context after re-focusing changes list"
            );
            assert!(
                context.contains("ChangesList"),
                "should have ChangesList context after re-focusing changes list"
            );
        });
    }

    #[gpui::test]
    async fn test_fill_commit_editor_toggle(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root",
            json!({ "project": { ".git": {}, "src": { "main.rs": "fn main() {}" } } }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new(path!("/root/project"))], cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window_handle.into(), cx);
        cx.executor().run_until_parked();

        let panel = workspace.update_in(cx, GitPanel::new);

        panel.update_in(cx, |panel, window, cx| {
            assert!(!panel.commit_editor_expanded);
            assert!(matches!(
                panel.commit_editor.read(cx).mode().clone(),
                EditorMode::AutoHeight { .. }
            ));

            panel.toggle_fill_commit_editor(&ToggleFillCommitEditor, window, cx);
            assert!(panel.commit_editor_expanded);
            assert!(matches!(
                panel.commit_editor.read(cx).mode().clone(),
                EditorMode::Full { .. }
            ));

            panel.toggle_fill_commit_editor(&ToggleFillCommitEditor, window, cx);
            assert!(!panel.commit_editor_expanded);
            assert!(matches!(
                panel.commit_editor.read(cx).mode().clone(),
                EditorMode::AutoHeight { .. }
            ));
        });
    }

    #[gpui::test]
    async fn test_focus_handle(cx: &mut TestAppContext) {
        init_test(cx);

        let (_project, workspace, panel, mut cx) = setup_git_panel_with_changes(
            cx,
            json!({
                ".git": {},
                "tracked": "tracked\n",
            }),
            &[("tracked", StatusCode::Modified)],
        )
        .await;

        workspace.update_in(&mut cx, |workspace, window, cx| {
            workspace.add_panel(panel.clone(), window, cx);
        });

        // With changes present and the editor not expanded, the panel's own
        // focus handle should be returned, in order for
        // `git_panel::ToggleFocus` to focus on the panel itself.
        panel.update_in(&mut cx, |panel, _window, cx| {
            assert!(!panel.entries.is_empty());
            assert!(!panel.commit_editor_expanded);
            assert_eq!(panel.focus_handle(cx), panel.focus_handle.clone());
        });

        // Expand the editor so we can later confirm that toggling focus
        // actually focuses on the commit editor, seeing as it has been
        // expanded.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_fill_commit_editor(&ToggleFillCommitEditor, window, cx);
            assert!(panel.commit_editor_expanded);
        });

        cx.dispatch_action(super::ToggleFocus);
        panel.update_in(&mut cx, |panel, window, cx| {
            assert!(panel.commit_editor.focus_handle(cx).is_focused(window));
        });
    }
}
