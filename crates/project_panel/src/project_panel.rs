mod project_panel_settings;
mod utils;

use anyhow::{Context as _, Result};
use client::{ErrorCode, ErrorExt};
use collections::{BTreeSet, HashMap, hash_map};
use command_palette_hooks::CommandPaletteFilter;
use db::kvp::KEY_VALUE_STORE;
use editor::{
    Editor, EditorEvent, MultiBufferOffset,
    items::{
        entry_diagnostic_aware_icon_decoration_and_color,
        entry_diagnostic_aware_icon_name_and_color, entry_git_aware_label_color,
    },
};
use file_icons::FileIcons;
use git;
use git::status::GitSummary;
use git_ui;
use git_ui::file_diff_view::FileDiffView;
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Bounds, ClipboardItem, Context, CursorStyle,
    DismissEvent, Div, DragMoveEvent, Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable,
    Hsla, InteractiveElement, KeyContext, ListHorizontalSizingBehavior, ListSizingBehavior,
    Modifiers, ModifiersChangedEvent, MouseButton, MouseDownEvent, ParentElement, Pixels, Point,
    PromptLevel, Render, ScrollStrategy, Stateful, Styled, Subscription, Task,
    UniformListScrollHandle, WeakEntity, Window, actions, anchored, deferred, div, hsla,
    linear_color_stop, linear_gradient, point, px, size, transparent_white, uniform_list,
};
use language::DiagnosticSeverity;
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::{
    Entry, EntryKind, Fs, GitEntry, GitEntryRef, GitTraversal, Project, ProjectEntryId,
    ProjectPath, Worktree, WorktreeId,
    git_store::{GitStoreEvent, RepositoryEvent, git_traversal::ChildEntriesGitIter},
    project_settings::GoToDiagnosticSeverityFilter,
};
use project_panel_settings::ProjectPanelSettings;
use rayon::slice::ParallelSliceMut;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DockSide, ProjectPanelEntrySpacing, Settings, SettingsStore, ShowDiagnostics, ShowIndentGuides,
    update_settings_file,
};
use smallvec::SmallVec;
use std::{any::TypeId, time::Instant};
use std::{
    cell::OnceCell,
    cmp,
    collections::HashSet,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::ThemeSettings;
use ui::{
    Color, ContextMenu, DecoratedIcon, Divider, Icon, IconDecoration, IconDecorationKind,
    IndentGuideColors, IndentGuideLayout, KeyBinding, Label, LabelSize, ListItem, ListItemSpacing,
    ScrollAxes, ScrollableHandle, Scrollbars, StickyCandidate, Tooltip, WithScrollbar, prelude::*,
    v_flex,
};
use util::{ResultExt, TakeUntilExt, TryFutureExt, maybe, paths::compare_paths, rel_path::RelPath};
use workspace::{
    DraggedSelection, OpenInTerminal, OpenOptions, OpenVisible, PreviewTabsSettings, SelectedEntry,
    SplitDirection, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{DetachAndPromptErr, NotifyResultExt, NotifyTaskExt},
};
use worktree::CreatedEntry;
use zed_actions::{project_panel::ToggleFocus, workspace::OpenWithSystem};

const PROJECT_PANEL_KEY: &str = "ProjectPanel";
const NEW_ENTRY_ID: ProjectEntryId = ProjectEntryId::MAX;

struct VisibleEntriesForWorktree {
    worktree_id: WorktreeId,
    entries: Vec<GitEntry>,
    index: OnceCell<HashSet<Arc<RelPath>>>,
}

struct State {
    last_worktree_root_id: Option<ProjectEntryId>,
    /// Maps from leaf project entry ID to the currently selected ancestor.
    /// Relevant only for auto-fold dirs, where a single project panel entry may actually consist of several
    /// project entries (and all non-leaf nodes are guaranteed to be directories).
    ancestors: HashMap<ProjectEntryId, FoldedAncestors>,
    visible_entries: Vec<VisibleEntriesForWorktree>,
    max_width_item_index: Option<usize>,
    // Currently selected leaf entry (see auto-folding for a definition of that) in a file tree
    selection: Option<SelectedEntry>,
    edit_state: Option<EditState>,
    unfolded_dir_ids: HashSet<ProjectEntryId>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
}

impl State {
    fn derive(old: &Self) -> Self {
        Self {
            last_worktree_root_id: None,
            ancestors: Default::default(),
            visible_entries: Default::default(),
            max_width_item_index: None,
            edit_state: old.edit_state.clone(),
            unfolded_dir_ids: old.unfolded_dir_ids.clone(),
            selection: old.selection,
            expanded_dir_ids: old.expanded_dir_ids.clone(),
        }
    }
}

pub struct ProjectPanel {
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    // An update loop that keeps incrementing/decrementing scroll offset while there is a dragged entry that's
    // hovered over the start/end of a list.
    hover_scroll_task: Option<Task<()>>,
    rendered_entries_len: usize,
    folded_directory_drag_target: Option<FoldedDirectoryDragTarget>,
    drag_target_entry: Option<DragTarget>,
    marked_entries: Vec<SelectedEntry>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    filename_editor: Entity<Editor>,
    clipboard: Option<ClipboardEntry>,
    _dragged_entry_destination: Option<Arc<Path>>,
    workspace: WeakEntity<Workspace>,
    width: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
    diagnostics: HashMap<(WorktreeId, Arc<RelPath>), DiagnosticSeverity>,
    diagnostic_summary_update: Task<()>,
    // We keep track of the mouse down state on entries so we don't flash the UI
    // in case a user clicks to open a file.
    mouse_down: bool,
    hover_expand_task: Option<Task<()>>,
    previous_drag_position: Option<Point<Pixels>>,
    sticky_items_count: usize,
    last_reported_update: Instant,
    update_visible_entries_task: UpdateVisibleEntriesTask,
    state: State,
}

struct UpdateVisibleEntriesTask {
    _visible_entries_task: Task<()>,
    focus_filename_editor: bool,
    autoscroll: bool,
}

impl Default for UpdateVisibleEntriesTask {
    fn default() -> Self {
        UpdateVisibleEntriesTask {
            _visible_entries_task: Task::ready(()),
            focus_filename_editor: Default::default(),
            autoscroll: Default::default(),
        }
    }
}

enum DragTarget {
    /// Dragging on an entry
    Entry {
        /// The entry currently under the mouse cursor during a drag operation
        entry_id: ProjectEntryId,
        /// Highlight this entry along with all of its children
        highlight_entry_id: ProjectEntryId,
    },
    /// Dragging on background
    Background,
}

#[derive(Copy, Clone, Debug)]
struct FoldedDirectoryDragTarget {
    entry_id: ProjectEntryId,
    index: usize,
    /// Whether we are dragging over the delimiter rather than the component itself.
    is_delimiter_target: bool,
}

#[derive(Clone, Debug)]
enum ValidationState {
    None,
    Warning(String),
    Error(String),
}

#[derive(Clone, Debug)]
struct EditState {
    worktree_id: WorktreeId,
    entry_id: ProjectEntryId,
    leaf_entry_id: Option<ProjectEntryId>,
    is_dir: bool,
    depth: usize,
    processing_filename: Option<Arc<RelPath>>,
    previously_focused: Option<SelectedEntry>,
    validation_state: ValidationState,
}

impl EditState {
    fn is_new_entry(&self) -> bool {
        self.leaf_entry_id.is_none()
    }
}

#[derive(Clone, Debug)]
enum ClipboardEntry {
    Copied(BTreeSet<SelectedEntry>),
    Cut(BTreeSet<SelectedEntry>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct EntryDetails {
    filename: String,
    icon: Option<SharedString>,
    path: Arc<RelPath>,
    depth: usize,
    kind: EntryKind,
    is_ignored: bool,
    is_expanded: bool,
    is_selected: bool,
    is_marked: bool,
    is_editing: bool,
    is_processing: bool,
    is_cut: bool,
    sticky: Option<StickyDetails>,
    filename_text_color: Color,
    diagnostic_severity: Option<DiagnosticSeverity>,
    git_status: GitSummary,
    is_private: bool,
    worktree_id: WorktreeId,
    canonical_path: Option<Arc<Path>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct StickyDetails {
    sticky_index: usize,
}

/// Permanently deletes the selected file or directory.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = project_panel)]
#[serde(deny_unknown_fields)]
struct Delete {
    #[serde(default)]
    pub skip_prompt: bool,
}

/// Moves the selected file or directory to the system trash.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = project_panel)]
#[serde(deny_unknown_fields)]
struct Trash {
    #[serde(default)]
    pub skip_prompt: bool,
}

/// Selects the next entry with diagnostics.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = project_panel)]
#[serde(deny_unknown_fields)]
struct SelectNextDiagnostic {
    #[serde(default)]
    pub severity: GoToDiagnosticSeverityFilter,
}

/// Selects the previous entry with diagnostics.
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = project_panel)]
#[serde(deny_unknown_fields)]
struct SelectPrevDiagnostic {
    #[serde(default)]
    pub severity: GoToDiagnosticSeverityFilter,
}

actions!(
    project_panel,
    [
        /// Expands the selected entry in the project tree.
        ExpandSelectedEntry,
        /// Collapses the selected entry in the project tree.
        CollapseSelectedEntry,
        /// Collapses all entries in the project tree.
        CollapseAllEntries,
        /// Creates a new directory.
        NewDirectory,
        /// Creates a new file.
        NewFile,
        /// Copies the selected file or directory.
        Copy,
        /// Duplicates the selected file or directory.
        Duplicate,
        /// Reveals the selected item in the system file manager.
        RevealInFileManager,
        /// Removes the selected folder from the project.
        RemoveFromProject,
        /// Cuts the selected file or directory.
        Cut,
        /// Pastes the previously cut or copied item.
        Paste,
        /// Renames the selected file or directory.
        Rename,
        /// Opens the selected file in the editor.
        Open,
        /// Opens the selected file in a permanent tab.
        OpenPermanent,
        /// Opens the selected file in a vertical split.
        OpenSplitVertical,
        /// Opens the selected file in a horizontal split.
        OpenSplitHorizontal,
        /// Toggles visibility of git-ignored files.
        ToggleHideGitIgnore,
        /// Toggles visibility of hidden files.
        ToggleHideHidden,
        /// Starts a new search in the selected directory.
        NewSearchInDirectory,
        /// Unfolds the selected directory.
        UnfoldDirectory,
        /// Folds the selected directory.
        FoldDirectory,
        /// Scroll half a page upwards
        ScrollUp,
        /// Scroll half a page downwards
        ScrollDown,
        /// Scroll until the cursor displays at the center
        ScrollCursorCenter,
        /// Scroll until the cursor displays at the top
        ScrollCursorTop,
        /// Scroll until the cursor displays at the bottom
        ScrollCursorBottom,
        /// Selects the parent directory.
        SelectParent,
        /// Selects the next entry with git changes.
        SelectNextGitEntry,
        /// Selects the previous entry with git changes.
        SelectPrevGitEntry,
        /// Selects the next directory.
        SelectNextDirectory,
        /// Selects the previous directory.
        SelectPrevDirectory,
        /// Opens a diff view to compare two marked files.
        CompareMarkedFiles,
    ]
);

#[derive(Clone, Debug, Default)]
struct FoldedAncestors {
    current_ancestor_depth: usize,
    ancestors: Vec<ProjectEntryId>,
}

impl FoldedAncestors {
    fn max_ancestor_depth(&self) -> usize {
        self.ancestors.len()
    }

    /// Note: This returns None for last item in ancestors list
    fn active_ancestor(&self) -> Option<ProjectEntryId> {
        if self.current_ancestor_depth == 0 {
            return None;
        }
        self.ancestors.get(self.current_ancestor_depth).copied()
    }

    fn active_index(&self) -> usize {
        self.max_ancestor_depth()
            .saturating_sub(1)
            .saturating_sub(self.current_ancestor_depth)
    }

    fn active_component(&self, file_name: &str) -> Option<String> {
        Path::new(file_name)
            .components()
            .nth(self.active_index())
            .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<ProjectPanel>(window, cx);
        });

        workspace.register_action(|workspace, _: &ToggleHideGitIgnore, _, cx| {
            let fs = workspace.app_state().fs.clone();
            update_settings_file(fs, cx, move |setting, _| {
                setting.project_panel.get_or_insert_default().hide_gitignore = Some(
                    !setting
                        .project_panel
                        .get_or_insert_default()
                        .hide_gitignore
                        .unwrap_or(false),
                );
            })
        });

        workspace.register_action(|workspace, _: &ToggleHideHidden, _, cx| {
            let fs = workspace.app_state().fs.clone();
            update_settings_file(fs, cx, move |setting, _| {
                setting.project_panel.get_or_insert_default().hide_hidden = Some(
                    !setting
                        .project_panel
                        .get_or_insert_default()
                        .hide_hidden
                        .unwrap_or(false),
                );
            })
        });

        workspace.register_action(|workspace, action: &CollapseAllEntries, window, cx| {
            if let Some(panel) = workspace.panel::<ProjectPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.collapse_all_entries(action, window, cx);
                });
            }
        });

        workspace.register_action(|workspace, action: &Rename, window, cx| {
            workspace.open_panel::<ProjectPanel>(window, cx);
            if let Some(panel) = workspace.panel::<ProjectPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    if let Some(first_marked) = panel.marked_entries.first() {
                        let first_marked = *first_marked;
                        panel.marked_entries.clear();
                        panel.state.selection = Some(first_marked);
                    }
                    panel.rename(action, window, cx);
                });
            }
        });

        workspace.register_action(|workspace, action: &Duplicate, window, cx| {
            workspace.open_panel::<ProjectPanel>(window, cx);
            if let Some(panel) = workspace.panel::<ProjectPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.duplicate(action, window, cx);
                });
            }
        });

        workspace.register_action(|workspace, action: &Delete, window, cx| {
            if let Some(panel) = workspace.panel::<ProjectPanel>(cx) {
                panel.update(cx, |panel, cx| panel.delete(action, window, cx));
            }
        });

        workspace.register_action(|workspace, _: &git::FileHistory, window, cx| {
            // First try to get from project panel if it's focused
            if let Some(panel) = workspace.panel::<ProjectPanel>(cx) {
                let maybe_project_path = panel.read(cx).state.selection.and_then(|selection| {
                    let project = workspace.project().read(cx);
                    let worktree = project.worktree_for_id(selection.worktree_id, cx)?;
                    let entry = worktree.read(cx).entry_for_id(selection.entry_id)?;
                    if entry.is_file() {
                        Some(ProjectPath {
                            worktree_id: selection.worktree_id,
                            path: entry.path.clone(),
                        })
                    } else {
                        None
                    }
                });

                if let Some(project_path) = maybe_project_path {
                    let project = workspace.project();
                    let git_store = project.read(cx).git_store();
                    if let Some((repo, repo_path)) = git_store
                        .read(cx)
                        .repository_and_path_for_project_path(&project_path, cx)
                    {
                        git_ui::file_history_view::FileHistoryView::open(
                            repo_path,
                            git_store.downgrade(),
                            repo.downgrade(),
                            workspace.weak_handle(),
                            window,
                            cx,
                        );
                        return;
                    }
                }
            }

            // Fallback: try to get from active editor
            if let Some(active_item) = workspace.active_item(cx)
                && let Some(editor) = active_item.downcast::<Editor>()
                && let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton()
                && let Some(file) = buffer.read(cx).file()
            {
                let worktree_id = file.worktree_id(cx);
                let project_path = ProjectPath {
                    worktree_id,
                    path: file.path().clone(),
                };
                let project = workspace.project();
                let git_store = project.read(cx).git_store();
                if let Some((repo, repo_path)) = git_store
                    .read(cx)
                    .repository_and_path_for_project_path(&project_path, cx)
                {
                    git_ui::file_history_view::FileHistoryView::open(
                        repo_path,
                        git_store.downgrade(),
                        repo.downgrade(),
                        workspace.weak_handle(),
                        window,
                        cx,
                    );
                }
            }
        });
    })
    .detach();
}

#[derive(Debug)]
pub enum Event {
    OpenedEntry {
        entry_id: ProjectEntryId,
        focus_opened_item: bool,
        allow_preview: bool,
    },
    SplitEntry {
        entry_id: ProjectEntryId,
        allow_preview: bool,
        split_direction: Option<SplitDirection>,
    },
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedProjectPanel {
    width: Option<Pixels>,
}

struct DraggedProjectEntryView {
    selection: SelectedEntry,
    icon: Option<SharedString>,
    filename: String,
    click_offset: Point<Pixels>,
    selections: Arc<[SelectedEntry]>,
}

struct ItemColors {
    default: Hsla,
    hover: Hsla,
    drag_over: Hsla,
    marked: Hsla,
    focused: Hsla,
}

fn get_item_color(is_sticky: bool, cx: &App) -> ItemColors {
    let colors = cx.theme().colors();

    ItemColors {
        default: if is_sticky {
            colors.panel_overlay_background
        } else {
            colors.panel_background
        },
        hover: if is_sticky {
            colors.panel_overlay_hover
        } else {
            colors.element_hover
        },
        marked: colors.element_selected,
        focused: colors.panel_focused_border,
        drag_over: colors.drop_target_background,
    }
}

impl ProjectPanel {
    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let path_style = project.read(cx).path_style(cx);
        let project_panel = cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, Self::focus_in).detach();

            cx.subscribe_in(
                &git_store,
                window,
                |this, _, event, window, cx| match event {
                    GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::StatusesChanged, _)
                    | GitStoreEvent::RepositoryAdded
                    | GitStoreEvent::RepositoryRemoved(_) => {
                        this.update_visible_entries(None, false, false, window, cx);
                        cx.notify();
                    }
                    _ => {}
                },
            )
            .detach();

            cx.subscribe_in(
                &project,
                window,
                |this, project, event, window, cx| match event {
                    project::Event::ActiveEntryChanged(Some(entry_id)) => {
                        if ProjectPanelSettings::get_global(cx).auto_reveal_entries {
                            this.reveal_entry(project.clone(), *entry_id, true, window, cx)
                                .ok();
                        }
                    }
                    project::Event::ActiveEntryChanged(None) => {
                        let is_active_item_file_diff_view = this
                            .workspace
                            .upgrade()
                            .and_then(|ws| ws.read(cx).active_item(cx))
                            .map(|item| {
                                item.act_as_type(TypeId::of::<FileDiffView>(), cx).is_some()
                            })
                            .unwrap_or(false);
                        if !is_active_item_file_diff_view {
                            this.marked_entries.clear();
                        }
                    }
                    project::Event::RevealInProjectPanel(entry_id) => {
                        if let Some(()) = this
                            .reveal_entry(project.clone(), *entry_id, false, window, cx)
                            .log_err()
                        {
                            cx.emit(PanelEvent::Activate);
                        }
                    }
                    project::Event::ActivateProjectPanel => {
                        cx.emit(PanelEvent::Activate);
                    }
                    project::Event::DiskBasedDiagnosticsFinished { .. }
                    | project::Event::DiagnosticsUpdated { .. } => {
                        if ProjectPanelSettings::get_global(cx).show_diagnostics
                            != ShowDiagnostics::Off
                        {
                            this.diagnostic_summary_update = cx.spawn(async move |this, cx| {
                                cx.background_executor()
                                    .timer(Duration::from_millis(30))
                                    .await;
                                this.update(cx, |this, cx| {
                                    this.update_diagnostics(cx);
                                    cx.notify();
                                })
                                .log_err();
                            });
                        }
                    }
                    project::Event::WorktreeRemoved(id) => {
                        this.state.expanded_dir_ids.remove(id);
                        this.update_visible_entries(None, false, false, window, cx);
                        cx.notify();
                    }
                    project::Event::WorktreeUpdatedEntries(_, _)
                    | project::Event::WorktreeAdded(_)
                    | project::Event::WorktreeOrderChanged => {
                        this.update_visible_entries(None, false, false, window, cx);
                        cx.notify();
                    }
                    project::Event::ExpandedAllForEntry(worktree_id, entry_id) => {
                        if let Some((worktree, expanded_dir_ids)) = project
                            .read(cx)
                            .worktree_for_id(*worktree_id, cx)
                            .zip(this.state.expanded_dir_ids.get_mut(worktree_id))
                        {
                            let worktree = worktree.read(cx);

                            let Some(entry) = worktree.entry_for_id(*entry_id) else {
                                return;
                            };
                            let include_ignored_dirs = !entry.is_ignored;

                            let mut dirs_to_expand = vec![*entry_id];
                            while let Some(current_id) = dirs_to_expand.pop() {
                                let Some(current_entry) = worktree.entry_for_id(current_id) else {
                                    continue;
                                };
                                for child in worktree.child_entries(&current_entry.path) {
                                    if !child.is_dir() || (include_ignored_dirs && child.is_ignored)
                                    {
                                        continue;
                                    }

                                    dirs_to_expand.push(child.id);

                                    if let Err(ix) = expanded_dir_ids.binary_search(&child.id) {
                                        expanded_dir_ids.insert(ix, child.id);
                                    }
                                    this.state.unfolded_dir_ids.insert(child.id);
                                }
                            }
                            this.update_visible_entries(None, false, false, window, cx);
                            cx.notify();
                        }
                    }
                    _ => {}
                },
            )
            .detach();

            let trash_action = [TypeId::of::<Trash>()];
            let is_remote = project.read(cx).is_remote();

            // Make sure the trash option is never displayed anywhere on remote
            // hosts since they may not support trashing. May want to dynamically
            // detect this in the future.
            if is_remote {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.hide_action_types(&trash_action);
                });
            }

            let filename_editor = cx.new(|cx| Editor::single_line(window, cx));

            cx.subscribe_in(
                &filename_editor,
                window,
                |project_panel, _, editor_event, window, cx| match editor_event {
                    EditorEvent::BufferEdited => {
                        project_panel.populate_validation_error(cx);
                        project_panel.autoscroll(cx);
                    }
                    EditorEvent::SelectionsChanged { .. } => {
                        project_panel.autoscroll(cx);
                    }
                    EditorEvent::Blurred => {
                        if project_panel
                            .state
                            .edit_state
                            .as_ref()
                            .is_some_and(|state| state.processing_filename.is_none())
                        {
                            match project_panel.confirm_edit(false, window, cx) {
                                Some(task) => {
                                    task.detach_and_notify_err(window, cx);
                                }
                                None => {
                                    project_panel.state.edit_state = None;
                                    project_panel
                                        .update_visible_entries(None, false, false, window, cx);
                                    cx.notify();
                                }
                            }
                        }
                    }
                    _ => {}
                },
            )
            .detach();

            cx.observe_global::<FileIcons>(|_, cx| {
                cx.notify();
            })
            .detach();

            let mut project_panel_settings = *ProjectPanelSettings::get_global(cx);
            cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
                let new_settings = *ProjectPanelSettings::get_global(cx);
                if project_panel_settings != new_settings {
                    if project_panel_settings.hide_gitignore != new_settings.hide_gitignore {
                        this.update_visible_entries(None, false, false, window, cx);
                    }
                    if project_panel_settings.hide_root != new_settings.hide_root {
                        this.update_visible_entries(None, false, false, window, cx);
                    }
                    if project_panel_settings.hide_hidden != new_settings.hide_hidden {
                        this.update_visible_entries(None, false, false, window, cx);
                    }
                    if project_panel_settings.sort_mode != new_settings.sort_mode {
                        this.update_visible_entries(None, false, false, window, cx);
                    }
                    if project_panel_settings.sticky_scroll && !new_settings.sticky_scroll {
                        this.sticky_items_count = 0;
                    }
                    project_panel_settings = new_settings;
                    this.update_diagnostics(cx);
                    cx.notify();
                }
            })
            .detach();

            let scroll_handle = UniformListScrollHandle::new();
            let mut this = Self {
                project: project.clone(),
                hover_scroll_task: None,
                fs: workspace.app_state().fs.clone(),
                focus_handle,
                rendered_entries_len: 0,
                folded_directory_drag_target: None,
                drag_target_entry: None,

                marked_entries: Default::default(),
                context_menu: None,
                filename_editor,
                clipboard: None,
                _dragged_entry_destination: None,
                workspace: workspace.weak_handle(),
                width: None,
                pending_serialization: Task::ready(None),
                diagnostics: Default::default(),
                diagnostic_summary_update: Task::ready(()),
                scroll_handle,
                mouse_down: false,
                hover_expand_task: None,
                previous_drag_position: None,
                sticky_items_count: 0,
                last_reported_update: Instant::now(),
                state: State {
                    max_width_item_index: None,
                    edit_state: None,
                    selection: None,
                    last_worktree_root_id: Default::default(),
                    visible_entries: Default::default(),
                    ancestors: Default::default(),
                    expanded_dir_ids: Default::default(),
                    unfolded_dir_ids: Default::default(),
                },
                update_visible_entries_task: Default::default(),
            };
            this.update_visible_entries(None, false, false, window, cx);

            this
        });

        cx.subscribe_in(&project_panel, window, {
            let project_panel = project_panel.downgrade();
            move |workspace, _, event, window, cx| match event {
                &Event::OpenedEntry {
                    entry_id,
                    focus_opened_item,
                    allow_preview,
                } => {
                    if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx)
                        && let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                            let file_path = entry.path.clone();
                            let worktree_id = worktree.read(cx).id();
                            let entry_id = entry.id;
                            let is_via_ssh = project.read(cx).is_via_remote_server();

                            workspace
                                .open_path_preview(
                                    ProjectPath {
                                        worktree_id,
                                        path: file_path.clone(),
                                    },
                                    None,
                                    focus_opened_item,
                                    allow_preview,
                                    true,
                                    window, cx,
                                )
                                .detach_and_prompt_err("Failed to open file", window, cx, move |e, _, _| {
                                    match e.error_code() {
                                        ErrorCode::Disconnected => if is_via_ssh {
                                            Some("Disconnected from SSH host".to_string())
                                        } else {
                                            Some("Disconnected from remote project".to_string())
                                        },
                                        ErrorCode::UnsharedItem => Some(format!(
                                            "{} is not shared by the host. This could be because it has been marked as `private`",
                                            file_path.display(path_style)
                                        )),
                                        // See note in worktree.rs where this error originates. Returning Some in this case prevents
                                        // the error popup from saying "Try Again", which is a red herring in this case
                                        ErrorCode::Internal if e.to_string().contains("File is too large to load") => Some(e.to_string()),
                                        _ => None,
                                    }
                                });

                            if let Some(project_panel) = project_panel.upgrade() {
                                // Always select and mark the entry, regardless of whether it is opened or not.
                                project_panel.update(cx, |project_panel, _| {
                                    let entry = SelectedEntry { worktree_id, entry_id };
                                    project_panel.marked_entries.clear();
                                    project_panel.marked_entries.push(entry);
                                    project_panel.state.selection = Some(entry);
                                });
                                if !focus_opened_item {
                                    let focus_handle = project_panel.read(cx).focus_handle.clone();
                                    window.focus(&focus_handle, cx);
                                }
                            }
                        }
                }
                &Event::SplitEntry {
                    entry_id,
                    allow_preview,
                    split_direction,
                } => {
                    if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx)
                        && let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                            workspace
                                .split_path_preview(
                                    ProjectPath {
                                        worktree_id: worktree.read(cx).id(),
                                        path: entry.path.clone(),
                                    },
                                    allow_preview,
                                    split_direction,
                                    window, cx,
                                )
                                .detach_and_log_err(cx);
                        }
                }

                _ => {}
            }
        })
        .detach();

        project_panel
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let serialized_panel = match workspace
            .read_with(&cx, |workspace, _| {
                ProjectPanel::serialization_key(workspace)
            })
            .ok()
            .flatten()
        {
            Some(serialization_key) => cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(&serialization_key) })
                .await
                .context("loading project panel")
                .log_err()
                .flatten()
                .map(|panel| serde_json::from_str::<SerializedProjectPanel>(&panel))
                .transpose()
                .log_err()
                .flatten(),
            None => None,
        };

        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width.map(|px| px.round());
                    cx.notify();
                });
            }
            panel
        })
    }

    fn update_diagnostics(&mut self, cx: &mut Context<Self>) {
        let mut diagnostics: HashMap<(WorktreeId, Arc<RelPath>), DiagnosticSeverity> =
            Default::default();
        let show_diagnostics_setting = ProjectPanelSettings::get_global(cx).show_diagnostics;

        if show_diagnostics_setting != ShowDiagnostics::Off {
            self.project
                .read(cx)
                .diagnostic_summaries(false, cx)
                .filter_map(|(path, _, diagnostic_summary)| {
                    if diagnostic_summary.error_count > 0 {
                        Some((path, DiagnosticSeverity::ERROR))
                    } else if show_diagnostics_setting == ShowDiagnostics::All
                        && diagnostic_summary.warning_count > 0
                    {
                        Some((path, DiagnosticSeverity::WARNING))
                    } else {
                        None
                    }
                })
                .for_each(|(project_path, diagnostic_severity)| {
                    let ancestors = project_path.path.ancestors().collect::<Vec<_>>();
                    for path in ancestors.into_iter().rev() {
                        Self::update_strongest_diagnostic_severity(
                            &mut diagnostics,
                            &project_path,
                            path.into(),
                            diagnostic_severity,
                        );
                    }
                });
        }
        self.diagnostics = diagnostics;
    }

    fn update_strongest_diagnostic_severity(
        diagnostics: &mut HashMap<(WorktreeId, Arc<RelPath>), DiagnosticSeverity>,
        project_path: &ProjectPath,
        path_buffer: Arc<RelPath>,
        diagnostic_severity: DiagnosticSeverity,
    ) {
        diagnostics
            .entry((project_path.worktree_id, path_buffer))
            .and_modify(|strongest_diagnostic_severity| {
                *strongest_diagnostic_severity =
                    cmp::min(*strongest_diagnostic_severity, diagnostic_severity);
            })
            .or_insert(diagnostic_severity);
    }

    fn serialization_key(workspace: &Workspace) -> Option<String> {
        workspace
            .database_id()
            .map(|id| i64::from(id).to_string())
            .or(workspace.session_id())
            .map(|id| format!("{}-{:?}", PROJECT_PANEL_KEY, id))
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let Some(serialization_key) = self
            .workspace
            .read_with(cx, |workspace, _| {
                ProjectPanel::serialization_key(workspace)
            })
            .ok()
            .flatten()
        else {
            return;
        };
        let width = self.width;
        self.pending_serialization = cx.background_spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        serialization_key,
                        serde_json::to_string(&SerializedProjectPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.contains_focused(window, cx) {
            cx.emit(Event::Focus);
        }
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        entry_id: ProjectEntryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let project = self.project.read(cx);

        let worktree_id = if let Some(id) = project.worktree_id_for_entry(entry_id, cx) {
            id
        } else {
            return;
        };

        self.state.selection = Some(SelectedEntry {
            worktree_id,
            entry_id,
        });

        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            let auto_fold_dirs = ProjectPanelSettings::get_global(cx).auto_fold_dirs;
            let worktree = worktree.read(cx);
            let is_root = Some(entry) == worktree.root_entry();
            let is_dir = entry.is_dir();
            let is_foldable = auto_fold_dirs && self.is_foldable(entry, worktree);
            let is_unfoldable = auto_fold_dirs && self.is_unfoldable(entry, worktree);
            let is_read_only = project.is_read_only(cx);
            let is_remote = project.is_remote();
            let is_local = project.is_local();

            let settings = ProjectPanelSettings::get_global(cx);
            let visible_worktrees_count = project.visible_worktrees(cx).count();
            let should_hide_rename = is_root
                && (cfg!(target_os = "windows")
                    || (settings.hide_root && visible_worktrees_count == 1));
            let should_show_compare = !is_dir && self.file_abs_paths_to_diff(cx).is_some();

            let has_git_repo = !is_dir && {
                let project_path = project::ProjectPath {
                    worktree_id,
                    path: entry.path.clone(),
                };
                project
                    .git_store()
                    .read(cx)
                    .repository_and_path_for_project_path(&project_path, cx)
                    .is_some()
            };

            let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
                menu.context(self.focus_handle.clone()).map(|menu| {
                    if is_read_only {
                        menu.when(is_dir, |menu| {
                            menu.action("Search Inside", Box::new(NewSearchInDirectory))
                        })
                    } else {
                        menu.action("New File", Box::new(NewFile))
                            .action("New Folder", Box::new(NewDirectory))
                            .separator()
                            .when(is_local && cfg!(target_os = "macos"), |menu| {
                                menu.action("Reveal in Finder", Box::new(RevealInFileManager))
                            })
                            .when(is_local && cfg!(not(target_os = "macos")), |menu| {
                                menu.action("Reveal in File Manager", Box::new(RevealInFileManager))
                            })
                            .when(is_local, |menu| {
                                menu.action("Open in Default App", Box::new(OpenWithSystem))
                            })
                            .action("Open in Terminal", Box::new(OpenInTerminal))
                            .when(is_dir, |menu| {
                                menu.separator()
                                    .action("Find in Folder…", Box::new(NewSearchInDirectory))
                            })
                            .when(is_unfoldable, |menu| {
                                menu.action("Unfold Directory", Box::new(UnfoldDirectory))
                            })
                            .when(is_foldable, |menu| {
                                menu.action("Fold Directory", Box::new(FoldDirectory))
                            })
                            .when(should_show_compare, |menu| {
                                menu.separator()
                                    .action("Compare marked files", Box::new(CompareMarkedFiles))
                            })
                            .separator()
                            .action("Cut", Box::new(Cut))
                            .action("Copy", Box::new(Copy))
                            .action("Duplicate", Box::new(Duplicate))
                            // TODO: Paste should always be visible, cbut disabled when clipboard is empty
                            .action_disabled_when(
                                self.clipboard.as_ref().is_none(),
                                "Paste",
                                Box::new(Paste),
                            )
                            .separator()
                            .action("Copy Path", Box::new(zed_actions::workspace::CopyPath))
                            .action(
                                "Copy Relative Path",
                                Box::new(zed_actions::workspace::CopyRelativePath),
                            )
                            .when(!is_dir && self.has_git_changes(entry_id), |menu| {
                                menu.separator().action(
                                    "Restore File",
                                    Box::new(git::RestoreFile { skip_prompt: false }),
                                )
                            })
                            .when(has_git_repo, |menu| {
                                menu.separator()
                                    .action("View File History", Box::new(git::FileHistory))
                            })
                            .when(!should_hide_rename, |menu| {
                                menu.separator().action("Rename", Box::new(Rename))
                            })
                            .when(!is_root && !is_remote, |menu| {
                                menu.action("Trash", Box::new(Trash { skip_prompt: false }))
                            })
                            .when(!is_root, |menu| {
                                menu.action("Delete", Box::new(Delete { skip_prompt: false }))
                            })
                            .when(!is_remote && is_root, |menu| {
                                menu.separator()
                                    .action(
                                        "Add Folder to Project…",
                                        Box::new(workspace::AddFolderToProject),
                                    )
                                    .action("Remove from Project", Box::new(RemoveFromProject))
                            })
                            .when(is_root, |menu| {
                                menu.separator()
                                    .action("Collapse All", Box::new(CollapseAllEntries))
                            })
                    }
                })
            });

            window.focus(&context_menu.focus_handle(cx), cx);
            let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                this.context_menu.take();
                cx.notify();
            });
            self.context_menu = Some((context_menu, position, subscription));
        }

        cx.notify();
    }

    fn has_git_changes(&self, entry_id: ProjectEntryId) -> bool {
        for visible in &self.state.visible_entries {
            if let Some(git_entry) = visible.entries.iter().find(|e| e.id == entry_id) {
                let total_modified =
                    git_entry.git_summary.index.modified + git_entry.git_summary.worktree.modified;
                let total_deleted =
                    git_entry.git_summary.index.deleted + git_entry.git_summary.worktree.deleted;
                return total_modified > 0 || total_deleted > 0;
            }
        }
        false
    }

    fn is_unfoldable(&self, entry: &Entry, worktree: &Worktree) -> bool {
        if !entry.is_dir() || self.state.unfolded_dir_ids.contains(&entry.id) {
            return false;
        }

        if let Some(parent_path) = entry.path.parent() {
            let snapshot = worktree.snapshot();
            let mut child_entries = snapshot.child_entries(parent_path);
            if let Some(child) = child_entries.next()
                && child_entries.next().is_none()
            {
                return child.kind.is_dir();
            }
        };
        false
    }

    fn is_foldable(&self, entry: &Entry, worktree: &Worktree) -> bool {
        if entry.is_dir() {
            let snapshot = worktree.snapshot();

            let mut child_entries = snapshot.child_entries(&entry.path);
            if let Some(child) = child_entries.next()
                && child_entries.next().is_none()
            {
                return child.kind.is_dir();
            }
        }
        false
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            if let Some(folded_ancestors) = self.state.ancestors.get_mut(&entry.id)
                && folded_ancestors.current_ancestor_depth > 0
            {
                folded_ancestors.current_ancestor_depth -= 1;
                cx.notify();
                return;
            }
            if entry.is_dir() {
                let worktree_id = worktree.id();
                let entry_id = entry.id;
                let expanded_dir_ids = if let Some(expanded_dir_ids) =
                    self.state.expanded_dir_ids.get_mut(&worktree_id)
                {
                    expanded_dir_ids
                } else {
                    return;
                };

                match expanded_dir_ids.binary_search(&entry_id) {
                    Ok(_) => self.select_next(&SelectNext, window, cx),
                    Err(ix) => {
                        self.project.update(cx, |project, cx| {
                            project.expand_entry(worktree_id, entry_id, cx);
                        });

                        expanded_dir_ids.insert(ix, entry_id);
                        self.update_visible_entries(None, false, false, window, cx);
                        cx.notify();
                    }
                }
            }
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((worktree, entry)) = self.selected_entry_handle(cx) else {
            return;
        };
        self.collapse_entry(entry.clone(), worktree, window, cx)
    }

    fn collapse_entry(
        &mut self,
        entry: Entry,
        worktree: Entity<Worktree>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let worktree = worktree.read(cx);
        if let Some(folded_ancestors) = self.state.ancestors.get_mut(&entry.id)
            && folded_ancestors.current_ancestor_depth + 1 < folded_ancestors.max_ancestor_depth()
        {
            folded_ancestors.current_ancestor_depth += 1;
            cx.notify();
            return;
        }
        let worktree_id = worktree.id();
        let expanded_dir_ids =
            if let Some(expanded_dir_ids) = self.state.expanded_dir_ids.get_mut(&worktree_id) {
                expanded_dir_ids
            } else {
                return;
            };

        let mut entry = &entry;
        loop {
            let entry_id = entry.id;
            match expanded_dir_ids.binary_search(&entry_id) {
                Ok(ix) => {
                    expanded_dir_ids.remove(ix);
                    self.update_visible_entries(
                        Some((worktree_id, entry_id)),
                        false,
                        false,
                        window,
                        cx,
                    );
                    cx.notify();
                    break;
                }
                Err(_) => {
                    if let Some(parent_entry) =
                        entry.path.parent().and_then(|p| worktree.entry_for_path(p))
                    {
                        entry = parent_entry;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn collapse_all_entries(
        &mut self,
        _: &CollapseAllEntries,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // By keeping entries for fully collapsed worktrees, we avoid expanding them within update_visible_entries
        // (which is it's default behavior when there's no entry for a worktree in expanded_dir_ids).
        let multiple_worktrees = self.project.read(cx).worktrees(cx).count() > 1;
        let project = self.project.read(cx);

        self.state
            .expanded_dir_ids
            .iter_mut()
            .for_each(|(worktree_id, expanded_entries)| {
                if multiple_worktrees {
                    *expanded_entries = Default::default();
                    return;
                }

                let root_entry_id = project
                    .worktree_for_id(*worktree_id, cx)
                    .map(|worktree| worktree.read(cx).snapshot())
                    .and_then(|worktree_snapshot| {
                        worktree_snapshot.root_entry().map(|entry| entry.id)
                    });

                match root_entry_id {
                    Some(id) => {
                        expanded_entries.retain(|entry_id| entry_id == &id);
                    }
                    None => *expanded_entries = Default::default(),
                };
            });

        self.update_visible_entries(None, false, false, window, cx);
        cx.notify();
    }

    fn toggle_expanded(
        &mut self,
        entry_id: ProjectEntryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx)
            && let Some(expanded_dir_ids) = self.state.expanded_dir_ids.get_mut(&worktree_id)
        {
            self.project.update(cx, |project, cx| {
                match expanded_dir_ids.binary_search(&entry_id) {
                    Ok(ix) => {
                        expanded_dir_ids.remove(ix);
                    }
                    Err(ix) => {
                        project.expand_entry(worktree_id, entry_id, cx);
                        expanded_dir_ids.insert(ix, entry_id);
                    }
                }
            });
            self.update_visible_entries(Some((worktree_id, entry_id)), false, false, window, cx);
            window.focus(&self.focus_handle, cx);
            cx.notify();
        }
    }

    fn toggle_expand_all(
        &mut self,
        entry_id: ProjectEntryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx)
            && let Some(expanded_dir_ids) = self.state.expanded_dir_ids.get_mut(&worktree_id)
        {
            match expanded_dir_ids.binary_search(&entry_id) {
                Ok(_ix) => {
                    self.collapse_all_for_entry(worktree_id, entry_id, cx);
                }
                Err(_ix) => {
                    self.expand_all_for_entry(worktree_id, entry_id, cx);
                }
            }
            self.update_visible_entries(Some((worktree_id, entry_id)), false, false, window, cx);
            window.focus(&self.focus_handle, cx);
            cx.notify();
        }
    }

    fn expand_all_for_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut Context<Self>,
    ) {
        self.project.update(cx, |project, cx| {
            if let Some((worktree, expanded_dir_ids)) = project
                .worktree_for_id(worktree_id, cx)
                .zip(self.state.expanded_dir_ids.get_mut(&worktree_id))
            {
                if let Some(task) = project.expand_all_for_entry(worktree_id, entry_id, cx) {
                    task.detach();
                }

                let worktree = worktree.read(cx);

                if let Some(mut entry) = worktree.entry_for_id(entry_id) {
                    loop {
                        if let Err(ix) = expanded_dir_ids.binary_search(&entry.id) {
                            expanded_dir_ids.insert(ix, entry.id);
                        }

                        if let Some(parent_entry) =
                            entry.path.parent().and_then(|p| worktree.entry_for_path(p))
                        {
                            entry = parent_entry;
                        } else {
                            break;
                        }
                    }
                }
            }
        });
    }

    fn collapse_all_for_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut Context<Self>,
    ) {
        self.project.update(cx, |project, cx| {
            if let Some((worktree, expanded_dir_ids)) = project
                .worktree_for_id(worktree_id, cx)
                .zip(self.state.expanded_dir_ids.get_mut(&worktree_id))
            {
                let worktree = worktree.read(cx);
                let mut dirs_to_collapse = vec![entry_id];
                let auto_fold_enabled = ProjectPanelSettings::get_global(cx).auto_fold_dirs;
                while let Some(current_id) = dirs_to_collapse.pop() {
                    let Some(current_entry) = worktree.entry_for_id(current_id) else {
                        continue;
                    };
                    if let Ok(ix) = expanded_dir_ids.binary_search(&current_id) {
                        expanded_dir_ids.remove(ix);
                    }
                    if auto_fold_enabled {
                        self.state.unfolded_dir_ids.remove(&current_id);
                    }
                    for child in worktree.child_entries(&current_entry.path) {
                        if child.is_dir() {
                            dirs_to_collapse.push(child.id);
                        }
                    }
                }
            }
        });
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(edit_state) = &self.state.edit_state
            && edit_state.processing_filename.is_none()
        {
            self.filename_editor.update(cx, |editor, cx| {
                editor.move_to_beginning_of_line(
                    &editor::actions::MoveToBeginningOfLine {
                        stop_at_soft_wraps: false,
                        stop_at_indent: false,
                    },
                    window,
                    cx,
                );
            });
            return;
        }
        if let Some(selection) = self.state.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if entry_ix > 0 {
                entry_ix -= 1;
            } else if worktree_ix > 0 {
                worktree_ix -= 1;
                entry_ix = self.state.visible_entries[worktree_ix].entries.len() - 1;
            } else {
                return;
            }

            let VisibleEntriesForWorktree {
                worktree_id,
                entries,
                ..
            } = &self.state.visible_entries[worktree_ix];
            let selection = SelectedEntry {
                worktree_id: *worktree_id,
                entry_id: entries[entry_ix].id,
            };
            self.state.selection = Some(selection);
            if window.modifiers().shift {
                self.marked_entries.push(selection);
            }
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst {}, window, cx);
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(task) = self.confirm_edit(true, window, cx) {
            task.detach_and_notify_err(window, cx);
        }
    }

    fn open(&mut self, _: &Open, window: &mut Window, cx: &mut Context<Self>) {
        let preview_tabs_enabled =
            PreviewTabsSettings::get_global(cx).enable_preview_from_project_panel;
        self.open_internal(true, !preview_tabs_enabled, None, window, cx);
    }

    fn open_permanent(&mut self, _: &OpenPermanent, window: &mut Window, cx: &mut Context<Self>) {
        self.open_internal(false, true, None, window, cx);
    }

    fn open_split_vertical(
        &mut self,
        _: &OpenSplitVertical,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_internal(false, true, Some(SplitDirection::vertical(cx)), window, cx);
    }

    fn open_split_horizontal(
        &mut self,
        _: &OpenSplitHorizontal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_internal(
            false,
            true,
            Some(SplitDirection::horizontal(cx)),
            window,
            cx,
        );
    }

    fn open_internal(
        &mut self,
        allow_preview: bool,
        focus_opened_item: bool,
        split_direction: Option<SplitDirection>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((_, entry)) = self.selected_entry(cx) {
            if entry.is_file() {
                if split_direction.is_some() {
                    self.split_entry(entry.id, allow_preview, split_direction, cx);
                } else {
                    self.open_entry(entry.id, focus_opened_item, allow_preview, cx);
                }
                cx.notify();
            } else {
                self.toggle_expanded(entry.id, window, cx);
            }
        }
    }

    fn populate_validation_error(&mut self, cx: &mut Context<Self>) {
        let edit_state = match self.state.edit_state.as_mut() {
            Some(state) => state,
            None => return,
        };
        let filename = self.filename_editor.read(cx).text(cx);
        if !filename.is_empty() {
            if filename.is_empty() {
                edit_state.validation_state =
                    ValidationState::Error("File or directory name cannot be empty.".to_string());
                cx.notify();
                return;
            }

            let trimmed_filename = filename.trim();
            if trimmed_filename != filename {
                edit_state.validation_state = ValidationState::Warning(
                    "File or directory name contains leading or trailing whitespace.".to_string(),
                );
                cx.notify();
                return;
            }
            let trimmed_filename = trimmed_filename.trim_start_matches('/');

            let Ok(filename) = RelPath::unix(trimmed_filename) else {
                edit_state.validation_state = ValidationState::Warning(
                    "File or directory name contains leading or trailing whitespace.".to_string(),
                );
                cx.notify();
                return;
            };

            if let Some(worktree) = self
                .project
                .read(cx)
                .worktree_for_id(edit_state.worktree_id, cx)
                && let Some(entry) = worktree.read(cx).entry_for_id(edit_state.entry_id)
            {
                let mut already_exists = false;
                if edit_state.is_new_entry() {
                    let new_path = entry.path.join(filename);
                    if worktree.read(cx).entry_for_path(&new_path).is_some() {
                        already_exists = true;
                    }
                } else {
                    let new_path = if let Some(parent) = entry.path.clone().parent() {
                        parent.join(&filename)
                    } else {
                        filename.into()
                    };
                    if let Some(existing) = worktree.read(cx).entry_for_path(&new_path)
                        && existing.id != entry.id
                    {
                        already_exists = true;
                    }
                };
                if already_exists {
                    edit_state.validation_state = ValidationState::Error(format!(
                        "File or directory '{}' already exists at location. Please choose a different name.",
                        filename.as_unix_str()
                    ));
                    cx.notify();
                    return;
                }
            }
        }
        edit_state.validation_state = ValidationState::None;
        cx.notify();
    }

    fn confirm_edit(
        &mut self,
        refocus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let edit_state = self.state.edit_state.as_mut()?;
        let worktree_id = edit_state.worktree_id;
        let is_new_entry = edit_state.is_new_entry();
        let mut filename = self.filename_editor.read(cx).text(cx);
        let path_style = self.project.read(cx).path_style(cx);
        if path_style.is_windows() {
            // on windows, trailing dots are ignored in paths
            // this can cause project panel to create a new entry with a trailing dot
            // while the actual one without the dot gets populated by the file watcher
            while let Some(trimmed) = filename.strip_suffix('.') {
                filename = trimmed.to_string();
            }
        }
        if filename.trim().is_empty() {
            return None;
        }

        let filename_indicates_dir = if path_style.is_windows() {
            filename.ends_with('/') || filename.ends_with('\\')
        } else {
            filename.ends_with('/')
        };
        let filename = if path_style.is_windows() {
            filename.trim_start_matches(&['/', '\\'])
        } else {
            filename.trim_start_matches('/')
        };
        let filename = RelPath::new(filename.as_ref(), path_style).ok()?.into_arc();

        edit_state.is_dir =
            edit_state.is_dir || (edit_state.is_new_entry() && filename_indicates_dir);
        let is_dir = edit_state.is_dir;
        let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(edit_state.entry_id)?.clone();

        let edit_task;
        let edited_entry_id;
        if is_new_entry {
            self.state.selection = Some(SelectedEntry {
                worktree_id,
                entry_id: NEW_ENTRY_ID,
            });
            let new_path = entry.path.join(&filename);
            if worktree.read(cx).entry_for_path(&new_path).is_some() {
                return None;
            }

            edited_entry_id = NEW_ENTRY_ID;
            edit_task = self.project.update(cx, |project, cx| {
                project.create_entry((worktree_id, new_path), is_dir, cx)
            });
        } else {
            let new_path = if let Some(parent) = entry.path.clone().parent() {
                parent.join(&filename)
            } else {
                filename.clone()
            };
            if let Some(existing) = worktree.read(cx).entry_for_path(&new_path) {
                if existing.id == entry.id && refocus {
                    window.focus(&self.focus_handle, cx);
                }
                return None;
            }
            edited_entry_id = entry.id;
            edit_task = self.project.update(cx, |project, cx| {
                project.rename_entry(entry.id, (worktree_id, new_path).into(), cx)
            });
        };

        if refocus {
            window.focus(&self.focus_handle, cx);
        }
        edit_state.processing_filename = Some(filename);
        cx.notify();

        Some(cx.spawn_in(window, async move |project_panel, cx| {
            let new_entry = edit_task.await;
            project_panel.update(cx, |project_panel, cx| {
                project_panel.state.edit_state = None;
                cx.notify();
            })?;

            match new_entry {
                Err(e) => {
                    project_panel
                        .update_in(cx, |project_panel, window, cx| {
                            project_panel.marked_entries.clear();
                            project_panel.update_visible_entries(None, false, false, window, cx);
                        })
                        .ok();
                    Err(e)?;
                }
                Ok(CreatedEntry::Included(new_entry)) => {
                    project_panel.update_in(cx, |project_panel, window, cx| {
                        if let Some(selection) = &mut project_panel.state.selection
                            && selection.entry_id == edited_entry_id
                        {
                            selection.worktree_id = worktree_id;
                            selection.entry_id = new_entry.id;
                            project_panel.marked_entries.clear();
                            project_panel.expand_to_selection(cx);
                        }
                        project_panel.update_visible_entries(None, false, false, window, cx);
                        if is_new_entry && !is_dir {
                            let settings = ProjectPanelSettings::get_global(cx);
                            if settings.auto_open.should_open_on_create() {
                                project_panel.open_entry(new_entry.id, true, false, cx);
                            }
                        }
                        cx.notify();
                    })?;
                }
                Ok(CreatedEntry::Excluded { abs_path }) => {
                    if let Some(open_task) = project_panel
                        .update_in(cx, |project_panel, window, cx| {
                            project_panel.marked_entries.clear();
                            project_panel.update_visible_entries(None, false, false, window, cx);

                            if is_dir {
                                project_panel.project.update(cx, |_, cx| {
                                    cx.emit(project::Event::Toast {
                                        notification_id: "excluded-directory".into(),
                                        message: format!(
                                            concat!(
                                                "Created an excluded directory at {:?}.\n",
                                                "Alter `file_scan_exclusions` in the settings ",
                                                "to show it in the panel"
                                            ),
                                            abs_path
                                        ),
                                    })
                                });
                                None
                            } else {
                                project_panel
                                    .workspace
                                    .update(cx, |workspace, cx| {
                                        workspace.open_abs_path(
                                            abs_path,
                                            OpenOptions {
                                                visible: Some(OpenVisible::All),
                                                ..Default::default()
                                            },
                                            window,
                                            cx,
                                        )
                                    })
                                    .ok()
                            }
                        })
                        .ok()
                        .flatten()
                    {
                        let _ = open_task.await?;
                    }
                }
            }
            Ok(())
        }))
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if cx.stop_active_drag(window) {
            self.drag_target_entry.take();
            self.hover_expand_task.take();
            return;
        }

        let previous_edit_state = self.state.edit_state.take();
        self.update_visible_entries(None, false, false, window, cx);
        self.marked_entries.clear();

        if let Some(previously_focused) =
            previous_edit_state.and_then(|edit_state| edit_state.previously_focused)
        {
            self.state.selection = Some(previously_focused);
            self.autoscroll(cx);
        }

        window.focus(&self.focus_handle, cx);
        cx.notify();
    }

    fn open_entry(
        &mut self,
        entry_id: ProjectEntryId,
        focus_opened_item: bool,
        allow_preview: bool,

        cx: &mut Context<Self>,
    ) {
        cx.emit(Event::OpenedEntry {
            entry_id,
            focus_opened_item,
            allow_preview,
        });
    }

    fn split_entry(
        &mut self,
        entry_id: ProjectEntryId,
        allow_preview: bool,
        split_direction: Option<SplitDirection>,

        cx: &mut Context<Self>,
    ) {
        cx.emit(Event::SplitEntry {
            entry_id,
            allow_preview,
            split_direction,
        });
    }

    fn new_file(&mut self, _: &NewFile, window: &mut Window, cx: &mut Context<Self>) {
        self.add_entry(false, window, cx)
    }

    fn new_directory(&mut self, _: &NewDirectory, window: &mut Window, cx: &mut Context<Self>) {
        self.add_entry(true, window, cx)
    }

    fn add_entry(&mut self, is_dir: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some((worktree_id, entry_id)) = self
            .state
            .selection
            .map(|entry| (entry.worktree_id, entry.entry_id))
            .or_else(|| {
                let entry_id = self.state.last_worktree_root_id?;
                let worktree_id = self
                    .project
                    .read(cx)
                    .worktree_for_entry(entry_id, cx)?
                    .read(cx)
                    .id();

                self.state.selection = Some(SelectedEntry {
                    worktree_id,
                    entry_id,
                });

                Some((worktree_id, entry_id))
            })
        else {
            return;
        };

        let directory_id;
        let new_entry_id = self.resolve_entry(entry_id);
        if let Some((worktree, expanded_dir_ids)) = self
            .project
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            .zip(self.state.expanded_dir_ids.get_mut(&worktree_id))
        {
            let worktree = worktree.read(cx);
            if let Some(mut entry) = worktree.entry_for_id(new_entry_id) {
                loop {
                    if entry.is_dir() {
                        if let Err(ix) = expanded_dir_ids.binary_search(&entry.id) {
                            expanded_dir_ids.insert(ix, entry.id);
                        }
                        directory_id = entry.id;
                        break;
                    } else {
                        if let Some(parent_path) = entry.path.parent()
                            && let Some(parent_entry) = worktree.entry_for_path(parent_path)
                        {
                            entry = parent_entry;
                            continue;
                        }
                        return;
                    }
                }
            } else {
                return;
            };
        } else {
            return;
        };

        self.marked_entries.clear();
        self.state.edit_state = Some(EditState {
            worktree_id,
            entry_id: directory_id,
            leaf_entry_id: None,
            is_dir,
            processing_filename: None,
            previously_focused: self.state.selection,
            depth: 0,
            validation_state: ValidationState::None,
        });
        self.filename_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
        self.update_visible_entries(Some((worktree_id, NEW_ENTRY_ID)), true, true, window, cx);
        cx.notify();
    }

    fn unflatten_entry_id(&self, leaf_entry_id: ProjectEntryId) -> ProjectEntryId {
        if let Some(ancestors) = self.state.ancestors.get(&leaf_entry_id) {
            ancestors
                .ancestors
                .get(ancestors.current_ancestor_depth)
                .copied()
                .unwrap_or(leaf_entry_id)
        } else {
            leaf_entry_id
        }
    }

    fn rename_impl(
        &mut self,
        selection: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(SelectedEntry {
            worktree_id,
            entry_id,
        }) = self.state.selection
            && let Some(worktree) = self.project.read(cx).worktree_for_id(worktree_id, cx)
        {
            let sub_entry_id = self.unflatten_entry_id(entry_id);
            if let Some(entry) = worktree.read(cx).entry_for_id(sub_entry_id) {
                #[cfg(target_os = "windows")]
                if Some(entry) == worktree.read(cx).root_entry() {
                    return;
                }

                if Some(entry) == worktree.read(cx).root_entry() {
                    let settings = ProjectPanelSettings::get_global(cx);
                    let visible_worktrees_count =
                        self.project.read(cx).visible_worktrees(cx).count();
                    if settings.hide_root && visible_worktrees_count == 1 {
                        return;
                    }
                }

                self.state.edit_state = Some(EditState {
                    worktree_id,
                    entry_id: sub_entry_id,
                    leaf_entry_id: Some(entry_id),
                    is_dir: entry.is_dir(),
                    processing_filename: None,
                    previously_focused: None,
                    depth: 0,
                    validation_state: ValidationState::None,
                });
                let file_name = entry.path.file_name().unwrap_or_default().to_string();
                let selection = selection.unwrap_or_else(|| {
                    let file_stem = entry.path.file_stem().map(|s| s.to_string());
                    let selection_end =
                        file_stem.map_or(file_name.len(), |file_stem| file_stem.len());
                    0..selection_end
                });
                self.filename_editor.update(cx, |editor, cx| {
                    editor.set_text(file_name, window, cx);
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_ranges([
                            MultiBufferOffset(selection.start)..MultiBufferOffset(selection.end)
                        ])
                    });
                });
                self.update_visible_entries(None, true, true, window, cx);
                cx.notify();
            }
        }
    }

    fn rename(&mut self, _: &Rename, window: &mut Window, cx: &mut Context<Self>) {
        self.rename_impl(None, window, cx);
    }

    fn trash(&mut self, action: &Trash, window: &mut Window, cx: &mut Context<Self>) {
        self.remove(true, action.skip_prompt, window, cx);
    }

    fn delete(&mut self, action: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        self.remove(false, action.skip_prompt, window, cx);
    }

    fn restore_file(
        &mut self,
        action: &git::RestoreFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let selection = self.state.selection?;
            let project = self.project.read(cx);

            let (_worktree, entry) = self.selected_sub_entry(cx)?;
            if entry.is_dir() {
                return None;
            }

            let project_path = project.path_for_entry(selection.entry_id, cx)?;

            let git_store = project.git_store();
            let (repository, repo_path) = git_store
                .read(cx)
                .repository_and_path_for_project_path(&project_path, cx)?;

            let snapshot = repository.read(cx).snapshot();
            let status = snapshot.status_for_path(&repo_path)?;
            if !status.status.is_modified() && !status.status.is_deleted() {
                return None;
            }

            let file_name = entry.path.file_name()?.to_string();

            let answer = if !action.skip_prompt {
                let prompt = format!("Discard changes to {}?", file_name);
                Some(window.prompt(PromptLevel::Info, &prompt, None, &["Restore", "Cancel"], cx))
            } else {
                None
            };

            cx.spawn_in(window, async move |panel, cx| {
                if let Some(answer) = answer
                    && answer.await != Ok(0)
                {
                    return anyhow::Ok(());
                }

                let task = panel.update(cx, |_panel, cx| {
                    repository.update(cx, |repo, cx| {
                        repo.checkout_files("HEAD", vec![repo_path], cx)
                    })
                })?;

                if let Err(e) = task.await {
                    panel
                        .update(cx, |panel, cx| {
                            let message = format!("Failed to restore {}: {}", file_name, e);
                            let toast = StatusToast::new(message, cx, |this, _| {
                                this.icon(ToastIcon::new(IconName::XCircle).color(Color::Error))
                                    .dismiss_button(true)
                            });
                            panel
                                .workspace
                                .update(cx, |workspace, cx| {
                                    workspace.toggle_status_toast(toast, cx);
                                })
                                .ok();
                        })
                        .ok();
                }

                panel
                    .update(cx, |panel, cx| {
                        panel.project.update(cx, |project, cx| {
                            if let Some(buffer_id) = project
                                .buffer_store()
                                .read(cx)
                                .buffer_id_for_project_path(&project_path)
                            {
                                if let Some(buffer) = project.buffer_for_id(*buffer_id, cx) {
                                    buffer.update(cx, |buffer, cx| {
                                        let _ = buffer.reload(cx);
                                    });
                                }
                            }
                        })
                    })
                    .ok();

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            Some(())
        });
    }

    fn remove(
        &mut self,
        trash: bool,
        skip_prompt: bool,
        window: &mut Window,
        cx: &mut Context<ProjectPanel>,
    ) {
        maybe!({
            let items_to_delete = self.disjoint_entries(cx);
            if items_to_delete.is_empty() {
                return None;
            }
            let project = self.project.read(cx);

            let mut dirty_buffers = 0;
            let file_paths = items_to_delete
                .iter()
                .filter_map(|selection| {
                    let project_path = project.path_for_entry(selection.entry_id, cx)?;
                    dirty_buffers +=
                        project.dirty_buffers(cx).any(|path| path == project_path) as usize;
                    Some((
                        selection.entry_id,
                        project_path.path.file_name()?.to_string(),
                    ))
                })
                .collect::<Vec<_>>();
            if file_paths.is_empty() {
                return None;
            }
            let answer = if !skip_prompt {
                let operation = if trash { "Trash" } else { "Delete" };
                let prompt = match file_paths.first() {
                    Some((_, path)) if file_paths.len() == 1 => {
                        let unsaved_warning = if dirty_buffers > 0 {
                            "\n\nIt has unsaved changes, which will be lost."
                        } else {
                            ""
                        };

                        format!("{operation} {path}?{unsaved_warning}")
                    }
                    _ => {
                        const CUTOFF_POINT: usize = 10;
                        let names = if file_paths.len() > CUTOFF_POINT {
                            let truncated_path_counts = file_paths.len() - CUTOFF_POINT;
                            let mut paths = file_paths
                                .iter()
                                .map(|(_, path)| path.clone())
                                .take(CUTOFF_POINT)
                                .collect::<Vec<_>>();
                            paths.truncate(CUTOFF_POINT);
                            if truncated_path_counts == 1 {
                                paths.push(".. 1 file not shown".into());
                            } else {
                                paths.push(format!(".. {} files not shown", truncated_path_counts));
                            }
                            paths
                        } else {
                            file_paths.iter().map(|(_, path)| path.clone()).collect()
                        };
                        let unsaved_warning = if dirty_buffers == 0 {
                            String::new()
                        } else if dirty_buffers == 1 {
                            "\n\n1 of these has unsaved changes, which will be lost.".to_string()
                        } else {
                            format!(
                                "\n\n{dirty_buffers} of these have unsaved changes, which will be lost."
                            )
                        };

                        format!(
                            "Do you want to {} the following {} files?\n{}{unsaved_warning}",
                            operation.to_lowercase(),
                            file_paths.len(),
                            names.join("\n")
                        )
                    }
                };
                Some(window.prompt(PromptLevel::Info, &prompt, None, &[operation, "Cancel"], cx))
            } else {
                None
            };
            let next_selection = self.find_next_selection_after_deletion(items_to_delete, cx);
            cx.spawn_in(window, async move |panel, cx| {
                if let Some(answer) = answer
                    && answer.await != Ok(0)
                {
                    return anyhow::Ok(());
                }
                for (entry_id, _) in file_paths {
                    panel
                        .update(cx, |panel, cx| {
                            panel
                                .project
                                .update(cx, |project, cx| project.delete_entry(entry_id, trash, cx))
                                .context("no such entry")
                        })??
                        .await?;
                }
                panel.update_in(cx, |panel, window, cx| {
                    if let Some(next_selection) = next_selection {
                        panel.update_visible_entries(
                            Some((next_selection.worktree_id, next_selection.entry_id)),
                            false,
                            true,
                            window,
                            cx,
                        );
                    } else {
                        panel.select_last(&SelectLast {}, window, cx);
                    }
                })?;
                Ok(())
            })
            .detach_and_log_err(cx);
            Some(())
        });
    }

    fn find_next_selection_after_deletion(
        &self,
        sanitized_entries: BTreeSet<SelectedEntry>,
        cx: &mut Context<Self>,
    ) -> Option<SelectedEntry> {
        if sanitized_entries.is_empty() {
            return None;
        }
        let project = self.project.read(cx);
        let (worktree_id, worktree) = sanitized_entries
            .iter()
            .map(|entry| entry.worktree_id)
            .filter_map(|id| project.worktree_for_id(id, cx).map(|w| (id, w.read(cx))))
            .max_by(|(_, a), (_, b)| a.root_name().cmp(b.root_name()))?;
        let git_store = project.git_store().read(cx);

        let marked_entries_in_worktree = sanitized_entries
            .iter()
            .filter(|e| e.worktree_id == worktree_id)
            .collect::<HashSet<_>>();
        let latest_entry = marked_entries_in_worktree
            .iter()
            .max_by(|a, b| {
                match (
                    worktree.entry_for_id(a.entry_id),
                    worktree.entry_for_id(b.entry_id),
                ) {
                    (Some(a), Some(b)) => compare_paths(
                        (a.path.as_std_path(), a.is_file()),
                        (b.path.as_std_path(), b.is_file()),
                    ),
                    _ => cmp::Ordering::Equal,
                }
            })
            .and_then(|e| worktree.entry_for_id(e.entry_id))?;

        let parent_path = latest_entry.path.parent()?;
        let parent_entry = worktree.entry_for_path(parent_path)?;

        // Remove all siblings that are being deleted except the last marked entry
        let repo_snapshots = git_store.repo_snapshots(cx);
        let worktree_snapshot = worktree.snapshot();
        let hide_gitignore = ProjectPanelSettings::get_global(cx).hide_gitignore;
        let mut siblings: Vec<_> =
            ChildEntriesGitIter::new(&repo_snapshots, &worktree_snapshot, parent_path)
                .filter(|sibling| {
                    (sibling.id == latest_entry.id)
                        || (!marked_entries_in_worktree.contains(&&SelectedEntry {
                            worktree_id,
                            entry_id: sibling.id,
                        }) && (!hide_gitignore || !sibling.is_ignored))
                })
                .map(|entry| entry.to_owned())
                .collect();

        let mode = ProjectPanelSettings::get_global(cx).sort_mode;
        sort_worktree_entries_with_mode(&mut siblings, mode);
        let sibling_entry_index = siblings
            .iter()
            .position(|sibling| sibling.id == latest_entry.id)?;

        if let Some(next_sibling) = sibling_entry_index
            .checked_add(1)
            .and_then(|i| siblings.get(i))
        {
            return Some(SelectedEntry {
                worktree_id,
                entry_id: next_sibling.id,
            });
        }
        if let Some(prev_sibling) = sibling_entry_index
            .checked_sub(1)
            .and_then(|i| siblings.get(i))
        {
            return Some(SelectedEntry {
                worktree_id,
                entry_id: prev_sibling.id,
            });
        }
        // No neighbour sibling found, fall back to parent
        Some(SelectedEntry {
            worktree_id,
            entry_id: parent_entry.id,
        })
    }

    fn unfold_directory(
        &mut self,
        _: &UnfoldDirectory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.state.unfolded_dir_ids.insert(entry.id);

            let snapshot = worktree.snapshot();
            let mut parent_path = entry.path.parent();
            while let Some(path) = parent_path {
                if let Some(parent_entry) = worktree.entry_for_path(path) {
                    let mut children_iter = snapshot.child_entries(path);

                    if children_iter.by_ref().take(2).count() > 1 {
                        break;
                    }

                    self.state.unfolded_dir_ids.insert(parent_entry.id);
                    parent_path = path.parent();
                } else {
                    break;
                }
            }

            self.update_visible_entries(None, false, true, window, cx);
            cx.notify();
        }
    }

    fn fold_directory(&mut self, _: &FoldDirectory, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.state.unfolded_dir_ids.remove(&entry.id);

            let snapshot = worktree.snapshot();
            let mut path = &*entry.path;
            loop {
                let mut child_entries_iter = snapshot.child_entries(path);
                if let Some(child) = child_entries_iter.next() {
                    if child_entries_iter.next().is_none() && child.is_dir() {
                        self.state.unfolded_dir_ids.remove(&child.id);
                        path = &*child.path;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            self.update_visible_entries(None, false, true, window, cx);
            cx.notify();
        }
    }

    fn scroll_up(&mut self, _: &ScrollUp, window: &mut Window, cx: &mut Context<Self>) {
        for _ in 0..self.rendered_entries_len / 2 {
            window.dispatch_action(SelectPrevious.boxed_clone(), cx);
        }
    }

    fn scroll_down(&mut self, _: &ScrollDown, window: &mut Window, cx: &mut Context<Self>) {
        for _ in 0..self.rendered_entries_len / 2 {
            window.dispatch_action(SelectNext.boxed_clone(), cx);
        }
    }

    fn scroll_cursor_center(
        &mut self,
        _: &ScrollCursorCenter,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((_, _, index)) = self
            .state
            .selection
            .and_then(|s| self.index_for_selection(s))
        {
            self.scroll_handle
                .scroll_to_item_strict(index, ScrollStrategy::Center);
            cx.notify();
        }
    }

    fn scroll_cursor_top(&mut self, _: &ScrollCursorTop, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((_, _, index)) = self
            .state
            .selection
            .and_then(|s| self.index_for_selection(s))
        {
            self.scroll_handle
                .scroll_to_item_strict(index, ScrollStrategy::Top);
            cx.notify();
        }
    }

    fn scroll_cursor_bottom(
        &mut self,
        _: &ScrollCursorBottom,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((_, _, index)) = self
            .state
            .selection
            .and_then(|s| self.index_for_selection(s))
        {
            self.scroll_handle
                .scroll_to_item_strict(index, ScrollStrategy::Bottom);
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(edit_state) = &self.state.edit_state
            && edit_state.processing_filename.is_none()
        {
            self.filename_editor.update(cx, |editor, cx| {
                editor.move_to_end_of_line(
                    &editor::actions::MoveToEndOfLine {
                        stop_at_soft_wraps: false,
                    },
                    window,
                    cx,
                );
            });
            return;
        }
        if let Some(selection) = self.state.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if let Some(worktree_entries) = self
                .state
                .visible_entries
                .get(worktree_ix)
                .map(|v| &v.entries)
            {
                if entry_ix + 1 < worktree_entries.len() {
                    entry_ix += 1;
                } else {
                    worktree_ix += 1;
                    entry_ix = 0;
                }
            }

            if let Some(VisibleEntriesForWorktree {
                worktree_id,
                entries,
                ..
            }) = self.state.visible_entries.get(worktree_ix)
                && let Some(entry) = entries.get(entry_ix)
            {
                let selection = SelectedEntry {
                    worktree_id: *worktree_id,
                    entry_id: entry.id,
                };
                self.state.selection = Some(selection);
                if window.modifiers().shift {
                    self.marked_entries.push(selection);
                }

                self.autoscroll(cx);
                cx.notify();
            }
        } else {
            self.select_first(&SelectFirst {}, window, cx);
        }
    }

    fn select_prev_diagnostic(
        &mut self,
        action: &SelectPrevDiagnostic,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.state.selection.as_ref(),
            true,
            |entry, worktree_id| {
                self.state.selection.is_none_or(|selection| {
                    if selection.worktree_id == worktree_id {
                        selection.entry_id != entry.id
                    } else {
                        true
                    }
                }) && entry.is_file()
                    && self
                        .diagnostics
                        .get(&(worktree_id, entry.path.clone()))
                        .is_some_and(|severity| action.severity.matches(*severity))
            },
            cx,
        );

        if let Some(selection) = selection {
            self.state.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(
                Some((selection.worktree_id, selection.entry_id)),
                false,
                true,
                window,
                cx,
            );
            cx.notify();
        }
    }

    fn select_next_diagnostic(
        &mut self,
        action: &SelectNextDiagnostic,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.state.selection.as_ref(),
            false,
            |entry, worktree_id| {
                self.state.selection.is_none_or(|selection| {
                    if selection.worktree_id == worktree_id {
                        selection.entry_id != entry.id
                    } else {
                        true
                    }
                }) && entry.is_file()
                    && self
                        .diagnostics
                        .get(&(worktree_id, entry.path.clone()))
                        .is_some_and(|severity| action.severity.matches(*severity))
            },
            cx,
        );

        if let Some(selection) = selection {
            self.state.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(
                Some((selection.worktree_id, selection.entry_id)),
                false,
                true,
                window,
                cx,
            );
            cx.notify();
        }
    }

    fn select_prev_git_entry(
        &mut self,
        _: &SelectPrevGitEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.state.selection.as_ref(),
            true,
            |entry, worktree_id| {
                (self.state.selection.is_none()
                    || self.state.selection.is_some_and(|selection| {
                        if selection.worktree_id == worktree_id {
                            selection.entry_id != entry.id
                        } else {
                            true
                        }
                    }))
                    && entry.is_file()
                    && entry.git_summary.index.modified + entry.git_summary.worktree.modified > 0
            },
            cx,
        );

        if let Some(selection) = selection {
            self.state.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(
                Some((selection.worktree_id, selection.entry_id)),
                false,
                true,
                window,
                cx,
            );
            cx.notify();
        }
    }

    fn select_prev_directory(
        &mut self,
        _: &SelectPrevDirectory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_visible_entry(
            self.state.selection.as_ref(),
            true,
            |entry, worktree_id| {
                self.state.selection.is_none_or(|selection| {
                    if selection.worktree_id == worktree_id {
                        selection.entry_id != entry.id
                    } else {
                        true
                    }
                }) && entry.is_dir()
            },
            cx,
        );

        if let Some(selection) = selection {
            self.state.selection = Some(selection);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_next_directory(
        &mut self,
        _: &SelectNextDirectory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_visible_entry(
            self.state.selection.as_ref(),
            false,
            |entry, worktree_id| {
                self.state.selection.is_none_or(|selection| {
                    if selection.worktree_id == worktree_id {
                        selection.entry_id != entry.id
                    } else {
                        true
                    }
                }) && entry.is_dir()
            },
            cx,
        );

        if let Some(selection) = selection {
            self.state.selection = Some(selection);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_next_git_entry(
        &mut self,
        _: &SelectNextGitEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.state.selection.as_ref(),
            false,
            |entry, worktree_id| {
                self.state.selection.is_none_or(|selection| {
                    if selection.worktree_id == worktree_id {
                        selection.entry_id != entry.id
                    } else {
                        true
                    }
                }) && entry.is_file()
                    && entry.git_summary.index.modified + entry.git_summary.worktree.modified > 0
            },
            cx,
        );

        if let Some(selection) = selection {
            self.state.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(
                Some((selection.worktree_id, selection.entry_id)),
                false,
                true,
                window,
                cx,
            );
            cx.notify();
        }
    }

    fn select_parent(&mut self, _: &SelectParent, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            if let Some(parent) = entry.path.parent() {
                let worktree = worktree.read(cx);
                if let Some(parent_entry) = worktree.entry_for_path(parent) {
                    self.state.selection = Some(SelectedEntry {
                        worktree_id: worktree.id(),
                        entry_id: parent_entry.id,
                    });
                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        } else {
            self.select_first(&SelectFirst {}, window, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(VisibleEntriesForWorktree {
            worktree_id,
            entries,
            ..
        }) = self.state.visible_entries.first()
            && let Some(entry) = entries.first()
        {
            let selection = SelectedEntry {
                worktree_id: *worktree_id,
                entry_id: entry.id,
            };
            self.state.selection = Some(selection);
            if window.modifiers().shift {
                self.marked_entries.push(selection);
            }
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(VisibleEntriesForWorktree {
            worktree_id,
            entries,
            ..
        }) = self.state.visible_entries.last()
        {
            let worktree = self.project.read(cx).worktree_for_id(*worktree_id, cx);
            if let (Some(worktree), Some(entry)) = (worktree, entries.last()) {
                let worktree = worktree.read(cx);
                if let Some(entry) = worktree.entry_for_id(entry.id) {
                    let selection = SelectedEntry {
                        worktree_id: *worktree_id,
                        entry_id: entry.id,
                    };
                    self.state.selection = Some(selection);
                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        }
    }

    fn autoscroll(&mut self, cx: &mut Context<Self>) {
        if let Some((_, _, index)) = self
            .state
            .selection
            .and_then(|s| self.index_for_selection(s))
        {
            self.scroll_handle.scroll_to_item_with_offset(
                index,
                ScrollStrategy::Center,
                self.sticky_items_count,
            );
            cx.notify();
        }
    }

    fn cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        let entries = self.disjoint_entries(cx);
        if !entries.is_empty() {
            self.clipboard = Some(ClipboardEntry::Cut(entries));
            cx.notify();
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        let entries = self.disjoint_entries(cx);
        if !entries.is_empty() {
            self.clipboard = Some(ClipboardEntry::Copied(entries));
            cx.notify();
        }
    }

    fn create_paste_path(
        &self,
        source: &SelectedEntry,
        (worktree, target_entry): (Entity<Worktree>, &Entry),
        cx: &App,
    ) -> Option<(Arc<RelPath>, Option<Range<usize>>)> {
        let mut new_path = target_entry.path.to_rel_path_buf();
        // If we're pasting into a file, or a directory into itself, go up one level.
        if target_entry.is_file() || (target_entry.is_dir() && target_entry.id == source.entry_id) {
            new_path.pop();
        }
        let clipboard_entry_file_name = self
            .project
            .read(cx)
            .path_for_entry(source.entry_id, cx)?
            .path
            .file_name()?
            .to_string();
        new_path.push(RelPath::unix(&clipboard_entry_file_name).unwrap());
        let extension = new_path.extension().map(|s| s.to_string());
        let file_name_without_extension = new_path.file_stem()?.to_string();
        let file_name_len = file_name_without_extension.len();
        let mut disambiguation_range = None;
        let mut ix = 0;
        {
            let worktree = worktree.read(cx);
            while worktree.entry_for_path(&new_path).is_some() {
                new_path.pop();

                let mut new_file_name = file_name_without_extension.to_string();

                let disambiguation = " copy";
                let mut disambiguation_len = disambiguation.len();

                new_file_name.push_str(disambiguation);

                if ix > 0 {
                    let extra_disambiguation = format!(" {}", ix);
                    disambiguation_len += extra_disambiguation.len();
                    new_file_name.push_str(&extra_disambiguation);
                }
                if let Some(extension) = extension.as_ref() {
                    new_file_name.push_str(".");
                    new_file_name.push_str(extension);
                }

                new_path.push(RelPath::unix(&new_file_name).unwrap());

                disambiguation_range = Some(file_name_len..(file_name_len + disambiguation_len));
                ix += 1;
            }
        }
        Some((new_path.as_rel_path().into(), disambiguation_range))
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let (worktree, entry) = self.selected_entry_handle(cx)?;
            let entry = entry.clone();
            let worktree_id = worktree.read(cx).id();
            let clipboard_entries = self
                .clipboard
                .as_ref()
                .filter(|clipboard| !clipboard.items().is_empty())?;

            enum PasteTask {
                Rename(Task<Result<CreatedEntry>>),
                Copy(Task<Result<Option<Entry>>>),
            }

            let mut paste_tasks = Vec::new();
            let mut disambiguation_range = None;
            let clip_is_cut = clipboard_entries.is_cut();
            for clipboard_entry in clipboard_entries.items() {
                let (new_path, new_disambiguation_range) =
                    self.create_paste_path(clipboard_entry, self.selected_sub_entry(cx)?, cx)?;
                let clip_entry_id = clipboard_entry.entry_id;
                let task = if clipboard_entries.is_cut() {
                    let task = self.project.update(cx, |project, cx| {
                        project.rename_entry(clip_entry_id, (worktree_id, new_path).into(), cx)
                    });
                    PasteTask::Rename(task)
                } else {
                    let task = self.project.update(cx, |project, cx| {
                        project.copy_entry(clip_entry_id, (worktree_id, new_path).into(), cx)
                    });
                    PasteTask::Copy(task)
                };
                paste_tasks.push(task);
                disambiguation_range = new_disambiguation_range.or(disambiguation_range);
            }

            let item_count = paste_tasks.len();

            cx.spawn_in(window, async move |project_panel, cx| {
                let mut last_succeed = None;
                for task in paste_tasks {
                    match task {
                        PasteTask::Rename(task) => {
                            if let Some(CreatedEntry::Included(entry)) =
                                task.await.notify_async_err(cx)
                            {
                                last_succeed = Some(entry);
                            }
                        }
                        PasteTask::Copy(task) => {
                            if let Some(Some(entry)) = task.await.notify_async_err(cx) {
                                last_succeed = Some(entry);
                            }
                        }
                    }
                }
                // update selection
                if let Some(entry) = last_succeed {
                    project_panel
                        .update_in(cx, |project_panel, window, cx| {
                            project_panel.state.selection = Some(SelectedEntry {
                                worktree_id,
                                entry_id: entry.id,
                            });

                            if item_count == 1 {
                                // open entry if not dir, setting is enabled, and only focus if rename is not pending
                                if !entry.is_dir() {
                                    let settings = ProjectPanelSettings::get_global(cx);
                                    if settings.auto_open.should_open_on_paste() {
                                        project_panel.open_entry(
                                            entry.id,
                                            disambiguation_range.is_none(),
                                            false,
                                            cx,
                                        );
                                    }
                                }

                                // if only one entry was pasted and it was disambiguated, open the rename editor
                                if disambiguation_range.is_some() {
                                    cx.defer_in(window, |this, window, cx| {
                                        this.rename_impl(disambiguation_range, window, cx);
                                    });
                                }
                            }
                        })
                        .ok();
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            if clip_is_cut {
                // Convert the clipboard cut entry to a copy entry after the first paste.
                self.clipboard = self.clipboard.take().map(ClipboardEntry::into_copy_entry);
            }

            self.expand_entry(worktree_id, entry.id, cx);
            Some(())
        });
    }

    fn duplicate(&mut self, _: &Duplicate, window: &mut Window, cx: &mut Context<Self>) {
        self.copy(&Copy {}, window, cx);
        self.paste(&Paste {}, window, cx);
    }

    fn copy_path(
        &mut self,
        _: &zed_actions::workspace::CopyPath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let abs_file_paths = {
            let project = self.project.read(cx);
            self.effective_entries()
                .into_iter()
                .filter_map(|entry| {
                    let entry_path = project.path_for_entry(entry.entry_id, cx)?.path;
                    Some(
                        project
                            .worktree_for_id(entry.worktree_id, cx)?
                            .read(cx)
                            .absolutize(&entry_path)
                            .to_string_lossy()
                            .to_string(),
                    )
                })
                .collect::<Vec<_>>()
        };
        if !abs_file_paths.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(abs_file_paths.join("\n")));
        }
    }

    fn copy_relative_path(
        &mut self,
        _: &zed_actions::workspace::CopyRelativePath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let path_style = self.project.read(cx).path_style(cx);
        let file_paths = {
            let project = self.project.read(cx);
            self.effective_entries()
                .into_iter()
                .filter_map(|entry| {
                    Some(
                        project
                            .path_for_entry(entry.entry_id, cx)?
                            .path
                            .display(path_style)
                            .into_owned(),
                    )
                })
                .collect::<Vec<_>>()
        };
        if !file_paths.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(file_paths.join("\n")));
        }
    }

    fn reveal_in_finder(
        &mut self,
        _: &RevealInFileManager,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            cx.reveal_path(&worktree.read(cx).absolutize(&entry.path));
        }
    }

    fn remove_from_project(
        &mut self,
        _: &RemoveFromProject,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for entry in self.effective_entries().iter() {
            let worktree_id = entry.worktree_id;
            self.project
                .update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        }
    }

    fn file_abs_paths_to_diff(&self, cx: &Context<Self>) -> Option<(PathBuf, PathBuf)> {
        let mut selections_abs_path = self
            .marked_entries
            .iter()
            .filter_map(|entry| {
                let project = self.project.read(cx);
                let worktree = project.worktree_for_id(entry.worktree_id, cx)?;
                let entry = worktree.read(cx).entry_for_id(entry.entry_id)?;
                if !entry.is_file() {
                    return None;
                }
                Some(worktree.read(cx).absolutize(&entry.path))
            })
            .rev();

        let last_path = selections_abs_path.next()?;
        let previous_to_last = selections_abs_path.next()?;
        Some((previous_to_last, last_path))
    }

    fn compare_marked_files(
        &mut self,
        _: &CompareMarkedFiles,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_files = self.file_abs_paths_to_diff(cx);
        if let Some((file_path1, file_path2)) = selected_files {
            self.workspace
                .update(cx, |workspace, cx| {
                    FileDiffView::open(file_path1, file_path2, workspace, window, cx)
                        .detach_and_log_err(cx);
                })
                .ok();
        }
    }

    fn open_system(&mut self, _: &OpenWithSystem, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let abs_path = worktree.absolutize(&entry.path);
            cx.open_with_system(&abs_path);
        }
    }

    fn open_in_terminal(
        &mut self,
        _: &OpenInTerminal,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            let abs_path = match &entry.canonical_path {
                Some(canonical_path) => canonical_path.to_path_buf(),
                None => worktree.read(cx).absolutize(&entry.path),
            };

            let working_directory = if entry.is_dir() {
                Some(abs_path)
            } else {
                abs_path.parent().map(|path| path.to_path_buf())
            };
            if let Some(working_directory) = working_directory {
                window.dispatch_action(
                    workspace::OpenTerminal { working_directory }.boxed_clone(),
                    cx,
                )
            }
        }
    }

    pub fn new_search_in_directory(
        &mut self,
        _: &NewSearchInDirectory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            let dir_path = if entry.is_dir() {
                entry.path.clone()
            } else {
                // entry is a file, use its parent directory
                match entry.path.parent() {
                    Some(parent) => Arc::from(parent),
                    None => {
                        // File at root, open search with empty filter
                        self.workspace
                            .update(cx, |workspace, cx| {
                                search::ProjectSearchView::new_search_in_directory(
                                    workspace,
                                    RelPath::empty(),
                                    window,
                                    cx,
                                );
                            })
                            .ok();
                        return;
                    }
                }
            };

            let include_root = self.project.read(cx).visible_worktrees(cx).count() > 1;
            let dir_path = if include_root {
                worktree.read(cx).root_name().join(&dir_path)
            } else {
                dir_path
            };

            self.workspace
                .update(cx, |workspace, cx| {
                    search::ProjectSearchView::new_search_in_directory(
                        workspace, &dir_path, window, cx,
                    );
                })
                .ok();
        }
    }

    fn move_entry(
        &mut self,
        entry_to_move: ProjectEntryId,
        destination: ProjectEntryId,
        destination_is_file: bool,
        cx: &mut Context<Self>,
    ) {
        if self
            .project
            .read(cx)
            .entry_is_worktree_root(entry_to_move, cx)
        {
            self.move_worktree_root(entry_to_move, destination, cx)
        } else {
            self.move_worktree_entry(entry_to_move, destination, destination_is_file, cx)
        }
    }

    fn move_worktree_root(
        &mut self,
        entry_to_move: ProjectEntryId,
        destination: ProjectEntryId,
        cx: &mut Context<Self>,
    ) {
        self.project.update(cx, |project, cx| {
            let Some(worktree_to_move) = project.worktree_for_entry(entry_to_move, cx) else {
                return;
            };
            let Some(destination_worktree) = project.worktree_for_entry(destination, cx) else {
                return;
            };

            let worktree_id = worktree_to_move.read(cx).id();
            let destination_id = destination_worktree.read(cx).id();

            project
                .move_worktree(worktree_id, destination_id, cx)
                .log_err();
        });
    }

    fn move_worktree_entry(
        &mut self,
        entry_to_move: ProjectEntryId,
        destination_entry: ProjectEntryId,
        destination_is_file: bool,
        cx: &mut Context<Self>,
    ) {
        if entry_to_move == destination_entry {
            return;
        }

        let destination_worktree = self.project.update(cx, |project, cx| {
            let source_path = project.path_for_entry(entry_to_move, cx)?;
            let destination_path = project.path_for_entry(destination_entry, cx)?;
            let destination_worktree_id = destination_path.worktree_id;

            let mut destination_path = destination_path.path.as_ref();
            if destination_is_file {
                destination_path = destination_path.parent()?;
            }

            let mut new_path = destination_path.to_rel_path_buf();
            new_path.push(RelPath::unix(source_path.path.file_name()?).unwrap());
            if new_path.as_rel_path() != source_path.path.as_ref() {
                let task = project.rename_entry(
                    entry_to_move,
                    (destination_worktree_id, new_path).into(),
                    cx,
                );
                cx.foreground_executor().spawn(task).detach_and_log_err(cx);
            }

            project.worktree_id_for_entry(destination_entry, cx)
        });

        if let Some(destination_worktree) = destination_worktree {
            self.expand_entry(destination_worktree, destination_entry, cx);
        }
    }

    fn index_for_selection(&self, selection: SelectedEntry) -> Option<(usize, usize, usize)> {
        self.index_for_entry(selection.entry_id, selection.worktree_id)
    }

    fn disjoint_entries(&self, cx: &App) -> BTreeSet<SelectedEntry> {
        let marked_entries = self.effective_entries();
        let mut sanitized_entries = BTreeSet::new();
        if marked_entries.is_empty() {
            return sanitized_entries;
        }

        let project = self.project.read(cx);
        let marked_entries_by_worktree: HashMap<WorktreeId, Vec<SelectedEntry>> = marked_entries
            .into_iter()
            .filter(|entry| !project.entry_is_worktree_root(entry.entry_id, cx))
            .fold(HashMap::default(), |mut map, entry| {
                map.entry(entry.worktree_id).or_default().push(entry);
                map
            });

        for (worktree_id, marked_entries) in marked_entries_by_worktree {
            if let Some(worktree) = project.worktree_for_id(worktree_id, cx) {
                let worktree = worktree.read(cx);
                let marked_dir_paths = marked_entries
                    .iter()
                    .filter_map(|entry| {
                        worktree.entry_for_id(entry.entry_id).and_then(|entry| {
                            if entry.is_dir() {
                                Some(entry.path.as_ref())
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<BTreeSet<_>>();

                sanitized_entries.extend(marked_entries.into_iter().filter(|entry| {
                    let Some(entry_info) = worktree.entry_for_id(entry.entry_id) else {
                        return false;
                    };
                    let entry_path = entry_info.path.as_ref();
                    let inside_marked_dir = marked_dir_paths.iter().any(|&marked_dir_path| {
                        entry_path != marked_dir_path && entry_path.starts_with(marked_dir_path)
                    });
                    !inside_marked_dir
                }));
            }
        }

        sanitized_entries
    }

    fn effective_entries(&self) -> BTreeSet<SelectedEntry> {
        if let Some(selection) = self.state.selection {
            let selection = SelectedEntry {
                entry_id: self.resolve_entry(selection.entry_id),
                worktree_id: selection.worktree_id,
            };

            // Default to using just the selected item when nothing is marked.
            if self.marked_entries.is_empty() {
                return BTreeSet::from([selection]);
            }

            // Allow operating on the selected item even when something else is marked,
            // making it easier to perform one-off actions without clearing a mark.
            if self.marked_entries.len() == 1 && !self.marked_entries.contains(&selection) {
                return BTreeSet::from([selection]);
            }
        }

        // Return only marked entries since we've already handled special cases where
        // only selection should take precedence. At this point, marked entries may or
        // may not include the current selection, which is intentional.
        self.marked_entries
            .iter()
            .map(|entry| SelectedEntry {
                entry_id: self.resolve_entry(entry.entry_id),
                worktree_id: entry.worktree_id,
            })
            .collect::<BTreeSet<_>>()
    }

    /// Finds the currently selected subentry for a given leaf entry id. If a given entry
    /// has no ancestors, the project entry ID that's passed in is returned as-is.
    fn resolve_entry(&self, id: ProjectEntryId) -> ProjectEntryId {
        self.state
            .ancestors
            .get(&id)
            .and_then(|ancestors| ancestors.active_ancestor())
            .unwrap_or(id)
    }

    pub fn selected_entry<'a>(&self, cx: &'a App) -> Option<(&'a Worktree, &'a project::Entry)> {
        let (worktree, entry) = self.selected_entry_handle(cx)?;
        Some((worktree.read(cx), entry))
    }

    /// Compared to selected_entry, this function resolves to the currently
    /// selected subentry if dir auto-folding is enabled.
    fn selected_sub_entry<'a>(
        &self,
        cx: &'a App,
    ) -> Option<(Entity<Worktree>, &'a project::Entry)> {
        let (worktree, mut entry) = self.selected_entry_handle(cx)?;

        let resolved_id = self.resolve_entry(entry.id);
        if resolved_id != entry.id {
            let worktree = worktree.read(cx);
            entry = worktree.entry_for_id(resolved_id)?;
        }
        Some((worktree, entry))
    }
    fn selected_entry_handle<'a>(
        &self,
        cx: &'a App,
    ) -> Option<(Entity<Worktree>, &'a project::Entry)> {
        let selection = self.state.selection?;
        let project = self.project.read(cx);
        let worktree = project.worktree_for_id(selection.worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(selection.entry_id)?;
        Some((worktree, entry))
    }

    fn expand_to_selection(&mut self, cx: &mut Context<Self>) -> Option<()> {
        let (worktree, entry) = self.selected_entry(cx)?;
        let expanded_dir_ids = self
            .state
            .expanded_dir_ids
            .entry(worktree.id())
            .or_default();

        for path in entry.path.ancestors() {
            let Some(entry) = worktree.entry_for_path(path) else {
                continue;
            };
            if entry.is_dir()
                && let Err(idx) = expanded_dir_ids.binary_search(&entry.id)
            {
                expanded_dir_ids.insert(idx, entry.id);
            }
        }

        Some(())
    }

    fn create_new_git_entry(
        parent_entry: &Entry,
        git_summary: GitSummary,
        new_entry_kind: EntryKind,
    ) -> GitEntry {
        GitEntry {
            entry: Entry {
                id: NEW_ENTRY_ID,
                kind: new_entry_kind,
                path: parent_entry.path.join(RelPath::unix("\0").unwrap()),
                inode: 0,
                mtime: parent_entry.mtime,
                size: parent_entry.size,
                is_ignored: parent_entry.is_ignored,
                is_hidden: parent_entry.is_hidden,
                is_external: false,
                is_private: false,
                is_always_included: parent_entry.is_always_included,
                canonical_path: parent_entry.canonical_path.clone(),
                char_bag: parent_entry.char_bag,
                is_fifo: parent_entry.is_fifo,
            },
            git_summary,
        }
    }

    fn update_visible_entries(
        &mut self,
        new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        focus_filename_editor: bool,
        autoscroll: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let now = Instant::now();
        let settings = ProjectPanelSettings::get_global(cx);
        let auto_collapse_dirs = settings.auto_fold_dirs;
        let hide_gitignore = settings.hide_gitignore;
        let sort_mode = settings.sort_mode;
        let project = self.project.read(cx);
        let repo_snapshots = project.git_store().read(cx).repo_snapshots(cx);

        let old_ancestors = self.state.ancestors.clone();
        let mut new_state = State::derive(&self.state);
        new_state.last_worktree_root_id = project
            .visible_worktrees(cx)
            .next_back()
            .and_then(|worktree| worktree.read(cx).root_entry())
            .map(|entry| entry.id);
        let mut max_width_item = None;

        let visible_worktrees: Vec<_> = project
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).snapshot())
            .collect();
        let hide_root = settings.hide_root && visible_worktrees.len() == 1;
        let hide_hidden = settings.hide_hidden;

        let visible_entries_task = cx.spawn_in(window, async move |this, cx| {
            let new_state = cx
                .background_spawn(async move {
                    for worktree_snapshot in visible_worktrees {
                        let worktree_id = worktree_snapshot.id();

                        let expanded_dir_ids = match new_state.expanded_dir_ids.entry(worktree_id) {
                            hash_map::Entry::Occupied(e) => e.into_mut(),
                            hash_map::Entry::Vacant(e) => {
                                // The first time a worktree's root entry becomes available,
                                // mark that root entry as expanded.
                                if let Some(entry) = worktree_snapshot.root_entry() {
                                    e.insert(vec![entry.id]).as_slice()
                                } else {
                                    &[]
                                }
                            }
                        };

                        let mut new_entry_parent_id = None;
                        let mut new_entry_kind = EntryKind::Dir;
                        if let Some(edit_state) = &new_state.edit_state
                            && edit_state.worktree_id == worktree_id
                            && edit_state.is_new_entry()
                        {
                            new_entry_parent_id = Some(edit_state.entry_id);
                            new_entry_kind = if edit_state.is_dir {
                                EntryKind::Dir
                            } else {
                                EntryKind::File
                            };
                        }

                        let mut visible_worktree_entries = Vec::new();
                        let mut entry_iter =
                            GitTraversal::new(&repo_snapshots, worktree_snapshot.entries(true, 0));
                        let mut auto_folded_ancestors = vec![];
                        let worktree_abs_path = worktree_snapshot.abs_path();
                        while let Some(entry) = entry_iter.entry() {
                            if hide_root && Some(entry.entry) == worktree_snapshot.root_entry() {
                                if new_entry_parent_id == Some(entry.id) {
                                    visible_worktree_entries.push(Self::create_new_git_entry(
                                        entry.entry,
                                        entry.git_summary,
                                        new_entry_kind,
                                    ));
                                    new_entry_parent_id = None;
                                }
                                entry_iter.advance();
                                continue;
                            }
                            if auto_collapse_dirs && entry.kind.is_dir() {
                                auto_folded_ancestors.push(entry.id);
                                if !new_state.unfolded_dir_ids.contains(&entry.id)
                                    && let Some(root_path) = worktree_snapshot.root_entry()
                                {
                                    let mut child_entries =
                                        worktree_snapshot.child_entries(&entry.path);
                                    if let Some(child) = child_entries.next()
                                        && entry.path != root_path.path
                                        && child_entries.next().is_none()
                                        && child.kind.is_dir()
                                    {
                                        entry_iter.advance();

                                        continue;
                                    }
                                }
                                let depth = old_ancestors
                                    .get(&entry.id)
                                    .map(|ancestor| ancestor.current_ancestor_depth)
                                    .unwrap_or_default()
                                    .min(auto_folded_ancestors.len());
                                if let Some(edit_state) = &mut new_state.edit_state
                                    && edit_state.entry_id == entry.id
                                {
                                    edit_state.depth = depth;
                                }
                                let mut ancestors = std::mem::take(&mut auto_folded_ancestors);
                                if ancestors.len() > 1 {
                                    ancestors.reverse();
                                    new_state.ancestors.insert(
                                        entry.id,
                                        FoldedAncestors {
                                            current_ancestor_depth: depth,
                                            ancestors,
                                        },
                                    );
                                }
                            }
                            auto_folded_ancestors.clear();
                            if (!hide_gitignore || !entry.is_ignored)
                                && (!hide_hidden || !entry.is_hidden)
                            {
                                visible_worktree_entries.push(entry.to_owned());
                            }
                            let precedes_new_entry = if let Some(new_entry_id) = new_entry_parent_id
                            {
                                entry.id == new_entry_id || {
                                    new_state.ancestors.get(&entry.id).is_some_and(|entries| {
                                        entries.ancestors.contains(&new_entry_id)
                                    })
                                }
                            } else {
                                false
                            };
                            if precedes_new_entry
                                && (!hide_gitignore || !entry.is_ignored)
                                && (!hide_hidden || !entry.is_hidden)
                            {
                                visible_worktree_entries.push(Self::create_new_git_entry(
                                    entry.entry,
                                    entry.git_summary,
                                    new_entry_kind,
                                ));
                            }

                            let (depth, chars) = if Some(entry.entry)
                                == worktree_snapshot.root_entry()
                            {
                                let Some(path_name) = worktree_abs_path.file_name() else {
                                    entry_iter.advance();
                                    continue;
                                };
                                let depth = 0;
                                (depth, path_name.to_string_lossy().chars().count())
                            } else if entry.is_file() {
                                let Some(path_name) = entry
                                    .path
                                    .file_name()
                                    .with_context(|| {
                                        format!("Non-root entry has no file name: {entry:?}")
                                    })
                                    .log_err()
                                else {
                                    continue;
                                };
                                let depth = entry.path.ancestors().count() - 1;
                                (depth, path_name.chars().count())
                            } else {
                                let path = new_state
                                    .ancestors
                                    .get(&entry.id)
                                    .and_then(|ancestors| {
                                        let outermost_ancestor = ancestors.ancestors.last()?;
                                        let root_folded_entry = worktree_snapshot
                                            .entry_for_id(*outermost_ancestor)?
                                            .path
                                            .as_ref();
                                        entry.path.strip_prefix(root_folded_entry).ok().and_then(
                                            |suffix| {
                                                Some(
                                                    RelPath::unix(root_folded_entry.file_name()?)
                                                        .unwrap()
                                                        .join(suffix),
                                                )
                                            },
                                        )
                                    })
                                    .or_else(|| {
                                        entry.path.file_name().map(|file_name| {
                                            RelPath::unix(file_name).unwrap().into()
                                        })
                                    })
                                    .unwrap_or_else(|| entry.path.clone());
                                let depth = path.components().count();
                                (depth, path.as_unix_str().chars().count())
                            };
                            let width_estimate =
                                item_width_estimate(depth, chars, entry.canonical_path.is_some());

                            match max_width_item.as_mut() {
                                Some((id, worktree_id, width)) => {
                                    if *width < width_estimate {
                                        *id = entry.id;
                                        *worktree_id = worktree_snapshot.id();
                                        *width = width_estimate;
                                    }
                                }
                                None => {
                                    max_width_item =
                                        Some((entry.id, worktree_snapshot.id(), width_estimate))
                                }
                            }

                            if expanded_dir_ids.binary_search(&entry.id).is_err()
                                && entry_iter.advance_to_sibling()
                            {
                                continue;
                            }
                            entry_iter.advance();
                        }

                        par_sort_worktree_entries_with_mode(
                            &mut visible_worktree_entries,
                            sort_mode,
                        );
                        new_state.visible_entries.push(VisibleEntriesForWorktree {
                            worktree_id,
                            entries: visible_worktree_entries,
                            index: OnceCell::new(),
                        })
                    }
                    if let Some((project_entry_id, worktree_id, _)) = max_width_item {
                        let mut visited_worktrees_length = 0;
                        let index = new_state
                            .visible_entries
                            .iter()
                            .find_map(|visible_entries| {
                                if worktree_id == visible_entries.worktree_id {
                                    visible_entries
                                        .entries
                                        .iter()
                                        .position(|entry| entry.id == project_entry_id)
                                } else {
                                    visited_worktrees_length += visible_entries.entries.len();
                                    None
                                }
                            });
                        if let Some(index) = index {
                            new_state.max_width_item_index = Some(visited_worktrees_length + index);
                        }
                    }
                    new_state
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                let current_selection = this.state.selection;
                this.state = new_state;
                if let Some((worktree_id, entry_id)) = new_selected_entry {
                    this.state.selection = Some(SelectedEntry {
                        worktree_id,
                        entry_id,
                    });
                } else {
                    this.state.selection = current_selection;
                }
                let elapsed = now.elapsed();
                if this.last_reported_update.elapsed() > Duration::from_secs(3600) {
                    telemetry::event!(
                        "Project Panel Updated",
                        elapsed_ms = elapsed.as_millis() as u64,
                        worktree_entries = this
                            .state
                            .visible_entries
                            .iter()
                            .map(|worktree| worktree.entries.len())
                            .sum::<usize>(),
                    )
                }
                if this.update_visible_entries_task.focus_filename_editor {
                    this.update_visible_entries_task.focus_filename_editor = false;
                    this.filename_editor.update(cx, |editor, cx| {
                        window.focus(&editor.focus_handle(cx), cx);
                    });
                }
                if this.update_visible_entries_task.autoscroll {
                    this.update_visible_entries_task.autoscroll = false;
                    this.autoscroll(cx);
                }
                cx.notify();
            })
            .ok();
        });

        self.update_visible_entries_task = UpdateVisibleEntriesTask {
            _visible_entries_task: visible_entries_task,
            focus_filename_editor: focus_filename_editor
                || self.update_visible_entries_task.focus_filename_editor,
            autoscroll: autoscroll || self.update_visible_entries_task.autoscroll,
        };
    }

    fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut Context<Self>,
    ) {
        self.project.update(cx, |project, cx| {
            if let Some((worktree, expanded_dir_ids)) = project
                .worktree_for_id(worktree_id, cx)
                .zip(self.state.expanded_dir_ids.get_mut(&worktree_id))
            {
                project.expand_entry(worktree_id, entry_id, cx);
                let worktree = worktree.read(cx);

                if let Some(mut entry) = worktree.entry_for_id(entry_id) {
                    loop {
                        if let Err(ix) = expanded_dir_ids.binary_search(&entry.id) {
                            expanded_dir_ids.insert(ix, entry.id);
                        }

                        if let Some(parent_entry) =
                            entry.path.parent().and_then(|p| worktree.entry_for_path(p))
                        {
                            entry = parent_entry;
                        } else {
                            break;
                        }
                    }
                }
            }
        });
    }

    fn drop_external_files(
        &mut self,
        paths: &[PathBuf],
        entry_id: ProjectEntryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut paths: Vec<Arc<Path>> = paths.iter().map(|path| Arc::from(path.clone())).collect();

        let open_file_after_drop = paths.len() == 1 && paths[0].is_file();

        let Some((target_directory, worktree, fs)) = maybe!({
            let project = self.project.read(cx);
            let fs = project.fs().clone();
            let worktree = project.worktree_for_entry(entry_id, cx)?;
            let entry = worktree.read(cx).entry_for_id(entry_id)?;
            let path = entry.path.clone();
            let target_directory = if entry.is_dir() {
                path
            } else {
                path.parent()?.into()
            };
            Some((target_directory, worktree, fs))
        }) else {
            return;
        };

        let mut paths_to_replace = Vec::new();
        for path in &paths {
            if let Some(name) = path.file_name()
                && let Some(name) = name.to_str()
            {
                let target_path = target_directory.join(RelPath::unix(name).unwrap());
                if worktree.read(cx).entry_for_path(&target_path).is_some() {
                    paths_to_replace.push((name.to_string(), path.clone()));
                }
            }
        }

        cx.spawn_in(window, async move |this, cx| {
            async move {
                for (filename, original_path) in &paths_to_replace {
                    let prompt_message = format!(
                        concat!(
                            "A file or folder with name {} ",
                            "already exists in the destination folder. ",
                            "Do you want to replace it?"
                        ),
                        filename
                    );
                    let answer = cx
                        .update(|window, cx| {
                            window.prompt(
                                PromptLevel::Info,
                                &prompt_message,
                                None,
                                &["Replace", "Cancel"],
                                cx,
                            )
                        })?
                        .await?;

                    if answer == 1
                        && let Some(item_idx) = paths.iter().position(|p| p == original_path)
                    {
                        paths.remove(item_idx);
                    }
                }

                if paths.is_empty() {
                    return Ok(());
                }

                let task = worktree.update(cx, |worktree, cx| {
                    worktree.copy_external_entries(target_directory, paths, fs, cx)
                })?;

                let opened_entries = task
                    .await
                    .with_context(|| "failed to copy external paths")?;
                this.update(cx, |this, cx| {
                    if open_file_after_drop && !opened_entries.is_empty() {
                        let settings = ProjectPanelSettings::get_global(cx);
                        if settings.auto_open.should_open_on_drop() {
                            this.open_entry(opened_entries[0], true, false, cx);
                        }
                    }
                })
            }
            .log_err()
            .await
        })
        .detach();
    }

    fn refresh_drag_cursor_style(
        &self,
        modifiers: &Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(existing_cursor) = cx.active_drag_cursor_style() {
            let new_cursor = if Self::is_copy_modifier_set(modifiers) {
                CursorStyle::DragCopy
            } else {
                CursorStyle::PointingHand
            };
            if existing_cursor != new_cursor {
                cx.set_active_drag_cursor_style(new_cursor, window);
            }
        }
    }

    fn is_copy_modifier_set(modifiers: &Modifiers) -> bool {
        cfg!(target_os = "macos") && modifiers.alt
            || cfg!(not(target_os = "macos")) && modifiers.control
    }

    fn drag_onto(
        &mut self,
        selections: &DraggedSelection,
        target_entry_id: ProjectEntryId,
        is_file: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if Self::is_copy_modifier_set(&window.modifiers()) {
            let _ = maybe!({
                let project = self.project.read(cx);
                let target_worktree = project.worktree_for_entry(target_entry_id, cx)?;
                let worktree_id = target_worktree.read(cx).id();
                let target_entry = target_worktree
                    .read(cx)
                    .entry_for_id(target_entry_id)?
                    .clone();

                let mut copy_tasks = Vec::new();
                let mut disambiguation_range = None;
                for selection in selections.items() {
                    let (new_path, new_disambiguation_range) = self.create_paste_path(
                        selection,
                        (target_worktree.clone(), &target_entry),
                        cx,
                    )?;

                    let task = self.project.update(cx, |project, cx| {
                        project.copy_entry(selection.entry_id, (worktree_id, new_path).into(), cx)
                    });
                    copy_tasks.push(task);
                    disambiguation_range = new_disambiguation_range.or(disambiguation_range);
                }

                let item_count = copy_tasks.len();

                cx.spawn_in(window, async move |project_panel, cx| {
                    let mut last_succeed = None;
                    for task in copy_tasks.into_iter() {
                        if let Some(Some(entry)) = task.await.log_err() {
                            last_succeed = Some(entry.id);
                        }
                    }
                    // update selection
                    if let Some(entry_id) = last_succeed {
                        project_panel
                            .update_in(cx, |project_panel, window, cx| {
                                project_panel.state.selection = Some(SelectedEntry {
                                    worktree_id,
                                    entry_id,
                                });

                                // if only one entry was dragged and it was disambiguated, open the rename editor
                                if item_count == 1 && disambiguation_range.is_some() {
                                    project_panel.rename_impl(disambiguation_range, window, cx);
                                }
                            })
                            .ok();
                    }
                })
                .detach();
                Some(())
            });
        } else {
            for selection in selections.items() {
                self.move_entry(selection.entry_id, target_entry_id, is_file, cx);
            }
        }
    }

    fn index_for_entry(
        &self,
        entry_id: ProjectEntryId,
        worktree_id: WorktreeId,
    ) -> Option<(usize, usize, usize)> {
        let mut total_ix = 0;
        for (worktree_ix, visible) in self.state.visible_entries.iter().enumerate() {
            if worktree_id != visible.worktree_id {
                total_ix += visible.entries.len();
                continue;
            }

            return visible
                .entries
                .iter()
                .enumerate()
                .find(|(_, entry)| entry.id == entry_id)
                .map(|(ix, _)| (worktree_ix, ix, total_ix + ix));
        }
        None
    }

    fn entry_at_index(&self, index: usize) -> Option<(WorktreeId, GitEntryRef<'_>)> {
        let mut offset = 0;
        for worktree in &self.state.visible_entries {
            let current_len = worktree.entries.len();
            if index < offset + current_len {
                return worktree
                    .entries
                    .get(index - offset)
                    .map(|entry| (worktree.worktree_id, entry.to_ref()));
            }
            offset += current_len;
        }
        None
    }

    fn iter_visible_entries(
        &self,
        range: Range<usize>,
        window: &mut Window,
        cx: &mut Context<ProjectPanel>,
        mut callback: impl FnMut(
            &Entry,
            usize,
            &HashSet<Arc<RelPath>>,
            &mut Window,
            &mut Context<ProjectPanel>,
        ),
    ) {
        let mut ix = 0;
        for visible in &self.state.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible.entries.len() <= range.start {
                ix += visible.entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible.entries.len());
            let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
            let entries = visible
                .index
                .get_or_init(|| visible.entries.iter().map(|e| e.path.clone()).collect());
            let base_index = ix + entry_range.start;
            for (i, entry) in visible.entries[entry_range].iter().enumerate() {
                let global_index = base_index + i;
                callback(entry, global_index, entries, window, cx);
            }
            ix = end_ix;
        }
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        window: &mut Window,
        cx: &mut Context<ProjectPanel>,
        mut callback: impl FnMut(ProjectEntryId, EntryDetails, &mut Window, &mut Context<ProjectPanel>),
    ) {
        let mut ix = 0;
        for visible in &self.state.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible.entries.len() <= range.start {
                ix += visible.entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible.entries.len());
            let git_status_setting = {
                let settings = ProjectPanelSettings::get_global(cx);
                settings.git_status
            };
            if let Some(worktree) = self
                .project
                .read(cx)
                .worktree_for_id(visible.worktree_id, cx)
            {
                let snapshot = worktree.read(cx).snapshot();
                let root_name = snapshot.root_name();

                let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
                let entries = visible
                    .index
                    .get_or_init(|| visible.entries.iter().map(|e| e.path.clone()).collect());
                for entry in visible.entries[entry_range].iter() {
                    let status = git_status_setting
                        .then_some(entry.git_summary)
                        .unwrap_or_default();

                    let mut details = self.details_for_entry(
                        entry,
                        visible.worktree_id,
                        root_name,
                        entries,
                        status,
                        None,
                        window,
                        cx,
                    );

                    if let Some(edit_state) = &self.state.edit_state {
                        let is_edited_entry = if edit_state.is_new_entry() {
                            entry.id == NEW_ENTRY_ID
                        } else {
                            entry.id == edit_state.entry_id
                                || self.state.ancestors.get(&entry.id).is_some_and(
                                    |auto_folded_dirs| {
                                        auto_folded_dirs.ancestors.contains(&edit_state.entry_id)
                                    },
                                )
                        };

                        if is_edited_entry {
                            if let Some(processing_filename) = &edit_state.processing_filename {
                                details.is_processing = true;
                                if let Some(ancestors) = edit_state
                                    .leaf_entry_id
                                    .and_then(|entry| self.state.ancestors.get(&entry))
                                {
                                    let position = ancestors.ancestors.iter().position(|entry_id| *entry_id == edit_state.entry_id).expect("Edited sub-entry should be an ancestor of selected leaf entry") + 1;
                                    let all_components = ancestors.ancestors.len();

                                    let prefix_components = all_components - position;
                                    let suffix_components = position.checked_sub(1);
                                    let mut previous_components =
                                        Path::new(&details.filename).components();
                                    let mut new_path = previous_components
                                        .by_ref()
                                        .take(prefix_components)
                                        .collect::<PathBuf>();
                                    if let Some(last_component) =
                                        processing_filename.components().next_back()
                                    {
                                        new_path.push(last_component);
                                        previous_components.next();
                                    }

                                    if suffix_components.is_some() {
                                        new_path.push(previous_components);
                                    }
                                    if let Some(str) = new_path.to_str() {
                                        details.filename.clear();
                                        details.filename.push_str(str);
                                    }
                                } else {
                                    details.filename.clear();
                                    details.filename.push_str(processing_filename.as_unix_str());
                                }
                            } else {
                                if edit_state.is_new_entry() {
                                    details.filename.clear();
                                }
                                details.is_editing = true;
                            }
                        }
                    }

                    callback(entry.id, details, window, cx);
                }
            }
            ix = end_ix;
        }
    }

    fn find_entry_in_worktree(
        &self,
        worktree_id: WorktreeId,
        reverse_search: bool,
        only_visible_entries: bool,
        predicate: impl Fn(GitEntryRef, WorktreeId) -> bool,
        cx: &mut Context<Self>,
    ) -> Option<GitEntry> {
        if only_visible_entries {
            let entries = self
                .state
                .visible_entries
                .iter()
                .find_map(|visible| {
                    if worktree_id == visible.worktree_id {
                        Some(&visible.entries)
                    } else {
                        None
                    }
                })?
                .clone();

            return utils::ReversibleIterable::new(entries.iter(), reverse_search)
                .find(|ele| predicate(ele.to_ref(), worktree_id))
                .cloned();
        }

        let repo_snapshots = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .repo_snapshots(cx);
        let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
        worktree.read_with(cx, |tree, _| {
            utils::ReversibleIterable::new(
                GitTraversal::new(&repo_snapshots, tree.entries(true, 0usize)),
                reverse_search,
            )
            .find_single_ended(|ele| predicate(*ele, worktree_id))
            .map(|ele| ele.to_owned())
        })
    }

    fn find_entry(
        &self,
        start: Option<&SelectedEntry>,
        reverse_search: bool,
        predicate: impl Fn(GitEntryRef, WorktreeId) -> bool,
        cx: &mut Context<Self>,
    ) -> Option<SelectedEntry> {
        let mut worktree_ids: Vec<_> = self
            .state
            .visible_entries
            .iter()
            .map(|worktree| worktree.worktree_id)
            .collect();
        let repo_snapshots = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .repo_snapshots(cx);

        let mut last_found: Option<SelectedEntry> = None;

        if let Some(start) = start {
            let worktree = self
                .project
                .read(cx)
                .worktree_for_id(start.worktree_id, cx)?
                .read(cx);

            let search = {
                let entry = worktree.entry_for_id(start.entry_id)?;
                let root_entry = worktree.root_entry()?;
                let tree_id = worktree.id();

                let mut first_iter = GitTraversal::new(
                    &repo_snapshots,
                    worktree.traverse_from_path(true, true, true, entry.path.as_ref()),
                );

                if reverse_search {
                    first_iter.next();
                }

                let first = first_iter
                    .enumerate()
                    .take_until(|(count, entry)| entry.entry == root_entry && *count != 0usize)
                    .map(|(_, entry)| entry)
                    .find(|ele| predicate(*ele, tree_id))
                    .map(|ele| ele.to_owned());

                let second_iter =
                    GitTraversal::new(&repo_snapshots, worktree.entries(true, 0usize));

                let second = if reverse_search {
                    second_iter
                        .take_until(|ele| ele.id == start.entry_id)
                        .filter(|ele| predicate(*ele, tree_id))
                        .last()
                        .map(|ele| ele.to_owned())
                } else {
                    second_iter
                        .take_while(|ele| ele.id != start.entry_id)
                        .filter(|ele| predicate(*ele, tree_id))
                        .last()
                        .map(|ele| ele.to_owned())
                };

                if reverse_search {
                    Some((second, first))
                } else {
                    Some((first, second))
                }
            };

            if let Some((first, second)) = search {
                let first = first.map(|entry| SelectedEntry {
                    worktree_id: start.worktree_id,
                    entry_id: entry.id,
                });

                let second = second.map(|entry| SelectedEntry {
                    worktree_id: start.worktree_id,
                    entry_id: entry.id,
                });

                if first.is_some() {
                    return first;
                }
                last_found = second;

                let idx = worktree_ids
                    .iter()
                    .enumerate()
                    .find(|(_, ele)| **ele == start.worktree_id)
                    .map(|(idx, _)| idx);

                if let Some(idx) = idx {
                    worktree_ids.rotate_left(idx + 1usize);
                    worktree_ids.pop();
                }
            }
        }

        for tree_id in worktree_ids.into_iter() {
            if let Some(found) =
                self.find_entry_in_worktree(tree_id, reverse_search, false, &predicate, cx)
            {
                return Some(SelectedEntry {
                    worktree_id: tree_id,
                    entry_id: found.id,
                });
            }
        }

        last_found
    }

    fn find_visible_entry(
        &self,
        start: Option<&SelectedEntry>,
        reverse_search: bool,
        predicate: impl Fn(GitEntryRef, WorktreeId) -> bool,
        cx: &mut Context<Self>,
    ) -> Option<SelectedEntry> {
        let mut worktree_ids: Vec<_> = self
            .state
            .visible_entries
            .iter()
            .map(|worktree| worktree.worktree_id)
            .collect();

        let mut last_found: Option<SelectedEntry> = None;

        if let Some(start) = start {
            let entries = self
                .state
                .visible_entries
                .iter()
                .find(|worktree| worktree.worktree_id == start.worktree_id)
                .map(|worktree| &worktree.entries)?;

            let mut start_idx = entries
                .iter()
                .enumerate()
                .find(|(_, ele)| ele.id == start.entry_id)
                .map(|(idx, _)| idx)?;

            if reverse_search {
                start_idx = start_idx.saturating_add(1usize);
            }

            let (left, right) = entries.split_at_checked(start_idx)?;

            let (first_iter, second_iter) = if reverse_search {
                (
                    utils::ReversibleIterable::new(left.iter(), reverse_search),
                    utils::ReversibleIterable::new(right.iter(), reverse_search),
                )
            } else {
                (
                    utils::ReversibleIterable::new(right.iter(), reverse_search),
                    utils::ReversibleIterable::new(left.iter(), reverse_search),
                )
            };

            let first_search = first_iter.find(|ele| predicate(ele.to_ref(), start.worktree_id));
            let second_search = second_iter.find(|ele| predicate(ele.to_ref(), start.worktree_id));

            if first_search.is_some() {
                return first_search.map(|entry| SelectedEntry {
                    worktree_id: start.worktree_id,
                    entry_id: entry.id,
                });
            }

            last_found = second_search.map(|entry| SelectedEntry {
                worktree_id: start.worktree_id,
                entry_id: entry.id,
            });

            let idx = worktree_ids
                .iter()
                .enumerate()
                .find(|(_, ele)| **ele == start.worktree_id)
                .map(|(idx, _)| idx);

            if let Some(idx) = idx {
                worktree_ids.rotate_left(idx + 1usize);
                worktree_ids.pop();
            }
        }

        for tree_id in worktree_ids.into_iter() {
            if let Some(found) =
                self.find_entry_in_worktree(tree_id, reverse_search, true, &predicate, cx)
            {
                return Some(SelectedEntry {
                    worktree_id: tree_id,
                    entry_id: found.id,
                });
            }
        }

        last_found
    }

    fn calculate_depth_and_difference(
        entry: &Entry,
        visible_worktree_entries: &HashSet<Arc<RelPath>>,
    ) -> (usize, usize) {
        let (depth, difference) = entry
            .path
            .ancestors()
            .skip(1) // Skip the entry itself
            .find_map(|ancestor| {
                if let Some(parent_entry) = visible_worktree_entries.get(ancestor) {
                    let entry_path_components_count = entry.path.components().count();
                    let parent_path_components_count = parent_entry.components().count();
                    let difference = entry_path_components_count - parent_path_components_count;
                    let depth = parent_entry
                        .ancestors()
                        .skip(1)
                        .filter(|ancestor| visible_worktree_entries.contains(*ancestor))
                        .count();
                    Some((depth + 1, difference))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| (0, entry.path.components().count()));

        (depth, difference)
    }

    fn highlight_entry_for_external_drag(
        &self,
        target_entry: &Entry,
        target_worktree: &Worktree,
    ) -> Option<ProjectEntryId> {
        // Always highlight directory or parent directory if it's file
        if target_entry.is_dir() {
            Some(target_entry.id)
        } else {
            target_entry
                .path
                .parent()
                .and_then(|parent_path| target_worktree.entry_for_path(parent_path))
                .map(|parent_entry| parent_entry.id)
        }
    }

    fn highlight_entry_for_selection_drag(
        &self,
        target_entry: &Entry,
        target_worktree: &Worktree,
        drag_state: &DraggedSelection,
        cx: &Context<Self>,
    ) -> Option<ProjectEntryId> {
        let target_parent_path = target_entry.path.parent();

        // In case of single item drag, we do not highlight existing
        // directory which item belongs too
        if drag_state.items().count() == 1
            && drag_state.active_selection.worktree_id == target_worktree.id()
        {
            let active_entry_path = self
                .project
                .read(cx)
                .path_for_entry(drag_state.active_selection.entry_id, cx)?;

            if let Some(active_parent_path) = active_entry_path.path.parent() {
                // Do not highlight active entry parent
                if active_parent_path == target_entry.path.as_ref() {
                    return None;
                }

                // Do not highlight active entry sibling files
                if Some(active_parent_path) == target_parent_path && target_entry.is_file() {
                    return None;
                }
            }
        }

        // Always highlight directory or parent directory if it's file
        if target_entry.is_dir() {
            Some(target_entry.id)
        } else {
            target_parent_path
                .and_then(|parent_path| target_worktree.entry_for_path(parent_path))
                .map(|parent_entry| parent_entry.id)
        }
    }

    fn should_highlight_background_for_selection_drag(
        &self,
        drag_state: &DraggedSelection,
        last_root_id: ProjectEntryId,
        cx: &App,
    ) -> bool {
        // Always highlight for multiple entries
        if drag_state.items().count() > 1 {
            return true;
        }

        // Since root will always have empty relative path
        if let Some(entry_path) = self
            .project
            .read(cx)
            .path_for_entry(drag_state.active_selection.entry_id, cx)
        {
            if let Some(parent_path) = entry_path.path.parent() {
                if !parent_path.is_empty() {
                    return true;
                }
            }
        }

        // If parent is empty, check if different worktree
        if let Some(last_root_worktree_id) = self
            .project
            .read(cx)
            .worktree_id_for_entry(last_root_id, cx)
        {
            if drag_state.active_selection.worktree_id != last_root_worktree_id {
                return true;
            }
        }

        false
    }

    fn render_entry(
        &self,
        entry_id: ProjectEntryId,
        details: EntryDetails,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        const GROUP_NAME: &str = "project_entry";

        let kind = details.kind;
        let is_sticky = details.sticky.is_some();
        let sticky_index = details.sticky.as_ref().map(|this| this.sticky_index);
        let settings = ProjectPanelSettings::get_global(cx);
        let show_editor = details.is_editing && !details.is_processing;

        let selection = SelectedEntry {
            worktree_id: details.worktree_id,
            entry_id,
        };

        let is_marked = self.marked_entries.contains(&selection);
        let is_active = self
            .state
            .selection
            .is_some_and(|selection| selection.entry_id == entry_id);

        let file_name = details.filename.clone();

        let mut icon = details.icon.clone();
        if settings.file_icons && show_editor && details.kind.is_file() {
            let filename = self.filename_editor.read(cx).text(cx);
            if filename.len() > 2 {
                icon = FileIcons::get_icon(Path::new(&filename), cx);
            }
        }

        let filename_text_color = details.filename_text_color;
        let diagnostic_severity = details.diagnostic_severity;
        let item_colors = get_item_color(is_sticky, cx);

        let canonical_path = details
            .canonical_path
            .as_ref()
            .map(|f| f.to_string_lossy().into_owned());
        let path_style = self.project.read(cx).path_style(cx);
        let path = details.path.clone();
        let path_for_external_paths = path.clone();
        let path_for_dragged_selection = path.clone();

        let depth = details.depth;
        let worktree_id = details.worktree_id;
        let dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: selection.worktree_id,
                entry_id: self.resolve_entry(selection.entry_id),
            },
            marked_selections: Arc::from(self.marked_entries.clone()),
        };

        let bg_color = if is_marked {
            item_colors.marked
        } else {
            item_colors.default
        };

        let bg_hover_color = if is_marked {
            item_colors.marked
        } else {
            item_colors.hover
        };

        let validation_color_and_message = if show_editor {
            match self
                .state
                .edit_state
                .as_ref()
                .map_or(ValidationState::None, |e| e.validation_state.clone())
            {
                ValidationState::Error(msg) => Some((Color::Error.color(cx), msg)),
                ValidationState::Warning(msg) => Some((Color::Warning.color(cx), msg)),
                ValidationState::None => None,
            }
        } else {
            None
        };

        let border_color =
            if !self.mouse_down && is_active && self.focus_handle.contains_focused(window, cx) {
                match validation_color_and_message {
                    Some((color, _)) => color,
                    None => item_colors.focused,
                }
            } else {
                bg_color
            };

        let border_hover_color =
            if !self.mouse_down && is_active && self.focus_handle.contains_focused(window, cx) {
                match validation_color_and_message {
                    Some((color, _)) => color,
                    None => item_colors.focused,
                }
            } else {
                bg_hover_color
            };

        let folded_directory_drag_target = self.folded_directory_drag_target;
        let is_highlighted = {
            if let Some(highlight_entry_id) =
                self.drag_target_entry
                    .as_ref()
                    .and_then(|drag_target| match drag_target {
                        DragTarget::Entry {
                            highlight_entry_id, ..
                        } => Some(*highlight_entry_id),
                        DragTarget::Background => self.state.last_worktree_root_id,
                    })
            {
                // Highlight if same entry or it's children
                if entry_id == highlight_entry_id {
                    true
                } else {
                    maybe!({
                        let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
                        let highlight_entry = worktree.read(cx).entry_for_id(highlight_entry_id)?;
                        Some(path.starts_with(&highlight_entry.path))
                    })
                    .unwrap_or(false)
                }
            } else {
                false
            }
        };

        let id: ElementId = if is_sticky {
            SharedString::from(format!("project_panel_sticky_item_{}", entry_id.to_usize())).into()
        } else {
            (entry_id.to_proto() as usize).into()
        };

        div()
            .id(id.clone())
            .relative()
            .group(GROUP_NAME)
            .cursor_pointer()
            .rounded_none()
            .bg(bg_color)
            .border_1()
            .border_r_2()
            .border_color(border_color)
            .hover(|style| style.bg(bg_hover_color).border_color(border_hover_color))
            .when(is_sticky, |this| {
                this.block_mouse_except_scroll()
            })
            .when(!is_sticky, |this| {
                this
                .when(is_highlighted && folded_directory_drag_target.is_none(), |this| this.border_color(transparent_white()).bg(item_colors.drag_over))
                .when(settings.drag_and_drop, |this| this
                .on_drag_move::<ExternalPaths>(cx.listener(
                    move |this, event: &DragMoveEvent<ExternalPaths>, _, cx| {
                        let is_current_target = this.drag_target_entry.as_ref()
                             .and_then(|entry| match entry {
                                 DragTarget::Entry { entry_id: target_id, .. } => Some(*target_id),
                                 DragTarget::Background { .. } => None,
                             }) == Some(entry_id);

                        if !event.bounds.contains(&event.event.position) {
                            // Entry responsible for setting drag target is also responsible to
                            // clear it up after drag is out of bounds
                            if is_current_target {
                                this.drag_target_entry = None;
                            }
                            return;
                        }

                        if is_current_target {
                            return;
                        }

                        this.marked_entries.clear();

                        let Some((entry_id, highlight_entry_id)) = maybe!({
                            let target_worktree = this.project.read(cx).worktree_for_id(selection.worktree_id, cx)?.read(cx);
                            let target_entry = target_worktree.entry_for_path(&path_for_external_paths)?;
                            let highlight_entry_id = this.highlight_entry_for_external_drag(target_entry, target_worktree)?;
                            Some((target_entry.id, highlight_entry_id))
                        }) else {
                            return;
                        };

                        this.drag_target_entry = Some(DragTarget::Entry {
                            entry_id,
                            highlight_entry_id,
                        });

                    },
                ))
                .on_drop(cx.listener(
                    move |this, external_paths: &ExternalPaths, window, cx| {
                        this.drag_target_entry = None;
                        this.hover_scroll_task.take();
                        this.drop_external_files(external_paths.paths(), entry_id, window, cx);
                        cx.stop_propagation();
                    },
                ))
                .on_drag_move::<DraggedSelection>(cx.listener(
                    move |this, event: &DragMoveEvent<DraggedSelection>, window, cx| {
                        let is_current_target = this.drag_target_entry.as_ref()
                             .and_then(|entry| match entry {
                                 DragTarget::Entry { entry_id: target_id, .. } => Some(*target_id),
                                 DragTarget::Background { .. } => None,
                             }) == Some(entry_id);

                        if !event.bounds.contains(&event.event.position) {
                            // Entry responsible for setting drag target is also responsible to
                            // clear it up after drag is out of bounds
                            if is_current_target {
                                this.drag_target_entry = None;
                            }
                            return;
                        }

                        if is_current_target {
                            return;
                        }

                        let drag_state = event.drag(cx);

                        if drag_state.items().count() == 1 {
                            this.marked_entries.clear();
                            this.marked_entries.push(drag_state.active_selection);
                        }

                        let Some((entry_id, highlight_entry_id)) = maybe!({
                            let target_worktree = this.project.read(cx).worktree_for_id(selection.worktree_id, cx)?.read(cx);
                            let target_entry = target_worktree.entry_for_path(&path_for_dragged_selection)?;
                            let highlight_entry_id = this.highlight_entry_for_selection_drag(target_entry, target_worktree, drag_state, cx)?;
                            Some((target_entry.id, highlight_entry_id))
                        }) else {
                            return;
                        };

                        this.drag_target_entry = Some(DragTarget::Entry {
                            entry_id,
                            highlight_entry_id,
                        });

                        this.hover_expand_task.take();

                        if !kind.is_dir()
                            || this
                                .state
                                .expanded_dir_ids
                                .get(&details.worktree_id)
                                .is_some_and(|ids| ids.binary_search(&entry_id).is_ok())
                        {
                            return;
                        }

                        let bounds = event.bounds;
                        this.hover_expand_task =
                            Some(cx.spawn_in(window, async move |this, cx| {
                                cx.background_executor()
                                    .timer(Duration::from_millis(500))
                                    .await;
                                this.update_in(cx, |this, window, cx| {
                                    this.hover_expand_task.take();
                                    if this.drag_target_entry.as_ref().and_then(|entry| match entry {
                                        DragTarget::Entry { entry_id: target_id, .. } => Some(*target_id),
                                        DragTarget::Background { .. } => None,
                                    }) == Some(entry_id)
                                        && bounds.contains(&window.mouse_position())
                                    {
                                        this.expand_entry(worktree_id, entry_id, cx);
                                        this.update_visible_entries(
                                            Some((worktree_id, entry_id)),
                                            false,
                                            false,
                                            window,
                                            cx,
                                        );
                                        cx.notify();
                                    }
                                })
                                .ok();
                            }));
                    },
                ))
                .on_drag(
                    dragged_selection,
                    {
                        let active_component = self.state.ancestors.get(&entry_id).and_then(|ancestors| ancestors.active_component(&details.filename));
                        move |selection, click_offset, _window, cx| {
                            let filename = active_component.as_ref().unwrap_or_else(|| &details.filename);
                            cx.new(|_| DraggedProjectEntryView {
                                icon: details.icon.clone(),
                                filename: filename.clone(),
                                click_offset,
                                selection: selection.active_selection,
                                selections: selection.marked_selections.clone(),
                            })
                        }
                    }
                )
                .on_drop(
                    cx.listener(move |this, selections: &DraggedSelection, window, cx| {
                        this.drag_target_entry = None;
                        this.hover_scroll_task.take();
                        this.hover_expand_task.take();
                        if folded_directory_drag_target.is_some() {
                            return;
                        }
                        this.drag_onto(selections, entry_id, kind.is_file(), window, cx);
                    }),
                ))
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.mouse_down = true;
                    cx.propagate();
                }),
            )
            .on_click(
                cx.listener(move |project_panel, event: &gpui::ClickEvent, window, cx| {
                    if event.is_right_click() || event.first_focus()
                        || show_editor
                    {
                        return;
                    }
                    if event.standard_click() {
                        project_panel.mouse_down = false;
                    }
                    cx.stop_propagation();

                    if let Some(selection) = project_panel.state.selection.filter(|_| event.modifiers().shift) {
                        let current_selection = project_panel.index_for_selection(selection);
                        let clicked_entry = SelectedEntry {
                            entry_id,
                            worktree_id,
                        };
                        let target_selection = project_panel.index_for_selection(clicked_entry);
                        if let Some(((_, _, source_index), (_, _, target_index))) =
                            current_selection.zip(target_selection)
                        {
                            let range_start = source_index.min(target_index);
                            let range_end = source_index.max(target_index) + 1;
                            let mut new_selections = Vec::new();
                            project_panel.for_each_visible_entry(
                                range_start..range_end,
                                window,
                                cx,
                                |entry_id, details, _, _| {
                                    new_selections.push(SelectedEntry {
                                        entry_id,
                                        worktree_id: details.worktree_id,
                                    });
                                },
                            );

                            for selection in &new_selections {
                                if !project_panel.marked_entries.contains(selection) {
                                    project_panel.marked_entries.push(*selection);
                                }
                            }

                            project_panel.state.selection = Some(clicked_entry);
                            if !project_panel.marked_entries.contains(&clicked_entry) {
                                project_panel.marked_entries.push(clicked_entry);
                            }
                        }
                    } else if event.modifiers().secondary() {
                        if event.click_count() > 1 {
                            project_panel.split_entry(entry_id, false, None, cx);
                        } else {
                            project_panel.state.selection = Some(selection);
                            if let Some(position) = project_panel.marked_entries.iter().position(|e| *e == selection) {
                                project_panel.marked_entries.remove(position);
                            } else {
                                project_panel.marked_entries.push(selection);
                            }
                        }
                    } else if kind.is_dir() {
                        project_panel.marked_entries.clear();
                        if is_sticky
                            && let Some((_, _, index)) = project_panel.index_for_entry(entry_id, worktree_id) {
                                project_panel.scroll_handle.scroll_to_item_strict_with_offset(index, ScrollStrategy::Top, sticky_index.unwrap_or(0));
                                cx.notify();
                                // move down by 1px so that clicked item
                                // don't count as sticky anymore
                                cx.on_next_frame(window, |_, window, cx| {
                                    cx.on_next_frame(window, |this, _, cx| {
                                        let mut offset = this.scroll_handle.offset();
                                        offset.y += px(1.);
                                        this.scroll_handle.set_offset(offset);
                                        cx.notify();
                                    });
                                });
                                return;
                            }
                        if event.modifiers().alt {
                            project_panel.toggle_expand_all(entry_id, window, cx);
                        } else {
                            project_panel.toggle_expanded(entry_id, window, cx);
                        }
                    } else {
                        let preview_tabs_enabled = PreviewTabsSettings::get_global(cx).enable_preview_from_project_panel;
                        let click_count = event.click_count();
                        let focus_opened_item = click_count > 1;
                        let allow_preview = preview_tabs_enabled && click_count == 1;
                        project_panel.open_entry(entry_id, focus_opened_item, allow_preview, cx);
                    }
                }),
            )
            .child(
                ListItem::new(id)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .spacing(match settings.entry_spacing {
                        ProjectPanelEntrySpacing::Comfortable => ListItemSpacing::Dense,
                        ProjectPanelEntrySpacing::Standard => {
                            ListItemSpacing::ExtraDense
                        }
                    })
                    .selectable(false)
                    .when_some(canonical_path, |this, path| {
                        this.end_slot::<AnyElement>(
                            div()
                                .id("symlink_icon")
                                .pr_3()
                                .tooltip(move |_window, cx| {
                                    Tooltip::with_meta(
                                        path.to_string(),
                                        None,
                                        "Symbolic Link",
                                        cx,
                                    )
                                })
                                .child(
                                    Icon::new(IconName::ArrowUpRight)
                                        .size(IconSize::Indicator)
                                        .color(filename_text_color),
                                )
                                .into_any_element(),
                        )
                    })
                    .child(if let Some(icon) = &icon {
                        if let Some((_, decoration_color)) =
                            entry_diagnostic_aware_icon_decoration_and_color(diagnostic_severity)
                        {
                            let is_warning = diagnostic_severity
                                .map(|severity| matches!(severity, DiagnosticSeverity::WARNING))
                                .unwrap_or(false);
                            div().child(
                                DecoratedIcon::new(
                                    Icon::from_path(icon.clone()).color(Color::Muted),
                                    Some(
                                        IconDecoration::new(
                                            if kind.is_file() {
                                                if is_warning {
                                                    IconDecorationKind::Triangle
                                                } else {
                                                    IconDecorationKind::X
                                                }
                                            } else {
                                                IconDecorationKind::Dot
                                            },
                                            bg_color,
                                            cx,
                                        )
                                        .group_name(Some(GROUP_NAME.into()))
                                        .knockout_hover_color(bg_hover_color)
                                        .color(decoration_color.color(cx))
                                        .position(Point {
                                            x: px(-2.),
                                            y: px(-2.),
                                        }),
                                    ),
                                )
                                .into_any_element(),
                            )
                        } else {
                            h_flex().child(Icon::from_path(icon.to_string()).color(Color::Muted))
                        }
                    } else if let Some((icon_name, color)) =
                        entry_diagnostic_aware_icon_name_and_color(diagnostic_severity)
                    {
                        h_flex()
                            .size(IconSize::default().rems())
                            .child(Icon::new(icon_name).color(color).size(IconSize::Small))
                    } else {
                        h_flex()
                            .size(IconSize::default().rems())
                            .invisible()
                            .flex_none()
                    })
                    .child(
                        if let (Some(editor), true) = (Some(&self.filename_editor), show_editor) {
                            h_flex().h_6().w_full().child(editor.clone())
                        } else {
                            h_flex().h_6().map(|mut this| {
                                if let Some(folded_ancestors) = self.state.ancestors.get(&entry_id) {
                                    let components = Path::new(&file_name)
                                        .components()
                                        .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
                                        .collect::<Vec<_>>();
                                    let active_index = folded_ancestors.active_index();
                                    let components_len = components.len();
                                    let delimiter = SharedString::new(path_style.primary_separator());
                                    for (index, component) in components.iter().enumerate() {
                                        if index != 0 {
                                                let delimiter_target_index = index - 1;
                                                let target_entry_id = folded_ancestors.ancestors.get(components_len - 1 - delimiter_target_index).cloned();
                                                this = this.child(
                                                    div()
                                                    .when(!is_sticky, |div| {
                                                        div
                                                            .when(settings.drag_and_drop, |div| div
                                                            .on_drop(cx.listener(move |this, selections: &DraggedSelection, window, cx| {
                                                            this.hover_scroll_task.take();
                                                            this.drag_target_entry = None;
                                                            this.folded_directory_drag_target = None;
                                                            if let Some(target_entry_id) = target_entry_id {
                                                                this.drag_onto(selections, target_entry_id, kind.is_file(), window, cx);
                                                            }
                                                        }))
                                                        .on_drag_move(cx.listener(
                                                            move |this, event: &DragMoveEvent<DraggedSelection>, _, _| {
                                                                if event.bounds.contains(&event.event.position) {
                                                                    this.folded_directory_drag_target = Some(
                                                                        FoldedDirectoryDragTarget {
                                                                            entry_id,
                                                                            index: delimiter_target_index,
                                                                            is_delimiter_target: true,
                                                                        }
                                                                    );
                                                                } else {
                                                                    let is_current_target = this.folded_directory_drag_target
                                                                        .is_some_and(|target|
                                                                            target.entry_id == entry_id &&
                                                                            target.index == delimiter_target_index &&
                                                                            target.is_delimiter_target
                                                                        );
                                                                    if is_current_target {
                                                                        this.folded_directory_drag_target = None;
                                                                    }
                                                                }

                                                            },
                                                        )))
                                                    })
                                                    .child(
                                                        Label::new(delimiter.clone())
                                                            .single_line()
                                                            .color(filename_text_color)
                                                    )
                                                );
                                        }
                                        let id = SharedString::from(format!(
                                            "project_panel_path_component_{}_{index}",
                                            entry_id.to_usize()
                                        ));
                                        let label = div()
                                            .id(id)
                                            .when(!is_sticky,| div| {
                                                div
                                                .when(index != components_len - 1, |div|{
                                                    let target_entry_id = folded_ancestors.ancestors.get(components_len - 1 - index).cloned();
                                                    div
                                                    .when(settings.drag_and_drop, |div| div
                                                    .on_drag_move(cx.listener(
                                                        move |this, event: &DragMoveEvent<DraggedSelection>, _, _| {
                                                        if event.bounds.contains(&event.event.position) {
                                                                this.folded_directory_drag_target = Some(
                                                                    FoldedDirectoryDragTarget {
                                                                        entry_id,
                                                                        index,
                                                                        is_delimiter_target: false,
                                                                    }
                                                                );
                                                            } else {
                                                                let is_current_target = this.folded_directory_drag_target
                                                                    .as_ref()
                                                                    .is_some_and(|target|
                                                                        target.entry_id == entry_id &&
                                                                        target.index == index &&
                                                                        !target.is_delimiter_target
                                                                    );
                                                                if is_current_target {
                                                                    this.folded_directory_drag_target = None;
                                                                }
                                                            }
                                                        },
                                                    ))
                                                    .on_drop(cx.listener(move |this, selections: &DraggedSelection, window,cx| {
                                                        this.hover_scroll_task.take();
                                                        this.drag_target_entry = None;
                                                        this.folded_directory_drag_target = None;
                                                        if let Some(target_entry_id) = target_entry_id {
                                                            this.drag_onto(selections, target_entry_id, kind.is_file(), window, cx);
                                                        }
                                                    }))
                                                    .when(folded_directory_drag_target.is_some_and(|target|
                                                        target.entry_id == entry_id &&
                                                        target.index == index
                                                    ), |this| {
                                                        this.bg(item_colors.drag_over)
                                                    }))
                                                })
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _, _, cx| {
                                                    if index != active_index
                                                        && let Some(folds) =
                                                            this.state.ancestors.get_mut(&entry_id)
                                                        {
                                                            folds.current_ancestor_depth =
                                                                components_len - 1 - index;
                                                            cx.notify();
                                                        }
                                                }),
                                            )
                                            .child(
                                                Label::new(component)
                                                    .single_line()
                                                    .color(filename_text_color)
                                                    .when(
                                                        index == active_index
                                                            && (is_active || is_marked),
                                                        |this| this.underline(),
                                                    ),
                                            );

                                        this = this.child(label);
                                    }

                                    this
                                } else {
                                    this.child(
                                        Label::new(file_name)
                                            .single_line()
                                            .color(filename_text_color),
                                    )
                                }
                            })
                        },
                    )
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, window, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            // Some context menu actions apply to all marked entries. If the user
                            // right-clicks on an entry that is not marked, they may not realize the
                            // action applies to multiple entries. To avoid inadvertent changes, all
                            // entries are unmarked.
                            if !this.marked_entries.contains(&selection) {
                                this.marked_entries.clear();
                            }
                            this.deploy_context_menu(event.position, entry_id, window, cx);
                        },
                    ))
                    .overflow_x(),
            )
            .when_some(
                validation_color_and_message,
                |this, (color, message)| {
                    this
                    .relative()
                    .child(
                        deferred(
                            div()
                            .occlude()
                            .absolute()
                            .top_full()
                            .left(px(-1.)) // Used px over rem so that it doesn't change with font size
                            .right(px(-0.5))
                            .py_1()
                            .px_2()
                            .border_1()
                            .border_color(color)
                            .bg(cx.theme().colors().background)
                            .child(
                                Label::new(message)
                                .color(Color::from(color))
                                .size(LabelSize::Small)
                            )
                        )
                    )
                }
            )
    }

    fn details_for_entry(
        &self,
        entry: &Entry,
        worktree_id: WorktreeId,
        root_name: &RelPath,
        entries_paths: &HashSet<Arc<RelPath>>,
        git_status: GitSummary,
        sticky: Option<StickyDetails>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> EntryDetails {
        let (show_file_icons, show_folder_icons) = {
            let settings = ProjectPanelSettings::get_global(cx);
            (settings.file_icons, settings.folder_icons)
        };

        let expanded_entry_ids = self
            .state
            .expanded_dir_ids
            .get(&worktree_id)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let is_expanded = expanded_entry_ids.binary_search(&entry.id).is_ok();

        let icon = match entry.kind {
            EntryKind::File => {
                if show_file_icons {
                    FileIcons::get_icon(entry.path.as_std_path(), cx)
                } else {
                    None
                }
            }
            _ => {
                if show_folder_icons {
                    FileIcons::get_folder_icon(is_expanded, entry.path.as_std_path(), cx)
                } else {
                    FileIcons::get_chevron_icon(is_expanded, cx)
                }
            }
        };

        let path_style = self.project.read(cx).path_style(cx);
        let (depth, difference) =
            ProjectPanel::calculate_depth_and_difference(entry, entries_paths);

        let filename = if difference > 1 {
            entry
                .path
                .last_n_components(difference)
                .map_or(String::new(), |suffix| {
                    suffix.display(path_style).to_string()
                })
        } else {
            entry
                .path
                .file_name()
                .map(|name| name.to_string())
                .unwrap_or_else(|| root_name.as_unix_str().to_string())
        };

        let selection = SelectedEntry {
            worktree_id,
            entry_id: entry.id,
        };
        let is_marked = self.marked_entries.contains(&selection);
        let is_selected = self.state.selection == Some(selection);

        let diagnostic_severity = self
            .diagnostics
            .get(&(worktree_id, entry.path.clone()))
            .cloned();

        let filename_text_color =
            entry_git_aware_label_color(git_status, entry.is_ignored, is_marked);

        let is_cut = self
            .clipboard
            .as_ref()
            .is_some_and(|e| e.is_cut() && e.items().contains(&selection));

        EntryDetails {
            filename,
            icon,
            path: entry.path.clone(),
            depth,
            kind: entry.kind,
            is_ignored: entry.is_ignored,
            is_expanded,
            is_selected,
            is_marked,
            is_editing: false,
            is_processing: false,
            is_cut,
            sticky,
            filename_text_color,
            diagnostic_severity,
            git_status,
            is_private: entry.is_private,
            worktree_id,
            canonical_path: entry.canonical_path.clone(),
        }
    }

    fn dispatch_context(&self, window: &Window, cx: &Context<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("ProjectPanel");
        dispatch_context.add("menu");

        let identifier = if self.filename_editor.focus_handle(cx).is_focused(window) {
            "editing"
        } else {
            "not_editing"
        };

        dispatch_context.add(identifier);
        dispatch_context
    }

    fn reveal_entry(
        &mut self,
        project: Entity<Project>,
        entry_id: ProjectEntryId,
        skip_ignored: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let worktree = project
            .read(cx)
            .worktree_for_entry(entry_id, cx)
            .context("can't reveal a non-existent entry in the project panel")?;
        let worktree = worktree.read(cx);
        if skip_ignored
            && worktree
                .entry_for_id(entry_id)
                .is_none_or(|entry| entry.is_ignored && !entry.is_always_included)
        {
            anyhow::bail!("can't reveal an ignored entry in the project panel");
        }
        let is_active_item_file_diff_view = self
            .workspace
            .upgrade()
            .and_then(|ws| ws.read(cx).active_item(cx))
            .map(|item| item.act_as_type(TypeId::of::<FileDiffView>(), cx).is_some())
            .unwrap_or(false);
        if is_active_item_file_diff_view {
            return Ok(());
        }

        let worktree_id = worktree.id();
        self.expand_entry(worktree_id, entry_id, cx);
        self.update_visible_entries(Some((worktree_id, entry_id)), false, true, window, cx);
        self.marked_entries.clear();
        self.marked_entries.push(SelectedEntry {
            worktree_id,
            entry_id,
        });
        cx.notify();
        Ok(())
    }

    fn find_active_indent_guide(
        &self,
        indent_guides: &[IndentGuideLayout],
        cx: &App,
    ) -> Option<usize> {
        let (worktree, entry) = self.selected_entry(cx)?;

        // Find the parent entry of the indent guide, this will either be the
        // expanded folder we have selected, or the parent of the currently
        // selected file/collapsed directory
        let mut entry = entry;
        loop {
            let is_expanded_dir = entry.is_dir()
                && self
                    .state
                    .expanded_dir_ids
                    .get(&worktree.id())
                    .map(|ids| ids.binary_search(&entry.id).is_ok())
                    .unwrap_or(false);
            if is_expanded_dir {
                break;
            }
            entry = worktree.entry_for_path(&entry.path.parent()?)?;
        }

        let (active_indent_range, depth) = {
            let (worktree_ix, child_offset, ix) = self.index_for_entry(entry.id, worktree.id())?;
            let child_paths = &self.state.visible_entries[worktree_ix].entries;
            let mut child_count = 0;
            let depth = entry.path.ancestors().count();
            while let Some(entry) = child_paths.get(child_offset + child_count + 1) {
                if entry.path.ancestors().count() <= depth {
                    break;
                }
                child_count += 1;
            }

            let start = ix + 1;
            let end = start + child_count;

            let visible_worktree = &self.state.visible_entries[worktree_ix];
            let visible_worktree_entries = visible_worktree.index.get_or_init(|| {
                visible_worktree
                    .entries
                    .iter()
                    .map(|e| e.path.clone())
                    .collect()
            });

            // Calculate the actual depth of the entry, taking into account that directories can be auto-folded.
            let (depth, _) = Self::calculate_depth_and_difference(entry, visible_worktree_entries);
            (start..end, depth)
        };

        let candidates = indent_guides
            .iter()
            .enumerate()
            .filter(|(_, indent_guide)| indent_guide.offset.x == depth);

        for (i, indent) in candidates {
            // Find matches that are either an exact match, partially on screen, or inside the enclosing indent
            if active_indent_range.start <= indent.offset.y + indent.length
                && indent.offset.y <= active_indent_range.end
            {
                return Some(i);
            }
        }
        None
    }

    fn render_sticky_entries(
        &self,
        child: StickyProjectPanelCandidate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> SmallVec<[AnyElement; 8]> {
        let project = self.project.read(cx);

        let Some((worktree_id, entry_ref)) = self.entry_at_index(child.index) else {
            return SmallVec::new();
        };

        let Some(visible) = self
            .state
            .visible_entries
            .iter()
            .find(|worktree| worktree.worktree_id == worktree_id)
        else {
            return SmallVec::new();
        };

        let Some(worktree) = project.worktree_for_id(worktree_id, cx) else {
            return SmallVec::new();
        };
        let worktree = worktree.read(cx).snapshot();

        let paths = visible
            .index
            .get_or_init(|| visible.entries.iter().map(|e| e.path.clone()).collect());

        let mut sticky_parents = Vec::new();
        let mut current_path = entry_ref.path.clone();

        'outer: loop {
            if let Some(parent_path) = current_path.parent() {
                for ancestor_path in parent_path.ancestors() {
                    if paths.contains(ancestor_path)
                        && let Some(parent_entry) = worktree.entry_for_path(ancestor_path)
                    {
                        sticky_parents.push(parent_entry.clone());
                        current_path = parent_entry.path.clone();
                        continue 'outer;
                    }
                }
            }
            break 'outer;
        }

        if sticky_parents.is_empty() {
            return SmallVec::new();
        }

        sticky_parents.reverse();

        let panel_settings = ProjectPanelSettings::get_global(cx);
        let git_status_enabled = panel_settings.git_status;
        let root_name = worktree.root_name();

        let git_summaries_by_id = if git_status_enabled {
            visible
                .entries
                .iter()
                .map(|e| (e.id, e.git_summary))
                .collect::<HashMap<_, _>>()
        } else {
            Default::default()
        };

        // already checked if non empty above
        let last_item_index = sticky_parents.len() - 1;
        sticky_parents
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let git_status = git_summaries_by_id
                    .get(&entry.id)
                    .copied()
                    .unwrap_or_default();
                let sticky_details = Some(StickyDetails {
                    sticky_index: index,
                });
                let details = self.details_for_entry(
                    entry,
                    worktree_id,
                    root_name,
                    paths,
                    git_status,
                    sticky_details,
                    window,
                    cx,
                );
                self.render_entry(entry.id, details, window, cx)
                    .when(index == last_item_index, |this| {
                        let shadow_color_top = hsla(0.0, 0.0, 0.0, 0.1);
                        let shadow_color_bottom = hsla(0.0, 0.0, 0.0, 0.);
                        let sticky_shadow = div()
                            .absolute()
                            .left_0()
                            .bottom_neg_1p5()
                            .h_1p5()
                            .w_full()
                            .bg(linear_gradient(
                                0.,
                                linear_color_stop(shadow_color_top, 1.),
                                linear_color_stop(shadow_color_bottom, 0.),
                            ));
                        this.child(sticky_shadow)
                    })
                    .into_any()
            })
            .collect()
    }
}

#[derive(Clone)]
struct StickyProjectPanelCandidate {
    index: usize,
    depth: usize,
}

impl StickyCandidate for StickyProjectPanelCandidate {
    fn depth(&self) -> usize {
        self.depth
    }
}

fn item_width_estimate(depth: usize, item_text_chars: usize, is_symlink: bool) -> usize {
    const ICON_SIZE_FACTOR: usize = 2;
    let mut item_width = depth * ICON_SIZE_FACTOR + item_text_chars;
    if is_symlink {
        item_width += ICON_SIZE_FACTOR;
    }
    item_width
}

impl Render for ProjectPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_worktree = !self.state.visible_entries.is_empty();
        let project = self.project.read(cx);
        let panel_settings = ProjectPanelSettings::get_global(cx);
        let indent_size = panel_settings.indent_size;
        let show_indent_guides = panel_settings.indent_guides.show == ShowIndentGuides::Always;
        let show_sticky_entries = {
            if panel_settings.sticky_scroll {
                let is_scrollable = self.scroll_handle.is_scrollable();
                let is_scrolled = self.scroll_handle.offset().y < px(0.);
                is_scrollable && is_scrolled
            } else {
                false
            }
        };

        let is_local = project.is_local();

        if has_worktree {
            let item_count = self
                .state
                .visible_entries
                .iter()
                .map(|worktree| worktree.entries.len())
                .sum();

            fn handle_drag_move<T: 'static>(
                this: &mut ProjectPanel,
                e: &DragMoveEvent<T>,
                window: &mut Window,
                cx: &mut Context<ProjectPanel>,
            ) {
                if let Some(previous_position) = this.previous_drag_position {
                    // Refresh cursor only when an actual drag happens,
                    // because modifiers are not updated when the cursor is not moved.
                    if e.event.position != previous_position {
                        this.refresh_drag_cursor_style(&e.event.modifiers, window, cx);
                    }
                }
                this.previous_drag_position = Some(e.event.position);

                if !e.bounds.contains(&e.event.position) {
                    this.drag_target_entry = None;
                    return;
                }
                this.hover_scroll_task.take();
                let panel_height = e.bounds.size.height;
                if panel_height <= px(0.) {
                    return;
                }

                let event_offset = e.event.position.y - e.bounds.origin.y;
                // How far along in the project panel is our cursor? (0. is the top of a list, 1. is the bottom)
                let hovered_region_offset = event_offset / panel_height;

                // We want the scrolling to be a bit faster when the cursor is closer to the edge of a list.
                // These pixels offsets were picked arbitrarily.
                let vertical_scroll_offset = if hovered_region_offset <= 0.05 {
                    8.
                } else if hovered_region_offset <= 0.15 {
                    5.
                } else if hovered_region_offset >= 0.95 {
                    -8.
                } else if hovered_region_offset >= 0.85 {
                    -5.
                } else {
                    return;
                };
                let adjustment = point(px(0.), px(vertical_scroll_offset));
                this.hover_scroll_task = Some(cx.spawn_in(window, async move |this, cx| {
                    loop {
                        let should_stop_scrolling = this
                            .update(cx, |this, cx| {
                                this.hover_scroll_task.as_ref()?;
                                let handle = this.scroll_handle.0.borrow_mut();
                                let offset = handle.base_handle.offset();

                                handle.base_handle.set_offset(offset + adjustment);
                                cx.notify();
                                Some(())
                            })
                            .ok()
                            .flatten()
                            .is_some();
                        if should_stop_scrolling {
                            return;
                        }
                        cx.background_executor()
                            .timer(Duration::from_millis(16))
                            .await;
                    }
                }));
            }
            h_flex()
                .id("project-panel")
                .group("project-panel")
                .when(panel_settings.drag_and_drop, |this| {
                    this.on_drag_move(cx.listener(handle_drag_move::<ExternalPaths>))
                        .on_drag_move(cx.listener(handle_drag_move::<DraggedSelection>))
                })
                .size_full()
                .relative()
                .on_modifiers_changed(cx.listener(
                    |this, event: &ModifiersChangedEvent, window, cx| {
                        this.refresh_drag_cursor_style(&event.modifiers, window, cx);
                    },
                ))
                .key_context(self.dispatch_context(window, cx))
                .on_action(cx.listener(Self::scroll_up))
                .on_action(cx.listener(Self::scroll_down))
                .on_action(cx.listener(Self::scroll_cursor_center))
                .on_action(cx.listener(Self::scroll_cursor_top))
                .on_action(cx.listener(Self::scroll_cursor_bottom))
                .on_action(cx.listener(Self::select_next))
                .on_action(cx.listener(Self::select_previous))
                .on_action(cx.listener(Self::select_first))
                .on_action(cx.listener(Self::select_last))
                .on_action(cx.listener(Self::select_parent))
                .on_action(cx.listener(Self::select_next_git_entry))
                .on_action(cx.listener(Self::select_prev_git_entry))
                .on_action(cx.listener(Self::select_next_diagnostic))
                .on_action(cx.listener(Self::select_prev_diagnostic))
                .on_action(cx.listener(Self::select_next_directory))
                .on_action(cx.listener(Self::select_prev_directory))
                .on_action(cx.listener(Self::expand_selected_entry))
                .on_action(cx.listener(Self::collapse_selected_entry))
                .on_action(cx.listener(Self::collapse_all_entries))
                .on_action(cx.listener(Self::open))
                .on_action(cx.listener(Self::open_permanent))
                .on_action(cx.listener(Self::open_split_vertical))
                .on_action(cx.listener(Self::open_split_horizontal))
                .on_action(cx.listener(Self::confirm))
                .on_action(cx.listener(Self::cancel))
                .on_action(cx.listener(Self::copy_path))
                .on_action(cx.listener(Self::copy_relative_path))
                .on_action(cx.listener(Self::new_search_in_directory))
                .on_action(cx.listener(Self::unfold_directory))
                .on_action(cx.listener(Self::fold_directory))
                .on_action(cx.listener(Self::remove_from_project))
                .on_action(cx.listener(Self::compare_marked_files))
                .when(!project.is_read_only(cx), |el| {
                    el.on_action(cx.listener(Self::new_file))
                        .on_action(cx.listener(Self::new_directory))
                        .on_action(cx.listener(Self::rename))
                        .on_action(cx.listener(Self::delete))
                        .on_action(cx.listener(Self::cut))
                        .on_action(cx.listener(Self::copy))
                        .on_action(cx.listener(Self::paste))
                        .on_action(cx.listener(Self::duplicate))
                        .on_action(cx.listener(Self::restore_file))
                        .when(!project.is_remote(), |el| {
                            el.on_action(cx.listener(Self::trash))
                        })
                })
                .when(project.is_local(), |el| {
                    el.on_action(cx.listener(Self::reveal_in_finder))
                        .on_action(cx.listener(Self::open_system))
                        .on_action(cx.listener(Self::open_in_terminal))
                })
                .when(project.is_via_remote_server(), |el| {
                    el.on_action(cx.listener(Self::open_in_terminal))
                })
                .track_focus(&self.focus_handle(cx))
                .child(
                    v_flex()
                        .child(
                            uniform_list("entries", item_count, {
                                cx.processor(|this, range: Range<usize>, window, cx| {
                                    this.rendered_entries_len = range.end - range.start;
                                    let mut items = Vec::with_capacity(this.rendered_entries_len);
                                    this.for_each_visible_entry(
                                        range,
                                        window,
                                        cx,
                                        |id, details, window, cx| {
                                            items.push(this.render_entry(id, details, window, cx));
                                        },
                                    );
                                    items
                                })
                            })
                            .when(show_indent_guides, |list| {
                                list.with_decoration(
                                    ui::indent_guides(
                                        px(indent_size),
                                        IndentGuideColors::panel(cx),
                                    )
                                    .with_compute_indents_fn(
                                        cx.entity(),
                                        |this, range, window, cx| {
                                            let mut items =
                                                SmallVec::with_capacity(range.end - range.start);
                                            this.iter_visible_entries(
                                                range,
                                                window,
                                                cx,
                                                |entry, _, entries, _, _| {
                                                    let (depth, _) =
                                                        Self::calculate_depth_and_difference(
                                                            entry, entries,
                                                        );
                                                    items.push(depth);
                                                },
                                            );
                                            items
                                        },
                                    )
                                    .on_click(cx.listener(
                                        |this,
                                         active_indent_guide: &IndentGuideLayout,
                                         window,
                                         cx| {
                                            if window.modifiers().secondary() {
                                                let ix = active_indent_guide.offset.y;
                                                let Some((target_entry, worktree)) = maybe!({
                                                    let (worktree_id, entry) =
                                                        this.entry_at_index(ix)?;
                                                    let worktree = this
                                                        .project
                                                        .read(cx)
                                                        .worktree_for_id(worktree_id, cx)?;
                                                    let target_entry = worktree
                                                        .read(cx)
                                                        .entry_for_path(&entry.path.parent()?)?;
                                                    Some((target_entry, worktree))
                                                }) else {
                                                    return;
                                                };

                                                this.collapse_entry(
                                                    target_entry.clone(),
                                                    worktree,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        },
                                    ))
                                    .with_render_fn(
                                        cx.entity(),
                                        move |this, params, _, cx| {
                                            const LEFT_OFFSET: Pixels = px(14.);
                                            const PADDING_Y: Pixels = px(4.);
                                            const HITBOX_OVERDRAW: Pixels = px(3.);

                                            let active_indent_guide_index = this
                                                .find_active_indent_guide(
                                                    &params.indent_guides,
                                                    cx,
                                                );

                                            let indent_size = params.indent_size;
                                            let item_height = params.item_height;

                                            params
                                                .indent_guides
                                                .into_iter()
                                                .enumerate()
                                                .map(|(idx, layout)| {
                                                    let offset = if layout.continues_offscreen {
                                                        px(0.)
                                                    } else {
                                                        PADDING_Y
                                                    };
                                                    let bounds = Bounds::new(
                                                        point(
                                                            layout.offset.x * indent_size
                                                                + LEFT_OFFSET,
                                                            layout.offset.y * item_height + offset,
                                                        ),
                                                        size(
                                                            px(1.),
                                                            layout.length * item_height
                                                                - offset * 2.,
                                                        ),
                                                    );
                                                    ui::RenderedIndentGuide {
                                                        bounds,
                                                        layout,
                                                        is_active: Some(idx)
                                                            == active_indent_guide_index,
                                                        hitbox: Some(Bounds::new(
                                                            point(
                                                                bounds.origin.x - HITBOX_OVERDRAW,
                                                                bounds.origin.y,
                                                            ),
                                                            size(
                                                                bounds.size.width
                                                                    + HITBOX_OVERDRAW * 2.,
                                                                bounds.size.height,
                                                            ),
                                                        )),
                                                    }
                                                })
                                                .collect()
                                        },
                                    ),
                                )
                            })
                            .when(show_sticky_entries, |list| {
                                let sticky_items = ui::sticky_items(
                                    cx.entity(),
                                    |this, range, window, cx| {
                                        let mut items =
                                            SmallVec::with_capacity(range.end - range.start);
                                        this.iter_visible_entries(
                                            range,
                                            window,
                                            cx,
                                            |entry, index, entries, _, _| {
                                                let (depth, _) =
                                                    Self::calculate_depth_and_difference(
                                                        entry, entries,
                                                    );
                                                let candidate =
                                                    StickyProjectPanelCandidate { index, depth };
                                                items.push(candidate);
                                            },
                                        );
                                        items
                                    },
                                    |this, marker_entry, window, cx| {
                                        let sticky_entries =
                                            this.render_sticky_entries(marker_entry, window, cx);
                                        this.sticky_items_count = sticky_entries.len();
                                        sticky_entries
                                    },
                                );
                                list.with_decoration(if show_indent_guides {
                                    sticky_items.with_decoration(
                                        ui::indent_guides(
                                            px(indent_size),
                                            IndentGuideColors::panel(cx),
                                        )
                                        .with_render_fn(
                                            cx.entity(),
                                            move |_, params, _, _| {
                                                const LEFT_OFFSET: Pixels = px(14.);

                                                let indent_size = params.indent_size;
                                                let item_height = params.item_height;

                                                params
                                                    .indent_guides
                                                    .into_iter()
                                                    .map(|layout| {
                                                        let bounds = Bounds::new(
                                                            point(
                                                                layout.offset.x * indent_size
                                                                    + LEFT_OFFSET,
                                                                layout.offset.y * item_height,
                                                            ),
                                                            size(
                                                                px(1.),
                                                                layout.length * item_height,
                                                            ),
                                                        );
                                                        ui::RenderedIndentGuide {
                                                            bounds,
                                                            layout,
                                                            is_active: false,
                                                            hitbox: None,
                                                        }
                                                    })
                                                    .collect()
                                            },
                                        ),
                                    )
                                } else {
                                    sticky_items
                                })
                            })
                            .with_sizing_behavior(ListSizingBehavior::Infer)
                            .with_horizontal_sizing_behavior(
                                ListHorizontalSizingBehavior::Unconstrained,
                            )
                            .with_width_from_item(self.state.max_width_item_index)
                            .track_scroll(&self.scroll_handle),
                        )
                        .child(
                            div()
                                .id("project-panel-blank-area")
                                .block_mouse_except_scroll()
                                .flex_grow()
                                .when(
                                    self.drag_target_entry.as_ref().is_some_and(
                                        |entry| match entry {
                                            DragTarget::Background => true,
                                            DragTarget::Entry {
                                                highlight_entry_id, ..
                                            } => self.state.last_worktree_root_id.is_some_and(
                                                |root_id| *highlight_entry_id == root_id,
                                            ),
                                        },
                                    ),
                                    |div| div.bg(cx.theme().colors().drop_target_background),
                                )
                                .on_drag_move::<ExternalPaths>(cx.listener(
                                    move |this, event: &DragMoveEvent<ExternalPaths>, _, _| {
                                        let Some(_last_root_id) = this.state.last_worktree_root_id
                                        else {
                                            return;
                                        };
                                        if event.bounds.contains(&event.event.position) {
                                            this.drag_target_entry = Some(DragTarget::Background);
                                        } else {
                                            if this.drag_target_entry.as_ref().is_some_and(|e| {
                                                matches!(e, DragTarget::Background)
                                            }) {
                                                this.drag_target_entry = None;
                                            }
                                        }
                                    },
                                ))
                                .on_drag_move::<DraggedSelection>(cx.listener(
                                    move |this, event: &DragMoveEvent<DraggedSelection>, _, cx| {
                                        let Some(last_root_id) = this.state.last_worktree_root_id
                                        else {
                                            return;
                                        };
                                        if event.bounds.contains(&event.event.position) {
                                            let drag_state = event.drag(cx);
                                            if this.should_highlight_background_for_selection_drag(
                                                &drag_state,
                                                last_root_id,
                                                cx,
                                            ) {
                                                this.drag_target_entry =
                                                    Some(DragTarget::Background);
                                            }
                                        } else {
                                            if this.drag_target_entry.as_ref().is_some_and(|e| {
                                                matches!(e, DragTarget::Background)
                                            }) {
                                                this.drag_target_entry = None;
                                            }
                                        }
                                    },
                                ))
                                .on_drop(cx.listener(
                                    move |this, external_paths: &ExternalPaths, window, cx| {
                                        this.drag_target_entry = None;
                                        this.hover_scroll_task.take();
                                        if let Some(entry_id) = this.state.last_worktree_root_id {
                                            this.drop_external_files(
                                                external_paths.paths(),
                                                entry_id,
                                                window,
                                                cx,
                                            );
                                        }
                                        cx.stop_propagation();
                                    },
                                ))
                                .on_drop(cx.listener(
                                    move |this, selections: &DraggedSelection, window, cx| {
                                        this.drag_target_entry = None;
                                        this.hover_scroll_task.take();
                                        if let Some(entry_id) = this.state.last_worktree_root_id {
                                            this.drag_onto(selections, entry_id, false, window, cx);
                                        }
                                        cx.stop_propagation();
                                    },
                                ))
                                .on_click(cx.listener(|this, event, window, cx| {
                                    if matches!(event, gpui::ClickEvent::Keyboard(_)) {
                                        return;
                                    }
                                    cx.stop_propagation();
                                    this.state.selection = None;
                                    this.marked_entries.clear();
                                    this.focus_handle(cx).focus(window, cx);
                                }))
                                .on_mouse_down(
                                    MouseButton::Right,
                                    cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                                        // When deploying the context menu anywhere below the last project entry,
                                        // act as if the user clicked the root of the last worktree.
                                        if let Some(entry_id) = this.state.last_worktree_root_id {
                                            this.deploy_context_menu(
                                                event.position,
                                                entry_id,
                                                window,
                                                cx,
                                            );
                                        }
                                    }),
                                )
                                .when(!project.is_read_only(cx), |el| {
                                    el.on_click(cx.listener(
                                        |this, event: &gpui::ClickEvent, window, cx| {
                                            if event.click_count() > 1
                                                && let Some(entry_id) =
                                                    this.state.last_worktree_root_id
                                            {
                                                let project = this.project.read(cx);

                                                let worktree_id = if let Some(worktree) =
                                                    project.worktree_for_entry(entry_id, cx)
                                                {
                                                    worktree.read(cx).id()
                                                } else {
                                                    return;
                                                };

                                                this.state.selection = Some(SelectedEntry {
                                                    worktree_id,
                                                    entry_id,
                                                });

                                                this.new_file(&NewFile, window, cx);
                                            }
                                        },
                                    ))
                                }),
                        )
                        .size_full(),
                )
                .custom_scrollbars(
                    Scrollbars::for_settings::<ProjectPanelSettings>()
                        .tracked_scroll_handle(&self.scroll_handle)
                        .with_track_along(
                            ScrollAxes::Horizontal,
                            cx.theme().colors().panel_background,
                        )
                        .notify_content(),
                    window,
                    cx,
                )
                .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                    deferred(
                        anchored()
                            .position(*position)
                            .anchor(gpui::Corner::TopLeft)
                            .child(menu.clone()),
                    )
                    .with_priority(3)
                }))
        } else {
            let focus_handle = self.focus_handle(cx);

            v_flex()
                .id("empty-project_panel")
                .p_4()
                .size_full()
                .items_center()
                .justify_center()
                .gap_1()
                .track_focus(&self.focus_handle(cx))
                .child(
                    Button::new("open_project", "Open Project")
                        .full_width()
                        .key_binding(KeyBinding::for_action_in(
                            &workspace::Open,
                            &focus_handle,
                            cx,
                        ))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.workspace
                                .update(cx, |_, cx| {
                                    window.dispatch_action(workspace::Open.boxed_clone(), cx);
                                })
                                .log_err();
                        })),
                )
                .child(
                    h_flex()
                        .w_1_2()
                        .gap_2()
                        .child(Divider::horizontal())
                        .child(Label::new("or").size(LabelSize::XSmall).color(Color::Muted))
                        .child(Divider::horizontal()),
                )
                .child(
                    Button::new("clone_repo", "Clone Repository")
                        .full_width()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.workspace
                                .update(cx, |_, cx| {
                                    window.dispatch_action(git::Clone.boxed_clone(), cx);
                                })
                                .log_err();
                        })),
                )
                .when(is_local, |div| {
                    div.when(panel_settings.drag_and_drop, |div| {
                        div.drag_over::<ExternalPaths>(|style, _, _, cx| {
                            style.bg(cx.theme().colors().drop_target_background)
                        })
                        .on_drop(cx.listener(
                            move |this, external_paths: &ExternalPaths, window, cx| {
                                this.drag_target_entry = None;
                                this.hover_scroll_task.take();
                                if let Some(task) = this
                                    .workspace
                                    .update(cx, |workspace, cx| {
                                        workspace.open_workspace_for_paths(
                                            true,
                                            external_paths.paths().to_owned(),
                                            window,
                                            cx,
                                        )
                                    })
                                    .log_err()
                                {
                                    task.detach_and_log_err(cx);
                                }
                                cx.stop_propagation();
                            },
                        ))
                    })
                })
        }
    }
}

impl Render for DraggedProjectEntryView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();
        h_flex()
            .font(ui_font)
            .pl(self.click_offset.x + px(12.))
            .pt(self.click_offset.y + px(12.))
            .child(
                div()
                    .flex()
                    .gap_1()
                    .items_center()
                    .py_1()
                    .px_2()
                    .rounded_lg()
                    .bg(cx.theme().colors().background)
                    .map(|this| {
                        if self.selections.len() > 1 && self.selections.contains(&self.selection) {
                            this.child(Label::new(format!("{} entries", self.selections.len())))
                        } else {
                            this.child(if let Some(icon) = &self.icon {
                                div().child(Icon::from_path(icon.clone()))
                            } else {
                                div()
                            })
                            .child(Label::new(self.filename.clone()))
                        }
                    }),
            )
    }
}

impl EventEmitter<Event> for ProjectPanel {}

impl EventEmitter<PanelEvent> for ProjectPanel {}

impl Panel for ProjectPanel {
    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        match ProjectPanelSettings::get_global(cx).dock {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            let dock = match position {
                DockPosition::Left | DockPosition::Bottom => DockSide::Left,
                DockPosition::Right => DockSide::Right,
            };
            settings.project_panel.get_or_insert_default().dock = Some(dock);
        });
    }

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| ProjectPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        });
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        ProjectPanelSettings::get_global(cx)
            .button
            .then_some(IconName::FileTree)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Project Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "Project Panel"
    }

    fn panel_key() -> &'static str {
        PROJECT_PANEL_KEY
    }

    fn starts_open(&self, _: &Window, cx: &App) -> bool {
        if !ProjectPanelSettings::get_global(cx).starts_open {
            return false;
        }

        let project = &self.project.read(cx);
        project.visible_worktrees(cx).any(|tree| {
            tree.read(cx)
                .root_entry()
                .is_some_and(|entry| entry.is_dir())
        })
    }

    fn activation_priority(&self) -> u32 {
        0
    }
}

impl Focusable for ProjectPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ClipboardEntry {
    fn is_cut(&self) -> bool {
        matches!(self, Self::Cut { .. })
    }

    fn items(&self) -> &BTreeSet<SelectedEntry> {
        match self {
            ClipboardEntry::Copied(entries) | ClipboardEntry::Cut(entries) => entries,
        }
    }

    fn into_copy_entry(self) -> Self {
        match self {
            ClipboardEntry::Copied(_) => self,
            ClipboardEntry::Cut(entries) => ClipboardEntry::Copied(entries),
        }
    }
}

#[inline]
fn cmp_directories_first(a: &Entry, b: &Entry) -> cmp::Ordering {
    util::paths::compare_rel_paths((&a.path, a.is_file()), (&b.path, b.is_file()))
}

#[inline]
fn cmp_mixed(a: &Entry, b: &Entry) -> cmp::Ordering {
    util::paths::compare_rel_paths_mixed((&a.path, a.is_file()), (&b.path, b.is_file()))
}

#[inline]
fn cmp_files_first(a: &Entry, b: &Entry) -> cmp::Ordering {
    util::paths::compare_rel_paths_files_first((&a.path, a.is_file()), (&b.path, b.is_file()))
}

#[inline]
fn cmp_with_mode(a: &Entry, b: &Entry, mode: &settings::ProjectPanelSortMode) -> cmp::Ordering {
    match mode {
        settings::ProjectPanelSortMode::DirectoriesFirst => cmp_directories_first(a, b),
        settings::ProjectPanelSortMode::Mixed => cmp_mixed(a, b),
        settings::ProjectPanelSortMode::FilesFirst => cmp_files_first(a, b),
    }
}

pub fn sort_worktree_entries_with_mode(
    entries: &mut [impl AsRef<Entry>],
    mode: settings::ProjectPanelSortMode,
) {
    entries.sort_by(|lhs, rhs| cmp_with_mode(lhs.as_ref(), rhs.as_ref(), &mode));
}

pub fn par_sort_worktree_entries_with_mode(
    entries: &mut Vec<GitEntry>,
    mode: settings::ProjectPanelSortMode,
) {
    entries.par_sort_by(|lhs, rhs| cmp_with_mode(lhs, rhs, &mode));
}

#[cfg(test)]
mod project_panel_tests;
