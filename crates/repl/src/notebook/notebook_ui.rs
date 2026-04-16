use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use collections::HashMap;
use feature_flags::{FeatureFlagAppExt as _, NotebookFeatureFlag};
use futures::FutureExt as _;
use futures::future::Shared;
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, KeyContext, ListState, Point,
    Task, WeakEntity, list, prelude::*,
};
use language::{Language, LanguageRegistry};
use project::{Project, ProjectEntryId, ProjectPath};
use ui::{PopoverMenuHandle, Tooltip, prelude::*};
use workspace::item::{ItemEvent, SaveOptions, TabContentParams};
use workspace::searchable::SearchableItemHandle;
use workspace::{
    ItemId, Item, Pane, ProjectItem, SerializableItem, Workspace, WorkspaceId,
    delete_unloaded_items,
};

use super::{Cell, CellEvent, CellPosition, MarkdownCellEvent, RenderableCell};

use nbformat::v4::CellId;
use uuid::Uuid;

use crate::components::{KernelPickerDelegate, KernelSelector};
use crate::kernels::{
    Kernel, KernelSession, KernelSpecification, KernelStatus, NativeRunningKernel,
    RemoteRunningKernel, SshRunningKernel, WslRunningKernel,
};
use crate::repl_store::ReplStore;

use picker::Picker;
use runtimelib::{ExecuteRequest, JupyterMessage, JupyterMessageContent};
use zed_actions::editor::{MoveDown, MoveUp};
use zed_actions::notebook::{
    AddCodeBlock, AddMarkdownBlock, ClearOutputs, DeleteCell, EnterCommandMode, EnterEditMode,
    InterruptKernel, MoveCellDown, MoveCellUp, NotebookMoveDown, NotebookMoveUp, OpenNotebook,
    RestartKernel, Run, RunAll, RunAndAdvance,
};

/// Whether the notebook is in command mode (navigating cells) or edit mode (editing a cell).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NotebookMode {
    Command,
    Edit,
}

/// Events emitted by `NotebookEditor` so the workspace can update the tab dirty
/// indicator, schedule autosave, and trigger the unsaved-changes prompt on close.
pub enum NotebookEditorEvent {
    /// A cell was edited, added, removed, or moved.
    Edit,
    /// The displayed title (filename) changed.
    TitleChanged,
}

/// Toolbar-relevant snapshot of a `NotebookEditor`. Kept as a plain struct so
/// the toolbar can render without holding a borrow on the notebook itself.
pub struct NotebookToolbarState {
    pub kernel_status: KernelStatus,
    pub kernel_name: String,
    pub has_outputs: bool,
    pub cell_count: usize,
}

pub(crate) const MEDIUM_SPACING_SIZE: f32 = 12.0;
pub(crate) const GUTTER_WIDTH: f32 = 19.0;
pub(crate) const CODE_BLOCK_INSET: f32 = MEDIUM_SPACING_SIZE;
pub(crate) const CONTROL_SIZE: f32 = 20.0;

pub fn init(cx: &mut App) {
    if cx.has_flag::<NotebookFeatureFlag>() {
        workspace::register_project_item::<NotebookEditor>(cx);
        workspace::register_serializable_item::<NotebookEditor>(cx);
    }

    cx.observe_flag::<NotebookFeatureFlag, _>({
        move |is_enabled, cx| {
            if is_enabled {
                workspace::register_project_item::<NotebookEditor>(cx);
                workspace::register_serializable_item::<NotebookEditor>(cx);
            } else {
                // There is no way to unregister a project item; if the flag is turned off
                // server-side, the user must restart Zed for it to take effect.
            }
        }
    })
    .detach();
}

pub struct NotebookEditor {
    languages: Arc<LanguageRegistry>,
    project: Entity<Project>,
    worktree_id: project::WorktreeId,

    focus_handle: FocusHandle,
    notebook_item: Entity<NotebookItem>,
    notebook_language: Shared<Task<Option<Arc<Language>>>>,

    cell_list: ListState,

    notebook_mode: NotebookMode,
    selected_cell_index: usize,
    cell_order: Vec<CellId>,
    original_cell_order: Vec<CellId>,
    cell_map: HashMap<CellId, Cell>,
    kernel: Kernel,
    kernel_specification: Option<KernelSpecification>,
    execution_requests: HashMap<String, CellId>,
    kernel_picker_handle: PopoverMenuHandle<Picker<KernelPickerDelegate>>,
}

impl NotebookEditor {
    pub fn new(
        project: Entity<Project>,
        notebook_item: Entity<NotebookItem>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let languages = project.read(cx).languages().clone();
        let worktree_id = notebook_item.read(cx).project_path.worktree_id;

        let notebook_language = notebook_item.read(cx).notebook_language();
        let notebook_language = cx
            .spawn_in(window, async move |_, _| notebook_language.await)
            .shared();

        let mut cell_order = vec![]; // Vec<CellId>
        let mut cell_map = HashMap::default(); // HashMap<CellId, Cell>

        let cell_count = notebook_item.read(cx).notebook.cells.len();
        for index in 0..cell_count {
            let cell = notebook_item.read(cx).notebook.cells[index].clone();
            let cell_id = cell.id();
            cell_order.push(cell_id.clone());
            let cell_entity = Cell::load(&cell, &languages, notebook_language.clone(), window, cx);

            match &cell_entity {
                Cell::Code(code_cell) => {
                    Self::subscribe_to_code_cell(code_cell, cell_id.clone(), cx);
                }
                Cell::Markdown(markdown_cell) => {
                    Self::subscribe_to_markdown_cell(markdown_cell, cell_id.clone(), cx);
                }
                Cell::Raw(_) => {}
            }

            cell_map.insert(cell_id.clone(), cell_entity);
        }

        let cell_list = ListState::new(cell_order.len(), gpui::ListAlignment::Top, px(1000.));

        let mut editor = Self {
            project,
            languages: languages.clone(),
            worktree_id,
            focus_handle,
            notebook_item: notebook_item.clone(),
            notebook_language,
            cell_list,
            notebook_mode: NotebookMode::Command,
            selected_cell_index: 0,
            cell_order: cell_order.clone(),
            original_cell_order: cell_order.clone(),
            cell_map: cell_map.clone(),
            // Initial kernel state — `launch_kernel` below picks a real one from
            // `ReplStore::active_kernelspec` if the user's worktree has a Python with
            // ipykernel installed. Otherwise the kernel stays Shutdown until the user
            // picks one via the kernel picker.
            kernel: Kernel::Shutdown,
            kernel_specification: None,
            execution_requests: HashMap::default(),
            kernel_picker_handle: PopoverMenuHandle::default(),
        };
        editor.launch_kernel(window, cx);
        editor.refresh_language(cx);

        cx.subscribe(&notebook_item, |this, _item, _event, cx| {
            this.refresh_language(cx);
        })
        .detach();

        // Reload the notebook when its underlying buffer changes on disk
        // (e.g. external editor save, `git checkout`, file revert).
        let buffer = notebook_item.read(cx).buffer.clone();
        cx.subscribe_in(
            &buffer,
            window,
            |this, _buffer, event, window, cx| {
                if matches!(event, language::BufferEvent::Reloaded) {
                    let project = this.project.clone();
                    this.reload(project, window, cx)
                        .detach_and_log_err(cx);
                }
            },
        )
        .detach();

        editor
    }

    /// Subscribe `NotebookEditor` to a code cell so its run/focus events route here
    /// and edits inside the cell's embedded editor bubble up as `NotebookEditorEvent::Edit`.
    fn subscribe_to_code_cell(
        code_cell: &Entity<super::CodeCell>,
        cell_id: CellId,
        cx: &mut Context<Self>,
    ) {
        let cell_id_for_focus = cell_id.clone();
        cx.subscribe(code_cell, move |this, _cell, event, cx| match event {
            CellEvent::Run(cell_id) => this.execute_cell(cell_id.clone(), cx),
            CellEvent::FocusedIn(_) => this.select_cell_by_id(&cell_id_for_focus, cx),
        })
        .detach();

        let editor = code_cell.read(cx).editor().clone();
        cx.subscribe(&editor, move |this, _editor, event, cx| match event {
            editor::EditorEvent::Focused => {
                this.select_cell_by_id(&cell_id, cx);
            }
            editor::EditorEvent::BufferEdited => {
                this.note_edit(cx);
            }
            _ => {}
        })
        .detach();
    }

    /// Subscribe `NotebookEditor` to a markdown cell so render/run events route here
    /// and edits inside the cell's embedded editor bubble up as `NotebookEditorEvent::Edit`.
    fn subscribe_to_markdown_cell(
        markdown_cell: &Entity<super::MarkdownCell>,
        cell_id: CellId,
        cx: &mut Context<Self>,
    ) {
        cx.subscribe(
            markdown_cell,
            move |_this, cell, event: &MarkdownCellEvent, cx| match event {
                MarkdownCellEvent::FinishedEditing | MarkdownCellEvent::Run(_) => {
                    cell.update(cx, |cell, cx| {
                        cell.reparse_markdown(cx);
                    });
                }
            },
        )
        .detach();

        let editor = markdown_cell.read(cx).editor().clone();
        cx.subscribe(&editor, move |this, _editor, event, cx| match event {
            editor::EditorEvent::Focused => {
                this.select_cell_by_id(&cell_id, cx);
            }
            editor::EditorEvent::BufferEdited => {
                this.note_edit(cx);
            }
            _ => {}
        })
        .detach();
    }

    /// Mark the notebook as edited, propagating the change to the workspace tab indicator,
    /// autosave, and the unsaved-changes prompt via `to_item_events`.
    fn note_edit(&mut self, cx: &mut Context<Self>) {
        self.notebook_item.update(cx, |item, _cx| {
            item.dirty = true;
        });
        cx.emit(NotebookEditorEvent::Edit);
    }

    /// Snapshot of the notebook state needed by the toolbar.
    pub fn toolbar_state(&self, cx: &App) -> NotebookToolbarState {
        let has_outputs = self.cell_map.values().any(|cell| {
            if let Cell::Code(code) = cell {
                code.read(cx).has_outputs()
            } else {
                false
            }
        });
        let kernel_name = self
            .kernel_specification
            .as_ref()
            .map(|spec| spec.name().to_string())
            .unwrap_or_else(|| "Select Kernel".to_string());
        NotebookToolbarState {
            kernel_status: self.kernel.status(),
            kernel_name,
            has_outputs,
            cell_count: self.cell_map.len(),
        }
    }

    fn refresh_language(&mut self, cx: &mut Context<Self>) {
        let notebook_language = self.notebook_item.read(cx).notebook_language();
        let task = cx.spawn(async move |this, cx| {
            let language = notebook_language.await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| {
                    for cell in this.cell_map.values() {
                        if let Cell::Code(code_cell) = cell {
                            code_cell.update(cx, |cell, cx| {
                                cell.set_language(language.clone(), cx);
                            });
                        }
                    }
                });
            }
            language
        });
        self.notebook_language = task.shared();
    }

    fn has_structural_changes(&self) -> bool {
        self.cell_order != self.original_cell_order
    }

    fn has_content_changes(&self, cx: &App) -> bool {
        self.cell_map.values().any(|cell| cell.is_dirty(cx))
    }

    pub fn to_notebook(&self, cx: &App) -> nbformat::v4::Notebook {
        let cells: Vec<nbformat::v4::Cell> = self
            .cell_order
            .iter()
            .filter_map(|cell_id| {
                self.cell_map
                    .get(cell_id)
                    .map(|cell| cell.to_nbformat_cell(cx))
            })
            .collect();

        let metadata = self.notebook_item.read(cx).notebook.metadata.clone();

        nbformat::v4::Notebook {
            metadata,
            nbformat: 4,
            nbformat_minor: 5,
            cells,
        }
    }

    pub fn mark_as_saved(&mut self, cx: &mut Context<Self>) {
        self.original_cell_order = self.cell_order.clone();

        for cell in self.cell_map.values() {
            match cell {
                Cell::Code(code_cell) => {
                    code_cell.update(cx, |code_cell, cx| {
                        let editor = code_cell.editor();
                        editor.update(cx, |editor, cx| {
                            editor.buffer().update(cx, |buffer, cx| {
                                if let Some(buf) = buffer.as_singleton() {
                                    buf.update(cx, |b, cx| {
                                        let version = b.version();
                                        b.did_save(version, None, cx);
                                    });
                                }
                            });
                        });
                    });
                }
                Cell::Markdown(markdown_cell) => {
                    markdown_cell.update(cx, |markdown_cell, cx| {
                        let editor = markdown_cell.editor();
                        editor.update(cx, |editor, cx| {
                            editor.buffer().update(cx, |buffer, cx| {
                                if let Some(buf) = buffer.as_singleton() {
                                    buf.update(cx, |b, cx| {
                                        let version = b.version();
                                        b.did_save(version, None, cx);
                                    });
                                }
                            });
                        });
                    });
                }
                Cell::Raw(_) => {}
            }
        }
        self.notebook_item.update(cx, |item, _cx| {
            item.dirty = false;
        });
        cx.emit(NotebookEditorEvent::Edit);
        cx.notify();
    }

    /// Pick a sensible kernel and launch it. Reuses `ReplStore::active_kernelspec`,
    /// which prefers the worktree's active Python toolchain if it has `ipykernel`
    /// installed and falls back to other discovered Python environments.
    ///
    /// If no kernel can be picked yet (the async kernelspec discovery is still in
    /// flight, or the user has no Python with ipykernel installed), we leave the
    /// kernel in `Shutdown` state and wait — either for `ReplStore` to discover
    /// new specs (we observe it once and retry), or for the user to pick a kernel
    /// manually via the kernel picker.
    fn launch_kernel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(spec) = self.kernel_specification.clone() {
            self.launch_kernel_with_spec(spec, window, cx);
            return;
        }

        if let Some(spec) = self.pick_recommended_kernel(cx) {
            self.launch_kernel_with_spec(spec, window, cx);
            return;
        }

        // The notebook's language may not have resolved yet, or `ReplStore` may
        // still be scanning Python environments. Spawn a one-shot retry that
        // awaits the language and tries again.
        let language_task = self.notebook_language.clone();
        cx.spawn_in(window, async move |this, cx| {
            let _ = language_task.await;
            this.update_in(cx, |this, window, cx| {
                if matches!(this.kernel, Kernel::Shutdown)
                    && let Some(spec) = this.pick_recommended_kernel(cx)
                {
                    this.launch_kernel_with_spec(spec, window, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    fn pick_recommended_kernel(&self, cx: &App) -> Option<KernelSpecification> {
        let language = self.notebook_language.clone().now_or_never().flatten();
        ReplStore::global(cx)
            .read(cx)
            .active_kernelspec(self.worktree_id, language, cx)
    }

    fn launch_kernel_with_spec(
        &mut self,
        spec: KernelSpecification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entity_id = cx.entity_id();
        let working_directory = self
            .project
            .read(cx)
            .worktree_for_id(self.worktree_id, cx)
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .unwrap_or_else(std::env::temp_dir);
        let fs = self.project.read(cx).fs().clone();
        let view = cx.entity();

        self.kernel_specification = Some(spec.clone());

        self.notebook_item.update(cx, |item, cx| {
            let kernel_name = spec.name().to_string();
            let language = spec.language().to_string();

            let display_name = match &spec {
                KernelSpecification::Jupyter(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::PythonEnv(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::JupyterServer(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::SshRemote(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::WslRemote(s) => s.kernelspec.display_name.clone(),
            };

            let kernelspec_json = serde_json::json!({
                "display_name": display_name,
                "name": kernel_name,
                "language": language
            });

            if let Ok(k) = serde_json::from_value(kernelspec_json) {
                item.notebook.metadata.kernelspec = Some(k);
                cx.emit(());
            }
        });

        let kernel_task = match spec {
            KernelSpecification::Jupyter(local_spec) => NativeRunningKernel::new(
                local_spec,
                entity_id,
                working_directory,
                fs,
                view,
                window,
                cx,
            ),
            KernelSpecification::PythonEnv(env_spec) => NativeRunningKernel::new(
                env_spec.as_local_spec(),
                entity_id,
                working_directory,
                fs,
                view,
                window,
                cx,
            ),
            KernelSpecification::JupyterServer(remote_spec) => {
                RemoteRunningKernel::new(remote_spec, working_directory, view, window, cx)
            }

            KernelSpecification::SshRemote(spec) => {
                let project = self.project.clone();
                SshRunningKernel::new(spec, working_directory, project, view, window, cx)
            }
            KernelSpecification::WslRemote(spec) => {
                WslRunningKernel::new(spec, entity_id, working_directory, fs, view, window, cx)
            }
        };

        let pending_kernel = cx
            .spawn(async move |this, cx| {
                let kernel = kernel_task.await;

                match kernel {
                    Ok(kernel) => {
                        this.update(cx, |editor, cx| {
                            editor.kernel = Kernel::RunningKernel(kernel);
                            cx.notify();
                        })
                        .ok();
                    }
                    Err(err) => {
                        log::error!("Kernel failed to start: {:?}", err);
                        this.update(cx, |editor, cx| {
                            editor.kernel = Kernel::ErroredLaunch(err.to_string());
                            cx.notify();
                        })
                        .ok();
                    }
                }
            })
            .shared();

        self.kernel = Kernel::StartingKernel(pending_kernel);
        cx.notify();
    }

    // Note: Python environments are only detected as kernels if ipykernel is installed.
    // Users need to run `pip install ipykernel` (or `uv pip install ipykernel`) in their
    // virtual environment for it to appear in the kernel selector.
    // This happens because we have an ipykernel check inside the function python_env_kernel_specification in mod.rs L:121

    fn change_kernel(
        &mut self,
        spec: KernelSpecification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Kernel::RunningKernel(kernel) = &mut self.kernel {
            kernel.force_shutdown(window, cx).detach();
        }

        self.execution_requests.clear();

        self.launch_kernel_with_spec(spec, window, cx);
    }

    fn restart_kernel(&mut self, _: &RestartKernel, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(spec) = self.kernel_specification.clone() {
            if let Kernel::RunningKernel(kernel) = &mut self.kernel {
                kernel.force_shutdown(window, cx).detach();
            }

            self.kernel = Kernel::Restarting;
            cx.notify();

            self.launch_kernel_with_spec(spec, window, cx);
        }
    }

    fn interrupt_kernel(
        &mut self,
        _: &InterruptKernel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Kernel::RunningKernel(kernel) = &self.kernel {
            let interrupt_request = runtimelib::InterruptRequest {};
            let message: JupyterMessage = interrupt_request.into();
            kernel.request_tx().try_send(message).ok();
            cx.notify();
        }
    }

    fn execute_cell(&mut self, cell_id: CellId, cx: &mut Context<Self>) {
        let code = if let Some(Cell::Code(cell)) = self.cell_map.get(&cell_id) {
            let editor = cell.read(cx).editor().clone();
            let buffer = editor.read(cx).buffer().read(cx);
            buffer
                .as_singleton()
                .map(|b| b.read(cx).text())
                .unwrap_or_default()
        } else {
            return;
        };

        if let Some(Cell::Code(cell)) = self.cell_map.get(&cell_id) {
            cell.update(cx, |cell, cx| {
                if cell.has_outputs() {
                    cell.clear_outputs();
                }
                cell.start_execution();
                cx.notify();
            });
        }

        let request = ExecuteRequest {
            code,
            ..Default::default()
        };
        let message: JupyterMessage = request.into();
        let msg_id = message.header.msg_id.clone();

        self.execution_requests.insert(msg_id, cell_id.clone());

        if let Kernel::RunningKernel(kernel) = &mut self.kernel {
            kernel.request_tx().try_send(message).ok();
        }

        // Outputs and execution_count are persisted in the .ipynb file, so an
        // execution always dirties the notebook.
        self.note_edit(cx);
    }

    fn has_outputs(&self, _window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.cell_map.values().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).has_outputs()
            } else {
                false
            }
        })
    }

    fn clear_outputs(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let mut cleared_any = false;
        for cell in self.cell_map.values() {
            if let Cell::Code(code_cell) = cell {
                code_cell.update(cx, |cell, cx| {
                    if cell.has_outputs() {
                        cell.clear_outputs();
                        cleared_any = true;
                        cx.notify();
                    }
                });
            }
        }
        if cleared_any {
            self.note_edit(cx);
        } else {
            cx.notify();
        }
    }

    fn run_cells(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        for cell_id in self.cell_order.clone() {
            self.execute_cell(cell_id, cx);
        }
    }

    fn run_current_cell(&mut self, _: &Run, window: &mut Window, cx: &mut Context<Self>) {
        let Some(cell_id) = self.cell_order.get(self.selected_cell_index).cloned() else {
            return;
        };
        let Some(cell) = self.cell_map.get(&cell_id) else {
            return;
        };
        match cell {
            Cell::Code(_) => {
                self.execute_cell(cell_id, cx);
            }
            Cell::Markdown(markdown_cell) => {
                // for markdown, finish editing and move to next cell
                let is_editing = markdown_cell.read(cx).is_editing();
                if is_editing {
                    markdown_cell.update(cx, |cell, cx| {
                        cell.run(cx);
                    });
                    self.enter_command_mode(window, cx);
                }
            }
            Cell::Raw(_) => {}
        }
    }

    fn run_and_advance(&mut self, _: &RunAndAdvance, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(cell_id) = self.cell_order.get(self.selected_cell_index).cloned() {
            if let Some(cell) = self.cell_map.get(&cell_id) {
                match cell {
                    Cell::Code(_) => {
                        self.execute_cell(cell_id, cx);
                    }
                    Cell::Markdown(markdown_cell) => {
                        if markdown_cell.read(cx).is_editing() {
                            markdown_cell.update(cx, |cell, cx| {
                                cell.run(cx);
                            });
                        }
                    }
                    Cell::Raw(_) => {}
                }
            }
        }

        let is_last_cell = self.selected_cell_index == self.cell_count().saturating_sub(1);
        if is_last_cell {
            self.add_code_block(window, cx);
            self.enter_command_mode(window, cx);
        } else {
            self.advance_in_command_mode(window, cx);
        }
    }

    fn enter_edit_mode(&mut self, _: &EnterEditMode, window: &mut Window, cx: &mut Context<Self>) {
        self.notebook_mode = NotebookMode::Edit;
        if let Some(cell_id) = self.cell_order.get(self.selected_cell_index) {
            if let Some(cell) = self.cell_map.get(cell_id) {
                match cell {
                    Cell::Code(code_cell) => {
                        let editor = code_cell.read(cx).editor().clone();
                        window.focus(&editor.focus_handle(cx), cx);
                    }
                    Cell::Markdown(markdown_cell) => {
                        markdown_cell.update(cx, |cell, cx| {
                            cell.set_editing(true);
                            cx.notify();
                        });
                        let editor = markdown_cell.read(cx).editor().clone();
                        window.focus(&editor.focus_handle(cx), cx);
                    }
                    Cell::Raw(_) => {}
                }
            }
        }
        cx.notify();
    }

    fn enter_command_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.notebook_mode = NotebookMode::Command;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn handle_enter_command_mode(
        &mut self,
        _: &EnterCommandMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.enter_command_mode(window, cx);
    }

    /// Advances to the next cell while staying in command mode (used by RunAndAdvance and shift-enter).
    fn advance_in_command_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.cell_count();
        if count == 0 {
            return;
        }
        if self.selected_cell_index < count - 1 {
            self.selected_cell_index += 1;
            self.cell_list
                .scroll_to_reveal_item(self.selected_cell_index);
        }
        self.notebook_mode = NotebookMode::Command;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn open_notebook(&mut self, _: &OpenNotebook, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn move_cell_up(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_cell_index > 0 {
            self.cell_order
                .swap(self.selected_cell_index, self.selected_cell_index - 1);
            self.selected_cell_index -= 1;
            self.note_edit(cx);
        }
    }

    fn move_cell_down(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.cell_order.is_empty() && self.selected_cell_index < self.cell_order.len() - 1 {
            self.cell_order
                .swap(self.selected_cell_index, self.selected_cell_index + 1);
            self.selected_cell_index += 1;
            self.note_edit(cx);
        }
    }

    /// Delete the currently selected cell. Refuses to delete the last cell so
    /// the notebook always has at least one cell to focus.
    fn delete_cell(&mut self, _: &DeleteCell, _window: &mut Window, cx: &mut Context<Self>) {
        if self.cell_count() <= 1 {
            return;
        }
        let cell_id = self.cell_order.remove(self.selected_cell_index);
        self.cell_map.remove(&cell_id);
        self.cell_list
            .splice(self.selected_cell_index..self.selected_cell_index + 1, 0);
        if self.selected_cell_index >= self.cell_count() {
            self.selected_cell_index = self.cell_count().saturating_sub(1);
        }
        self.cell_list
            .scroll_to_reveal_item(self.selected_cell_index);
        self.note_edit(cx);
    }

    fn insert_cell_at_current_position(&mut self, cell_id: CellId, cell: Cell) {
        let insert_index = if self.cell_order.is_empty() {
            0
        } else {
            self.selected_cell_index + 1
        };
        self.cell_order.insert(insert_index, cell_id.clone());
        self.cell_map.insert(cell_id, cell);
        self.selected_cell_index = insert_index;
        self.cell_list.splice(insert_index..insert_index, 1);
        self.cell_list.scroll_to_reveal_item(insert_index);
    }

    fn add_markdown_block(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let new_cell_id: CellId = Uuid::new_v4().into();
        let languages = self.languages.clone();
        let metadata: nbformat::v4::CellMetadata =
            serde_json::from_str("{}").expect("empty object should parse");

        let markdown_cell = cx.new(|cx| {
            super::MarkdownCell::new(
                new_cell_id.clone(),
                metadata,
                String::new(),
                languages,
                window,
                cx,
            )
        });

        Self::subscribe_to_markdown_cell(&markdown_cell, new_cell_id.clone(), cx);

        self.insert_cell_at_current_position(new_cell_id, Cell::Markdown(markdown_cell.clone()));
        markdown_cell.update(cx, |cell, cx| {
            cell.set_editing(true);
            cx.notify();
        });
        let editor = markdown_cell.read(cx).editor().clone();
        window.focus(&editor.focus_handle(cx), cx);
        self.notebook_mode = NotebookMode::Edit;
        self.note_edit(cx);
    }

    fn add_code_block(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let new_cell_id: CellId = Uuid::new_v4().into();
        let notebook_language = self.notebook_language.clone();
        let metadata: nbformat::v4::CellMetadata =
            serde_json::from_str("{}").expect("empty object should parse");

        let code_cell = cx.new(|cx| {
            super::CodeCell::new(
                new_cell_id.clone(),
                metadata,
                String::new(),
                notebook_language,
                window,
                cx,
            )
        });

        Self::subscribe_to_code_cell(&code_cell, new_cell_id.clone(), cx);

        self.insert_cell_at_current_position(new_cell_id, Cell::Code(code_cell.clone()));
        let editor = code_cell.read(cx).editor().clone();
        window.focus(&editor.focus_handle(cx), cx);
        self.notebook_mode = NotebookMode::Edit;
        self.note_edit(cx);
    }

    fn cell_count(&self) -> usize {
        self.cell_map.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_cell_index
    }

    fn select_cell_by_id(&mut self, cell_id: &CellId, cx: &mut Context<Self>) {
        if let Some(index) = self.cell_order.iter().position(|id| id == cell_id) {
            self.selected_cell_index = index;
            self.notebook_mode = NotebookMode::Edit;
            cx.notify();
        }
    }

    pub fn set_selected_index(
        &mut self,
        index: usize,
        jump_to_index: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // let previous_index = self.selected_cell_index;
        self.selected_cell_index = index;
        let current_index = self.selected_cell_index;

        // in the future we may have some `on_cell_change` event that we want to fire here

        if jump_to_index {
            self.jump_to_cell(current_index, window, cx);
        }
    }

    pub fn select_next(
        &mut self,
        _: &menu::SelectNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == count - 1 {
                count - 1
            } else {
                index + 1
            };
            self.set_selected_index(ix, true, window, cx);
            cx.notify();
        }
    }

    pub fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == 0 { 0 } else { index - 1 };
            self.set_selected_index(ix, true, window, cx);
            cx.notify();
        }
    }

    pub fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(0, true, window, cx);
            cx.notify();
        }
    }

    pub fn select_last(
        &mut self,
        _: &menu::SelectLast,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(count - 1, true, window, cx);
            cx.notify();
        }
    }

    fn jump_to_cell(&mut self, index: usize, _window: &mut Window, _cx: &mut Context<Self>) {
        self.cell_list.scroll_to_reveal_item(index);
    }

    fn button_group(_window: &mut Window, cx: &mut Context<Self>) -> Div {
        v_flex()
            .gap(DynamicSpacing::Base04.rems(cx))
            .items_center()
            .w(px(CONTROL_SIZE + 4.0))
            .overflow_hidden()
            .rounded(px(5.))
            .bg(cx.theme().colors().title_bar_background)
            .p_px()
            .border_1()
            .border_color(cx.theme().colors().border)
    }

    fn render_notebook_control(
        id: impl Into<SharedString>,
        icon: IconName,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> IconButton {
        let id: ElementId = ElementId::Name(id.into());
        IconButton::new(id, icon).width(px(CONTROL_SIZE))
    }

    fn render_notebook_controls(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_outputs = self.has_outputs(window, cx);

        v_flex()
            .max_w(px(CONTROL_SIZE + 4.0))
            .items_center()
            .gap(DynamicSpacing::Base16.rems(cx))
            .justify_between()
            .flex_none()
            .h_full()
            .py(DynamicSpacing::Base12.px(cx))
            .child(
                v_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "run-all-cells",
                                    IconName::PlayFilled,
                                    window,
                                    cx,
                                )
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action("Execute all cells", &RunAll, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(RunAll), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "clear-all-outputs",
                                    IconName::ListX,
                                    window,
                                    cx,
                                )
                                .disabled(!has_outputs)
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action("Clear all outputs", &ClearOutputs, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(ClearOutputs), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-up",
                                    IconName::ArrowUp,
                                    window,
                                    cx,
                                )
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action("Move cell up", &MoveCellUp, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(MoveCellUp), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-down",
                                    IconName::ArrowDown,
                                    window,
                                    cx,
                                )
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action("Move cell down", &MoveCellDown, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(MoveCellDown), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "new-markdown-cell",
                                    IconName::Plus,
                                    window,
                                    cx,
                                )
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action("Add markdown block", &AddMarkdownBlock, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddMarkdownBlock), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "new-code-cell",
                                    IconName::Code,
                                    window,
                                    cx,
                                )
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action("Add code block", &AddCodeBlock, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddCodeBlock), cx);
                                }),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .items_center()
                    .child(
                        Self::render_notebook_control("more-menu", IconName::Ellipsis, window, cx)
                            .tooltip(move |window, cx| (Tooltip::text("More options"))(window, cx)),
                    )
                    .child(Self::button_group(window, cx).child({
                        let kernel_status = self.kernel.status();
                        let (icon, icon_color) = match &kernel_status {
                            KernelStatus::Idle => (IconName::ReplNeutral, Color::Success),
                            KernelStatus::Busy => (IconName::ReplNeutral, Color::Warning),
                            KernelStatus::Starting => (IconName::ReplNeutral, Color::Muted),
                            KernelStatus::Error => (IconName::ReplNeutral, Color::Error),
                            KernelStatus::ShuttingDown => (IconName::ReplNeutral, Color::Muted),
                            KernelStatus::Shutdown => (IconName::ReplNeutral, Color::Disabled),
                            KernelStatus::Restarting => (IconName::ReplNeutral, Color::Warning),
                        };
                        let kernel_name = self
                            .kernel_specification
                            .as_ref()
                            .map(|spec| spec.name().to_string())
                            .unwrap_or_else(|| "Select Kernel".to_string());
                        IconButton::new("repl", icon)
                            .icon_color(icon_color)
                            .tooltip(move |window, cx| {
                                Tooltip::text(format!(
                                    "{} ({}). Click to change kernel.",
                                    kernel_name,
                                    kernel_status.to_string()
                                ))(window, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.kernel_picker_handle.toggle(window, cx);
                            }))
                    })),
            )
    }

    fn render_kernel_status_bar(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let kernel_status = self.kernel.status();
        let kernel_name = self
            .kernel_specification
            .as_ref()
            .map(|spec| spec.name().to_string())
            .unwrap_or_else(|| "Select Kernel".to_string());

        let (status_icon, status_color) = match &kernel_status {
            KernelStatus::Idle => (IconName::Circle, Color::Success),
            KernelStatus::Busy => (IconName::ArrowCircle, Color::Warning),
            KernelStatus::Starting => (IconName::ArrowCircle, Color::Muted),
            KernelStatus::Error => (IconName::XCircle, Color::Error),
            KernelStatus::ShuttingDown => (IconName::ArrowCircle, Color::Muted),
            KernelStatus::Shutdown => (IconName::Circle, Color::Muted),
            KernelStatus::Restarting => (IconName::ArrowCircle, Color::Warning),
        };

        let worktree_id = self.worktree_id;
        let kernel_picker_handle = self.kernel_picker_handle.clone();
        let view = cx.entity().downgrade();

        h_flex()
            .w_full()
            .px_3()
            .py_1()
            .gap_2()
            .items_center()
            .justify_between()
            .bg(cx.theme().colors().status_bar_background)
            .child(
                KernelSelector::new(
                    Box::new(move |spec: KernelSpecification, window, cx| {
                        if let Some(view) = view.upgrade() {
                            view.update(cx, |this, cx| {
                                this.change_kernel(spec, window, cx);
                            });
                        }
                    }),
                    worktree_id,
                    Button::new("kernel-selector", kernel_name.clone())
                        .label_size(LabelSize::Small)
                        .start_icon(
                            Icon::new(status_icon)
                                .size(IconSize::Small)
                                .color(status_color),
                        ),
                    Tooltip::text(format!(
                        "Kernel: {} ({}). Click to change.",
                        kernel_name,
                        kernel_status.to_string()
                    )),
                )
                .with_handle(kernel_picker_handle),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("restart-kernel", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(|_window, cx| {
                                Tooltip::for_action("Restart Kernel", &RestartKernel, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.restart_kernel(&RestartKernel, window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("interrupt-kernel", IconName::Stop)
                            .icon_size(IconSize::Small)
                            .disabled(!matches!(kernel_status, KernelStatus::Busy))
                            .tooltip(|_window, cx| {
                                Tooltip::for_action("Interrupt Kernel", &InterruptKernel, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.interrupt_kernel(&InterruptKernel, window, cx);
                            })),
                    ),
            )
    }

    fn cell_list(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        list(self.cell_list.clone(), move |index, window, cx| {
            view.update(cx, |this, cx| {
                let cell_id = &this.cell_order[index];
                let cell = this.cell_map.get(cell_id).unwrap();
                this.render_cell(index, cell, window, cx).into_any_element()
            })
        })
        .size_full()
    }

    fn cell_position(&self, index: usize) -> CellPosition {
        match index {
            0 => CellPosition::First,
            index if index == self.cell_count() - 1 => CellPosition::Last,
            _ => CellPosition::Middle,
        }
    }

    fn render_cell(
        &self,
        index: usize,
        cell: &Cell,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let cell_position = self.cell_position(index);

        let is_selected = index == self.selected_cell_index;

        match cell {
            Cell::Code(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
            Cell::Markdown(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
            Cell::Raw(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
        }
    }
}

impl Render for NotebookEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("NotebookEditor");
        key_context.set(
            "notebook_mode",
            match self.notebook_mode {
                NotebookMode::Command => "command",
                NotebookMode::Edit => "edit",
            },
        );

        v_flex()
            .size_full()
            .key_context(key_context)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &OpenNotebook, window, cx| {
                this.open_notebook(&OpenNotebook, window, cx)
            }))
            .on_action(
                cx.listener(|this, _: &ClearOutputs, window, cx| this.clear_outputs(window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &Run, window, cx| this.run_current_cell(&Run, window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.run_and_advance(action, window, cx)),
            )
            .on_action(cx.listener(|this, _: &RunAll, window, cx| this.run_cells(window, cx)))
            .on_action(
                cx.listener(|this, _: &MoveCellUp, window, cx| this.move_cell_up(window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &MoveCellDown, window, cx| this.move_cell_down(window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.delete_cell(action, window, cx)),
            )
            .on_action(cx.listener(|this, _: &AddMarkdownBlock, window, cx| {
                this.add_markdown_block(window, cx)
            }))
            .on_action(
                cx.listener(|this, _: &AddCodeBlock, window, cx| this.add_code_block(window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.enter_edit_mode(action, window, cx)),
            )
            .on_action(cx.listener(|this, action, window, cx| {
                this.handle_enter_command_mode(action, window, cx)
            }))
            .on_action(cx.listener(|this, action, window, cx| this.select_next(action, window, cx)))
            .on_action(
                cx.listener(|this, action, window, cx| this.select_previous(action, window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.select_first(action, window, cx)),
            )
            .on_action(cx.listener(|this, action, window, cx| this.select_last(action, window, cx)))
            .on_action(cx.listener(|this, _: &MoveUp, window, cx| {
                this.select_previous(&menu::SelectPrevious, window, cx);
                if let Some(cell_id) = this.cell_order.get(this.selected_cell_index) {
                    if let Some(cell) = this.cell_map.get(cell_id) {
                        match cell {
                            Cell::Code(cell) => {
                                let editor = cell.read(cx).editor().clone();
                                editor.update(cx, |editor, cx| {
                                    editor.move_to_end(&Default::default(), window, cx);
                                });
                                editor.focus_handle(cx).focus(window, cx);
                            }
                            Cell::Markdown(cell) => {
                                cell.update(cx, |cell, cx| {
                                    cell.set_editing(true);
                                    cx.notify();
                                });
                                let editor = cell.read(cx).editor().clone();
                                editor.update(cx, |editor, cx| {
                                    editor.move_to_end(&Default::default(), window, cx);
                                });
                                editor.focus_handle(cx).focus(window, cx);
                            }
                            _ => {}
                        }
                    }
                }
            }))
            .on_action(cx.listener(|this, _: &MoveDown, window, cx| {
                this.select_next(&menu::SelectNext, window, cx);
                if let Some(cell_id) = this.cell_order.get(this.selected_cell_index) {
                    if let Some(cell) = this.cell_map.get(cell_id) {
                        match cell {
                            Cell::Code(cell) => {
                                let editor = cell.read(cx).editor().clone();
                                editor.update(cx, |editor, cx| {
                                    editor.move_to_beginning(&Default::default(), window, cx);
                                });
                                editor.focus_handle(cx).focus(window, cx);
                            }
                            Cell::Markdown(cell) => {
                                cell.update(cx, |cell, cx| {
                                    cell.set_editing(true);
                                    cx.notify();
                                });
                                let editor = cell.read(cx).editor().clone();
                                editor.update(cx, |editor, cx| {
                                    editor.move_to_beginning(&Default::default(), window, cx);
                                });
                                editor.focus_handle(cx).focus(window, cx);
                            }
                            _ => {}
                        }
                    }
                }
            }))
            .on_action(cx.listener(|this, _: &NotebookMoveDown, window, cx| {
                let Some(cell_id) = this.cell_order.get(this.selected_cell_index) else {
                    return;
                };
                let Some(cell) = this.cell_map.get(cell_id) else {
                    return;
                };

                let editor = match cell {
                    Cell::Code(cell) => cell.read(cx).editor().clone(),
                    Cell::Markdown(cell) => cell.read(cx).editor().clone(),
                    _ => return,
                };

                let is_at_last_line = editor.update(cx, |editor, cx| {
                    let display_snapshot = editor.display_snapshot(cx);
                    let selections = editor.selections.all_display(&display_snapshot);
                    if let Some(selection) = selections.last() {
                        let head = selection.head();
                        let cursor_row = head.row();
                        let max_row = display_snapshot.max_point().row();

                        cursor_row >= max_row
                    } else {
                        false
                    }
                });

                if is_at_last_line {
                    this.select_next(&menu::SelectNext, window, cx);
                    if let Some(cell_id) = this.cell_order.get(this.selected_cell_index) {
                        if let Some(cell) = this.cell_map.get(cell_id) {
                            match cell {
                                Cell::Code(cell) => {
                                    let editor = cell.read(cx).editor().clone();
                                    editor.update(cx, |editor, cx| {
                                        editor.move_to_beginning(&Default::default(), window, cx);
                                    });
                                    editor.focus_handle(cx).focus(window, cx);
                                }
                                Cell::Markdown(cell) => {
                                    cell.update(cx, |cell, cx| {
                                        cell.set_editing(true);
                                        cx.notify();
                                    });
                                    let editor = cell.read(cx).editor().clone();
                                    editor.update(cx, |editor, cx| {
                                        editor.move_to_beginning(&Default::default(), window, cx);
                                    });
                                    editor.focus_handle(cx).focus(window, cx);
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    editor.update(cx, |editor, cx| {
                        editor.move_down(&Default::default(), window, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &NotebookMoveUp, window, cx| {
                let Some(cell_id) = this.cell_order.get(this.selected_cell_index) else {
                    return;
                };
                let Some(cell) = this.cell_map.get(cell_id) else {
                    return;
                };

                let editor = match cell {
                    Cell::Code(cell) => cell.read(cx).editor().clone(),
                    Cell::Markdown(cell) => cell.read(cx).editor().clone(),
                    _ => return,
                };

                let is_at_first_line = editor.update(cx, |editor, cx| {
                    let display_snapshot = editor.display_snapshot(cx);
                    let selections = editor.selections.all_display(&display_snapshot);
                    if let Some(selection) = selections.first() {
                        let head = selection.head();
                        let cursor_row = head.row();

                        cursor_row.0 == 0
                    } else {
                        false
                    }
                });

                if is_at_first_line {
                    this.select_previous(&menu::SelectPrevious, window, cx);
                    if let Some(cell_id) = this.cell_order.get(this.selected_cell_index) {
                        if let Some(cell) = this.cell_map.get(cell_id) {
                            match cell {
                                Cell::Code(cell) => {
                                    let editor = cell.read(cx).editor().clone();
                                    editor.update(cx, |editor, cx| {
                                        editor.move_to_end(&Default::default(), window, cx);
                                    });
                                    editor.focus_handle(cx).focus(window, cx);
                                }
                                Cell::Markdown(cell) => {
                                    cell.update(cx, |cell, cx| {
                                        cell.set_editing(true);
                                        cx.notify();
                                    });
                                    let editor = cell.read(cx).editor().clone();
                                    editor.update(cx, |editor, cx| {
                                        editor.move_to_end(&Default::default(), window, cx);
                                    });
                                    editor.focus_handle(cx).focus(window, cx);
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    editor.update(cx, |editor, cx| {
                        editor.move_up(&Default::default(), window, cx);
                    });
                }
            }))
            .on_action(
                cx.listener(|this, action, window, cx| this.restart_kernel(action, window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.interrupt_kernel(action, window, cx)),
            )
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .h_full()
                    .gap_2()
                    .child(div().flex_1().h_full().child(self.cell_list(window, cx)))
                    .child(self.render_notebook_controls(window, cx)),
            )
            .child(self.render_kernel_status_bar(window, cx))
    }
}

impl Focusable for NotebookEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// Intended to be a NotebookBuffer
pub struct NotebookItem {
    path: PathBuf,
    project_path: ProjectPath,
    languages: Arc<LanguageRegistry>,
    // Raw notebook data
    notebook: nbformat::v4::Notebook,
    // Store our version of the notebook in memory (cell_order, cell_map)
    id: ProjectEntryId,
    /// Mirrors `NotebookEditor::is_dirty`. The `project::ProjectItem::is_dirty`
    /// trait method takes `&self` only, so the editor pushes its dirty state here
    /// via `note_edit` / `mark_as_saved` instead of reading it back through `cx`.
    pub(super) dirty: bool,
    /// The underlying file buffer. We hold on to it so `NotebookEditor` can
    /// subscribe to `BufferEvent::Reloaded` and reload the cells in response to
    /// external edits (e.g. `git checkout`, save from another editor).
    pub(super) buffer: Entity<language::Buffer>,
}

impl project::ProjectItem for NotebookItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
        let path = path.clone();
        let project = project.clone();
        let languages = project.read(cx).languages().clone();

        if path.path.extension().unwrap_or_default() == "ipynb" {
            Some(cx.spawn(async move |cx| {
                let abs_path = project
                    .read_with(cx, |project, cx| project.absolute_path(&path, cx))
                    .with_context(|| format!("finding the absolute path of {path:?}"))?;

                // todo: watch for changes to the file
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(path.clone(), cx))
                    .await?;
                let file_content = buffer.read_with(cx, |buffer, _| buffer.text());

                let notebook = if file_content.trim().is_empty() {
                    nbformat::v4::Notebook {
                        nbformat: 4,
                        nbformat_minor: 5,
                        cells: vec![],
                        metadata: serde_json::from_str("{}").unwrap(),
                    }
                } else {
                    let notebook = match nbformat::parse_notebook(&file_content) {
                        Ok(nb) => nb,
                        Err(_) => {
                            // Pre-process to ensure IDs exist
                            let mut json: serde_json::Value = serde_json::from_str(&file_content)?;
                            if let Some(cells) =
                                json.get_mut("cells").and_then(|c| c.as_array_mut())
                            {
                                for cell in cells {
                                    if cell.get("id").is_none() {
                                        cell["id"] =
                                            serde_json::Value::String(Uuid::new_v4().to_string());
                                    }
                                }
                            }
                            let file_content = serde_json::to_string(&json)?;
                            nbformat::parse_notebook(&file_content)?
                        }
                    };

                    match notebook {
                        nbformat::Notebook::V4(notebook) => notebook,
                        // 4.1 - 4.4 are converted to 4.5
                        nbformat::Notebook::Legacy(legacy_notebook) => {
                            // TODO: Decide if we want to mutate the notebook by including Cell IDs
                            // and any other conversions

                            nbformat::upgrade_legacy_notebook(legacy_notebook)?
                        }
                        nbformat::Notebook::V3(v3_notebook) => {
                            nbformat::upgrade_v3_notebook(v3_notebook)?
                        }
                    }
                };

                let id = project
                    .update(cx, |project, cx| {
                        project.entry_for_path(&path, cx).map(|entry| entry.id)
                    })
                    .context("Entry not found")?;

                Ok(cx.new(|_| NotebookItem {
                    path: abs_path,
                    project_path: path,
                    languages,
                    notebook,
                    id,
                    dirty: false,
                    buffer,
                }))
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        Some(self.id)
    }

    fn project_path(&self, _: &App) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl NotebookItem {
    pub fn language_name(&self) -> Option<String> {
        self.notebook
            .metadata
            .language_info
            .as_ref()
            .map(|l| l.name.clone())
            .or(self
                .notebook
                .metadata
                .kernelspec
                .as_ref()
                .and_then(|spec| spec.language.clone()))
    }

    pub fn notebook_language(&self) -> impl Future<Output = Option<Arc<Language>>> + use<> {
        let language_name = self.language_name();
        let languages = self.languages.clone();

        async move {
            if let Some(language_name) = language_name {
                languages.language_for_name(&language_name).await.ok()
            } else {
                None
            }
        }
    }
}

impl EventEmitter<()> for NotebookItem {}

impl EventEmitter<NotebookEditorEvent> for NotebookEditor {}

impl Item for NotebookEditor {
    type Event = NotebookEditorEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        match event {
            NotebookEditorEvent::Edit => {
                f(ItemEvent::Edit);
                f(ItemEvent::UpdateTab);
            }
            NotebookEditorEvent::TitleChanged => {
                f(ItemEvent::UpdateTab);
            }
        }
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| {
            Self::new(self.project.clone(), self.notebook_item.clone(), window, cx)
        })))
    }

    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        f(self.notebook_item.entity_id(), self.notebook_item.read(cx))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.notebook_item
            .read(cx)
            .project_path
            .path
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_default()
            .into()
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or(0), cx))
            .single_line()
            .color(params.text_color())
            .when(params.preview, |this| this.italic())
            .into_any_element()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(IconName::Book.into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Notebook Editor")
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    // TODO
    fn pixel_position_of_cursor(&self, _: &App) -> Option<Point<Pixels>> {
        None
    }

    // TODO
    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn set_nav_history(
        &mut self,
        _: workspace::ItemNavHistory,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        // TODO
    }

    fn can_save(&self, _cx: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        _options: SaveOptions,
        project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let notebook = self.to_notebook(cx);
        let path = self.notebook_item.read(cx).path.clone();
        let fs = project.read(cx).fs().clone();

        self.mark_as_saved(cx);

        cx.spawn(async move |_this, _cx| {
            let json =
                serde_json::to_string_pretty(&notebook).context("Failed to serialize notebook")?;
            fs.atomic_write(path, json).await?;
            Ok(())
        })
    }

    fn save_as(
        &mut self,
        project: Entity<Project>,
        path: ProjectPath,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let notebook = self.to_notebook(cx);
        let fs = project.read(cx).fs().clone();

        let abs_path = project.read(cx).absolute_path(&path, cx);

        self.mark_as_saved(cx);

        cx.spawn(async move |_this, _cx| {
            let abs_path = abs_path.context("Failed to get absolute path")?;
            let json =
                serde_json::to_string_pretty(&notebook).context("Failed to serialize notebook")?;
            fs.atomic_write(abs_path, json).await?;
            Ok(())
        })
    }

    fn reload(
        &mut self,
        _project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let project_path = self.notebook_item.read(cx).project_path.clone();
        let languages = self.languages.clone();
        let notebook_language = self.notebook_language.clone();

        cx.spawn_in(window, async move |this, cx| {
            let buffer = this
                .update(cx, |this, cx| {
                    this.project
                        .update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            let file_content = buffer.read_with(cx, |buffer, _| buffer.text());

            let mut json: serde_json::Value = serde_json::from_str(&file_content)?;
            if let Some(cells) = json.get_mut("cells").and_then(|c| c.as_array_mut()) {
                for cell in cells {
                    if cell.get("id").is_none() {
                        cell["id"] = serde_json::Value::String(Uuid::new_v4().to_string());
                    }
                }
            }
            let file_content = serde_json::to_string(&json)?;

            let notebook = nbformat::parse_notebook(&file_content);
            let notebook = match notebook {
                Ok(nbformat::Notebook::V4(notebook)) => notebook,
                Ok(nbformat::Notebook::Legacy(legacy_notebook)) => {
                    nbformat::upgrade_legacy_notebook(legacy_notebook)?
                }
                Ok(nbformat::Notebook::V3(v3_notebook)) => {
                    nbformat::upgrade_v3_notebook(v3_notebook)?
                }
                Err(e) => {
                    anyhow::bail!("Failed to parse notebook: {:?}", e);
                }
            };

            this.update_in(cx, |this, window, cx| {
                let mut cell_order = vec![];
                let mut cell_map = HashMap::default();

                for cell in notebook.cells.iter() {
                    let cell_id = cell.id();
                    cell_order.push(cell_id.clone());
                    let cell_entity =
                        Cell::load(cell, &languages, notebook_language.clone(), window, cx);
                    cell_map.insert(cell_id.clone(), cell_entity);
                }

                this.cell_order = cell_order.clone();
                this.original_cell_order = cell_order;
                this.cell_map = cell_map;
                this.cell_list =
                    ListState::new(this.cell_order.len(), gpui::ListAlignment::Top, px(1000.));
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.has_structural_changes() || self.has_content_changes(cx)
    }
}

impl ProjectItem for NotebookEditor {
    type Item = NotebookItem;

    fn for_project_item(
        project: Entity<Project>,
        _pane: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(project, item, window, cx)
    }
}

/// Whether a kernel message updates state that gets serialized into the .ipynb
/// file (outputs or execution_count). Used to keep the notebook's dirty state
/// in sync with kernel responses.
fn mutates_persisted_cell_state(content: &JupyterMessageContent) -> bool {
    matches!(
        content,
        JupyterMessageContent::StreamContent(_)
            | JupyterMessageContent::DisplayData(_)
            | JupyterMessageContent::ExecuteResult(_)
            | JupyterMessageContent::ExecuteInput(_)
            | JupyterMessageContent::ErrorOutput(_)
    )
}

impl KernelSession for NotebookEditor {
    fn route(&mut self, message: &JupyterMessage, window: &mut Window, cx: &mut Context<Self>) {
        // Handle kernel status updates (these are broadcast to all)
        if let JupyterMessageContent::Status(status) = &message.content {
            self.kernel.set_execution_state(&status.execution_state);
            cx.notify();
        }

        if let JupyterMessageContent::KernelInfoReply(reply) = &message.content {
            self.kernel.set_kernel_info(reply);

            if let Ok(language_info) = serde_json::from_value::<nbformat::v4::LanguageInfo>(
                serde_json::to_value(&reply.language_info).unwrap(),
            ) {
                self.notebook_item.update(cx, |item, cx| {
                    item.notebook.metadata.language_info = Some(language_info);
                    cx.emit(());
                });
            }
            cx.notify();
        }

        // Handle cell-specific messages
        if let Some(parent_header) = &message.parent_header {
            if let Some(cell_id) = self.execution_requests.get(&parent_header.msg_id) {
                if let Some(Cell::Code(cell)) = self.cell_map.get(cell_id) {
                    cell.update(cx, |cell, cx| {
                        cell.handle_message(message, window, cx);
                    });
                    if mutates_persisted_cell_state(&message.content) {
                        self.note_edit(cx);
                    }
                }
            }
        }
    }

    fn kernel_errored(&mut self, error_message: String, cx: &mut Context<Self>) {
        self.kernel = Kernel::ErroredLaunch(error_message);
        cx.notify();
    }
}

impl SerializableItem for NotebookEditor {
    fn serialized_item_kind() -> &'static str {
        "NotebookEditor"
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "notebook_editors",
            &persistence::NotebookEditorDb::global(cx),
            cx,
        )
    }

    fn deserialize(
        project: Entity<Project>,
        _workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        use project::ProjectItem as _;

        let db = persistence::NotebookEditorDb::global(cx);
        window.spawn(cx, async move |cx| {
            let abs_path = db
                .get_notebook_path(item_id, workspace_id)?
                .context("No notebook path found")?;

            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(abs_path.clone(), false, cx)
                })
                .await
                .context("Worktree not found")?;
            let worktree_id = worktree.read_with(cx, |worktree, _cx| worktree.id());
            let project_path = ProjectPath {
                worktree_id,
                path: relative_path,
            };

            let open_task = cx.update(|_window, cx| {
                NotebookItem::try_open(&project, &project_path, cx)
            })?;
            let notebook_item = open_task
                .context("Notebook items must claim .ipynb files")?
                .await?;

            cx.update(|window, cx| {
                cx.new(|cx| NotebookEditor::new(project, notebook_item, window, cx))
            })
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let abs_path = self.notebook_item.read(cx).path.clone();
        let db = persistence::NotebookEditorDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_notebook_path(item_id, workspace_id, abs_path).await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct NotebookEditorDb(ThreadSafeConnection);

    impl Domain for NotebookEditorDb {
        const NAME: &str = stringify!(NotebookEditorDb);

        const MIGRATIONS: &[&str] = &[sql!(
            CREATE TABLE notebook_editors (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,

                notebook_path BLOB,

                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
    }

    db::static_connection!(NotebookEditorDb, [WorkspaceDb]);

    impl NotebookEditorDb {
        query! {
            pub async fn save_notebook_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                notebook_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO notebook_editors(item_id, workspace_id, notebook_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_notebook_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT notebook_path
                FROM notebook_editors
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
