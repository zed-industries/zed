pub mod file_associations;
mod project_panel_settings;
use client::{ErrorCode, ErrorExt};
use settings::Settings;

use db::kvp::KEY_VALUE_STORE;
use editor::{actions::Cancel, items::entry_git_aware_label_color, scroll::Autoscroll, Editor};
use file_associations::FileAssociations;

use anyhow::{anyhow, Result};
use collections::{hash_map, HashMap};
use gpui::{
    actions, anchored, deferred, div, impl_actions, px, uniform_list, Action, AppContext,
    AssetSource, AsyncWindowContext, ClipboardItem, DismissEvent, Div, EventEmitter, FocusHandle,
    FocusableView, InteractiveElement, KeyContext, Model, MouseButton, MouseDownEvent,
    ParentElement, Pixels, Point, PromptLevel, Render, Stateful, Styled, Subscription, Task,
    UniformListScrollHandle, View, ViewContext, VisualContext as _, WeakView, WindowContext,
};
use menu::{Confirm, SelectNext, SelectPrev};
use project::{
    repository::GitFileStatus, Entry, EntryKind, Fs, Project, ProjectEntryId, ProjectPath,
    Worktree, WorktreeId,
};
use project_panel_settings::{ProjectPanelDockPosition, ProjectPanelSettings};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    ffi::OsStr,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use theme::ThemeSettings;
use ui::{prelude::*, v_flex, ContextMenu, Icon, KeyBinding, Label, ListItem};
use unicase::UniCase;
use util::{maybe, NumericPrefixWithSuffix, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    notifications::DetachAndPromptErr,
    Workspace,
};

const PROJECT_PANEL_KEY: &str = "ProjectPanel";
const NEW_ENTRY_ID: ProjectEntryId = ProjectEntryId::MAX;

pub struct ProjectPanel {
    project: Model<Project>,
    fs: Arc<dyn Fs>,
    scroll_handle: UniformListScrollHandle,
    focus_handle: FocusHandle,
    visible_entries: Vec<(WorktreeId, Vec<Entry>)>,
    last_worktree_root_id: Option<ProjectEntryId>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
    selection: Option<Selection>,
    context_menu: Option<(View<ContextMenu>, Point<Pixels>, Subscription)>,
    edit_state: Option<EditState>,
    filename_editor: View<Editor>,
    clipboard_entry: Option<ClipboardEntry>,
    _dragged_entry_destination: Option<Arc<Path>>,
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
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

#[derive(Debug, PartialEq, Eq, Clone)]
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
    is_dotenv: bool,
}

#[derive(PartialEq, Clone, Default, Debug, Deserialize)]
pub struct Delete {
    #[serde(default)]
    pub skip_prompt: bool,
}

impl_actions!(project_panel, [Delete]);

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
        OpenInTerminal,
        Cut,
        Paste,
        Rename,
        Open,
        ToggleFocus,
        NewSearchInDirectory,
    ]
);

pub fn init_settings(cx: &mut AppContext) {
    ProjectPanelSettings::register(cx);
}

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    init_settings(cx);
    file_associations::init(assets, cx);

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
    entry_id: ProjectEntryId,
    details: EntryDetails,
    width: Pixels,
}

impl ProjectPanel {
    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let project = workspace.project().clone();
        let project_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, Self::focus_in).detach();

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
                project::Event::WorktreeRemoved(id) => {
                    this.expanded_dir_ids.remove(id);
                    this.update_visible_entries(None, cx);
                    cx.notify();
                }
                project::Event::WorktreeUpdatedEntries(_, _) | project::Event::WorktreeAdded => {
                    this.update_visible_entries(None, cx);
                    cx.notify();
                }
                _ => {}
            })
            .detach();

            let filename_editor = cx.new_view(|cx| Editor::single_line(cx));

            cx.subscribe(&filename_editor, |this, _, event, cx| match event {
                editor::EditorEvent::BufferEdited
                | editor::EditorEvent::SelectionsChanged { .. } => {
                    this.autoscroll(cx);
                }
                editor::EditorEvent::Blurred => {
                    if this
                        .edit_state
                        .as_ref()
                        .map_or(false, |state| state.processing_filename.is_none())
                    {
                        this.edit_state = None;
                        this.update_visible_entries(None, cx);
                    }
                }
                _ => {}
            })
            .detach();

            cx.observe_global::<FileAssociations>(|_, cx| {
                cx.notify();
            })
            .detach();

            let mut this = Self {
                project: project.clone(),
                fs: workspace.app_state().fs.clone(),
                scroll_handle: UniformListScrollHandle::new(),
                focus_handle,
                visible_entries: Default::default(),
                last_worktree_root_id: Default::default(),
                expanded_dir_ids: Default::default(),
                selection: None,
                edit_state: None,
                context_menu: None,
                filename_editor,
                clipboard_entry: None,
                _dragged_entry_destination: None,
                workspace: workspace.weak_handle(),
                width: None,
                pending_serialization: Task::ready(None),
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
                            let file_path = entry.path.clone();
                            let worktree_id = worktree.read(cx).id();
                            let entry_id = entry.id;

                            workspace
                                .open_path(
                                    ProjectPath {
                                        worktree_id,
                                        path: file_path.clone(),
                                    },
                                    None,
                                    focus_opened_item,
                                    cx,
                                )
                                .detach_and_prompt_err("Failed to open file", cx, move |e, _| {
                                    match e.error_code() {
                                        ErrorCode::UnsharedItem => Some(format!(
                                            "{} is not shared by the host. This could be because it has been marked as `private`",
                                            file_path.display()
                                        )),
                                        _ => None,
                                    }
                                });

                            if let Some(project_panel) = project_panel.upgrade() {
                                // Always select the entry, regardless of whether it is opened or not.
                                project_panel.update(cx, |project_panel, _| {
                                    project_panel.selection = Some(Selection {
                                        worktree_id,
                                        entry_id
                                    });
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
        let this = cx.view().clone();
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

        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let is_root = Some(entry) == worktree.root_entry();
            let is_dir = entry.is_dir();
            let worktree_id = worktree.id();
            let is_local = project.is_local();
            let is_read_only = project.is_read_only();

            let context_menu = ContextMenu::build(cx, |menu, cx| {
                menu.context(self.focus_handle.clone()).when_else(
                    is_read_only,
                    |menu| {
                        menu.action("Copy Relative Path", Box::new(CopyRelativePath))
                            .when(is_dir, |menu| {
                                menu.action("Search Inside", Box::new(NewSearchInDirectory))
                            })
                    },
                    |menu| {
                        menu.when(is_local, |menu| {
                            menu.action(
                                "Add Folder to Project",
                                Box::new(workspace::AddFolderToProject),
                            )
                            .when(is_root, |menu| {
                                menu.entry(
                                    "Remove from Project",
                                    None,
                                    cx.handler_for(&this, move |this, cx| {
                                        this.project.update(cx, |project, cx| {
                                            project.remove_worktree(worktree_id, cx)
                                        });
                                    }),
                                )
                                .action("Collapse All", Box::new(CollapseAllEntries))
                            })
                        })
                        .action("New File", Box::new(NewFile))
                        .action("New Folder", Box::new(NewDirectory))
                        .separator()
                        .action("Cut", Box::new(Cut))
                        .action("Copy", Box::new(Copy))
                        .when_some(self.clipboard_entry, |menu, entry| {
                            menu.when(entry.worktree_id() == worktree_id, |menu| {
                                menu.action("Paste", Box::new(Paste))
                            })
                        })
                        .separator()
                        .action("Copy Path", Box::new(CopyPath))
                        .action("Copy Relative Path", Box::new(CopyRelativePath))
                        .separator()
                        .action("Reveal in Finder", Box::new(RevealInFinder))
                        .when(is_dir, |menu| {
                            menu.action("Open in Terminal", Box::new(OpenInTerminal))
                                .action("Search Inside", Box::new(NewSearchInDirectory))
                        })
                        .separator()
                        .action("Rename", Box::new(Rename))
                        .when(!is_root, |menu| {
                            menu.action("Delete", Box::new(Delete { skip_prompt: false }))
                        })
                    },
                )
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
                cx.focus(&self.focus_handle);
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

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(task) = self.confirm_edit(cx) {
            task.detach_and_log_err(cx);
        }
    }

    fn open(&mut self, _: &Open, cx: &mut ViewContext<Self>) {
        if let Some((_, entry)) = self.selected_entry(cx) {
            if entry.is_file() {
                self.open_entry(entry.id, true, cx);
            } else {
                self.toggle_expanded(entry.id, cx);
            }
        }
    }

    fn confirm_edit(&mut self, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let edit_state = self.edit_state.as_mut()?;
        cx.focus(&self.focus_handle);

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
            let new_path = entry.path.join(&filename.trim_start_matches('/'));
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

        Some(cx.spawn(|this, mut cx| async move {
            let new_entry = edit_task.await;
            this.update(&mut cx, |this, cx| {
                this.edit_state.take();
                cx.notify();
            })?;

            if let Some(new_entry) = new_entry? {
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
            }
            Ok(())
        }))
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        self.edit_state = None;
        self.update_visible_entries(None, cx);
        cx.focus(&self.focus_handle);
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
            self.filename_editor.update(cx, |editor, cx| {
                editor.clear(cx);
                editor.focus(cx);
            });
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

    fn delete(&mut self, action: &Delete, cx: &mut ViewContext<Self>) {
        maybe!({
            let Selection { entry_id, .. } = self.selection?;
            let path = self.project.read(cx).path_for_entry(entry_id, cx)?.path;
            let file_name = path.file_name()?;

            let answer = (!action.skip_prompt).then(|| {
                cx.prompt(
                    PromptLevel::Info,
                    &format!("Delete {file_name:?}?"),
                    None,
                    &["Delete", "Cancel"],
                )
            });

            cx.spawn(|this, mut cx| async move {
                if let Some(answer) = answer {
                    if answer.await != Ok(0) {
                        return Ok(());
                    }
                }
                this.update(&mut cx, |this, cx| {
                    this.project
                        .update(cx, |project, cx| project.delete_entry(entry_id, cx))
                        .ok_or_else(|| anyhow!("no such entry"))
                })??
                .await
            })
            .detach_and_log_err(cx);
            Some(())
        });
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
            self.scroll_handle.scroll_to_item(index);
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

    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        maybe!({
            let (worktree, entry) = self.selected_entry(cx)?;
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
            // If we're pasting into a file, or a directory into itself, go up one level.
            if entry.is_file() || (entry.is_dir() && entry.id == clipboard_entry.entry_id()) {
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
                self.project
                    .update(cx, |project, cx| {
                        project.rename_entry(clipboard_entry.entry_id(), new_path, cx)
                    })
                    .detach_and_log_err(cx)
            } else {
                self.project
                    .update(cx, |project, cx| {
                        project.copy_entry(clipboard_entry.entry_id(), new_path, cx)
                    })
                    .detach_and_log_err(cx)
            }

            Some(())
        });
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

    fn open_in_terminal(&mut self, _: &OpenInTerminal, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let path = worktree.abs_path().join(&entry.path);
            cx.dispatch_action(
                workspace::OpenTerminal {
                    working_directory: path,
                }
                .boxed_clone(),
            )
        }
    }

    pub fn new_search_in_directory(
        &mut self,
        _: &NewSearchInDirectory,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            if entry.is_dir() {
                let include_root = self.project.read(cx).visible_worktrees(cx).count() > 1;
                let dir_path = if include_root {
                    let mut full_path = PathBuf::from(worktree.root_name());
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
                        is_ignored: entry.is_ignored,
                        is_external: false,
                        is_private: false,
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
                                let maybe_numeric_ordering = maybe!({
                                    let num_and_remainder_a = Path::new(component_a.as_os_str())
                                        .file_stem()
                                        .and_then(|s| s.to_str())
                                        .and_then(
                                            NumericPrefixWithSuffix::from_numeric_prefixed_str,
                                        )?;
                                    let num_and_remainder_b = Path::new(component_b.as_os_str())
                                        .file_stem()
                                        .and_then(|s| s.to_str())
                                        .and_then(
                                            NumericPrefixWithSuffix::from_numeric_prefixed_str,
                                        )?;

                                    num_and_remainder_a.partial_cmp(&num_and_remainder_b)
                                });

                                maybe_numeric_ordering.unwrap_or_else(|| {
                                    let name_a =
                                        UniCase::new(component_a.as_os_str().to_string_lossy());
                                    let name_b =
                                        UniCase::new(component_b.as_os_str().to_string_lossy());

                                    name_a.cmp(&name_b)
                                })
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
                for entry in visible_worktree_entries[entry_range].iter() {
                    let status = git_status_setting.then(|| entry.git_status).flatten();
                    let is_expanded = expanded_entry_ids.binary_search(&entry.id).is_ok();
                    let icon = match entry.kind {
                        EntryKind::File(_) => {
                            if show_file_icons {
                                FileAssociations::get_icon(&entry.path, cx)
                            } else {
                                None
                            }
                        }
                        _ => {
                            if show_folder_icons {
                                FileAssociations::get_folder_icon(is_expanded, cx)
                            } else {
                                FileAssociations::get_chevron_icon(is_expanded, cx)
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
                        is_dotenv: entry.is_private,
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

    fn render_entry(
        &self,
        entry_id: ProjectEntryId,
        details: EntryDetails,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        let kind = details.kind;
        let settings = ProjectPanelSettings::get_global(cx);
        let show_editor = details.is_editing && !details.is_processing;
        let is_selected = self
            .selection
            .map_or(false, |selection| selection.entry_id == entry_id);
        let width = self.size(cx);
        let filename_text_color =
            entry_git_aware_label_color(details.git_status, details.is_ignored, is_selected);
        let file_name = details.filename.clone();
        let icon = details.icon.clone();
        let depth = details.depth;
        div()
            .id(entry_id.to_proto() as usize)
            .on_drag(entry_id, move |entry_id, cx| {
                cx.new_view(|_| DraggedProjectEntryView {
                    details: details.clone(),
                    width,
                    entry_id: *entry_id,
                })
            })
            .drag_over::<ProjectEntryId>(|style, _, cx| {
                style.bg(cx.theme().colors().drop_target_background)
            })
            .on_drop(cx.listener(move |this, dragged_id: &ProjectEntryId, cx| {
                this.move_entry(*dragged_id, entry_id, kind.is_file(), cx);
            }))
            .child(
                ListItem::new(entry_id.to_proto() as usize)
                    .indent_level(depth)
                    .indent_step_size(px(settings.indent_size))
                    .selected(is_selected)
                    .child(if let Some(icon) = &icon {
                        div().child(Icon::from_path(icon.to_string()).color(filename_text_color))
                    } else {
                        div().size(IconSize::default().rems()).invisible()
                    })
                    .child(
                        if let (Some(editor), true) = (Some(&self.filename_editor), show_editor) {
                            h_flex().h_6().w_full().child(editor.clone())
                        } else {
                            div()
                                .h_6()
                                .child(Label::new(file_name).color(filename_text_color))
                        }
                        .ml_1(),
                    )
                    .on_click(cx.listener(move |this, event: &gpui::ClickEvent, cx| {
                        if event.down.button == MouseButton::Right || event.down.first_mouse {
                            return;
                        }
                        if !show_editor {
                            if kind.is_dir() {
                                this.toggle_expanded(entry_id, cx);
                            } else {
                                if event.down.modifiers.command {
                                    this.split_entry(entry_id, cx);
                                } else {
                                    this.open_entry(entry_id, event.up.click_count > 1, cx);
                                }
                            }
                        }
                    }))
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, cx| {
                            // Stop propagation to prevent the catch-all context menu for the project
                            // panel from being deployed.
                            cx.stop_propagation();
                            this.deploy_context_menu(event.position, entry_id, cx);
                        },
                    )),
            )
    }

    fn dispatch_context(&self, cx: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::default();
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

    fn reveal_entry(
        &mut self,
        project: Model<Project>,
        entry_id: ProjectEntryId,
        skip_ignored: bool,
        cx: &mut ViewContext<'_, ProjectPanel>,
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
            self.autoscroll(cx);
            cx.notify();
        }
    }
}

impl Render for ProjectPanel {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let has_worktree = self.visible_entries.len() != 0;
        let project = self.project.read(cx);

        if has_worktree {
            div()
                .id("project-panel")
                .size_full()
                .relative()
                .key_context(self.dispatch_context(cx))
                .on_action(cx.listener(Self::select_next))
                .on_action(cx.listener(Self::select_prev))
                .on_action(cx.listener(Self::expand_selected_entry))
                .on_action(cx.listener(Self::collapse_selected_entry))
                .on_action(cx.listener(Self::collapse_all_entries))
                .on_action(cx.listener(Self::open))
                .on_action(cx.listener(Self::confirm))
                .on_action(cx.listener(Self::cancel))
                .on_action(cx.listener(Self::copy_path))
                .on_action(cx.listener(Self::copy_relative_path))
                .on_action(cx.listener(Self::new_search_in_directory))
                .when(!project.is_read_only(), |el| {
                    el.on_action(cx.listener(Self::new_file))
                        .on_action(cx.listener(Self::new_directory))
                        .on_action(cx.listener(Self::rename))
                        .on_action(cx.listener(Self::delete))
                        .on_action(cx.listener(Self::cut))
                        .on_action(cx.listener(Self::copy))
                        .on_action(cx.listener(Self::paste))
                })
                .when(project.is_local(), |el| {
                    el.on_action(cx.listener(Self::reveal_in_finder))
                        .on_action(cx.listener(Self::open_in_terminal))
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
                .track_focus(&self.focus_handle)
                .child(
                    uniform_list(
                        cx.view().clone(),
                        "entries",
                        self.visible_entries
                            .iter()
                            .map(|(_, worktree_entries)| worktree_entries.len())
                            .sum(),
                        {
                            |this, range, cx| {
                                let mut items = Vec::new();
                                this.for_each_visible_entry(range, cx, |id, details, cx| {
                                    items.push(this.render_entry(id, details, cx));
                                });
                                items
                            }
                        },
                    )
                    .size_full()
                    .track_scroll(self.scroll_handle.clone()),
                )
                .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                    deferred(
                        anchored()
                            .position(*position)
                            .anchor(gpui::AnchorCorner::TopLeft)
                            .child(menu.clone()),
                    )
                }))
        } else {
            v_flex()
                .id("empty-project_panel")
                .size_full()
                .p_4()
                .track_focus(&self.focus_handle)
                .child(
                    Button::new("open_project", "Open a project")
                        .style(ButtonStyle::Filled)
                        .full_width()
                        .key_binding(KeyBinding::for_action(&workspace::Open, cx))
                        .on_click(cx.listener(|this, _, cx| {
                            this.workspace
                                .update(cx, |workspace, cx| workspace.open(&workspace::Open, cx))
                                .log_err();
                        })),
                )
        }
    }
}

impl Render for DraggedProjectEntryView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element {
        let settings = ProjectPanelSettings::get_global(cx);
        let ui_font = ThemeSettings::get_global(cx).ui_font.family.clone();
        h_flex()
            .font(ui_font)
            .bg(cx.theme().colors().background)
            .w(self.width)
            .child(
                ListItem::new(self.entry_id.to_proto() as usize)
                    .indent_level(self.details.depth)
                    .indent_step_size(px(settings.indent_size))
                    .child(if let Some(icon) = &self.details.icon {
                        div().child(Icon::from_path(icon.to_string()))
                    } else {
                        div()
                    })
                    .child(Label::new(self.details.filename.clone())),
            )
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
            move |settings| {
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

    fn icon(&self, _: &WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::FileTree)
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
        self.project.read(cx).visible_worktrees(cx).any(|tree| {
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
    use collections::HashSet;
    use gpui::{TestAppContext, View, VisualTestContext, WindowHandle};
    use pretty_assertions::assert_eq;
    use project::{FakeFs, WorktreeSettings};
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::{Path, PathBuf};
    use workspace::AppState;

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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();
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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();
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
                assert_eq!(file_name_selection.end, "a-different-filename.tar".len(), "Should not select file extension, but still may select anything up to the last dot..");

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
        workspace.update(cx, |_, cx| cx.blur()).unwrap();
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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

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
        cx.executor().run_until_parked();

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
        cx.executor().run_until_parked();

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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

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

        toggle_expand_dir(&panel, "root/b", cx);
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
                "v root  <== selected",
                "    > a",
                "    > a copy",
                "    > a copy 1",
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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

        toggle_expand_dir(&panel, "src/test", cx);
        select_path(&panel, "src/test/first.rs", cx);
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
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
        ensure_single_file_is_opened(&workspace, "test/first.rs", cx);

        submit_deletion(&panel, cx);
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
        ensure_no_open_items_and_panes(&workspace, cx);

        select_path(&panel, "src/test/second.rs", cx);
        panel.update(cx, |panel, cx| panel.open(&Open, cx));
        cx.executor().run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v src",
                "    v test",
                "          second.rs  <== selected",
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
            &["v src", "    v test", "          third.rs"],
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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

        // Make a new buffer with no backing file
        workspace
            .update(cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
            .unwrap();

        // "Save as"" the buffer, creating a new backing file for it
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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

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
        let panel = workspace
            .update(cx, |workspace, cx| ProjectPanel::new(workspace, cx))
            .unwrap();

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

    fn toggle_expand_dir(
        panel: &View<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut VisualTestContext,
    ) {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees().collect::<Vec<_>>() {
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
            for worktree in panel.project.read(cx).worktrees().collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                    panel.selection = Some(crate::Selection {
                        worktree_id: worktree.id(),
                        entry_id,
                    });
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
            for worktree in panel.project.read(cx).worktrees().collect::<Vec<_>>() {
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
                result.push(format!("{indent}{icon}{name}{selected}"));
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
}
