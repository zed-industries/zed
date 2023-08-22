pub mod file_associations;
mod project_panel_settings;

use context_menu::{ContextMenu, ContextMenuItem};
use db::kvp::KEY_VALUE_STORE;
use drag_and_drop::{DragAndDrop, Draggable};
use editor::{scroll::autoscroll::Autoscroll, Cancel, Editor};
use file_associations::FileAssociations;

use futures::stream::StreamExt;
use gpui::{
    actions,
    anyhow::{self, anyhow, Result},
    elements::{
        AnchorCorner, ChildView, ContainerStyle, Empty, Flex, Label, MouseEventHandler,
        ParentElement, ScrollTarget, Stack, Svg, UniformList, UniformListState,
    },
    geometry::vector::Vector2F,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, MouseButton, PromptLevel},
    Action, AnyElement, AppContext, AssetSource, AsyncAppContext, ClipboardItem, Element, Entity,
    ModelHandle, Task, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use menu::{Confirm, SelectNext, SelectPrev};
use project::{
    repository::GitFileStatus, Entry, EntryKind, Fs, Project, ProjectEntryId, ProjectPath,
    Worktree, WorktreeId,
};
use project_panel_settings::{ProjectPanelDockPosition, ProjectPanelSettings};
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap},
    ffi::OsStr,
    ops::Range,
    path::Path,
    sync::Arc,
};
use theme::ProjectPanelEntry;
use unicase::UniCase;
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    Workspace,
};

const PROJECT_PANEL_KEY: &'static str = "ProjectPanel";
const NEW_ENTRY_ID: ProjectEntryId = ProjectEntryId::MAX;

pub struct ProjectPanel {
    project: ModelHandle<Project>,
    fs: Arc<dyn Fs>,
    list: UniformListState,
    visible_entries: Vec<(WorktreeId, Vec<Entry>)>,
    last_worktree_root_id: Option<ProjectEntryId>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
    selection: Option<Selection>,
    edit_state: Option<EditState>,
    filename_editor: ViewHandle<Editor>,
    clipboard_entry: Option<ClipboardEntry>,
    context_menu: ViewHandle<ContextMenu>,
    dragged_entry_destination: Option<Arc<Path>>,
    workspace: WeakViewHandle<Workspace>,
    has_focus: bool,
    width: Option<f32>,
    pending_serialization: Task<Option<()>>,
}

#[derive(Copy, Clone, Debug)]
struct Selection {
    worktree_id: WorktreeId,
    entry_id: ProjectEntryId,
}

#[derive(Clone, Debug)]
struct EditState {
    worktree_id: WorktreeId,
    entry_id: ProjectEntryId,
    is_new_entry: bool,
    is_dir: bool,
    processing_filename: Option<String>,
}

#[derive(Copy, Clone)]
pub enum ClipboardEntry {
    Copied {
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
    },
    Cut {
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntryDetails {
    filename: String,
    icon: Option<Arc<str>>,
    path: Arc<Path>,
    depth: usize,
    kind: EntryKind,
    is_ignored: bool,
    is_expanded: bool,
    is_selected: bool,
    is_editing: bool,
    is_processing: bool,
    is_cut: bool,
    git_status: Option<GitFileStatus>,
}

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
        RevealInFinder,
        Cut,
        Paste,
        Delete,
        Rename,
        Open,
        ToggleFocus,
        NewSearchInDirectory,
    ]
);

pub fn init_settings(cx: &mut AppContext) {
    settings::register::<ProjectPanelSettings>(cx);
}

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    init_settings(cx);
    file_associations::init(assets, cx);
    cx.add_action(ProjectPanel::expand_selected_entry);
    cx.add_action(ProjectPanel::collapse_selected_entry);
    cx.add_action(ProjectPanel::collapse_all_entries);
    cx.add_action(ProjectPanel::select_prev);
    cx.add_action(ProjectPanel::select_next);
    cx.add_action(ProjectPanel::new_file);
    cx.add_action(ProjectPanel::new_directory);
    cx.add_action(ProjectPanel::rename);
    cx.add_async_action(ProjectPanel::delete);
    cx.add_async_action(ProjectPanel::confirm);
    cx.add_async_action(ProjectPanel::open_file);
    cx.add_action(ProjectPanel::cancel);
    cx.add_action(ProjectPanel::cut);
    cx.add_action(ProjectPanel::copy);
    cx.add_action(ProjectPanel::copy_path);
    cx.add_action(ProjectPanel::copy_relative_path);
    cx.add_action(ProjectPanel::reveal_in_finder);
    cx.add_action(ProjectPanel::new_search_in_directory);
    cx.add_action(
        |this: &mut ProjectPanel, action: &Paste, cx: &mut ViewContext<ProjectPanel>| {
            this.paste(action, cx);
        },
    );
}

#[derive(Debug)]
pub enum Event {
    OpenedEntry {
        entry_id: ProjectEntryId,
        focus_opened_item: bool,
    },
    SplitEntry {
        entry_id: ProjectEntryId,
    },
    DockPositionChanged,
    Focus,
    NewSearchInDirectory {
        dir_entry: Entry,
    },
    ActivatePanel,
}

#[derive(Serialize, Deserialize)]
struct SerializedProjectPanel {
    width: Option<f32>,
}

impl ProjectPanel {
    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
        let project = workspace.project().clone();
        let project_panel = cx.add_view(|cx: &mut ViewContext<Self>| {
            cx.observe(&project, |this, _, cx| {
                this.update_visible_entries(None, cx);
                cx.notify();
            })
            .detach();
            cx.subscribe(&project, |this, project, event, cx| match event {
                project::Event::ActiveEntryChanged(Some(entry_id)) => {
                    if let Some(worktree_id) = project.read(cx).worktree_id_for_entry(*entry_id, cx)
                    {
                        this.expand_entry(worktree_id, *entry_id, cx);
                        this.update_visible_entries(Some((worktree_id, *entry_id)), cx);
                        this.autoscroll(cx);
                        cx.notify();
                    }
                }
                project::Event::ActivateProjectPanel => {
                    cx.emit(Event::ActivatePanel);
                }
                project::Event::WorktreeRemoved(id) => {
                    this.expanded_dir_ids.remove(id);
                    this.update_visible_entries(None, cx);
                    cx.notify();
                }
                _ => {}
            })
            .detach();

            let filename_editor = cx.add_view(|cx| {
                Editor::single_line(
                    Some(Arc::new(|theme| {
                        let mut style = theme.project_panel.filename_editor.clone();
                        style.container.background_color.take();
                        style
                    })),
                    cx,
                )
            });

            cx.subscribe(&filename_editor, |this, _, event, cx| match event {
                editor::Event::BufferEdited | editor::Event::SelectionsChanged { .. } => {
                    this.autoscroll(cx);
                }
                _ => {}
            })
            .detach();
            cx.observe_focus(&filename_editor, |this, _, is_focused, cx| {
                if !is_focused
                    && this
                        .edit_state
                        .as_ref()
                        .map_or(false, |state| state.processing_filename.is_none())
                {
                    this.edit_state = None;
                    this.update_visible_entries(None, cx);
                }
            })
            .detach();

            cx.observe_global::<FileAssociations, _>(|_, cx| {
                cx.notify();
            })
            .detach();

            let view_id = cx.view_id();
            let mut this = Self {
                project: project.clone(),
                fs: workspace.app_state().fs.clone(),
                list: Default::default(),
                visible_entries: Default::default(),
                last_worktree_root_id: Default::default(),
                expanded_dir_ids: Default::default(),
                selection: None,
                edit_state: None,
                filename_editor,
                clipboard_entry: None,
                context_menu: cx.add_view(|cx| ContextMenu::new(view_id, cx)),
                dragged_entry_destination: None,
                workspace: workspace.weak_handle(),
                has_focus: false,
                width: None,
                pending_serialization: Task::ready(None),
            };
            this.update_visible_entries(None, cx);

            // Update the dock position when the setting changes.
            let mut old_dock_position = this.position(cx);
            cx.observe_global::<SettingsStore, _>(move |this, cx| {
                let new_dock_position = this.position(cx);
                if new_dock_position != old_dock_position {
                    old_dock_position = new_dock_position;
                    cx.emit(Event::DockPositionChanged);
                }
            })
            .detach();

            this
        });

        cx.subscribe(&project_panel, {
            let project_panel = project_panel.downgrade();
            move |workspace, _, event, cx| match event {
                &Event::OpenedEntry {
                    entry_id,
                    focus_opened_item,
                } => {
                    if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx) {
                        if let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                            workspace
                                .open_path(
                                    ProjectPath {
                                        worktree_id: worktree.read(cx).id(),
                                        path: entry.path.clone(),
                                    },
                                    None,
                                    focus_opened_item,
                                    cx,
                                )
                                .detach_and_log_err(cx);
                            if !focus_opened_item {
                                if let Some(project_panel) = project_panel.upgrade(cx) {
                                    cx.focus(&project_panel);
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

    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background()
                .spawn(async move { KEY_VALUE_STORE.read_kvp(PROJECT_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedProjectPanel>(&panel)?)
            } else {
                None
            };
            workspace.update(&mut cx, |workspace, cx| {
                let panel = ProjectPanel::new(workspace, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width;
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background().spawn(
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

    fn deploy_context_menu(
        &mut self,
        position: Vector2F,
        entry_id: ProjectEntryId,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.project.read(cx);

        let worktree_id = if let Some(id) = project.worktree_id_for_entry(entry_id, cx) {
            id
        } else {
            return;
        };

        self.selection = Some(Selection {
            worktree_id,
            entry_id,
        });

        let mut menu_entries = Vec::new();
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let is_root = Some(entry) == worktree.root_entry();
            if !project.is_remote() {
                menu_entries.push(ContextMenuItem::action(
                    "Add Folder to Project",
                    workspace::AddFolderToProject,
                ));
                if is_root {
                    let project = self.project.clone();
                    menu_entries.push(ContextMenuItem::handler("Remove from Project", move |cx| {
                        project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
                    }));
                }
            }
            menu_entries.push(ContextMenuItem::action("New File", NewFile));
            menu_entries.push(ContextMenuItem::action("New Folder", NewDirectory));
            menu_entries.push(ContextMenuItem::Separator);
            menu_entries.push(ContextMenuItem::action("Cut", Cut));
            menu_entries.push(ContextMenuItem::action("Copy", Copy));
            menu_entries.push(ContextMenuItem::Separator);
            menu_entries.push(ContextMenuItem::action("Copy Path", CopyPath));
            menu_entries.push(ContextMenuItem::action(
                "Copy Relative Path",
                CopyRelativePath,
            ));
            menu_entries.push(ContextMenuItem::action("Reveal in Finder", RevealInFinder));
            if entry.is_dir() {
                menu_entries.push(ContextMenuItem::action(
                    "Search Inside",
                    NewSearchInDirectory,
                ));
            }
            if let Some(clipboard_entry) = self.clipboard_entry {
                if clipboard_entry.worktree_id() == worktree.id() {
                    menu_entries.push(ContextMenuItem::action("Paste", Paste));
                }
            }
            menu_entries.push(ContextMenuItem::Separator);
            menu_entries.push(ContextMenuItem::action("Rename", Rename));
            if !is_root {
                menu_entries.push(ContextMenuItem::action("Delete", Delete));
            }
        }

        self.context_menu.update(cx, |menu, cx| {
            menu.show(position, AnchorCorner::TopLeft, menu_entries, cx);
        });

        cx.notify();
    }

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
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
        if let Some((worktree, mut entry)) = self.selected_entry(cx) {
            let worktree_id = worktree.id();
            let expanded_dir_ids =
                if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                    expanded_dir_ids
                } else {
                    return;
                };

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
    }

    pub fn collapse_all_entries(&mut self, _: &CollapseAllEntries, cx: &mut ViewContext<Self>) {
        self.expanded_dir_ids.clear();
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
                cx.focus_self();
                cx.notify();
            }
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
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

            let (worktree_id, worktree_entries) = &self.visible_entries[worktree_ix];
            self.selection = Some(Selection {
                worktree_id: *worktree_id,
                entry_id: worktree_entries[entry_ix].id,
            });
            self.autoscroll(cx);
            cx.notify();
        } else {
            self.select_first(cx);
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        if let Some(task) = self.confirm_edit(cx) {
            return Some(task);
        }

        None
    }

    fn open_file(&mut self, _: &Open, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        if let Some((_, entry)) = self.selected_entry(cx) {
            if entry.is_file() {
                self.open_entry(entry.id, true, cx);
            }
        }

        None
    }

    fn confirm_edit(&mut self, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let edit_state = self.edit_state.as_mut()?;
        cx.focus_self();

        let worktree_id = edit_state.worktree_id;
        let is_new_entry = edit_state.is_new_entry;
        let is_dir = edit_state.is_dir;
        let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(edit_state.entry_id)?.clone();
        let filename = self.filename_editor.read(cx).text(cx);

        let path_already_exists = |path| worktree.read(cx).entry_for_path(path).is_some();
        let edit_task;
        let edited_entry_id;
        if is_new_entry {
            self.selection = Some(Selection {
                worktree_id,
                entry_id: NEW_ENTRY_ID,
            });
            let new_path = entry.path.join(&filename.trim_start_matches("/"));
            if path_already_exists(new_path.as_path()) {
                return None;
            }

            edited_entry_id = NEW_ENTRY_ID;
            edit_task = self.project.update(cx, |project, cx| {
                project.create_entry((worktree_id, &new_path), is_dir, cx)
            })?;
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
            })?;
        };

        edit_state.processing_filename = Some(filename);
        cx.notify();

        Some(cx.spawn(|this, mut cx| async move {
            let new_entry = edit_task.await;
            this.update(&mut cx, |this, cx| {
                this.edit_state.take();
                cx.notify();
            })?;

            let new_entry = new_entry?;
            this.update(&mut cx, |this, cx| {
                if let Some(selection) = &mut this.selection {
                    if selection.entry_id == edited_entry_id {
                        selection.worktree_id = worktree_id;
                        selection.entry_id = new_entry.id;
                        this.expand_to_selection(cx);
                    }
                }
                this.update_visible_entries(None, cx);
                if is_new_entry && !is_dir {
                    this.open_entry(new_entry.id, true, cx);
                }
                cx.notify();
            })?;
            Ok(())
        }))
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        self.edit_state = None;
        self.update_visible_entries(None, cx);
        cx.focus_self();
        cx.notify();
    }

    fn open_entry(
        &mut self,
        entry_id: ProjectEntryId,
        focus_opened_item: bool,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(Event::OpenedEntry {
            entry_id,
            focus_opened_item,
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
        if let Some(Selection {
            worktree_id,
            entry_id,
        }) = self.selection
        {
            let directory_id;
            if let Some((worktree, expanded_dir_ids)) = self
                .project
                .read(cx)
                .worktree_for_id(worktree_id, cx)
                .zip(self.expanded_dir_ids.get_mut(&worktree_id))
            {
                let worktree = worktree.read(cx);
                if let Some(mut entry) = worktree.entry_for_id(entry_id) {
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

            self.edit_state = Some(EditState {
                worktree_id,
                entry_id: directory_id,
                is_new_entry: true,
                is_dir,
                processing_filename: None,
            });
            self.filename_editor
                .update(cx, |editor, cx| editor.clear(cx));
            cx.focus(&self.filename_editor);
            self.update_visible_entries(Some((worktree_id, NEW_ENTRY_ID)), cx);
            self.autoscroll(cx);
            cx.notify();
        }
    }

    fn rename(&mut self, _: &Rename, cx: &mut ViewContext<Self>) {
        if let Some(Selection {
            worktree_id,
            entry_id,
        }) = self.selection
        {
            if let Some(worktree) = self.project.read(cx).worktree_for_id(worktree_id, cx) {
                if let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                    self.edit_state = Some(EditState {
                        worktree_id,
                        entry_id,
                        is_new_entry: false,
                        is_dir: entry.is_dir(),
                        processing_filename: None,
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
                        })
                    });
                    cx.focus(&self.filename_editor);
                    self.update_visible_entries(None, cx);
                    self.autoscroll(cx);
                    cx.notify();
                }
            }

            cx.update_global(|drag_and_drop: &mut DragAndDrop<Workspace>, cx| {
                drag_and_drop.cancel_dragging::<ProjectEntryId>(cx);
            })
        }
    }

    fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let Selection { entry_id, .. } = self.selection?;
        let path = self.project.read(cx).path_for_entry(entry_id, cx)?.path;
        let file_name = path.file_name()?;

        let mut answer = cx.prompt(
            PromptLevel::Info,
            &format!("Delete {file_name:?}?"),
            &["Delete", "Cancel"],
        );
        Some(cx.spawn(|this, mut cx| async move {
            if answer.next().await != Some(0) {
                return Ok(());
            }
            this.update(&mut cx, |this, cx| {
                this.project
                    .update(cx, |project, cx| project.delete_entry(entry_id, cx))
                    .ok_or_else(|| anyhow!("no such entry"))
            })??
            .await
        }))
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if let Some((_, worktree_entries)) = self.visible_entries.get(worktree_ix) {
                if entry_ix + 1 < worktree_entries.len() {
                    entry_ix += 1;
                } else {
                    worktree_ix += 1;
                    entry_ix = 0;
                }
            }

            if let Some((worktree_id, worktree_entries)) = self.visible_entries.get(worktree_ix) {
                if let Some(entry) = worktree_entries.get(entry_ix) {
                    self.selection = Some(Selection {
                        worktree_id: *worktree_id,
                        entry_id: entry.id,
                    });
                    self.autoscroll(cx);
                    cx.notify();
                }
            }
        } else {
            self.select_first(cx);
        }
    }

    fn select_first(&mut self, cx: &mut ViewContext<Self>) {
        let worktree = self
            .visible_entries
            .first()
            .and_then(|(worktree_id, _)| self.project.read(cx).worktree_for_id(*worktree_id, cx));
        if let Some(worktree) = worktree {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            if let Some(root_entry) = worktree.root_entry() {
                self.selection = Some(Selection {
                    worktree_id,
                    entry_id: root_entry.id,
                });
                self.autoscroll(cx);
                cx.notify();
            }
        }
    }

    fn autoscroll(&mut self, cx: &mut ViewContext<Self>) {
        if let Some((_, _, index)) = self.selection.and_then(|s| self.index_for_selection(s)) {
            self.list.scroll_to(ScrollTarget::Show(index));
            cx.notify();
        }
    }

    fn cut(&mut self, _: &Cut, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.clipboard_entry = Some(ClipboardEntry::Cut {
                worktree_id: worktree.id(),
                entry_id: entry.id,
            });
            cx.notify();
        }
    }

    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            self.clipboard_entry = Some(ClipboardEntry::Copied {
                worktree_id: worktree.id(),
                entry_id: entry.id,
            });
            cx.notify();
        }
    }

    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) -> Option<()> {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let clipboard_entry = self.clipboard_entry?;
            if clipboard_entry.worktree_id() != worktree.id() {
                return None;
            }

            let clipboard_entry_file_name = self
                .project
                .read(cx)
                .path_for_entry(clipboard_entry.entry_id(), cx)?
                .path
                .file_name()?
                .to_os_string();

            let mut new_path = entry.path.to_path_buf();
            if entry.is_file() {
                new_path.pop();
            }

            new_path.push(&clipboard_entry_file_name);
            let extension = new_path.extension().map(|e| e.to_os_string());
            let file_name_without_extension = Path::new(&clipboard_entry_file_name).file_stem()?;
            let mut ix = 0;
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

            if clipboard_entry.is_cut() {
                if let Some(task) = self.project.update(cx, |project, cx| {
                    project.rename_entry(clipboard_entry.entry_id(), new_path, cx)
                }) {
                    task.detach_and_log_err(cx)
                }
            } else if let Some(task) = self.project.update(cx, |project, cx| {
                project.copy_entry(clipboard_entry.entry_id(), new_path, cx)
            }) {
                task.detach_and_log_err(cx)
            }
        }
        None
    }

    fn copy_path(&mut self, _: &CopyPath, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            cx.write_to_clipboard(ClipboardItem::new(
                worktree
                    .abs_path()
                    .join(&entry.path)
                    .to_string_lossy()
                    .to_string(),
            ));
        }
    }

    fn copy_relative_path(&mut self, _: &CopyRelativePath, cx: &mut ViewContext<Self>) {
        if let Some((_, entry)) = self.selected_entry(cx) {
            cx.write_to_clipboard(ClipboardItem::new(entry.path.to_string_lossy().to_string()));
        }
    }

    fn reveal_in_finder(&mut self, _: &RevealInFinder, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            cx.reveal_path(&worktree.abs_path().join(&entry.path));
        }
    }

    pub fn new_search_in_directory(
        &mut self,
        _: &NewSearchInDirectory,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some((_, entry)) = self.selected_entry(cx) {
            if entry.is_dir() {
                cx.emit(Event::NewSearchInDirectory {
                    dir_entry: entry.clone(),
                });
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
                let task = project.rename_entry(entry_to_move, new_path, cx)?;
                cx.foreground().spawn(task).detach_and_log_err(cx);
            }

            Some(project.worktree_id_for_entry(destination, cx)?)
        });

        if let Some(destination_worktree) = destination_worktree {
            self.expand_entry(destination_worktree, destination, cx);
        }
    }

    fn index_for_selection(&self, selection: Selection) -> Option<(usize, usize, usize)> {
        let mut entry_index = 0;
        let mut visible_entries_index = 0;
        for (worktree_index, (worktree_id, worktree_entries)) in
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

    pub fn selected_entry<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<(&'a Worktree, &'a project::Entry)> {
        let (worktree, entry) = self.selected_entry_handle(cx)?;
        Some((worktree.read(cx), entry))
    }

    fn selected_entry_handle<'a>(
        &self,
        cx: &'a AppContext,
    ) -> Option<(ModelHandle<Worktree>, &'a project::Entry)> {
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
        let project = self.project.read(cx);
        self.last_worktree_root_id = project
            .visible_worktrees(cx)
            .rev()
            .next()
            .and_then(|worktree| worktree.read(cx).root_entry())
            .map(|entry| entry.id);

        self.visible_entries.clear();
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
                if edit_state.worktree_id == worktree_id && edit_state.is_new_entry {
                    new_entry_parent_id = Some(edit_state.entry_id);
                    new_entry_kind = if edit_state.is_dir {
                        EntryKind::Dir
                    } else {
                        EntryKind::File(Default::default())
                    };
                }
            }

            let mut visible_worktree_entries = Vec::new();
            let mut entry_iter = snapshot.entries(true);

            while let Some(entry) = entry_iter.entry() {
                visible_worktree_entries.push(entry.clone());
                if Some(entry.id) == new_entry_parent_id {
                    visible_worktree_entries.push(Entry {
                        id: NEW_ENTRY_ID,
                        kind: new_entry_kind,
                        path: entry.path.join("\0").into(),
                        inode: 0,
                        mtime: entry.mtime,
                        is_symlink: false,
                        is_ignored: false,
                        is_external: false,
                        git_status: entry.git_status,
                    });
                }
                if expanded_dir_ids.binary_search(&entry.id).is_err()
                    && entry_iter.advance_to_sibling()
                {
                    continue;
                }
                entry_iter.advance();
            }

            snapshot.propagate_git_statuses(&mut visible_worktree_entries);

            visible_worktree_entries.sort_by(|entry_a, entry_b| {
                let mut components_a = entry_a.path.components().peekable();
                let mut components_b = entry_b.path.components().peekable();
                loop {
                    match (components_a.next(), components_b.next()) {
                        (Some(component_a), Some(component_b)) => {
                            let a_is_file = components_a.peek().is_none() && entry_a.is_file();
                            let b_is_file = components_b.peek().is_none() && entry_b.is_file();
                            let ordering = a_is_file.cmp(&b_is_file).then_with(|| {
                                let name_a =
                                    UniCase::new(component_a.as_os_str().to_string_lossy());
                                let name_b =
                                    UniCase::new(component_b.as_os_str().to_string_lossy());
                                name_a.cmp(&name_b)
                            });
                            if !ordering.is_eq() {
                                return ordering;
                            }
                        }
                        (Some(_), None) => break Ordering::Greater,
                        (None, Some(_)) => break Ordering::Less,
                        (None, None) => break Ordering::Equal,
                    }
                }
            });
            self.visible_entries
                .push((worktree_id, visible_worktree_entries));
        }

        if let Some((worktree_id, entry_id)) = new_selected_entry {
            self.selection = Some(Selection {
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

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<ProjectPanel>,
        mut callback: impl FnMut(ProjectEntryId, EntryDetails, &mut ViewContext<ProjectPanel>),
    ) {
        let mut ix = 0;
        for (worktree_id, visible_worktree_entries) in &self.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible_worktree_entries.len() <= range.start {
                ix += visible_worktree_entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible_worktree_entries.len());
            let (git_status_setting, show_file_icons, show_folder_icons) = {
                let settings = settings::get::<ProjectPanelSettings>(cx);
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
                for entry in visible_worktree_entries[entry_range].iter() {
                    let status = git_status_setting.then(|| entry.git_status).flatten();
                    let is_expanded = expanded_entry_ids.binary_search(&entry.id).is_ok();
                    let icon = match entry.kind {
                        EntryKind::File(_) => {
                            if show_file_icons {
                                Some(FileAssociations::get_icon(&entry.path, cx))
                            } else {
                                None
                            }
                        }
                        _ => {
                            if show_folder_icons {
                                Some(FileAssociations::get_folder_icon(is_expanded, cx))
                            } else {
                                Some(FileAssociations::get_chevron_icon(is_expanded, cx))
                            }
                        }
                    };

                    let mut details = EntryDetails {
                        filename: entry
                            .path
                            .file_name()
                            .unwrap_or(root_name)
                            .to_string_lossy()
                            .to_string(),
                        icon,
                        path: entry.path.clone(),
                        depth: entry.path.components().count(),
                        kind: entry.kind,
                        is_ignored: entry.is_ignored,
                        is_expanded,
                        is_selected: self.selection.map_or(false, |e| {
                            e.worktree_id == snapshot.id() && e.entry_id == entry.id
                        }),
                        is_editing: false,
                        is_processing: false,
                        is_cut: self
                            .clipboard_entry
                            .map_or(false, |e| e.is_cut() && e.entry_id() == entry.id),
                        git_status: status,
                    };

                    if let Some(edit_state) = &self.edit_state {
                        let is_edited_entry = if edit_state.is_new_entry {
                            entry.id == NEW_ENTRY_ID
                        } else {
                            entry.id == edit_state.entry_id
                        };

                        if is_edited_entry {
                            if let Some(processing_filename) = &edit_state.processing_filename {
                                details.is_processing = true;
                                details.filename.clear();
                                details.filename.push_str(processing_filename);
                            } else {
                                if edit_state.is_new_entry {
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

    fn render_entry_visual_element<V: View>(
        details: &EntryDetails,
        editor: Option<&ViewHandle<Editor>>,
        padding: f32,
        row_container_style: ContainerStyle,
        style: &ProjectPanelEntry,
        cx: &mut ViewContext<V>,
    ) -> AnyElement<V> {
        let show_editor = details.is_editing && !details.is_processing;

        let mut filename_text_style = style.text.clone();
        filename_text_style.color = details
            .git_status
            .as_ref()
            .map(|status| match status {
                GitFileStatus::Added => style.status.git.inserted,
                GitFileStatus::Modified => style.status.git.modified,
                GitFileStatus::Conflict => style.status.git.conflict,
            })
            .unwrap_or(style.text.color);

        Flex::row()
            .with_child(if let Some(icon) = &details.icon {
                Svg::new(icon.to_string())
                    .with_color(style.icon_color)
                    .constrained()
                    .with_max_width(style.icon_size)
                    .with_max_height(style.icon_size)
                    .aligned()
                    .constrained()
                    .with_width(style.icon_size)
            } else {
                Empty::new()
                    .constrained()
                    .with_max_width(style.icon_size)
                    .with_max_height(style.icon_size)
                    .aligned()
                    .constrained()
                    .with_width(style.icon_size)
            })
            .with_child(if show_editor && editor.is_some() {
                ChildView::new(editor.as_ref().unwrap(), cx)
                    .contained()
                    .with_margin_left(style.icon_spacing)
                    .aligned()
                    .left()
                    .flex(1.0, true)
                    .into_any()
            } else {
                Label::new(details.filename.clone(), filename_text_style)
                    .contained()
                    .with_margin_left(style.icon_spacing)
                    .aligned()
                    .left()
                    .into_any()
            })
            .constrained()
            .with_height(style.height)
            .contained()
            .with_style(row_container_style)
            .with_padding_left(padding)
            .into_any_named("project panel entry visual element")
    }

    fn render_entry(
        entry_id: ProjectEntryId,
        details: EntryDetails,
        editor: &ViewHandle<Editor>,
        dragged_entry_destination: &mut Option<Arc<Path>>,
        theme: &theme::ProjectPanel,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let kind = details.kind;
        let path = details.path.clone();
        let settings = settings::get::<ProjectPanelSettings>(cx);
        let padding = theme.container.padding.left + details.depth as f32 * settings.indent_size;

        let entry_style = if details.is_cut {
            &theme.cut_entry
        } else if details.is_ignored {
            &theme.ignored_entry
        } else {
            &theme.entry
        };

        let show_editor = details.is_editing && !details.is_processing;

        MouseEventHandler::new::<Self, _>(entry_id.to_usize(), cx, |state, cx| {
            let mut style = entry_style
                .in_state(details.is_selected)
                .style_for(state)
                .clone();

            if cx
                .global::<DragAndDrop<Workspace>>()
                .currently_dragged::<ProjectEntryId>(cx.window())
                .is_some()
                && dragged_entry_destination
                    .as_ref()
                    .filter(|destination| details.path.starts_with(destination))
                    .is_some()
            {
                style = entry_style.active_state().default.clone();
            }

            let row_container_style = if show_editor {
                theme.filename_editor.container
            } else {
                style.container
            };

            Self::render_entry_visual_element(
                &details,
                Some(editor),
                padding,
                row_container_style,
                &style,
                cx,
            )
        })
        .on_click(MouseButton::Left, move |event, this, cx| {
            if !show_editor {
                if kind.is_dir() {
                    this.toggle_expanded(entry_id, cx);
                } else {
                    if event.cmd {
                        this.split_entry(entry_id, cx);
                    } else if !event.cmd {
                        this.open_entry(entry_id, event.click_count > 1, cx);
                    }
                }
            }
        })
        .on_down(MouseButton::Right, move |event, this, cx| {
            this.deploy_context_menu(event.position, entry_id, cx);
        })
        .on_up(MouseButton::Left, move |_, this, cx| {
            if let Some((_, dragged_entry)) = cx
                .global::<DragAndDrop<Workspace>>()
                .currently_dragged::<ProjectEntryId>(cx.window())
            {
                this.move_entry(
                    *dragged_entry,
                    entry_id,
                    matches!(details.kind, EntryKind::File(_)),
                    cx,
                );
            }
        })
        .on_move(move |_, this, cx| {
            if cx
                .global::<DragAndDrop<Workspace>>()
                .currently_dragged::<ProjectEntryId>(cx.window())
                .is_some()
            {
                this.dragged_entry_destination = if matches!(kind, EntryKind::File(_)) {
                    path.parent().map(|parent| Arc::from(parent))
                } else {
                    Some(path.clone())
                };
            }
        })
        .as_draggable(entry_id, {
            let row_container_style = theme.dragged_entry.container;

            move |_, cx: &mut ViewContext<Workspace>| {
                let theme = theme::current(cx).clone();
                Self::render_entry_visual_element(
                    &details,
                    None,
                    padding,
                    row_container_style,
                    &theme.project_panel.dragged_entry,
                    cx,
                )
            }
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .into_any_named("project panel entry")
    }
}

impl View for ProjectPanel {
    fn ui_name() -> &'static str {
        "ProjectPanel"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> gpui::AnyElement<Self> {
        enum ProjectPanel {}
        let theme = &theme::current(cx).project_panel;
        let mut container_style = theme.container;
        let padding = std::mem::take(&mut container_style.padding);
        let last_worktree_root_id = self.last_worktree_root_id;

        let has_worktree = self.visible_entries.len() != 0;

        if has_worktree {
            Stack::new()
                .with_child(
                    MouseEventHandler::new::<ProjectPanel, _>(0, cx, |_, cx| {
                        UniformList::new(
                            self.list.clone(),
                            self.visible_entries
                                .iter()
                                .map(|(_, worktree_entries)| worktree_entries.len())
                                .sum(),
                            cx,
                            move |this, range, items, cx| {
                                let theme = theme::current(cx).clone();
                                let mut dragged_entry_destination =
                                    this.dragged_entry_destination.clone();
                                this.for_each_visible_entry(range, cx, |id, details, cx| {
                                    items.push(Self::render_entry(
                                        id,
                                        details,
                                        &this.filename_editor,
                                        &mut dragged_entry_destination,
                                        &theme.project_panel,
                                        cx,
                                    ));
                                });
                                this.dragged_entry_destination = dragged_entry_destination;
                            },
                        )
                        .with_padding_top(padding.top)
                        .with_padding_bottom(padding.bottom)
                        .contained()
                        .with_style(container_style)
                        .expanded()
                    })
                    .on_down(MouseButton::Right, move |event, this, cx| {
                        // When deploying the context menu anywhere below the last project entry,
                        // act as if the user clicked the root of the last worktree.
                        if let Some(entry_id) = last_worktree_root_id {
                            this.deploy_context_menu(event.position, entry_id, cx);
                        }
                    }),
                )
                .with_child(ChildView::new(&self.context_menu, cx))
                .into_any_named("project panel")
        } else {
            Flex::column()
                .with_child(
                    MouseEventHandler::new::<Self, _>(2, cx, {
                        let button_style = theme.open_project_button.clone();
                        let context_menu_item_style = theme::current(cx).context_menu.item.clone();
                        move |state, cx| {
                            let button_style = button_style.style_for(state).clone();
                            let context_menu_item = context_menu_item_style
                                .active_state()
                                .style_for(state)
                                .clone();

                            theme::ui::keystroke_label(
                                "Open a project",
                                &button_style,
                                &context_menu_item.keystroke,
                                Box::new(workspace::Open),
                                cx,
                            )
                        }
                    })
                    .on_click(MouseButton::Left, move |_, this, cx| {
                        if let Some(workspace) = this.workspace.upgrade(cx) {
                            workspace.update(cx, |workspace, cx| {
                                if let Some(task) = workspace.open(&Default::default(), cx) {
                                    task.detach_and_log_err(cx);
                                }
                            })
                        }
                    })
                    .with_cursor_style(CursorStyle::PointingHand),
                )
                .contained()
                .with_style(container_style)
                .into_any_named("empty project panel")
        }
    }

    fn update_keymap_context(&self, keymap: &mut KeymapContext, _: &AppContext) {
        Self::reset_to_default_keymap_context(keymap);
        keymap.add_identifier("menu");
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.has_focus {
            self.has_focus = true;
            cx.emit(Event::Focus);
        }
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }
}

impl Entity for ProjectPanel {
    type Event = Event;
}

impl workspace::dock::Panel for ProjectPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        match settings::get::<ProjectPanelSettings>(cx).dock {
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
            move |settings| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => ProjectPanelDockPosition::Left,
                    DockPosition::Right => ProjectPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> f32 {
        self.width
            .unwrap_or_else(|| settings::get::<ProjectPanelSettings>(cx).default_width)
    }

    fn set_size(&mut self, size: Option<f32>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon_path(&self, _: &WindowContext) -> Option<&'static str> {
        Some("icons/project.svg")
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn Action>>) {
        ("Project Panel".into(), Some(Box::new(ToggleFocus)))
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::DockPositionChanged)
    }

    fn has_focus(&self, _: &WindowContext) -> bool {
        self.has_focus
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, Event::Focus)
    }
}

impl ClipboardEntry {
    fn is_cut(&self) -> bool {
        matches!(self, Self::Cut { .. })
    }

    fn entry_id(&self) -> ProjectEntryId {
        match self {
            ClipboardEntry::Copied { entry_id, .. } | ClipboardEntry::Cut { entry_id, .. } => {
                *entry_id
            }
        }
    }

    fn worktree_id(&self) -> WorktreeId {
        match self {
            ClipboardEntry::Copied { worktree_id, .. }
            | ClipboardEntry::Cut { worktree_id, .. } => *worktree_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AnyWindowHandle, TestAppContext, ViewHandle, WindowHandle};
    use pretty_assertions::assert_eq;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{
        collections::HashSet,
        path::Path,
        sync::atomic::{self, AtomicUsize},
    };
    use workspace::{pane, AppState};

    #[gpui::test]
    async fn test_visible_list(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
        let workspace = cx
            .add_window(|cx| Workspace::test_new(project.clone(), cx))
            .root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));
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

    #[gpui::test(iterations = 30)]
    async fn test_editing_files(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));

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
        window.read_with(cx, |cx| {
            let panel = panel.read(cx);
            assert!(panel.filename_editor.is_focused(cx));
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
            panel.confirm(&Confirm, cx).unwrap()
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
                "      the-new-filename  <== selected",
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
                panel.confirm(&Confirm, cx).unwrap()
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
                "          another-filename.txt  <== selected",
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
                "          [EDITOR: 'another-filename.txt']  <== selected",
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
            panel.confirm(&Confirm, cx).unwrap()
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
                "          [PROCESSING: 'a-different-filename.tar.gz']  <== selected",
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
                assert_eq!(file_name_selection.end, "a-different-filename.tar".len(), "Should not select file extension, but still may select anything up to the last dot");

            });
            panel.cancel(&Cancel, cx)
        });

        panel.update(cx, |panel, cx| panel.new_directory(&NewDirectory, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > [EDITOR: '']  <== selected",
                "        > 3",
                "        > 4",
                "          a-different-filename.tar.gz",
                "    > C",
                "      .dockerignore",
            ]
        );

        let confirm = panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("new-dir", cx));
            panel.confirm(&Confirm, cx).unwrap()
        });
        panel.update(cx, |panel, cx| panel.select_next(&Default::default(), cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > [PROCESSING: 'new-dir']",
                "        > 3  <== selected",
                "        > 4",
                "          a-different-filename.tar.gz",
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
                "        > 3  <== selected",
                "        > 4",
                "        > new-dir",
                "          a-different-filename.tar.gz",
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
                "        > [EDITOR: '3']  <== selected",
                "        > 4",
                "        > new-dir",
                "          a-different-filename.tar.gz",
                "    > C",
                "      .dockerignore",
            ]
        );

        // Dismiss the rename editor when it loses focus.
        workspace.update(cx, |_, cx| cx.focus_self());
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    > a",
                "    v b",
                "        > 3  <== selected",
                "        > 4",
                "        > new-dir",
                "          a-different-filename.tar.gz",
                "    > C",
                "      .dockerignore",
            ]
        );
    }

    #[gpui::test(iterations = 30)]
    async fn test_adding_directories_via_file(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));

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
        window.read_with(cx, |cx| {
            let panel = panel.read(cx);
            assert!(panel.filename_editor.is_focused(cx));
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
            panel.confirm(&Confirm, cx).unwrap()
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
                "              the-new-filename  <== selected",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_paste(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root1",
            json!({
                "one.two.txt": "",
                "one.txt": ""
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
        let workspace = cx
            .add_window(|cx| Workspace::test_new(project.clone(), cx))
            .root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));

        panel.update(cx, |panel, cx| {
            panel.select_next(&Default::default(), cx);
            panel.select_next(&Default::default(), cx);
        });

        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "      one.two.txt  <== selected",
                "      one.txt",
            ]
        );

        // Regression test - file name is created correctly when
        // the copied file's name contains multiple dots.
        panel.update(cx, |panel, cx| {
            panel.copy(&Default::default(), cx);
            panel.paste(&Default::default(), cx);
        });
        cx.foreground().run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "      one.two copy.txt",
                "      one.two.txt  <== selected",
                "      one.txt",
            ]
        );

        panel.update(cx, |panel, cx| {
            panel.paste(&Default::default(), cx);
        });
        cx.foreground().run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                //
                "v root1",
                "      one.two copy 1.txt",
                "      one.two copy.txt",
                "      one.two.txt  <== selected",
                "      one.txt",
            ]
        );
    }

    #[gpui::test]
    async fn test_remove_opened_file(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.background());
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
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));

        toggle_expand_dir(&panel, "src/test", cx);
        select_path(&panel, "src/test/first.rs", cx);
        panel.update(cx, |panel, cx| panel.open_file(&Open, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs  <== selected",
                "          second.rs",
                "          third.rs"
            ]
        );
        ensure_single_file_is_opened(window, "test/first.rs", cx);

        submit_deletion(window.into(), &panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          second.rs",
                "          third.rs"
            ],
            "Project panel should have no deleted file, no other file is selected in it"
        );
        ensure_no_open_items_and_panes(window.into(), &workspace, cx);

        select_path(&panel, "src/test/second.rs", cx);
        panel.update(cx, |panel, cx| panel.open_file(&Open, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          second.rs  <== selected",
                "          third.rs"
            ]
        );
        ensure_single_file_is_opened(window, "test/second.rs", cx);

        window.update(cx, |cx| {
            let active_items = workspace
                .read(cx)
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
        });
        submit_deletion(window.into(), &panel, cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v src", "    v test", "          third.rs"],
            "Project panel should have no deleted file, with one last file remaining"
        );
        ensure_no_open_items_and_panes(window.into(), &workspace, cx);
    }

    #[gpui::test]
    async fn test_create_duplicate_items(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.background());
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
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));

        select_path(&panel, "src/", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v src  <== selected", "    > test"]
        );
        panel.update(cx, |panel, cx| panel.new_directory(&NewDirectory, cx));
        window.read_with(cx, |cx| {
            let panel = panel.read(cx);
            assert!(panel.filename_editor.is_focused(cx));
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v src", "    > [EDITOR: '']  <== selected", "    > test"]
        );
        panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("test", cx));
            assert!(
                panel.confirm(&Confirm, cx).is_none(),
                "Should not allow to confirm on conflicting new directory name"
            )
        });
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v src", "    > test"],
            "File list should be unchanged after failed folder create confirmation"
        );

        select_path(&panel, "src/test/", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v src", "    > test  <== selected"]
        );
        panel.update(cx, |panel, cx| panel.new_file(&NewFile, cx));
        window.read_with(cx, |cx| {
            let panel = panel.read(cx);
            assert!(panel.filename_editor.is_focused(cx));
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
                panel.confirm(&Confirm, cx).is_none(),
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
        cx.foreground().run_until_parked();
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
        window.read_with(cx, |cx| {
            let panel = panel.read(cx);
            assert!(panel.filename_editor.is_focused(cx));
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
                panel.confirm(&Confirm, cx).is_none(),
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
    async fn test_new_search_in_directory_trigger(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.background());
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
        let workspace = cx
            .add_window(|cx| Workspace::test_new(project.clone(), cx))
            .root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));

        let new_search_events_count = Arc::new(AtomicUsize::new(0));
        let _subscription = panel.update(cx, |_, cx| {
            let subcription_count = Arc::clone(&new_search_events_count);
            cx.subscribe(&cx.handle(), move |_, _, event, _| {
                if matches!(event, Event::NewSearchInDirectory { .. }) {
                    subcription_count.fetch_add(1, atomic::Ordering::SeqCst);
                }
            })
        });

        toggle_expand_dir(&panel, "src/test", cx);
        select_path(&panel, "src/test/first.rs", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          first.rs  <== selected",
                "          second.rs",
                "          third.rs"
            ]
        );
        panel.update(cx, |panel, cx| {
            panel.new_search_in_directory(&NewSearchInDirectory, cx)
        });
        assert_eq!(
            new_search_events_count.load(atomic::Ordering::SeqCst),
            0,
            "Should not trigger new search in directory when called on a file"
        );

        select_path(&panel, "src/test", cx);
        panel.update(cx, |panel, cx| panel.confirm(&Confirm, cx));
        cx.foreground().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test  <== selected",
                "          first.rs",
                "          second.rs",
                "          third.rs"
            ]
        );
        panel.update(cx, |panel, cx| {
            panel.new_search_in_directory(&NewSearchInDirectory, cx)
        });
        assert_eq!(
            new_search_events_count.load(atomic::Ordering::SeqCst),
            1,
            "Should trigger new search in directory when called on a directory"
        );
    }

    #[gpui::test]
    async fn test_collapse_all_entries(cx: &mut gpui::TestAppContext) {
        init_test_with_editor(cx);

        let fs = FakeFs::new(cx.background());
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
        let workspace = cx
            .add_window(|cx| Workspace::test_new(project.clone(), cx))
            .root(cx);
        let panel = workspace.update(cx, |workspace, cx| ProjectPanel::new(workspace, cx));

        panel.update(cx, |panel, cx| {
            panel.collapse_all_entries(&CollapseAllEntries, cx)
        });
        cx.foreground().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["v project_root", "    > dir_1", "    > dir_2",]
        );

        // Open dir_1 and make sure nested_dir was collapsed when running collapse_all_entries
        toggle_expand_dir(&panel, "project_root/dir_1", cx);
        cx.foreground().run_until_parked();
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

    fn toggle_expand_dir(
        panel: &ViewHandle<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut TestAppContext,
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

    fn select_path(
        panel: &ViewHandle<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                    panel.selection = Some(Selection {
                        worktree_id: worktree.id(),
                        entry_id,
                    });
                    return;
                }
            }
            panic!("no worktree for path {:?}", path);
        });
    }

    fn visible_entries_as_strings(
        panel: &ViewHandle<ProjectPanel>,
        range: Range<usize>,
        cx: &mut TestAppContext,
    ) -> Vec<String> {
        let mut result = Vec::new();
        let mut project_entries = HashSet::new();
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
                result.push(format!("{indent}{icon}{name}{selected}"));
            });
        });

        result
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            cx.set_global(SettingsStore::test(cx));
            init_settings(cx);
            theme::init((), cx);
            language::init(cx);
            editor::init_settings(cx);
            crate::init((), cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
        });
    }

    fn init_test_with_editor(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme::init((), cx);
            init_settings(cx);
            language::init(cx);
            editor::init(cx);
            pane::init(cx);
            crate::init((), cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);
        });
    }

    fn ensure_single_file_is_opened(
        window: WindowHandle<Workspace>,
        expected_path: &str,
        cx: &mut TestAppContext,
    ) {
        window.update_root(cx, |workspace, cx| {
            let worktrees = workspace.worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            let worktree_id = WorktreeId::from_usize(worktrees[0].id());

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
        });
    }

    fn submit_deletion(
        window: AnyWindowHandle,
        panel: &ViewHandle<ProjectPanel>,
        cx: &mut TestAppContext,
    ) {
        assert!(
            !window.has_pending_prompt(cx),
            "Should have no prompts before the deletion"
        );
        panel.update(cx, |panel, cx| {
            panel
                .delete(&Delete, cx)
                .expect("Deletion start")
                .detach_and_log_err(cx);
        });
        assert!(
            window.has_pending_prompt(cx),
            "Should have a prompt after the deletion"
        );
        window.simulate_prompt_answer(0, cx);
        assert!(
            !window.has_pending_prompt(cx),
            "Should have no prompts after prompt was replied to"
        );
        cx.foreground().run_until_parked();
    }

    fn ensure_no_open_items_and_panes(
        window: AnyWindowHandle,
        workspace: &ViewHandle<Workspace>,
        cx: &mut TestAppContext,
    ) {
        assert!(
            !window.has_pending_prompt(cx),
            "Should have no prompts after deletion operation closes the file"
        );
        window.read_with(cx, |cx| {
            let open_project_paths = workspace
                .read(cx)
                .panes()
                .iter()
                .filter_map(|pane| pane.read(cx).active_item()?.project_path(cx))
                .collect::<Vec<_>>();
            assert!(
                open_project_paths.is_empty(),
                "Deleted file's buffer should be closed, but got open files: {open_project_paths:?}"
            );
        });
    }
}
// TODO - a workspace command?
