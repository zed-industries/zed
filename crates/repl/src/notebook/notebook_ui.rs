#![allow(unused, dead_code)]
use std::future::Future;
use std::ops::Range;
use std::{
    cmp, mem,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, Result};
use client::proto::ViewId;
use collections::{HashMap, HashSet};
use editor::{
    Anchor, CompletionContext, CompletionProvider, DefinitionNavigator, DisplayPoint, Editor,
    GotoDefinitionKind, RenameTarget, SelectionEffects, SemanticsProvider, scroll::Autoscroll,
};
use feature_flags::{FeatureFlagAppExt as _, NotebookFeatureFlag};
use futures::FutureExt;
use futures::channel::oneshot;
use futures::future::Shared;
use gpui::{
    AnyElement, App, BackgroundExecutor, Entity, EventEmitter, FocusHandle, Focusable, KeyContext,
    ListScrollEvent, ListState, Point, Task, TaskExt, WeakEntity, actions, list, prelude::*,
};
use jupyter_protocol::JupyterKernelspec;
use language::{
    Buffer, BufferEvent, BufferRow, CharScopeContext, CodeLabel, Language, LanguageName,
    LanguageRegistry, LanguageServerId, ToOffset as _,
};
use log;
use project::{
    CompletionDisplayOptions, CompletionResponse, DocumentHighlight, Hover, HoverBlock,
    HoverBlockKind, InlayHint, InvalidationStrategy, Location, LocationLink, Project,
    ProjectEntryId, ProjectPath, ProjectTransaction,
    lsp_store::{BufferSemanticTokens, CacheInlayHints, CompletionDocumentation},
};
use settings::Settings as _;
use task::{
    HideStrategy, RevealStrategy, RevealTarget, SaveStrategy, Shell, SpawnInTerminal, TaskId,
};
use text::BufferId;
use ui::{CommonAnimationExt, KeyBinding, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::item::{ItemEvent, SaveOptions, TabContentParams};
use workspace::notifications::NotifyTaskExt as _;
use workspace::searchable::{
    Direction, SearchEvent, SearchOptions, SearchToken, SearchableItem, SearchableItemHandle,
};
use workspace::{Item, ItemHandle, Pane, ProjectItem, ToolbarItemLocation, Workspace};

use super::{Cell, CellEvent, CellPosition, MarkdownCellEvent, RenderableCell};

use nbformat::v4::CellId;
use nbformat::v4::Metadata as NotebookMetadata;
use serde_json;
use uuid::Uuid;

use crate::components::{KernelPickerDelegate, KernelSelector};
use crate::kernels::{
    GENERIC_PYTHON_KERNEL_NAME, Kernel, KernelSession, KernelSpecification, KernelStatus,
    NativeRunningKernel, PythonEnvKernelSpecification, RemoteRunningKernel, SshRunningKernel,
    WslRunningKernel, ensure_generic_python_environment, find_venv_python, install_ipykernel,
    python_env_kernel_specification, python_environment_variables, python_has_ipykernel,
};
use crate::notebook::MovementDirection;
use crate::repl_store::ReplStore;

use jupyter_protocol::{ExpressionResult, MediaType};
use picker::Picker;
use runtimelib::{
    CompleteRequest, ExecuteReply, ExecuteRequest, JupyterMessage, JupyterMessageContent,
};
use ui::PopoverMenuHandle;
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

#[derive(PartialEq, Eq)]
enum SelectionMode {
    SelectOnly,
    SelectAndMove,
}

pub(crate) const MAX_TEXT_BLOCK_WIDTH: f32 = 9999.0;
pub(crate) const SMALL_SPACING_SIZE: f32 = 8.0;
pub(crate) const MEDIUM_SPACING_SIZE: f32 = 12.0;
pub(crate) const LARGE_SPACING_SIZE: f32 = 16.0;
pub(crate) const GUTTER_WIDTH: f32 = 19.0;
pub(crate) const CODE_BLOCK_INSET: f32 = MEDIUM_SPACING_SIZE;
pub(crate) const CONTROL_SIZE: f32 = 20.0;

const NOTEBOOK_EXTENSION: &str = "ipynb";

pub fn init(cx: &mut App) {
    if cx.has_flag::<NotebookFeatureFlag>() || std::env::var("LOCAL_NOTEBOOK_DEV").is_ok() {
        workspace::register_project_item::<NotebookEditor>(cx);
    }

    cx.observe_flag::<NotebookFeatureFlag, _>({
        move |flag, cx| {
            if *flag {
                workspace::register_project_item::<NotebookEditor>(cx);
            } else {
                // todo: there is no way to unregister a project item, so if the feature flag
                // gets turned off they need to restart Zed.
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
    remote_id: Option<ViewId>,
    cell_list: ListState,
    notebook_mode: NotebookMode,
    selected_cell_index: usize,
    cell_order: Vec<CellId>,
    original_cell_order: Vec<CellId>,
    cell_map: HashMap<CellId, Cell>,
    kernel: Kernel,
    kernel_specification: Option<KernelSpecification>,
    execution_requests: HashMap<String, CellId>,
    pending_kernel_replies: HashMap<String, oneshot::Sender<JupyterMessageContent>>,
    kernel_picker_handle: PopoverMenuHandle<Picker<KernelPickerDelegate>>,
    markdown_cells_revealed_by_search: HashSet<CellId>,
}

enum SaveDestination {
    CurrentPath,
    NewPath(ProjectPath),
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
        let language_name = notebook_item.read(cx).language_name();
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
            Self::subscribe_to_cell(&cell_id, &cell_entity, window, cx);
            cell_map.insert(cell_id.clone(), cell_entity);
        }

        let notebook_handle = cx.entity().downgrade();
        let cell_count = cell_order.len();

        let this = cx.entity();
        let cell_list = ListState::new(cell_count, gpui::ListAlignment::Top, px(1000.));

        let mut editor = Self {
            project,
            languages: languages.clone(),
            worktree_id,
            focus_handle,
            notebook_item: notebook_item.clone(),
            notebook_language,
            remote_id: None,
            cell_list,
            notebook_mode: NotebookMode::Command,
            selected_cell_index: 0,
            cell_order: cell_order.clone(),
            original_cell_order: cell_order.clone(),
            cell_map: cell_map.clone(),
            kernel: Kernel::Shutdown,
            kernel_specification: None,
            execution_requests: HashMap::default(),
            pending_kernel_replies: HashMap::default(),
            kernel_picker_handle: PopoverMenuHandle::default(),
            markdown_cells_revealed_by_search: HashSet::default(),
        };
        editor.launch_kernel(window, cx);
        editor.refresh_language(cx);
        editor.refresh_kernelspecs(cx);

        cx.subscribe(&notebook_item, |this, _item, _event, cx| {
            this.refresh_language(cx);
        })
        .detach();

        let buffer = notebook_item.read(cx).buffer.clone();
        let mut previous_language = buffer.read(cx).language().map(|language| language.name());
        cx.subscribe_in(&buffer, window, move |_, buffer, event, window, cx| {
            if !matches!(event, BufferEvent::LanguageChanged(_)) {
                return;
            }
            let language = buffer.read(cx).language().map(|language| language.name());
            // Only a change away from the notebook language is a request to see the file as
            // text: the language detected when the file loads is not.
            let switched_away = previous_language.as_ref().is_some_and(is_notebook_language)
                && language
                    .as_ref()
                    .is_some_and(|language| !is_notebook_language(language));
            previous_language = language;
            if switched_away {
                show_notebook_as_text(cx.entity(), window, cx);
            }
        })
        .detach();

        editor
    }

    fn subscribe_to_cell(
        cell_id: &CellId,
        cell: &Cell,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match cell {
            Cell::Code(code_cell) => {
                let completion_provider = NotebookCellCompletionProvider {
                    notebook: cx.entity().downgrade(),
                    cell_id: cell_id.clone(),
                };
                let semantics_provider = NotebookCellSemanticsProvider {
                    notebook: cx.entity().downgrade(),
                    cell_id: cell_id.clone(),
                };
                let definition_navigator = definition_navigator(cx.entity().downgrade());
                let editor = code_cell.read(cx).editor().clone();
                editor.update(cx, |editor, _| {
                    editor.set_completion_provider(Some(Rc::new(completion_provider)));
                    editor.set_semantics_provider(Some(Rc::new(semantics_provider)));
                    editor.set_definition_navigator(Some(definition_navigator));
                });
                let cell_id = cell_id.clone();
                cx.subscribe_in(
                    code_cell,
                    window,
                    move |this, _cell, event, window, cx| match event {
                        CellEvent::Run(cell_id) => this.execute_cell(cell_id.clone(), window, cx),
                        CellEvent::FocusedIn(_) => this.select_cell_by_id(&cell_id, cx),
                    },
                )
                .detach();
            }
            Cell::Markdown(markdown_cell) => {
                cx.subscribe(
                    markdown_cell,
                    |_this, cell, event: &MarkdownCellEvent, cx| match event {
                        MarkdownCellEvent::FinishedEditing | MarkdownCellEvent::Run(_) => {
                            cell.update(cx, |cell, cx| {
                                cell.reparse_markdown(cx);
                            });
                        }
                    },
                )
                .detach();
            }
            Cell::Raw(_) => {}
        }

        if let Some(editor) = cell.editor(cx).cloned() {
            let cell_id = cell_id.clone();
            cx.subscribe(&editor, move |this, _editor, event, cx| match event {
                editor::EditorEvent::Focused => this.select_cell_by_id(&cell_id, cx),
                // Only edits are forwarded: the editor also emits search events when the
                // notebook updates its highlights, and echoing those would make the search
                // bar search the whole notebook again once per cell.
                editor::EditorEvent::BufferEdited => cx.emit(SearchEvent::MatchesInvalidated),
                _ => {}
            })
            .detach();
        }
    }

    fn refresh_kernelspecs(&mut self, cx: &mut Context<Self>) {
        let store = ReplStore::global(cx);
        let project = self.project.clone();
        let worktree_id = self.worktree_id;

        let refresh_task = store.update(cx, |store, cx| {
            store.refresh_python_kernelspecs(worktree_id, &project, cx)
        });

        cx.background_spawn(refresh_task).detach_and_log_err(cx);
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
        cx.notify();
    }

    fn save_impl(
        &mut self,
        destination: SaveDestination,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let notebook = self.to_notebook(cx);
        let project_path = self.notebook_item.read(cx).project_path.clone();

        self.mark_as_saved(cx);

        cx.spawn(async move |this, cx| {
            let json =
                serde_json::to_string_pretty(&notebook).context("Failed to serialize notebook")?;
            let buffer = project
                .update(cx, |project, cx| project.open_buffer(project_path, cx))
                .await?;
            buffer.update(cx, |buffer, cx| buffer.set_text(json, cx));

            match destination {
                SaveDestination::CurrentPath => {
                    project
                        .update(cx, |project, cx| project.save_buffer(buffer, cx))
                        .await
                }
                SaveDestination::NewPath(new_path) => {
                    project
                        .update(cx, |project, cx| {
                            project.save_buffer_as(buffer, new_path.clone(), cx)
                        })
                        .await?;

                    // The buffer now lives at the new path, so the notebook has
                    // to follow it or the next save writes to the old file.
                    let entry_id = project.read_with(cx, |project, cx| {
                        project.entry_for_path(&new_path, cx).map(|entry| entry.id)
                    });
                    this.update(cx, |this, cx| {
                        this.notebook_item.update(cx, |notebook_item, _| {
                            notebook_item.project_path = new_path;
                            if let Some(entry_id) = entry_id {
                                notebook_item.id = entry_id;
                            }
                        })
                    })
                }
            }
        })
    }

    fn launch_kernel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let selected_spec = self.kernel_specification.clone().or_else(|| {
            ReplStore::global(cx)
                .read(cx)
                .selected_kernel(self.worktree_id)
                .cloned()
        });
        if let Some(spec) = selected_spec {
            self.launch_kernel_with_spec(spec, window, cx);
            return;
        }

        let default_spec = self.resolve_default_kernel(cx);
        let pending_kernel = cx
            .spawn_in(window, async move |this, cx| {
                let default_spec = default_spec.await;
                this.update_in(cx, |this, window, cx| match default_spec {
                    Ok(spec) => this.launch_kernel_with_spec(spec, window, cx),
                    Err(error) => {
                        log::error!("notebook: failed to find a kernel: {error:#}");
                        this.kernel = Kernel::ErroredLaunch(format!("{error:#}"));
                        cx.notify();
                    }
                })
                .ok();
            })
            .shared();

        self.kernel = Kernel::StartingKernel(pending_kernel);
        cx.notify();
    }

    /// Prefers a Python environment that lives in the opened folder, so the notebook runs
    /// against the project's dependencies. Without one, falls back to Zed's generic
    /// environment, which users can install packages into from its terminal.
    fn resolve_default_kernel(&self, cx: &mut Context<Self>) -> Task<Result<KernelSpecification>> {
        let worktree_id = self.worktree_id;
        let project = self.project.clone();
        let store = ReplStore::global(cx);
        let refresh_kernelspecs = store.update(cx, |store, cx| {
            store.refresh_python_kernelspecs(worktree_id, &project, cx)
        });
        let notebook_directory = self.notebook_directory(cx);
        let worktree_root = project
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            .map(|worktree| worktree.read(cx).abs_path());
        // Probing for and creating environments runs real processes on the real disk, which a
        // project backed by a fake filesystem (tests) must not do.
        let can_probe_environments = !project.read(cx).fs().is_fake();

        cx.spawn(async move |_, cx| {
            refresh_kernelspecs.await.log_err();

            let project_environment = store.read_with(cx, |store, _| {
                let is_in_opened_folder = |python_path: &Path| {
                    notebook_directory
                        .as_deref()
                        .is_some_and(|directory| python_path.starts_with(directory))
                        || worktree_root
                            .as_deref()
                            .is_some_and(|root| python_path.starts_with(root))
                };
                let candidates = store
                    .kernel_specifications_for_worktree(worktree_id)
                    .filter_map(|spec| match spec {
                        KernelSpecification::PythonEnv(env_spec)
                            if is_in_opened_folder(&env_spec.path) =>
                        {
                            Some(env_spec)
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let active_toolchain_path = store.active_python_toolchain_path(worktree_id);
                candidates
                    .iter()
                    .find(|env_spec| {
                        active_toolchain_path.is_some_and(|active_path| {
                            env_spec.path.as_path() == Path::new(active_path.as_ref())
                        })
                    })
                    .or(candidates.first())
                    .map(|env_spec| (*env_spec).clone())
            });

            let project_environment = match project_environment {
                Some(env_spec) => Some(env_spec),
                None if !can_probe_environments => None,
                None => match notebook_directory.as_deref().and_then(find_venv_python) {
                    Some(python_path) => {
                        let has_ipykernel = python_has_ipykernel(&python_path).await;
                        let name = python_path
                            .parent()
                            .and_then(Path::parent)
                            .and_then(Path::file_name)
                            .map(|name| name.to_string_lossy().to_string())
                            .unwrap_or_else(|| "venv".to_string());
                        Some(python_env_kernel_specification(
                            name,
                            python_path,
                            has_ipykernel,
                            None,
                        ))
                    }
                    None => None,
                },
            };

            if let Some(env_spec) = project_environment {
                if !env_spec.has_ipykernel {
                    let python_path = env_spec.path.clone();
                    let prefer_uv = env_spec.is_uv();
                    cx.background_spawn(
                        async move { install_ipykernel(&python_path, prefer_uv).await },
                    )
                    .await
                    .with_context(|| format!("failed to install ipykernel in {}", env_spec.name))?;
                    store.update(cx, |store, cx| {
                        store.mark_ipykernel_installed(cx, &env_spec)
                    });
                }
                return Ok(KernelSpecification::PythonEnv(
                    PythonEnvKernelSpecification {
                        has_ipykernel: true,
                        ..env_spec
                    },
                ));
            }

            anyhow::ensure!(
                can_probe_environments,
                "no Python environment found for this notebook"
            );
            let python_path = cx
                .background_spawn(ensure_generic_python_environment())
                .await?;
            Ok(KernelSpecification::PythonEnv(
                python_env_kernel_specification(
                    GENERIC_PYTHON_KERNEL_NAME.to_string(),
                    python_path,
                    true,
                    Some("venv".to_string()),
                ),
            ))
        })
    }

    fn notebook_directory(&self, cx: &App) -> Option<PathBuf> {
        let project_path = self.notebook_item.read(cx).project_path.clone();
        self.project
            .read(cx)
            .absolute_path(&project_path, cx)
            .and_then(|notebook_path| notebook_path.parent().map(Path::to_path_buf))
    }

    /// The code of every code cell up to and including `cell_id`, joined as one Python
    /// source, so the kernel analyzes the cell as if the notebook were a single file and
    /// names defined above resolve even before those cells have run.
    ///
    /// `cell_buffer` is the requested cell's buffer: its editor is the one asking, and is
    /// mid-update, so it can't be read.
    fn notebook_code(
        &self,
        cell_id: &CellId,
        cell_buffer: &Entity<Buffer>,
        cx: &App,
    ) -> Option<NotebookCode> {
        let mut text = String::new();
        let mut cells = Vec::new();
        let mut row = 0;
        let mut characters = 0;
        for current_cell_id in &self.cell_order {
            let Some(Cell::Code(cell)) = self.cell_map.get(current_cell_id) else {
                continue;
            };
            let buffer = if current_cell_id == cell_id {
                cell_buffer.clone()
            } else {
                cell.read(cx)
                    .editor()
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()?
            };
            let cell_text = buffer.read(cx).text();
            cells.push(NotebookCodeCell {
                buffer,
                first_row: row,
                start_character: characters,
            });
            text.push_str(&cell_text);
            if current_cell_id == cell_id {
                return Some(NotebookCode { text, cells });
            }
            text.push('\n');
            row += cell_text.matches('\n').count() as u32 + 1;
            characters += cell_text.chars().count() + 1;
        }
        None
    }

    fn send_kernel_request(
        &mut self,
        message: JupyterMessage,
    ) -> Option<oneshot::Receiver<JupyterMessageContent>> {
        let Kernel::RunningKernel(kernel) = &mut self.kernel else {
            return None;
        };
        let msg_id = message.header.msg_id.clone();
        kernel.request_tx().try_send(message).log_err()?;
        let (sender, receiver) = oneshot::channel();
        self.pending_kernel_replies.insert(msg_id, sender);
        Some(receiver)
    }

    /// Asks the kernel for completions at `cursor_character` (counted in characters) of
    /// `cell_id`. Returns where that cell starts in the code sent, to map the reply back.
    fn request_completions(
        &mut self,
        cell_id: &CellId,
        cell_buffer: &Entity<Buffer>,
        cursor_character: usize,
        cx: &App,
    ) -> Option<(oneshot::Receiver<JupyterMessageContent>, usize)> {
        let code = self.notebook_code(cell_id, cell_buffer, cx)?;
        let cell_start = code.cells.last()?.start_character;
        let message: JupyterMessage = CompleteRequest {
            code: code.text,
            cursor_pos: cell_start + cursor_character,
        }
        .into();
        let receiver = self.send_kernel_request(message)?;
        Some((receiver, cell_start))
    }

    /// Runs a jedi query inside the kernel, which knows both the notebook's code and the
    /// installed packages. `query` receives the 1-based line and the character column of
    /// the cursor in the notebook's joined code. The query runs as a silent user expression,
    /// so it leaves no trace in the execution count or history.
    fn request_jedi_query(
        &mut self,
        cell_id: &CellId,
        cell_buffer: &Entity<Buffer>,
        row: u32,
        column_character: usize,
        query: impl FnOnce(u32, usize) -> String,
        cx: &App,
    ) -> Option<(oneshot::Receiver<JupyterMessageContent>, NotebookCode)> {
        let code = self.notebook_code(cell_id, cell_buffer, cx)?;
        let line = code.cells.last()?.first_row + row + 1;
        let expression = jedi_expression(&code.text, &query(line, column_character))?;
        let message: JupyterMessage = ExecuteRequest {
            code: String::new(),
            silent: true,
            store_history: false,
            user_expressions: Some(std::collections::HashMap::from([(
                JEDI_QUERY_EXPRESSION.to_string(),
                expression,
            )])),
            allow_stdin: false,
            stop_on_error: false,
        }
        .into();
        let receiver = self.send_kernel_request(message)?;
        Some((receiver, code))
    }

    fn request_documentation(
        &mut self,
        cell_id: &CellId,
        cell_buffer: &Entity<Buffer>,
        row: u32,
        column_character: usize,
        cx: &App,
    ) -> Option<oneshot::Receiver<JupyterMessageContent>> {
        let query = |line, column| {
            format!(
                "[[definition.type, definition.name, definition.docstring(), \
definition.get_type_hint()] for definition in script.help({line}, {column})]"
            )
        };
        let (receiver, _) =
            self.request_jedi_query(cell_id, cell_buffer, row, column_character, query, cx)?;
        Some(receiver)
    }

    fn request_definitions(
        &mut self,
        cell_id: &CellId,
        cell_buffer: &Entity<Buffer>,
        row: u32,
        column_character: usize,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<Vec<Location>>>> {
        let query = |line, column| {
            format!(
                "[[str(definition.module_path) if definition.module_path else None, \
definition.line, definition.column, len(definition.name)] \
for definition in script.goto({line}, {column}, follow_imports=True) \
if definition.line is not None]"
            )
        };
        let (receiver, code) =
            self.request_jedi_query(cell_id, cell_buffer, row, column_character, query, cx)?;

        let cell_buffers = code
            .cells
            .into_iter()
            .map(|cell| (cell.first_row, cell.buffer))
            .collect::<Vec<_>>();
        let project = self.project.clone();
        let executor = cx.background_executor().clone();
        Some(cx.spawn(async move |_, cx| {
            let Some(JupyterMessageContent::ExecuteReply(reply)) =
                kernel_reply(receiver, executor).await
            else {
                return Ok(Vec::new());
            };
            let definitions = parse_jedi_definitions(&reply)?;

            // A name defined in the notebook resolves both to its cell and, once run, to
            // the kernel's copy of that cell's code; only the cell is worth showing.
            let in_notebook = definitions
                .iter()
                .filter(|definition| definition.module_path.is_none())
                .collect::<Vec<_>>();
            if !in_notebook.is_empty() {
                return Ok(cx.update(|cx| {
                    in_notebook
                        .into_iter()
                        .filter_map(|definition| {
                            let row = definition.line.checked_sub(1)?;
                            let (first_row, buffer) = cell_buffers
                                .iter()
                                .rev()
                                .find(|(first_row, _)| *first_row <= row)?;
                            Some(definition_location(buffer, row - first_row, definition, cx))
                        })
                        .collect()
                }));
            }

            let mut locations = Vec::new();
            for definition in &definitions {
                let (Some(module_path), Some(row)) =
                    (&definition.module_path, definition.line.checked_sub(1))
                else {
                    continue;
                };
                let buffer = project
                    .update(cx, |project, cx| {
                        project.open_local_buffer(module_path.clone(), cx)
                    })
                    .await?;
                locations.push(cx.update(|cx| definition_location(&buffer, row, definition, cx)));
            }
            Ok(locations)
        }))
    }

    /// Shows a definition found in another cell by moving to that cell, and one found in a
    /// file by opening it in the workspace.
    fn navigate_to_definition(
        &mut self,
        buffer: Entity<Buffer>,
        ranges: Vec<Range<language::Point>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(range) = ranges.into_iter().next() else {
            return false;
        };

        let target_cell = self
            .cell_order
            .iter()
            .enumerate()
            .find_map(|(index, cell_id)| {
                let editor = self.cell_map.get(cell_id)?.editor(cx)?;
                let cell_buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
                (cell_buffer == buffer).then(|| (index, editor.clone()))
            });
        if let Some((index, editor)) = target_cell {
            self.selected_cell_index = index;
            self.notebook_mode = NotebookMode::Edit;
            self.cell_list.scroll_to_reveal_item(index);
            window.focus(&editor.focus_handle(cx), cx);
            editor.update(cx, |editor, cx| {
                editor.change_selections(Default::default(), window, cx, |selections| {
                    selections.select_ranges([range.start..range.start])
                });
            });
            cx.notify();
            return true;
        }

        let Some(workspace) = Workspace::for_window(window, cx) else {
            return false;
        };
        let target_editor = workspace.update(cx, |workspace, cx| {
            workspace.open_project_item::<Editor>(None, buffer, true, true, true, true, window, cx)
        });
        target_editor.update(cx, |editor, cx| {
            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::center()),
                window,
                cx,
                |selections| selections.select_ranges([range.start..range.start]),
            );
        });
        true
    }

    fn kernel_python_path(&self) -> Option<&Path> {
        match self.kernel_specification.as_ref()? {
            KernelSpecification::PythonEnv(env_spec) => Some(&env_spec.path),
            KernelSpecification::Jupyter(local_spec) if local_spec.path.is_absolute() => {
                Some(&local_spec.path)
            }
            _ => None,
        }
    }

    /// Opens a terminal with the kernel's Python environment activated, so packages installed
    /// there (e.g. `uv pip install numpy`) become importable after a kernel restart.
    fn open_kernel_terminal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(python_path) = self.kernel_python_path().map(Path::to_path_buf) else {
            return;
        };
        let Some(workspace) = Workspace::for_window(window, cx) else {
            return;
        };
        let kernel_name = self
            .kernel_specification
            .as_ref()
            .map(|spec| spec.name().to_string())
            .unwrap_or_default();
        let label = format!("Kernel: {kernel_name}");
        let spawn_in_terminal = SpawnInTerminal {
            id: TaskId(format!("notebook-kernel-terminal-{}", cx.entity_id())),
            full_label: label.clone(),
            label: label.clone(),
            command: None,
            args: Vec::new(),
            command_label: label,
            cwd: self.notebook_directory(cx),
            env: python_environment_variables(&python_path)
                .into_iter()
                .collect(),
            use_new_terminal: true,
            allow_concurrent_runs: true,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            show_summary: false,
            show_command: false,
            show_rerun: false,
            save: SaveStrategy::default(),
        };
        workspace.update(cx, |workspace, cx| {
            workspace
                .spawn_in_terminal(spawn_in_terminal, window, cx)
                .detach();
        });
    }

    fn launch_kernel_with_spec(
        &mut self,
        spec: KernelSpecification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let entity_id = cx.entity_id();
        self.pending_kernel_replies.clear();
        // Match Jupyter by running the kernel from the notebook's directory. The worktree root
        // can't be used directly: when a single notebook is opened, the worktree is the file itself.
        let working_directory = self
            .notebook_directory(cx)
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

    fn execute_cell(&mut self, cell_id: CellId, window: &mut Window, cx: &mut Context<Self>) {
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

        let request = ExecuteRequest {
            code,
            ..Default::default()
        };
        let message: JupyterMessage = request.into();
        let msg_id = message.header.msg_id.clone();

        let send_result = match &mut self.kernel {
            Kernel::RunningKernel(kernel) => kernel
                .request_tx()
                .try_send(message)
                .map_err(|err| format!("failed to send execute request to kernel (the kernel process may have died): {err}")),
            Kernel::StartingKernel(_) => Err("the kernel is still starting".to_string()),
            Kernel::ErroredLaunch(error) => Err(format!("the kernel failed to launch: {error}")),
            Kernel::ShuttingDown | Kernel::Shutdown => Err("the kernel is shut down".to_string()),
            Kernel::Restarting => Err("the kernel is restarting".to_string()),
        };

        if let Some(Cell::Code(cell)) = self.cell_map.get(&cell_id) {
            cell.update(cx, |cell, cx| {
                if cell.has_outputs() {
                    cell.clear_outputs();
                }
                if let Err(error) = &send_result {
                    cell.show_kernel_error(error, window, cx);
                } else {
                    cell.start_execution();
                }
                cx.notify();
            });
        }

        if let Err(error) = send_result {
            log::error!("notebook: cannot execute cell: {error}");
        } else {
            self.execution_requests.insert(msg_id, cell_id.clone());
        }
    }

    fn get_selected_cell(&self) -> Option<&Cell> {
        self.cell_order
            .get(self.selected_cell_index)
            .and_then(|cell_id| self.cell_map.get(cell_id))
    }

    fn has_outputs(&self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.cell_map.values().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).has_outputs()
            } else {
                false
            }
        })
    }

    fn clear_outputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for cell in self.cell_map.values() {
            if let Cell::Code(code_cell) = cell {
                code_cell.update(cx, |cell, cx| {
                    cell.clear_outputs();
                    cx.notify();
                });
            }
        }
        cx.notify();
    }

    fn run_cells(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for cell_id in self.cell_order.clone() {
            self.execute_cell(cell_id, window, cx);
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
                self.execute_cell(cell_id, window, cx);
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
                        self.execute_cell(cell_id, window, cx);
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

    // Discussion can be done on this default implementation
    /// Moves focus to the next cell editor (used when already in edit mode).
    fn move_to_next_cell(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.cell_order.is_empty() && self.selected_cell_index < self.cell_order.len() - 1 {
            self.selected_cell_index += 1;
            // focus the new cell's editor
            if let Some(cell_id) = self.cell_order.get(self.selected_cell_index) {
                if let Some(cell) = self.cell_map.get(cell_id) {
                    match cell {
                        Cell::Code(code_cell) => {
                            let editor = code_cell.read(cx).editor();
                            window.focus(&editor.focus_handle(cx), cx);
                        }
                        Cell::Markdown(markdown_cell) => {
                            // Don't auto-enter edit mode for next markdown cell
                            // Just select it
                        }
                        Cell::Raw(_) => {}
                    }
                }
            }
            cx.notify();
        } else {
            // in the end, could optionally create a new cell
            // For now, just stay on the current cell
        }
    }

    fn open_notebook(&mut self, _: &OpenNotebook, _window: &mut Window, _cx: &mut Context<Self>) {
        println!("Open notebook triggered");
    }

    fn move_cell_up(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Move cell up triggered");
        if self.selected_cell_index > 0 {
            self.cell_order
                .swap(self.selected_cell_index, self.selected_cell_index - 1);
            self.selected_cell_index -= 1;
            cx.emit(SearchEvent::MatchesInvalidated);
            cx.notify();
        }
    }

    fn move_cell_down(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Move cell down triggered");
        if !self.cell_order.is_empty() && self.selected_cell_index < self.cell_order.len() - 1 {
            self.cell_order
                .swap(self.selected_cell_index, self.selected_cell_index + 1);
            self.selected_cell_index += 1;
            cx.emit(SearchEvent::MatchesInvalidated);
            cx.notify();
        }
    }

    fn delete_cell(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.cell_order.is_empty() {
            return;
        }
        let index = self.selected_cell_index.min(self.cell_order.len() - 1);
        let cell_id = self.cell_order.remove(index);
        self.cell_map.remove(&cell_id);
        self.cell_list.splice(index..index + 1, 0);
        cx.emit(SearchEvent::MatchesInvalidated);

        if self.cell_order.is_empty() {
            self.selected_cell_index = 0;
        } else {
            self.selected_cell_index = index.min(self.cell_order.len() - 1);
            self.cell_list
                .scroll_to_reveal_item(self.selected_cell_index);
        }
        self.notebook_mode = NotebookMode::Command;
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }

    fn insert_cell_at_current_position(
        &mut self,
        cell_id: CellId,
        cell: Cell,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        Self::subscribe_to_cell(&cell_id, &cell, window, cx);
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
        cx.emit(SearchEvent::MatchesInvalidated);
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

        self.insert_cell_at_current_position(
            new_cell_id,
            Cell::Markdown(markdown_cell.clone()),
            window,
            cx,
        );
        markdown_cell.update(cx, |cell, cx| {
            cell.set_editing(true);
            cx.notify();
        });
        let editor = markdown_cell.read(cx).editor().clone();
        window.focus(&editor.focus_handle(cx), cx);
        self.notebook_mode = NotebookMode::Edit;
        cx.notify();
    }

    fn add_code_block(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let new_cell_id: CellId = Uuid::new_v4().into();
        let notebook_language = self.notebook_language.clone();
        let metadata: nbformat::v4::CellMetadata =
            serde_json::from_str("{}").expect("empty object should parse");

        let code_cell = cx.new(|cx| {
            super::CodeCell::new(
                super::CellSource::None,
                new_cell_id.clone(),
                metadata,
                String::new(),
                notebook_language,
                window,
                cx,
            )
        });

        self.insert_cell_at_current_position(
            new_cell_id,
            Cell::Code(code_cell.clone()),
            window,
            cx,
        );
        let editor = code_cell.read(cx).editor().clone();
        window.focus(&editor.focus_handle(cx), cx);
        self.notebook_mode = NotebookMode::Edit;
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
            // The status bar follows the selected cell's cursor.
            cx.emit(());
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
        // The status bar follows the selected cell's cursor.
        cx.emit(());

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

    fn button_group(window: &mut Window, cx: &mut Context<Self>) -> Div {
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
                                .tooltip(move |window, cx| {
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
                                .tooltip(move |window, cx| {
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
                                .tooltip(move |window, cx| {
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
                                .tooltip(move |window, cx| {
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
                                .tooltip(move |window, cx| {
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
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Add code block", &AddCodeBlock, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddCodeBlock), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx).child(
                            Self::render_notebook_control(
                                "delete-cell",
                                IconName::Trash,
                                window,
                                cx,
                            )
                            .disabled(self.cell_order.is_empty())
                            .tooltip(move |window, cx| {
                                Tooltip::for_action("Delete cell", &DeleteCell, cx)
                            })
                            .on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(DeleteCell), cx);
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

        let is_spinning = matches!(
            kernel_status,
            KernelStatus::Busy
                | KernelStatus::Starting
                | KernelStatus::ShuttingDown
                | KernelStatus::Restarting
        );

        let status_icon_element = if is_spinning {
            Icon::new(status_icon)
                .size(IconSize::Small)
                .color(status_color)
                .with_rotate_animation(2)
                .into_any_element()
        } else {
            Icon::new(status_icon)
                .size(IconSize::Small)
                .color(status_color)
                .into_any_element()
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
                    .when(self.kernel_python_path().is_some(), |this| {
                        this.child(
                            IconButton::new("open-kernel-terminal", IconName::Terminal)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Open Terminal in Kernel Environment"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.open_kernel_terminal(window, cx);
                                })),
                        )
                    })
                    .child(
                        IconButton::new("restart-kernel", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(|window, cx| {
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
                            .tooltip(|window, cx| {
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

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_3()
            .child(Label::new("This notebook is empty.").color(Color::Muted))
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("empty-state-add-code", "Add code cell")
                            .start_icon(Icon::new(IconName::Code))
                            .key_binding(KeyBinding::for_action_in(
                                &AddCodeBlock,
                                &self.focus_handle,
                                cx,
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.add_code_block(window, cx)),
                            ),
                    )
                    .child(
                        Button::new("empty-state-add-markdown", "Add markdown cell")
                            .style(ButtonStyle::Subtle)
                            .start_icon(Icon::new(IconName::FileMarkdown))
                            .key_binding(KeyBinding::for_action_in(
                                &AddMarkdownBlock,
                                &self.focus_handle,
                                cx,
                            ))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.add_markdown_block(window, cx)
                            })),
                    ),
            )
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
        window: &mut Window,
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
            .on_action(cx.listener(|this, _: &AddMarkdownBlock, window, cx| {
                this.add_markdown_block(window, cx)
            }))
            .on_action(
                cx.listener(|this, _: &AddCodeBlock, window, cx| this.add_code_block(window, cx)),
            )
            .on_action(cx.listener(|this, _: &DeleteCell, window, cx| this.delete_cell(window, cx)))
            .on_action(
                cx.listener(|this, action, window, cx| this.enter_edit_mode(action, window, cx)),
            )
            .on_action(cx.listener(|this, action, window, cx| {
                this.handle_enter_command_mode(action, window, cx)
            }))
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
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .h_full()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .child(if self.cell_order.is_empty() {
                                self.render_empty_state(cx).into_any_element()
                            } else {
                                self.cell_list(window, cx).into_any_element()
                            }),
                    )
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
    project_path: ProjectPath,
    /// The file's buffer, which the notebook view and a text editor of the file share.
    buffer: Entity<Buffer>,
    languages: Arc<LanguageRegistry>,
    // Raw notebook data
    notebook: nbformat::v4::Notebook,
    // Store our version of the notebook in memory (cell_order, cell_map)
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
        let languages = project.read(cx).languages().clone();

        // For single-file worktrees the relative path is empty, so fall back
        // to the absolute path to detect notebooks opened directly.
        let abs_path = project.read(cx).absolute_path(&path, cx);
        let is_notebook = path.path.extension().unwrap_or_default() == NOTEBOOK_EXTENSION
            || abs_path
                .as_ref()
                .and_then(|abs_path| abs_path.extension())
                .is_some_and(|extension| extension == NOTEBOOK_EXTENSION);

        if is_notebook {
            Some(cx.spawn(async move |cx| {
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
                    project_path: path,
                    buffer,
                    languages,
                    notebook,
                    id,
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
        // TODO: Track if notebook metadata or structure has changed
        false
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

impl EventEmitter<()> for NotebookEditor {}

// pub struct NotebookControls {
//     pane_focused: bool,
//     active_item: Option<Box<dyn ItemHandle>>,
//     // subscription: Option<Subscription>,
// }

// impl NotebookControls {
//     pub fn new() -> Self {
//         Self {
//             pane_focused: false,
//             active_item: Default::default(),
//             // subscription: Default::default(),
//         }
//     }
// }

// impl EventEmitter<ToolbarItemEvent> for NotebookControls {}

// impl Render for NotebookControls {
//     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         div().child("notebook controls")
//     }
// }

// impl ToolbarItemView for NotebookControls {
//     fn set_active_pane_item(
//         &mut self,
//         active_pane_item: Option<&dyn workspace::ItemHandle>,
//         window: &mut Window, cx: &mut Context<Self>,
//     ) -> workspace::ToolbarItemLocation {
//         cx.notify();
//         self.active_item = None;

//         let Some(item) = active_pane_item else {
//             return ToolbarItemLocation::Hidden;
//         };

//         ToolbarItemLocation::PrimaryLeft
//     }

//     fn pane_focus_update(&mut self, pane_focused: bool, _window: &mut Window, _cx: &mut Context<Self>) {
//         self.pane_focused = pane_focused;
//     }
// }

impl Item for NotebookEditor {
    type Event = ();

    fn to_item_events(_event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(ItemEvent::UpdateTab)
    }

    fn content_buffer(&self, cx: &App) -> Option<Entity<Buffer>> {
        Some(self.notebook_item.read(cx).buffer.clone())
    }

    fn focused_editor(&self, cx: &App) -> Option<gpui::AnyEntity> {
        let cell_id = self.cell_order.get(self.selected_cell_index)?;
        let editor = self.cell_map.get(cell_id)?.editor(cx)?;
        Some(editor.clone().into_any())
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

    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or(0), cx))
            .single_line()
            .color(params.text_color())
            .when(params.preview, |this| this.italic())
            .into_any_element()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(IconName::Book.into())
    }

    // The buffer search bar lives in the toolbar.
    fn show_toolbar(&self) -> bool {
        true
    }

    // TODO
    fn pixel_position_of_cursor(&self, _: &App) -> Option<Point<Pixels>> {
        None
    }

    fn as_searchable(
        &self,
        handle: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
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
        self.save_impl(SaveDestination::CurrentPath, project, cx)
    }

    fn save_as(
        &mut self,
        project: Entity<Project>,
        path: ProjectPath,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.save_impl(SaveDestination::NewPath(path), project, cx)
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
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
                    Self::subscribe_to_cell(&cell_id, &cell_entity, window, cx);
                    cell_map.insert(cell_id.clone(), cell_entity);
                }

                this.cell_order = cell_order.clone();
                this.original_cell_order = cell_order;
                this.cell_map = cell_map;
                this.cell_list =
                    ListState::new(this.cell_order.len(), gpui::ListAlignment::Top, px(1000.));
                cx.emit(SearchEvent::MatchesInvalidated);
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

fn definition_navigator(notebook: WeakEntity<NotebookEditor>) -> DefinitionNavigator {
    Rc::new(move |buffer, ranges, window, cx| {
        // The cell editor that asks is mid-update, and navigating reads every cell's editor.
        let notebook = notebook.clone();
        window.defer(cx, move |window, cx| {
            notebook
                .update(cx, |notebook, cx| {
                    notebook.navigate_to_definition(buffer, ranges, window, cx);
                })
                .log_err();
        });
        true
    })
}

/// The language Zed gives `.ipynb` files. Selecting another language for a notebook shows
/// the file as text, and selecting this one for such a text editor shows the notebook again.
const NOTEBOOK_LANGUAGE_NAME: &str = "Jupyter Notebook";

fn is_notebook_language(language: &LanguageName) -> bool {
    language.as_ref() == NOTEBOOK_LANGUAGE_NAME
}

/// Replaces `notebook` in its pane with a text editor of the same file. Unsaved notebook
/// changes are written to the shared buffer first, so they carry over as unsaved text.
fn show_notebook_as_text(notebook: Entity<NotebookEditor>, window: &mut Window, cx: &mut App) {
    // Called while the notebook is mid-update, and removing it from its pane updates it.
    window.defer(cx, move |window, cx| {
        let Some(workspace) = Workspace::for_window(window, cx) else {
            return;
        };
        let (buffer, project, project_path, unsaved_notebook) =
            notebook.update(cx, |notebook, cx| {
                let notebook_item = notebook.notebook_item.read(cx);
                let unsaved_notebook = (notebook.has_structural_changes()
                    || notebook.has_content_changes(cx))
                .then(|| notebook.to_notebook(cx));
                (
                    notebook_item.buffer.clone(),
                    notebook.project.clone(),
                    notebook_item.project_path.clone(),
                    unsaved_notebook,
                )
            });
        if let Some(unsaved_notebook) = unsaved_notebook {
            match serde_json::to_string_pretty(&unsaved_notebook) {
                Ok(json) => {
                    buffer.update(cx, |buffer, cx| {
                        buffer.set_text(json, cx);
                    });
                }
                Err(error) => {
                    log::error!("notebook: failed to serialize unsaved changes: {error}");
                    return;
                }
            }
        }

        let editor =
            cx.new(|cx| Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx));
        watch_for_notebook_language(
            &editor,
            &buffer,
            project,
            project_path,
            workspace.downgrade(),
            window,
            cx,
        );
        replace_pane_item(&workspace, &notebook, Box::new(editor), window, cx);
    });
}

/// Switches a text editor of a notebook file back to the notebook view when its language is
/// set to the notebook language.
fn watch_for_notebook_language(
    editor: &Entity<Editor>,
    buffer: &Entity<Buffer>,
    project: Entity<Project>,
    project_path: ProjectPath,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let mut previous_language = buffer.read(cx).language().map(|language| language.name());
    editor.update(cx, |_, cx| {
        cx.subscribe_in(buffer, window, move |_, buffer, event, window, cx| {
            if !matches!(event, BufferEvent::LanguageChanged(_)) {
                return;
            }
            let language = buffer.read(cx).language().map(|language| language.name());
            let switched_to_notebook = previous_language
                .as_ref()
                .is_some_and(|language| !is_notebook_language(language))
                && language.as_ref().is_some_and(is_notebook_language);
            previous_language = language;
            if switched_to_notebook {
                show_text_as_notebook(
                    cx.entity(),
                    project.clone(),
                    project_path.clone(),
                    workspace.clone(),
                    window,
                    cx,
                );
            }
        })
        .detach();
    });
}

fn show_text_as_notebook(
    editor: Entity<Editor>,
    project: Entity<Project>,
    project_path: ProjectPath,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(open_notebook) =
        <NotebookItem as project::ProjectItem>::try_open(&project, &project_path, cx)
    else {
        return;
    };
    let open_task = window.spawn(cx, {
        let workspace = workspace.clone();
        async move |cx| {
            let notebook_item = open_notebook
                .await
                .context("the file isn't a valid notebook")?;
            let workspace = workspace.upgrade().context("the workspace was closed")?;
            cx.update(|window, cx| {
                let notebook = cx.new(|cx| NotebookEditor::new(project, notebook_item, window, cx));
                replace_pane_item(&workspace, &editor, Box::new(notebook), window, cx);
            })
        }
    });
    open_task.detach_and_notify_err(workspace, window, cx);
}

fn replace_pane_item(
    workspace: &Entity<Workspace>,
    old_item: &dyn ItemHandle,
    new_item: Box<dyn ItemHandle>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(pane) = workspace.read(cx).pane_for(old_item) else {
        return;
    };
    let old_item_id = old_item.item_id();
    // Both items show the same file, and a pane activates its existing item for a file
    // rather than adding another, so the old item has to go first.
    pane.update(cx, |pane, cx| {
        let index = pane.index_for_item(old_item);
        pane.remove_item(old_item_id, false, false, window, cx);
        pane.add_item(new_item, true, true, index, window, cx);
    });
}

/// A kernel busy running a cell answers requests only once it's done, so stop waiting
/// rather than leave completions, hovers or navigation pending.
const KERNEL_REPLY_TIMEOUT: Duration = Duration::from_secs(3);

async fn kernel_reply(
    receiver: oneshot::Receiver<JupyterMessageContent>,
    executor: BackgroundExecutor,
) -> Option<JupyterMessageContent> {
    let timeout = executor.timer(KERNEL_REPLY_TIMEOUT);
    futures::select_biased! {
        reply = receiver.fuse() => reply.ok(),
        _ = timeout.fuse() => None,
    }
}

fn character_count(text: &str, byte_offset: usize) -> Option<usize> {
    Some(text.get(..byte_offset)?.chars().count())
}

struct NotebookCode {
    text: String,
    cells: Vec<NotebookCodeCell>,
}

struct NotebookCodeCell {
    buffer: Entity<Buffer>,
    first_row: u32,
    start_character: usize,
}

const JEDI_QUERY_EXPRESSION: &str = "zed_jedi_query";

/// Wraps `query`, which reads from `script`, a jedi interpreter over `code` and the kernel's
/// namespace. The result is hex-encoded JSON because user expressions come back as their
/// `repr`, and the `repr` of a hex string is trivial to undo.
fn jedi_expression(code: &str, query: &str) -> Option<String> {
    let code = serde_json::to_string(code).log_err()?;
    Some(format!(
        "(lambda script: __import__('json').dumps({query}).encode().hex())\
(__import__('jedi').Interpreter({code}, [get_ipython().user_ns]))"
    ))
}

fn jedi_query_result(reply: &ExecuteReply) -> Result<Vec<u8>> {
    let result = reply
        .user_expressions
        .as_ref()
        .and_then(|expressions| expressions.get(JEDI_QUERY_EXPRESSION))
        .context("the kernel didn't evaluate the jedi query")?;
    let data = match result {
        ExpressionResult::Ok { data, .. } => data,
        ExpressionResult::Error { ename, evalue, .. } => {
            anyhow::bail!("the jedi query failed: {ename}: {evalue}")
        }
    };
    let text = data
        .content
        .iter()
        .find_map(|media| match media {
            MediaType::Plain(text) => Some(text),
            _ => None,
        })
        .context("the jedi query returned no text")?;
    let hex = text.trim().trim_matches(['\'', '"']);
    decode_hex(hex).context("the jedi query returned malformed data")
}

/// jedi's `help` entries are `[type, name, docstring, type hint]`. The docstring starts with
/// the signature for callables; plain variables only have a type hint, which jedi reports as
/// `None` when it can't infer one.
fn documentation_text(reply: &ExecuteReply) -> Result<Option<String>> {
    let entries: Vec<(String, String, String, String)> =
        serde_json::from_slice(&jedi_query_result(reply)?)?;
    let Some((kind, name, docstring, type_hint)) = entries.into_iter().next() else {
        return Ok(None);
    };
    let text = if !docstring.trim().is_empty() {
        docstring
    } else if !type_hint.is_empty() && type_hint != "None" {
        format!("{name}: {type_hint}")
    } else if kind == "statement" {
        name
    } else {
        format!("{kind} {name}")
    };
    Ok(Some(text))
}

#[derive(Debug, PartialEq)]
struct JediDefinition {
    module_path: Option<PathBuf>,
    /// 1-based, as jedi reports it.
    line: u32,
    /// In characters.
    column: usize,
    name_length: usize,
}

fn parse_jedi_definitions(reply: &ExecuteReply) -> Result<Vec<JediDefinition>> {
    let entries: Vec<(Option<String>, u32, usize, usize)> =
        serde_json::from_slice(&jedi_query_result(reply)?)?;
    Ok(entries
        .into_iter()
        .map(|(module_path, line, column, name_length)| JediDefinition {
            module_path: module_path.map(PathBuf::from),
            line,
            column,
            name_length,
        })
        .collect())
}

fn decode_hex(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(hex.get(index..index + 2)?, 16).ok())
        .collect()
}

fn definition_location(
    buffer: &Entity<Buffer>,
    row: u32,
    definition: &JediDefinition,
    cx: &App,
) -> Location {
    let snapshot = buffer.read(cx).snapshot();
    let start = point_for_character(&snapshot, row, definition.column);
    let end = point_for_character(&snapshot, row, definition.column + definition.name_length);
    Location {
        buffer: buffer.clone(),
        range: snapshot.anchor_before(start)..snapshot.anchor_after(end),
    }
}

fn point_for_character(
    snapshot: &language::BufferSnapshot,
    row: u32,
    character: usize,
) -> language::Point {
    let row = row.min(snapshot.max_point().row);
    let line_end = language::Point::new(row, snapshot.line_len(row));
    let line = snapshot
        .text_for_range(language::Point::new(row, 0)..line_end)
        .collect::<String>();
    let column = line
        .char_indices()
        .map(|(offset, _)| offset)
        .chain([line.len()])
        .nth(character)
        .unwrap_or(line.len());
    language::Point::new(row, column as u32)
}

/// jedi addresses positions by row and by column in characters.
fn row_and_column_character(snapshot: &language::BufferSnapshot, offset: usize) -> (u32, usize) {
    let point = snapshot.offset_to_point(offset);
    let line_start = snapshot.point_to_offset(language::Point::new(point.row, 0));
    let column_character = snapshot
        .text_for_range(line_start..offset)
        .collect::<String>()
        .chars()
        .count();
    (point.row, column_character)
}

struct NotebookCellSemanticsProvider {
    notebook: WeakEntity<NotebookEditor>,
    cell_id: CellId,
}

impl SemanticsProvider for NotebookCellSemanticsProvider {
    fn hover(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        cx: &mut App,
    ) -> Option<Task<Option<Vec<Hover>>>> {
        let snapshot = buffer.read(cx).snapshot();
        let offset = position.to_offset(&snapshot);
        let (row, column_character) = row_and_column_character(&snapshot, offset);
        let (word, _) = snapshot.surrounding_word(offset, None);
        let word_range = snapshot.anchor_before(word.start)..snapshot.anchor_after(word.end);
        let receiver = self.notebook.upgrade()?.update(cx, |notebook, cx| {
            notebook.request_documentation(&self.cell_id, buffer, row, column_character, cx)
        })?;
        let executor = cx.background_executor().clone();
        Some(cx.background_spawn(async move {
            let JupyterMessageContent::ExecuteReply(reply) =
                kernel_reply(receiver, executor).await?
            else {
                return None;
            };
            let text = documentation_text(&reply).log_err().flatten()?;
            Some(vec![Hover {
                contents: vec![HoverBlock {
                    text,
                    kind: HoverBlockKind::PlainText,
                }],
                range: Some(word_range),
                language: None,
            }])
        }))
    }

    fn definitions(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        _kind: GotoDefinitionKind,
        cx: &mut App,
    ) -> Option<Task<Result<Option<Vec<LocationLink>>>>> {
        let snapshot = buffer.read(cx).snapshot();
        let offset = position.to_offset(&snapshot);
        let (row, column_character) = row_and_column_character(&snapshot, offset);
        let (word, _) = snapshot.surrounding_word(offset, None);
        let origin = Location {
            buffer: buffer.clone(),
            range: snapshot.anchor_before(word.start)..snapshot.anchor_after(word.end),
        };
        let definitions = self.notebook.upgrade()?.update(cx, |notebook, cx| {
            notebook.request_definitions(&self.cell_id, buffer, row, column_character, cx)
        })?;
        Some(cx.spawn(async move |_| {
            let targets = definitions.await?;
            Ok(Some(
                targets
                    .into_iter()
                    .map(|target| LocationLink {
                        origin: Some(origin.clone()),
                        target,
                    })
                    .collect(),
            ))
        }))
    }

    fn inline_values(
        &self,
        _buffer_handle: Entity<Buffer>,
        _range: Range<language::Anchor>,
        _cx: &mut App,
    ) -> Option<Task<Result<Vec<InlayHint>>>> {
        None
    }

    fn applicable_inlay_chunks(
        &self,
        _buffer: &Entity<Buffer>,
        _ranges: &[Range<language::Anchor>],
        _cx: &mut App,
    ) -> Vec<Range<BufferRow>> {
        Vec::new()
    }

    fn invalidate_inlay_hints(&self, _for_buffers: &HashSet<BufferId>, _cx: &mut App) {}

    fn inlay_hints(
        &self,
        _invalidate: InvalidationStrategy,
        _buffer: Entity<Buffer>,
        _ranges: Vec<Range<language::Anchor>>,
        _known_chunks: Option<(clock::Global, HashSet<Range<BufferRow>>)>,
        _cx: &mut App,
    ) -> Option<HashMap<Range<BufferRow>, Task<Result<CacheInlayHints>>>> {
        None
    }

    fn semantic_tokens(
        &self,
        _buffer: Entity<Buffer>,
        _cx: &mut App,
    ) -> Option<Shared<Task<std::result::Result<BufferSemanticTokens, Arc<anyhow::Error>>>>> {
        None
    }

    fn supports_inlay_hints(&self, _buffer: &Entity<Buffer>, _cx: &mut App) -> bool {
        false
    }

    fn supports_semantic_tokens(&self, _buffer: &Entity<Buffer>, _cx: &mut App) -> bool {
        false
    }

    fn document_highlights(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        _cx: &mut App,
    ) -> Option<Task<Result<Vec<DocumentHighlight>>>> {
        None
    }

    fn range_for_rename(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        _cx: &mut App,
    ) -> Task<Result<Option<RenameTarget>>> {
        Task::ready(Ok(None))
    }

    fn perform_rename(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        _new_name: String,
        _language_server_id: Option<LanguageServerId>,
        _cx: &mut App,
    ) -> Option<Task<Result<ProjectTransaction>>> {
        None
    }
}

struct NotebookCellCompletionProvider {
    notebook: WeakEntity<NotebookEditor>,
    cell_id: CellId,
}

impl CompletionProvider for NotebookCellCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        _trigger: CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let snapshot = buffer.read(cx).snapshot();
        let cell_text = snapshot.text();
        let Some(cursor_character) =
            character_count(&cell_text, buffer_position.to_offset(&snapshot))
        else {
            return Task::ready(Ok(Vec::new()));
        };
        let request = self
            .notebook
            .update(cx, |notebook, cx| {
                notebook.request_completions(&self.cell_id, buffer, cursor_character, cx)
            })
            .ok()
            .flatten();
        let Some((reply, preceding_cells_length)) = request else {
            return Task::ready(Ok(Vec::new()));
        };

        let executor = cx.background_executor().clone();
        cx.spawn(async move |_, _| {
            let Some(JupyterMessageContent::CompleteReply(reply)) =
                kernel_reply(reply, executor).await
            else {
                return Ok(Vec::new());
            };

            let cell_offset = |position: usize| {
                let characters = position.checked_sub(preceding_cells_length)?;
                cell_text
                    .char_indices()
                    .map(|(offset, _)| offset)
                    .chain([cell_text.len()])
                    .nth(characters)
            };
            let (Some(start), Some(end)) = (
                cell_offset(reply.cursor_start),
                cell_offset(reply.cursor_end),
            ) else {
                return Ok(Vec::new());
            };
            let replace_range = snapshot.anchor_before(start)..snapshot.anchor_after(end);

            let types = completion_types(&reply.metadata);
            let completions = reply
                .matches
                .into_iter()
                .map(|text| project::Completion {
                    replace_range: replace_range.clone(),
                    documentation: types
                        .get(&text)
                        .map(|description| CompletionDocumentation::SingleLine(description.into())),
                    label: CodeLabel::plain(text.clone(), None),
                    new_text: text,
                    match_start: None,
                    snippet_deduplication_key: None,
                    icon_path: None,
                    icon_color: None,
                    confirm: None,
                    source: project::CompletionSource::Custom,
                    insert_text_mode: None,
                    group: None,
                })
                .collect();

            Ok(vec![CompletionResponse {
                completions,
                display_options: CompletionDisplayOptions::default(),
                is_incomplete: false,
            }])
        })
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        let Some(character) = text.chars().next() else {
            return false;
        };
        if character == '.' {
            return true;
        }
        let classifier = buffer
            .read(cx)
            .snapshot()
            .char_classifier_at(position)
            .scope_context(Some(CharScopeContext::Completion));
        trigger_in_words && classifier.is_word(character)
    }
}

/// ipykernel describes its matches in the `_jupyter_types_experimental` metadata, e.g.
/// `{"text": "mean", "type": "function", "signature": "(a, axis=None)"}`.
fn completion_types(
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> HashMap<String, String> {
    let Some(serde_json::Value::Array(entries)) = metadata.get("_jupyter_types_experimental")
    else {
        return HashMap::default();
    };
    entries
        .iter()
        .filter_map(|entry| {
            let text = entry.get("text")?.as_str()?;
            let kind = entry
                .get("type")
                .and_then(|kind| kind.as_str())
                .unwrap_or("");
            let signature = entry
                .get("signature")
                .and_then(|signature| signature.as_str())
                .unwrap_or("");
            let description = format!("{kind} {text}{signature}").trim().to_string();
            Some((text.to_string(), description))
        })
        .collect()
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

        if matches!(
            message.content,
            JupyterMessageContent::CompleteReply(_) | JupyterMessageContent::ExecuteReply(_)
        ) && let Some(parent_header) = &message.parent_header
            && let Some(sender) = self.pending_kernel_replies.remove(&parent_header.msg_id)
        {
            // The receiver is gone when the editor stopped waiting for this reply.
            sender.send(message.content.clone()).ok();
            return;
        }

        // Handle cell-specific messages
        if let Some(parent_header) = &message.parent_header {
            if let Some(cell_id) = self.execution_requests.get(&parent_header.msg_id) {
                if let Some(Cell::Code(cell)) = self.cell_map.get(cell_id) {
                    cell.update(cx, |cell, cx| {
                        cell.handle_message(message, window, cx);
                    });
                }
            }
        }
    }

    fn kernel_errored(&mut self, error_message: String, cx: &mut Context<Self>) {
        self.kernel = Kernel::ErroredLaunch(error_message);
        cx.notify();
    }
}

/// A search match inside one cell of a notebook.
#[derive(Clone, Debug)]
pub struct NotebookSearchMatch {
    cell_id: CellId,
    range: Range<Anchor>,
}

impl EventEmitter<SearchEvent> for NotebookEditor {}

impl NotebookEditor {
    /// The cells that can be searched, in notebook order, with their editors.
    fn searchable_cell_editors(&self, cx: &App) -> Vec<(CellId, Entity<Editor>)> {
        self.cell_order
            .iter()
            .filter_map(|cell_id| {
                let editor = self.cell_map.get(cell_id)?.editor(cx)?.clone();
                Some((cell_id.clone(), editor))
            })
            .collect()
    }

    /// Returns the ranges of the matches inside `cell_id`, and the position of
    /// `active_index` among them when it points into that cell.
    fn matches_in_cell(
        cell_id: &CellId,
        matches: &[NotebookSearchMatch],
        active_index: Option<usize>,
    ) -> (Vec<Range<Anchor>>, Option<usize>) {
        let mut ranges = Vec::new();
        let mut local_active_index = None;
        for (index, search_match) in matches.iter().enumerate() {
            if &search_match.cell_id == cell_id {
                if active_index == Some(index) {
                    local_active_index = Some(ranges.len());
                }
                ranges.push(search_match.range.clone());
            }
        }
        (ranges, local_active_index)
    }

    /// Like [`Self::matches_in_cell`], for every cell at once.
    fn matches_by_cell(
        matches: &[NotebookSearchMatch],
        active_index: Option<usize>,
    ) -> HashMap<&CellId, (Vec<Range<Anchor>>, Option<usize>)> {
        let mut matches_by_cell: HashMap<&CellId, (Vec<Range<Anchor>>, Option<usize>)> =
            HashMap::default();
        for (index, search_match) in matches.iter().enumerate() {
            let (ranges, local_active_index) =
                matches_by_cell.entry(&search_match.cell_id).or_default();
            if active_index == Some(index) {
                *local_active_index = Some(ranges.len());
            }
            ranges.push(search_match.range.clone());
        }
        matches_by_cell
    }

    fn cell_index_for_id(&self, cell_id: &CellId) -> Option<usize> {
        self.cell_order.iter().position(|id| id == cell_id)
    }

    fn cell_positions(&self) -> HashMap<&CellId, usize> {
        self.cell_order
            .iter()
            .enumerate()
            .map(|(position, cell_id)| (cell_id, position))
            .collect()
    }

    /// Returns the first match after the cursor of the selected cell (or the last one before
    /// it), wrapping around the notebook. `matches` must not be empty.
    fn match_index_from_selection(
        &self,
        direction: Direction,
        matches: &[NotebookSearchMatch],
        current_index: usize,
        cx: &App,
    ) -> usize {
        let selected_position = self.selected_cell_index;
        let selected_cell_id = self.cell_order.get(selected_position);
        // When the user moved to another cell since the current match was activated, a match
        // right at the cursor is the one they expect, rather than the one after it.
        let selection_moved = matches
            .get(current_index)
            .is_none_or(|search_match| Some(&search_match.cell_id) != selected_cell_id);
        // In command mode the cursor isn't shown, so a newly selected cell is searched from
        // its start rather than from wherever its cursor was left.
        let cursor = if selection_moved && self.notebook_mode == NotebookMode::Command {
            None
        } else {
            selected_cell_id
                .and_then(|cell_id| self.cell_map.get(cell_id))
                .and_then(|cell| cell.editor(cx))
                .map(|editor| {
                    let editor = editor.read(cx);
                    (
                        editor.selections.newest_anchor().head(),
                        editor.buffer().read(cx).snapshot(cx),
                    )
                })
        };

        let cell_positions = self.cell_positions();
        let is_beyond_cursor = |search_match: &NotebookSearchMatch| {
            let Some(&position) = cell_positions.get(&search_match.cell_id) else {
                return false;
            };
            match (direction, position.cmp(&selected_position)) {
                (Direction::Next, cmp::Ordering::Greater)
                | (Direction::Prev, cmp::Ordering::Less) => true,
                (_, cmp::Ordering::Equal) => {
                    // Without a cursor, the cell is searched from its start.
                    let Some((cursor, snapshot)) = cursor.as_ref() else {
                        return direction == Direction::Next;
                    };
                    let ordering = match direction {
                        Direction::Next => search_match.range.start.cmp(cursor, snapshot),
                        Direction::Prev => search_match.range.end.cmp(cursor, snapshot).reverse(),
                    };
                    ordering.is_gt() || (selection_moved && ordering.is_eq())
                }
                _ => false,
            }
        };

        match direction {
            Direction::Next => matches.iter().position(is_beyond_cursor).unwrap_or(0),
            Direction::Prev => matches
                .iter()
                .rposition(is_beyond_cursor)
                .unwrap_or(matches.len().saturating_sub(1)),
        }
    }

    /// Renders again the markdown cells whose source was shown to reveal a match, except
    /// `except_cell_id`. Without this they would stay in edit mode, because they only leave
    /// it when their editor loses focus, and the search never focuses them.
    fn rerender_markdown_cells_revealed_by_search(
        &mut self,
        except_cell_id: Option<&CellId>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let revealed_cell_ids = mem::take(&mut self.markdown_cells_revealed_by_search);
        for cell_id in revealed_cell_ids {
            if Some(&cell_id) == except_cell_id {
                self.markdown_cells_revealed_by_search.insert(cell_id);
                continue;
            }
            let Some(Cell::Markdown(markdown_cell)) = self.cell_map.get(&cell_id) else {
                continue;
            };
            markdown_cell.update(cx, |cell, cx| {
                let is_focused = cell.editor().focus_handle(cx).contains_focused(window, cx);
                if cell.is_editing() && !is_focused {
                    cell.set_editing(false);
                    // A replacement may have changed the source since it was last rendered.
                    cell.reparse_markdown(cx);
                    cx.notify();
                }
            });
        }
    }
}

impl SearchableItem for NotebookEditor {
    type Match = NotebookSearchMatch;

    fn supported_options(&self) -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
            replacement: true,
            selection: false,
            select_all: false,
            find_in_results: false,
        }
    }

    fn search_bar_visibility_changed(
        &mut self,
        visible: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if visible {
            return;
        }
        self.rerender_markdown_cells_revealed_by_search(None, window, cx);
        // Dismissing the search bar focuses the notebook itself rather than a cell editor,
        // which is command mode.
        self.notebook_mode = NotebookMode::Command;
        cx.notify();
    }

    fn clear_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for (_, editor) in self.searchable_cell_editors(cx) {
            editor.update(cx, |editor, cx| editor.clear_matches(window, cx));
        }
    }

    fn update_matches(
        &mut self,
        matches: &[Self::Match],
        active_match_index: Option<usize>,
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut matches_by_cell = Self::matches_by_cell(matches, active_match_index);
        for (cell_id, editor) in self.searchable_cell_editors(cx) {
            let cell_matches = matches_by_cell.remove(&cell_id);
            editor.update(cx, |editor, cx| match cell_matches {
                Some((ranges, local_active_index)) => {
                    editor.update_matches(&ranges, local_active_index, token, window, cx)
                }
                None => editor.clear_matches(window, cx),
            });
        }
    }

    fn query_suggestion(
        &mut self,
        seed_query_override: Option<settings::SeedQuerySetting>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> String {
        if self.notebook_mode != NotebookMode::Edit {
            return String::new();
        }
        let Some(editor) = self
            .cell_order
            .get(self.selected_cell_index)
            .and_then(|cell_id| self.cell_map.get(cell_id))
            .and_then(|cell| cell.editor(cx))
            .cloned()
        else {
            return String::new();
        };
        editor.update(cx, |editor, cx| {
            editor.query_suggestion(seed_query_override, window, cx)
        })
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(search_match) = matches.get(index) else {
            return;
        };
        let cell_id = search_match.cell_id.clone();
        let Some(cell_index) = self.cell_index_for_id(&cell_id) else {
            return;
        };
        let Some(cell) = self.cell_map.get(&cell_id).cloned() else {
            return;
        };

        self.rerender_markdown_cells_revealed_by_search(Some(&cell_id), window, cx);

        // A rendered markdown cell hides its editor, so show the source to
        // make the match visible.
        if let Cell::Markdown(markdown_cell) = &cell {
            let revealed = markdown_cell.update(cx, |cell, cx| {
                if cell.is_editing() {
                    return false;
                }
                cell.set_editing(true);
                cx.notify();
                true
            });
            if revealed {
                self.markdown_cells_revealed_by_search
                    .insert(cell_id.clone());
            }
        }

        let Some(editor) = cell.editor(cx).cloned() else {
            return;
        };

        self.selected_cell_index = cell_index;
        self.cell_list.scroll_to_reveal_item(cell_index);

        let (ranges, local_index) = Self::matches_in_cell(&cell_id, matches, Some(index));
        if let Some(local_index) = local_index {
            editor.update(cx, |editor, cx| {
                editor.activate_match(local_index, &ranges, token, window, cx);
            });
        }
        cx.notify();
    }

    fn select_matches(
        &mut self,
        matches: &[Self::Match],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let matches_by_cell = Self::matches_by_cell(matches, None);
        for (cell_id, editor) in self.searchable_cell_editors(cx) {
            if let Some((ranges, _)) = matches_by_cell.get(&cell_id) {
                editor.update(cx, |editor, cx| {
                    editor.select_matches(ranges, token, window, cx);
                });
            }
        }
    }

    fn replace(
        &mut self,
        search_match: &Self::Match,
        query: &project::search::SearchQuery,
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(editor) = self
            .cell_map
            .get(&search_match.cell_id)
            .and_then(|cell| cell.editor(cx))
            .cloned()
        else {
            return;
        };
        editor.update(cx, |editor, cx| {
            editor.replace(&search_match.range, query, token, window, cx);
        });
    }

    fn replace_all(
        &mut self,
        matches: &mut dyn Iterator<Item = &Self::Match>,
        query: &project::search::SearchQuery,
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Replacing cell by cell makes a single undo step per cell, instead of one per match.
        let matches = matches.cloned().collect::<Vec<_>>();
        let matches_by_cell = Self::matches_by_cell(&matches, None);
        for (cell_id, editor) in self.searchable_cell_editors(cx) {
            if let Some((ranges, _)) = matches_by_cell.get(&cell_id) {
                editor.update(cx, |editor, cx| {
                    editor.replace_all(&mut ranges.iter(), query, token, window, cx);
                });
            }
        }
    }

    fn match_index_for_direction(
        &mut self,
        matches: &[Self::Match],
        current_index: usize,
        direction: Direction,
        count: usize,
        _token: SearchToken,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> usize {
        if count == 0 || matches.is_empty() {
            return current_index;
        }
        // Like the editor, navigate from the cursor rather than from `current_index`, so that
        // selecting another cell or moving the cursor is taken into account.
        let nearest_index = self.match_index_from_selection(direction, matches, current_index, cx);
        let remaining_steps = (count - 1) % matches.len();
        match direction {
            Direction::Next => (nearest_index + remaining_steps) % matches.len(),
            Direction::Prev => (nearest_index + matches.len() - remaining_steps) % matches.len(),
        }
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Self::Match>> {
        let searches = self
            .searchable_cell_editors(cx)
            .into_iter()
            .map(|(cell_id, editor)| {
                let search = editor.update(cx, |editor, cx| {
                    editor.find_matches(query.clone(), window, cx)
                });
                (cell_id, search)
            })
            .collect::<Vec<_>>();

        cx.spawn(async move |_, _| {
            let mut matches = Vec::new();
            for (cell_id, search) in searches {
                matches.extend(search.await.into_iter().map(|range| NotebookSearchMatch {
                    cell_id: cell_id.clone(),
                    range,
                }));
            }
            matches
        })
    }

    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Self::Match],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        if matches.is_empty() {
            return None;
        }

        let selected_cell_id = self.cell_order.get(self.selected_cell_index)?.clone();
        let indices_in_selected_cell = matches
            .iter()
            .enumerate()
            .filter(|(_, search_match)| search_match.cell_id == selected_cell_id)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        // Let the selected cell's editor pick the match closest to its cursor.
        if !indices_in_selected_cell.is_empty()
            && let Some(editor) = self
                .cell_map
                .get(&selected_cell_id)
                .and_then(|cell| cell.editor(cx))
                .cloned()
        {
            let (ranges, _) = Self::matches_in_cell(&selected_cell_id, matches, None);
            let local_index = editor.update(cx, |editor, cx| {
                editor.active_match_index(direction, &ranges, token, window, cx)
            })?;
            return indices_in_selected_cell.get(local_index).copied();
        }

        // Otherwise pick the nearest match in the next (or previous) cells,
        // wrapping around the notebook.
        let selected_position = self.selected_cell_index;
        let cell_positions = self.cell_positions();
        let position_of =
            |search_match: &NotebookSearchMatch| cell_positions.get(&search_match.cell_id).copied();
        match direction {
            Direction::Next => matches
                .iter()
                .position(|search_match| {
                    position_of(search_match).is_some_and(|position| position > selected_position)
                })
                .or(Some(0)),
            Direction::Prev => matches
                .iter()
                .rposition(|search_match| {
                    position_of(search_match).is_some_and(|position| position < selected_position)
                })
                .or(Some(matches.len() - 1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernels::LocalKernelSpecification;
    use gpui::{TestAppContext, VisualTestContext};
    use project::{FakeFs, Project, ProjectItem as _};
    use runtimelib::CompleteReply;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{cell::RefCell, rc::Rc};
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

    /// When the configured interpreter doesn't exist (e.g. Python isn't installed),
    /// running a cell must not leave it stuck in the executing state. It should
    /// instead surface the kernel launch error as an error output on the cell.
    #[gpui::test]
    async fn test_run_cell_with_missing_interpreter_shows_error(cx: &mut TestAppContext) {
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

        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        // Select a kernel whose interpreter doesn't exist, simulating a machine
        // where Python isn't installed properly. This is the same path the
        // kernel picker uses.
        let missing_interpreter = path!("/nonexistent/python3");
        let broken_spec = KernelSpecification::Jupyter(LocalKernelSpecification {
            name: "python3".to_string(),
            path: PathBuf::from(missing_interpreter),
            kernelspec: JupyterKernelspec {
                argv: vec![
                    missing_interpreter.to_string(),
                    "-m".to_string(),
                    "ipykernel_launcher".to_string(),
                    "-f".to_string(),
                    "{connection_file}".to_string(),
                ],
                display_name: "Python 3".to_string(),
                language: "python".to_string(),
                interrupt_mode: None,
                metadata: None,
                env: None,
            },
        });
        cx.update(|cx| {
            ReplStore::global(cx).update(cx, |store, cx| {
                store.set_active_kernelspec(worktree_id, broken_spec, cx);
            })
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
                .expect("ipynb files should be openable as notebooks")
            })
            .await
            .expect("notebook should parse");

        // Don't render the notebook UI itself: its animated kernel status icon
        // schedules a new frame on every render, which makes `run_until_parked`
        // spin forever in tests. The editor entity is created inside an empty
        // window instead; we are testing execution behavior, not rendering.
        let cx = cx.add_empty_window();

        // Launching a kernel probes real TCP ports on localhost, which the
        // deterministic test scheduler cannot drive.
        cx.executor().allow_parking();

        let editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project.clone(), notebook_item, window, cx))
        });

        // Creating the editor launches the kernel. Wait for the actual launch
        // task, which fails because the interpreter cannot be spawned.
        let pending_kernel = editor.read_with(cx, |editor, _| match &editor.kernel {
            Kernel::StartingKernel(task) => task.clone(),
            _ => panic!("kernel should be starting right after the editor is created"),
        });
        pending_kernel.await;

        editor.read_with(cx, |editor, _| {
            assert!(
                matches!(editor.kernel, Kernel::ErroredLaunch(_)),
                "kernel launch should fail, instead status is: {}",
                editor.kernel.status().to_string()
            );
        });

        // Run the (only) cell via the production action handler.
        editor.update_in(cx, |editor, window, cx| {
            editor.run_current_cell(&Run, window, cx);
        });

        editor.read_with(cx, |editor, cx| {
            let cell_id = editor.cell_order.first().expect("notebook has one cell");
            let Some(Cell::Code(cell)) = editor.cell_map.get(cell_id) else {
                panic!("expected a code cell");
            };
            let cell = cell.read(cx);

            assert!(
                !cell.is_executing(),
                "cell must not be stuck in the executing state when the kernel is not running"
            );

            let nbformat::v4::Cell::Code { outputs, .. } = cell.to_nbformat_cell(cx) else {
                panic!("expected a code cell");
            };
            match outputs.as_slice() {
                [nbformat::v4::Output::Error(error)] => {
                    assert_eq!(error.ename, "Kernel Error");
                    let traceback = error.traceback.join("\n");
                    assert!(
                        traceback.contains("the kernel failed to launch"),
                        "error output should explain why the cell could not run, got: {traceback}"
                    );
                }
                other => panic!("expected a single error output, got: {other:?}"),
            }
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

    const NOTEBOOK_FOR_SEARCH: &str = r#"{
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
                "id": "first-code",
                "metadata": {},
                "execution_count": null,
                "outputs": [],
                "source": ["x = 1\n", "print(x)"]
            },
            {
                "cell_type": "markdown",
                "id": "notes",
                "metadata": {},
                "source": ["the x value"]
            },
            {
                "cell_type": "code",
                "id": "no-match",
                "metadata": {},
                "execution_count": null,
                "outputs": [],
                "source": ["y = 2"]
            },
            {
                "cell_type": "code",
                "id": "last-code",
                "metadata": {},
                "execution_count": null,
                "outputs": [],
                "source": ["x + y"]
            }
        ]
    }"#;

    async fn open_notebook_for_search(
        cx: &mut TestAppContext,
    ) -> (Entity<NotebookEditor>, &mut VisualTestContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "search.ipynb": NOTEBOOK_FOR_SEARCH }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));

        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        // Use an interpreter that doesn't exist so that opening the notebook
        // doesn't start a real kernel.
        let missing_interpreter = path!("/nonexistent/python3");
        let spec = KernelSpecification::Jupyter(LocalKernelSpecification {
            name: "python3".to_string(),
            path: PathBuf::from(missing_interpreter),
            kernelspec: JupyterKernelspec {
                argv: vec![
                    missing_interpreter.to_string(),
                    "-f".to_string(),
                    "{connection_file}".to_string(),
                ],
                display_name: "Python 3".to_string(),
                language: "python".to_string(),
                interrupt_mode: None,
                metadata: None,
                env: None,
            },
        });
        cx.update(|cx| {
            ReplStore::global(cx).update(cx, |store, cx| {
                store.set_active_kernelspec(worktree_id, spec, cx);
            })
        });

        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(
                    &project,
                    &ProjectPath {
                        worktree_id,
                        path: rel_path("search.ipynb").into(),
                    },
                    cx,
                )
                .expect("ipynb files should be openable as notebooks")
            })
            .await
            .expect("notebook should parse");

        let cx = cx.add_empty_window();
        cx.executor().allow_parking();

        let editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project.clone(), notebook_item, window, cx))
        });
        (editor, cx)
    }

    fn text_query(text: &str, replacement: &str) -> Arc<project::search::SearchQuery> {
        Arc::new(
            project::search::SearchQuery::text(
                text,
                true,
                true,
                false,
                util::paths::PathMatcher::default(),
                util::paths::PathMatcher::default(),
                false,
                None,
            )
            .unwrap()
            .with_replacement(replacement.to_string()),
        )
    }

    fn cell_editor(notebook: &NotebookEditor, cell_id: &str, cx: &App) -> Entity<Editor> {
        let cell_id = notebook
            .cell_order
            .iter()
            .find(|id| id.to_string() == cell_id)
            .expect("cell should exist");
        notebook
            .cell_map
            .get(cell_id)
            .and_then(|cell| cell.editor(cx))
            .cloned()
            .expect("cell should have an editor")
    }

    fn markdown_cell(
        notebook: &NotebookEditor,
        index: usize,
    ) -> Entity<crate::notebook::MarkdownCell> {
        match notebook.cell_map.get(&notebook.cell_order[index]) {
            Some(Cell::Markdown(cell)) => cell.clone(),
            _ => panic!("expected a markdown cell at index {index}"),
        }
    }

    #[derive(Debug)]
    struct FakeKernel {
        request_tx: futures::channel::mpsc::Sender<JupyterMessage>,
        stdin_tx: futures::channel::mpsc::Sender<JupyterMessage>,
        working_directory: PathBuf,
        execution_state: runtimelib::ExecutionState,
    }

    impl crate::kernels::RunningKernel for FakeKernel {
        fn request_tx(&self) -> futures::channel::mpsc::Sender<JupyterMessage> {
            self.request_tx.clone()
        }
        fn stdin_tx(&self) -> futures::channel::mpsc::Sender<JupyterMessage> {
            self.stdin_tx.clone()
        }
        fn working_directory(&self) -> &PathBuf {
            &self.working_directory
        }
        fn execution_state(&self) -> &runtimelib::ExecutionState {
            &self.execution_state
        }
        fn set_execution_state(&mut self, state: runtimelib::ExecutionState) {
            self.execution_state = state;
        }
        fn kernel_info(&self) -> Option<&runtimelib::KernelInfoReply> {
            None
        }
        fn set_kernel_info(&mut self, _info: runtimelib::KernelInfoReply) {}
        fn force_shutdown(&mut self, _window: &mut Window, _cx: &mut App) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }
        fn kill(&mut self) {}
    }

    #[gpui::test]
    async fn test_completions_include_code_from_cells_above(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        let (request_tx, mut request_rx) = futures::channel::mpsc::channel(8);
        let (stdin_tx, _stdin_rx) = futures::channel::mpsc::channel(8);
        editor.update(cx, |editor, _| {
            editor.kernel = Kernel::RunningKernel(Box::new(FakeKernel {
                request_tx,
                stdin_tx,
                working_directory: PathBuf::from(path!("/notebooks")),
                execution_state: runtimelib::ExecutionState::Idle,
            }));
        });

        let last_cell_editor =
            editor.read_with(cx, |editor, cx| cell_editor(editor, "last-code", cx));
        let last_cell_id = editor.read_with(cx, |editor, _| editor.cell_order[3].clone());
        let completions = last_cell_editor.update_in(cx, |cell_editor, window, cx| {
            let buffer = cell_editor
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("cells have a single buffer");
            let position = buffer.read(cx).snapshot().anchor_after(1);
            let provider = NotebookCellCompletionProvider {
                notebook: editor.downgrade(),
                cell_id: last_cell_id,
            };
            provider.completions(
                &buffer,
                position,
                CompletionContext {
                    trigger_kind: lsp::CompletionTriggerKind::INVOKED,
                    trigger_character: None,
                },
                window,
                cx,
            )
        });

        let request = request_rx
            .try_recv()
            .ok()
            .expect("a completion request should be sent to the kernel");
        let JupyterMessageContent::CompleteRequest(complete_request) = &request.content else {
            panic!("expected a complete_request, got {:?}", request.content);
        };
        assert_eq!(
            complete_request.code, "x = 1\nprint(x)\ny = 2\nx + y",
            "the code of the code cells above should precede the cell, skipping markdown"
        );
        assert_eq!(complete_request.cursor_pos, 22);

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "_jupyter_types_experimental".to_string(),
            json!([{ "text": "xor", "type": "function", "signature": "(a, b)" }]),
        );
        let reply = CompleteReply {
            matches: vec!["x".to_string(), "xor".to_string()],
            cursor_start: 21,
            cursor_end: 22,
            metadata,
            ..Default::default()
        }
        .as_child_of(&request);
        editor.update_in(cx, |editor, window, cx| editor.route(&reply, window, cx));

        let responses = completions.await.expect("completions should resolve");
        let completions = &responses[0].completions;
        assert_eq!(
            completions
                .iter()
                .map(|completion| completion.new_text.as_str())
                .collect::<Vec<_>>(),
            ["x", "xor"]
        );
        last_cell_editor.update(cx, |cell_editor, cx| {
            let buffer = cell_editor.buffer().read(cx).as_singleton().unwrap();
            let snapshot = buffer.read(cx).snapshot();
            let replace_range = &completions[0].replace_range;
            assert_eq!(
                replace_range.start.to_offset(&snapshot)..replace_range.end.to_offset(&snapshot),
                0..1,
                "the reply's range should map back into the cell being edited"
            );
        });
        assert!(matches!(
            &completions[1].documentation,
            Some(CompletionDocumentation::SingleLine(text)) if text.as_ref() == "function xor(a, b)"
        ));
    }

    fn install_fake_kernel(
        editor: &Entity<NotebookEditor>,
        cx: &mut VisualTestContext,
    ) -> futures::channel::mpsc::Receiver<JupyterMessage> {
        let (request_tx, request_rx) = futures::channel::mpsc::channel(8);
        let (stdin_tx, _stdin_rx) = futures::channel::mpsc::channel(8);
        editor.update(cx, |editor, _| {
            editor.kernel = Kernel::RunningKernel(Box::new(FakeKernel {
                request_tx,
                stdin_tx,
                working_directory: PathBuf::from(path!("/notebooks")),
                execution_state: runtimelib::ExecutionState::Idle,
            }));
        });
        request_rx
    }

    fn jedi_reply(request: &JupyterMessage, json: &str) -> JupyterMessage {
        let hex = json
            .bytes()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        ExecuteReply {
            user_expressions: Some(std::collections::HashMap::from([(
                JEDI_QUERY_EXPRESSION.to_string(),
                ExpressionResult::Ok {
                    data: jupyter_protocol::Media {
                        content: vec![MediaType::Plain(format!("'{hex}'"))],
                    },
                    metadata: Default::default(),
                },
            )])),
            ..Default::default()
        }
        .as_child_of(request)
    }

    #[gpui::test]
    async fn test_definitions_in_cells_above_navigate_to_that_cell(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        let mut request_rx = install_fake_kernel(&editor, cx);

        let last_cell_editor =
            editor.read_with(cx, |editor, cx| cell_editor(editor, "last-code", cx));
        let last_cell_id = editor.read_with(cx, |editor, _| editor.cell_order[3].clone());
        // Cursor on `y` in `x + y`.
        let definitions = last_cell_editor.update(cx, |cell_editor, cx| {
            let buffer = cell_editor.buffer().read(cx).as_singleton().unwrap();
            let position = buffer.read(cx).snapshot().anchor_after(4);
            let provider = NotebookCellSemanticsProvider {
                notebook: editor.downgrade(),
                cell_id: last_cell_id,
            };
            provider
                .definitions(&buffer, position, GotoDefinitionKind::Symbol, cx)
                .expect("a running kernel should be asked for definitions")
        });

        let request = request_rx.try_recv().expect("a jedi query should be sent");
        let JupyterMessageContent::ExecuteRequest(execute_request) = &request.content else {
            panic!("expected an execute_request, got {:?}", request.content);
        };
        assert!(execute_request.silent && !execute_request.store_history);
        let expression = &execute_request.user_expressions.as_ref().unwrap()[JEDI_QUERY_EXPRESSION];
        assert!(
            expression.contains("script.goto(4, 4, follow_imports=True)"),
            "the cursor should be addressed in the code joined from the cells above: {expression}"
        );
        assert!(expression.contains(r#"Interpreter("x = 1\nprint(x)\ny = 2\nx + y""#));

        // jedi finds `y` on line 3 of the joined code, which is the `y = 2` cell.
        let reply = jedi_reply(&request, "[[null, 3, 0, 1]]");
        editor.update_in(cx, |editor, window, cx| editor.route(&reply, window, cx));
        let links = definitions
            .await
            .expect("definitions should resolve")
            .unwrap();
        let target_cell_editor =
            editor.read_with(cx, |editor, cx| cell_editor(editor, "no-match", cx));
        target_cell_editor.update(cx, |target_cell_editor, cx| {
            let target_buffer = target_cell_editor.buffer().read(cx).as_singleton().unwrap();
            assert_eq!(links.len(), 1);
            assert_eq!(links[0].target.buffer, target_buffer);
            let snapshot = target_buffer.read(cx).snapshot();
            let range = &links[0].target.range;
            assert_eq!(
                range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot),
                0..1
            );
        });

        let target_buffer = target_cell_editor.read_with(cx, |target_cell_editor, cx| {
            target_cell_editor.buffer().read(cx).as_singleton().unwrap()
        });
        // The editor calls the navigator while it is being updated.
        let navigator = definition_navigator(editor.downgrade());
        last_cell_editor.update_in(cx, |_, window, cx| {
            let start = language::Point::new(0, 0);
            assert!(navigator(target_buffer, vec![start..start], window, cx));
        });
        cx.run_until_parked();
        editor.read_with(cx, |editor, _| {
            assert_eq!(
                editor.selected_cell_index, 2,
                "navigating should select the cell holding the definition"
            );
        });
    }

    #[test]
    fn test_documentation_text() {
        fn documentation(json: &str) -> Option<String> {
            let request: JupyterMessage = ExecuteRequest::default().into();
            let JupyterMessageContent::ExecuteReply(reply) = jedi_reply(&request, json).content
            else {
                unreachable!()
            };
            documentation_text(&reply).unwrap()
        }

        assert_eq!(
            documentation(r#"[["function", "draw", "draw(n)\n\nDraw n samples.", ""]]"#),
            Some("draw(n)\n\nDraw n samples.".to_string())
        );
        assert_eq!(
            documentation(r#"[["statement", "sample_size", "", "int"]]"#),
            Some("sample_size: int".to_string())
        );
        assert_eq!(
            documentation(r#"[["statement", "values", "", "None"]]"#),
            Some("values".to_string())
        );
        assert_eq!(documentation("[]"), None);
    }

    #[gpui::test]
    async fn test_notebook_exposes_its_buffer_and_selected_cell_editor(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        cx.run_until_parked();
        editor.update(cx, |editor, cx| {
            let buffer = editor
                .content_buffer(cx)
                .expect("notebooks expose their buffer");
            assert_eq!(buffer, editor.notebook_item.read(cx).buffer);

            editor.selected_cell_index = 2;
            let focused_editor = editor
                .focused_editor(cx)
                .and_then(|editor| editor.downcast::<Editor>().ok())
                .expect("the selected cell's editor has the cursor");
            assert_eq!(focused_editor, cell_editor(editor, "no-match", cx));
        });
    }

    #[gpui::test]
    async fn test_choosing_a_language_switches_between_notebook_and_text(cx: &mut TestAppContext) {
        cx.update(|cx| {
            workspace::AppState::test(cx);
            editor::init(cx);
            workspace::register_project_item::<NotebookEditor>(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "search.ipynb": NOTEBOOK_FOR_SEARCH }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));
        let (notebook_language, json_language) = project.read_with(cx, |project, _| {
            let notebook_language = Arc::new(Language::new(
                language::LanguageConfig {
                    name: NOTEBOOK_LANGUAGE_NAME.into(),
                    matcher: Arc::new(language::LanguageMatcher {
                        path_suffixes: vec!["ipynb".into()],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                None,
            ));
            let json_language = Arc::new(Language::new(
                language::LanguageConfig {
                    name: "JSON".into(),
                    ..Default::default()
                },
                None,
            ));
            project.languages().add(notebook_language.clone());
            project.languages().add(json_language.clone());
            (notebook_language, json_language)
        });

        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let notebook = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("search.ipynb")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<NotebookEditor>()
            .expect(".ipynb files open as notebooks");
        cx.run_until_parked();

        let buffer = notebook.read_with(cx, |notebook, cx| {
            notebook.notebook_item.read(cx).buffer.clone()
        });
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer
                .language()
                .map(|language| language.name())),
            Some(NOTEBOOK_LANGUAGE_NAME.into()),
            "loading the file must not switch away from the notebook view"
        );
        let first_cell =
            notebook.read_with(cx, |notebook, cx| cell_editor(notebook, "first-code", cx));
        first_cell.update_in(cx, |first_cell, window, cx| {
            first_cell.set_text("x = 42", window, cx)
        });

        project.update(cx, |project, cx| {
            project.set_language_for_buffer(&buffer, json_language, cx)
        });
        cx.run_until_parked();
        let text_editor = workspace
            .read_with(cx, |workspace, cx| workspace.active_item(cx))
            .and_then(|item| item.downcast::<Editor>())
            .expect("choosing another language shows the file as text");
        text_editor.read_with(cx, |text_editor, cx| {
            assert_eq!(
                text_editor.buffer().read(cx).as_singleton(),
                Some(buffer.clone())
            );
        });
        assert!(
            buffer.read_with(cx, |buffer, _| buffer.text().contains("x = 42")),
            "unsaved notebook changes carry over to the text"
        );

        project.update(cx, |project, cx| {
            project.set_language_for_buffer(&buffer, notebook_language, cx)
        });
        cx.run_until_parked();
        let notebook = workspace
            .read_with(cx, |workspace, cx| workspace.active_item(cx))
            .and_then(|item| item.downcast::<NotebookEditor>())
            .expect("choosing the notebook language shows the notebook again");
        notebook.read_with(cx, |notebook, cx| {
            let first_cell = notebook.cell_map.get(&notebook.cell_order[0]).unwrap();
            assert_eq!(first_cell.current_source(cx), "x = 42");
        });
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(
                workspace.active_pane().read(cx).items_len(),
                1,
                "each switch replaces the tab rather than adding one"
            );
        });
    }

    #[gpui::test]
    async fn test_search_across_cells(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        let query = text_query("x", "z");

        let matches = editor
            .update_in(cx, |editor, window, cx| {
                editor.find_matches(query.clone(), window, cx)
            })
            .await;
        let cell_ids = matches
            .iter()
            .map(|search_match| search_match.cell_id.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            cell_ids,
            ["first-code", "first-code", "notes", "last-code"],
            "matches should come from every searchable cell, in notebook order"
        );

        // With a cell selected that has no match, navigation goes to the
        // nearest match after or before it.
        editor.update_in(cx, |editor, window, cx| {
            editor.selected_cell_index = 2;
            assert_eq!(
                editor.active_match_index(
                    Direction::Next,
                    &matches,
                    SearchToken::default(),
                    window,
                    cx
                ),
                Some(3)
            );
            assert_eq!(
                editor.active_match_index(
                    Direction::Prev,
                    &matches,
                    SearchToken::default(),
                    window,
                    cx
                ),
                Some(2)
            );
        });

        // Activating a match in a rendered markdown cell selects the cell
        // and shows its source.
        editor.update_in(cx, |editor, window, cx| {
            editor.activate_match(2, &matches, SearchToken::default(), window, cx);
            assert_eq!(editor.selected_cell_index, 1);
            assert!(markdown_cell(editor, 1).read(cx).is_editing());
        });

        // Replacing a match edits the cell that contains it.
        editor.update_in(cx, |editor, window, cx| {
            editor.replace(&matches[3], &query, SearchToken::default(), window, cx);
            let last_cell = editor.cell_map.get(&editor.cell_order[3]).unwrap();
            assert_eq!(last_cell.current_source(cx), "z + y");
            let first_cell = editor.cell_map.get(&editor.cell_order[0]).unwrap();
            assert_eq!(first_cell.current_source(cx), "x = 1\nprint(x)");
        });
    }

    #[gpui::test]
    async fn test_search_invalidates_matches_only_on_edits(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        let query = text_query("x", "z");

        let invalidations = Rc::new(RefCell::new(0));
        let _subscription = cx.update(|_, cx| {
            let invalidations = invalidations.clone();
            cx.subscribe(&editor, move |_, event: &SearchEvent, _| {
                if let SearchEvent::MatchesInvalidated = event {
                    *invalidations.borrow_mut() += 1;
                }
            })
        });

        let matches = editor
            .update_in(cx, |editor, window, cx| {
                editor.find_matches(query.clone(), window, cx)
            })
            .await;

        // Highlighting matches must not invalidate them, or the search bar
        // would search the whole notebook again once per cell.
        editor.update_in(cx, |editor, window, cx| {
            editor.update_matches(&matches, Some(0), SearchToken::default(), window, cx);
            editor.update_matches(&matches, Some(1), SearchToken::default(), window, cx);
            editor.clear_matches(window, cx);
        });
        cx.run_until_parked();
        assert_eq!(*invalidations.borrow(), 0);

        let last_cell_editor =
            editor.read_with(cx, |editor, cx| cell_editor(editor, "last-code", cx));
        last_cell_editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&Default::default(), window, cx);
            editor.insert(" x", window, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            *invalidations.borrow(),
            1,
            "editing a cell invalidates matches"
        );

        // Cells added after opening the notebook are searched and tracked too.
        editor.update_in(cx, |editor, window, cx| editor.add_code_block(window, cx));
        cx.run_until_parked();
        assert_eq!(
            *invalidations.borrow(),
            2,
            "adding a cell invalidates matches"
        );

        let new_cell_editor = editor.read_with(cx, |editor, cx| {
            editor
                .get_selected_cell()
                .and_then(|cell| cell.editor(cx))
                .cloned()
                .expect("the new cell should be selected")
        });
        new_cell_editor.update_in(cx, |editor, window, cx| editor.insert("x", window, cx));
        cx.run_until_parked();
        assert_eq!(
            *invalidations.borrow(),
            3,
            "editing a new cell invalidates matches"
        );

        let matches = editor
            .update_in(cx, |editor, window, cx| {
                editor.find_matches(query.clone(), window, cx)
            })
            .await;
        assert_eq!(matches.len(), 6);
    }

    #[gpui::test]
    async fn test_replace_all_across_cells(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        let query = text_query("x", "z");

        let matches = editor
            .update_in(cx, |editor, window, cx| {
                editor.find_matches(query.clone(), window, cx)
            })
            .await;
        editor.update_in(cx, |editor, window, cx| {
            editor.replace_all(
                &mut matches.iter(),
                &query,
                SearchToken::default(),
                window,
                cx,
            );
            let sources = editor
                .cell_order
                .iter()
                .map(|cell_id| editor.cell_map[cell_id].current_source(cx))
                .collect::<Vec<_>>();
            assert_eq!(
                sources,
                ["z = 1\nprint(z)", "the z value", "y = 2", "z + y"]
            );
        });
    }

    #[gpui::test]
    async fn test_match_navigation_starts_from_the_selected_cell(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        let query = text_query("x", "z");

        let matches = editor
            .update_in(cx, |editor, window, cx| {
                editor.find_matches(query.clone(), window, cx)
            })
            .await;
        let token = SearchToken::default();

        editor.update_in(cx, |editor, window, cx| {
            editor.activate_match(1, &matches, token, window, cx);
            assert_eq!(editor.selected_cell_index, 0);
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    1,
                    token,
                    window,
                    cx
                ),
                2
            );
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Prev,
                    1,
                    token,
                    window,
                    cx
                ),
                0
            );
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    2,
                    token,
                    window,
                    cx
                ),
                3
            );
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    4,
                    token,
                    window,
                    cx
                ),
                1,
                "navigating wraps around the notebook"
            );
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    0,
                    token,
                    window,
                    cx
                ),
                1
            );

            // After the user selects a cell without matches, navigation
            // continues from that cell instead of from the previous match.
            editor.selected_cell_index = 2;
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    1,
                    token,
                    window,
                    cx
                ),
                3
            );
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Prev,
                    1,
                    token,
                    window,
                    cx
                ),
                2
            );

            // In command mode the cursor isn't shown, so a newly selected
            // cell is searched from its start.
            editor.selected_cell_index = 3;
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    1,
                    token,
                    window,
                    cx
                ),
                3
            );
        });

        // In edit mode, navigation starts from the cursor of the cell.
        let last_cell_editor =
            editor.read_with(cx, |editor, cx| cell_editor(editor, "last-code", cx));
        last_cell_editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&Default::default(), window, cx)
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.notebook_mode = NotebookMode::Edit;
            editor.selected_cell_index = 3;
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    1,
                    token,
                    window,
                    cx
                ),
                0,
                "there is no match after the cursor, so navigation wraps around"
            );
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Prev,
                    1,
                    token,
                    window,
                    cx
                ),
                3
            );
        });
        last_cell_editor.update_in(cx, |editor, window, cx| {
            editor.move_to_beginning(&Default::default(), window, cx)
        });
        editor.update_in(cx, |editor, window, cx| {
            assert_eq!(
                editor.match_index_for_direction(
                    &matches,
                    1,
                    Direction::Next,
                    1,
                    token,
                    window,
                    cx
                ),
                3,
                "the match at the cursor of a newly selected cell comes first"
            );
        });
    }

    #[gpui::test]
    async fn test_markdown_cells_revealed_by_search_are_rendered_again(cx: &mut TestAppContext) {
        let (editor, cx) = open_notebook_for_search(cx).await;
        let query = text_query("x", "z");

        let matches = editor
            .update_in(cx, |editor, window, cx| {
                editor.find_matches(query.clone(), window, cx)
            })
            .await;
        let token = SearchToken::default();

        editor.update_in(cx, |editor, window, cx| {
            editor.activate_match(2, &matches, token, window, cx);
            assert!(markdown_cell(editor, 1).read(cx).is_editing());

            editor.replace(&matches[2], &query, token, window, cx);
            editor.activate_match(3, &matches, token, window, cx);
            let notes = markdown_cell(editor, 1);
            assert!(
                !notes.read(cx).is_editing(),
                "moving to a match in another cell renders the markdown again"
            );
            assert_eq!(
                notes.read(cx).source(),
                "the z value",
                "the rendered markdown includes the replacement"
            );

            editor.activate_match(2, &matches, token, window, cx);
            assert!(markdown_cell(editor, 1).read(cx).is_editing());
            editor.notebook_mode = NotebookMode::Edit;
            editor.search_bar_visibility_changed(false, window, cx);
            assert!(
                !markdown_cell(editor, 1).read(cx).is_editing(),
                "dismissing the search renders the markdown again"
            );
            assert!(
                editor.notebook_mode == NotebookMode::Command,
                "the search bar gives the focus back to the notebook, in command mode"
            );

            // A cell that the user was already editing is left alone.
            markdown_cell(editor, 1).update(cx, |cell, _| cell.set_editing(true));
            editor.activate_match(2, &matches, token, window, cx);
            editor.activate_match(3, &matches, token, window, cx);
            assert!(markdown_cell(editor, 1).read(cx).is_editing());
        });
    }
}
