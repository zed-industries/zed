use std::{collections::VecDeque, future::Future, sync::Arc, time::Duration};

use super::{
    Cell, CellEvent, CellPosition, CodeCell, MarkdownCellEvent, RenderableCell, RunnableCell,
    document::{CellChange, CellEdit, CellFields, ParsedNotebook, Projection, plan_cell_edit},
};
use crate::kernels::{
    Kernel, KernelSession, KernelSpecification, LocalKernelSpecification, NativeRunningKernel,
    RemoteRunningKernel, SshRunningKernel, WslRunningKernel,
};
use crate::notebook::MovementDirection;
use anyhow::{Context as _, Result};
use collections::HashMap;
use editor::{EditorSettings, EditorSettingsScrollbarProxy};
use feature_flags::{FeatureFlagAppExt as _, NotebookFeatureFlag};
use futures::{FutureExt, future::Shared};
use gpui::{
    App, Entity, FocusHandle, Focusable, KeyContext, ListState, MouseButton, MouseDownEvent, Task,
    list, prelude::*,
};
use jupyter_protocol::{ExecuteRequest, JupyterMessage, JupyterMessageContent};
use language::{Language, LanguageRegistry};
use nbformat::v4::CellId;
use project::{
    Project, ProjectEntryId, ProjectPath,
    trusted_worktrees::{TrustedWorktrees, TrustedWorktreesEvent},
};
use settings::Settings as _;
use text::ToOffset;
use ui::{Scrollbars, Tooltip, WithScrollbar, prelude::*};
use workspace::item::SaveOptions;
use zed_actions::editor::{MoveDown, MoveUp};
use zed_actions::notebook::{
    AddCodeBlock, AddMarkdownBlock, ClearOutputs, DeleteCell, EnterCommandMode, EnterEditMode,
    InterruptKernel, MoveCellDown, MoveCellUp, NotebookMoveDown, NotebookMoveUp, RestartKernel,
    Run, RunAll, RunAndAdvance,
};

/// Whether the notebook is in command mode (navigating cells) or edit mode (editing a cell).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NotebookMode {
    Command,
    Edit,
}

#[derive(PartialEq, Eq)]
enum SelectionMode {
    SelectOnly,
    SelectAndMove,
}

pub(crate) const MEDIUM_SPACING_SIZE: f32 = 12.0;
pub(crate) const GUTTER_WIDTH: f32 = 19.0;
pub(crate) const CODE_BLOCK_INSET: f32 = MEDIUM_SPACING_SIZE;

const NOTEBOOK_EXTENSION: &str = "ipynb";

pub fn init(cx: &mut App) {
    super::preview::init(cx);
}

pub struct NotebookEditor {
    project: Entity<Project>,
    worktree_id: project::WorktreeId,
    focus_handle: FocusHandle,
    notebook_item: Entity<NotebookItem>,
    cell_list: ListState,
    mouse_autoscroll: Option<Task<()>>,
    notebook_mode: NotebookMode,
    selected_cell_index: usize,
    structure_error: Option<String>,
    structure_undo: Vec<StructureStep>,
    structure_redo: Vec<StructureStep>,
    cell_order: Vec<CellId>,
    source_version: clock::Global,
    sync_errors: HashMap<usize, String>,
    cell_sources: HashMap<CellId, CellSourceHistory>,
    notebook_language: Shared<Task<Option<Arc<Language>>>>,
    cell_map: HashMap<CellId, Cell>,
    kernel: Kernel,
    kernel_specification: Option<KernelSpecification>,
    kernel_session: Option<Entity<NotebookKernelSession>>,
    kernel_task: Option<Shared<Task<()>>>,
    preparing_cell: Option<CellId>,
    remaining_runs: Option<VecDeque<CellRun>>,
    kernel_notice: Option<String>,
    execution_requests: HashMap<String, CellExecution>,
    display_slots: HashMap<CellId, DisplaySlots>,
}

struct NotebookKernelSession(gpui::WeakEntity<NotebookEditor>);

struct StructureStep {
    edit: CellEdit,
    before: usize,
    after: usize,
    rewind: Option<BufferRewind>,
    removed: Option<(Cell, SourceTransactions, LineEndings)>,
}

#[derive(Clone, Copy)]
enum BufferRewind {
    Undo(text::TransactionId),
    Redo(text::TransactionId),
}

impl BufferRewind {
    fn apply(
        self,
        buffer: &mut language::Buffer,
        cx: &mut Context<language::Buffer>,
    ) -> Option<Self> {
        match self {
            Self::Undo(id) if buffer.peek_undo_stack()?.transaction_id() == id => {
                buffer.undo(cx).map(Self::Redo)
            }
            Self::Redo(id) if buffer.peek_redo_stack()?.transaction_id() == id => {
                buffer.redo(cx).map(Self::Undo)
            }
            _ => None,
        }
    }
}

type SourceTransactions =
    HashMap<clock::Lamport, Option<(language::Transaction, std::ops::Range<language::Anchor>)>>;

/// Newlines whose saved terminator differs from the cell buffer's line ending.
type LineEndings = Vec<(language::Anchor, &'static str)>;

struct CellSourceHistory {
    cell: Entity<language::Buffer>,
    observed: clock::Global,
    transactions: SourceTransactions,
    line_endings: LineEndings,
    _subscriptions: [gpui::Subscription; 3],
}

impl CellSourceHistory {
    fn has_pending_edits(&self, cx: &App) -> bool {
        self.cell.read(cx).as_text_snapshot().version() != &self.observed
    }

    fn sync(
        &mut self,
        operation: &text::Operation,
        fields: &mut CellFields<language::Anchor>,
        buffer: &mut language::Buffer,
        cx: &mut Context<language::Buffer>,
    ) -> Result<()> {
        let mut version = match operation {
            text::Operation::Edit(edit) => edit.version.clone(),
            text::Operation::Undo(undo) => undo.version.clone(),
        };
        version.observe(operation.timestamp());
        let cell = self.cell.read(cx);
        let text = cell.rope_for_version(&version);
        let source = if self.line_endings.is_empty() {
            text::chunks_with_line_ending(&text, cell.line_ending()).collect::<String>()
        } else {
            let kept: HashMap<usize, &str> = cell
                .offsets_at_version(
                    self.line_endings.iter().map(|(newline, _)| newline),
                    &version,
                )
                .zip(self.line_endings.iter().map(|(_, ending)| *ending))
                .filter_map(|(offset, ending)| Some((offset?, ending)))
                .collect();
            let default_ending = cell.line_ending().as_str();
            let normalized = text.to_string();
            let mut source = String::with_capacity(normalized.len());
            for (offset, character) in normalized.char_indices() {
                if character == '\n' {
                    source.push_str(kept.get(&offset).copied().unwrap_or(default_ending));
                } else {
                    source.push(character);
                }
            }
            source
        };

        // A moved or deleted cell's earlier edits live in hidden notebook text
        // until a structure undo makes that text visible at the cell again.
        if let text::Operation::Undo(undo) = operation
            && undo.counts.keys().all(|id| {
                self.transactions.get(id).is_some_and(|edit| {
                    edit.as_ref().is_none_or(|(_, source)| {
                        source.start.is_valid(buffer)
                            && source.end.is_valid(buffer)
                            && source.start.to_offset(buffer)
                                == fields.source.start.to_offset(buffer)
                            && source.end.to_offset(buffer) == fields.source.end.to_offset(buffer)
                    })
                })
            })
        {
            let mut affected: Vec<_> = undo
                .counts
                .iter()
                .filter_map(|(edit, count)| {
                    self.transactions
                        .get(edit)?
                        .as_ref()
                        .map(|(transaction, _)| (transaction, *count))
                })
                .collect();
            affected.sort_unstable_by_key(|(transaction, _)| transaction.id);
            for (transaction, _) in affected.iter().rev().filter(|(_, count)| count % 2 == 1) {
                anyhow::ensure!(
                    buffer.undo_transaction(transaction.id, cx),
                    "Could not undo the notebook cell edit"
                );
            }
            for (transaction, _) in affected.into_iter().filter(|(_, count)| count % 2 == 0) {
                anyhow::ensure!(
                    buffer.redo_transaction(transaction.clone(), cx),
                    "Could not redo the notebook cell edit"
                );
                buffer.finalize_last_transaction();
            }

            let raw = buffer
                .text_for_range(fields.source.clone())
                .collect::<String>();
            let mut restored: String = if fields.multiline {
                serde_json::from_str::<Vec<String>>(&raw)?.concat()
            } else {
                serde_json::from_str(&raw)?
            };
            language::LineEnding::normalize(&mut restored);
            anyhow::ensure!(
                restored == text.to_string(),
                "Cell source history no longer matches the notebook"
            );
            fields.last_source = source;
            return Ok(());
        }

        let edits = fields.edit_source(&source)?;
        let transaction = if edits.is_empty() {
            None
        } else {
            buffer.finalize_last_transaction();
            buffer.edit(edits, None, cx);
            Some((
                buffer
                    .finalize_last_transaction()
                    .context("Missing notebook source transaction")?
                    .clone(),
                fields.source.clone(),
            ))
        };
        match operation {
            text::Operation::Edit(edit) => {
                self.transactions.insert(edit.timestamp, transaction);
            }
            text::Operation::Undo(_) => self.transactions.clear(),
        }
        Ok(())
    }
}

struct CellRun {
    cell_id: CellId,
    source: String,
    advance: bool,
}

type DisplaySlots = Vec<(String, usize)>;

struct CellExecution {
    cell_id: CellId,
    source: String,
    outputs: Vec<serde_json::Value>,
    reply: Option<jupyter_protocol::ExecuteReply>,
    idle: bool,
    clear_on_output: bool,
    previous: Option<(super::cell::PreviousExecution, DisplaySlots)>,
}

impl CellExecution {
    fn new(cell_id: CellId, source: String) -> Self {
        Self {
            cell_id,
            source,
            outputs: Vec::new(),
            reply: None,
            idle: false,
            clear_on_output: false,
            previous: None,
        }
    }

    fn started(&self) -> bool {
        self.previous.is_some()
    }

    fn queued(cell: &Entity<CodeCell>, source: String, cx: &mut App) -> Self {
        cell.update(cx, |cell, cx| {
            cell.queue_execution();
            cx.notify();
            Self::new(cell.id().clone(), source)
        })
    }
}

impl NotebookEditor {
    pub fn new(
        project: Entity<Project>,
        notebook_item: Entity<NotebookItem>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |notebook, window, cx| {
            // Blur hides a Markdown editor, so refocusing alone would send input to an unmounted editor.
            if notebook.notebook_mode == NotebookMode::Edit {
                notebook.enter_edit_mode(&EnterEditMode, window, cx);
            }
        })
        .detach();

        let worktree_id = notebook_item.read(cx).project_path.worktree_id;

        let notebook_language = notebook_item.read(cx).notebook_language();
        let notebook_language = cx
            .spawn_in(window, async move |_, _| notebook_language.await)
            .shared();

        let cells = notebook_item.update(cx, |item, _| std::mem::take(&mut item.notebook.cells));
        let cell_list =
            ListState::new(cells.len(), gpui::ListAlignment::Top, px(1000.)).measure_all();

        let mut editor = Self {
            project,
            worktree_id,
            focus_handle,
            notebook_item: notebook_item.clone(),
            cell_list,
            mouse_autoscroll: None,
            notebook_mode: NotebookMode::Command,
            selected_cell_index: 0,
            structure_error: None,
            structure_undo: Vec::new(),
            structure_redo: Vec::new(),
            cell_order: Vec::with_capacity(cells.len()),
            source_version: notebook_item.read(cx).buffer.read(cx).version(),
            sync_errors: HashMap::default(),
            cell_sources: HashMap::default(),
            cell_map: HashMap::default(),
            notebook_language,
            kernel: Kernel::Shutdown,
            kernel_specification: None,
            execution_requests: HashMap::default(),
            display_slots: HashMap::default(),
            kernel_session: None,
            kernel_task: None,
            preparing_cell: None,
            remaining_runs: None,
            kernel_notice: None,
        };
        let languages = editor.project.read(cx).languages().clone();
        for cell in cells {
            let cell_entity = Cell::load(
                &cell,
                &languages,
                editor.notebook_language.clone(),
                window,
                cx,
            );
            let cell_id =
                editor.attach_cell(cell_entity, &cell.source().concat(), None, window, cx);
            editor.cell_order.push(cell_id);
        }
        editor.watch_kernel_trust(cx);
        editor
    }

    fn attach_cell(
        &mut self,
        cell_entity: Cell,
        source: &str,
        history: Option<(SourceTransactions, LineEndings)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> CellId {
        let cell_id = cell_entity.id(cx);
        let cell_events = match &cell_entity {
            Cell::Code(cell) => {
                Some(
                    cx.subscribe_in(cell, window, |this, _, event, window, cx| match event {
                        CellEvent::Run(id) => this.execute_cell(id.clone(), false, window, cx),
                        CellEvent::FocusedIn(id) => this.select_cell_by_id(id, cx),
                    }),
                )
            }
            Cell::Markdown(cell) => {
                Some(cx.subscribe(cell, |_, cell, _: &MarkdownCellEvent, cx| {
                    cell.update(cx, |cell, cx| cell.reparse_markdown(cx));
                }))
            }
            Cell::Raw(_) => None,
        };

        if let Some(cell_events) = cell_events
            && let Some(editor) = cell_entity.editor(cx).cloned()
            && let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton()
        {
            let focused_id = cell_id.clone();
            let focus_events = cx.subscribe(&editor, move |this, _, event, cx| {
                if matches!(event, editor::EditorEvent::Focused) {
                    this.select_cell_by_id(&focused_id, cx);
                }
            });
            let edited_id = cell_id.clone();
            let buffer_events = cx.subscribe(&buffer, move |this, buffer, event, cx| {
                if let language::BufferEvent::Operation {
                    operation: language::Operation::Buffer(operation),
                    ..
                } = event
                {
                    this.apply_cell_operation(&edited_id, &buffer, operation, cx);
                }
            });
            let (transactions, line_endings) = history
                .unwrap_or_else(|| (HashMap::default(), line_endings(buffer.read(cx), source)));
            self.cell_sources.insert(
                cell_id.clone(),
                CellSourceHistory {
                    observed: buffer.read(cx).version(),
                    cell: buffer,
                    transactions,
                    line_endings,
                    _subscriptions: [cell_events, focus_events, buffer_events],
                },
            );
        }
        self.cell_map.insert(cell_id.clone(), cell_entity);
        cell_id
    }

    fn check_execution(&self, cx: &mut App) -> Result<()> {
        self.check_trust(cx)?;
        let project = self.project.read(cx);
        if matches!(
            self.kernel_specification,
            Some(
                KernelSpecification::JupyterServer(_)
                    | KernelSpecification::SshRemote(_)
                    | KernelSpecification::WslRemote(_)
            )
        ) {
            anyhow::ensure!(
                Self::remote_enabled(cx),
                "Remote notebook kernels are disabled."
            );
            return Ok(());
        }
        anyhow::ensure!(
            project.is_local(),
            "Notebook execution requires a local project."
        );
        if matches!(
            &self.kernel_specification,
            Some(KernelSpecification::Jupyter(spec)) if !spec.is_notebook_python()
        ) {
            anyhow::ensure!(
                Self::remote_enabled(cx),
                "Choose a Python kernel. Other local kernels are disabled."
            );
            return Ok(());
        }
        anyhow::ensure!(
            self.is_python_notebook(cx),
            "Only Python notebooks can be executed."
        );
        Ok(())
    }

    fn is_python_notebook(&self, cx: &App) -> bool {
        self.notebook_item
            .read(cx)
            .language_name()
            .is_none_or(|language| language.eq_ignore_ascii_case("python"))
    }

    fn remote_enabled(cx: &App) -> bool {
        cx.has_flag::<NotebookFeatureFlag>() || std::env::var_os("LOCAL_NOTEBOOK_DEV").is_some()
    }

    fn check_trust(&self, cx: &mut App) -> Result<()> {
        let project = self.project.read(cx);
        anyhow::ensure!(
            !project.is_via_collab(),
            "Notebook execution is unavailable in shared guest projects."
        );
        let store = project.worktree_store();
        let trusted = TrustedWorktrees::try_get_global(cx).is_some_and(|trust| {
            !TrustedWorktrees::has_restricted_worktrees(&store, cx)
                && trust.update(cx, |trust, cx| {
                    trust.can_trust(&store, self.worktree_id, cx)
                })
        });
        anyhow::ensure!(
            trusted,
            "Notebook execution is disabled in Restricted Mode. Trust this workspace to run cells."
        );
        anyhow::ensure!(
            self.source_is_current(cx) && self.sync_errors.is_empty(),
            "The notebook source changed. Reopen the notebook before running it."
        );
        Ok(())
    }

    fn watch_kernel_trust(&mut self, cx: &mut Context<Self>) {
        if let Some(trust) = TrustedWorktrees::try_get_global(cx) {
            let store = self.project.read(cx).worktree_store().downgrade();
            cx.subscribe(&trust, move |this, _, event, cx| {
                if matches!(event, TrustedWorktreesEvent::Restricted(changed, _) if changed == &store) {
                    this.stop_kernel(cx);
                    this.kernel_specification = None;
                    this.kernel_notice = Some("Notebook execution is disabled in Restricted Mode.".into());
                    cx.notify();
                }
            }).detach();
        }
    }

    pub(super) fn stop_kernel(&mut self, cx: &mut Context<Self>) {
        self.remaining_runs = None;
        self.kernel_task.take();
        self.preparing_cell = None;
        self.kernel_session = None;
        self.kernel = Kernel::Shutdown;
        self.kernel_notice = (!self.execution_requests.is_empty())
            .then(|| "Kernel stopped. Your previous results are unchanged.".into());
        self.display_slots.clear();
        for (_, run) in self.execution_requests.drain() {
            if let Some(Cell::Code(cell)) = self.cell_map.get(&run.cell_id) {
                cell.update(cx, |cell, cx| {
                    cell.cancel_execution(run.previous.map(|(previous, _)| previous));
                    cx.notify();
                });
            }
        }
        cx.notify();
    }

    pub(super) fn source_is_current(&self, cx: &App) -> bool {
        self.notebook_item.read(cx).buffer.read(cx).version() == self.source_version
    }

    pub(super) fn project_path<'a>(&self, cx: &'a App) -> &'a ProjectPath {
        &self.notebook_item.read(cx).project_path
    }

    fn patch_cell(
        &mut self,
        cell_index: usize,
        cx: &mut Context<Self>,
        patch: impl FnOnce(
            &mut CellFields<language::Anchor>,
        ) -> Result<Vec<(std::ops::Range<language::Anchor>, String)>>,
    ) -> Result<()> {
        self.update_cell(cell_index, cx, |fields, _, buffer, cx| {
            buffer.edit(patch(fields)?, None, cx);
            Ok(())
        })
    }

    fn update_cell(
        &mut self,
        cell_index: usize,
        cx: &mut Context<Self>,
        update: impl FnOnce(
            &mut CellFields<language::Anchor>,
            &mut CellSourceHistory,
            &mut language::Buffer,
            &mut Context<language::Buffer>,
        ) -> Result<()>,
    ) -> Result<()> {
        let buffer = self.notebook_item.read(cx).buffer.clone();
        anyhow::ensure!(
            self.source_is_current(cx),
            "The notebook source changed. Reopen the notebook before editing its cells."
        );
        anyhow::ensure!(
            buffer.read(cx).capability().editable(),
            "The notebook source is read-only."
        );
        let history = self
            .cell_order
            .get(cell_index)
            .and_then(|id| self.cell_sources.get_mut(id))
            .context("Cell is not editable")?;
        let result = self.notebook_item.update(cx, |item, cx| {
            let fields = item
                .fields
                .get_mut(cell_index)
                .context("Cell no longer exists")?;
            buffer.update(cx, |buffer, cx| update(fields, history, buffer, cx))
        });
        self.source_version = buffer.read(cx).version();
        result
    }

    fn apply_cell_operation(
        &mut self,
        cell_id: &CellId,
        cell: &Entity<language::Buffer>,
        operation: &text::Operation,
        cx: &mut Context<Self>,
    ) {
        let Some(history) = self.cell_sources.get(cell_id) else {
            return;
        };
        if &history.cell != cell || history.observed.observed(operation.timestamp()) {
            return;
        }
        let Some(index) = self.cell_order.iter().position(|id| id == cell_id) else {
            return;
        };
        let result = self.update_cell(index, cx, |fields, history, buffer, cx| {
            history.observed.observe(operation.timestamp());
            history.sync(operation, fields, buffer, cx)
        });
        match result {
            Ok(()) => {
                self.sync_errors.remove(&index);
            }
            Err(error) => {
                self.sync_errors
                    .insert(index, format!("Cell {}: {error}", index + 1));
            }
        }
        cx.notify();
    }

    pub(super) fn flush_cell_edits(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let mut pending = Vec::new();
        for (index, cell_id) in self.cell_order.iter().enumerate() {
            if let Some(history) = self.cell_sources.get(cell_id)
                && history.has_pending_edits(cx)
            {
                let cell = history.cell.read(cx);
                let mut version = history.observed.clone();
                for (id, operation) in cell
                    .operations()
                    .iter()
                    .filter(|(id, _)| !history.observed.observed(**id))
                {
                    pending.push((cell_id.clone(), history.cell.clone(), operation.clone()));
                    version.observe(*id);
                }
                anyhow::ensure!(
                    &version == cell.as_text_snapshot().version(),
                    "Cell {} has unmirrored edits",
                    index + 1
                );
            }
        }
        for (cell_id, cell, operation) in pending {
            self.apply_cell_operation(&cell_id, &cell, &operation, cx);
        }
        if let Some(error) = self.sync_errors.values().next() {
            anyhow::bail!("{error}");
        }
        Ok(())
    }

    pub(super) fn has_unsynced_edits(&self, cx: &App) -> bool {
        !self.sync_errors.is_empty()
            || self
                .cell_sources
                .values()
                .any(|history| history.has_pending_edits(cx))
    }

    fn persist_execution(
        &mut self,
        cell_index: usize,
        source: &str,
        outputs: &[serde_json::Value],
        execution_count: Option<i64>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let cell = self
            .cell_order
            .get(cell_index)
            .and_then(|id| self.cell_map.get(id))
            .context("The executed cell no longer exists")?;
        anyhow::ensure!(
            cell.current_source(cx) == source,
            "The cell changed while running. Run it again to save its outputs."
        );
        self.patch_cell(cell_index, cx, |fields| {
            fields.edit_execution(outputs, execution_count)
        })
    }

    pub(super) fn cell(&self, index: usize) -> Option<&Cell> {
        self.cell_order
            .get(index)
            .and_then(|id| self.cell_map.get(id))
    }

    pub(super) fn save(
        &mut self,
        _options: SaveOptions,
        project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if let Err(error) = self.flush_cell_edits(cx) {
            return Task::ready(Err(error));
        }
        let buffer = self.notebook_item.read(cx).buffer.clone();
        project.update(cx, |project, cx| project.save_buffer(buffer, cx))
    }

    fn project_python(
        &self,
        use_toolchain: bool,
        cx: &App,
    ) -> Result<Task<Option<std::path::PathBuf>>> {
        let project = self.project.read(cx);
        let path = self.notebook_item.read(cx).project_path.clone();
        let tree = project
            .worktree_for_id(path.worktree_id, cx)
            .context("Notebook worktree is no longer open")?;
        let tree = tree.read(cx);
        if tree.is_single_file() {
            return Ok(Task::ready(None));
        }
        let default_python = tree.abs_path().join(if cfg!(windows) {
            ".venv/Scripts/python.exe"
        } else {
            ".venv/bin/python"
        });
        if !use_toolchain {
            return Ok(Task::ready(Some(default_python)));
        }
        let toolchain =
            project.active_toolchain(path, language::LanguageName::new_static("Python"), cx);
        Ok(cx.background_spawn(async move {
            Some(
                toolchain
                    .await
                    .map(|toolchain| std::path::PathBuf::from(toolchain.path.as_ref()))
                    .unwrap_or(default_python),
            )
        }))
    }

    fn choose_or_start_kernel(
        &mut self,
        run: Option<CellRun>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Err(error) = self.check_run(run.as_ref(), cx) {
            self.kernel_notice = Some(error.to_string());
            cx.notify();
            return;
        }
        if self.kernel_task.is_some() {
            self.kernel_notice = Some("Cell not queued. Run it again after kernel setup.".into());
            cx.notify();
            return;
        }
        let selected = run.as_ref().and(self.kernel_specification.clone());
        self.preparing_cell = run.as_ref().map(|run| run.cell_id.clone());
        let fs = self.project.read(cx).fs().clone();
        self.kernel_notice = Some("Finding a kernel…".into());
        let prepare = cx.spawn_in(window, async move |this, cx| {
            let result = Self::prepare_kernel(&this, selected, fs, run.as_ref(), cx).await;
            this.update_in(cx, |editor, window, cx| {
                editor.kernel_task = None;
                editor.preparing_cell = None;
                if result.is_err() || editor.execution_requests.is_empty() {
                    editor.remaining_runs = None;
                }
                editor.kernel_notice = result.err().map(|error| {
                    error
                        .chain()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join("\n")
                });
                if editor
                    .kernel_session
                    .take_if(|_| matches!(editor.kernel, Kernel::Shutdown))
                    .is_some()
                    && let Some(error) = &editor.kernel_notice
                    && let Some(run) = run
                    && let Some(Cell::Code(cell)) = editor.cell_map.get(&run.cell_id)
                {
                    cell.update(cx, |cell, cx| cell.show_kernel_error(error, window, cx));
                }
                cx.notify();
            })
            .ok();
        });
        self.kernel_task = Some(prepare.shared());
        cx.notify();
    }

    fn check_run(&self, run: Option<&CellRun>, cx: &mut App) -> Result<()> {
        let Some(run) = run else {
            self.check_trust(cx)?;
            anyhow::ensure!(
                self.project.read(cx).is_local(),
                "Choose a remote kernel for this project."
            );
            anyhow::ensure!(
                Self::remote_enabled(cx) || self.is_python_notebook(cx),
                "Only Python notebooks can be executed."
            );
            return Ok(());
        };
        self.check_execution(cx)?;
        anyhow::ensure!(
            self.cell_map
                .get(&run.cell_id)
                .is_some_and(|cell| cell.current_source(cx) == run.source),
            "The cell changed. Run it again."
        );
        Ok(())
    }

    async fn prepare_kernel(
        this: &gpui::WeakEntity<Self>,
        selected: Option<KernelSpecification>,
        fs: Arc<dyn project::Fs>,
        run: Option<&CellRun>,
        cx: &mut gpui::AsyncWindowContext,
    ) -> Result<()> {
        let Some(spec) = Self::prepare_spec(this, selected, fs, run, cx).await? else {
            return Ok(());
        };
        let Some(run) = run else { return Ok(()) };
        let launch = this.update_in(cx, |editor, window, cx| {
            editor.check_run(Some(run), cx)?;
            let directory = editor
                .project
                .read(cx)
                .absolute_path(&editor.notebook_item.read(cx).project_path, cx)
                .and_then(|path| path.parent().map(ToOwned::to_owned))
                .context("Cannot find notebook directory")?;
            let owner = cx.weak_entity();
            let session = cx.new(|_| NotebookKernelSession(owner));
            editor.kernel_session = Some(session.clone());
            editor.kernel_specification = Some(spec.clone());
            editor.kernel_notice = Some(format!("Starting {}…", spec.name()));
            cx.notify();
            Ok::<_, anyhow::Error>(match spec {
                KernelSpecification::Jupyter(spec) => NativeRunningKernel::new(
                    spec,
                    session.entity_id(),
                    directory,
                    editor.project.read(cx).fs().clone(),
                    session,
                    window,
                    cx,
                ),
                KernelSpecification::JupyterServer(spec) => {
                    RemoteRunningKernel::new(spec, directory, session, window, cx)
                }
                KernelSpecification::SshRemote(spec) => SshRunningKernel::new(
                    spec,
                    directory,
                    editor.project.clone(),
                    session,
                    window,
                    cx,
                ),
                KernelSpecification::WslRemote(spec) => WslRunningKernel::new(
                    spec,
                    session.entity_id(),
                    directory,
                    editor.project.read(cx).fs().clone(),
                    session,
                    window,
                    cx,
                ),
                KernelSpecification::PythonEnv(_) => {
                    anyhow::bail!("Use the local Python kernel picker.")
                }
            })
        })??;
        let kernel = launch.await.context("the kernel failed to launch")?;
        this.update_in(cx, |editor, window, cx| {
            editor.check_run(Some(run), cx)?;
            editor.kernel = Kernel::RunningKernel(kernel);
            editor.dispatch_cell(run, window, cx)
        })??;
        Ok(())
    }

    async fn prepare_spec(
        this: &gpui::WeakEntity<Self>,
        selected: Option<KernelSpecification>,
        fs: Arc<dyn project::Fs>,
        run: Option<&CellRun>,
        cx: &mut gpui::AsyncWindowContext,
    ) -> Result<Option<KernelSpecification>> {
        let (mut specs, preferred, list_kernelspecs) = if let Some(spec) = selected {
            match spec {
                KernelSpecification::Jupyter(spec) if spec.is_notebook_python() => {
                    (vec![spec], None, false)
                }
                spec => return Ok(Some(spec)),
            }
        } else {
            let (project_python, list_kernelspecs) = this.update(cx, |editor, cx| {
                editor.check_run(run, cx)?;
                anyhow::Ok((
                    editor.project_python(run.is_some(), cx)?,
                    run.is_none() && Self::remote_enabled(cx),
                ))
            })??;
            let project_python = project_python.await;
            let specs = cx
                .background_spawn(crate::kernels::notebook_kernel_choices(
                    fs,
                    project_python.clone(),
                    list_kernelspecs,
                ))
                .await?;
            (specs, project_python, list_kernelspecs)
        };
        let mut diagnostic = None;
        if run.is_some() {
            this.update(cx, |editor, cx| editor.check_run(run, cx))??;
            (specs, diagnostic) = cx
                .background_spawn(async move {
                    crate::kernels::suitable_notebook_python_kernels(specs, preferred.as_deref())
                        .await
                })
                .await;
        }
        this.update(cx, |editor, cx| editor.check_run(run, cx))??;
        let spec = if specs.is_empty() {
            anyhow::ensure!(
                run.is_some(),
                "No Python kernel found. Run a cell to set up Python for Zed."
            );
            let consent = this.update_in(cx, |editor, window, cx| {
                    let reason = if let Some(selected) = editor.kernel_specification.take() {
                        if let Some(error) = diagnostic {
                            if let Some(run) = run
                                && let Some(Cell::Code(cell)) = editor.cell_map.get(&run.cell_id) {
                                cell.update(cx, |cell, cx| cell.show_kernel_error(&error, window, cx));
                            }
                            error
                        } else {
                            format!("ipykernel is not installed in {} ({}).", selected.name(), selected.path())
                        }
                    } else {
                        diagnostic.unwrap_or_else(|| "No local Python kernel was found.".into())
                    };
                    editor.kernel_notice = Some(reason.clone());
                    cx.notify();
                    window.prompt(gpui::PromptLevel::Info, "Set up Python for notebooks?",
                        Some(&format!("{reason}\n\nZed will download a private Python environment with ipykernel and matplotlib. This won't change your project's Python environment.")),
                        &["Set Up Python", "Cancel"], cx)
                })?;
            if consent.await.ok() != Some(0) {
                anyhow::bail!("Python setup was canceled. Run a cell to try again.");
            }
            let install = this.update(cx, |editor, cx| {
                editor.check_run(run, cx)?;
                editor.kernel_notice = Some("Setting up Python for notebooks…".into());
                cx.notify();
                anyhow::Ok(super::kernel_setup::install_python_kernel(cx))
            })??;
            let path = install.await?;
            LocalKernelSpecification::for_notebook_python(path, "Zed Python".into())?
        } else if run.is_some() && specs.len() == 1 {
            specs.remove(0)
        } else {
            let choices = specs
                .iter()
                .map(|spec| {
                    format!("{} ({})", spec.kernelspec.display_name, spec.path.display()).into()
                })
                .collect();
            let choice = cx
                .update(|window, cx| {
                    ui_prompt::prompt_for_choices(
                        if list_kernelspecs {
                            "Select a local kernel"
                        } else {
                            "Select a local Python kernel"
                        },
                        if run.is_some() {
                            "The requested execution will use this kernel."
                        } else {
                            "Selecting a kernel does not run a cell."
                        },
                        choices,
                        window,
                        cx,
                    )
                })?
                .await?;
            let Some(spec) = specs.get(choice) else {
                return Ok(None);
            };
            if run.is_none() {
                this.update(cx, |editor, cx| {
                    editor.check_run(run, cx)?;
                    editor.select_kernel(KernelSpecification::Jupyter(spec.clone()), cx);
                    anyhow::Ok(())
                })??;
                return Ok(None);
            }
            spec.clone()
        };
        Ok(Some(KernelSpecification::Jupyter(spec)))
    }

    fn choose_remote_kernel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !Self::remote_enabled(cx)
            || self.kernel_task.is_some()
            || !self.execution_requests.is_empty()
        {
            return;
        }
        if let Err(error) = self.check_trust(cx) {
            self.kernel_notice = Some(error.to_string());
            cx.notify();
            return;
        }
        let project = self.project.read(cx);
        let remote_project = project.is_via_remote_server();
        let global_kernels = project.is_local()
            || matches!(
                project.remote_connection_options(cx),
                Some(remote::RemoteConnectionOptions::Wsl(_))
            );
        let mut sources = Vec::new();
        if remote_project {
            let specs = crate::kernels::python_env_kernel_specifications(
                &self.project,
                self.worktree_id,
                cx,
            );
            sources.push(("Remote project", cx.background_spawn(specs)));
        }
        if global_kernels {
            if cfg!(windows) {
                sources.push((
                    "WSL",
                    cx.background_spawn(crate::kernels::wsl_kernel_specifications(
                        cx.background_executor().clone(),
                    )),
                ));
            }
            if let Some(specs) = crate::repl_store::ReplStore::get_remote_kernel_specifications(cx)
            {
                sources.push(("Jupyter server", specs));
            }
        }
        self.kernel_notice = Some("Finding remote notebook kernels…".into());
        self.kernel_task = Some(cx.spawn_in(window, async move |this, cx| {
            let pick = async {
                let mut specs = Vec::new();
                let mut details = vec!["Running a cell sends its source to the selected kernel. Selecting does not run code.".to_owned()];
                for (name, source) in sources {
                    match source.await {
                        Ok(found) => specs.extend(found),
                        Err(_) => details.push(format!("{name} kernel discovery failed. Check the connection and credentials.")),
                    }
                }
                anyhow::ensure!(!specs.is_empty(), "{}", details.get(1).map(String::as_str).unwrap_or("No remote kernels found. Configure JUPYTER_SERVER and JUPYTER_TOKEN or open an SSH or WSL project."));
                let choices = specs.iter().map(|spec| {
                    let location = match spec {
                        KernelSpecification::JupyterServer(server) => http_client::Url::parse(&server.url).map(|url| url.origin().ascii_serialization()).unwrap_or_else(|_| "Jupyter server".into()),
                        _ => spec.path().to_string(),
                    };
                    format!("{} ({}: {})", spec.name(), spec.type_name(), location).into()
                }).collect();
                let choice = this.update_in(cx, |editor, window, cx| {
                    editor.check_trust(cx)?;
                    Ok::<_, anyhow::Error>(ui_prompt::prompt_for_choices("Select a remote kernel", &details.join("\n"), choices, window, cx))
                })??.await?;
                if let Some(spec) = specs.get(choice) {
                    this.update(cx, |editor, cx| {
                        editor.check_trust(cx)?;
                        editor.select_kernel(spec.clone(), cx);
                        anyhow::Ok(())
                    })??;
                }
                anyhow::Ok(())
            }.await;
            this.update(cx, |editor, cx| {
                editor.kernel_task = None;
                editor.kernel_notice = pick.err().map(|error| error.to_string());
                cx.notify();
            }).ok();
        }).shared());
        cx.notify();
    }

    fn select_kernel(&mut self, spec: KernelSpecification, cx: &mut Context<Self>) {
        if self.kernel_specification.as_ref() != Some(&spec) {
            self.stop_kernel(cx);
            self.kernel_specification = Some(spec);
        }
        self.kernel_notice = None;
        cx.notify();
    }

    fn restart_kernel(&mut self, _: &RestartKernel, _window: &mut Window, cx: &mut Context<Self>) {
        self.stop_kernel(cx);
        self.kernel_notice
            .get_or_insert_with(|| "Kernel stopped. Run a cell to start a new session.".into());
    }

    fn interrupt_kernel(
        &mut self,
        _: &InterruptKernel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.remaining_runs = None;
        if self.kernel_task.is_some() {
            self.stop_kernel(cx);
            return;
        }
        if let Kernel::RunningKernel(kernel) = &self.kernel
            && let Err(error) = kernel
                .request_tx()
                .try_send(jupyter_protocol::InterruptRequest {}.into())
        {
            self.kernel_notice = Some(format!("Could not interrupt the kernel: {error}"));
            cx.notify();
        }
    }

    fn execute_cell(
        &mut self,
        cell_id: CellId,
        advance: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.remaining_runs.is_some() {
            self.kernel_notice = Some("Run All is in progress.".into());
            cx.notify();
            return;
        }
        let Some(Cell::Code(cell)) = self.cell_map.get(&cell_id) else {
            return;
        };
        let run = CellRun {
            cell_id,
            source: cell.read(cx).current_source(cx),
            advance,
        };
        self.start_run(run, window, cx);
    }

    fn run_all(&mut self, _: &RunAll, window: &mut Window, cx: &mut Context<Self>) {
        if self.kernel_task.is_some() || !self.execution_requests.is_empty() {
            self.kernel_notice = Some("Wait for the current run or kernel setup to finish.".into());
            cx.notify();
            return;
        }
        if let Err(error) = self.check_execution(cx) {
            self.kernel_notice = Some(error.to_string());
            cx.notify();
            return;
        }
        let mut runs: VecDeque<_> = self
            .cell_order
            .iter()
            .filter_map(|id| {
                let Cell::Code(cell) = self.cell_map.get(id)? else {
                    return None;
                };
                Some(CellRun {
                    cell_id: id.clone(),
                    source: cell.read(cx).current_source(cx),
                    advance: false,
                })
            })
            .collect();
        if let Some(first) = runs.pop_front() {
            self.remaining_runs = Some(runs);
            self.start_run(first, window, cx);
        }
    }

    fn start_run(&mut self, run: CellRun, window: &mut Window, cx: &mut Context<Self>) {
        if !matches!(self.kernel, Kernel::RunningKernel(_)) {
            self.choose_or_start_kernel(Some(run), window, cx);
            return;
        }
        let result = self.dispatch_cell(&run, window, cx);
        if result.is_err() {
            self.remaining_runs = None;
        }
        self.kernel_notice = result.err().map(|error| error.to_string());
        cx.notify();
    }

    fn dispatch_cell(
        &mut self,
        run: &CellRun,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.check_execution(cx)?;
        anyhow::ensure!(
            !self
                .execution_requests
                .values()
                .any(|execution| execution.cell_id == run.cell_id),
            "This cell is already running."
        );
        let Some(Cell::Code(cell)) = self.cell_map.get(&run.cell_id) else {
            anyhow::bail!("Cell not found");
        };
        anyhow::ensure!(
            cell.read(cx).current_source(cx) == run.source,
            "The queued cell changed. Run it again."
        );
        let Kernel::RunningKernel(kernel) = &self.kernel else {
            anyhow::bail!("Kernel is not running");
        };
        let message: JupyterMessage = ExecuteRequest {
            code: run.source.clone(),
            ..Default::default()
        }
        .into();
        let id = message.header.msg_id.clone();
        kernel
            .request_tx()
            .try_send(message)
            .map_err(|error| anyhow::anyhow!("Could not send code to the kernel: {error}"))?;
        self.execution_requests
            .insert(id, CellExecution::queued(cell, run.source.clone(), cx));
        if run.advance && self.cell_order.get(self.selected_cell_index) == Some(&run.cell_id) {
            self.advance_in_command_mode(window, cx);
        }
        cx.notify();
        Ok(())
    }

    fn change_structure(
        &mut self,
        change: CellChange,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let before = self.selected_cell_index;
        let count = self.cell_order.len();
        let (edit, after) = match change {
            CellChange::InsertCode | CellChange::InsertMarkdown => {
                let index = (before + 1).min(count);
                (
                    CellEdit::Add {
                        index,
                        markdown: matches!(change, CellChange::InsertMarkdown),
                    },
                    index,
                )
            }
            CellChange::Delete if before < count => (
                CellEdit::Remove(before),
                before.min(count.saturating_sub(2)),
            ),
            CellChange::MoveUp if before > 0 => (CellEdit::Swap(before - 1), before - 1),
            CellChange::MoveDown if before + 1 < count => (CellEdit::Swap(before), before + 1),
            _ => return,
        };
        let mut step = StructureStep {
            edit,
            before,
            after,
            rewind: None,
            removed: None,
        };
        self.structure_error = match self.apply_structure(&mut step, window, cx) {
            Ok(inverse) => {
                self.structure_undo.push(inverse);
                self.structure_redo.clear();
                if matches!(change, CellChange::InsertCode | CellChange::InsertMarkdown) {
                    self.enter_edit_mode(&EnterEditMode, window, cx);
                }
                None
            }
            Err(error) => Some(error.to_string()),
        };
        cx.notify();
    }

    fn undo_structure(&mut self, redo: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(mut step) = (if redo {
            &mut self.structure_redo
        } else {
            &mut self.structure_undo
        })
        .pop() else {
            return;
        };
        let result = self.apply_structure(&mut step, window, cx);
        let (from, to) = if redo {
            (&mut self.structure_redo, &mut self.structure_undo)
        } else {
            (&mut self.structure_undo, &mut self.structure_redo)
        };
        self.structure_error = match result {
            Ok(inverse) => {
                to.push(inverse);
                None
            }
            Err(error) => {
                from.push(step);
                Some(error.to_string())
            }
        };
        cx.notify();
    }

    fn apply_structure(
        &mut self,
        step: &mut StructureStep,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<StructureStep> {
        self.flush_cell_edits(cx)?;
        anyhow::ensure!(
            self.source_is_current(cx),
            "The notebook source changed. Reopen the notebook before editing its cells."
        );
        let buffer = self.notebook_item.read(cx).buffer.clone();
        anyhow::ensure!(
            buffer.read(cx).capability().editable(),
            "The notebook source is read-only."
        );
        if let CellEdit::Remove(index) = step.edit {
            let id = self
                .cell_order
                .get(index)
                .context("Cell no longer exists")?;
            anyhow::ensure!(
                !self
                    .remaining_runs
                    .iter()
                    .flatten()
                    .any(|run| &run.cell_id == id)
                    && !self
                        .execution_requests
                        .values()
                        .any(|run| &run.cell_id == id),
                "This cell is queued or running. Wait for it to finish or stop the kernel before deleting it."
            );
            anyhow::ensure!(
                self.preparing_cell.as_ref() != Some(id),
                "Wait for this cell’s kernel setup to finish before deleting it."
            );
        }
        let snapshot = buffer.read(cx).as_text_snapshot().clone();
        let mut source = snapshot.text();
        let (edits, inverse, projection) = plan_cell_edit(&source, &step.edit)?;
        let rewind = buffer.update(cx, |buffer, cx| {
            if let Some(rewind) = step.rewind
                && let Some(reverse) = rewind.apply(buffer, cx)
            {
                for (range, replacement) in edits.iter().rev() {
                    source.replace_range(range.clone(), replacement);
                }
                if buffer.text_for_range(0..buffer.len()).equals_str(&source) {
                    return Ok(reverse);
                }
                reverse
                    .apply(buffer, cx)
                    .context("Could not restore notebook history")?;
            }
            buffer.finalize_last_transaction();
            buffer
                .edit(edits, None, cx)
                .context("Could not edit notebook cells")?;
            let transaction = buffer
                .finalize_last_transaction()
                .context("Missing notebook structure transaction")?;
            Ok::<_, anyhow::Error>(BufferRewind::Undo(transaction.id))
        })?;
        let mut removed = None;
        match projection {
            Projection::Remove(index) => {
                let id = self.cell_order.remove(index);
                let (transactions, line_endings) = self
                    .cell_sources
                    .remove(&id)
                    .map(|history| (history.transactions, history.line_endings))
                    .unwrap_or_default();
                removed = self
                    .cell_map
                    .remove(&id)
                    .map(|cell| (cell, transactions, line_endings));
                self.display_slots.remove(&id);
                self.notebook_item
                    .update(cx, |item, _| item.fields.remove(index));
                self.cell_list.splice(index..index + 1, 0);
            }
            Projection::Insert {
                index,
                cell,
                fields,
            } => {
                let fields = anchor_fields(fields, buffer.read(cx));
                self.notebook_item
                    .update(cx, |item, _| item.fields.insert(index, fields));
                let source = cell.source().concat();
                let cell_id = match step.removed.take() {
                    Some((cell, transactions, line_endings)) => self.attach_cell(
                        cell,
                        &source,
                        Some((transactions, line_endings)),
                        window,
                        cx,
                    ),
                    None => {
                        let languages = self.project.read(cx).languages().clone();
                        let cell = Cell::load(
                            &cell,
                            &languages,
                            self.notebook_language.clone(),
                            window,
                            cx,
                        );
                        self.attach_cell(cell, &source, None, window, cx)
                    }
                };
                self.cell_order.insert(index, cell_id);
                self.cell_list.splice(index..index, 1);
            }
            Projection::Swap { index, left, right } => {
                self.cell_order.swap(index, index + 1);
                self.notebook_item.update(cx, |item, cx| {
                    let buffer = buffer.read(cx);
                    let fields: Vec<_> = item
                        .fields
                        .drain(index..index + 2)
                        .rev()
                        .zip([
                            (right.start, left.start),
                            (left.start, right.end - left.len()),
                        ])
                        .map(|(fields, (from, to))| {
                            let relocate = |anchor: language::Anchor| {
                                to + (anchor.to_offset(&snapshot) - from)
                            };
                            fields.map_offsets(|range| {
                                buffer.anchor_before(relocate(range.start))
                                    ..buffer.anchor_after(relocate(range.end))
                            })
                        })
                        .collect();
                    item.fields.splice(index..index, fields);
                });
                self.cell_list.splice(index..index + 2, 2);
            }
        }
        self.source_version = buffer.read(cx).version();
        self.selected_cell_index = step.after;
        if !self.cell_order.is_empty() {
            self.cell_list.scroll_to_reveal_item(step.after);
        }
        self.enter_command_mode(window, cx);
        Ok(StructureStep {
            edit: inverse,
            before: step.after,
            after: step.before,
            rewind: Some(rewind),
            removed,
        })
    }

    fn get_selected_cell(&self) -> Option<&Cell> {
        self.cell_order
            .get(self.selected_cell_index)
            .and_then(|cell_id| self.cell_map.get(cell_id))
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
                self.execute_cell(cell_id, false, window, cx);
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
                        self.execute_cell(cell_id, true, window, cx);
                        return;
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

        self.advance_in_command_mode(window, cx);
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

    fn select_next(
        &mut self,
        _: &menu::SelectNext,
        selection_mode: SelectionMode,
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

            if selection_mode == SelectionMode::SelectAndMove
                && let Some(cell) = self.get_selected_cell()
            {
                cell.move_to(MovementDirection::Start, window, cx);
            }

            cx.notify();
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        selection_mode: SelectionMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == 0 { 0 } else { index - 1 };
            self.set_selected_index(ix, true, window, cx);

            if selection_mode == SelectionMode::SelectAndMove
                && let Some(cell) = self.get_selected_cell()
            {
                cell.move_to(MovementDirection::End, window, cx);
            }

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

    fn has_results(&self, cx: &App) -> bool {
        self.cell_map
            .values()
            .any(|cell| matches!(cell, Cell::Code(cell) if cell.read(cx).has_results()))
    }

    fn confirm_clear_outputs(
        &mut self,
        _: &ClearOutputs,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.kernel_task.is_some() || !self.execution_requests.is_empty() {
            self.kernel_notice = Some(
                "Wait for execution to finish or stop the kernel before clearing outputs.".into(),
            );
            cx.notify();
            return;
        }
        if !self.has_results(cx) {
            return;
        }
        let version = self.source_version.clone();
        let answer = window.prompt(gpui::PromptLevel::Warning, "Clear all notebook outputs?", Some("This removes the saved outputs and execution counts from every code cell. Cell sources are kept."), &["Clear Outputs", "Cancel"], cx);
        cx.spawn(async move |this, cx| {
            if answer.await.ok() == Some(0) {
                this.update(cx, |editor, cx| {
                    editor.structure_error = editor
                        .clear_outputs(&version, cx)
                        .err()
                        .map(|error| error.to_string());
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn clear_outputs(&mut self, version: &clock::Global, cx: &mut Context<Self>) -> Result<()> {
        self.flush_cell_edits(cx)?;
        anyhow::ensure!(
            self.source_is_current(cx) && &self.source_version == version,
            "The notebook changed. Review it before clearing outputs."
        );
        anyhow::ensure!(
            self.kernel_task.is_none() && self.execution_requests.is_empty(),
            "Wait for execution to finish before clearing outputs."
        );
        let item = self.notebook_item.read(cx);
        let buffer = item.buffer.clone();
        anyhow::ensure!(
            buffer.read(cx).capability().editable(),
            "The notebook source is read-only."
        );
        let mut edits = Vec::new();
        for (index, id) in self.cell_order.iter().enumerate() {
            if let Some(Cell::Code(cell)) = self.cell_map.get(id)
                && cell.read(cx).has_results()
            {
                edits.extend(
                    item.fields
                        .get(index)
                        .context("Cell no longer exists")?
                        .edit_execution(&[], None)?,
                );
            }
        }
        buffer.update(cx, |buffer, cx| {
            buffer.finalize_last_transaction();
            buffer.edit(edits, None, cx);
            buffer.finalize_last_transaction();
        });
        self.source_version = buffer.read(cx).version();
        self.display_slots.clear();
        for cell in self.cell_map.values() {
            if let Cell::Code(cell) = cell {
                cell.update(cx, |cell, cx| {
                    cell.clear_results();
                    cx.notify();
                });
            }
        }
        Ok(())
    }

    fn render_cell_controls(&self, cx: &App) -> impl IntoElement {
        let on_click = |action: &'static dyn gpui::Action| {
            let notebook = self.focus_handle.clone();
            move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                notebook.dispatch_action(action, window, cx)
            }
        };
        h_flex()
            .flex_wrap()
            .px_3()
            .py_1()
            .gap_1()
            .child(
                Button::new("run-all-cells", "Run All")
                    .start_icon(Icon::new(IconName::PlayFilled))
                    .disabled(
                        self.cell_order.is_empty()
                            || self.kernel_task.is_some()
                            || !self.execution_requests.is_empty(),
                    )
                    .tooltip(Tooltip::for_action_title("Execute all code cells", &RunAll))
                    .on_click(on_click(&RunAll)),
            )
            .child(
                Button::new("clear-all-outputs", "Clear Outputs")
                    .disabled(
                        !self.has_results(cx)
                            || self.kernel_task.is_some()
                            || !self.execution_requests.is_empty(),
                    )
                    .tooltip(Tooltip::for_action_title(
                        "Clear all outputs and execution counts",
                        &ClearOutputs,
                    ))
                    .on_click(on_click(&ClearOutputs)),
            )
            .child(
                Button::new("new-code-cell", "Add Code")
                    .start_icon(Icon::new(IconName::Plus))
                    .tooltip(Tooltip::for_action_title("Add Code Cell", &AddCodeBlock))
                    .on_click(on_click(&AddCodeBlock)),
            )
            .child(
                Button::new("new-markdown-cell", "Add Markdown")
                    .start_icon(Icon::new(IconName::Plus))
                    .tooltip(Tooltip::for_action_title(
                        "Add Markdown Cell",
                        &AddMarkdownBlock,
                    ))
                    .on_click(on_click(&AddMarkdownBlock)),
            )
            .child(
                IconButton::new("move-cell-up", IconName::ArrowUp)
                    .disabled(self.selected_cell_index == 0)
                    .tooltip(Tooltip::for_action_title("Move Cell Up", &MoveCellUp))
                    .on_click(on_click(&MoveCellUp)),
            )
            .child(
                IconButton::new("move-cell-down", IconName::ArrowDown)
                    .disabled(self.selected_cell_index >= self.cell_order.len().saturating_sub(1))
                    .tooltip(Tooltip::for_action_title("Move Cell Down", &MoveCellDown))
                    .on_click(on_click(&MoveCellDown)),
            )
            .child(
                IconButton::new("delete-cell", IconName::Trash)
                    .disabled(self.cell_order.is_empty())
                    .tooltip(Tooltip::for_action_title("Delete Cell", &DeleteCell))
                    .on_click(on_click(&DeleteCell)),
            )
    }

    fn render_kernel_status_bar(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let status = self.kernel.status();
        let name = self
            .kernel_specification
            .as_ref()
            .map(KernelSpecification::name)
            .unwrap_or_else(|| "Select Kernel".into());
        let notice = self
            .kernel_notice
            .clone()
            .unwrap_or_else(|| match &self.kernel {
                Kernel::Shutdown => "Not running".into(),
                Kernel::ErroredLaunch(error) => error.clone(),
                _ => status.to_string(),
            });
        h_flex()
            .w_full()
            .px_3()
            .py_1()
            .gap_2()
            .bg(cx.theme().colors().status_bar_background)
            .child(
                div().min_w_0().child(
                    Button::new("kernel-selector", name.clone())
                        .full_width()
                        .truncate(true)
                        .tooltip(Tooltip::text(name))
                        .label_size(LabelSize::Small)
                        .disabled(self.kernel_task.is_some() || !self.project.read(cx).is_local())
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.choose_or_start_kernel(None, window, cx);
                        })),
                ),
            )
            .when(Self::remote_enabled(cx), |bar| {
                bar.child(
                    Button::new("remote-kernel-selector", "Remote Kernels…")
                        .label_size(LabelSize::Small)
                        .disabled(self.kernel_task.is_some() || !self.execution_requests.is_empty())
                        .on_click(
                            cx.listener(|this, _, window, cx| {
                                this.choose_remote_kernel(window, cx)
                            }),
                        ),
                )
            })
            .child(
                div()
                    .id("kernel-status")
                    .min_w_0()
                    .when(
                        self.kernel_notice.is_none()
                            && !matches!(self.kernel, Kernel::ErroredLaunch(_)),
                        |this| this.flex_shrink_0(),
                    )
                    .child(
                        Label::new(notice.lines().next().unwrap_or_default().to_owned())
                            .size(LabelSize::Small)
                            .truncate(),
                    )
                    .tooltip(Tooltip::text(notice)),
            )
            .when(matches!(self.kernel, Kernel::RunningKernel(_)), |bar| {
                bar.child(
                    IconButton::new("stop-kernel", IconName::Power)
                        .tooltip(Tooltip::text("Stop Kernel"))
                        .on_click(cx.listener(|this, _, _, cx| this.stop_kernel(cx))),
                )
                .child(
                    IconButton::new("interrupt-kernel", IconName::Stop)
                        .disabled(self.execution_requests.is_empty())
                        .tooltip(|_, cx| {
                            Tooltip::for_action("Interrupt Kernel", &InterruptKernel, cx)
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.interrupt_kernel(&InterruptKernel, window, cx)
                        })),
                )
            })
    }

    fn start_mouse_autoscroll(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.modifiers.modified() || self.cell_list.max_offset_for_scrollbar().y <= px(0.) {
            return;
        }
        if !self.focus_handle.contains_focused(window, cx) {
            self.focus_handle.focus(window, cx);
        }
        let window_handle = window.window_handle();
        let cancel_on_escape =
            cx.listener(move |this, event: &gpui::KeystrokeEvent, window, cx| {
                if window.window_handle() == window_handle
                    && event.keystroke.key == "escape"
                    && this.focus_handle.contains_focused(window, cx)
                {
                    this.mouse_autoscroll = None;
                    cx.stop_propagation();
                    cx.notify();
                }
            });
        let escape = cx.intercept_keystrokes(cancel_on_escape);
        let origin = event.position.y;
        self.mouse_autoscroll = Some(cx.spawn_in(window, async move |this, cx| {
            let _escape = escape;
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                let keep_scrolling = this.update_in(cx, |this, window, cx| {
                    if !window.is_window_active() || !this.focus_handle.contains_focused(window, cx)
                    {
                        this.mouse_autoscroll = None;
                        cx.notify();
                        return false;
                    }
                    let distance = window.mouse_position().y - origin;
                    if distance.abs() > px(6.) {
                        this.cell_list.scroll_by(distance * 0.12);
                        cx.notify();
                    }
                    true
                });
                if !matches!(keep_scrolling, Ok(true)) {
                    break;
                }
            }
        }));
        window.prevent_default();
        cx.stop_propagation();
        cx.notify();
    }

    fn cell_list(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        let cells = list(self.cell_list.clone(), move |index, _, cx| {
            view.update(cx, |this, cx| match this.cell(index) {
                Some(cell) => this.render_cell(index, cell, cx).into_any_element(),
                None => div().into_any_element(),
            })
        })
        .size_full();
        div()
            .id("notebook-page")
            .relative()
            .flex_1()
            .min_h_0()
            .overflow_hidden()
            .child(cells)
            .when(
                EditorSettings::get_global(cx).scrollbar.axes.vertical,
                |div| {
                    div.custom_scrollbars(
                        Scrollbars::for_settings::<EditorSettingsScrollbarProxy>()
                            .tracked_scroll_handle(&self.cell_list)
                            .notify_content(),
                        window,
                        cx,
                    )
                },
            )
            .when(
                cx.platform_info().os == gpui::OperatingSystem::Windows,
                |div| {
                    div.on_mouse_down(
                        MouseButton::Middle,
                        cx.listener(Self::start_mouse_autoscroll),
                    )
                },
            )
    }

    fn cell_position(&self, index: usize) -> CellPosition {
        match index {
            0 => CellPosition::First,
            index if index == self.cell_count() - 1 => CellPosition::Last,
            _ => CellPosition::Middle,
        }
    }

    fn render_cell(&self, index: usize, cell: &Cell, cx: &mut Context<Self>) -> impl IntoElement {
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
        use editor::actions::{Redo, Undo};

        let mut key_context = KeyContext::new_with_defaults(cx.platform_info().os);
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
            .on_action(cx.listener(Self::run_all))
            .on_action(cx.listener(Self::confirm_clear_outputs))
            .on_action(
                cx.listener(|this, _: &Run, window, cx| this.run_current_cell(&Run, window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.run_and_advance(action, window, cx)),
            )
            .when(
                self.get_selected_cell()
                    .and_then(|cell| cell.editor(cx))
                    .is_some(),
                |this| this.on_action(cx.listener(Self::enter_edit_mode)),
            )
            .on_action(cx.listener(|this, action, window, cx| {
                this.handle_enter_command_mode(action, window, cx)
            }))
            .on_action(cx.listener(|this, _: &AddCodeBlock, window, cx| {
                this.change_structure(CellChange::InsertCode, window, cx)
            }))
            .on_action(cx.listener(|this, _: &AddMarkdownBlock, window, cx| {
                this.change_structure(CellChange::InsertMarkdown, window, cx)
            }))
            .on_action(cx.listener(|this, _: &MoveCellUp, window, cx| {
                this.change_structure(CellChange::MoveUp, window, cx)
            }))
            .on_action(cx.listener(|this, _: &MoveCellDown, window, cx| {
                this.change_structure(CellChange::MoveDown, window, cx)
            }))
            .on_action(cx.listener(|this, _: &DeleteCell, window, cx| {
                this.change_structure(CellChange::Delete, window, cx)
            }))
            .when(self.notebook_mode == NotebookMode::Command, |this| {
                this.on_action(
                    cx.listener(|this, _: &Undo, window, cx| {
                        this.undo_structure(false, window, cx)
                    }),
                )
                .on_action(
                    cx.listener(|this, _: &Redo, window, cx| this.undo_structure(true, window, cx)),
                )
            })
            .on_action(cx.listener(|this, action, window, cx| {
                this.select_next(action, SelectionMode::SelectOnly, window, cx)
            }))
            .on_action(cx.listener(|this, action, window, cx| {
                this.select_previous(action, SelectionMode::SelectOnly, window, cx)
            }))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(|this, _: &MoveDown, window, cx| {
                this.select_next(
                    &Default::default(),
                    SelectionMode::SelectAndMove,
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &MoveUp, window, cx| {
                this.select_previous(
                    &Default::default(),
                    SelectionMode::SelectAndMove,
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &NotebookMoveDown, window, cx| {
                let Some(cell) = this.get_selected_cell() else {
                    return;
                };

                let Some(editor) = cell.editor(cx).cloned() else {
                    return;
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
                    this.select_next(
                        &Default::default(),
                        SelectionMode::SelectAndMove,
                        window,
                        cx,
                    );
                } else {
                    editor.update(cx, |editor, cx| {
                        editor.move_down(&Default::default(), window, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &NotebookMoveUp, window, cx| {
                let Some(cell) = this.get_selected_cell() else {
                    return;
                };

                let Some(editor) = cell.editor(cx).cloned() else {
                    return;
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
                    this.select_previous(
                        &Default::default(),
                        SelectionMode::SelectAndMove,
                        window,
                        cx,
                    );
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
            .when_some(
                self.structure_error
                    .as_ref()
                    .or_else(|| self.sync_errors.values().next())
                    .cloned(),
                |container, error| container.child(Label::new(error).color(Color::Error)),
            )
            .child(self.render_cell_controls(cx))
            .child(if self.cell_order.is_empty() {
                div()
                    .flex_1()
                    .p_4()
                    .child(Label::new(
                        "This notebook is empty. Add a code or Markdown cell to get started.",
                    ))
                    .into_any_element()
            } else {
                self.cell_list(window, cx).into_any_element()
            })
            .child(self.render_kernel_status_bar(window, cx))
            .when(self.mouse_autoscroll.is_some(), |notebook| {
                notebook.child(
                    div()
                        .absolute()
                        .inset_0()
                        .occlude()
                        .cursor(gpui::CursorStyle::ResizeUpDown)
                        .on_any_mouse_down(cx.listener(|this, _, window, cx| {
                            this.mouse_autoscroll = None;
                            window.prevent_default();
                            cx.stop_propagation();
                            cx.notify();
                        }))
                        .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                            this.mouse_autoscroll = None;
                            cx.notify();
                        })),
                )
            })
    }
}

impl Focusable for NotebookEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub struct NotebookItem {
    project_path: ProjectPath,
    languages: Arc<LanguageRegistry>,
    pub(super) buffer: Entity<language::Buffer>,
    fields: Vec<CellFields<language::Anchor>>,
    notebook: nbformat::v4::Notebook,
    id: ProjectEntryId,
}

impl project::ProjectItem for NotebookItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
        let path = path.clone();
        let project = project.clone();

        if Self::supports_path(project.read(cx), &path, cx) {
            Some(cx.spawn(async move |cx| {
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(path.clone(), cx))
                    .await?;
                let (source, version) =
                    buffer.read_with(cx, |buffer, _| (buffer.as_rope().clone(), buffer.version()));
                let notebook = cx
                    .background_spawn(
                        async move { super::document::parse_view(&source.to_string()) },
                    )
                    .await?;

                cx.update(|cx| {
                    anyhow::ensure!(
                        buffer.read(cx).version() == version,
                        "The notebook changed while opening"
                    );
                    Self::for_buffer(&project, path, buffer, notebook, cx)
                        .context("Entry not found")
                })
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
        // TODO: Track if notebook metadata or structure has changed
        false
    }
}

impl NotebookItem {
    pub(super) fn supports_path(project: &Project, path: &ProjectPath, cx: &App) -> bool {
        project
            .absolute_path(path, cx)
            .as_ref()
            .and_then(|path| path.extension())
            .is_some_and(|extension| extension.eq_ignore_ascii_case(NOTEBOOK_EXTENSION))
            || path
                .path
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case(NOTEBOOK_EXTENSION))
    }

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
        let language_name = self.language_name().unwrap_or_else(|| "Python".into());
        let languages = self.languages.clone();
        async move { languages.language_for_name(&language_name).await.ok() }
    }

    fn for_buffer(
        project: &Entity<Project>,
        path: ProjectPath,
        buffer: Entity<language::Buffer>,
        parsed: ParsedNotebook,
        cx: &mut App,
    ) -> Option<Entity<Self>> {
        let id = project.read(cx).entry_for_path(&path, cx)?.id;
        let languages = project.read(cx).languages().clone();
        let fields = parsed
            .fields
            .into_iter()
            .map(|fields| anchor_fields(fields, buffer.read(cx)))
            .collect();
        Some(cx.new(|_| Self {
            project_path: path,
            id,
            languages,
            buffer,
            fields,
            notebook: parsed.notebook,
        }))
    }
}

fn line_endings(cell: &language::Buffer, source: &str) -> LineEndings {
    let default_ending = cell.line_ending().as_str();
    let mut line_endings = Vec::new();
    let mut offset = 0;
    let mut rest = source;
    while let Some(index) = rest.find(['\r', '\n']) {
        let ending = if rest[index..].starts_with("\r\n") {
            "\r\n"
        } else if rest[index..].starts_with('\r') {
            "\r"
        } else {
            "\n"
        };
        offset += index;
        if ending != default_ending {
            line_endings.push((cell.anchor_after(offset), ending));
        }
        offset += 1;
        rest = &rest[index + ending.len()..];
    }
    line_endings
}

fn anchor_fields(
    fields: CellFields<usize>,
    buffer: &language::Buffer,
) -> CellFields<language::Anchor> {
    fields.map_offsets(|range| buffer.anchor_before(range.start)..buffer.anchor_after(range.end))
}

impl KernelSession for NotebookKernelSession {
    const IS_NOTEBOOK: bool = true;

    fn check_launch(&self, cx: &mut Context<Self>) -> Result<()> {
        let id = cx.entity_id();
        self.0.update(cx, |editor, cx| {
            anyhow::ensure!(
                editor
                    .kernel_session
                    .as_ref()
                    .is_some_and(|session| session.entity_id() == id),
                "Kernel startup was canceled"
            );
            editor.check_execution(cx)
        })?
    }

    fn route(&mut self, message: &JupyterMessage, window: &mut Window, cx: &mut Context<Self>) {
        let id = cx.entity_id();
        self.0
            .update(cx, |editor, cx| {
                if editor
                    .kernel_session
                    .as_ref()
                    .is_none_or(|session| session.entity_id() != id)
                {
                    return;
                }
                if let Err(error) = editor.route_message(message, window, cx) {
                    editor.kernel_notice = Some(error.to_string());
                }
                cx.notify();
            })
            .ok();
    }

    fn kernel_errored(&mut self, error: String, cx: &mut Context<Self>) {
        let id = cx.entity_id();
        self.0
            .update(cx, |editor, cx| {
                if editor
                    .kernel_session
                    .as_ref()
                    .is_some_and(|session| session.entity_id() == id)
                {
                    editor.stop_kernel(cx);
                    editor.kernel_notice = Some(error);
                    cx.notify();
                }
            })
            .ok();
    }
}

impl NotebookEditor {
    fn update_display(
        &mut self,
        update: &jupyter_protocol::UpdateDisplayData,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let Some(id) = &update.transient.display_id else {
            return Ok(());
        };
        self.check_execution(cx)?;
        let data = serde_json::to_value(&update.data)?;
        let metadata = serde_json::Value::Object(update.metadata.clone());
        let targets: Vec<_> = self
            .display_slots
            .iter()
            .filter_map(|(cell, slots)| {
                let indices: Vec<_> = slots
                    .iter()
                    .filter(|(display, _)| display == id)
                    .map(|(_, index)| *index)
                    .collect();
                (!indices.is_empty()).then(|| (cell.clone(), indices))
            })
            .collect();
        for (cell_id, indices) in targets {
            let Some(Cell::Code(cell)) = self.cell_map.get(&cell_id).cloned() else {
                continue;
            };
            let replace = |outputs: &mut [serde_json::Value]| -> Result<()> {
                for index in &indices {
                    let output = outputs
                        .get_mut(*index)
                        .context("Displayed output no longer exists")?;
                    output["data"] = data.clone();
                    output["metadata"] = metadata.clone();
                }
                Ok(())
            };
            if let Some(run) = self
                .execution_requests
                .values_mut()
                .find(|run| run.cell_id == cell_id && run.started())
            {
                replace(&mut run.outputs)?;
            } else {
                if cell.read(cx).has_stale_outputs(cx) {
                    continue;
                }
                let index = self
                    .cell_order
                    .iter()
                    .position(|id| *id == cell_id)
                    .context("Displayed cell no longer exists")?;
                let buffer = self.notebook_item.read(cx).buffer.read(cx).snapshot();
                self.patch_cell(index, cx, |fields| {
                    let range = &fields
                        .execution
                        .as_ref()
                        .context("Only code cells have output")?
                        .outputs;
                    let text: String = buffer.text_for_range(range.clone()).collect();
                    let mut outputs: Vec<serde_json::Value> = serde_json::from_str(&text)?;
                    replace(&mut outputs)?;
                    Ok(vec![(range.clone(), serde_json::to_string(&outputs)?)])
                })?;
            }
            cell.update(cx, |cell, cx| {
                cell.update_display(&indices, &update.data, window, cx)
            });
        }
        Ok(())
    }

    fn route_message(
        &mut self,
        message: &JupyterMessage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let JupyterMessageContent::Status(status) = &message.content {
            self.kernel.set_execution_state(&status.execution_state);
        }
        if let JupyterMessageContent::UpdateDisplayData(update) = &message.content {
            return self.update_display(update, window, cx);
        }
        let Some(id) = message.parent_header.as_ref().map(|header| &header.msg_id) else {
            return Ok(());
        };
        let Some(run) = self.execution_requests.get_mut(id) else {
            return Ok(());
        };
        let Some(Cell::Code(cell)) = self.cell_map.get(&run.cell_id).cloned() else {
            return Ok(());
        };
        if !run.started()
            && match &message.content {
                JupyterMessageContent::ExecuteInput(_)
                | JupyterMessageContent::ClearOutput(_)
                | JupyterMessageContent::StreamContent(_)
                | JupyterMessageContent::DisplayData(_)
                | JupyterMessageContent::ExecuteResult(_)
                | JupyterMessageContent::ErrorOutput(_) => true,
                JupyterMessageContent::ExecuteReply(reply) => {
                    reply.status != jupyter_protocol::ReplyStatus::Aborted
                }
                _ => false,
            }
        {
            let previous = cell.update(cx, |cell, cx| {
                let previous = cell.start_execution(run.source.clone());
                cx.notify();
                previous
            });
            run.previous = Some((
                previous,
                self.display_slots.remove(&run.cell_id).unwrap_or_default(),
            ));
        }
        match &message.content {
            JupyterMessageContent::Status(status) => {
                if status.execution_state == jupyter_protocol::ExecutionState::Idle {
                    run.idle = true;
                }
            }
            JupyterMessageContent::ExecuteReply(reply) => {
                run.reply = Some(reply.clone());
            }
            JupyterMessageContent::ClearOutput(clear) => {
                run.clear_on_output = clear.wait;
                if !clear.wait {
                    self.display_slots.remove(&run.cell_id);
                    run.outputs.clear();
                    cell.update(cx, |cell, cx| {
                        cell.clear_outputs();
                        cx.notify();
                    });
                }
            }
            JupyterMessageContent::ExecuteInput(_) => {
                cell.update(cx, |cell, cx| cell.handle_message(message, window, cx))
            }
            JupyterMessageContent::StreamContent(_)
            | JupyterMessageContent::DisplayData(_)
            | JupyterMessageContent::ExecuteResult(_)
            | JupyterMessageContent::ErrorOutput(_) => {
                let mut output = serde_json::to_value(&message.content)?;
                let fields = output.as_object_mut().context("Invalid kernel output")?;
                let transient = fields.remove("transient");
                fields.insert("output_type".into(), message.content.message_type().into());
                let clear = std::mem::take(&mut run.clear_on_output);
                if clear {
                    self.display_slots.remove(&run.cell_id);
                    run.outputs.clear();
                }
                if let Some(id) = transient
                    .as_ref()
                    .and_then(|value| value["display_id"].as_str())
                {
                    self.display_slots
                        .entry(run.cell_id.clone())
                        .or_default()
                        .push((id.to_owned(), run.outputs.len()));
                }
                run.outputs.push(output);
                cell.update(cx, |cell, cx| {
                    if clear {
                        cell.clear_outputs();
                    }
                    cell.handle_message(message, window, cx);
                });
            }
            _ => return Ok(()),
        }
        let Some(reply) = run.reply.as_ref() else {
            return Ok(());
        };
        if !run.idle && reply.status != jupyter_protocol::ReplyStatus::Aborted {
            return Ok(());
        }
        let mut run = self
            .execution_requests
            .remove(id)
            .context("Cell execution not found")?;
        let succeeded = run
            .reply
            .as_ref()
            .is_some_and(|reply| reply.status == jupyter_protocol::ReplyStatus::Ok);
        let result = self.commit_execution(&mut run, &cell, window, cx);
        cell.update(cx, |cell, cx| {
            match &result {
                Ok(count) => {
                    cell.set_execution_count(*count);
                    cell.finish_execution();
                }
                Err(_) => {
                    let previous = run.previous.map(|(previous, slots)| {
                        self.display_slots.remove(&run.cell_id);
                        if !slots.is_empty() {
                            self.display_slots.insert(run.cell_id, slots);
                        }
                        previous
                    });
                    cell.cancel_execution(previous);
                }
            }
            cx.notify();
        });
        if result.is_err() || !succeeded {
            if self.remaining_runs.take().is_some() {
                self.kernel_notice = Some("Run All stopped at the failed cell.".into());
            }
        } else if let Some(next) = self.remaining_runs.as_mut().and_then(VecDeque::pop_front) {
            self.start_run(next, window, cx);
        } else {
            self.remaining_runs = None;
        }
        result.map(|_| ())
    }

    fn commit_execution(
        &mut self,
        run: &mut CellExecution,
        cell: &Entity<CodeCell>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<i32> {
        let reply = run.reply.take().context("Missing kernel reply")?;
        anyhow::ensure!(
            reply.status != jupyter_protocol::ReplyStatus::Aborted,
            "The kernel aborted this cell before it ran. Its previous results are unchanged."
        );
        self.check_execution(cx)?;
        let index = self
            .cell_order
            .iter()
            .position(|id| id == &run.cell_id)
            .context("Executed cell is no longer in the notebook")?;
        let count = i32::try_from(reply.execution_count.0)?;
        if reply.status == jupyter_protocol::ReplyStatus::Error
            && !run
                .outputs
                .iter()
                .any(|output| output["output_type"] == "error")
        {
            let error = reply
                .error
                .context("The kernel reported an execution error without details")?;
            let output = jupyter_protocol::ErrorOutput {
                ename: error.ename,
                evalue: error.evalue,
                traceback: error.traceback,
            };
            run.outputs.push(serde_json::json!({"output_type":"error", "ename":output.ename, "evalue":output.evalue, "traceback":output.traceback}));
            cell.update(cx, |cell, cx| {
                cell.handle_message(&output.into(), window, cx)
            });
        }
        self.persist_execution(index, &run.source, &run.outputs, Some(i64::from(count)), cx)?;
        Ok(count)
    }
}

pub(super) fn upgrade_notebook(
    notebook: nbformat::Notebook,
) -> anyhow::Result<nbformat::v4::Notebook> {
    match notebook {
        nbformat::Notebook::V4(notebook) => Ok(notebook),
        nbformat::Notebook::V4QuirksMode(notebook) => {
            log::warn!("Repairing notebook cell IDs: {:?}", notebook.quirks());
            Ok(notebook.repair())
        }
        nbformat::Notebook::Legacy(notebook) => nbformat::upgrade_legacy_notebook(notebook),
        nbformat::Notebook::V3(notebook) => {
            for worksheet in notebook.worksheets.iter().flatten() {
                for cell in &worksheet.cells {
                    if let nbformat::v3::Cell::Heading { level, .. } = cell {
                        anyhow::ensure!((1..=6).contains(level), "Invalid notebook heading level");
                    }
                }
            }
            nbformat::upgrade_v3_notebook(notebook)
        }
        _ => anyhow::bail!("Unsupported notebook format"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReplStore;
    use gpui::TestAppContext;
    use project::{FakeFs, Project, ProjectItem as _};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use util::rel_path::rel_path;

    const NOTEBOOK_WITH_ONE_CODE_CELL: &str = r#"{
        "metadata": {
            "kernelspec": {
                "display_name": "Python 3",
                "language": "python",
                "name": "python3"
            },
            "language_info": {
                "name": "python"
            }
        },
        "nbformat": 4,
        "nbformat_minor": 5,
        "cells": [
            {
                "cell_type": "code",
                "id": "cell-one",
                "metadata": {},
                "execution_count": null,
                "outputs": [],
                "source": ["print('hello')"]
            }
        ]
    }"#;

    #[test]
    fn test_upgrade_notebook_repairs_missing_cell_ids() -> anyhow::Result<()> {
        let mut value: serde_json::Value = serde_json::from_str(NOTEBOOK_WITH_ONE_CODE_CELL)?;
        value["cells"][0]
            .as_object_mut()
            .context("cell is not an object")?
            .remove("id");
        let notebook = nbformat::parse_notebook(&serde_json::to_string(&value)?)?;
        assert!(matches!(notebook, nbformat::Notebook::V4QuirksMode(_)));
        let repaired = upgrade_notebook(notebook)?;
        let serialized = nbformat::serialize_notebook(&nbformat::Notebook::V4(repaired))?;
        assert!(matches!(
            nbformat::parse_notebook(&serialized)?,
            nbformat::Notebook::V4(_)
        ));
        let saved: serde_json::Value = serde_json::from_str(&serialized)?;
        assert_eq!(saved["cells"][0]["source"], value["cells"][0]["source"]);
        assert!(
            saved["cells"][0]["id"]
                .as_str()
                .is_some_and(|id| !id.is_empty())
        );
        Ok(())
    }

    #[test]
    fn test_upgrade_notebook_preserves_cell_ids() -> anyhow::Result<()> {
        let notebook = upgrade_notebook(nbformat::parse_notebook(NOTEBOOK_WITH_ONE_CODE_CELL)?)?;
        let value = serde_json::to_value(notebook)?;
        assert_eq!(value["cells"][0]["id"], "cell-one");
        Ok(())
    }

    async fn kernel_test_editor<'a>(
        notebook: &str,
        cx: &'a mut TestAppContext,
    ) -> (Entity<NotebookEditor>, &'a mut gpui::VisualTestContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            project::trusted_worktrees::init(Default::default(), cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/notebooks"), json!({ "test.ipynb": notebook }))
            .await;
        let project =
            Project::test_with_worktree_trust(fs, [path!("/notebooks").as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(
                    &project,
                    &ProjectPath {
                        worktree_id,
                        path: rel_path("test.ipynb").into(),
                    },
                    cx,
                )
                .expect("open notebook")
            })
            .await
            .expect("notebook should parse");
        let cx = cx.add_empty_window();
        let editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project.clone(), notebook_item, window, cx))
        });
        (editor, cx)
    }

    fn trust_notebook(editor: &Entity<NotebookEditor>, cx: &mut gpui::VisualTestContext) {
        editor.update(cx, |editor, cx| {
            let store = editor.project.read(cx).worktree_store();
            let worktree_id = editor.worktree_id;
            TrustedWorktrees::try_get_global(cx)
                .unwrap()
                .update(cx, |trust, cx| {
                    trust.trust(
                        &store,
                        [project::trusted_worktrees::PathTrust::Worktree(worktree_id)]
                            .into_iter()
                            .collect(),
                        cx,
                    );
                });
        });
    }

    #[gpui::test]
    async fn scroll_notebook_page_with_mouse(cx: &mut TestAppContext) {
        use crate::notebook::{
            NotebookPreview,
            preview::preview_tests::{notebook_workspace, open_file},
        };
        use gpui::{
            Modifiers, OperatingSystem, PlatformInfo, PlatformTarget, UpdateGlobal as _, point,
        };
        cx.set_platform_info(PlatformInfo {
            os: OperatingSystem::Windows,
            target: PlatformTarget::Native,
        });
        let cells: Vec<_> = (0..100).map(|ix| json!({
            "cell_type": "raw", "id": format!("cell-{ix}"), "source": format!("Row {ix}"), "metadata": {}
        })).collect();
        let file =
            json!({"nbformat":4, "nbformat_minor":5, "metadata":{}, "cells": cells}).to_string();
        let (_, workspace, cx) = notebook_workspace(json!({"scroll.ipynb": file}), cx).await;
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.scrollbar.get_or_insert_default().show =
                        Some(settings::ShowScrollbar::Always);
                });
            })
        });
        open_file("scroll.ipynb", &workspace, cx).await;
        let notebook = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item_as::<NotebookPreview>(cx)
                .unwrap()
                .read(cx)
                .notebook
                .clone()
                .unwrap()
        });
        let list = notebook.read_with(cx, |notebook, _| notebook.cell_list.clone());
        let bounds = list.viewport_bounds();
        assert!(list.max_offset_for_scrollbar().y > px(0.));
        let origin = bounds.center();
        let down = point(origin.x, origin.y + px(100.));
        let modifiers = Modifiers::none();
        cx.simulate_mouse_down(origin, MouseButton::Middle, modifiers);
        cx.simulate_mouse_up(origin, MouseButton::Middle, modifiers);
        cx.simulate_mouse_move(down, None, modifiers);
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        assert!(
            list.scroll_px_offset_for_scrollbar().y < px(0.),
            "middle-click then move scrolls the page"
        );
        cx.simulate_keystrokes("escape");
        let stopped = list.logical_scroll_top();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        assert_eq!(list.logical_scroll_top(), stopped, "Escape stops scrolling");
        cx.simulate_event(gpui::ScrollEvent {
            position: origin,
            delta: gpui::ScrollDelta::Pixels(point(px(0.), list.max_offset_for_scrollbar().y)),
            ..Default::default()
        });
        assert_eq!(
            list.scroll_px_offset_for_scrollbar().y,
            px(0.),
            "wheel returns to the top"
        );
        let thumb = point(bounds.right() - px(7.), bounds.top() + px(10.));
        let bottom = point(thumb.x, bounds.bottom() - px(1.));
        cx.simulate_mouse_down(thumb, MouseButton::Left, modifiers);
        assert!(
            list.is_scrollbar_dragging(),
            "the page thumb captured the press"
        );
        cx.simulate_mouse_move(bottom, MouseButton::Left, modifiers);
        cx.simulate_mouse_up(bottom, MouseButton::Left, modifiers);
        assert!(
            list.bounds_for_item(99)
                .is_some_and(|last| last.bottom() <= bounds.bottom() + px(1.)),
            "dragging to the bottom reveals the last cell"
        );
        notebook.read_with(cx, |notebook, _| {
            assert_eq!(notebook.selected_cell_index, 0);
            assert!(matches!(notebook.notebook_mode, NotebookMode::Command));
            assert!(notebook.execution_requests.is_empty());
        });
    }

    /// When the configured interpreter doesn't exist (e.g. Python isn't installed),
    /// running a cell must not leave it stuck in the executing state. It should
    /// instead surface the kernel launch error as an error output on the cell.
    #[gpui::test]
    async fn test_run_cell_with_missing_interpreter_shows_error(cx: &mut TestAppContext) {
        let (editor, cx) = kernel_test_editor(NOTEBOOK_WITH_ONE_CODE_CELL, cx).await;
        editor.update_in(cx, |editor, window, cx| {
            assert!(editor.kernel_specification.is_none());
            assert!(editor.kernel_task.is_none());
            editor.run_current_cell(&Run, window, cx);
            assert!(
                editor
                    .kernel_notice
                    .as_deref()
                    .unwrap()
                    .contains("Restricted Mode")
            );
            assert!(editor.kernel_task.is_none());
        });
        trust_notebook(&editor, cx);
        cx.executor().allow_parking();
        let launch = editor.update_in(cx, |editor, window, cx| {
            let spec = LocalKernelSpecification::for_notebook_python(
                path!("/nonexistent/python3").into(),
                "Missing Python".into(),
            )
            .unwrap();
            editor.select_kernel(KernelSpecification::Jupyter(spec), cx);
            assert!(editor.kernel_task.is_none());
            assert!(matches!(editor.kernel, Kernel::Shutdown));
            editor.run_current_cell(&Run, window, cx);
            editor
                .kernel_task
                .clone()
                .expect("explicit run should start the kernel")
        });
        cx.condition(&editor, |editor, _| editor.kernel_specification.is_none())
            .await;
        let (title, detail) = cx.pending_prompt().expect("setup offer");
        assert_eq!(title, "Set up Python for notebooks?");
        assert!(detail.contains("Could not check Python at"));
        cx.simulate_prompt_answer("Cancel");
        launch.await;
        editor.read_with(cx, |editor, cx| {
            let Some(Cell::Code(cell)) = editor.cell(0) else {
                panic!("expected a code cell")
            };
            let cell = cell.read(cx);
            assert!(!cell.is_executing());
            let [crate::outputs::Output::ErrorOutput(error)] = cell.output_views() else {
                panic!("expected a kernel error")
            };
            assert_eq!(error.ename, "Kernel Error");
            assert!(
                error
                    .traceback
                    .read(cx)
                    .full_text(cx)
                    .contains("Could not check Python at")
            );
            assert_eq!(
                editor.notebook_item.read(cx).buffer.read(cx).text(),
                NOTEBOOK_WITH_ONE_CODE_CELL
            );
        });
    }

    #[gpui::test]
    async fn run_all_sends_one_request_at_a_time_and_stops_on_failure(cx: &mut TestAppContext) {
        use crate::kernels::RunningKernel;
        use futures::channel::mpsc;
        use jupyter_protocol::{ExecutionState, KernelInfoReply, ReplyStatus};
        use std::path::PathBuf;

        #[derive(Debug)]
        struct RecordingKernel(mpsc::Sender<JupyterMessage>);
        impl RunningKernel for RecordingKernel {
            fn request_tx(&self) -> mpsc::Sender<JupyterMessage> {
                self.0.clone()
            }
            fn stdin_tx(&self) -> mpsc::Sender<JupyterMessage> {
                self.0.clone()
            }
            fn working_directory(&self) -> &PathBuf {
                unreachable!()
            }
            fn execution_state(&self) -> &ExecutionState {
                &ExecutionState::Idle
            }
            fn set_execution_state(&mut self, _: ExecutionState) {}
            fn kernel_info(&self) -> Option<&KernelInfoReply> {
                None
            }
            fn set_kernel_info(&mut self, _: KernelInfoReply) {}
            fn force_shutdown(&mut self, _: &mut Window, _: &mut App) -> Task<Result<()>> {
                Task::ready(Ok(()))
            }
            fn kill(&mut self) {}
        }
        let code = |id, source| json!({"cell_type":"code","id":id,"source":source,"metadata":{},"execution_count":null,"outputs":[]});
        let raw = json!({"nbformat":4,"nbformat_minor":5,"metadata":{},"cells":[
            code("first", "x = 21"),
            {"cell_type":"markdown","id":"note","source":"# Heading","metadata":{}},
            code("second", "x * 2"),
            code("third", "not_run()")
        ]})
        .to_string();
        let (editor, cx) = kernel_test_editor(&raw, cx).await;
        editor.update_in(cx, |editor, window, cx| {
            editor.run_all(&RunAll, window, cx);
            assert!(
                editor
                    .kernel_notice
                    .as_deref()
                    .unwrap()
                    .contains("Restricted Mode")
            );
            assert!(editor.kernel_task.is_none());
        });
        trust_notebook(&editor, cx);
        editor.update_in(cx, |editor, window, cx| {
            let (sender, mut received) = mpsc::channel(8);
            editor.kernel = Kernel::RunningKernel(Box::new(RecordingKernel(sender)));
            editor.run_all(&RunAll, window, cx);
            editor.run_all(&RunAll, window, cx);
            let first = received.try_recv().unwrap();
            assert!(matches!(&first.content, JupyterMessageContent::ExecuteRequest(r) if r.code == "x = 21"));
            editor.route_message(&jupyter_protocol::ExecuteReply::default().as_child_of(&first), window, cx).unwrap();
            assert!(received.try_recv().is_err());
            editor.route_message(&jupyter_protocol::Status::idle().as_child_of(&first), window, cx).unwrap();
            let second = received.try_recv().unwrap();
            assert!(matches!(&second.content, JupyterMessageContent::ExecuteRequest(r) if r.code == "x * 2"));
            editor.confirm_clear_outputs(&ClearOutputs, window, cx);
            assert!(editor.kernel_notice.as_deref().unwrap().contains("stop the kernel"));
            for message in [
                jupyter_protocol::ErrorOutput { ename:"ExampleError".into(), ..Default::default() }.as_child_of(&second),
                jupyter_protocol::ExecuteReply { status:ReplyStatus::Error, execution_count:2.into(), ..Default::default() }.as_child_of(&second),
                jupyter_protocol::Status::idle().as_child_of(&second),
            ] {
                editor.route_message(&message, window, cx).unwrap();
            }
            assert!(received.try_recv().is_err());
            let saved: serde_json::Value = serde_json::from_str(&editor.notebook_item.read(cx).buffer.read(cx).text()).unwrap();
            assert_eq!(saved["cells"][2]["outputs"][0]["ename"], "ExampleError");
            assert!(saved["cells"][3]["execution_count"].is_null());

            editor.run_all(&RunAll, window, cx);
            let first = received.try_recv().unwrap();
            editor.interrupt_kernel(&InterruptKernel, window, cx);
            assert!(matches!(received.try_recv().unwrap().content, JupyterMessageContent::InterruptRequest(_)));
            editor.route_message(&jupyter_protocol::Status::idle().as_child_of(&first), window, cx).unwrap();
            editor.route_message(&jupyter_protocol::ExecuteReply::default().as_child_of(&first), window, cx).unwrap();
            assert!(received.try_recv().is_err());
            let version = editor.source_version.clone();
            editor.clear_outputs(&version, cx).unwrap();
            let cleared = editor.notebook_item.read(cx).buffer.read(cx).text();
            editor.route_message(&jupyter_protocol::StreamContent::stdout("late").as_child_of(&first), window, cx).unwrap();
            assert_eq!(editor.notebook_item.read(cx).buffer.read(cx).text(), cleared);
        });
    }

    #[gpui::test]
    async fn test_aborted_and_stopped_cells_keep_previous_results(cx: &mut TestAppContext) {
        let saved = json!({"output_type": "stream", "name": "stdout", "text": "saved\n"});
        let notebook = json!({
            "nbformat": 4, "nbformat_minor": 5, "metadata": {}, "cells": [
                {"id":"a", "cell_type":"code", "source":"raise ValueError()", "metadata":{},
                 "execution_count":null, "outputs":[]},
                {"id":"b", "cell_type":"code", "source":"print('partial')", "metadata":{},
                 "execution_count":17, "outputs":[saved]}
            ]
        });
        let (editor, cx) = kernel_test_editor(&notebook.to_string(), cx).await;
        trust_notebook(&editor, cx);
        editor.update_in(cx, |editor, window, cx| {
            let queue = |editor: &mut NotebookEditor, index, cx: &mut Context<NotebookEditor>| {
                let Some(Cell::Code(cell)) = editor.cell(index).cloned() else {
                    panic!("code cell")
                };
                let request: JupyterMessage = ExecuteRequest::default().into();
                let run = CellExecution::queued(&cell, cell.read(cx).current_source(cx), cx);
                editor
                    .execution_requests
                    .insert(request.header.msg_id.clone(), run);
                (request, cell)
            };
            let (a, _) = queue(editor, 0, cx);
            let (b, cell) = queue(editor, 1, cx);
            let assert_visible_result = |text, count, cx: &App| {
                let view = cell.read(cx);
                assert_eq!(view.execution_count(), Some(count));
                let [crate::outputs::Output::Stream { content }] = view.output_views() else {
                    panic!("expected one stream")
                };
                assert_eq!(content.read(cx).full_text(cx), text);
            };
            assert_visible_result("saved\n", 17, cx);
            for message in [
                jupyter_protocol::ErrorOutput {
                    ename: "ValueError".into(),
                    ..Default::default()
                }
                .as_child_of(&a),
                jupyter_protocol::ExecuteReply {
                    status: jupyter_protocol::ReplyStatus::Error,
                    execution_count: 1.into(),
                    ..Default::default()
                }
                .as_child_of(&a),
                jupyter_protocol::Status::idle().as_child_of(&a),
                jupyter_protocol::Status::busy().as_child_of(&b),
                jupyter_protocol::Status::idle().as_child_of(&b),
            ] {
                editor.route_message(&message, window, cx).unwrap();
            }
            let aborted = jupyter_protocol::ExecuteReply {
                status: jupyter_protocol::ReplyStatus::Aborted,
                ..Default::default()
            }
            .as_child_of(&b);
            assert!(
                editor
                    .route_message(&aborted, window, cx)
                    .unwrap_err()
                    .to_string()
                    .contains("aborted")
            );
            assert_visible_result("saved\n", 17, cx);
            assert!(!cell.read(cx).is_executing());
            assert_eq!(cell.read(cx).execution_duration(), None);
            let buffer = editor.notebook_item.read(cx).buffer.clone();
            let after_abort = buffer.read(cx).text();
            let doc: serde_json::Value = serde_json::from_str(&after_abort).unwrap();
            assert_eq!(doc["cells"][0]["outputs"][0]["ename"], "ValueError");
            assert_eq!(doc["cells"][1], notebook["cells"][1]);

            let (rerun, _) = queue(editor, 1, cx);
            for message in [
                jupyter_protocol::ExecuteInput {
                    execution_count: 2.into(),
                    ..Default::default()
                }
                .as_child_of(&rerun),
                jupyter_protocol::StreamContent::stdout("partial\n").as_child_of(&rerun),
            ] {
                editor.route_message(&message, window, cx).unwrap();
            }
            assert_visible_result("partial\n", 2, cx);
            editor.stop_kernel(cx);
            assert_visible_result("saved\n", 17, cx);
            assert!(
                editor
                    .kernel_notice
                    .as_deref()
                    .unwrap()
                    .contains("previous results are unchanged")
            );
            assert!(!cell.read(cx).is_executing());
            assert_eq!(cell.read(cx).execution_duration(), None);
            assert_eq!(buffer.read(cx).text(), after_abort);
        });
    }

    #[gpui::test]
    async fn test_notebook_run_persists_matching_outputs_after_reply_and_idle(
        cx: &mut TestAppContext,
    ) {
        let (editor, cx) = kernel_test_editor(NOTEBOOK_WITH_ONE_CODE_CELL, cx).await;
        trust_notebook(&editor, cx);
        editor.update_in(cx, |editor, window, cx| {
            let request: JupyterMessage = ExecuteRequest::default().into();
            let other: JupyterMessage = ExecuteRequest::default().into();
            let id = editor.cell_order.first().unwrap().clone();
            let source = editor.cell(0).unwrap().current_source(cx);
            editor.execution_requests.insert(
                request.header.msg_id.clone(),
                CellExecution::new(id, source),
            );
            let duplicate = CellRun {
                cell_id: editor.cell_order[0].clone(),
                source: editor.cell(0).unwrap().current_source(cx),
                advance: true,
            };
            let error = editor
                .dispatch_cell(&duplicate, window, cx)
                .expect_err("duplicate run");
            assert!(error.to_string().contains("already running"));
            let buffer = editor.notebook_item.read(cx).buffer.clone();
            let stream = jupyter_protocol::StreamContent::stdout("from this request\n");
            let reply = jupyter_protocol::ExecuteReply {
                execution_count: 9.into(),
                ..Default::default()
            };
            for msg in [
                stream.as_child_of(&other),
                jupyter_protocol::Status::idle().as_child_of(&other),
                reply.as_child_of(&request),
                jupyter_protocol::StreamContent::stdout("cleared").as_child_of(&request),
                jupyter_protocol::ClearOutput { wait: true }.as_child_of(&request),
                stream.as_child_of(&request),
            ] {
                editor
                    .route_message(&msg, window, cx)
                    .expect("route kernel message");
            }
            assert_eq!(buffer.read(cx).text(), NOTEBOOK_WITH_ONE_CODE_CELL);
            editor
                .route_message(
                    &jupyter_protocol::Status::idle().as_child_of(&request),
                    window,
                    cx,
                )
                .unwrap();
            let value: serde_json::Value = serde_json::from_str(&buffer.read(cx).text()).unwrap();
            assert_eq!(value["cells"][0]["execution_count"], 9);
            assert_eq!(
                value["cells"][0]["outputs"],
                json!([{"output_type":"stream","name":"stdout","text":"from this request\n"}])
            );
        });
    }

    #[gpui::test]
    async fn moving_a_queued_cell_keeps_its_output_with_its_source(cx: &mut TestAppContext) {
        use jupyter_protocol::{ExecuteReply, Status, StreamContent};
        let mut fixture: serde_json::Value =
            serde_json::from_str(NOTEBOOK_WITH_ONE_CODE_CELL).unwrap();
        fixture["nbformat_minor"] = json!(4);
        fixture["cells"][0].as_object_mut().unwrap().remove("id");
        let mut second = fixture["cells"][0].clone();
        second["source"] = json!("print('second')");
        fixture["cells"].as_array_mut().unwrap().push(second);
        let (editor, cx) = kernel_test_editor(&fixture.to_string(), cx).await;
        trust_notebook(&editor, cx);
        editor.update_in(cx, |editor, window, cx| {
            let request: JupyterMessage = ExecuteRequest::default().into();
            editor.preparing_cell = editor.cell_order.first().cloned();
            editor.change_structure(CellChange::Delete, window, cx);
            assert!(
                editor
                    .structure_error
                    .as_ref()
                    .unwrap()
                    .contains("kernel setup")
            );
            editor.selected_cell_index = 1;
            editor.change_structure(CellChange::Delete, window, cx);
            assert_eq!(editor.cell_count(), 1);
            editor.undo_structure(false, window, cx);
            editor.selected_cell_index = 0;
            editor.preparing_cell = None;
            let Some(Cell::Code(cell)) = editor.cell(0) else {
                panic!("missing code cell")
            };
            let run = CellExecution::queued(cell, cell.read(cx).current_source(cx), cx);
            editor
                .execution_requests
                .insert(request.header.msg_id.clone(), run);
            editor.change_structure(CellChange::MoveDown, window, cx);
            assert!(editor.structure_error.is_none());
            editor.change_structure(CellChange::Delete, window, cx);
            assert!(
                editor
                    .structure_error
                    .as_ref()
                    .unwrap()
                    .contains("queued or running")
            );
            editor.selected_cell_index = 0;
            editor.change_structure(CellChange::InsertMarkdown, window, cx);
            editor.change_structure(CellChange::Delete, window, cx);
            assert!(editor.structure_error.is_none());
            for message in [
                StreamContent::stdout("first cell").as_child_of(&request),
                ExecuteReply {
                    execution_count: 12.into(),
                    ..Default::default()
                }
                .as_child_of(&request),
                Status::idle().as_child_of(&request),
            ] {
                editor.route_message(&message, window, cx).unwrap();
            }
            let saved: serde_json::Value =
                serde_json::from_str(&editor.notebook_item.read(cx).buffer.read(cx).text())
                    .unwrap();
            assert_eq!(saved["cells"][0], fixture["cells"][1]);
            assert_eq!(saved["cells"][1]["source"], fixture["cells"][0]["source"]);
            assert_eq!(saved["cells"][1]["outputs"][0]["text"], "first cell");
            assert_eq!(saved["cells"][1]["execution_count"], 12);
            assert!(saved["cells"][1].get("id").is_none());
            for _ in 0..3 {
                editor.undo_structure(false, window, cx);
                assert!(editor.structure_error.is_none());
            }
            let restored: serde_json::Value =
                serde_json::from_str(&editor.notebook_item.read(cx).buffer.read(cx).text())
                    .unwrap();
            assert_eq!(restored["cells"][0], saved["cells"][1]);
            assert_eq!(restored["cells"][1], fixture["cells"][1]);
        });
    }

    #[gpui::test]
    async fn test_display_updates_replace_matching_slots_across_cells(cx: &mut TestAppContext) {
        use jupyter_protocol::{
            DisplayData, ExecuteReply, MediaType, Status, StreamContent, Transient,
            UpdateDisplayData,
        };
        let mut fixture: serde_json::Value =
            serde_json::from_str(NOTEBOOK_WITH_ONE_CODE_CELL).unwrap();
        let mut next = fixture["cells"][0].clone();
        next["id"] = json!("second");
        fixture["cells"].as_array_mut().unwrap().push(next);
        let (editor, cx) = kernel_test_editor(&fixture.to_string(), cx).await;
        trust_notebook(&editor, cx);
        editor.update_in(cx, |editor, window, cx| {
            for index in 0..2 {
                let request: JupyterMessage = ExecuteRequest::default().into();
                let cell = editor.cell(index).unwrap();
                editor.execution_requests.insert(request.header.msg_id.clone(), CellExecution::new(cell.id(cx), cell.current_source(cx)));
                let original = DisplayData {
                    data: vec![MediaType::Plain("initial".into())].into(),
                    transient: Some(Transient { display_id: Some("figure".into()) }),
                    ..Default::default()
                };
                let final_text = if index == 0 { "first update" } else { "final update" };
                let mut update = UpdateDisplayData::new(vec![MediaType::Plain(final_text.into())].into(), "figure");
                update.metadata.insert("phase".into(), json!(index));
                for msg in [
                    original.as_child_of(&request),
                    StreamContent::stdout("untouched").as_child_of(&request),
                    original.as_child_of(&request),
                    update.as_child_of(&request),
                    ExecuteReply { execution_count: (index + 1).into(), ..Default::default() }.as_child_of(&request),
                    Status::idle().as_child_of(&request),
                ] {
                    editor.route_message(&msg, window, cx).unwrap();
                }
                let text = editor.notebook_item.read(cx).buffer.read(cx).text();
                let document: serde_json::Value = serde_json::from_str(&text).unwrap();
                for target in 0..=index {
                    let cell = &document["cells"][target];
                    assert_eq!(cell["execution_count"], target + 1);
                    assert_eq!(cell["outputs"][1]["text"], "untouched");
                    let Some(Cell::Code(view)) = editor.cell(target) else { panic!("expected code cell") };
                    for slot in [0, 2] {
                        assert_eq!(cell["outputs"][slot], json!({"output_type": "display_data", "data": {"text/plain": final_text}, "metadata": {"phase": index}}));
                        let crate::outputs::Output::Plain { content, .. } = &view.read(cx).output_views()[slot] else { panic!("expected plain output") };
                        assert_eq!(content.read(cx).full_text(cx).trim_end(), final_text);
                    }
                }
            }
        });
        let first = editor.read_with(cx, |editor, cx| {
            editor.cell(0).unwrap().editor(cx).unwrap().clone()
        });
        first.update_in(cx, |editor, window, cx| {
            editor.set_text("changed since execution", window, cx)
        });
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            let later =
                UpdateDisplayData::new(vec![MediaType::Plain("later".into())].into(), "figure");
            editor.route_message(&later.into(), window, cx).unwrap();
            let text = editor.notebook_item.read(cx).buffer.read(cx).text();
            let document: serde_json::Value = serde_json::from_str(&text).unwrap();
            for (index, expected) in ["final update", "later"].into_iter().enumerate() {
                let saved = &document["cells"][index];
                assert_eq!(saved["execution_count"], index + 1);
                let Some(Cell::Code(view)) = editor.cell(index) else {
                    panic!("expected code cell")
                };
                for slot in [0, 2] {
                    assert_eq!(saved["outputs"][slot]["data"]["text/plain"], expected);
                    let crate::outputs::Output::Plain { content, .. } =
                        &view.read(cx).output_views()[slot]
                    else {
                        panic!("expected plain output")
                    };
                    assert_eq!(content.read(cx).full_text(cx).trim_end(), expected);
                }
            }
            editor.stop_kernel(cx);
            let before = editor.notebook_item.read(cx).buffer.read(cx).text();
            let late = UpdateDisplayData::new(
                vec![MediaType::Plain("old kernel".into())].into(),
                "figure",
            );
            editor.route_message(&late.into(), window, cx).unwrap();
            assert_eq!(editor.notebook_item.read(cx).buffer.read(cx).text(), before);
        });
    }

    #[gpui::test]
    async fn test_save_and_reopen_a_kernel_chart(cx: &mut TestAppContext) {
        use crate::notebook::{
            NotebookPreview,
            preview::preview_tests::{notebook_workspace, open_file},
        };
        use project::Fs as _;

        const PNG: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAAAAAA6fptVAAAACklEQVR4nGP4DwABAQEAsTj2FAAAAABJRU5ErkJggg==";
        let (fs, workspace, cx) =
            notebook_workspace(json!({"chart.ipynb": NOTEBOOK_WITH_ONE_CODE_CELL}), cx).await;
        open_file("chart.ipynb", &workspace, cx).await;
        let active_notebook = |cx: &mut gpui::VisualTestContext| {
            workspace.read_with(cx, |workspace, cx| {
                workspace
                    .active_item_as::<NotebookPreview>(cx)
                    .expect("notebook tab")
                    .read(cx)
                    .notebook
                    .clone()
                    .expect("cell editor")
            })
        };
        let notebook = active_notebook(cx);
        trust_notebook(&notebook, cx);
        let data =
            json!({"image/png": PNG, "text/plain": "Figure", "text/html": "<span>Figure</span>"});
        notebook.update_in(cx, |editor, window, cx| {
            let request: JupyterMessage = ExecuteRequest::default().into();
            let cell = editor.cell(0).expect("code cell");
            editor.execution_requests.insert(
                request.header.msg_id.clone(),
                CellExecution::new(cell.id(cx), cell.current_source(cx)),
            );
            let display = jupyter_protocol::DisplayData::new(
                serde_json::from_value(data.clone()).expect("valid MIME bundle"),
            );
            let reply = jupyter_protocol::ExecuteReply {
                execution_count: 3.into(),
                ..Default::default()
            };
            for msg in [
                display.as_child_of(&request),
                jupyter_protocol::Status::idle().as_child_of(&request),
                reply.as_child_of(&request),
            ] {
                editor
                    .route_message(&msg, window, cx)
                    .expect("route kernel result");
            }
        });
        drop(notebook);
        cx.dispatch_action(workspace::Save { save_intent: None });
        cx.run_until_parked();
        let bytes = fs
            .load_bytes(std::path::Path::new(path!("/notebooks/chart.ipynb")))
            .await
            .expect("saved notebook");
        let saved: serde_json::Value = serde_json::from_slice(&bytes).expect("valid notebook");
        assert_eq!(saved["cells"][0]["execution_count"], 3);
        assert_eq!(saved["cells"][0]["outputs"][0]["data"], data);
        cx.dispatch_action(workspace::pane::CloseActiveItem::default());
        cx.run_until_parked();
        open_file("chart.ipynb", &workspace, cx).await;
        active_notebook(cx).read_with(cx, |editor, cx| {
            assert!(matches!(editor.kernel, Kernel::Shutdown));
            let Some(Cell::Code(cell)) = editor.cell(0) else {
                panic!("expected a code cell")
            };
            assert!(matches!(
                cell.read(cx).output_views(),
                [crate::outputs::Output::Image { .. }]
            ));
        });
    }

    #[gpui::test]
    async fn test_restricting_worktree_discards_in_flight_cell_output(cx: &mut TestAppContext) {
        let (editor, cx) = kernel_test_editor(NOTEBOOK_WITH_ONE_CODE_CELL, cx).await;
        trust_notebook(&editor, cx);
        let request: JupyterMessage = ExecuteRequest::default().into();
        editor.update_in(cx, |editor, window, cx| {
            let cell = editor.cell(0).unwrap();
            let run = CellExecution::new(cell.id(cx), cell.current_source(cx));
            editor
                .execution_requests
                .insert(request.header.msg_id.clone(), run);
            editor
                .route_message(
                    &jupyter_protocol::StreamContent::stdout("in flight").as_child_of(&request),
                    window,
                    cx,
                )
                .unwrap();
            let store = editor.project.read(cx).worktree_store().downgrade();
            let id = project::trusted_worktrees::PathTrust::Worktree(editor.worktree_id);
            TrustedWorktrees::try_get_global(cx)
                .unwrap()
                .update(cx, |trust, cx| {
                    trust.restrict(store, [id].into_iter().collect(), cx)
                });
        });
        cx.run_until_parked();
        editor.update_in(cx, |editor, window, cx| {
            for message in [
                jupyter_protocol::Status::idle().as_child_of(&request),
                jupyter_protocol::ExecuteReply::default().as_child_of(&request),
            ] {
                editor.route_message(&message, window, cx).unwrap();
            }
            assert!(
                editor
                    .kernel_notice
                    .as_deref()
                    .unwrap()
                    .contains("Restricted Mode")
            );
            assert_eq!(
                editor.notebook_item.read(cx).buffer.read(cx).text(),
                NOTEBOOK_WITH_ONE_CODE_CELL
            );
        });
    }

    #[gpui::test]
    async fn test_run_and_advance_during_kernel_setup_stays_on_cell(cx: &mut TestAppContext) {
        let mut value: serde_json::Value =
            serde_json::from_str(NOTEBOOK_WITH_ONE_CODE_CELL).unwrap();
        let mut second = value["cells"][0].clone();
        second["id"] = "cell-two".into();
        value["cells"].as_array_mut().unwrap().push(second);
        let (editor, cx) = kernel_test_editor(&value.to_string(), cx).await;
        trust_notebook(&editor, cx);
        editor.update_in(cx, |editor, window, cx| {
            editor.kernel_task = Some(Task::ready(()).shared());
            editor.run_and_advance(&RunAndAdvance, window, cx);
            assert_eq!(editor.selected_cell_index, 0);
            assert!(
                editor
                    .kernel_notice
                    .as_deref()
                    .unwrap()
                    .contains("not queued")
            );
        });
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn test_selected_python_without_ipykernel_offers_setup_without_advancing(
        cx: &mut TestAppContext,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let root = tempfile::tempdir().unwrap();
        let interpreter = root.path().join("python");
        std::fs::write(&interpreter, "#!/bin/sh\nexit 77\n").unwrap();
        std::fs::set_permissions(&interpreter, std::fs::Permissions::from_mode(0o700)).unwrap();
        let mut value: serde_json::Value =
            serde_json::from_str(NOTEBOOK_WITH_ONE_CODE_CELL).unwrap();
        let mut second = value["cells"][0].clone();
        second["id"] = "second".into();
        value["cells"].as_array_mut().unwrap().push(second);
        let raw = value.to_string();
        let (editor, cx) = kernel_test_editor(&raw, cx).await;
        trust_notebook(&editor, cx);
        cx.executor().allow_parking();
        let run = editor.update_in(cx, |editor, window, cx| {
            let spec =
                LocalKernelSpecification::for_notebook_python(interpreter, "System Python".into())
                    .unwrap();
            editor.select_kernel(KernelSpecification::Jupyter(spec), cx);
            editor.run_and_advance(&RunAndAdvance, window, cx);
            assert_eq!(editor.selected_cell_index, 0, "setup is not execution");
            editor.kernel_task.clone().unwrap()
        });
        cx.condition(&editor, |editor, _| editor.kernel_specification.is_none())
            .await;
        let (_, detail) = cx.pending_prompt().expect("consent before installing");
        assert!(detail.contains("ipykernel is not installed in System Python"));
        cx.simulate_prompt_answer("Cancel");
        run.await;
        editor.read_with(cx, |editor, cx| {
            assert_eq!(editor.selected_cell_index, 0);
            assert!(editor.kernel_session.is_none());
            assert!(
                editor.kernel_specification.is_none(),
                "the next Run can rediscover a usable kernel"
            );
            assert_eq!(editor.notebook_item.read(cx).buffer.read(cx).text(), raw);
        });
    }

    fn set_notebook_flag(enabled: bool, cx: &mut gpui::VisualTestContext) {
        use feature_flags::{FeatureFlag as _, FeatureFlagStore};
        use gpui::UpdateGlobal as _;
        cx.update(|_, cx| {
            FeatureFlagStore::init(cx);
            if enabled {
                cx.update_flags(false, vec![NotebookFeatureFlag::NAME.into()]);
            } else {
                SettingsStore::update_global(cx, |store, cx| {
                    store.update_user_settings(cx, |settings| {
                        settings
                            .feature_flags
                            .get_or_insert_default()
                            .insert(NotebookFeatureFlag::NAME.into(), "off".into());
                    });
                });
            }
            assert_eq!(NotebookEditor::remote_enabled(cx), enabled);
        });
    }

    fn fake_r_kernelspec() -> (std::path::PathBuf, serde_json::Value) {
        let directory = jupyter_zmq_client::dirs::data_dirs()
            .into_iter()
            .find(|directory| directory.is_absolute())
            .unwrap()
            .join("kernels/fake-r");
        let kernelspec = json!({
            "argv": [path!("/missing/fake-r"), "--vanilla", "{connection_file}"],
            "display_name": "Fake R",
            "language": "R",
            "env": { "ZED_NOTEBOOK_MARKER": "synthetic-marker" },
        });
        (directory, kernelspec)
    }

    #[gpui::test]
    async fn test_flagged_picker_launches_a_local_kernelspec_unchanged(cx: &mut TestAppContext) {
        let raw = NOTEBOOK_WITH_ONE_CODE_CELL.replace("python", "R");
        let (editor, cx) = kernel_test_editor(&raw, cx).await;
        set_notebook_flag(true, cx);
        let (_, kernelspec) = fake_r_kernelspec();
        let directory =
            std::path::PathBuf::from(path!("/notebooks/.venv/share/jupyter/kernels/fake-r"));
        let fs = editor.read_with(cx, |editor, cx| editor.project.read(cx).fs().as_fake());
        fs.insert_tree(&directory, json!({ "kernel.json": kernelspec.to_string() }))
            .await;
        trust_notebook(&editor, cx);
        cx.executor().allow_parking();

        let pick = editor.update_in(cx, |editor, window, cx| {
            editor.choose_or_start_kernel(None, window, cx);
            editor
                .kernel_task
                .clone()
                .expect("the picker lists kernels")
        });
        cx.run_until_parked();
        cx.simulate_resize(gpui::size(px(800.), px(600.)));
        cx.run_until_parked();
        cx.dispatch_action(menu::SelectFirst);
        cx.dispatch_action(menu::SelectPrevious);
        cx.dispatch_action(menu::Confirm);
        pick.await;
        editor.read_with(cx, |editor, _| {
            let Some(KernelSpecification::Jupyter(selected)) = &editor.kernel_specification else {
                panic!("expected the kernelspec, got {:?}", editor.kernel_notice)
            };
            assert_eq!(selected.path, directory);
            assert_eq!(selected.kernelspec.argv[1], "--vanilla");
            assert_eq!(
                selected.kernelspec.env.as_ref().unwrap()["ZED_NOTEBOOK_MARKER"],
                "synthetic-marker"
            );
        });

        let run = editor.update_in(cx, |editor, window, cx| {
            editor.run_current_cell(&Run, window, cx);
            editor
                .kernel_task
                .clone()
                .expect("Run launches the selected kernel")
        });
        run.await;
        editor.read_with(cx, |editor, cx| {
            let notice = editor.kernel_notice.as_deref().unwrap();
            assert!(notice.contains("Failed to start kernel fake-r"), "{notice}");
            assert!(!notice.contains("synthetic-marker") && !notice.contains("--vanilla"));
            assert_eq!(editor.notebook_item.read(cx).buffer.read(cx).text(), raw);
        });
    }

    #[gpui::test]
    async fn test_unflagged_notebook_refuses_local_kernelspecs(cx: &mut TestAppContext) {
        let (editor, cx) = kernel_test_editor(NOTEBOOK_WITH_ONE_CODE_CELL, cx).await;
        set_notebook_flag(false, cx);
        trust_notebook(&editor, cx);
        editor.update_in(cx, |editor, window, cx| {
            let (path, kernelspec) = fake_r_kernelspec();
            let stale = LocalKernelSpecification {
                name: "fake-r".into(),
                path,
                kernelspec: serde_json::from_value(kernelspec).unwrap(),
            };
            editor.select_kernel(KernelSpecification::Jupyter(stale), cx);
            editor.run_current_cell(&Run, window, cx);
            assert_eq!(
                editor.kernel_notice.as_deref(),
                Some("Choose a Python kernel. Other local kernels are disabled.")
            );
            assert!(editor.kernel_task.is_none());
        });
    }

    /// Opening a notebook as a single file (its own worktree) leaves the
    /// worktree-relative path empty, so only the absolute path carries the
    /// `.ipynb` extension. `try_open` must still recognize it as a notebook.
    #[gpui::test]
    async fn test_open_single_file_notebook(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "single.ipynb": NOTEBOOK_WITH_ONE_CODE_CELL }),
        )
        .await;

        let project =
            Project::test(fs.clone(), [path!("/notebooks/single.ipynb").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));

        let project_path = project.read_with(cx, |project, cx| {
            let worktree = project.worktrees(cx).next().unwrap();
            let worktree = worktree.read(cx);
            assert!(
                worktree.is_single_file(),
                "opening a bare .ipynb should create a single-file worktree"
            );
            ProjectPath {
                worktree_id: worktree.id(),
                path: worktree.root_entry().unwrap().path.clone(),
            }
        });

        assert!(
            project_path.path.extension().is_none(),
            "single-file worktree relative path should have no extension"
        );

        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(&project, &project_path, cx)
                    .expect("single-file .ipynb should open as a notebook")
            })
            .await
            .expect("notebook should parse");

        notebook_item.read_with(cx, |item, _| {
            assert_eq!(item.notebook.cells.len(), 1);
        });
        let cx = cx.add_empty_window();
        let editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project, notebook_item, window, cx))
        });
        let interpreter = editor
            .read_with(cx, |editor, cx| editor.project_python(true, cx).unwrap())
            .await;
        assert!(
            interpreter.is_none(),
            "opening a single file does not trust Python environments in its parent directory"
        );
    }

    /// Notebooks must be saved through the project rather than through the
    /// client's own filesystem, otherwise a remote notebook's path is resolved
    /// against the local machine and the save fails.
    #[gpui::test]
    async fn test_save_goes_through_the_project(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "test.ipynb": NOTEBOOK_WITH_ONE_CODE_CELL }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));

        let project_path = project.read_with(cx, |project, cx| ProjectPath {
            worktree_id: project.worktrees(cx).next().unwrap().read(cx).id(),
            path: rel_path("test.ipynb").into(),
        });

        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(&project, &project_path, cx)
                    .expect("ipynb files should be openable as notebooks")
            })
            .await
            .expect("notebook should parse");

        // Held across the save: a save that bypasses the project writes the file
        // behind this buffer's back, leaving it stale.
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(project_path.clone(), cx)
            })
            .await
            .expect("notebook buffer should open");

        // Rendering the notebook animates the kernel status icon, which makes
        // `run_until_parked` spin forever; only the editor entity is needed here.
        let cx = cx.add_empty_window();
        let notebook_editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project.clone(), notebook_item, window, cx))
        });

        let cell_editor = notebook_editor.read_with(cx, |notebook_editor, cx| {
            let cell_id = notebook_editor
                .cell_order
                .first()
                .expect("notebook has one cell");
            let Some(Cell::Code(cell)) = notebook_editor.cell_map.get(cell_id) else {
                panic!("expected a code cell");
            };
            cell.read(cx).editor().clone()
        });
        cell_editor.update_in(cx, |cell_editor, window, cx| {
            cell_editor.set_text("print('goodbye')", window, cx);
        });

        notebook_editor
            .update_in(cx, |notebook_editor, window, cx| {
                notebook_editor.save(SaveOptions::default(), project.clone(), window, cx)
            })
            .await
            .expect("saving the notebook should succeed");

        let saved = String::from_utf8(
            fs.read_file_sync(path!("/notebooks/test.ipynb"))
                .expect("notebook should still exist"),
        )
        .expect("notebook should be valid UTF-8");
        assert!(
            saved.contains("print('goodbye')"),
            "the edited cell should be written to the notebook, got: {saved}"
        );

        buffer.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer.text(),
                saved,
                "the project's buffer should hold the saved notebook"
            );
            assert!(!buffer.is_dirty(), "saving should leave the buffer clean");
        });
    }
}
