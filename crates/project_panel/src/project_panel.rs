mod project_panel_settings;

use client::{ErrorCode, ErrorExt};
use language::DiagnosticSeverity;
use settings::{Settings, SettingsStore};

use db::kvp::KEY_VALUE_STORE;
use editor::{
    items::{
        entry_diagnostic_aware_icon_decoration_and_color,
        entry_diagnostic_aware_icon_name_and_color, entry_git_aware_label_color,
    },
    scroll::{Autoscroll, ScrollbarAutoHide},
    Editor, EditorEvent, EditorSettings, ShowScrollbar,
};
use file_icons::FileIcons;

use anyhow::{anyhow, Context as _, Result};
use collections::{hash_map, BTreeSet, HashMap};
use command_palette_hooks::CommandPaletteFilter;
use git::repository::GitFileStatus;
use gpui::{
    actions, anchored, deferred, div, impl_actions, point, px, size, uniform_list, Action,
    AnyElement, AppContext, AssetSource, AsyncWindowContext, Bounds, ClipboardItem, DismissEvent,
    Div, DragMoveEvent, EventEmitter, ExternalPaths, FocusHandle, FocusableView, Hsla,
    InteractiveElement, KeyContext, ListHorizontalSizingBehavior, ListSizingBehavior, Model,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, PromptLevel, Render, ScrollStrategy,
    Stateful, Styled, Subscription, Task, UniformListScrollHandle, View, ViewContext,
    VisualContext as _, WeakView, WindowContext,
};
use indexmap::IndexMap;
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrev};
use project::{
    relativize_path, Entry, EntryKind, Fs, Project, ProjectEntryId, ProjectPath, Worktree,
    WorktreeId,
};
use project_panel_settings::{
    ProjectPanelDockPosition, ProjectPanelSettings, ShowDiagnostics, ShowIndentGuides,
};
use serde::{Deserialize, Serialize};
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
    prelude::*, v_flex, ContextMenu, DecoratedIcon, Icon, IconDecoration, IconDecorationKind,
    IndentGuideColors, IndentGuideLayout, KeyBinding, Label, ListItem, Scrollbar, ScrollbarState,
    Tooltip,
};
use util::{maybe, paths::compare_paths, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{DetachAndPromptErr, NotifyTaskExt},
    DraggedSelection, OpenInTerminal, PreviewTabsSettings, SelectedEntry, Workspace,
};
use worktree::CreatedEntry;

const PROJECT_PANEL_KEY: &str = "ProjectPanel";
const NEW_ENTRY_ID: ProjectEntryId = ProjectEntryId::MAX;

pub struct ProjectPanel {
    project: Model<Project>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    // An update loop that keeps incrementing/decrementing scroll offset while there is a dragged entry that's
    // hovered over the start/end of a list.
    hover_scroll_task: Option<Task<()>>,
    visible_entries: Vec<(WorktreeId, Vec<Entry>, OnceCell<HashSet<Arc<Path>>>)>,
    /// Maps from leaf project entry ID to the currently selected ancestor.
    /// Relevant only for auto-fold dirs, where a single project panel entry may actually consist of several
    /// project entries (and all non-leaf nodes are guaranteed to be directories).
    ancestors: HashMap<ProjectEntryId, FoldedAncestors>,
    last_worktree_root_id: Option<ProjectEntryId>,
    last_external_paths_drag_over_entry: Option<ProjectEntryId>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
    unfolded_dir_ids: HashSet<ProjectEntryId>,
    // Currently selected leaf entry (see auto-folding for a definition of that) in a file tree
    selection: Option<SelectedEntry>,
    marked_entries: BTreeSet<SelectedEntry>,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    edit_state: Option<EditState>,
    filename_editor: View<Editor>,
    clipboard: Option<ClipboardEntry>,
    _dragged_entry_destination: Option<Arc<Path>>,
    workspace: WeakView<Workspace>,
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
    hovered_entries: HashSet<ProjectEntryId>,
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
    is_hovered: bool,
    is_cut: bool,
    filename_text_color: Color,
    diagnostic_severity: Option<DiagnosticSeverity>,
    git_status: Option<GitFileStatus>,
    is_private: bool,
    worktree_id: WorktreeId,
    canonical_path: Option<Box<Path>>,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize)]
struct Delete {
    #[serde(default)]
    pub skip_prompt: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize)]
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
        CopyPath,
        CopyRelativePath,
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
        NewSearchInDirectory,
        UnfoldDirectory,
        FoldDirectory,
        SelectParent,
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

pub fn init_settings(cx: &mut AppContext) {
    ProjectPanelSettings::register(cx);
}

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    init_settings(cx);
    file_icons::init(assets, cx);

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<ProjectPanel>(cx);
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
    width: Pixels,
    click_offset: Point<Pixels>,
    selections: Arc<BTreeSet<SelectedEntry>>,
}

struct ItemColors {
    default: Hsla,
    hover: Hsla,
    drag_over: Hsla,
    marked_active: Hsla,
}

fn get_item_color(cx: &ViewContext<ProjectPanel>) -> ItemColors {
    let colors = cx.theme().colors();

    ItemColors {
        default: colors.surface_background,
        hover: colors.ghost_element_hover,
        drag_over: colors.drop_target_background,
        marked_active: colors.ghost_element_selected,
    }
}

impl ProjectPanel {
    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let project = workspace.project().clone();
        let project_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, |this, _, cx| {
                this.hide_scrollbar(cx);
            })
            .detach();
            cx.subscribe(&project, |this, project, event, cx| match event {
                project::Event::ActiveEntryChanged(Some(entry_id)) => {
                    if ProjectPanelSettings::get_global(cx).auto_reveal_entries {
                        this.reveal_entry(project, *entry_id, true, cx);
                    }
                }
                project::Event::RevealInProjectPanel(entry_id) => {
                    this.reveal_entry(project, *entry_id, false, cx);
                    cx.emit(PanelEvent::Activate);
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
                | project::Event::WorktreeAdded
                | project::Event::WorktreeOrderChanged => {
                    this.update_visible_entries(None, cx);
                    cx.notify();
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

            let filename_editor = cx.new_view(Editor::single_line);

            cx.subscribe(
                &filename_editor,
                |project_panel, _, editor_event, cx| match editor_event {
                    EditorEvent::BufferEdited | EditorEvent::SelectionsChanged { .. } => {
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
                last_worktree_root_id: Default::default(),
                last_external_paths_drag_over_entry: None,
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
                    .parent_view(cx.view()),
                horizontal_scrollbar_state: ScrollbarState::new(scroll_handle.clone())
                    .parent_view(cx.view()),
                max_width_item_index: None,
                diagnostics: Default::default(),
                scroll_handle,
                mouse_down: false,
                hovered_entries: Default::default(),
            };
            this.update_visible_entries(None, cx);

            this
        });

        cx.subscribe(&project_panel, {
            let project_panel = project_panel.downgrade();
            move |workspace, _, event, cx| match event {
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
                                    cx,
                                )
                                .detach_and_prompt_err("Failed to open file", cx, move |e, _| {
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
                                    cx.focus(&focus_handle);
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
                                    cx,
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
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(PROJECT_PANEL_KEY) })
            .await
            .map_err(|e| anyhow!("Failed to load project panel: {}", e))
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedProjectPanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        workspace.update(&mut cx, |workspace, cx| {
            let panel = ProjectPanel::new(workspace, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width.map(|px| px.round());
                    cx.notify();
                });
            }
            panel
        })
    }

    fn update_diagnostics(&mut self, cx: &mut ViewContext<Self>) {
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

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
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

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focus_handle.contains_focused(cx) {
            cx.emit(Event::Focus);
        }
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        entry_id: ProjectEntryId,
        cx: &mut ViewContext<Self>,
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

            let context_menu = ContextMenu::build(cx, |menu, _| {
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
                            .action("Copy Path", Box::new(CopyPath))
                            .action("Copy Relative Path", Box::new(CopyRelativePath))
                            .separator()
                            .action("Rename", Box::new(Rename))
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

            cx.focus_view(&context_menu);
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

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
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
                    Ok(_) => self.select_next(&SelectNext, cx),
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

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        let Some((worktree, entry)) = self.selected_entry_handle(cx) else {
            return;
        };
        self.collapse_entry(entry.clone(), worktree, cx)
    }

    fn collapse_entry(
        &mut self,
        entry: Entry,
        worktree: Model<Worktree>,
        cx: &mut ViewContext<Self>,
    ) {
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

    pub fn collapse_all_entries(&mut self, _: &CollapseAllEntries, cx: &mut ViewContext<Self>) {
        // By keeping entries for fully collapsed worktrees, we avoid expanding them within update_visible_entries
        // (which is it's default behavior when there's no entry for a worktree in expanded_dir_ids).
        self.expanded_dir_ids
            .retain(|_, expanded_entries| expanded_entries.is_empty());
        self.update_visible_entries(None, cx);
        cx.notify();
    }

    fn toggle_expanded(&mut self, entry_id: ProjectEntryId, cx: &mut ViewContext<Self>) {
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
                cx.focus(&self.focus_handle);
                cx.notify();
            }
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(edit_state) = &self.edit_state {
            if edit_state.processing_filename.is_none() {
                self.filename_editor.update(cx, |editor, cx| {
                    editor.move_to_beginning_of_line(
                        &editor::actions::MoveToBeginningOfLine {
                            stop_at_soft_wraps: false,
                        },
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
            if cx.modifiers().shift {
                self.marked_entries.insert(selection);
            }
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(task) = self.confirm_edit(cx) {
            task.detach_and_notify_err(cx);
        }
    }

    fn open(&mut self, _: &Open, cx: &mut ViewContext<Self>) {
        let preview_tabs_enabled = PreviewTabsSettings::get_global(cx).enabled;
        self.open_internal(true, !preview_tabs_enabled, cx);
    }

    fn open_permanent(&mut self, _: &OpenPermanent, cx: &mut ViewContext<Self>) {
        self.open_internal(false, true, cx);
    }

    fn open_internal(
        &mut self,
        allow_preview: bool,
        focus_opened_item: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some((_, entry)) = self.selected_entry(cx) {
            if entry.is_file() {
                self.open_entry(entry.id, focus_opened_item, allow_preview, cx);
            } else {
                self.toggle_expanded(entry.id, cx);
            }
        }
    }

    fn confirm_edit(&mut self, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let edit_state = self.edit_state.as_mut()?;
        cx.focus(&self.focus_handle);

        let worktree_id = edit_state.worktree_id;
        let is_new_entry = edit_state.is_new_entry();
        let filename = self.filename_editor.read(cx).text(cx);
        edit_state.is_dir = edit_state.is_dir
            || (edit_state.is_new_entry() && filename.ends_with(std::path::MAIN_SEPARATOR));
        let is_dir = edit_state.is_dir;
        let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(edit_state.entry_id)?.clone();

        let path_already_exists = |path| worktree.read(cx).entry_for_path(path).is_some();
        let edit_task;
        let edited_entry_id;
        if is_new_entry {
            self.selection = Some(SelectedEntry {
                worktree_id,
                entry_id: NEW_ENTRY_ID,
            });
            let new_path = entry.path.join(filename.trim_start_matches('/'));
            if path_already_exists(new_path.as_path()) {
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
            if path_already_exists(new_path.as_path()) {
                return None;
            }
            edited_entry_id = entry.id;
            edit_task = self.project.update(cx, |project, cx| {
                project.rename_entry(entry.id, new_path.as_path(), cx)
            });
        };

        edit_state.processing_filename = Some(filename);
        cx.notify();

        Some(cx.spawn(|project_panel, mut cx| async move {
            let new_entry = edit_task.await;
            project_panel.update(&mut cx, |project_panel, cx| {
                project_panel.edit_state = None;
                cx.notify();
            })?;

            match new_entry {
                Err(e) => {
                    project_panel.update(&mut cx, |project_panel, cx| {
                        project_panel.marked_entries.clear();
                        project_panel.update_visible_entries(None, cx);
                    }).ok();
                    Err(e)?;
                }
                Ok(CreatedEntry::Included(new_entry)) => {
                    project_panel.update(&mut cx, |project_panel, cx| {
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
                        .update(&mut cx, |project_panel, cx| {
                            project_panel.marked_entries.clear();
                            project_panel.update_visible_entries(None, cx);

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
                                        workspace.open_abs_path(abs_path, true, cx)
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

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        let previous_edit_state = self.edit_state.take();
        self.update_visible_entries(None, cx);
        self.marked_entries.clear();

        if let Some(previously_focused) =
            previous_edit_state.and_then(|edit_state| edit_state.previously_focused)
        {
            self.selection = Some(previously_focused);
            self.autoscroll(cx);
        }

        cx.focus(&self.focus_handle);
        cx.notify();
    }

    fn open_entry(
        &mut self,
        entry_id: ProjectEntryId,
        focus_opened_item: bool,
        allow_preview: bool,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(Event::OpenedEntry {
            entry_id,
            focus_opened_item,
            allow_preview,
        });
    }

    fn split_entry(&mut self, entry_id: ProjectEntryId, cx: &mut ViewContext<Self>) {
        cx.emit(Event::SplitEntry { entry_id });
    }

    fn new_file(&mut self, _: &NewFile, cx: &mut ViewContext<Self>) {
        self.add_entry(false, cx)
    }

    fn new_directory(&mut self, _: &NewDirectory, cx: &mut ViewContext<Self>) {
        self.add_entry(true, cx)
    }

    fn add_entry(&mut self, is_dir: bool, cx: &mut ViewContext<Self>) {
        if let Some(SelectedEntry {
            worktree_id,
            entry_id,
        }) = self.selection
        {
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
            });
            self.filename_editor.update(cx, |editor, cx| {
                editor.clear(cx);
                editor.focus(cx);
            });
            self.update_visible_entries(Some((worktree_id, NEW_ENTRY_ID)), cx);
            self.autoscroll(cx);
            cx.notify();
        }
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

    fn rename(&mut self, _: &Rename, cx: &mut ViewContext<Self>) {
        if let Some(SelectedEntry {
            worktree_id,
            entry_id,
        }) = self.selection
        {
            if let Some(worktree) = self.project.read(cx).worktree_for_id(worktree_id, cx) {
                let sub_entry_id = self.unflatten_entry_id(entry_id);
                if let Some(entry) = worktree.read(cx).entry_for_id(sub_entry_id) {
                    self.edit_state = Some(EditState {
                        worktree_id,
                        entry_id: sub_entry_id,
                        leaf_entry_id: Some(entry_id),
                        is_dir: entry.is_dir(),
                        processing_filename: None,
                        previously_focused: None,
                        depth: 0,
                    });
                    let file_name = entry
                        .path
                        .file_name()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default()
                        .to_string();
                    let file_stem = entry.path.file_stem().map(|s| s.to_string_lossy());
                    let selection_end =
                        file_stem.map_or(file_name.len(), |file_stem| file_stem.len());
                    self.filename_editor.update(cx, |editor, cx| {
                        editor.set_text(file_name, cx);
                        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                            s.select_ranges([0..selection_end])
                        });
                        editor.focus(cx);
                    });
                    self.update_visible_entries(None, cx);
                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        }
    }

    fn trash(&mut self, action: &Trash, cx: &mut ViewContext<Self>) {
        self.remove(true, action.skip_prompt, cx);
    }

    fn delete(&mut self, action: &Delete, cx: &mut ViewContext<Self>) {
        self.remove(false, action.skip_prompt, cx);
    }

    fn remove(&mut self, trash: bool, skip_prompt: bool, cx: &mut ViewContext<'_, ProjectPanel>) {
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
                            format!("\n\n{dirty_buffers} of these have unsaved changes, which will be lost.")
                        };

                        format!(
                            "Do you want to {} the following {} files?\n{}{unsaved_warning}",
                            operation.to_lowercase(),
                            file_paths.len(),
                            names.join("\n")
                        )
                    }
                };
                Some(cx.prompt(PromptLevel::Info, &prompt, None, &[operation, "Cancel"]))
            } else {
                None
            };
            let next_selection = self.find_next_selection_after_deletion(items_to_delete, cx);
            cx.spawn(|panel, mut cx| async move {
                if let Some(answer) = answer {
                    if answer.await != Ok(0) {
                        return anyhow::Ok(());
                    }
                }
                for (entry_id, _) in file_paths {
                    panel
                        .update(&mut cx, |panel, cx| {
                            panel
                                .project
                                .update(cx, |project, cx| project.delete_entry(entry_id, trash, cx))
                                .context("no such entry")
                        })??
                        .await?;
                }
                panel.update(&mut cx, |panel, cx| {
                    if let Some(next_selection) = next_selection {
                        panel.selection = Some(next_selection);
                        panel.autoscroll(cx);
                    } else {
                        panel.select_last(&SelectLast {}, cx);
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
        cx: &mut ViewContext<Self>,
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
        let mut siblings: Vec<Entry> = worktree
            .snapshot()
            .child_entries(parent_path)
            .filter(|sibling| {
                sibling.id == latest_entry.id
                    || !marked_entries_in_worktree.contains(&&SelectedEntry {
                        worktree_id,
                        entry_id: sibling.id,
                    })
            })
            .cloned()
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

    fn unfold_directory(&mut self, _: &UnfoldDirectory, cx: &mut ViewContext<Self>) {
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

    fn fold_directory(&mut self, _: &FoldDirectory, cx: &mut ViewContext<Self>) {
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

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(edit_state) = &self.edit_state {
            if edit_state.processing_filename.is_none() {
                self.filename_editor.update(cx, |editor, cx| {
                    editor.move_to_end_of_line(
                        &editor::actions::MoveToEndOfLine {
                            stop_at_soft_wraps: false,
                        },
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
                    if cx.modifiers().shift {
                        self.marked_entries.insert(selection);
                    }

                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        } else {
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_parent(&mut self, _: &SelectParent, cx: &mut ViewContext<Self>) {
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
            self.select_first(&SelectFirst {}, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
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
                if cx.modifiers().shift {
                    self.marked_entries.insert(selection);
                }
                self.autoscroll(cx);
                cx.notify();
            }
        }
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        let worktree = self.visible_entries.last().and_then(|(worktree_id, _, _)| {
            self.project.read(cx).worktree_for_id(*worktree_id, cx)
        });
        if let Some(worktree) = worktree {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            if let Some(last_entry) = worktree.entries(true, 0).last() {
                self.selection = Some(SelectedEntry {
                    worktree_id,
                    entry_id: last_entry.id,
                });
                self.autoscroll(cx);
                cx.notify();
            }
        }
    }

    fn autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        if let Some((_, _, index)) = self.selection.and_then(|s| self.index_for_selection(s)) {
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Center);
            cx.notify();
        }
    }

    fn cut(&mut self, _: &Cut, cx: &mut ViewContext<Self>) {
        let entries = self.disjoint_entries(cx);
        if !entries.is_empty() {
            self.clipboard = Some(ClipboardEntry::Cut(entries));
            cx.notify();
        }
    }

    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        let entries = self.disjoint_entries(cx);
        if !entries.is_empty() {
            self.clipboard = Some(ClipboardEntry::Copied(entries));
            cx.notify();
        }
    }

    fn create_paste_path(
        &self,
        source: &SelectedEntry,
        (worktree, target_entry): (Model<Worktree>, &Entry),
        cx: &AppContext,
    ) -> Option<PathBuf> {
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
        let mut ix = 0;
        {
            let worktree = worktree.read(cx);
            while worktree.entry_for_path(&new_path).is_some() {
                new_path.pop();

                let mut new_file_name = file_name_without_extension.to_os_string();
                new_file_name.push(" copy");
                if ix > 0 {
                    new_file_name.push(format!(" {}", ix));
                }
                if let Some(extension) = extension.as_ref() {
                    new_file_name.push(".");
                    new_file_name.push(extension);
                }

                new_path.push(new_file_name);
                ix += 1;
            }
        }
        Some(new_path)
    }

    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
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
            let clip_is_cut = clipboard_entries.is_cut();
            for clipboard_entry in clipboard_entries.items() {
                let new_path =
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
            }

            cx.spawn(|project_panel, mut cx| async move {
                let mut last_succeed = None;
                let mut need_delete_ids = Vec::new();
                for ((entry_id, need_delete), task) in paste_entry_tasks.into_iter() {
                    match task {
                        PasteTask::Rename(task) => {
                            if let Some(CreatedEntry::Included(entry)) = task.await.log_err() {
                                last_succeed = Some(entry.id);
                            }
                        }
                        PasteTask::Copy(task) => {
                            if let Some(Some(entry)) = task.await.log_err() {
                                last_succeed = Some(entry.id);
                                if need_delete {
                                    need_delete_ids.push(entry_id);
                                }
                            }
                        }
                    }
                }
                // update selection
                if let Some(entry_id) = last_succeed {
                    project_panel
                        .update(&mut cx, |project_panel, _cx| {
                            project_panel.selection = Some(SelectedEntry {
                                worktree_id,
                                entry_id,
                            });
                        })
                        .ok();
                }
                // remove entry for cut in difference worktree
                for entry_id in need_delete_ids {
                    project_panel
                        .update(&mut cx, |project_panel, cx| {
                            project_panel
                                .project
                                .update(cx, |project, cx| project.delete_entry(entry_id, true, cx))
                                .ok_or_else(|| anyhow!("no such entry"))
                        })??
                        .await?;
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            self.expand_entry(worktree_id, entry.id, cx);
            Some(())
        });
    }

    fn duplicate(&mut self, _: &Duplicate, cx: &mut ViewContext<Self>) {
        self.copy(&Copy {}, cx);
        self.paste(&Paste {}, cx);
    }

    fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        let abs_file_paths = {
            let project = self.project.read(cx);
            self.marked_entries()
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

    fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        let file_paths = {
            let project = self.project.read(cx);
            self.marked_entries()
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

    fn reveal_in_finder(&mut self, _: &RevealInFileManager, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            cx.reveal_path(&worktree.read(cx).abs_path().join(&entry.path));
        }
    }

    fn remove_from_project(&mut self, _: &RemoveFromProject, cx: &mut ViewContext<Self>) {
        if let Some((worktree, _)) = self.selected_sub_entry(cx) {
            let worktree_id = worktree.read(cx).id();
            self.project
                .update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        }
    }

    fn open_system(&mut self, _: &OpenWithSystem, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let abs_path = worktree.abs_path().join(&entry.path);
            cx.open_with_system(&abs_path);
        }
    }

    fn open_in_terminal(&mut self, _: &OpenInTerminal, cx: &mut ViewContext<Self>) {
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
                cx.dispatch_action(workspace::OpenTerminal { working_directory }.boxed_clone())
            }
        }
    }

    pub fn new_search_in_directory(
        &mut self,
        _: &NewSearchInDirectory,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some((worktree, entry)) = self.selected_sub_entry(cx) {
            if entry.is_dir() {
                let include_root = self.project.read(cx).visible_worktrees(cx).count() > 1;
                let dir_path = if include_root {
                    let mut full_path = PathBuf::from(worktree.read(cx).root_name());
                    full_path.push(&entry.path);
                    Arc::from(full_path)
                } else {
                    entry.path.clone()
                };

                self.workspace
                    .update(cx, |workspace, cx| {
                        search::ProjectSearchView::new_search_in_directory(
                            workspace, &dir_path, cx,
                        );
                    })
                    .ok();
            }
        }
    }

    fn move_entry(
        &mut self,
        entry_to_move: ProjectEntryId,
        destination: ProjectEntryId,
        destination_is_file: bool,
        cx: &mut ViewContext<Self>,
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
        cx: &mut ViewContext<Self>,
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
        cx: &mut ViewContext<Self>,
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

    fn disjoint_entries(&self, cx: &AppContext) -> BTreeSet<SelectedEntry> {
        let marked_entries = self.marked_entries();
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

    // Returns the union of the currently selected entry and all marked entries.
    fn marked_entries(&self) -> BTreeSet<SelectedEntry> {
        let mut entries = self
            .marked_entries
            .iter()
            .map(|entry| SelectedEntry {
                entry_id: self.resolve_entry(entry.entry_id),
                worktree_id: entry.worktree_id,
            })
            .collect::<BTreeSet<_>>();

        if let Some(selection) = self.selection {
            entries.insert(SelectedEntry {
                entry_id: self.resolve_entry(selection.entry_id),
                worktree_id: selection.worktree_id,
            });
        }

        entries
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

    pub fn selected_entry<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<(&'a Worktree, &'a project::Entry)> {
        let (worktree, entry) = self.selected_entry_handle(cx)?;
        Some((worktree.read(cx), entry))
    }

    /// Compared to selected_entry, this function resolves to the currently
    /// selected subentry if dir auto-folding is enabled.
    fn selected_sub_entry<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<(Model<Worktree>, &'a project::Entry)> {
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
        cx: &'a AppContext,
    ) -> Option<(Model<Worktree>, &'a project::Entry)> {
        let selection = self.selection?;
        let project = self.project.read(cx);
        let worktree = project.worktree_for_id(selection.worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(selection.entry_id)?;
        Some((worktree, entry))
    }

    fn expand_to_selection(&mut self, cx: &mut ViewContext<Self>) -> Option<()> {
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
        cx: &mut ViewContext<Self>,
    ) {
        let auto_collapse_dirs = ProjectPanelSettings::get_global(cx).auto_fold_dirs;
        let project = self.project.read(cx);
        self.last_worktree_root_id = project
            .visible_worktrees(cx)
            .next_back()
            .and_then(|worktree| worktree.read(cx).root_entry())
            .map(|entry| entry.id);

        let old_ancestors = std::mem::take(&mut self.ancestors);
        self.visible_entries.clear();
        let mut max_width_item = None;
        for worktree in project.visible_worktrees(cx) {
            let snapshot = worktree.read(cx).snapshot();
            let worktree_id = snapshot.id();

            let expanded_dir_ids = match self.expanded_dir_ids.entry(worktree_id) {
                hash_map::Entry::Occupied(e) => e.into_mut(),
                hash_map::Entry::Vacant(e) => {
                    // The first time a worktree's root entry becomes available,
                    // mark that root entry as expanded.
                    if let Some(entry) = snapshot.root_entry() {
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
            let mut entry_iter = snapshot.entries(true, 0);
            let mut auto_folded_ancestors = vec![];
            while let Some(entry) = entry_iter.entry() {
                if auto_collapse_dirs && entry.kind.is_dir() {
                    auto_folded_ancestors.push(entry.id);
                    if !self.unfolded_dir_ids.contains(&entry.id) {
                        if let Some(root_path) = snapshot.root_entry() {
                            let mut child_entries = snapshot.child_entries(&entry.path);
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
                        .unwrap_or_default();
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
                visible_worktree_entries.push(entry.clone());
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
                if precedes_new_entry {
                    visible_worktree_entries.push(Entry {
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
                        git_status: entry.git_status,
                        canonical_path: entry.canonical_path.clone(),
                        char_bag: entry.char_bag,
                        is_fifo: entry.is_fifo,
                    });
                }
                let worktree_abs_path = worktree.read(cx).abs_path();
                let (depth, path) = if Some(entry) == worktree.read(cx).root_entry() {
                    let Some(path_name) = worktree_abs_path
                        .file_name()
                        .with_context(|| {
                            format!("Worktree abs path has no file name, root entry: {entry:?}")
                        })
                        .log_err()
                    else {
                        continue;
                    };
                    let path = Arc::from(Path::new(path_name));
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
                    let path = Arc::from(Path::new(path_name));
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
                                    Some(Arc::<Path>::from(full_path.join(suffix)))
                                })
                        })
                        .or_else(|| entry.path.file_name().map(Path::new).map(Arc::from))
                        .unwrap_or_else(|| entry.path.clone());
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

            snapshot.propagate_git_statuses(&mut visible_worktree_entries);
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
        cx: &mut ViewContext<Self>,
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
        cx: &mut ViewContext<Self>,
    ) {
        let mut paths: Vec<Arc<Path>> = paths.iter().map(|path| Arc::from(path.clone())).collect();

        let open_file_after_drop = paths.len() == 1 && paths[0].is_file();

        let Some((target_directory, worktree)) = maybe!({
            let worktree = self.project.read(cx).worktree_for_entry(entry_id, cx)?;
            let entry = worktree.read(cx).entry_for_id(entry_id)?;
            let path = worktree.read(cx).absolutize(&entry.path).ok()?;
            let target_directory = if path.is_dir() {
                path
            } else {
                path.parent()?.to_path_buf()
            };
            Some((target_directory, worktree))
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

        cx.spawn(|this, mut cx| {
            async move {
                for (filename, original_path) in &paths_to_replace {
                    let answer = cx
                        .prompt(
                            PromptLevel::Info,
                            format!("A file or folder with name {filename} already exists in the destination folder. Do you want to replace it?").as_str(),
                            None,
                            &["Replace", "Cancel"],
                        )
                        .await?;
                    if answer == 1 {
                        if let Some(item_idx) = paths.iter().position(|p| p == original_path) {
                            paths.remove(item_idx);
                        }
                    }
                }

                if paths.is_empty() {
                    return Ok(());
                }

                let task = worktree.update(&mut cx, |worktree, cx| {
                    worktree.copy_external_entries(target_directory, paths, true, cx)
                })?;

                let opened_entries = task.await?;
                this.update(&mut cx, |this, cx| {
                    if open_file_after_drop && !opened_entries.is_empty() {
                        this.open_entry(opened_entries[0], true, false, cx);
                    }
                })
            }
            .log_err()
        })
        .detach();
    }

    fn drag_onto(
        &mut self,
        selections: &DraggedSelection,
        target_entry_id: ProjectEntryId,
        is_file: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let should_copy = cx.modifiers().alt;
        if should_copy {
            let _ = maybe!({
                let project = self.project.read(cx);
                let target_worktree = project.worktree_for_entry(target_entry_id, cx)?;
                let target_entry = target_worktree
                    .read(cx)
                    .entry_for_id(target_entry_id)?
                    .clone();
                for selection in selections.items() {
                    let new_path = self.create_paste_path(
                        selection,
                        (target_worktree.clone(), &target_entry),
                        cx,
                    )?;
                    self.project
                        .update(cx, |project, cx| {
                            project.copy_entry(selection.entry_id, None, new_path, cx)
                        })
                        .detach_and_log_err(cx)
                }

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

    fn entry_at_index(&self, index: usize) -> Option<(WorktreeId, &Entry)> {
        let mut offset = 0;
        for (worktree_id, visible_worktree_entries, _) in &self.visible_entries {
            if visible_worktree_entries.len() > offset + index {
                return visible_worktree_entries
                    .get(index)
                    .map(|entry| (*worktree_id, entry));
            }
            offset += visible_worktree_entries.len();
        }
        None
    }

    fn iter_visible_entries(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<ProjectPanel>,
        mut callback: impl FnMut(&Entry, &HashSet<Arc<Path>>, &mut ViewContext<ProjectPanel>),
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
                callback(entry, entries, cx);
            }
            ix = end_ix;
        }
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<ProjectPanel>,
        mut callback: impl FnMut(ProjectEntryId, EntryDetails, &mut ViewContext<ProjectPanel>),
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
                    let status = git_status_setting.then_some(entry.git_status).flatten();
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
                        ProjectPanel::calculate_depth_and_difference(entry, entries);

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
                        is_hovered: self.hovered_entries.contains(&entry.id),
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
                                        Path::new(processing_filename).components().last()
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

                    callback(entry.id, details, cx);
                }
            }
            ix = end_ix;
        }
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
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
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
        let is_hovered = details.is_hovered;

        let width = self.size(cx);
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
        let is_local = self.project.read(cx).is_local();

        let dragged_selection = DraggedSelection {
            active_selection: selection,
            marked_selections: selections,
        };

        let (bg_color, border_color) = match (is_hovered, is_marked || is_active, self.mouse_down) {
            (true, _, true) => (item_colors.marked_active, item_colors.hover),
            (true, false, false) => (item_colors.hover, item_colors.hover),
            (true, true, false) => (item_colors.hover, item_colors.marked_active),
            (false, true, _) => (item_colors.marked_active, item_colors.marked_active),
            _ => (item_colors.default, item_colors.default),
        };

        div()
            .id(entry_id.to_proto() as usize)
            .when(is_local, |div| {
                div.on_drag_move::<ExternalPaths>(cx.listener(
                    move |this, event: &DragMoveEvent<ExternalPaths>, cx| {
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
                                let abs_path = worktree.absolutize(&path).log_err()?;
                                let path = if abs_path.is_dir() {
                                    path.as_ref()
                                } else {
                                    path.parent()?
                                };
                                let entry = worktree.entry_for_path(path)?;
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
                    move |this, external_paths: &ExternalPaths, cx| {
                        this.hover_scroll_task.take();
                        this.last_external_paths_drag_over_entry = None;
                        this.marked_entries.clear();
                        this.drop_external_files(external_paths.paths(), entry_id, cx);
                        cx.stop_propagation();
                    },
                ))
            })
            .on_drag(dragged_selection, move |selection, click_offset, cx| {
                cx.new_view(|_| DraggedProjectEntryView {
                    details: details.clone(),
                    width,
                    click_offset,
                    selection: selection.active_selection,
                    selections: selection.marked_selections.clone(),
                })
            })
            .drag_over::<DraggedSelection>(move |style, _, _| style.bg(item_colors.drag_over))
            .on_drop(cx.listener(move |this, selections: &DraggedSelection, cx| {
                this.hover_scroll_task.take();
                this.drag_onto(selections, entry_id, kind.is_file(), cx);
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, cx| {
                    this.mouse_down = true;
                    cx.propagate();
                }),
            )
            .on_hover(cx.listener(move |this, hover, cx| {
                if *hover {
                    this.hovered_entries.insert(entry_id);
                } else {
                    this.hovered_entries.remove(&entry_id);
                }
                cx.notify();
            }))
            .on_click(cx.listener(move |this, event: &gpui::ClickEvent, cx| {
                if event.down.button == MouseButton::Right || event.down.first_mouse || show_editor
                {
                    return;
                }
                if event.down.button == MouseButton::Left {
                    this.mouse_down = false;
                }
                cx.stop_propagation();

                if let Some(selection) = this.selection.filter(|_| event.down.modifiers.shift) {
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
                            cx,
                            |entry_id, details, _| {
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
                } else if event.down.modifiers.secondary() {
                    if event.down.click_count > 1 {
                        this.split_entry(entry_id, cx);
                    } else if !this.marked_entries.insert(selection) {
                        this.marked_entries.remove(&selection);
                    }
                } else if kind.is_dir() {
                    this.marked_entries.clear();
                    this.toggle_expanded(entry_id, cx);
                } else {
                    let preview_tabs_enabled = PreviewTabsSettings::get_global(cx).enabled;
                    let click_count = event.up.click_count;
                    let focus_opened_item = !preview_tabs_enabled || click_count > 1;
                    let allow_preview = preview_tabs_enabled && click_count == 1;
                    this.open_entry(entry_id, focus_opened_item, allow_preview, cx);
                }
            }))
            .cursor_pointer()
            .bg(bg_color)
            .border_color(border_color)
            .child(
                ListItem::new(entry_id.to_proto() as usize)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .selectable(false)
                    .when_some(canonical_path, |this, path| {
                        this.end_slot::<AnyElement>(
                            div()
                                .id("symlink_icon")
                                .pr_3()
                                .tooltip(move |cx| {
                                    Tooltip::with_meta(path.to_string(), None, "Symbolic Link", cx)
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
                        // Check if there's a diagnostic severity and get the decoration color
                        if let Some((_, decoration_color)) =
                            entry_diagnostic_aware_icon_decoration_and_color(diagnostic_severity)
                        {
                            // Determine if the diagnostic is a warning
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
                                            this = this.child(
                                                Label::new(DELIMITER.clone())
                                                    .single_line()
                                                    .color(filename_text_color),
                                            );
                                        }
                                        let id = SharedString::from(format!(
                                            "project_panel_path_component_{}_{index}",
                                            entry_id.to_usize()
                                        ));
                                        let label = div()
                                            .id(id)
                                            .on_click(cx.listener(move |this, _, cx| {
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
                                            .child(
                                                Label::new(component)
                                                    .single_line()
                                                    .color(filename_text_color)
                                                    .when(
                                                        index == active_index
                                                            && (is_active || is_marked),
                                                        |this| this.underline(true),
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
                        move |this, event: &MouseDownEvent, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            this.deploy_context_menu(event.position, entry_id, cx);
                        },
                    ))
                    .overflow_x(),
            )
            .border_1()
            .border_r_2()
            .rounded_none()
            .when(
                !self.mouse_down && is_active && self.focus_handle.contains_focused(cx),
                |this| this.border_color(Color::Selected.color(cx)),
            )
    }

    fn render_vertical_scrollbar(&self, cx: &mut ViewContext<Self>) -> Option<Stateful<Div>> {
        if !Self::should_show_scrollbar(cx)
            || !(self.show_scrollbar || self.vertical_scrollbar_state.is_dragging())
        {
            return None;
        }
        Some(
            div()
                .occlude()
                .id("project-panel-vertical-scroll")
                .on_mouse_move(cx.listener(|_, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, cx| {
                        if !this.vertical_scrollbar_state.is_dragging()
                            && !this.focus_handle.contains_focused(cx)
                        {
                            this.hide_scrollbar(cx);
                            cx.notify();
                        }

                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, cx| {
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

    fn render_horizontal_scrollbar(&self, cx: &mut ViewContext<Self>) -> Option<Stateful<Div>> {
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
                .on_mouse_move(cx.listener(|_, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, cx| {
                        if !this.horizontal_scrollbar_state.is_dragging()
                            && !this.focus_handle.contains_focused(cx)
                        {
                            this.hide_scrollbar(cx);
                            cx.notify();
                        }

                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, cx| {
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

    fn dispatch_context(&self, cx: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("ProjectPanel");
        dispatch_context.add("menu");

        let identifier = if self.filename_editor.focus_handle(cx).is_focused(cx) {
            "editing"
        } else {
            "not_editing"
        };

        dispatch_context.add(identifier);
        dispatch_context
    }

    fn should_show_scrollbar(cx: &AppContext) -> bool {
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

    fn should_autohide_scrollbar(cx: &AppContext) -> bool {
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

    fn hide_scrollbar(&mut self, cx: &mut ViewContext<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        if !Self::should_autohide_scrollbar(cx) {
            return;
        }
        self.hide_scrollbar_task = Some(cx.spawn(|panel, mut cx| async move {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            panel
                .update(&mut cx, |panel, cx| {
                    panel.show_scrollbar = false;
                    cx.notify();
                })
                .log_err();
        }))
    }

    fn reveal_entry(
        &mut self,
        project: Model<Project>,
        entry_id: ProjectEntryId,
        skip_ignored: bool,
        cx: &mut ViewContext<'_, Self>,
    ) {
        if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx) {
            let worktree = worktree.read(cx);
            if skip_ignored
                && worktree
                    .entry_for_id(entry_id)
                    .map_or(true, |entry| entry.is_ignored)
            {
                return;
            }

            let worktree_id = worktree.id();
            self.expand_entry(worktree_id, entry_id, cx);
            self.update_visible_entries(Some((worktree_id, entry_id)), cx);

            if self.marked_entries.len() == 1
                && self
                    .marked_entries
                    .first()
                    .filter(|entry| entry.entry_id == entry_id)
                    .is_none()
            {
                self.marked_entries.clear();
            }
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn find_active_indent_guide(
        &self,
        indent_guides: &[IndentGuideLayout],
        cx: &AppContext,
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
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
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
                cx: &mut ViewContext<ProjectPanel>,
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
                this.hover_scroll_task = Some(cx.spawn(move |this, mut cx| async move {
                    loop {
                        let should_stop_scrolling = this
                            .update(&mut cx, |this, cx| {
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
                .on_hover(cx.listener(|this, hovered, cx| {
                    if *hovered {
                        this.show_scrollbar = true;
                        this.hide_scrollbar_task.take();
                        cx.notify();
                    } else if !this.focus_handle.contains_focused(cx) {
                        this.hide_scrollbar(cx);
                    }
                }))
                .key_context(self.dispatch_context(cx))
                .on_action(cx.listener(Self::select_next))
                .on_action(cx.listener(Self::select_prev))
                .on_action(cx.listener(Self::select_first))
                .on_action(cx.listener(Self::select_last))
                .on_action(cx.listener(Self::select_parent))
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
                        .on_click(cx.listener(|this, event: &gpui::ClickEvent, cx| {
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

                                    this.new_file(&NewFile, cx);
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
                    cx.listener(move |this, event: &MouseDownEvent, cx| {
                        // When deploying the context menu anywhere below the last project entry,
                        // act as if the user clicked the root of the last worktree.
                        if let Some(entry_id) = this.last_worktree_root_id {
                            this.deploy_context_menu(event.position, entry_id, cx);
                        }
                    }),
                )
                .track_focus(&self.focus_handle(cx))
                .child(
                    uniform_list(cx.view().clone(), "entries", item_count, {
                        |this, range, cx| {
                            let mut items = Vec::with_capacity(range.end - range.start);
                            this.for_each_visible_entry(range, cx, |id, details, cx| {
                                items.push(this.render_entry(id, details, cx));
                            });
                            items
                        }
                    })
                    .when(show_indent_guides, |list| {
                        list.with_decoration(
                            ui::indent_guides(
                                cx.view().clone(),
                                px(indent_size),
                                IndentGuideColors::panel(cx),
                                |this, range, cx| {
                                    let mut items =
                                        SmallVec::with_capacity(range.end - range.start);
                                    this.iter_visible_entries(range, cx, |entry, entries, _| {
                                        let (depth, _) =
                                            Self::calculate_depth_and_difference(entry, entries);
                                        items.push(depth);
                                    });
                                    items
                                },
                            )
                            .on_click(cx.listener(
                                |this, active_indent_guide: &IndentGuideLayout, cx| {
                                    if cx.modifiers().secondary() {
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
                                cx.view().clone(),
                                move |this, params, cx| {
                                    const LEFT_OFFSET: f32 = 14.;
                                    const PADDING_Y: f32 = 4.;
                                    const HITBOX_OVERDRAW: f32 = 3.;

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
                                                px(PADDING_Y)
                                            };
                                            let bounds = Bounds::new(
                                                point(
                                                    px(layout.offset.x as f32) * indent_size
                                                        + px(LEFT_OFFSET),
                                                    px(layout.offset.y as f32) * item_height
                                                        + offset,
                                                ),
                                                size(
                                                    px(1.),
                                                    px(layout.length as f32) * item_height
                                                        - px(offset.0 * 2.),
                                                ),
                                            );
                                            ui::RenderedIndentGuide {
                                                bounds,
                                                layout,
                                                is_active: Some(idx) == active_indent_guide_index,
                                                hitbox: Some(Bounds::new(
                                                    point(
                                                        bounds.origin.x - px(HITBOX_OVERDRAW),
                                                        bounds.origin.y,
                                                    ),
                                                    size(
                                                        bounds.size.width
                                                            + px(2. * HITBOX_OVERDRAW),
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
                            .anchor(gpui::AnchorCorner::TopLeft)
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
                        .key_binding(KeyBinding::for_action(&workspace::Open, cx))
                        .on_click(cx.listener(|this, _, cx| {
                            this.workspace
                                .update(cx, |_, cx| cx.dispatch_action(Box::new(workspace::Open)))
                                .log_err();
                        })),
                )
                .when(is_local, |div| {
                    div.drag_over::<ExternalPaths>(|style, _, cx| {
                        style.bg(cx.theme().colors().drop_target_background)
                    })
                    .on_drop(cx.listener(
                        move |this, external_paths: &ExternalPaths, cx| {
                            this.last_external_paths_drag_over_entry = None;
                            this.marked_entries.clear();
                            this.hover_scroll_task.take();
                            if let Some(task) = this
                                .workspace
                                .update(cx, |workspace, cx| {
                                    workspace.open_workspace_for_paths(
                                        true,
                                        external_paths.paths().to_owned(),
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ProjectPanelSettings::get_global(cx);
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();

        h_flex().font(ui_font).map(|this| {
            if self.selections.len() > 1 && self.selections.contains(&self.selection) {
                this.flex_none()
                    .w(self.width)
                    .child(div().w(self.click_offset.x))
                    .child(
                        div()
                            .p_1()
                            .rounded_xl()
                            .bg(cx.theme().colors().background)
                            .child(Label::new(format!("{} entries", self.selections.len()))),
                    )
            } else {
                this.w(self.width).bg(cx.theme().colors().background).child(
                    ListItem::new(self.selection.entry_id.to_proto() as usize)
                        .indent_level(self.details.depth)
                        .indent_step_size(px(settings.indent_size))
                        .child(if let Some(icon) = &self.details.icon {
                            div().child(Icon::from_path(icon.clone()))
                        } else {
                            div()
                        })
                        .child(Label::new(self.details.filename.clone())),
                )
            }
        })
    }
}

impl EventEmitter<Event> for ProjectPanel {}

impl EventEmitter<PanelEvent> for ProjectPanel {}

impl Panel for ProjectPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        match ProjectPanelSettings::get_global(cx).dock {
            ProjectPanelDockPosition::Left => DockPosition::Left,
            ProjectPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
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

    fn size(&self, cx: &WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| ProjectPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        ProjectPanelSettings::get_global(cx)
            .button
            .then_some(IconName::FileTree)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Project Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "Project Panel"
    }

    fn starts_open(&self, cx: &WindowContext) -> bool {
        let project = &self.project.read(cx);
        project.visible_worktrees(cx).any(|tree| {
            tree.read(cx)
                .root_entry()
                .map_or(false, |entry| entry.is_dir())
        })
    }
}

impl FocusableView for ProjectPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
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
mod tests {
    use super::*;
    use collections::HashSet;
    use gpui::{Empty, TestAppContext, View, VisualTestContext, WindowHandle};
    use pretty_assertions::assert_eq;
    use project::{FakeFs, WorktreeSettings};
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::{Path, PathBuf};
    use ui::Context;
    use workspace::{
        item::{Item, ProjectItem},
        register_project_item, AppState,
    };

    #[gpui::test]
    async fn test_visible_list(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
                "C": {
                    "5": {},
                    "6": { "V": "", "W": "" },
                    "7": { "X": "" },
                    "8": { "Y": {}, "Z": "" }
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "d": {
                    "9": ""
                },
                "e": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        toggle_expand_dir(&panel, "root1/b", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b  <== selected",
                "        > 3",
                "        > 4",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        assert_eq!(
            visible_entries_as_strings(&panel, 6..9, cx),
            &[
                //
                "    > C",
                "      .dockerignore",
                "v root2",
            ]
        );
    }

    #[gpui::test]
    async fn test_opening_file(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "src/test", cx);
        select_path(&panel, "src/test/first.rs", cx);
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs  <== selected  <== marked",
                "          second.rs",
                "          third.rs"
            ]
        );
        ensure_single_file_is_opened(&workspace, "test/first.rs", cx);

        select_path(&panel, "src/test/second.rs", cx);
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs",
                "          second.rs  <== selected  <== marked",
                "          third.rs"
            ]
        );
        ensure_single_file_is_opened(&workspace, "test/second.rs", cx);
    }

    #[gpui::test]
    async fn test_exclusions_in_visible_list(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                    worktree_settings.file_scan_exclusions =
                        Some(vec!["**/.git".to_string(), "**/4/**".to_string()]);
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
                "C": {
                    "5": {},
                    "6": { "V": "", "W": "" },
                    "7": { "X": "" },
                    "8": { "Y": {}, "Z": "" }
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "d": {
                    "4": ""
                },
                "e": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        toggle_expand_dir(&panel, "root1/b", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > a",
                "    v b  <== selected",
                "        > 3",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        toggle_expand_dir(&panel, "root2/d", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > a",
                "    v b",
                "        > 3",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    v d  <== selected",
                "    > e",
            ]
        );

        toggle_expand_dir(&panel, "root2/e", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > a",
                "    v b",
                "        > 3",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    v d",
                "    v e  <== selected",
            ]
        );
    }

    #[gpui::test]
    async fn test_auto_collapse_dir_paths(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                "dir_1": {
                    "nested_dir_1": {
                        "nested_dir_2": {
                            "nested_dir_3": {
                                "file_a.java": "// File contents",
                                "file_b.java": "// File contents",
                                "file_c.java": "// File contents",
                                "nested_dir_4": {
                                    "nested_dir_5": {
                                        "file_d.java": "// File contents",
                                    }
                                }
                            }
                        }
                    }
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "dir_2": {
                    "file_1.java": "// File contents",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        cx.update(|cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    auto_fold_dirs: true,
                    ..settings
                },
                cx,
            );
        });
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
                "v root2",
                "    > dir_2",
            ]
        );

        toggle_expand_dir(
            &panel,
            "root1/dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
            cx,
        );
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3  <== selected",
                "        > nested_dir_4/nested_dir_5",
                "          file_a.java",
                "          file_b.java",
                "          file_c.java",
                "v root2",
                "    > dir_2",
            ]
        );

        toggle_expand_dir(
            &panel,
            "root1/dir_1/nested_dir_1/nested_dir_2/nested_dir_3/nested_dir_4/nested_dir_5",
            cx,
        );
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
                "        v nested_dir_4/nested_dir_5  <== selected",
                "              file_d.java",
                "          file_a.java",
                "          file_b.java",
                "          file_c.java",
                "v root2",
                "    > dir_2",
            ]
        );
        toggle_expand_dir(&panel, "root2/dir_2", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
                "        v nested_dir_4/nested_dir_5",
                "              file_d.java",
                "          file_a.java",
                "          file_b.java",
                "          file_c.java",
                "v root2",
                "    v dir_2  <== selected",
                "          file_1.java",
            ]
        );
    }

    #[gpui::test(iterations = 30)]
    async fn test_editing_files(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
                "C": {
                    "5": {},
                    "6": { "V": "", "W": "" },
                    "7": { "X": "" },
                    "8": { "Y": {}, "Z": "" }
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "d": {
                    "9": ""
                },
                "e": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace
            .update(cx, |workspace, cx| {
                let panel = ProjectPanel::new(workspace, cx);
                workspace.add_panel(panel.clone(), cx);
                panel
            })
            .unwrap();

        select_path(&panel, "root1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1  <== selected",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        // Add a file with the root folder selected. The filename editor is placed
        // before the first file in the root folder.
        panel.update(cx, |panel, cx| panel.new_file(&NewFile, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      [EDITOR: '']  <== selected",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        let confirm = panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("the-new-filename", cx));
            panel.confirm_edit(cx).unwrap()
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      [PROCESSING: 'the-new-filename']  <== selected",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        confirm.await.unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "      the-new-filename  <== selected  <== marked",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        select_path(&panel, "root1/b", cx);
        panel.update(cx, |panel, cx| panel.new_file(&NewFile, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          [EDITOR: '']  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        panel
            .update(cx, |panel, cx| {
                panel
                    .filename_editor
                    .update(cx, |editor, cx| editor.set_text("another-filename.txt", cx));
                panel.confirm_edit(cx).unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          another-filename.txt  <== selected  <== marked",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        select_path(&panel, "root1/b/another-filename.txt", cx);
        panel.update(cx, |panel, cx| panel.rename(&Rename, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          [EDITOR: 'another-filename.txt']  <== selected  <== marked",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        let confirm = panel.update(cx, |panel, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                let file_name_selections = editor.selections.all::<usize>(cx);
                assert_eq!(file_name_selections.len(), 1, "File editing should have a single selection, but got: {file_name_selections:?}");
                let file_name_selection = &file_name_selections[0];
                assert_eq!(file_name_selection.start, 0, "Should select the file name from the start");
                assert_eq!(file_name_selection.end, "another-filename".len(), "Should not select file extension");

                editor.set_text("a-different-filename.tar.gz", cx)
            });
            panel.confirm_edit(cx).unwrap()
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          [PROCESSING: 'a-different-filename.tar.gz']  <== selected  <== marked",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        confirm.await.unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          a-different-filename.tar.gz  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        panel.update(cx, |panel, cx| panel.rename(&Rename, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          [EDITOR: 'a-different-filename.tar.gz']  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                let file_name_selections = editor.selections.all::<usize>(cx);
                assert_eq!(file_name_selections.len(), 1, "File editing should have a single selection, but got: {file_name_selections:?}");
                let file_name_selection = &file_name_selections[0];
                assert_eq!(file_name_selection.start, 0, "Should select the file name from the start");
                assert_eq!(file_name_selection.end, "a-different-filename.tar".len(), "Should not select file extension, but still may select anything up to the last dot..");

            });
            panel.cancel(&menu::Cancel, cx)
        });

        panel.update(cx, |panel, cx| panel.new_directory(&NewDirectory, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "        > [EDITOR: '']  <== selected",
                "          a-different-filename.tar.gz",
                "    > C",
                "      .dockerignore",
            ]
        );

        let confirm = panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("new-dir", cx));
            panel.confirm_edit(cx).unwrap()
        });
        panel.update(cx, |panel, cx| panel.select_next(&Default::default(), cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "        > [PROCESSING: 'new-dir']",
                "          a-different-filename.tar.gz  <== selected",
                "    > C",
                "      .dockerignore",
            ]
        );

        confirm.await.unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "        > new-dir",
                "          a-different-filename.tar.gz  <== selected",
                "    > C",
                "      .dockerignore",
            ]
        );

        panel.update(cx, |panel, cx| panel.rename(&Default::default(), cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "        > new-dir",
                "          [EDITOR: 'a-different-filename.tar.gz']  <== selected",
                "    > C",
                "      .dockerignore",
            ]
        );

        // Dismiss the rename editor when it loses focus.
        workspace.update(cx, |_, cx| cx.blur()).unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "        > new-dir",
                "          a-different-filename.tar.gz  <== selected",
                "    > C",
                "      .dockerignore",
            ]
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_adding_directories_via_file(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
                "C": {
                    "5": {},
                    "6": { "V": "", "W": "" },
                    "7": { "X": "" },
                    "8": { "Y": {}, "Z": "" }
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "d": {
                    "9": ""
                },
                "e": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace
            .update(cx, |workspace, cx| {
                let panel = ProjectPanel::new(workspace, cx);
                workspace.add_panel(panel.clone(), cx);
                panel
            })
            .unwrap();

        select_path(&panel, "root1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1  <== selected",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        // Add a file with the root folder selected. The filename editor is placed
        // before the first file in the root folder.
        panel.update(cx, |panel, cx| panel.new_file(&NewFile, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      [EDITOR: '']  <== selected",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        let confirm = panel.update(cx, |panel, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text("/bdir1/dir2/the-new-filename", cx)
            });
            panel.confirm_edit(cx).unwrap()
        });

        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    > b",
                "    > C",
                "      [PROCESSING: '/bdir1/dir2/the-new-filename']  <== selected",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        confirm.await.unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..13, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    > b",
                "    v bdir1",
                "        v dir2",
                "              the-new-filename  <== selected  <== marked",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );
    }

    #[gpui::test]
    async fn test_adding_directory_via_file(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace
            .update(cx, |workspace, cx| {
                let panel = ProjectPanel::new(workspace, cx);
                workspace.add_panel(panel.clone(), cx);
                panel
            })
            .unwrap();

        select_path(&panel, "root1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v root1  <== selected", "    > .git", "      .dockerignore",]
        );

        // Add a file with the root folder selected. The filename editor is placed
        // before the first file in the root folder.
        panel.update(cx, |panel, cx| panel.new_file(&NewFile, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "      [EDITOR: '']  <== selected",
                "      .dockerignore",
            ]
        );

        let confirm = panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("/new_dir/", cx));
            panel.confirm_edit(cx).unwrap()
        });

        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "      [PROCESSING: '/new_dir/']  <== selected",
                "      .dockerignore",
            ]
        );

        confirm.await.unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..13, cx),
            &[
                "v root1",
                "    > .git",
                "    v new_dir  <== selected",
                "      .dockerignore",
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_paste(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                "one.two.txt": "",
                "one.txt": ""
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
            panel.select_next(&Default::default(), cx);
        });

        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "      one.txt  <== selected",
                "      one.two.txt",
            ]
        );

        // Regression test - file name is created correctly when
        // the copied file's name contains multiple dots.
        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });
        cx.executor().run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "      one.txt",
                "      one copy.txt  <== selected",
                "      one.two.txt",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel.paste(&Default::default(), cx);
        });
        cx.executor().run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "      one.txt",
                "      one copy.txt",
                "      one copy 1.txt  <== selected",
                "      one.two.txt",
            ]
        );
    }

    #[gpui::test]
    async fn test_cut_paste_between_different_worktrees(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                "one.txt": "",
                "two.txt": "",
                "three.txt": "",
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
            }),
        )
        .await;

        fs.insert_tree(
            "/root2",
            json!({
                "one.txt": "",
                "two.txt": "",
                "four.txt": "",
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        select_path(&panel, "root1/three.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.cut(&Default::default(), cx);
        });

        select_path(&panel, "root2/one.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "    > a",
                "      one.txt",
                "      two.txt",
                "v root2",
                "    > b",
                "      four.txt",
                "      one.txt",
                "      three.txt  <== selected",
                "      two.txt",
            ]
        );

        select_path(&panel, "root1/a", cx);
        panel.update(cx, |panel, cx| {
            panel.cut(&Default::default(), cx);
        });
        select_path(&panel, "root2/two.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });

        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "      one.txt",
                "      two.txt",
                "v root2",
                "    > a  <== selected",
                "    > b",
                "      four.txt",
                "      one.txt",
                "      three.txt",
                "      two.txt",
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_paste_between_different_worktrees(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                "one.txt": "",
                "two.txt": "",
                "three.txt": "",
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
            }),
        )
        .await;

        fs.insert_tree(
            "/root2",
            json!({
                "one.txt": "",
                "two.txt": "",
                "four.txt": "",
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        select_path(&panel, "root1/three.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
        });

        select_path(&panel, "root2/one.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "    > a",
                "      one.txt",
                "      three.txt",
                "      two.txt",
                "v root2",
                "    > b",
                "      four.txt",
                "      one.txt",
                "      three.txt  <== selected",
                "      two.txt",
            ]
        );

        select_path(&panel, "root1/three.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
        });
        select_path(&panel, "root2/two.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });

        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "    > a",
                "      one.txt",
                "      three.txt",
                "      two.txt",
                "v root2",
                "    > b",
                "      four.txt",
                "      one.txt",
                "      three.txt",
                "      three copy.txt  <== selected",
                "      two.txt",
            ]
        );

        select_path(&panel, "root1/a", cx);
        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
        });
        select_path(&panel, "root2/two.txt", cx);
        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });

        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "    > a",
                "      one.txt",
                "      three.txt",
                "      two.txt",
                "v root2",
                "    > a  <== selected",
                "    > b",
                "      four.txt",
                "      one.txt",
                "      three.txt",
                "      three copy.txt",
                "      two.txt",
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_paste_directory(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root",
            json!({
                "a": {
                    "one.txt": "",
                    "two.txt": "",
                    "inner_dir": {
                        "three.txt": "",
                        "four.txt": "",
                    }
                },
                "b": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        select_path(&panel, "root/a", cx);
        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
            panel.select_next(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });
        cx.executor().run_until_parked();

        let pasted_dir = find_project_entry(&panel, "root/b/a", cx);
        assert_ne!(pasted_dir, None, "Pasted directory should have an entry");

        let pasted_dir_file = find_project_entry(&panel, "root/b/a/one.txt", cx);
        assert_ne!(
            pasted_dir_file, None,
            "Pasted directory file should have an entry"
        );

        let pasted_dir_inner_dir = find_project_entry(&panel, "root/b/a/inner_dir", cx);
        assert_ne!(
            pasted_dir_inner_dir, None,
            "Directories inside pasted directory should have an entry"
        );

        toggle_expand_dir(&panel, "root/b/a", cx);
        toggle_expand_dir(&panel, "root/b/a/inner_dir", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root",
                "    > a",
                "    v b",
                "        v a",
                "            v inner_dir  <== selected",
                "                  four.txt",
                "                  three.txt",
                "              one.txt",
                "              two.txt",
            ]
        );

        select_path(&panel, "root", cx);
        panel.update(cx, |panel, cx| panel.paste(&Default::default(), cx));
        cx.executor().run_until_parked();
        panel.update(cx, |panel, cx| panel.paste(&Default::default(), cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root",
                "    > a",
                "    v a copy",
                "        > a  <== selected",
                "        > inner_dir",
                "          one.txt",
                "          two.txt",
                "    v b",
                "        v a",
                "            v inner_dir",
                "                  four.txt",
                "                  three.txt",
                "              one.txt",
                "              two.txt"
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_paste_directory_with_sibling_file(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/test",
            json!({
                "dir1": {
                    "a.txt": "",
                    "b.txt": "",
                },
                "dir2": {},
                "c.txt": "",
                "d.txt": "",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "test/dir1", cx);

        cx.simulate_modifiers_change(gpui::Modifiers {
            control: true,
            ..Default::default()
        });

        select_path_with_mark(&panel, "test/dir1", cx);
        select_path_with_mark(&panel, "test/c.txt", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v test",
                "    v dir1  <== marked",
                "          a.txt",
                "          b.txt",
                "    > dir2",
                "      c.txt  <== selected  <== marked",
                "      d.txt",
            ],
            "Initial state before copying dir1 and c.txt"
        );

        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
        });
        select_path(&panel, "test/dir2", cx);
        panel.update(cx, |panel, cx| {
            panel.paste(&Default::default(), cx);
        });
        cx.executor().run_until_parked();

        toggle_expand_dir(&panel, "test/dir2/dir1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v test",
                "    v dir1  <== marked",
                "          a.txt",
                "          b.txt",
                "    v dir2",
                "        v dir1  <== selected",
                "              a.txt",
                "              b.txt",
                "          c.txt",
                "      c.txt  <== marked",
                "      d.txt",
            ],
            "Should copy dir1 as well as c.txt into dir2"
        );
    }

    #[gpui::test]
    async fn test_copy_paste_nested_and_root_entries(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/test",
            json!({
                "dir1": {
                    "a.txt": "",
                    "b.txt": "",
                },
                "dir2": {},
                "c.txt": "",
                "d.txt": "",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "test/dir1", cx);

        cx.simulate_modifiers_change(gpui::Modifiers {
            control: true,
            ..Default::default()
        });

        select_path_with_mark(&panel, "test/dir1/a.txt", cx);
        select_path_with_mark(&panel, "test/dir1", cx);
        select_path_with_mark(&panel, "test/c.txt", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v test",
                "    v dir1  <== marked",
                "          a.txt  <== marked",
                "          b.txt",
                "    > dir2",
                "      c.txt  <== selected  <== marked",
                "      d.txt",
            ],
            "Initial state before copying a.txt, dir1 and c.txt"
        );

        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
        });
        select_path(&panel, "test/dir2", cx);
        panel.update(cx, |panel, cx| {
            panel.paste(&Default::default(), cx);
        });
        cx.executor().run_until_parked();

        toggle_expand_dir(&panel, "test/dir2/dir1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v test",
                "    v dir1  <== marked",
                "          a.txt  <== marked",
                "          b.txt",
                "    v dir2",
                "        v dir1  <== selected",
                "              a.txt",
                "              b.txt",
                "          c.txt",
                "      c.txt  <== marked",
                "      d.txt",
            ],
            "Should copy dir1 and c.txt into dir2. a.txt is already present in copied dir1."
        );
    }

    #[gpui::test]
    async fn test_remove_opened_file(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "src/test", cx);
        select_path(&panel, "src/test/first.rs", cx);
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs  <== selected  <== marked",
                "          second.rs",
                "          third.rs"
            ]
        );
        ensure_single_file_is_opened(&workspace, "test/first.rs", cx);

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          second.rs  <== selected",
                "          third.rs"
            ],
            "Project panel should have no deleted file, no other file is selected in it"
        );
        ensure_no_open_items_and_panes(&workspace, cx);

        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          second.rs  <== selected  <== marked",
                "          third.rs"
            ]
        );
        ensure_single_file_is_opened(&workspace, "test/second.rs", cx);

        workspace
            .update(cx, |workspace, cx| {
                let active_items = workspace
                    .panes()
                    .iter()
                    .filter_map(|pane| pane.read(cx).active_item())
                    .collect::<Vec<_>>();
                assert_eq!(active_items.len(), 1);
                let open_editor = active_items
                    .into_iter()
                    .next()
                    .unwrap()
                    .downcast::<Editor>()
                    .expect("Open item should be an editor");
                open_editor.update(cx, |editor, cx| editor.set_text("Another text!", cx));
            })
            .unwrap();
        submit_deletion_skipping_prompt(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v src", "    v test", "          third.rs  <== selected"],
            "Project panel should have no deleted file, with one last file remaining"
        );
        ensure_no_open_items_and_panes(&workspace, cx);
    }

    #[gpui::test]
    async fn test_create_duplicate_items(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace
            .update(cx, |workspace, cx| {
                let panel = ProjectPanel::new(workspace, cx);
                workspace.add_panel(panel.clone(), cx);
                panel
            })
            .unwrap();

        select_path(&panel, "src/", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                //
                "v src  <== selected",
                "    > test"
            ]
        );
        panel.update(cx, |panel, cx| panel.new_directory(&NewDirectory, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                //
                "v src",
                "    > [EDITOR: '']  <== selected",
                "    > test"
            ]
        );
        panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("test", cx));
            assert!(
                panel.confirm_edit(cx).is_none(),
                "Should not allow to confirm on conflicting new directory name"
            )
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                //
                "v src",
                "    > test"
            ],
            "File list should be unchanged after failed folder create confirmation"
        );

        select_path(&panel, "src/test/", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                //
                "v src",
                "    > test  <== selected"
            ]
        );
        panel.update(cx, |panel, cx| panel.new_file(&NewFile, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          [EDITOR: '']  <== selected",
                "          first.rs",
                "          second.rs",
                "          third.rs"
            ]
        );
        panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("first.rs", cx));
            assert!(
                panel.confirm_edit(cx).is_none(),
                "Should not allow to confirm on conflicting new file name"
            )
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs",
                "          second.rs",
                "          third.rs"
            ],
            "File list should be unchanged after failed file create confirmation"
        );

        select_path(&panel, "src/test/first.rs", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs  <== selected",
                "          second.rs",
                "          third.rs"
            ],
        );
        panel.update(cx, |panel, cx| panel.rename(&Rename, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          [EDITOR: 'first.rs']  <== selected",
                "          second.rs",
                "          third.rs"
            ]
        );
        panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("second.rs", cx));
            assert!(
                panel.confirm_edit(cx).is_none(),
                "Should not allow to confirm on conflicting file rename"
            )
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs  <== selected",
                "          second.rs",
                "          third.rs"
            ],
            "File list should be unchanged after failed rename confirmation"
        );
    }

    #[gpui::test]
    async fn test_dir_toggle_collapse(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/project_root",
            json!({
                "dir_1": {
                    "nested_dir": {
                        "file_a.py": "# File contents",
                    }
                },
                "file_1.py": "# File contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
        select_path(&panel, "project_root/dir_1", cx);
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        select_path(&panel, "project_root/dir_1/nested_dir", cx);
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        > nested_dir  <== selected",
                "      file_1.py",
            ]
        );
    }

    #[gpui::test]
    async fn test_collapse_all_entries(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/project_root",
            json!({
                "dir_1": {
                    "nested_dir": {
                        "file_a.py": "# File contents",
                        "file_b.py": "# File contents",
                        "file_c.py": "# File contents",
                    },
                    "file_1.py": "# File contents",
                    "file_2.py": "# File contents",
                    "file_3.py": "# File contents",
                },
                "dir_2": {
                    "file_1.py": "# File contents",
                    "file_2.py": "# File contents",
                    "file_3.py": "# File contents",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        panel.update(cx, |panel, cx| {
            panel.collapse_all_entries(&CollapseAllEntries, cx)
        });
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v project_root", "    > dir_1", "    > dir_2",]
        );

        // Open dir_1 and make sure nested_dir was collapsed when running collapse_all_entries
        toggle_expand_dir(&panel, "project_root/dir_1", cx);
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1  <== selected",
                "        > nested_dir",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    > dir_2",
            ]
        );
    }

    #[gpui::test]
    async fn test_new_file_move(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.as_fake().insert_tree("/root", json!({})).await;
        let project = Project::test(fs, ["/root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        // Make a new buffer with no backing file
        workspace
            .update(cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
            .unwrap();

        cx.executor().run_until_parked();

        // "Save as" the buffer, creating a new backing file for it
        let save_task = workspace
            .update(cx, |workspace, cx| {
                workspace.save_active_item(workspace::SaveIntent::Save, cx)
            })
            .unwrap();

        cx.executor().run_until_parked();
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/new")));
        save_task.await.unwrap();

        // Rename the file
        select_path(&panel, "root/new", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v root", "      new  <== selected"]
        );
        panel.update(cx, |panel, cx| panel.rename(&Rename, cx));
        panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("newer", cx));
        });
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));

        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v root", "      newer  <== selected"]
        );

        workspace
            .update(cx, |workspace, cx| {
                workspace.save_active_item(workspace::SaveIntent::Save, cx)
            })
            .unwrap()
            .await
            .unwrap();

        cx.executor().run_until_parked();
        // assert that saving the file doesn't restore "new"
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v root", "      newer  <== selected"]
        );
    }

    #[gpui::test]
    async fn test_multiple_marked_entries(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);
        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/project_root",
            json!({
                "dir_1": {
                    "nested_dir": {
                        "file_a.py": "# File contents",
                    }
                },
                "file_1.py": "# File contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
        let worktree_id =
            cx.update(|cx| project.read(cx).worktrees(cx).next().unwrap().read(cx).id());
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                this.select_next(&Default::default(), cx);
                this.expand_selected_entry(&Default::default(), cx);
                this.expand_selected_entry(&Default::default(), cx);
                this.select_next(&Default::default(), cx);
                this.expand_selected_entry(&Default::default(), cx);
                this.select_next(&Default::default(), cx);
            })
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir",
                "              file_a.py  <== selected",
                "      file_1.py",
            ]
        );
        let modifiers_with_shift = gpui::Modifiers {
            shift: true,
            ..Default::default()
        };
        cx.simulate_modifiers_change(modifiers_with_shift);
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                this.select_next(&Default::default(), cx);
            })
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir",
                "              file_a.py",
                "      file_1.py  <== selected  <== marked",
            ]
        );
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                this.select_prev(&Default::default(), cx);
            })
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir",
                "              file_a.py  <== selected  <== marked",
                "      file_1.py  <== marked",
            ]
        );
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                let drag = DraggedSelection {
                    active_selection: this.selection.unwrap(),
                    marked_selections: Arc::new(this.marked_entries.clone()),
                };
                let target_entry = this
                    .project
                    .read(cx)
                    .entry_for_path(&(worktree_id, "").into(), cx)
                    .unwrap();
                this.drag_onto(&drag, target_entry.id, false, cx);
            });
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir",
                "      file_1.py  <== marked",
                "      file_a.py  <== selected  <== marked",
            ]
        );
        // ESC clears out all marks
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                this.cancel(&menu::Cancel, cx);
            })
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir",
                "      file_1.py",
                "      file_a.py  <== selected",
            ]
        );
        // ESC clears out all marks
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                this.select_prev(&SelectPrev, cx);
                this.select_next(&SelectNext, cx);
            })
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir",
                "      file_1.py  <== marked",
                "      file_a.py  <== selected  <== marked",
            ]
        );
        cx.simulate_modifiers_change(Default::default());
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                this.cut(&Cut, cx);
                this.select_prev(&SelectPrev, cx);
                this.select_prev(&SelectPrev, cx);

                this.paste(&Paste, cx);
                // this.expand_selected_entry(&ExpandSelectedEntry, cx);
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir",
                "              file_1.py  <== marked",
                "              file_a.py  <== selected  <== marked",
            ]
        );
        cx.simulate_modifiers_change(modifiers_with_shift);
        cx.update(|cx| {
            panel.update(cx, |this, cx| {
                this.expand_selected_entry(&Default::default(), cx);
                this.select_next(&SelectNext, cx);
                this.select_next(&SelectNext, cx);
            })
        });
        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v project_root",
                "    v dir_1",
                "        v nested_dir  <== selected",
            ]
        );
    }
    #[gpui::test]
    async fn test_autoreveal_and_gitignored_files(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                    worktree_settings.file_scan_exclusions = Some(Vec::new());
                });
                store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                    project_panel_settings.auto_reveal_entries = Some(false)
                });
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/project_root",
            json!({
                ".git": {},
                ".gitignore": "**/gitignored_dir",
                "dir_1": {
                    "file_1.py": "# File 1_1 contents",
                    "file_2.py": "# File 1_2 contents",
                    "file_3.py": "# File 1_3 contents",
                    "gitignored_dir": {
                        "file_a.py": "# File contents",
                        "file_b.py": "# File contents",
                        "file_c.py": "# File contents",
                    },
                },
                "dir_2": {
                    "file_1.py": "# File 2_1 contents",
                    "file_2.py": "# File 2_2 contents",
                    "file_3.py": "# File 2_3 contents",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    > dir_1",
                "    > dir_2",
                "      .gitignore",
            ]
        );

        let dir_1_file = find_project_entry(&panel, "project_root/dir_1/file_1.py", cx)
            .expect("dir 1 file is not ignored and should have an entry");
        let dir_2_file = find_project_entry(&panel, "project_root/dir_2/file_1.py", cx)
            .expect("dir 2 file is not ignored and should have an entry");
        let gitignored_dir_file =
            find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx);
        assert_eq!(
            gitignored_dir_file, None,
            "File in the gitignored dir should not have an entry before its dir is toggled"
        );

        toggle_expand_dir(&panel, "project_root/dir_1", cx);
        toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        v gitignored_dir  <== selected",
                "              file_a.py",
                "              file_b.py",
                "              file_c.py",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    > dir_2",
                "      .gitignore",
            ],
            "Should show gitignored dir file list in the project panel"
        );
        let gitignored_dir_file =
            find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx)
                .expect("after gitignored dir got opened, a file entry should be present");

        toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
        toggle_expand_dir(&panel, "project_root/dir_1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    > dir_1  <== selected",
                "    > dir_2",
                "      .gitignore",
            ],
            "Should hide all dir contents again and prepare for the auto reveal test"
        );

        for file_entry in [dir_1_file, dir_2_file, gitignored_dir_file] {
            panel.update(cx, |panel, cx| {
                panel.project.update(cx, |_, cx| {
                    cx.emit(project::Event::ActiveEntryChanged(Some(file_entry)))
                })
            });
            cx.run_until_parked();
            assert_eq!(
                visible_entries_as_strings(&panel, 0..20, cx),
                &[
                    "v project_root",
                    "    > .git",
                    "    > dir_1  <== selected",
                    "    > dir_2",
                    "      .gitignore",
                ],
                "When no auto reveal is enabled, the selected entry should not be revealed in the project panel"
            );
        }

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                    project_panel_settings.auto_reveal_entries = Some(true)
                });
            })
        });

        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::ActiveEntryChanged(Some(dir_1_file)))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        > gitignored_dir",
                "          file_1.py  <== selected",
                "          file_2.py",
                "          file_3.py",
                "    > dir_2",
                "      .gitignore",
            ],
            "When auto reveal is enabled, not ignored dir_1 entry should be revealed"
        );

        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::ActiveEntryChanged(Some(dir_2_file)))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        > gitignored_dir",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    v dir_2",
                "          file_1.py  <== selected",
                "          file_2.py",
                "          file_3.py",
                "      .gitignore",
            ],
            "When auto reveal is enabled, not ignored dir_2 entry should be revealed"
        );

        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::ActiveEntryChanged(Some(
                    gitignored_dir_file,
                )))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        > gitignored_dir",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    v dir_2",
                "          file_1.py  <== selected",
                "          file_2.py",
                "          file_3.py",
                "      .gitignore",
            ],
            "When auto reveal is enabled, a gitignored selected entry should not be revealed in the project panel"
        );

        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::RevealInProjectPanel(gitignored_dir_file))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        v gitignored_dir",
                "              file_a.py  <== selected",
                "              file_b.py",
                "              file_c.py",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    v dir_2",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "      .gitignore",
            ],
            "When a gitignored entry is explicitly revealed, it should be shown in the project tree"
        );
    }

    #[gpui::test]
    async fn test_explicit_reveal(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                    worktree_settings.file_scan_exclusions = Some(Vec::new());
                });
                store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                    project_panel_settings.auto_reveal_entries = Some(false)
                });
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/project_root",
            json!({
                ".git": {},
                ".gitignore": "**/gitignored_dir",
                "dir_1": {
                    "file_1.py": "# File 1_1 contents",
                    "file_2.py": "# File 1_2 contents",
                    "file_3.py": "# File 1_3 contents",
                    "gitignored_dir": {
                        "file_a.py": "# File contents",
                        "file_b.py": "# File contents",
                        "file_c.py": "# File contents",
                    },
                },
                "dir_2": {
                    "file_1.py": "# File 2_1 contents",
                    "file_2.py": "# File 2_2 contents",
                    "file_3.py": "# File 2_3 contents",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    > dir_1",
                "    > dir_2",
                "      .gitignore",
            ]
        );

        let dir_1_file = find_project_entry(&panel, "project_root/dir_1/file_1.py", cx)
            .expect("dir 1 file is not ignored and should have an entry");
        let dir_2_file = find_project_entry(&panel, "project_root/dir_2/file_1.py", cx)
            .expect("dir 2 file is not ignored and should have an entry");
        let gitignored_dir_file =
            find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx);
        assert_eq!(
            gitignored_dir_file, None,
            "File in the gitignored dir should not have an entry before its dir is toggled"
        );

        toggle_expand_dir(&panel, "project_root/dir_1", cx);
        toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        v gitignored_dir  <== selected",
                "              file_a.py",
                "              file_b.py",
                "              file_c.py",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    > dir_2",
                "      .gitignore",
            ],
            "Should show gitignored dir file list in the project panel"
        );
        let gitignored_dir_file =
            find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx)
                .expect("after gitignored dir got opened, a file entry should be present");

        toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
        toggle_expand_dir(&panel, "project_root/dir_1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    > dir_1  <== selected",
                "    > dir_2",
                "      .gitignore",
            ],
            "Should hide all dir contents again and prepare for the explicit reveal test"
        );

        for file_entry in [dir_1_file, dir_2_file, gitignored_dir_file] {
            panel.update(cx, |panel, cx| {
                panel.project.update(cx, |_, cx| {
                    cx.emit(project::Event::ActiveEntryChanged(Some(file_entry)))
                })
            });
            cx.run_until_parked();
            assert_eq!(
                visible_entries_as_strings(&panel, 0..20, cx),
                &[
                    "v project_root",
                    "    > .git",
                    "    > dir_1  <== selected",
                    "    > dir_2",
                    "      .gitignore",
                ],
                "When no auto reveal is enabled, the selected entry should not be revealed in the project panel"
            );
        }

        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::RevealInProjectPanel(dir_1_file))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        > gitignored_dir",
                "          file_1.py  <== selected",
                "          file_2.py",
                "          file_3.py",
                "    > dir_2",
                "      .gitignore",
            ],
            "With no auto reveal, explicit reveal should show the dir_1 entry in the project panel"
        );

        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::RevealInProjectPanel(dir_2_file))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        > gitignored_dir",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    v dir_2",
                "          file_1.py  <== selected",
                "          file_2.py",
                "          file_3.py",
                "      .gitignore",
            ],
            "With no auto reveal, explicit reveal should show the dir_2 entry in the project panel"
        );

        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::RevealInProjectPanel(gitignored_dir_file))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    v dir_1",
                "        v gitignored_dir",
                "              file_a.py  <== selected",
                "              file_b.py",
                "              file_c.py",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "    v dir_2",
                "          file_1.py",
                "          file_2.py",
                "          file_3.py",
                "      .gitignore",
            ],
            "With no auto reveal, explicit reveal should show the gitignored entry in the project panel"
        );
    }

    #[gpui::test]
    async fn test_creating_excluded_entries(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |project_settings| {
                    project_settings.file_scan_exclusions =
                        Some(vec!["excluded_dir".to_string(), "**/.git".to_string()]);
                });
            });
        });

        cx.update(|cx| {
            register_project_item::<TestProjectItemView>(cx);
        });

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace
            .update(cx, |workspace, cx| {
                let panel = ProjectPanel::new(workspace, cx);
                workspace.add_panel(panel.clone(), cx);
                panel
            })
            .unwrap();

        select_path(&panel, "root1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v root1  <== selected", "      .dockerignore",]
        );
        workspace
            .update(cx, |workspace, cx| {
                assert!(
                    workspace.active_item(cx).is_none(),
                    "Should have no active items in the beginning"
                );
            })
            .unwrap();

        let excluded_file_path = ".git/COMMIT_EDITMSG";
        let excluded_dir_path = "excluded_dir";

        panel.update(cx, |panel, cx| panel.new_file(&NewFile, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        panel
            .update(cx, |panel, cx| {
                panel
                    .filename_editor
                    .update(cx, |editor, cx| editor.set_text(excluded_file_path, cx));
                panel.confirm_edit(cx).unwrap()
            })
            .await
            .unwrap();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..13, cx),
            &["v root1", "      .dockerignore"],
            "Excluded dir should not be shown after opening a file in it"
        );
        panel.update(cx, |panel, cx| {
            assert!(
                !panel.filename_editor.read(cx).is_focused(cx),
                "Should have closed the file name editor"
            );
        });
        workspace
            .update(cx, |workspace, cx| {
                let active_entry_path = workspace
                    .active_item(cx)
                    .expect("should have opened and activated the excluded item")
                    .act_as::<TestProjectItemView>(cx)
                    .expect(
                        "should have opened the corresponding project item for the excluded item",
                    )
                    .read(cx)
                    .path
                    .clone();
                assert_eq!(
                    active_entry_path.path.as_ref(),
                    Path::new(excluded_file_path),
                    "Should open the excluded file"
                );

                assert!(
                    workspace.notification_ids().is_empty(),
                    "Should have no notifications after opening an excluded file"
                );
            })
            .unwrap();
        assert!(
            fs.is_file(Path::new("/root1/.git/COMMIT_EDITMSG")).await,
            "Should have created the excluded file"
        );

        select_path(&panel, "root1", cx);
        panel.update(cx, |panel, cx| panel.new_directory(&NewDirectory, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        panel
            .update(cx, |panel, cx| {
                panel
                    .filename_editor
                    .update(cx, |editor, cx| editor.set_text(excluded_file_path, cx));
                panel.confirm_edit(cx).unwrap()
            })
            .await
            .unwrap();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..13, cx),
            &["v root1", "      .dockerignore"],
            "Should not change the project panel after trying to create an excluded directorya directory with the same name as the excluded file"
        );
        panel.update(cx, |panel, cx| {
            assert!(
                !panel.filename_editor.read(cx).is_focused(cx),
                "Should have closed the file name editor"
            );
        });
        workspace
            .update(cx, |workspace, cx| {
                let notifications = workspace.notification_ids();
                assert_eq!(
                    notifications.len(),
                    1,
                    "Should receive one notification with the error message"
                );
                workspace.dismiss_notification(notifications.first().unwrap(), cx);
                assert!(workspace.notification_ids().is_empty());
            })
            .unwrap();

        select_path(&panel, "root1", cx);
        panel.update(cx, |panel, cx| panel.new_directory(&NewDirectory, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        panel
            .update(cx, |panel, cx| {
                panel
                    .filename_editor
                    .update(cx, |editor, cx| editor.set_text(excluded_dir_path, cx));
                panel.confirm_edit(cx).unwrap()
            })
            .await
            .unwrap();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..13, cx),
            &["v root1", "      .dockerignore"],
            "Should not change the project panel after trying to create an excluded directory"
        );
        panel.update(cx, |panel, cx| {
            assert!(
                !panel.filename_editor.read(cx).is_focused(cx),
                "Should have closed the file name editor"
            );
        });
        workspace
            .update(cx, |workspace, cx| {
                let notifications = workspace.notification_ids();
                assert_eq!(
                    notifications.len(),
                    1,
                    "Should receive one notification explaining that no directory is actually shown"
                );
                workspace.dismiss_notification(notifications.first().unwrap(), cx);
                assert!(workspace.notification_ids().is_empty());
            })
            .unwrap();
        assert!(
            fs.is_dir(Path::new("/root1/excluded_dir")).await,
            "Should have created the excluded directory"
        );
    }

    #[gpui::test]
    async fn test_selection_restored_when_creation_cancelled(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace
            .update(cx, |workspace, cx| {
                let panel = ProjectPanel::new(workspace, cx);
                workspace.add_panel(panel.clone(), cx);
                panel
            })
            .unwrap();

        select_path(&panel, "src/", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                //
                "v src  <== selected",
                "    > test"
            ]
        );
        panel.update(cx, |panel, cx| panel.new_directory(&NewDirectory, cx));
        panel.update(cx, |panel, cx| {
            assert!(panel.filename_editor.read(cx).is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                //
                "v src",
                "    > [EDITOR: '']  <== selected",
                "    > test"
            ]
        );

        panel.update(cx, |panel, cx| panel.cancel(&menu::Cancel, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                //
                "v src  <== selected",
                "    > test"
            ]
        );
    }

    #[gpui::test]
    async fn test_basic_file_deletion_scenarios(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root",
            json!({
                "dir1": {
                    "subdir1": {},
                    "file1.txt": "",
                    "file2.txt": "",
                },
                "dir2": {
                    "subdir2": {},
                    "file3.txt": "",
                    "file4.txt": "",
                },
                "file5.txt": "",
                "file6.txt": "",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir2", cx);

        // Test Case 1: Delete middle file in directory
        select_path(&panel, "root/dir1/file1.txt", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir1",
                "        > subdir1",
                "          file1.txt  <== selected",
                "          file2.txt",
                "    v dir2",
                "        > subdir2",
                "          file3.txt",
                "          file4.txt",
                "      file5.txt",
                "      file6.txt",
            ],
            "Initial state before deleting middle file"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir1",
                "        > subdir1",
                "          file2.txt  <== selected",
                "    v dir2",
                "        > subdir2",
                "          file3.txt",
                "          file4.txt",
                "      file5.txt",
                "      file6.txt",
            ],
            "Should select next file after deleting middle file"
        );

        // Test Case 2: Delete last file in directory
        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir1",
                "        > subdir1  <== selected",
                "    v dir2",
                "        > subdir2",
                "          file3.txt",
                "          file4.txt",
                "      file5.txt",
                "      file6.txt",
            ],
            "Should select next directory when last file is deleted"
        );

        // Test Case 3: Delete root level file
        select_path(&panel, "root/file6.txt", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir1",
                "        > subdir1",
                "    v dir2",
                "        > subdir2",
                "          file3.txt",
                "          file4.txt",
                "      file5.txt",
                "      file6.txt  <== selected",
            ],
            "Initial state before deleting root level file"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir1",
                "        > subdir1",
                "    v dir2",
                "        > subdir2",
                "          file3.txt",
                "          file4.txt",
                "      file5.txt  <== selected",
            ],
            "Should select prev entry at root level"
        );
    }

    #[gpui::test]
    async fn test_complex_selection_scenarios(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root",
            json!({
                "dir1": {
                    "subdir1": {
                        "a.txt": "",
                        "b.txt": ""
                    },
                    "file1.txt": "",
                },
                "dir2": {
                    "subdir2": {
                        "c.txt": "",
                        "d.txt": ""
                    },
                    "file2.txt": "",
                },
                "file3.txt": "",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir2", cx);
        toggle_expand_dir(&panel, "root/dir2/subdir2", cx);

        // Test Case 1: Select and delete nested directory with parent
        cx.simulate_modifiers_change(gpui::Modifiers {
            control: true,
            ..Default::default()
        });
        select_path_with_mark(&panel, "root/dir1/subdir1", cx);
        select_path_with_mark(&panel, "root/dir1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir1  <== selected  <== marked",
                "        v subdir1  <== marked",
                "              a.txt",
                "              b.txt",
                "          file1.txt",
                "    v dir2",
                "        v subdir2",
                "              c.txt",
                "              d.txt",
                "          file2.txt",
                "      file3.txt",
            ],
            "Initial state before deleting nested directory with parent"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir2  <== selected",
                "        v subdir2",
                "              c.txt",
                "              d.txt",
                "          file2.txt",
                "      file3.txt",
            ],
            "Should select next directory after deleting directory with parent"
        );

        // Test Case 2: Select mixed files and directories across levels
        select_path_with_mark(&panel, "root/dir2/subdir2/c.txt", cx);
        select_path_with_mark(&panel, "root/dir2/file2.txt", cx);
        select_path_with_mark(&panel, "root/file3.txt", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir2",
                "        v subdir2",
                "              c.txt  <== marked",
                "              d.txt",
                "          file2.txt  <== marked",
                "      file3.txt  <== selected  <== marked",
            ],
            "Initial state before deleting"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..15, cx),
            &[
                "v root",
                "    v dir2  <== selected",
                "        v subdir2",
                "              d.txt",
            ],
            "Should select sibling directory"
        );
    }

    #[gpui::test]
    async fn test_delete_all_files_and_directories(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root",
            json!({
                "dir1": {
                    "subdir1": {
                        "a.txt": "",
                        "b.txt": ""
                    },
                    "file1.txt": "",
                },
                "dir2": {
                    "subdir2": {
                        "c.txt": "",
                        "d.txt": ""
                    },
                    "file2.txt": "",
                },
                "file3.txt": "",
                "file4.txt": "",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir2", cx);
        toggle_expand_dir(&panel, "root/dir2/subdir2", cx);

        // Test Case 1: Select all root files and directories
        cx.simulate_modifiers_change(gpui::Modifiers {
            control: true,
            ..Default::default()
        });
        select_path_with_mark(&panel, "root/dir1", cx);
        select_path_with_mark(&panel, "root/dir2", cx);
        select_path_with_mark(&panel, "root/file3.txt", cx);
        select_path_with_mark(&panel, "root/file4.txt", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root",
                "    v dir1  <== marked",
                "        v subdir1",
                "              a.txt",
                "              b.txt",
                "          file1.txt",
                "    v dir2  <== marked",
                "        v subdir2",
                "              c.txt",
                "              d.txt",
                "          file2.txt",
                "      file3.txt  <== marked",
                "      file4.txt  <== selected  <== marked",
            ],
            "State before deleting all contents"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &["v root  <== selected"],
            "Only empty root directory should remain after deleting all contents"
        );
    }

    #[gpui::test]
    async fn test_nested_selection_deletion(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root",
            json!({
                "dir1": {
                    "subdir1": {
                        "file_a.txt": "content a",
                        "file_b.txt": "content b",
                    },
                    "subdir2": {
                        "file_c.txt": "content c",
                    },
                    "file1.txt": "content 1",
                },
                "dir2": {
                    "file2.txt": "content 2",
                },
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir2", cx);
        cx.simulate_modifiers_change(gpui::Modifiers {
            control: true,
            ..Default::default()
        });

        // Test Case 1: Select parent directory, subdirectory, and a file inside the subdirectory
        select_path_with_mark(&panel, "root/dir1", cx);
        select_path_with_mark(&panel, "root/dir1/subdir1", cx);
        select_path_with_mark(&panel, "root/dir1/subdir1/file_a.txt", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root",
                "    v dir1  <== marked",
                "        v subdir1  <== marked",
                "              file_a.txt  <== selected  <== marked",
                "              file_b.txt",
                "        > subdir2",
                "          file1.txt",
                "    v dir2",
                "          file2.txt",
            ],
            "State with parent dir, subdir, and file selected"
        );
        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &["v root", "    v dir2  <== selected", "          file2.txt",],
            "Only dir2 should remain after deletion"
        );
    }

    #[gpui::test]
    async fn test_multiple_worktrees_deletion(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        // First worktree
        fs.insert_tree(
            "/root1",
            json!({
                "dir1": {
                    "file1.txt": "content 1",
                    "file2.txt": "content 2",
                },
                "dir2": {
                    "file3.txt": "content 3",
                },
            }),
        )
        .await;

        // Second worktree
        fs.insert_tree(
            "/root2",
            json!({
                "dir3": {
                    "file4.txt": "content 4",
                    "file5.txt": "content 5",
                },
                "file6.txt": "content 6",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        // Expand all directories for testing
        toggle_expand_dir(&panel, "root1/dir1", cx);
        toggle_expand_dir(&panel, "root1/dir2", cx);
        toggle_expand_dir(&panel, "root2/dir3", cx);

        // Test Case 1: Delete files across different worktrees
        cx.simulate_modifiers_change(gpui::Modifiers {
            control: true,
            ..Default::default()
        });
        select_path_with_mark(&panel, "root1/dir1/file1.txt", cx);
        select_path_with_mark(&panel, "root2/dir3/file4.txt", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root1",
                "    v dir1",
                "          file1.txt  <== marked",
                "          file2.txt",
                "    v dir2",
                "          file3.txt",
                "v root2",
                "    v dir3",
                "          file4.txt  <== selected  <== marked",
                "          file5.txt",
                "      file6.txt",
            ],
            "Initial state with files selected from different worktrees"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root1",
                "    v dir1",
                "          file2.txt",
                "    v dir2",
                "          file3.txt",
                "v root2",
                "    v dir3",
                "          file5.txt  <== selected",
                "      file6.txt",
            ],
            "Should select next file in the last worktree after deletion"
        );

        // Test Case 2: Delete directories from different worktrees
        select_path_with_mark(&panel, "root1/dir1", cx);
        select_path_with_mark(&panel, "root2/dir3", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root1",
                "    v dir1  <== marked",
                "          file2.txt",
                "    v dir2",
                "          file3.txt",
                "v root2",
                "    v dir3  <== selected  <== marked",
                "          file5.txt",
                "      file6.txt",
            ],
            "State with directories marked from different worktrees"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root1",
                "    v dir2",
                "          file3.txt",
                "v root2",
                "      file6.txt  <== selected",
            ],
            "Should select remaining file in last worktree after directory deletion"
        );

        // Test Case 4: Delete all remaining files except roots
        select_path_with_mark(&panel, "root1/dir2/file3.txt", cx);
        select_path_with_mark(&panel, "root2/file6.txt", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root1",
                "    v dir2",
                "          file3.txt  <== marked",
                "v root2",
                "      file6.txt  <== selected  <== marked",
            ],
            "State with all remaining files marked"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &["v root1", "    v dir2", "v root2  <== selected"],
            "Second parent root should be selected after deleting"
        );
    }

    #[gpui::test]
    async fn test_selection_fallback_to_next_highest_worktree(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree(
            "/root_b",
            json!({
                "dir1": {
                    "file1.txt": "content 1",
                    "file2.txt": "content 2",
                },
            }),
        )
        .await;

        fs.insert_tree(
            "/root_c",
            json!({
                "dir2": {},
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root_b".as_ref(), "/root_c".as_ref()], cx).await;
        let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();

        toggle_expand_dir(&panel, "root_b/dir1", cx);
        toggle_expand_dir(&panel, "root_c/dir2", cx);

        cx.simulate_modifiers_change(gpui::Modifiers {
            control: true,
            ..Default::default()
        });
        select_path_with_mark(&panel, "root_b/dir1/file1.txt", cx);
        select_path_with_mark(&panel, "root_b/dir1/file2.txt", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root_b",
                "    v dir1",
                "          file1.txt  <== marked",
                "          file2.txt  <== selected  <== marked",
                "v root_c",
                "    v dir2",
            ],
            "Initial state with files marked in root_b"
        );

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root_b",
                "    v dir1  <== selected",
                "v root_c",
                "    v dir2",
            ],
            "After deletion in root_b as it's last deletion, selection should be in root_b"
        );

        select_path_with_mark(&panel, "root_c/dir2", cx);

        submit_deletion(&panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &["v root_b", "    v dir1", "v root_c  <== selected",],
            "After deleting from root_c, it should remain in root_c"
        );
    }

    fn toggle_expand_dir(
        panel: &View<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut VisualTestContext,
    ) {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                    panel.toggle_expanded(entry_id, cx);
                    return;
                }
            }
            panic!("no worktree for path {:?}", path);
        });
    }

    fn select_path(panel: &View<ProjectPanel>, path: impl AsRef<Path>, cx: &mut VisualTestContext) {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                    panel.selection = Some(crate::SelectedEntry {
                        worktree_id: worktree.id(),
                        entry_id,
                    });
                    return;
                }
            }
            panic!("no worktree for path {:?}", path);
        });
    }

    fn select_path_with_mark(
        panel: &View<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut VisualTestContext,
    ) {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                    let entry = crate::SelectedEntry {
                        worktree_id: worktree.id(),
                        entry_id,
                    };
                    if !panel.marked_entries.contains(&entry) {
                        panel.marked_entries.insert(entry);
                    }
                    panel.selection = Some(entry);
                    return;
                }
            }
            panic!("no worktree for path {:?}", path);
        });
    }

    fn find_project_entry(
        panel: &View<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut VisualTestContext,
    ) -> Option<ProjectEntryId> {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    return worktree.entry_for_path(relative_path).map(|entry| entry.id);
                }
            }
            panic!("no worktree for path {path:?}");
        })
    }

    fn visible_entries_as_strings(
        panel: &View<ProjectPanel>,
        range: Range<usize>,
        cx: &mut VisualTestContext,
    ) -> Vec<String> {
        let mut result = Vec::new();
        let mut project_entries = HashSet::default();
        let mut has_editor = false;

        panel.update(cx, |panel, cx| {
            panel.for_each_visible_entry(range, cx, |project_entry, details, _| {
                if details.is_editing {
                    assert!(!has_editor, "duplicate editor entry");
                    has_editor = true;
                } else {
                    assert!(
                        project_entries.insert(project_entry),
                        "duplicate project entry {:?} {:?}",
                        project_entry,
                        details
                    );
                }

                let indent = "    ".repeat(details.depth);
                let icon = if details.kind.is_dir() {
                    if details.is_expanded {
                        "v "
                    } else {
                        "> "
                    }
                } else {
                    "  "
                };
                let name = if details.is_editing {
                    format!("[EDITOR: '{}']", details.filename)
                } else if details.is_processing {
                    format!("[PROCESSING: '{}']", details.filename)
                } else {
                    details.filename.clone()
                };
                let selected = if details.is_selected {
                    "  <== selected"
                } else {
                    ""
                };
                let marked = if details.is_marked {
                    "  <== marked"
                } else {
                    ""
                };

                result.push(format!("{indent}{icon}{name}{selected}{marked}"));
            });
        });

        result
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            init_settings(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            editor::init_settings(cx);
            crate::init((), cx);
            workspace::init_settings(cx);
            client::init_settings(cx);
            Project::init_settings(cx);

            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                    project_panel_settings.auto_fold_dirs = Some(false);
                });
                store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                    worktree_settings.file_scan_exclusions = Some(Vec::new());
                });
            });
        });
    }

    fn init_test_with_editor(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            init_settings(cx);
            language::init(cx);
            editor::init(cx);
            crate::init((), cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);

            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                    project_panel_settings.auto_fold_dirs = Some(false);
                });
                store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                    worktree_settings.file_scan_exclusions = Some(Vec::new());
                });
            });
        });
    }

    fn ensure_single_file_is_opened(
        window: &WindowHandle<Workspace>,
        expected_path: &str,
        cx: &mut TestAppContext,
    ) {
        window
            .update(cx, |workspace, cx| {
                let worktrees = workspace.worktrees(cx).collect::<Vec<_>>();
                assert_eq!(worktrees.len(), 1);
                let worktree_id = worktrees[0].read(cx).id();

                let open_project_paths = workspace
                    .panes()
                    .iter()
                    .filter_map(|pane| pane.read(cx).active_item()?.project_path(cx))
                    .collect::<Vec<_>>();
                assert_eq!(
                    open_project_paths,
                    vec![ProjectPath {
                        worktree_id,
                        path: Arc::from(Path::new(expected_path))
                    }],
                    "Should have opened file, selected in project panel"
                );
            })
            .unwrap();
    }

    fn submit_deletion(panel: &View<ProjectPanel>, cx: &mut VisualTestContext) {
        assert!(
            !cx.has_pending_prompt(),
            "Should have no prompts before the deletion"
        );
        panel.update(cx, |panel, cx| {
            panel.delete(&Delete { skip_prompt: false }, cx)
        });
        assert!(
            cx.has_pending_prompt(),
            "Should have a prompt after the deletion"
        );
        cx.simulate_prompt_answer(0);
        assert!(
            !cx.has_pending_prompt(),
            "Should have no prompts after prompt was replied to"
        );
        cx.executor().run_until_parked();
    }

    fn submit_deletion_skipping_prompt(panel: &View<ProjectPanel>, cx: &mut VisualTestContext) {
        assert!(
            !cx.has_pending_prompt(),
            "Should have no prompts before the deletion"
        );
        panel.update(cx, |panel, cx| {
            panel.delete(&Delete { skip_prompt: true }, cx)
        });
        assert!(!cx.has_pending_prompt(), "Should have received no prompts");
        cx.executor().run_until_parked();
    }

    fn ensure_no_open_items_and_panes(
        workspace: &WindowHandle<Workspace>,
        cx: &mut VisualTestContext,
    ) {
        assert!(
            !cx.has_pending_prompt(),
            "Should have no prompts after deletion operation closes the file"
        );
        workspace
            .read_with(cx, |workspace, cx| {
                let open_project_paths = workspace
                    .panes()
                    .iter()
                    .filter_map(|pane| pane.read(cx).active_item()?.project_path(cx))
                    .collect::<Vec<_>>();
                assert!(
                    open_project_paths.is_empty(),
                    "Deleted file's buffer should be closed, but got open files: {open_project_paths:?}"
                );
            })
            .unwrap();
    }

    struct TestProjectItemView {
        focus_handle: FocusHandle,
        path: ProjectPath,
    }

    struct TestProjectItem {
        path: ProjectPath,
    }

    impl project::Item for TestProjectItem {
        fn try_open(
            _project: &Model<Project>,
            path: &ProjectPath,
            cx: &mut AppContext,
        ) -> Option<Task<gpui::Result<Model<Self>>>> {
            let path = path.clone();
            Some(cx.spawn(|mut cx| async move { cx.new_model(|_| Self { path }) }))
        }

        fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
            None
        }

        fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
            Some(self.path.clone())
        }
    }

    impl ProjectItem for TestProjectItemView {
        type Item = TestProjectItem;

        fn for_project_item(
            _: Model<Project>,
            project_item: Model<Self::Item>,
            cx: &mut ViewContext<Self>,
        ) -> Self
        where
            Self: Sized,
        {
            Self {
                path: project_item.update(cx, |project_item, _| project_item.path.clone()),
                focus_handle: cx.focus_handle(),
            }
        }
    }

    impl Item for TestProjectItemView {
        type Event = ();
    }

    impl EventEmitter<()> for TestProjectItemView {}

    impl FocusableView for TestProjectItemView {
        fn focus_handle(&self, _: &AppContext) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl Render for TestProjectItemView {
        fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
            Empty
        }
    }
}
