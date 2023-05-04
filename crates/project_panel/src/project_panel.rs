use context_menu::{ContextMenu, ContextMenuItem};
use drag_and_drop::{DragAndDrop, Draggable};
use editor::{Cancel, Editor};
use futures::stream::StreamExt;
use gpui::{
    actions,
    anyhow::{anyhow, Result},
    elements::{
        AnchorCorner, ChildView, ContainerStyle, Empty, Flex, Label, MouseEventHandler,
        ParentElement, ScrollTarget, Stack, Svg, UniformList, UniformListState,
    },
    geometry::vector::Vector2F,
    keymap_matcher::KeymapContext,
    platform::{CursorStyle, MouseButton, PromptLevel},
    AnyElement, AppContext, ClipboardItem, Element, Entity, ModelHandle, Task, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
use menu::{Confirm, SelectNext, SelectPrev};
use project::{Entry, EntryKind, Project, ProjectEntryId, ProjectPath, Worktree, WorktreeId};
use settings::Settings;
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
use workspace::Workspace;

const NEW_ENTRY_ID: ProjectEntryId = ProjectEntryId::MAX;

pub struct ProjectPanel {
    project: ModelHandle<Project>,
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
}

#[derive(Copy, Clone)]
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
    path: Arc<Path>,
    depth: usize,
    kind: EntryKind,
    is_ignored: bool,
    is_expanded: bool,
    is_selected: bool,
    is_editing: bool,
    is_processing: bool,
    is_cut: bool,
}

actions!(
    project_panel,
    [
        ExpandSelectedEntry,
        CollapseSelectedEntry,
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
        ToggleFocus
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.add_action(ProjectPanel::expand_selected_entry);
    cx.add_action(ProjectPanel::collapse_selected_entry);
    cx.add_action(ProjectPanel::select_prev);
    cx.add_action(ProjectPanel::select_next);
    cx.add_action(ProjectPanel::new_file);
    cx.add_action(ProjectPanel::new_directory);
    cx.add_action(ProjectPanel::rename);
    cx.add_async_action(ProjectPanel::delete);
    cx.add_async_action(ProjectPanel::confirm);
    cx.add_action(ProjectPanel::cancel);
    cx.add_action(ProjectPanel::cut);
    cx.add_action(ProjectPanel::copy);
    cx.add_action(ProjectPanel::copy_path);
    cx.add_action(ProjectPanel::copy_relative_path);
    cx.add_action(ProjectPanel::reveal_in_finder);
    cx.add_action(
        |this: &mut ProjectPanel, action: &Paste, cx: &mut ViewContext<ProjectPanel>| {
            this.paste(action, cx);
        },
    );
}

pub enum Event {
    OpenedEntry {
        entry_id: ProjectEntryId,
        focus_opened_item: bool,
    },
}

impl ProjectPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
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

            let mut this = Self {
                project: project.clone(),
                list: Default::default(),
                visible_entries: Default::default(),
                last_worktree_root_id: Default::default(),
                expanded_dir_ids: Default::default(),
                selection: None,
                edit_state: None,
                filename_editor,
                clipboard_entry: None,
                context_menu: cx.add_view(ContextMenu::new),
                dragged_entry_destination: None,
                workspace: workspace.weak_handle(),
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
            }
        })
        .detach();

        project_panel
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
                let expanded_dir_ids =
                    if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree.id()) {
                        expanded_dir_ids
                    } else {
                        return;
                    };

                match expanded_dir_ids.binary_search(&entry.id) {
                    Ok(_) => self.select_next(&SelectNext, cx),
                    Err(ix) => {
                        expanded_dir_ids.insert(ix, entry.id);
                        self.update_visible_entries(None, cx);
                        cx.notify();
                    }
                }
            }
        }
    }

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some((worktree, mut entry)) = self.selected_entry(cx) {
            let expanded_dir_ids =
                if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree.id()) {
                    expanded_dir_ids
                } else {
                    return;
                };

            loop {
                match expanded_dir_ids.binary_search(&entry.id) {
                    Ok(ix) => {
                        expanded_dir_ids.remove(ix);
                        self.update_visible_entries(Some((worktree.id(), entry.id)), cx);
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

    fn toggle_expanded(&mut self, entry_id: ProjectEntryId, cx: &mut ViewContext<Self>) {
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx) {
            if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                match expanded_dir_ids.binary_search(&entry_id) {
                    Ok(ix) => {
                        expanded_dir_ids.remove(ix);
                    }
                    Err(ix) => {
                        expanded_dir_ids.insert(ix, entry_id);
                    }
                }
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
            Some(task)
        } else if let Some((_, entry)) = self.selected_entry(cx) {
            if entry.is_file() {
                self.open_entry(entry.id, true, cx);
            }
            None
        } else {
            None
        }
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

        let edit_task;
        let edited_entry_id;

        if is_new_entry {
            self.selection = Some(Selection {
                worktree_id,
                entry_id: NEW_ENTRY_ID,
            });
            let new_path = entry.path.join(&filename);
            edited_entry_id = NEW_ENTRY_ID;
            edit_task = self.project.update(cx, |project, cx| {
                project.create_entry((worktree_id, new_path), is_dir, cx)
            })?;
        } else {
            let new_path = if let Some(parent) = entry.path.clone().parent() {
                parent.join(&filename)
            } else {
                filename.clone().into()
            };
            edited_entry_id = entry.id;
            edit_task = self.project.update(cx, |project, cx| {
                project.rename_entry(entry.id, new_path, cx)
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
                    let filename = entry
                        .path
                        .file_name()
                        .map_or(String::new(), |s| s.to_string_lossy().to_string());
                    self.filename_editor.update(cx, |editor, cx| {
                        editor.set_text(filename, cx);
                        editor.select_all(&Default::default(), cx);
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

    fn selected_entry<'a>(&self, cx: &'a AppContext) -> Option<(&'a Worktree, &'a project::Entry)> {
        let selection = self.selection?;
        let project = self.project.read(cx);
        let worktree = project.worktree_for_id(selection.worktree_id, cx)?.read(cx);
        Some((worktree, worktree.entry_for_id(selection.entry_id)?))
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
                    });
                }
                if expanded_dir_ids.binary_search(&entry.id).is_err()
                    && entry_iter.advance_to_sibling()
                {
                    continue;
                }
                entry_iter.advance();
            }
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
        let project = self.project.read(cx);
        if let Some((worktree, expanded_dir_ids)) = project
            .worktree_for_id(worktree_id, cx)
            .zip(self.expanded_dir_ids.get_mut(&worktree_id))
        {
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
            if let Some(worktree) = self.project.read(cx).worktree_for_id(*worktree_id, cx) {
                let snapshot = worktree.read(cx).snapshot();
                let root_name = OsStr::new(snapshot.root_name());
                let expanded_entry_ids = self
                    .expanded_dir_ids
                    .get(&snapshot.id())
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);

                let entry_range = range.start.saturating_sub(ix)..end_ix - ix;
                for entry in &visible_worktree_entries[entry_range] {
                    let mut details = EntryDetails {
                        filename: entry
                            .path
                            .file_name()
                            .unwrap_or(root_name)
                            .to_string_lossy()
                            .to_string(),
                        path: entry.path.clone(),
                        depth: entry.path.components().count(),
                        kind: entry.kind,
                        is_ignored: entry.is_ignored,
                        is_expanded: expanded_entry_ids.binary_search(&entry.id).is_ok(),
                        is_selected: self.selection.map_or(false, |e| {
                            e.worktree_id == snapshot.id() && e.entry_id == entry.id
                        }),
                        is_editing: false,
                        is_processing: false,
                        is_cut: self
                            .clipboard_entry
                            .map_or(false, |e| e.is_cut() && e.entry_id() == entry.id),
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
        let kind = details.kind;
        let show_editor = details.is_editing && !details.is_processing;

        Flex::row()
            .with_child(
                if kind == EntryKind::Dir {
                    if details.is_expanded {
                        Svg::new("icons/chevron_down_8.svg").with_color(style.icon_color)
                    } else {
                        Svg::new("icons/chevron_right_8.svg").with_color(style.icon_color)
                    }
                    .constrained()
                } else {
                    Empty::new().constrained()
                }
                .with_max_width(style.icon_size)
                .with_max_height(style.icon_size)
                .aligned()
                .constrained()
                .with_width(style.icon_size),
            )
            .with_child(if show_editor && editor.is_some() {
                ChildView::new(editor.as_ref().unwrap(), cx)
                    .contained()
                    .with_margin_left(style.icon_spacing)
                    .aligned()
                    .left()
                    .flex(1.0, true)
                    .into_any()
            } else {
                Label::new(details.filename.clone(), style.text.clone())
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
        let padding = theme.container.padding.left + details.depth as f32 * theme.indent_width;

        let entry_style = if details.is_cut {
            &theme.cut_entry
        } else if details.is_ignored {
            &theme.ignored_entry
        } else {
            &theme.entry
        };

        let show_editor = details.is_editing && !details.is_processing;

        MouseEventHandler::<Self, _>::new(entry_id.to_usize(), cx, |state, cx| {
            let mut style = entry_style.style_for(state, details.is_selected).clone();

            if cx
                .global::<DragAndDrop<Workspace>>()
                .currently_dragged::<ProjectEntryId>(cx.window_id())
                .is_some()
                && dragged_entry_destination
                    .as_ref()
                    .filter(|destination| details.path.starts_with(destination))
                    .is_some()
            {
                style = entry_style.active.clone().unwrap();
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
                if kind == EntryKind::Dir {
                    this.toggle_expanded(entry_id, cx);
                } else {
                    this.open_entry(entry_id, event.click_count > 1, cx);
                }
            }
        })
        .on_down(MouseButton::Right, move |event, this, cx| {
            this.deploy_context_menu(event.position, entry_id, cx);
        })
        .on_up(MouseButton::Left, move |_, this, cx| {
            if let Some((_, dragged_entry)) = cx
                .global::<DragAndDrop<Workspace>>()
                .currently_dragged::<ProjectEntryId>(cx.window_id())
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
                .currently_dragged::<ProjectEntryId>(cx.window_id())
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
                let theme = cx.global::<Settings>().theme.clone();
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
        let theme = &cx.global::<Settings>().theme.project_panel;
        let mut container_style = theme.container;
        let padding = std::mem::take(&mut container_style.padding);
        let last_worktree_root_id = self.last_worktree_root_id;

        let has_worktree = self.visible_entries.len() != 0;

        if has_worktree {
            Stack::new()
                .with_child(
                    MouseEventHandler::<ProjectPanel, _>::new(0, cx, |_, cx| {
                        UniformList::new(
                            self.list.clone(),
                            self.visible_entries
                                .iter()
                                .map(|(_, worktree_entries)| worktree_entries.len())
                                .sum(),
                            cx,
                            move |this, range, items, cx| {
                                let theme = cx.global::<Settings>().theme.clone();
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
                    MouseEventHandler::<Self, _>::new(2, cx, {
                        let button_style = theme.open_project_button.clone();
                        let context_menu_item_style =
                            cx.global::<Settings>().theme.context_menu.item.clone();
                        move |state, cx| {
                            let button_style = button_style.style_for(state, false).clone();
                            let context_menu_item =
                                context_menu_item_style.style_for(state, true).clone();

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
}

impl Entity for ProjectPanel {
    type Event = Event;
}

impl workspace::sidebar::SidebarItem for ProjectPanel {
    fn should_show_badge(&self, _: &AppContext) -> bool {
        false
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
    use gpui::{TestAppContext, ViewHandle};
    use project::FakeFs;
    use serde_json::json;
    use std::{collections::HashSet, path::Path};

    #[gpui::test]
    async fn test_visible_list(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let settings = Settings::test(cx);
            cx.set_global(settings);
        });

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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
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
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let settings = Settings::test(cx);
            cx.set_global(settings);
        });

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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
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
        cx.read_window(window_id, |cx| {
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
                    .update(cx, |editor, cx| editor.set_text("another-filename", cx));
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
                "          another-filename  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        select_path(&panel, "root1/b/another-filename", cx);
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
                "          [EDITOR: 'another-filename']  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        let confirm = panel.update(cx, |panel, cx| {
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("a-different-filename", cx));
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
                "          [PROCESSING: 'a-different-filename']  <== selected",
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
                "          a-different-filename  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

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
                "          a-different-filename",
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
                "          a-different-filename",
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
                "          a-different-filename",
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
                "          a-different-filename",
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
                "          a-different-filename",
                "    > C",
                "      .dockerignore",
            ]
        );
    }

    #[gpui::test]
    async fn test_copy_paste(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let settings = Settings::test(cx);
            cx.set_global(settings);
        });

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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
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
                let icon = if matches!(details.kind, EntryKind::Dir | EntryKind::PendingDir) {
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
}
