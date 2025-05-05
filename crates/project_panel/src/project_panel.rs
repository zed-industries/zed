mod project_panel_settings;
mod utils;

use anyhow::{Context as _, Result, anyhow};
use client::{ErrorCode, ErrorExt};
use collections::{BTreeSet, HashMap, hash_map};
use command_palette_hooks::CommandPaletteFilter;
use db::kvp::KEY_VALUE_STORE;
use editor::{
    Editor, EditorEvent, EditorSettings, ShowScrollbar,
    items::{
        entry_diagnostic_aware_icon_decoration_and_color,
        entry_diagnostic_aware_icon_name_and_color, entry_git_aware_label_color,
    },
    scroll::{Autoscroll, ScrollbarAutoHide},
};
use file_icons::FileIcons;
use git::status::GitSummary;
use gpui::{
    Action, AnyElement, App, ArcCow, AsyncWindowContext, Bounds, ClipboardItem, Context,
    DismissEvent, Div, DragMoveEvent, Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable,
    Hsla, InteractiveElement, KeyContext, ListHorizontalSizingBehavior, ListSizingBehavior,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, PromptLevel, Render, ScrollStrategy,
    Stateful, Styled, Subscription, Task, UniformListScrollHandle, WeakEntity, Window, actions,
    anchored, deferred, div, impl_actions, point, px, size, uniform_list,
};
use indexmap::IndexMap;
use language::DiagnosticSeverity;
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use project::{
    Entry, EntryKind, Fs, GitEntry, GitEntryRef, GitTraversal, Project, ProjectEntryId,
    ProjectPath, Worktree, WorktreeId,
    git_store::{GitStoreEvent, git_traversal::ChildEntriesGitIter},
    relativize_path,
};
use project_panel_settings::{
    ProjectPanelDockPosition, ProjectPanelSettings, ShowDiagnostics, ShowIndentGuides,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore, update_settings_file};
use smallvec::SmallVec;
use std::any::TypeId;
use std::{
    cell::OnceCell,
    cmp,
    collections::HashSet,
    ffi::OsStr,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::ThemeSettings;
use ui::{
    Color, ContextMenu, DecoratedIcon, Icon, IconDecoration, IconDecorationKind, IndentGuideColors,
    IndentGuideLayout, KeyBinding, Label, LabelSize, ListItem, ListItemSpacing, Scrollbar,
    ScrollbarState, Tooltip, prelude::*, v_flex,
};
use util::{ResultExt, TakeUntilExt, TryFutureExt, maybe, paths::compare_paths};
use workspace::{
    DraggedSelection, OpenInTerminal, OpenOptions, OpenVisible, PreviewTabsSettings, SelectedEntry,
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{DetachAndPromptErr, NotifyTaskExt},
};
use worktree::CreatedEntry;

const PROJECT_PANEL_KEY: &str = "ProjectPanel";
const NEW_ENTRY_ID: ProjectEntryId = ProjectEntryId::MAX;

pub struct ProjectPanel {
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    // An update loop that keeps incrementing/decrementing scroll offset while there is a dragged entry that's
    // hovered over the start/end of a list.
    hover_scroll_task: Option<Task<()>>,
    visible_entries: Vec<(WorktreeId, Vec<GitEntry>, OnceCell<HashSet<Arc<Path>>>)>,
    /// Maps from leaf project entry ID to the currently selected ancestor.
    /// Relevant only for auto-fold dirs, where a single project panel entry may actually consist of several
    /// project entries (and all non-leaf nodes are guaranteed to be directories).
    ancestors: HashMap<ProjectEntryId, FoldedAncestors>,
    folded_directory_drag_target: Option<FoldedDirectoryDragTarget>,
    last_worktree_root_id: Option<ProjectEntryId>,
    last_selection_drag_over_entry: Option<ProjectEntryId>,
    last_external_paths_drag_over_entry: Option<ProjectEntryId>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
    unfolded_dir_ids: HashSet<ProjectEntryId>,
    // Currently selected leaf entry (see auto-folding for a definition of that) in a file tree
    selection: Option<SelectedEntry>,
    marked_entries: BTreeSet<SelectedEntry>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    edit_state: Option<EditState>,
    filename_editor: Entity<Editor>,
    clipboard: Option<ClipboardEntry>,
    _dragged_entry_destination: Option<Arc<Path>>,
    workspace: WeakEntity<Workspace>,
    width: Option<Pixels>,
    pending_serialization: Task<Option<()>>,
    show_scrollbar: bool,
    vertical_scrollbar_state: ScrollbarState,
    horizontal_scrollbar_state: ScrollbarState,
    hide_scrollbar_task: Option<Task<()>>,
    diagnostics: HashMap<(WorktreeId, PathBuf), DiagnosticSeverity>,
    max_width_item_index: Option<usize>,
    // We keep track of the mouse down state on entries so we don't flash the UI
    // in case a user clicks to open a file.
    mouse_down: bool,
    hover_expand_task: Option<Task<()>>,
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
    processing_filename: Option<String>,
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
    path: Arc<Path>,
    depth: usize,
    kind: EntryKind,
    is_ignored: bool,
    is_expanded: bool,
    is_selected: bool,
    is_marked: bool,
    is_editing: bool,
    is_processing: bool,
    is_cut: bool,
    filename_text_color: Color,
    diagnostic_severity: Option<DiagnosticSeverity>,
    git_status: GitSummary,
    is_private: bool,
    worktree_id: WorktreeId,
    canonical_path: Option<Arc<Path>>,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Delete {
    #[serde(default)]
    pub skip_prompt: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Trash {
    #[serde(default)]
    pub skip_prompt: bool,
}

impl_actions!(project_panel, [Delete, Trash]);

actions!(
    project_panel,
    [
        ExpandSelectedEntry,
        CollapseSelectedEntry,
        CollapseAllEntries,
        NewDirectory,
        NewFile,
        Copy,
        Duplicate,
        RevealInFileManager,
        RemoveFromProject,
        OpenWithSystem,
        Cut,
        Paste,
        Rename,
        Open,
        OpenPermanent,
        ToggleFocus,
        ToggleHideGitIgnore,
        NewSearchInDirectory,
        UnfoldDirectory,
        FoldDirectory,
        SelectParent,
        SelectNextGitEntry,
        SelectPrevGitEntry,
        SelectNextDiagnostic,
        SelectPrevDiagnostic,
        SelectNextDirectory,
        SelectPrevDirectory,
    ]
);

#[derive(Debug, Default)]
struct FoldedAncestors {
    current_ancestor_depth: usize,
    ancestors: Vec<ProjectEntryId>,
}

impl FoldedAncestors {
    fn max_ancestor_depth(&self) -> usize {
        self.ancestors.len()
    }
}

pub fn init_settings(cx: &mut App) {
    ProjectPanelSettings::register(cx);
}

pub fn init(cx: &mut App) {
    init_settings(cx);

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<ProjectPanel>(window, cx);
        });

        workspace.register_action(|workspace, _: &ToggleHideGitIgnore, _, cx| {
            let fs = workspace.app_state().fs.clone();
            update_settings_file::<ProjectPanelSettings>(fs, cx, move |setting, _| {
                setting.hide_gitignore = Some(!setting.hide_gitignore.unwrap_or(false));
            })
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
    },
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedProjectPanel {
    width: Option<Pixels>,
}

struct DraggedProjectEntryView {
    selection: SelectedEntry,
    details: EntryDetails,
    click_offset: Point<Pixels>,
    selections: Arc<BTreeSet<SelectedEntry>>,
}

struct ItemColors {
    default: Hsla,
    hover: Hsla,
    drag_over: Hsla,
    marked: Hsla,
    focused: Hsla,
}

fn get_item_color(cx: &App) -> ItemColors {
    let colors = cx.theme().colors();

    ItemColors {
        default: colors.panel_background,
        hover: colors.element_hover,
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
        let project_panel = cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                this.focus_out(window, cx);
                this.hide_scrollbar(window, cx);
            })
            .detach();

            cx.subscribe(&git_store, |this, _, event, cx| match event {
                GitStoreEvent::RepositoryUpdated(_, _, _)
                | GitStoreEvent::RepositoryAdded(_)
                | GitStoreEvent::RepositoryRemoved(_) => {
                    this.update_visible_entries(None, cx);
                    cx.notify();
                }
                _ => {}
            })
            .detach();

            cx.subscribe(&project, |this, project, event, cx| match event {
                project::Event::ActiveEntryChanged(Some(entry_id)) => {
                    if ProjectPanelSettings::get_global(cx).auto_reveal_entries {
                        this.reveal_entry(project.clone(), *entry_id, true, cx).ok();
                    }
                }
                project::Event::ActiveEntryChanged(None) => {
                    this.marked_entries.clear();
                }
                project::Event::RevealInProjectPanel(entry_id) => {
                    if let Some(()) = this
                        .reveal_entry(project.clone(), *entry_id, false, cx)
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
                    if ProjectPanelSettings::get_global(cx).show_diagnostics != ShowDiagnostics::Off
                    {
                        this.update_diagnostics(cx);
                        cx.notify();
                    }
                }
                project::Event::WorktreeRemoved(id) => {
                    this.expanded_dir_ids.remove(id);
                    this.update_visible_entries(None, cx);
                    cx.notify();
                }
                project::Event::WorktreeUpdatedEntries(_, _)
                | project::Event::WorktreeAdded(_)
                | project::Event::WorktreeOrderChanged => {
                    this.update_visible_entries(None, cx);
                    cx.notify();
                }
                project::Event::ExpandedAllForEntry(worktree_id, entry_id) => {
                    if let Some((worktree, expanded_dir_ids)) = project
                        .read(cx)
                        .worktree_for_id(*worktree_id, cx)
                        .zip(this.expanded_dir_ids.get_mut(&worktree_id))
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
                                if !child.is_dir() || (include_ignored_dirs && child.is_ignored) {
                                    continue;
                                }

                                dirs_to_expand.push(child.id);

                                if let Err(ix) = expanded_dir_ids.binary_search(&child.id) {
                                    expanded_dir_ids.insert(ix, child.id);
                                }
                                this.unfolded_dir_ids.insert(child.id);
                            }
                        }
                        this.update_visible_entries(None, cx);
                        cx.notify();
                    }
                }
                _ => {}
            })
            .detach();

            let trash_action = [TypeId::of::<Trash>()];
            let is_remote = project.read(cx).is_via_collab();

            if is_remote {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.hide_action_types(&trash_action);
                });
            }

            let filename_editor = cx.new(|cx| Editor::single_line(window, cx));

            cx.subscribe(
                &filename_editor,
                |project_panel, _, editor_event, cx| match editor_event {
                    EditorEvent::BufferEdited => {
                        project_panel.populate_validation_error(cx);
                        project_panel.autoscroll(cx);
                    }
                    EditorEvent::SelectionsChanged { .. } => {
                        project_panel.autoscroll(cx);
                    }
                    EditorEvent::Blurred => {
                        if project_panel
                            .edit_state
                            .as_ref()
                            .map_or(false, |state| state.processing_filename.is_none())
                        {
                            project_panel.edit_state = None;
                            project_panel.update_visible_entries(None, cx);
                            cx.notify();
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
            cx.observe_global::<SettingsStore>(move |this, cx| {
                let new_settings = *ProjectPanelSettings::get_global(cx);
                if project_panel_settings != new_settings {
                    if project_panel_settings.hide_gitignore != new_settings.hide_gitignore {
                        this.update_visible_entries(None, cx);
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
                visible_entries: Default::default(),
                ancestors: Default::default(),
                folded_directory_drag_target: None,
                last_worktree_root_id: Default::default(),
                last_external_paths_drag_over_entry: None,
                last_selection_drag_over_entry: None,
                expanded_dir_ids: Default::default(),
                unfolded_dir_ids: Default::default(),
                selection: None,
                marked_entries: Default::default(),
                edit_state: None,
                context_menu: None,
                filename_editor,
                clipboard: None,
                _dragged_entry_destination: None,
                workspace: workspace.weak_handle(),
                width: None,
                pending_serialization: Task::ready(None),
                show_scrollbar: !Self::should_autohide_scrollbar(cx),
                hide_scrollbar_task: None,
                vertical_scrollbar_state: ScrollbarState::new(scroll_handle.clone())
                    .parent_entity(&cx.entity()),
                horizontal_scrollbar_state: ScrollbarState::new(scroll_handle.clone())
                    .parent_entity(&cx.entity()),
                max_width_item_index: None,
                diagnostics: Default::default(),
                scroll_handle,
                mouse_down: false,
                hover_expand_task: None,
            };
            this.update_visible_entries(None, cx);

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
                    if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx) {
                        if let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                            let file_path = entry.path.clone();
                            let worktree_id = worktree.read(cx).id();
                            let entry_id = entry.id;
                            let is_via_ssh = project.read(cx).is_via_ssh();

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
                                            file_path.display()
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
                                    project_panel.marked_entries.insert(entry);
                                    project_panel.selection = Some(entry);
                                });
                                if !focus_opened_item {
                                    let focus_handle = project_panel.read(cx).focus_handle.clone();
                                    window.focus(&focus_handle);
                                }
                            }
                        }
                    }
                }
                &Event::SplitEntry { entry_id } => {
                    if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx) {
                        if let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                            workspace
                                .split_path(
                                    ProjectPath {
                                        worktree_id: worktree.read(cx).id(),
                                        path: entry.path.clone(),
                                    },
                                    window, cx,
                                )
                                .detach_and_log_err(cx);
                        }
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
        let serialized_panel = cx
            .background_spawn(async move { KEY_VALUE_STORE.read_kvp(PROJECT_PANEL_KEY) })
            .await
            .map_err(|e| anyhow!("Failed to load project panel: {}", e))
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedProjectPanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

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
        let mut diagnostics: HashMap<(WorktreeId, PathBuf), DiagnosticSeverity> =
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
                    let mut path_buffer = PathBuf::new();
                    Self::update_strongest_diagnostic_severity(
                        &mut diagnostics,
                        &project_path,
                        path_buffer.clone(),
                        diagnostic_severity,
                    );

                    for component in project_path.path.components() {
                        path_buffer.push(component);
                        Self::update_strongest_diagnostic_severity(
                            &mut diagnostics,
                            &project_path,
                            path_buffer.clone(),
                            diagnostic_severity,
                        );
                    }
                });
        }
        self.diagnostics = diagnostics;
    }

    fn update_strongest_diagnostic_severity(
        diagnostics: &mut HashMap<(WorktreeId, PathBuf), DiagnosticSeverity>,
        project_path: &ProjectPath,
        path_buffer: PathBuf,
        diagnostic_severity: DiagnosticSeverity,
    ) {
        diagnostics
            .entry((project_path.worktree_id, path_buffer.clone()))
            .and_modify(|strongest_diagnostic_severity| {
                *strongest_diagnostic_severity =
                    cmp::min(*strongest_diagnostic_severity, diagnostic_severity);
            })
            .or_insert(diagnostic_severity);
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        PROJECT_PANEL_KEY.into(),
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

    fn focus_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.is_focused(window) {
            self.confirm(&Confirm, window, cx);
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

        self.selection = Some(SelectedEntry {
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
            let is_remote = project.is_via_collab();
            let is_local = project.is_local();

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
                            .separator()
                            .action("Cut", Box::new(Cut))
                            .action("Copy", Box::new(Copy))
                            .action("Duplicate", Box::new(Duplicate))
                            // TODO: Paste should always be visible, cbut disabled when clipboard is empty
                            .map(|menu| {
                                if self.clipboard.as_ref().is_some() {
                                    menu.action("Paste", Box::new(Paste))
                                } else {
                                    menu.disabled_action("Paste", Box::new(Paste))
                                }
                            })
                            .separator()
                            .action("Copy Path", Box::new(zed_actions::workspace::CopyPath))
                            .action(
                                "Copy Relative Path",
                                Box::new(zed_actions::workspace::CopyRelativePath),
                            )
                            .separator()
                            .when(!is_root || !cfg!(target_os = "windows"), |menu| {
                                menu.action("Rename", Box::new(Rename))
                            })
                            .when(!is_root & !is_remote, |menu| {
                                menu.action("Trash", Box::new(Trash { skip_prompt: false }))
                            })
                            .when(!is_root, |menu| {
                                menu.action("Delete", Box::new(Delete { skip_prompt: false }))
                            })
                            .when(!is_remote & is_root, |menu| {
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

            window.focus(&context_menu.focus_handle(cx));
            let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                this.context_menu.take();
                cx.notify();
            });
            self.context_menu = Some((context_menu, position, subscription));
        }

        cx.notify();
    }

    fn is_unfoldable(&self, entry: &Entry, worktree: &Worktree) -> bool {
        if !entry.is_dir() || self.unfolded_dir_ids.contains(&entry.id) {
            return false;
        }

        if let Some(parent_path) = entry.path.parent() {
            let snapshot = worktree.snapshot();
            let mut child_entries = snapshot.child_entries(parent_path);
            if let Some(child) = child_entries.next() {
                if child_entries.next().is_none() {
                    return child.kind.is_dir();
                }
            }
        };
        false
    }

    fn is_foldable(&self, entry: &Entry, worktree: &Worktree) -> bool {
        if entry.is_dir() {
            let snapshot = worktree.snapshot();

            let mut child_entries = snapshot.child_entries(&entry.path);
            if let Some(child) = child_entries.next() {
                if child_entries.next().is_none() {
                    return child.kind.is_dir();
                }
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
            if let Some(folded_ancestors) = self.ancestors.get_mut(&entry.id) {
                if folded_ancestors.current_ancestor_depth > 0 {
                    folded_ancestors.current_ancestor_depth -= 1;
                    cx.notify();
                    return;
                }
            }
            if entry.is_dir() {
                let worktree_id = worktree.id();
                let entry_id = entry.id;
                let expanded_dir_ids =
                    if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
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
                        self.update_visible_entries(None, cx);
                        cx.notify();
                    }
                }
            }
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((worktree, entry)) = self.selected_entry_handle(cx) else {
            return;
        };
        self.collapse_entry(entry.clone(), worktree, cx)
    }

    fn collapse_entry(&mut self, entry: Entry, worktree: Entity<Worktree>, cx: &mut Context<Self>) {
        let worktree = worktree.read(cx);
        if let Some(folded_ancestors) = self.ancestors.get_mut(&entry.id) {
            if folded_ancestors.current_ancestor_depth + 1 < folded_ancestors.max_ancestor_depth() {
                folded_ancestors.current_ancestor_depth += 1;
                cx.notify();
                return;
            }
        }
        let worktree_id = worktree.id();
        let expanded_dir_ids =
            if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
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
                    self.update_visible_entries(Some((worktree_id, entry_id)), cx);
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
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // By keeping entries for fully collapsed worktrees, we avoid expanding them within update_visible_entries
        // (which is it's default behavior when there's no entry for a worktree in expanded_dir_ids).
        self.expanded_dir_ids
            .retain(|_, expanded_entries| expanded_entries.is_empty());
        self.update_visible_entries(None, cx);
        cx.notify();
    }

    fn toggle_expanded(
        &mut self,
        entry_id: ProjectEntryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx) {
            if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
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
                self.update_visible_entries(Some((worktree_id, entry_id)), cx);
                window.focus(&self.focus_handle);
                cx.notify();
            }
        }
    }

    fn toggle_expand_all(
        &mut self,
        entry_id: ProjectEntryId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx) {
            if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                match expanded_dir_ids.binary_search(&entry_id) {
                    Ok(_ix) => {
                        self.collapse_all_for_entry(worktree_id, entry_id, cx);
                    }
                    Err(_ix) => {
                        self.expand_all_for_entry(worktree_id, entry_id, cx);
                    }
                }
                self.update_visible_entries(Some((worktree_id, entry_id)), cx);
                window.focus(&self.focus_handle);
                cx.notify();
            }
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
                .zip(self.expanded_dir_ids.get_mut(&worktree_id))
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
                .zip(self.expanded_dir_ids.get_mut(&worktree_id))
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
                        self.unfolded_dir_ids.remove(&current_id);
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
        if let Some(edit_state) = &self.edit_state {
            if edit_state.processing_filename.is_none() {
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
        }
        if let Some(selection) = self.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if entry_ix > 0 {
                entry_ix -= 1;
            } else if worktree_ix > 0 {
                worktree_ix -= 1;
                entry_ix = self.visible_entries[worktree_ix].1.len() - 1;
            } else {
                return;
            }

            let (worktree_id, worktree_entries, _) = &self.visible_entries[worktree_ix];
            let selection = SelectedEntry {
                worktree_id: *worktree_id,
                entry_id: worktree_entries[entry_ix].id,
            };
            self.selection = Some(selection);
            if window.modifiers().shift {
                self.marked_entries.insert(selection);
            }
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst {}, window, cx);
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(task) = self.confirm_edit(window, cx) {
            task.detach_and_notify_err(window, cx);
        }
    }

    fn open(&mut self, _: &Open, window: &mut Window, cx: &mut Context<Self>) {
        let preview_tabs_enabled = PreviewTabsSettings::get_global(cx).enabled;
        self.open_internal(true, !preview_tabs_enabled, window, cx);
    }

    fn open_permanent(&mut self, _: &OpenPermanent, window: &mut Window, cx: &mut Context<Self>) {
        self.open_internal(false, true, window, cx);
    }

    fn open_internal(
        &mut self,
        allow_preview: bool,
        focus_opened_item: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((_, entry)) = self.selected_entry(cx) {
            if entry.is_file() {
                self.open_entry(entry.id, focus_opened_item, allow_preview, cx);
                cx.notify();
            } else {
                self.toggle_expanded(entry.id, window, cx);
            }
        }
    }

    fn populate_validation_error(&mut self, cx: &mut Context<Self>) {
        let edit_state = match self.edit_state.as_mut() {
            Some(state) => state,
            None => return,
        };
        let filename = self.filename_editor.read(cx).text(cx);
        if !filename.is_empty() {
            if let Some(worktree) = self
                .project
                .read(cx)
                .worktree_for_id(edit_state.worktree_id, cx)
            {
                if let Some(entry) = worktree.read(cx).entry_for_id(edit_state.entry_id) {
                    let mut already_exists = false;
                    if edit_state.is_new_entry() {
                        let new_path = entry.path.join(filename.trim_start_matches('/'));
                        if worktree
                            .read(cx)
                            .entry_for_path(new_path.as_path())
                            .is_some()
                        {
                            already_exists = true;
                        }
                    } else {
                        let new_path = if let Some(parent) = entry.path.clone().parent() {
                            parent.join(&filename)
                        } else {
                            filename.clone().into()
                        };
                        if let Some(existing) = worktree.read(cx).entry_for_path(new_path.as_path())
                        {
                            if existing.id != entry.id {
                                already_exists = true;
                            }
                        }
                    };
                    if already_exists {
                        edit_state.validation_state = ValidationState::Error(format!(
                            "File or directory '{}' already exists at location. Please choose a different name.",
                            filename
                        ));
                        cx.notify();
                        return;
                    }
                }
            }
            let trimmed_filename = filename.trim();
            if trimmed_filename.is_empty() {
                edit_state.validation_state =
                    ValidationState::Error("File or directory name cannot be empty.".to_string());
                cx.notify();
                return;
            }
            if trimmed_filename != filename {
                edit_state.validation_state = ValidationState::Warning(
                    "File or directory name contains leading or trailing whitespace.".to_string(),
                );
                cx.notify();
                return;
            }
        }
        edit_state.validation_state = ValidationState::None;
        cx.notify();
    }

    fn confirm_edit(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let edit_state = self.edit_state.as_mut()?;
        let worktree_id = edit_state.worktree_id;
        let is_new_entry = edit_state.is_new_entry();
        let filename = self.filename_editor.read(cx).text(cx);
        if filename.trim().is_empty() {
            return None;
        }
        #[cfg(not(target_os = "windows"))]
        let filename_indicates_dir = filename.ends_with("/");
        // On Windows, path separator could be either `/` or `\`.
        #[cfg(target_os = "windows")]
        let filename_indicates_dir = filename.ends_with("/") || filename.ends_with("\\");
        edit_state.is_dir =
            edit_state.is_dir || (edit_state.is_new_entry() && filename_indicates_dir);
        let is_dir = edit_state.is_dir;
        let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(edit_state.entry_id)?.clone();

        let edit_task;
        let edited_entry_id;
        if is_new_entry {
            self.selection = Some(SelectedEntry {
                worktree_id,
                entry_id: NEW_ENTRY_ID,
            });
            let new_path = entry.path.join(filename.trim_start_matches('/'));
            if worktree
                .read(cx)
                .entry_for_path(new_path.as_path())
                .is_some()
            {
                return None;
            }

            edited_entry_id = NEW_ENTRY_ID;
            edit_task = self.project.update(cx, |project, cx| {
                project.create_entry((worktree_id, &new_path), is_dir, cx)
            });
        } else {
            let new_path = if let Some(parent) = entry.path.clone().parent() {
                parent.join(&filename)
            } else {
                filename.clone().into()
            };
            if let Some(existing) = worktree.read(cx).entry_for_path(new_path.as_path()) {
                if existing.id == entry.id {
                    window.focus(&self.focus_handle);
                }
                return None;
            }
            edited_entry_id = entry.id;
            edit_task = self.project.update(cx, |project, cx| {
                project.rename_entry(entry.id, new_path.as_path(), cx)
            });
        };

        window.focus(&self.focus_handle);
        edit_state.processing_filename = Some(filename);
        cx.notify();

        Some(cx.spawn_in(window, async move |project_panel, cx| {
            let new_entry = edit_task.await;
            project_panel.update(cx, |project_panel, cx| {
                project_panel.edit_state = None;
                cx.notify();
            })?;

            match new_entry {
                Err(e) => {
                    project_panel.update( cx, |project_panel, cx| {
                        project_panel.marked_entries.clear();
                        project_panel.update_visible_entries(None,  cx);
                    }).ok();
                    Err(e)?;
                }
                Ok(CreatedEntry::Included(new_entry)) => {
                    project_panel.update( cx, |project_panel, cx| {
                        if let Some(selection) = &mut project_panel.selection {
                            if selection.entry_id == edited_entry_id {
                                selection.worktree_id = worktree_id;
                                selection.entry_id = new_entry.id;
                                project_panel.marked_entries.clear();
                                project_panel.expand_to_selection(cx);
                            }
                        }
                        project_panel.update_visible_entries(None, cx);
                        if is_new_entry && !is_dir {
                            project_panel.open_entry(new_entry.id, true, false, cx);
                        }
                        cx.notify();
                    })?;
                }
                Ok(CreatedEntry::Excluded { abs_path }) => {
                    if let Some(open_task) = project_panel
                        .update_in( cx, |project_panel, window, cx| {
                            project_panel.marked_entries.clear();
                            project_panel.update_visible_entries(None,  cx);

                            if is_dir {
                                project_panel.project.update(cx, |_, cx| {
                                    cx.emit(project::Event::Toast {
                                        notification_id: "excluded-directory".into(),
                                        message: format!("Created an excluded directory at {abs_path:?}.\nAlter `file_scan_exclusions` in the settings to show it in the panel")
                                    })
                                });
                                None
                            } else {
                                project_panel
                                    .workspace
                                    .update(cx, |workspace, cx| {
                                        workspace.open_abs_path(abs_path, OpenOptions { visible: Some(OpenVisible::All), ..Default::default() }, window, cx)
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
        let previous_edit_state = self.edit_state.take();
        self.update_visible_entries(None, cx);
        self.marked_entries.clear();

        if let Some(previously_focused) =
            previous_edit_state.and_then(|edit_state| edit_state.previously_focused)
        {
            self.selection = Some(previously_focused);
            self.autoscroll(cx);
        }

        window.focus(&self.focus_handle);
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

    fn split_entry(&mut self, entry_id: ProjectEntryId, cx: &mut Context<Self>) {
        cx.emit(Event::SplitEntry { entry_id });
    }

    fn new_file(&mut self, _: &NewFile, window: &mut Window, cx: &mut Context<Self>) {
        self.add_entry(false, window, cx)
    }

    fn new_directory(&mut self, _: &NewDirectory, window: &mut Window, cx: &mut Context<Self>) {
        self.add_entry(true, window, cx)
    }

    fn add_entry(&mut self, is_dir: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some((worktree_id, entry_id)) = self
            .selection
            .map(|entry| (entry.worktree_id, entry.entry_id))
            .or_else(|| {
                let entry_id = self.last_worktree_root_id?;
                let worktree_id = self
                    .project
                    .read(cx)
                    .worktree_for_entry(entry_id, cx)?
                    .read(cx)
                    .id();

                self.selection = Some(SelectedEntry {
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
            .zip(self.expanded_dir_ids.get_mut(&worktree_id))
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
                        if let Some(parent_path) = entry.path.parent() {
                            if let Some(parent_entry) = worktree.entry_for_path(parent_path) {
                                entry = parent_entry;
                                continue;
                            }
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
        self.edit_state = Some(EditState {
            worktree_id,
            entry_id: directory_id,
            leaf_entry_id: None,
            is_dir,
            processing_filename: None,
            previously_focused: self.selection,
            depth: 0,
            validation_state: ValidationState::None,
        });
        self.filename_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
            window.focus(&editor.focus_handle(cx));
        });
        self.update_visible_entries(Some((worktree_id, NEW_ENTRY_ID)), cx);
        self.autoscroll(cx);
        cx.notify();
    }

    fn unflatten_entry_id(&self, leaf_entry_id: ProjectEntryId) -> ProjectEntryId {
        if let Some(ancestors) = self.ancestors.get(&leaf_entry_id) {
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
        }) = self.selection
        {
            if let Some(worktree) = self.project.read(cx).worktree_for_id(worktree_id, cx) {
                let sub_entry_id = self.unflatten_entry_id(entry_id);
                if let Some(entry) = worktree.read(cx).entry_for_id(sub_entry_id) {
                    #[cfg(target_os = "windows")]
                    if Some(entry) == worktree.read(cx).root_entry() {
                        return;
                    }
                    self.edit_state = Some(EditState {
                        worktree_id,
                        entry_id: sub_entry_id,
                        leaf_entry_id: Some(entry_id),
                        is_dir: entry.is_dir(),
                        processing_filename: None,
                        previously_focused: None,
                        depth: 0,
                        validation_state: ValidationState::None,
                    });
                    let file_name = entry
                        .path
                        .file_name()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default()
                        .to_string();
                    let selection = selection.unwrap_or_else(|| {
                        let file_stem = entry.path.file_stem().map(|s| s.to_string_lossy());
                        let selection_end =
                            file_stem.map_or(file_name.len(), |file_stem| file_stem.len());
                        0..selection_end
                    });
                    self.filename_editor.update(cx, |editor, cx| {
                        editor.set_text(file_name, window, cx);
                        editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
                            s.select_ranges([selection])
                        });
                        window.focus(&editor.focus_handle(cx));
                    });
                    self.update_visible_entries(None, cx);
                    self.autoscroll(cx);
                    cx.notify();
                }
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
                        project_path
                            .path
                            .file_name()?
                            .to_string_lossy()
                            .into_owned(),
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
                if let Some(answer) = answer {
                    if answer.await != Ok(0) {
                        return anyhow::Ok(());
                    }
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
                        panel.selection = Some(next_selection);
                        panel.autoscroll(cx);
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
                    (Some(a), Some(b)) => {
                        compare_paths((&a.path, a.is_file()), (&b.path, b.is_file()))
                    }
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

        project::sort_worktree_entries(&mut siblings);
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

    fn unfold_directory(&mut self, _: &UnfoldDirectory, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.unfolded_dir_ids.insert(entry.id);

            let snapshot = worktree.snapshot();
            let mut parent_path = entry.path.parent();
            while let Some(path) = parent_path {
                if let Some(parent_entry) = worktree.entry_for_path(path) {
                    let mut children_iter = snapshot.child_entries(path);

                    if children_iter.by_ref().take(2).count() > 1 {
                        break;
                    }

                    self.unfolded_dir_ids.insert(parent_entry.id);
                    parent_path = path.parent();
                } else {
                    break;
                }
            }

            self.update_visible_entries(None, cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn fold_directory(&mut self, _: &FoldDirectory, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.unfolded_dir_ids.remove(&entry.id);

            let snapshot = worktree.snapshot();
            let mut path = &*entry.path;
            loop {
                let mut child_entries_iter = snapshot.child_entries(path);
                if let Some(child) = child_entries_iter.next() {
                    if child_entries_iter.next().is_none() && child.is_dir() {
                        self.unfolded_dir_ids.remove(&child.id);
                        path = &*child.path;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            self.update_visible_entries(None, cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(edit_state) = &self.edit_state {
            if edit_state.processing_filename.is_none() {
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
        }
        if let Some(selection) = self.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if let Some((_, worktree_entries, _)) = self.visible_entries.get(worktree_ix) {
                if entry_ix + 1 < worktree_entries.len() {
                    entry_ix += 1;
                } else {
                    worktree_ix += 1;
                    entry_ix = 0;
                }
            }

            if let Some((worktree_id, worktree_entries, _)) = self.visible_entries.get(worktree_ix)
            {
                if let Some(entry) = worktree_entries.get(entry_ix) {
                    let selection = SelectedEntry {
                        worktree_id: *worktree_id,
                        entry_id: entry.id,
                    };
                    self.selection = Some(selection);
                    if window.modifiers().shift {
                        self.marked_entries.insert(selection);
                    }

                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        } else {
            self.select_first(&SelectFirst {}, window, cx);
        }
    }

    fn select_prev_diagnostic(
        &mut self,
        _: &SelectPrevDiagnostic,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.selection.as_ref(),
            true,
            |entry, worktree_id| {
                (self.selection.is_none()
                    || self.selection.is_some_and(|selection| {
                        if selection.worktree_id == worktree_id {
                            selection.entry_id != entry.id
                        } else {
                            true
                        }
                    }))
                    && entry.is_file()
                    && self
                        .diagnostics
                        .contains_key(&(worktree_id, entry.path.to_path_buf()))
            },
            cx,
        );

        if let Some(selection) = selection {
            self.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(Some((selection.worktree_id, selection.entry_id)), cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_next_diagnostic(
        &mut self,
        _: &SelectNextDiagnostic,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.selection.as_ref(),
            false,
            |entry, worktree_id| {
                (self.selection.is_none()
                    || self.selection.is_some_and(|selection| {
                        if selection.worktree_id == worktree_id {
                            selection.entry_id != entry.id
                        } else {
                            true
                        }
                    }))
                    && entry.is_file()
                    && self
                        .diagnostics
                        .contains_key(&(worktree_id, entry.path.to_path_buf()))
            },
            cx,
        );

        if let Some(selection) = selection {
            self.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(Some((selection.worktree_id, selection.entry_id)), cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_prev_git_entry(
        &mut self,
        _: &SelectPrevGitEntry,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.selection.as_ref(),
            true,
            |entry, worktree_id| {
                (self.selection.is_none()
                    || self.selection.is_some_and(|selection| {
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
            self.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(Some((selection.worktree_id, selection.entry_id)), cx);
            self.autoscroll(cx);
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
            self.selection.as_ref(),
            true,
            |entry, worktree_id| {
                (self.selection.is_none()
                    || self.selection.is_some_and(|selection| {
                        if selection.worktree_id == worktree_id {
                            selection.entry_id != entry.id
                        } else {
                            true
                        }
                    }))
                    && entry.is_dir()
            },
            cx,
        );

        if let Some(selection) = selection {
            self.selection = Some(selection);
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
            self.selection.as_ref(),
            false,
            |entry, worktree_id| {
                (self.selection.is_none()
                    || self.selection.is_some_and(|selection| {
                        if selection.worktree_id == worktree_id {
                            selection.entry_id != entry.id
                        } else {
                            true
                        }
                    }))
                    && entry.is_dir()
            },
            cx,
        );

        if let Some(selection) = selection {
            self.selection = Some(selection);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_next_git_entry(
        &mut self,
        _: &SelectNextGitEntry,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.find_entry(
            self.selection.as_ref(),
            false,
            |entry, worktree_id| {
                (self.selection.is_none()
                    || self.selection.is_some_and(|selection| {
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
            self.selection = Some(selection);
            self.expand_entry(selection.worktree_id, selection.entry_id, cx);
            self.update_visible_entries(Some((selection.worktree_id, selection.entry_id)), cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn select_parent(&mut self, _: &SelectParent, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            if let Some(parent) = entry.path.parent() {
                let worktree = worktree.read(cx);
                if let Some(parent_entry) = worktree.entry_for_path(parent) {
                    self.selection = Some(SelectedEntry {
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
        let worktree = self
            .visible_entries
            .first()
            .and_then(|(worktree_id, _, _)| {
                self.project.read(cx).worktree_for_id(*worktree_id, cx)
            });
        if let Some(worktree) = worktree {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            if let Some(root_entry) = worktree.root_entry() {
                let selection = SelectedEntry {
                    worktree_id,
                    entry_id: root_entry.id,
                };
                self.selection = Some(selection);
                if window.modifiers().shift {
                    self.marked_entries.insert(selection);
                }
                self.autoscroll(cx);
                cx.notify();
            }
        }
    }

    fn select_last(&mut self, _: &SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree_id, visible_worktree_entries, _)) = self.visible_entries.last() {
            let worktree = self.project.read(cx).worktree_for_id(*worktree_id, cx);
            if let (Some(worktree), Some(entry)) = (worktree, visible_worktree_entries.last()) {
                let worktree = worktree.read(cx);
                if let Some(entry) = worktree.entry_for_id(entry.id) {
                    let selection = SelectedEntry {
                        worktree_id: *worktree_id,
                        entry_id: entry.id,
                    };
                    self.selection = Some(selection);
                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        }
    }

    fn autoscroll(&mut self, cx: &mut Context<Self>) {
        if let Some((_, _, index)) = self.selection.and_then(|s| self.index_for_selection(s)) {
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Center);
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
    ) -> Option<(PathBuf, Option<Range<usize>>)> {
        let mut new_path = target_entry.path.to_path_buf();
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
            .to_os_string();
        new_path.push(&clipboard_entry_file_name);
        let extension = new_path.extension().map(|e| e.to_os_string());
        let file_name_without_extension = Path::new(&clipboard_entry_file_name).file_stem()?;
        let file_name_len = file_name_without_extension.to_string_lossy().len();
        let mut disambiguation_range = None;
        let mut ix = 0;
        {
            let worktree = worktree.read(cx);
            while worktree.entry_for_path(&new_path).is_some() {
                new_path.pop();

                let mut new_file_name = file_name_without_extension.to_os_string();

                let disambiguation = " copy";
                let mut disambiguation_len = disambiguation.len();

                new_file_name.push(disambiguation);

                if ix > 0 {
                    let extra_disambiguation = format!(" {}", ix);
                    disambiguation_len += extra_disambiguation.len();

                    new_file_name.push(extra_disambiguation);
                }
                if let Some(extension) = extension.as_ref() {
                    new_file_name.push(".");
                    new_file_name.push(extension);
                }

                new_path.push(new_file_name);
                disambiguation_range = Some(file_name_len..(file_name_len + disambiguation_len));
                ix += 1;
            }
        }
        Some((new_path, disambiguation_range))
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
            let mut paste_entry_tasks: IndexMap<(ProjectEntryId, bool), PasteTask> =
                IndexMap::default();
            let mut disambiguation_range = None;
            let clip_is_cut = clipboard_entries.is_cut();
            for clipboard_entry in clipboard_entries.items() {
                let (new_path, new_disambiguation_range) =
                    self.create_paste_path(clipboard_entry, self.selected_sub_entry(cx)?, cx)?;
                let clip_entry_id = clipboard_entry.entry_id;
                let is_same_worktree = clipboard_entry.worktree_id == worktree_id;
                let relative_worktree_source_path = if !is_same_worktree {
                    let target_base_path = worktree.read(cx).abs_path();
                    let clipboard_project_path =
                        self.project.read(cx).path_for_entry(clip_entry_id, cx)?;
                    let clipboard_abs_path = self
                        .project
                        .read(cx)
                        .absolute_path(&clipboard_project_path, cx)?;
                    Some(relativize_path(
                        &target_base_path,
                        clipboard_abs_path.as_path(),
                    ))
                } else {
                    None
                };
                let task = if clip_is_cut && is_same_worktree {
                    let task = self.project.update(cx, |project, cx| {
                        project.rename_entry(clip_entry_id, new_path, cx)
                    });
                    PasteTask::Rename(task)
                } else {
                    let entry_id = if is_same_worktree {
                        clip_entry_id
                    } else {
                        entry.id
                    };
                    let task = self.project.update(cx, |project, cx| {
                        project.copy_entry(entry_id, relative_worktree_source_path, new_path, cx)
                    });
                    PasteTask::Copy(task)
                };
                let needs_delete = !is_same_worktree && clip_is_cut;
                paste_entry_tasks.insert((clip_entry_id, needs_delete), task);
                disambiguation_range = new_disambiguation_range.or(disambiguation_range);
            }

            let item_count = paste_entry_tasks.len();

            cx.spawn_in(window, async move |project_panel, cx| {
                let mut last_succeed = None;
                let mut need_delete_ids = Vec::new();
                for ((entry_id, need_delete), task) in paste_entry_tasks.into_iter() {
                    match task {
                        PasteTask::Rename(task) => {
                            if let Some(CreatedEntry::Included(entry)) = task.await.log_err() {
                                last_succeed = Some(entry);
                            }
                        }
                        PasteTask::Copy(task) => {
                            if let Some(Some(entry)) = task.await.log_err() {
                                last_succeed = Some(entry);
                                if need_delete {
                                    need_delete_ids.push(entry_id);
                                }
                            }
                        }
                    }
                }
                // remove entry for cut in difference worktree
                for entry_id in need_delete_ids {
                    project_panel
                        .update(cx, |project_panel, cx| {
                            project_panel
                                .project
                                .update(cx, |project, cx| project.delete_entry(entry_id, true, cx))
                                .ok_or_else(|| anyhow!("no such entry"))
                        })??
                        .await?;
                }
                // update selection
                if let Some(entry) = last_succeed {
                    project_panel
                        .update_in(cx, |project_panel, window, cx| {
                            project_panel.selection = Some(SelectedEntry {
                                worktree_id,
                                entry_id: entry.id,
                            });

                            if item_count == 1 {
                                // open entry if not dir, and only focus if rename is not pending
                                if !entry.is_dir() {
                                    project_panel.open_entry(
                                        entry.id,
                                        disambiguation_range.is_none(),
                                        false,
                                        cx,
                                    );
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
                            .abs_path()
                            .join(entry_path)
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
        let file_paths = {
            let project = self.project.read(cx);
            self.effective_entries()
                .into_iter()
                .filter_map(|entry| {
                    Some(
                        project
                            .path_for_entry(entry.entry_id, cx)?
                            .path
                            .to_string_lossy()
                            .to_string(),
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
            cx.reveal_path(&worktree.read(cx).abs_path().join(&entry.path));
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

    fn open_system(&mut self, _: &OpenWithSystem, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let abs_path = worktree.abs_path().join(&entry.path);
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
                Some(canonical_path) => Some(canonical_path.to_path_buf()),
                None => worktree.read(cx).absolutize(&entry.path).ok(),
            };

            let working_directory = if entry.is_dir() {
                abs_path
            } else {
                abs_path.and_then(|path| Some(path.parent()?.to_path_buf()))
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
                                    Path::new(""),
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
                let mut full_path = PathBuf::from(worktree.read(cx).root_name());
                full_path.push(&dir_path);
                Arc::from(full_path)
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
        destination: ProjectEntryId,
        destination_is_file: bool,
        cx: &mut Context<Self>,
    ) {
        if entry_to_move == destination {
            return;
        }

        let destination_worktree = self.project.update(cx, |project, cx| {
            let entry_path = project.path_for_entry(entry_to_move, cx)?;
            let destination_entry_path = project.path_for_entry(destination, cx)?.path.clone();

            let mut destination_path = destination_entry_path.as_ref();
            if destination_is_file {
                destination_path = destination_path.parent()?;
            }

            let mut new_path = destination_path.to_path_buf();
            new_path.push(entry_path.path.file_name()?);
            if new_path != entry_path.path.as_ref() {
                let task = project.rename_entry(entry_to_move, new_path, cx);
                cx.foreground_executor().spawn(task).detach_and_log_err(cx);
            }

            project.worktree_id_for_entry(destination, cx)
        });

        if let Some(destination_worktree) = destination_worktree {
            self.expand_entry(destination_worktree, destination, cx);
        }
    }

    fn index_for_selection(&self, selection: SelectedEntry) -> Option<(usize, usize, usize)> {
        let mut entry_index = 0;
        let mut visible_entries_index = 0;
        for (worktree_index, (worktree_id, worktree_entries, _)) in
            self.visible_entries.iter().enumerate()
        {
            if *worktree_id == selection.worktree_id {
                for entry in worktree_entries {
                    if entry.id == selection.entry_id {
                        return Some((worktree_index, entry_index, visible_entries_index));
                    } else {
                        visible_entries_index += 1;
                        entry_index += 1;
                    }
                }
                break;
            } else {
                visible_entries_index += worktree_entries.len();
            }
        }
        None
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
        if let Some(selection) = self.selection {
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
        self.ancestors
            .get(&id)
            .and_then(|ancestors| {
                if ancestors.current_ancestor_depth == 0 {
                    return None;
                }
                ancestors.ancestors.get(ancestors.current_ancestor_depth)
            })
            .copied()
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
        let selection = self.selection?;
        let project = self.project.read(cx);
        let worktree = project.worktree_for_id(selection.worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(selection.entry_id)?;
        Some((worktree, entry))
    }

    fn expand_to_selection(&mut self, cx: &mut Context<Self>) -> Option<()> {
        let (worktree, entry) = self.selected_entry(cx)?;
        let expanded_dir_ids = self.expanded_dir_ids.entry(worktree.id()).or_default();

        for path in entry.path.ancestors() {
            let Some(entry) = worktree.entry_for_path(path) else {
                continue;
            };
            if entry.is_dir() {
                if let Err(idx) = expanded_dir_ids.binary_search(&entry.id) {
                    expanded_dir_ids.insert(idx, entry.id);
                }
            }
        }

        Some(())
    }

    fn update_visible_entries(
        &mut self,
        new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        cx: &mut Context<Self>,
    ) {
        let settings = ProjectPanelSettings::get_global(cx);
        let auto_collapse_dirs = settings.auto_fold_dirs;
        let hide_gitignore = settings.hide_gitignore;
        let project = self.project.read(cx);
        let repo_snapshots = project.git_store().read(cx).repo_snapshots(cx);
        self.last_worktree_root_id = project
            .visible_worktrees(cx)
            .next_back()
            .and_then(|worktree| worktree.read(cx).root_entry())
            .map(|entry| entry.id);

        let old_ancestors = std::mem::take(&mut self.ancestors);
        self.visible_entries.clear();
        let mut max_width_item = None;
        for worktree in project.visible_worktrees(cx) {
            let worktree_snapshot = worktree.read(cx).snapshot();
            let worktree_id = worktree_snapshot.id();

            let expanded_dir_ids = match self.expanded_dir_ids.entry(worktree_id) {
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
            if let Some(edit_state) = &self.edit_state {
                if edit_state.worktree_id == worktree_id && edit_state.is_new_entry() {
                    new_entry_parent_id = Some(edit_state.entry_id);
                    new_entry_kind = if edit_state.is_dir {
                        EntryKind::Dir
                    } else {
                        EntryKind::File
                    };
                }
            }

            let mut visible_worktree_entries = Vec::new();
            let mut entry_iter =
                GitTraversal::new(&repo_snapshots, worktree_snapshot.entries(true, 0));
            let mut auto_folded_ancestors = vec![];
            while let Some(entry) = entry_iter.entry() {
                if auto_collapse_dirs && entry.kind.is_dir() {
                    auto_folded_ancestors.push(entry.id);
                    if !self.unfolded_dir_ids.contains(&entry.id) {
                        if let Some(root_path) = worktree_snapshot.root_entry() {
                            let mut child_entries = worktree_snapshot.child_entries(&entry.path);
                            if let Some(child) = child_entries.next() {
                                if entry.path != root_path.path
                                    && child_entries.next().is_none()
                                    && child.kind.is_dir()
                                {
                                    entry_iter.advance();

                                    continue;
                                }
                            }
                        }
                    }
                    let depth = old_ancestors
                        .get(&entry.id)
                        .map(|ancestor| ancestor.current_ancestor_depth)
                        .unwrap_or_default()
                        .min(auto_folded_ancestors.len());
                    if let Some(edit_state) = &mut self.edit_state {
                        if edit_state.entry_id == entry.id {
                            edit_state.depth = depth;
                        }
                    }
                    let mut ancestors = std::mem::take(&mut auto_folded_ancestors);
                    if ancestors.len() > 1 {
                        ancestors.reverse();
                        self.ancestors.insert(
                            entry.id,
                            FoldedAncestors {
                                current_ancestor_depth: depth,
                                ancestors,
                            },
                        );
                    }
                }
                auto_folded_ancestors.clear();
                if !hide_gitignore || !entry.is_ignored {
                    visible_worktree_entries.push(entry.to_owned());
                }
                let precedes_new_entry = if let Some(new_entry_id) = new_entry_parent_id {
                    entry.id == new_entry_id || {
                        self.ancestors.get(&entry.id).map_or(false, |entries| {
                            entries
                                .ancestors
                                .iter()
                                .any(|entry_id| *entry_id == new_entry_id)
                        })
                    }
                } else {
                    false
                };
                if precedes_new_entry && (!hide_gitignore || !entry.is_ignored) {
                    visible_worktree_entries.push(GitEntry {
                        entry: Entry {
                            id: NEW_ENTRY_ID,
                            kind: new_entry_kind,
                            path: entry.path.join("\0").into(),
                            inode: 0,
                            mtime: entry.mtime,
                            size: entry.size,
                            is_ignored: entry.is_ignored,
                            is_external: false,
                            is_private: false,
                            is_always_included: entry.is_always_included,
                            canonical_path: entry.canonical_path.clone(),
                            char_bag: entry.char_bag,
                            is_fifo: entry.is_fifo,
                        },
                        git_summary: entry.git_summary,
                    });
                }
                let worktree_abs_path = worktree.read(cx).abs_path();
                let (depth, path) = if Some(entry.entry) == worktree.read(cx).root_entry() {
                    let Some(path_name) = worktree_abs_path
                        .file_name()
                        .with_context(|| {
                            format!("Worktree abs path has no file name, root entry: {entry:?}")
                        })
                        .log_err()
                    else {
                        continue;
                    };
                    let path = ArcCow::Borrowed(Path::new(path_name));
                    let depth = 0;
                    (depth, path)
                } else if entry.is_file() {
                    let Some(path_name) = entry
                        .path
                        .file_name()
                        .with_context(|| format!("Non-root entry has no file name: {entry:?}"))
                        .log_err()
                    else {
                        continue;
                    };
                    let path = ArcCow::Borrowed(Path::new(path_name));
                    let depth = entry.path.ancestors().count() - 1;
                    (depth, path)
                } else {
                    let path = self
                        .ancestors
                        .get(&entry.id)
                        .and_then(|ancestors| {
                            let outermost_ancestor = ancestors.ancestors.last()?;
                            let root_folded_entry = worktree
                                .read(cx)
                                .entry_for_id(*outermost_ancestor)?
                                .path
                                .as_ref();
                            entry
                                .path
                                .strip_prefix(root_folded_entry)
                                .ok()
                                .and_then(|suffix| {
                                    let full_path = Path::new(root_folded_entry.file_name()?);
                                    Some(ArcCow::Owned(Arc::<Path>::from(full_path.join(suffix))))
                                })
                        })
                        .or_else(|| entry.path.file_name().map(Path::new).map(ArcCow::Borrowed))
                        .unwrap_or_else(|| ArcCow::Owned(entry.path.clone()));
                    let depth = path.components().count();
                    (depth, path)
                };
                let width_estimate = item_width_estimate(
                    depth,
                    path.to_string_lossy().chars().count(),
                    entry.canonical_path.is_some(),
                );

                match max_width_item.as_mut() {
                    Some((id, worktree_id, width)) => {
                        if *width < width_estimate {
                            *id = entry.id;
                            *worktree_id = worktree.read(cx).id();
                            *width = width_estimate;
                        }
                    }
                    None => {
                        max_width_item = Some((entry.id, worktree.read(cx).id(), width_estimate))
                    }
                }

                if expanded_dir_ids.binary_search(&entry.id).is_err()
                    && entry_iter.advance_to_sibling()
                {
                    continue;
                }
                entry_iter.advance();
            }

            project::sort_worktree_entries(&mut visible_worktree_entries);

            self.visible_entries
                .push((worktree_id, visible_worktree_entries, OnceCell::new()));
        }

        if let Some((project_entry_id, worktree_id, _)) = max_width_item {
            let mut visited_worktrees_length = 0;
            let index = self.visible_entries.iter().find_map(|(id, entries, _)| {
                if worktree_id == *id {
                    entries
                        .iter()
                        .position(|entry| entry.id == project_entry_id)
                } else {
                    visited_worktrees_length += entries.len();
                    None
                }
            });
            if let Some(index) = index {
                self.max_width_item_index = Some(visited_worktrees_length + index);
            }
        }
        if let Some((worktree_id, entry_id)) = new_selected_entry {
            self.selection = Some(SelectedEntry {
                worktree_id,
                entry_id,
            });
        }
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
                .zip(self.expanded_dir_ids.get_mut(&worktree_id))
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
                path.to_path_buf()
            } else {
                path.parent()?.to_path_buf()
            };
            Some((target_directory, worktree, fs))
        }) else {
            return;
        };

        let mut paths_to_replace = Vec::new();
        for path in &paths {
            if let Some(name) = path.file_name() {
                let mut target_path = target_directory.clone();
                target_path.push(name);
                if target_path.exists() {
                    paths_to_replace.push((name.to_string_lossy().to_string(), path.clone()));
                }
            }
        }

        cx.spawn_in(window, async move |this, cx| {
            async move {
                for (filename, original_path) in &paths_to_replace {
                    let answer = cx.update(|window, cx| {
                        window
                            .prompt(
                                PromptLevel::Info,
                                format!("A file or folder with name {filename} already exists in the destination folder. Do you want to replace it?").as_str(),
                                None,
                                &["Replace", "Cancel"],
                                cx,
                            )
                    })?.await?;

                    if answer == 1 {
                        if let Some(item_idx) = paths.iter().position(|p| p == original_path) {
                            paths.remove(item_idx);
                        }
                    }
                }

                if paths.is_empty() {
                    return Ok(());
                }

                let task = worktree.update( cx, |worktree, cx| {
                    worktree.copy_external_entries(target_directory.into(), paths, fs, cx)
                })?;

                let opened_entries = task.await.with_context(|| "failed to copy external paths")?;
                this.update(cx, |this, cx| {
                    if open_file_after_drop && !opened_entries.is_empty() {
                        this.open_entry(opened_entries[0], true, false, cx);
                    }
                })
            }
            .log_err().await
        })
        .detach();
    }

    fn drag_onto(
        &mut self,
        selections: &DraggedSelection,
        target_entry_id: ProjectEntryId,
        is_file: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let should_copy = window.modifiers().alt;
        if should_copy {
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
                        project.copy_entry(selection.entry_id, None, new_path, cx)
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
                                project_panel.selection = Some(SelectedEntry {
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
        let mut worktree_ix = 0;
        let mut total_ix = 0;
        for (current_worktree_id, visible_worktree_entries, _) in &self.visible_entries {
            if worktree_id != *current_worktree_id {
                total_ix += visible_worktree_entries.len();
                worktree_ix += 1;
                continue;
            }

            return visible_worktree_entries
                .iter()
                .enumerate()
                .find(|(_, entry)| entry.id == entry_id)
                .map(|(ix, _)| (worktree_ix, ix, total_ix + ix));
        }
        None
    }

    fn entry_at_index(&self, index: usize) -> Option<(WorktreeId, GitEntryRef)> {
        let mut offset = 0;
        for (worktree_id, visible_worktree_entries, _) in &self.visible_entries {
            if visible_worktree_entries.len() > offset + index {
                return visible_worktree_entries
                    .get(index)
                    .map(|entry| (*worktree_id, entry.to_ref()));
            }
            offset += visible_worktree_entries.len();
        }
        None
    }

    fn iter_visible_entries(
        &self,
        range: Range<usize>,
        window: &mut Window,
        cx: &mut Context<ProjectPanel>,
        mut callback: impl FnMut(&Entry, &HashSet<Arc<Path>>, &mut Window, &mut Context<ProjectPanel>),
    ) {
        let mut ix = 0;
        for (_, visible_worktree_entries, entries_paths) in &self.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible_worktree_entries.len() <= range.start {
                ix += visible_worktree_entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible_worktree_entries.len());
            let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
            let entries = entries_paths.get_or_init(|| {
                visible_worktree_entries
                    .iter()
                    .map(|e| (e.path.clone()))
                    .collect()
            });
            for entry in visible_worktree_entries[entry_range].iter() {
                callback(&entry, entries, window, cx);
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
        for (worktree_id, visible_worktree_entries, entries_paths) in &self.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible_worktree_entries.len() <= range.start {
                ix += visible_worktree_entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible_worktree_entries.len());
            let (git_status_setting, show_file_icons, show_folder_icons) = {
                let settings = ProjectPanelSettings::get_global(cx);
                (
                    settings.git_status,
                    settings.file_icons,
                    settings.folder_icons,
                )
            };
            if let Some(worktree) = self.project.read(cx).worktree_for_id(*worktree_id, cx) {
                let snapshot = worktree.read(cx).snapshot();
                let root_name = OsStr::new(snapshot.root_name());
                let expanded_entry_ids = self
                    .expanded_dir_ids
                    .get(&snapshot.id())
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);

                let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
                let entries = entries_paths.get_or_init(|| {
                    visible_worktree_entries
                        .iter()
                        .map(|e| (e.path.clone()))
                        .collect()
                });
                for entry in visible_worktree_entries[entry_range].iter() {
                    let status = git_status_setting
                        .then_some(entry.git_summary)
                        .unwrap_or_default();
                    let is_expanded = expanded_entry_ids.binary_search(&entry.id).is_ok();
                    let icon = match entry.kind {
                        EntryKind::File => {
                            if show_file_icons {
                                FileIcons::get_icon(&entry.path, cx)
                            } else {
                                None
                            }
                        }
                        _ => {
                            if show_folder_icons {
                                FileIcons::get_folder_icon(is_expanded, cx)
                            } else {
                                FileIcons::get_chevron_icon(is_expanded, cx)
                            }
                        }
                    };

                    let (depth, difference) =
                        ProjectPanel::calculate_depth_and_difference(&entry, entries);

                    let filename = match difference {
                        diff if diff > 1 => entry
                            .path
                            .iter()
                            .skip(entry.path.components().count() - diff)
                            .collect::<PathBuf>()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                        _ => entry
                            .path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_else(|| root_name.to_string_lossy().to_string()),
                    };
                    let selection = SelectedEntry {
                        worktree_id: snapshot.id(),
                        entry_id: entry.id,
                    };

                    let is_marked = self.marked_entries.contains(&selection);

                    let diagnostic_severity = self
                        .diagnostics
                        .get(&(*worktree_id, entry.path.to_path_buf()))
                        .cloned();

                    let filename_text_color =
                        entry_git_aware_label_color(status, entry.is_ignored, is_marked);

                    let mut details = EntryDetails {
                        filename,
                        icon,
                        path: entry.path.clone(),
                        depth,
                        kind: entry.kind,
                        is_ignored: entry.is_ignored,
                        is_expanded,
                        is_selected: self.selection == Some(selection),
                        is_marked,
                        is_editing: false,
                        is_processing: false,
                        is_cut: self
                            .clipboard
                            .as_ref()
                            .map_or(false, |e| e.is_cut() && e.items().contains(&selection)),
                        filename_text_color,
                        diagnostic_severity,
                        git_status: status,
                        is_private: entry.is_private,
                        worktree_id: *worktree_id,
                        canonical_path: entry.canonical_path.clone(),
                    };

                    if let Some(edit_state) = &self.edit_state {
                        let is_edited_entry = if edit_state.is_new_entry() {
                            entry.id == NEW_ENTRY_ID
                        } else {
                            entry.id == edit_state.entry_id
                                || self
                                    .ancestors
                                    .get(&entry.id)
                                    .is_some_and(|auto_folded_dirs| {
                                        auto_folded_dirs
                                            .ancestors
                                            .iter()
                                            .any(|entry_id| *entry_id == edit_state.entry_id)
                                    })
                        };

                        if is_edited_entry {
                            if let Some(processing_filename) = &edit_state.processing_filename {
                                details.is_processing = true;
                                if let Some(ancestors) = edit_state
                                    .leaf_entry_id
                                    .and_then(|entry| self.ancestors.get(&entry))
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
                                        Path::new(processing_filename).components().next_back()
                                    {
                                        new_path.push(last_component);
                                        previous_components.next();
                                    }

                                    if let Some(_) = suffix_components {
                                        new_path.push(previous_components);
                                    }
                                    if let Some(str) = new_path.to_str() {
                                        details.filename.clear();
                                        details.filename.push_str(str);
                                    }
                                } else {
                                    details.filename.clear();
                                    details.filename.push_str(processing_filename);
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
                .visible_entries
                .iter()
                .find_map(|(tree_id, entries, _)| {
                    if worktree_id == *tree_id {
                        Some(entries)
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
        worktree.update(cx, |tree, _| {
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
            .visible_entries
            .iter()
            .map(|(worktree_id, _, _)| *worktree_id)
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
                .worktree_for_id(start.worktree_id, cx)?;

            let search = worktree.update(cx, |tree, _| {
                let entry = tree.entry_for_id(start.entry_id)?;
                let root_entry = tree.root_entry()?;
                let tree_id = tree.id();

                let mut first_iter = GitTraversal::new(
                    &repo_snapshots,
                    tree.traverse_from_path(true, true, true, entry.path.as_ref()),
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

                let second_iter = GitTraversal::new(&repo_snapshots, tree.entries(true, 0usize));

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
            });

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
            .visible_entries
            .iter()
            .map(|(worktree_id, _, _)| *worktree_id)
            .collect();

        let mut last_found: Option<SelectedEntry> = None;

        if let Some(start) = start {
            let entries = self
                .visible_entries
                .iter()
                .find(|(worktree_id, _, _)| *worktree_id == start.worktree_id)
                .map(|(_, entries, _)| entries)?;

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
        visible_worktree_entries: &HashSet<Arc<Path>>,
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
            .unwrap_or((0, 0));

        (depth, difference)
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
        let settings = ProjectPanelSettings::get_global(cx);
        let show_editor = details.is_editing && !details.is_processing;

        let selection = SelectedEntry {
            worktree_id: details.worktree_id,
            entry_id,
        };

        let is_marked = self.marked_entries.contains(&selection);
        let is_active = self
            .selection
            .map_or(false, |selection| selection.entry_id == entry_id);

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
        let item_colors = get_item_color(cx);

        let canonical_path = details
            .canonical_path
            .as_ref()
            .map(|f| f.to_string_lossy().to_string());
        let path = details.path.clone();

        let depth = details.depth;
        let worktree_id = details.worktree_id;
        let selections = Arc::new(self.marked_entries.clone());

        let dragged_selection = DraggedSelection {
            active_selection: selection,
            marked_selections: selections,
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
                .edit_state
                .as_ref()
                .map_or(ValidationState::None, |e| e.validation_state.clone())
            {
                ValidationState::Error(msg) => Some((Color::Error.color(cx), msg.clone())),
                ValidationState::Warning(msg) => Some((Color::Warning.color(cx), msg.clone())),
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

        div()
            .id(entry_id.to_proto() as usize)
            .group(GROUP_NAME)
            .cursor_pointer()
            .rounded_none()
            .bg(bg_color)
            .border_1()
            .border_r_2()
            .border_color(border_color)
            .hover(|style| style.bg(bg_hover_color).border_color(border_hover_color))
            .on_drag_move::<ExternalPaths>(cx.listener(
                move |this, event: &DragMoveEvent<ExternalPaths>, _, cx| {
                    if event.bounds.contains(&event.event.position) {
                        if this.last_external_paths_drag_over_entry == Some(entry_id) {
                            return;
                        }
                        this.last_external_paths_drag_over_entry = Some(entry_id);
                        this.marked_entries.clear();

                        let Some((worktree, path, entry)) = maybe!({
                            let worktree = this
                                .project
                                .read(cx)
                                .worktree_for_id(selection.worktree_id, cx)?;
                            let worktree = worktree.read(cx);
                            let entry = worktree.entry_for_path(&path)?;
                            let path = if entry.is_dir() {
                                path.as_ref()
                            } else {
                                path.parent()?
                            };
                            Some((worktree, path, entry))
                        }) else {
                            return;
                        };

                        this.marked_entries.insert(SelectedEntry {
                            entry_id: entry.id,
                            worktree_id: worktree.id(),
                        });

                        for entry in worktree.child_entries(path) {
                            this.marked_entries.insert(SelectedEntry {
                                entry_id: entry.id,
                                worktree_id: worktree.id(),
                            });
                        }

                        cx.notify();
                    }
                },
            ))
            .on_drop(cx.listener(
                move |this, external_paths: &ExternalPaths, window, cx| {
                    this.hover_scroll_task.take();
                    this.last_external_paths_drag_over_entry = None;
                    this.marked_entries.clear();
                    this.drop_external_files(external_paths.paths(), entry_id, window, cx);
                    cx.stop_propagation();
                },
            ))
            .on_drag_move::<DraggedSelection>(cx.listener(
                move |this, event: &DragMoveEvent<DraggedSelection>, window, cx| {
                    if event.bounds.contains(&event.event.position) {
                        if this.last_selection_drag_over_entry == Some(entry_id) {
                            return;
                        }
                        this.last_selection_drag_over_entry = Some(entry_id);
                        this.hover_expand_task.take();

                        if !kind.is_dir()
                            || this
                                .expanded_dir_ids
                                .get(&details.worktree_id)
                                .map_or(false, |ids| ids.binary_search(&entry_id).is_ok())
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
                                    if this.last_selection_drag_over_entry == Some(entry_id)
                                        && bounds.contains(&window.mouse_position())
                                    {
                                        this.expand_entry(worktree_id, entry_id, cx);
                                        this.update_visible_entries(
                                            Some((worktree_id, entry_id)),
                                            cx,
                                        );
                                        cx.notify();
                                    }
                                })
                                .ok();
                            }));
                    }
                },
            ))
            .on_drag(
                dragged_selection,
                move |selection, click_offset, _window, cx| {
                    cx.new(|_| DraggedProjectEntryView {
                        details: details.clone(),
                        click_offset,
                        selection: selection.active_selection,
                        selections: selection.marked_selections.clone(),
                    })
                },
            )
            .drag_over::<DraggedSelection>(move |style, _, _, _| {
                if  folded_directory_drag_target.is_some() {
                    return style;
                }
                style.bg(item_colors.drag_over)
            })
            .on_drop(
                cx.listener(move |this, selections: &DraggedSelection, window, cx| {
                    this.hover_scroll_task.take();
                    this.hover_expand_task.take();
                    if  folded_directory_drag_target.is_some() {
                        return;
                    }
                    this.drag_onto(selections, entry_id, kind.is_file(), window, cx);
                }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.mouse_down = true;
                    cx.propagate();
                }),
            )
            .on_click(
                cx.listener(move |this, event: &gpui::ClickEvent, window, cx| {
                    if event.down.button == MouseButton::Right
                        || event.down.first_mouse
                        || show_editor
                    {
                        return;
                    }
                    if event.down.button == MouseButton::Left {
                        this.mouse_down = false;
                    }
                    cx.stop_propagation();

                    if let Some(selection) = this.selection.filter(|_| event.modifiers().shift) {
                        let current_selection = this.index_for_selection(selection);
                        let clicked_entry = SelectedEntry {
                            entry_id,
                            worktree_id,
                        };
                        let target_selection = this.index_for_selection(clicked_entry);
                        if let Some(((_, _, source_index), (_, _, target_index))) =
                            current_selection.zip(target_selection)
                        {
                            let range_start = source_index.min(target_index);
                            let range_end = source_index.max(target_index) + 1; // Make the range inclusive.
                            let mut new_selections = BTreeSet::new();
                            this.for_each_visible_entry(
                                range_start..range_end,
                                window,
                                cx,
                                |entry_id, details, _, _| {
                                    new_selections.insert(SelectedEntry {
                                        entry_id,
                                        worktree_id: details.worktree_id,
                                    });
                                },
                            );

                            this.marked_entries = this
                                .marked_entries
                                .union(&new_selections)
                                .cloned()
                                .collect();

                            this.selection = Some(clicked_entry);
                            this.marked_entries.insert(clicked_entry);
                        }
                    } else if event.modifiers().secondary() {
                        if event.down.click_count > 1 {
                            this.split_entry(entry_id, cx);
                        } else {
                            this.selection = Some(selection);
                            if !this.marked_entries.insert(selection) {
                                this.marked_entries.remove(&selection);
                            }
                        }
                    } else if kind.is_dir() {
                        this.marked_entries.clear();
                        if event.modifiers().alt {
                            this.toggle_expand_all(entry_id, window, cx);
                        } else {
                            this.toggle_expanded(entry_id, window, cx);
                        }
                    } else {
                        let preview_tabs_enabled = PreviewTabsSettings::get_global(cx).enabled;
                        let click_count = event.up.click_count;
                        let focus_opened_item = !preview_tabs_enabled || click_count > 1;
                        let allow_preview = preview_tabs_enabled && click_count == 1;
                        this.open_entry(entry_id, focus_opened_item, allow_preview, cx);
                    }
                }),
            )
            .child(
                ListItem::new(entry_id.to_proto() as usize)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .spacing(match settings.entry_spacing {
                        project_panel_settings::EntrySpacing::Comfortable => ListItemSpacing::Dense,
                        project_panel_settings::EntrySpacing::Standard => {
                            ListItemSpacing::ExtraDense
                        }
                    })
                    .selectable(false)
                    .when_some(canonical_path, |this, path| {
                        this.end_slot::<AnyElement>(
                            div()
                                .id("symlink_icon")
                                .pr_3()
                                .tooltip(move |window, cx| {
                                    Tooltip::with_meta(
                                        path.to_string(),
                                        None,
                                        "Symbolic Link",
                                        window,
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
                    } else {
                        if let Some((icon_name, color)) =
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
                        }
                    })
                    .child(
                        if let (Some(editor), true) = (Some(&self.filename_editor), show_editor) {
                            h_flex().h_6().w_full().child(editor.clone())
                        } else {
                            h_flex().h_6().map(|mut this| {
                                if let Some(folded_ancestors) = self.ancestors.get(&entry_id) {
                                    let components = Path::new(&file_name)
                                        .components()
                                        .map(|comp| {
                                            let comp_str =
                                                comp.as_os_str().to_string_lossy().into_owned();
                                            comp_str
                                        })
                                        .collect::<Vec<_>>();

                                    let components_len = components.len();
                                    let active_index = components_len
                                        - 1
                                        - folded_ancestors.current_ancestor_depth;
                                        const DELIMITER: SharedString =
                                        SharedString::new_static(std::path::MAIN_SEPARATOR_STR);
                                    for (index, component) in components.into_iter().enumerate() {
                                        if index != 0 {
                                                let delimiter_target_index = index - 1;
                                                let target_entry_id = folded_ancestors.ancestors.get(components_len - 1 - delimiter_target_index).cloned();
                                                this = this.child(
                                                    div()
                                                    .on_drop(cx.listener(move |this, selections: &DraggedSelection, window, cx| {
                                                        this.hover_scroll_task.take();
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
                                                                    .map_or(false, |target|
                                                                        target.entry_id == entry_id &&
                                                                        target.index == delimiter_target_index &&
                                                                        target.is_delimiter_target
                                                                    );
                                                                if is_current_target {
                                                                    this.folded_directory_drag_target = None;
                                                                }
                                                            }

                                                        },
                                                    ))
                                                    .child(
                                                        Label::new(DELIMITER.clone())
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
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                if index != active_index {
                                                    if let Some(folds) =
                                                        this.ancestors.get_mut(&entry_id)
                                                    {
                                                        folds.current_ancestor_depth =
                                                            components_len - 1 - index;
                                                        cx.notify();
                                                    }
                                                }
                                            }))
                                            .when(index != components_len - 1, |div|{
                                                let target_entry_id = folded_ancestors.ancestors.get(components_len - 1 - index).cloned();
                                                div
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
                                                                .map_or(false, |target|
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
                                                    this.folded_directory_drag_target = None;
                                                    if let Some(target_entry_id) = target_entry_id {
                                                        this.drag_onto(selections, target_entry_id, kind.is_file(), window, cx);
                                                    }
                                                }))
                                                .when(folded_directory_drag_target.map_or(false, |target|
                                                    target.entry_id == entry_id &&
                                                    target.index == index
                                                ), |this| {
                                                    this.bg(item_colors.drag_over)
                                                })
                                            })
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
                        }
                        .ml_1(),
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

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !Self::should_show_scrollbar(cx)
            || !(self.show_scrollbar || self.vertical_scrollbar_state.is_dragging())
        {
            return None;
        }
        Some(
            div()
                .occlude()
                .id("project-panel-vertical-scroll")
                .on_mouse_move(cx.listener(|_, _, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        if !this.vertical_scrollbar_state.is_dragging()
                            && !this.focus_handle.contains_focused(window, cx)
                        {
                            this.hide_scrollbar(window, cx);
                            cx.notify();
                        }

                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .h_full()
                .absolute()
                .right_1()
                .top_1()
                .bottom_1()
                .w(px(12.))
                .cursor_default()
                .children(Scrollbar::vertical(
                    // percentage as f32..end_offset as f32,
                    self.vertical_scrollbar_state.clone(),
                )),
        )
    }

    fn render_horizontal_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !Self::should_show_scrollbar(cx)
            || !(self.show_scrollbar || self.horizontal_scrollbar_state.is_dragging())
        {
            return None;
        }

        let scroll_handle = self.scroll_handle.0.borrow();
        let longest_item_width = scroll_handle
            .last_item_size
            .filter(|size| size.contents.width > size.item.width)?
            .contents
            .width
            .0 as f64;
        if longest_item_width < scroll_handle.base_handle.bounds().size.width.0 as f64 {
            return None;
        }

        Some(
            div()
                .occlude()
                .id("project-panel-horizontal-scroll")
                .on_mouse_move(cx.listener(|_, _, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        if !this.horizontal_scrollbar_state.is_dragging()
                            && !this.focus_handle.contains_focused(window, cx)
                        {
                            this.hide_scrollbar(window, cx);
                            cx.notify();
                        }

                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .w_full()
                .absolute()
                .right_1()
                .left_1()
                .bottom_1()
                .h(px(12.))
                .cursor_default()
                .when(self.width.is_some(), |this| {
                    this.children(Scrollbar::horizontal(
                        self.horizontal_scrollbar_state.clone(),
                    ))
                }),
        )
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

    fn should_show_scrollbar(cx: &App) -> bool {
        let show = ProjectPanelSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show);
        match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => true,
            ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        }
    }

    fn should_autohide_scrollbar(cx: &App) -> bool {
        let show = ProjectPanelSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show);
        match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => cx
                .try_global::<ScrollbarAutoHide>()
                .map_or_else(|| cx.should_auto_hide_scrollbars(), |autohide| autohide.0),
            ShowScrollbar::Always => false,
            ShowScrollbar::Never => true,
        }
    }

    fn hide_scrollbar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        if !Self::should_autohide_scrollbar(cx) {
            return;
        }
        self.hide_scrollbar_task = Some(cx.spawn_in(window, async move |panel, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            panel
                .update(cx, |panel, cx| {
                    panel.show_scrollbar = false;
                    cx.notify();
                })
                .log_err();
        }))
    }

    fn reveal_entry(
        &mut self,
        project: Entity<Project>,
        entry_id: ProjectEntryId,
        skip_ignored: bool,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx) {
            let worktree = worktree.read(cx);
            if skip_ignored
                && worktree
                    .entry_for_id(entry_id)
                    .map_or(true, |entry| entry.is_ignored && !entry.is_always_included)
            {
                return Err(anyhow!(
                    "can't reveal an ignored entry in the project panel"
                ));
            }

            let worktree_id = worktree.id();
            self.expand_entry(worktree_id, entry_id, cx);
            self.update_visible_entries(Some((worktree_id, entry_id)), cx);
            self.marked_entries.clear();
            self.marked_entries.insert(SelectedEntry {
                worktree_id,
                entry_id,
            });
            self.autoscroll(cx);
            cx.notify();
            Ok(())
        } else {
            Err(anyhow!(
                "can't reveal a non-existent entry in the project panel"
            ))
        }
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
            let child_paths = &self.visible_entries[worktree_ix].1;
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

            let (_, entries, paths) = &self.visible_entries[worktree_ix];
            let visible_worktree_entries =
                paths.get_or_init(|| entries.iter().map(|e| (e.path.clone())).collect());

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
        let has_worktree = !self.visible_entries.is_empty();
        let project = self.project.read(cx);
        let indent_size = ProjectPanelSettings::get_global(cx).indent_size;
        let show_indent_guides =
            ProjectPanelSettings::get_global(cx).indent_guides.show == ShowIndentGuides::Always;
        let is_local = project.is_local();

        if has_worktree {
            let item_count = self
                .visible_entries
                .iter()
                .map(|(_, worktree_entries, _)| worktree_entries.len())
                .sum();

            fn handle_drag_move_scroll<T: 'static>(
                this: &mut ProjectPanel,
                e: &DragMoveEvent<T>,
                window: &mut Window,
                cx: &mut Context<ProjectPanel>,
            ) {
                if !e.bounds.contains(&e.event.position) {
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
                .on_drag_move(cx.listener(handle_drag_move_scroll::<ExternalPaths>))
                .on_drag_move(cx.listener(handle_drag_move_scroll::<DraggedSelection>))
                .size_full()
                .relative()
                .on_hover(cx.listener(|this, hovered, window, cx| {
                    if *hovered {
                        this.show_scrollbar = true;
                        this.hide_scrollbar_task.take();
                        cx.notify();
                    } else if !this.focus_handle.contains_focused(window, cx) {
                        this.hide_scrollbar(window, cx);
                    }
                }))
                .on_click(cx.listener(|this, _event, _, cx| {
                    cx.stop_propagation();
                    this.selection = None;
                    this.marked_entries.clear();
                }))
                .key_context(self.dispatch_context(window, cx))
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
                .on_action(cx.listener(Self::confirm))
                .on_action(cx.listener(Self::cancel))
                .on_action(cx.listener(Self::copy_path))
                .on_action(cx.listener(Self::copy_relative_path))
                .on_action(cx.listener(Self::new_search_in_directory))
                .on_action(cx.listener(Self::unfold_directory))
                .on_action(cx.listener(Self::fold_directory))
                .on_action(cx.listener(Self::remove_from_project))
                .when(!project.is_read_only(cx), |el| {
                    el.on_action(cx.listener(Self::new_file))
                        .on_action(cx.listener(Self::new_directory))
                        .on_action(cx.listener(Self::rename))
                        .on_action(cx.listener(Self::delete))
                        .on_action(cx.listener(Self::trash))
                        .on_action(cx.listener(Self::cut))
                        .on_action(cx.listener(Self::copy))
                        .on_action(cx.listener(Self::paste))
                        .on_action(cx.listener(Self::duplicate))
                        .on_click(cx.listener(|this, event: &gpui::ClickEvent, window, cx| {
                            if event.up.click_count > 1 {
                                if let Some(entry_id) = this.last_worktree_root_id {
                                    let project = this.project.read(cx);

                                    let worktree_id = if let Some(worktree) =
                                        project.worktree_for_entry(entry_id, cx)
                                    {
                                        worktree.read(cx).id()
                                    } else {
                                        return;
                                    };

                                    this.selection = Some(SelectedEntry {
                                        worktree_id,
                                        entry_id,
                                    });

                                    this.new_file(&NewFile, window, cx);
                                }
                            }
                        }))
                })
                .when(project.is_local(), |el| {
                    el.on_action(cx.listener(Self::reveal_in_finder))
                        .on_action(cx.listener(Self::open_system))
                        .on_action(cx.listener(Self::open_in_terminal))
                })
                .when(project.is_via_ssh(), |el| {
                    el.on_action(cx.listener(Self::open_in_terminal))
                })
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                        // When deploying the context menu anywhere below the last project entry,
                        // act as if the user clicked the root of the last worktree.
                        if let Some(entry_id) = this.last_worktree_root_id {
                            this.deploy_context_menu(event.position, entry_id, window, cx);
                        }
                    }),
                )
                .track_focus(&self.focus_handle(cx))
                .child(
                    uniform_list(cx.entity().clone(), "entries", item_count, {
                        |this, range, window, cx| {
                            let mut items = Vec::with_capacity(range.end - range.start);
                            this.for_each_visible_entry(
                                range,
                                window,
                                cx,
                                |id, details, window, cx| {
                                    items.push(this.render_entry(id, details, window, cx));
                                },
                            );
                            items
                        }
                    })
                    .when(show_indent_guides, |list| {
                        list.with_decoration(
                            ui::indent_guides(
                                cx.entity().clone(),
                                px(indent_size),
                                IndentGuideColors::panel(cx),
                                |this, range, window, cx| {
                                    let mut items =
                                        SmallVec::with_capacity(range.end - range.start);
                                    this.iter_visible_entries(
                                        range,
                                        window,
                                        cx,
                                        |entry, entries, _, _| {
                                            let (depth, _) = Self::calculate_depth_and_difference(
                                                entry, entries,
                                            );
                                            items.push(depth);
                                        },
                                    );
                                    items
                                },
                            )
                            .on_click(cx.listener(
                                |this, active_indent_guide: &IndentGuideLayout, window, cx| {
                                    if window.modifiers().secondary() {
                                        let ix = active_indent_guide.offset.y;
                                        let Some((target_entry, worktree)) = maybe!({
                                            let (worktree_id, entry) = this.entry_at_index(ix)?;
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

                                        this.collapse_entry(target_entry.clone(), worktree, cx);
                                    }
                                },
                            ))
                            .with_render_fn(
                                cx.entity().clone(),
                                move |this, params, _, cx| {
                                    const LEFT_OFFSET: Pixels = px(14.);
                                    const PADDING_Y: Pixels = px(4.);
                                    const HITBOX_OVERDRAW: Pixels = px(3.);

                                    let active_indent_guide_index =
                                        this.find_active_indent_guide(&params.indent_guides, cx);

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
                                                    layout.offset.x * indent_size + LEFT_OFFSET,
                                                    layout.offset.y * item_height + offset,
                                                ),
                                                size(
                                                    px(1.),
                                                    layout.length * item_height - offset * 2.,
                                                ),
                                            );
                                            ui::RenderedIndentGuide {
                                                bounds,
                                                layout,
                                                is_active: Some(idx) == active_indent_guide_index,
                                                hitbox: Some(Bounds::new(
                                                    point(
                                                        bounds.origin.x - HITBOX_OVERDRAW,
                                                        bounds.origin.y,
                                                    ),
                                                    size(
                                                        bounds.size.width + HITBOX_OVERDRAW * 2.,
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
                    .size_full()
                    .with_sizing_behavior(ListSizingBehavior::Infer)
                    .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
                    .with_width_from_item(self.max_width_item_index)
                    .track_scroll(self.scroll_handle.clone()),
                )
                .children(self.render_vertical_scrollbar(cx))
                .when_some(self.render_horizontal_scrollbar(cx), |this, scrollbar| {
                    this.pb_4().child(scrollbar)
                })
                .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                    deferred(
                        anchored()
                            .position(*position)
                            .anchor(gpui::Corner::TopLeft)
                            .child(menu.clone()),
                    )
                    .with_priority(1)
                }))
        } else {
            v_flex()
                .id("empty-project_panel")
                .size_full()
                .p_4()
                .track_focus(&self.focus_handle(cx))
                .child(
                    Button::new("open_project", "Open a project")
                        .full_width()
                        .key_binding(KeyBinding::for_action(&workspace::Open, window, cx))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.workspace
                                .update(cx, |_, cx| {
                                    window.dispatch_action(Box::new(workspace::Open), cx)
                                })
                                .log_err();
                        })),
                )
                .when(is_local, |div| {
                    div.drag_over::<ExternalPaths>(|style, _, _, cx| {
                        style.bg(cx.theme().colors().drop_target_background)
                    })
                    .on_drop(cx.listener(
                        move |this, external_paths: &ExternalPaths, window, cx| {
                            this.last_external_paths_drag_over_entry = None;
                            this.marked_entries.clear();
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
                            this.child(if let Some(icon) = &self.details.icon {
                                div().child(Icon::from_path(icon.clone()))
                            } else {
                                div()
                            })
                            .child(Label::new(self.details.filename.clone()))
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
            ProjectPanelDockPosition::Left => DockPosition::Left,
            ProjectPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file::<ProjectPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => ProjectPanelDockPosition::Left,
                    DockPosition::Right => ProjectPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| ProjectPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
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

    fn starts_open(&self, _: &Window, cx: &App) -> bool {
        let project = &self.project.read(cx);
        project.visible_worktrees(cx).any(|tree| {
            tree.read(cx)
                .root_entry()
                .map_or(false, |entry| entry.is_dir())
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
}

#[cfg(test)]
mod project_panel_tests;
