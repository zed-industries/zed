use std::{ops::Range, sync::Arc};

use collections::{BTreeMap, HashMap, HashSet, VecDeque};
use futures::{StreamExt as _, future::join_all, stream::FuturesUnordered};
use futures_lite::future::yield_now;
use gpui::{FontRun, MouseButton, SharedString, Task, TaskExt, WeakEntity};
use itertools::Itertools;
use language::{BufferId, BufferSnapshot, ClientCommand};
use multi_buffer::{
    Anchor, BufferOffset, ExcerptRange, MultiBufferRow, MultiBufferSnapshot, ToPoint as _,
};
use parking_lot::Mutex;
use project::{
    CodeAction, TaskSourceKind,
    lsp_store::code_lens::{CodeLensActionId, CodeLensActions, CodeLensResolveTask},
};
use task::TaskContext;
use text::ToOffset as _;

use ui::{Context, Window, div, prelude::*};

use crate::{
    Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT, SelectionEffects,
    actions::ToggleCodeLens,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId, RenderBlock},
    hover_links::HoverLink,
    runnables::RunnableTaskStatus,
};

static EMPTY_LENS_FALLBACK_TITLE: SharedString = SharedString::new_static("0 references");
const CODE_LENS_SEPARATOR: &str = " | ";
const CODE_LENS_FONT_SCALE: f32 = 0.9;
const CODE_LENS_RESOLVE_BATCH_SIZE: usize = 32;

#[derive(Clone, Debug)]
struct CodeLensLine {
    position: Anchor,
    indent_column: u32,
    items: Arc<Mutex<Vec<CodeLensItem>>>,
}

#[derive(Clone, Debug)]
struct CodeLensItem {
    id: CodeLensActionId,
    title: Option<SharedString>,
    action: CodeAction,
}

pub(super) struct CodeLensBlock {
    block_id: CustomBlockId,
    anchor: Anchor,
    line: CodeLensLine,
    pending_line: Option<CodeLensLine>,
}

pub(super) struct CodeLensState {
    pub(super) blocks: HashMap<BufferId, Vec<CodeLensBlock>>,
    actions: HashMap<BufferId, CodeLensActions>,
    indices: HashMap<BufferId, CodeLensIndex>,
    pending_resolves: HashMap<(BufferId, CodeLensActionId), CodeLensResolveTask>,
    resolve_task: Task<()>,
}

impl Default for CodeLensState {
    fn default() -> Self {
        Self {
            blocks: HashMap::default(),
            actions: HashMap::default(),
            indices: HashMap::default(),
            pending_resolves: HashMap::default(),
            resolve_task: Task::ready(()),
        }
    }
}

struct CodeLensIndex {
    positions: Arc<[(text::Anchor, CodeLensActionId)]>,
    items: HashMap<CodeLensActionId, (usize, usize)>,
    edit_count: usize,
}

struct VisibleLensRow {
    buffer_id: BufferId,
    lenses: Vec<CodeLensActionId>,
    next: usize,
    remaining: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LensSlot {
    Displayed,
    Unresolved,
    Hidden,
}

pub(super) fn try_handle_client_command(
    action: &CodeAction,
    editor: &mut Editor,
    workspace: &gpui::Entity<workspace::Workspace>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let Some(command) = action.lsp_action.command() else {
        return false;
    };

    let arguments = command.arguments.as_deref().unwrap_or_default();
    let project = workspace.read(cx).project().clone();
    let client_command = project
        .read(cx)
        .lsp_store()
        .read(cx)
        .language_server_adapter_for_id(action.server_id)
        .and_then(|adapter| adapter.adapter.client_command(&command.command, arguments))
        .or_else(|| match command.command.as_str() {
            "editor.action.showReferences"
            | "editor.action.goToLocations"
            | "editor.action.peekLocations" => Some(ClientCommand::ShowLocations),
            _ => None,
        });

    match client_command {
        Some(ClientCommand::ScheduleTask(task_template)) => {
            schedule_task(task_template, action, editor, workspace, window, cx)
        }
        Some(ClientCommand::ShowLocations) => {
            try_show_references(arguments, action, editor, window, cx)
        }
        None => false,
    }
}

fn schedule_task(
    task_template: task::TaskTemplate,
    action: &CodeAction,
    editor: &mut Editor,
    workspace: &gpui::Entity<workspace::Workspace>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let task_context = TaskContext {
        cwd: task_template.cwd.as_ref().map(std::path::PathBuf::from),
        ..TaskContext::default()
    };
    let language_name = editor
        .buffer()
        .read(cx)
        .buffer(action.range.start.buffer_id)
        .and_then(|buffer| buffer.read(cx).language())
        .map(|language| language.name());
    let task_source_kind = match language_name {
        Some(language_name) => TaskSourceKind::Lsp {
            server: action.server_id,
            language_name: SharedString::from(language_name),
        },
        None => TaskSourceKind::AbsPath {
            id_base: "code-lens".into(),
            abs_path: task_template
                .cwd
                .as_ref()
                .map(std::path::PathBuf::from)
                .unwrap_or_default(),
        },
    };

    let Some(resolved_task) =
        task_template.resolve_task(&task_source_kind.to_id_base(), &task_context)
    else {
        return true;
    };
    let runnable_task_key = editor
        .buffer()
        .read(cx)
        .buffer(action.range.start.buffer_id)
        .and_then(|buffer| {
            let buffer_snapshot = buffer.read(cx).snapshot();
            let buffer_id = buffer_snapshot.remote_id();
            let offset = BufferOffset(action.range.start.to_offset(&buffer_snapshot));
            editor.runnable_task_key_for_offset(buffer_id, offset)
        });
    if let Some((buffer_id, buffer_row)) = runnable_task_key {
        editor.set_runnable_task_status(buffer_id, buffer_row, RunnableTaskStatus::Running, cx);
    }
    let editor_handle = cx.weak_entity();
    workspace.update(cx, |workspace, cx| {
        if let Some((buffer_id, buffer_row)) = runnable_task_key {
            workspace.schedule_resolved_task_with_completion(
                task_source_kind,
                resolved_task,
                false,
                move |result, cx| {
                    editor_handle
                        .update(cx, |editor, cx| {
                            editor.set_runnable_task_status(
                                buffer_id,
                                buffer_row,
                                RunnableTaskStatus::from(result),
                                cx,
                            );
                        })
                        .ok();
                },
                window,
                cx,
            );
        } else {
            workspace.schedule_resolved_task(task_source_kind, resolved_task, false, window, cx);
        }
    });
    true
}

fn try_show_references(
    arguments: &[serde_json::Value],
    action: &CodeAction,
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    if arguments.len() < 3 {
        return false;
    }
    let Ok(locations) = serde_json::from_value::<Vec<lsp::Location>>(arguments[2].clone()) else {
        return false;
    };
    if locations.is_empty() {
        return false;
    }

    let server_id = action.server_id;
    let nav_entry = editor.navigation_entry(editor.selections.newest_anchor().head(), cx);
    let links = locations
        .into_iter()
        .map(|location| HoverLink::LspLocation(location, server_id))
        .collect();
    editor
        .navigate_to_hover_links(None, links, nav_entry, false, window, cx)
        .detach_and_log_err(cx);

    true
}

impl Editor {
    pub(super) fn refresh_code_lenses(
        &mut self,
        for_buffer: Option<BufferId>,
        _window: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.lsp_data_enabled() || self.code_lens.is_none() {
            return;
        }
        let Some(project) = self.project.clone() else {
            return;
        };

        let buffers_to_query = self
            .visible_buffers(cx)
            .into_iter()
            .filter(|buffer| self.is_lsp_relevant(buffer.read(cx).file(), cx))
            .chain(for_buffer.and_then(|buffer_id| self.buffer.read(cx).buffer(buffer_id)))
            .filter(|editor_buffer| {
                let editor_buffer_id = editor_buffer.read(cx).remote_id();
                for_buffer.is_none_or(|buffer_id| buffer_id == editor_buffer_id)
                    && self.registered_buffers.contains_key(&editor_buffer_id)
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        if buffers_to_query.is_empty() {
            return;
        }

        let project = project.downgrade();
        self.refresh_code_lens_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(LSP_REQUEST_DEBOUNCE_TIMEOUT)
                .await;

            let Some(tasks_per_buffer) = project
                .update(cx, |project, cx| {
                    project.lsp_store().update(cx, |lsp_store, cx| {
                        buffers_to_query
                            .into_iter()
                            .map(|buffer| {
                                let buffer_id = buffer.read(cx).remote_id();
                                let task = lsp_store.code_lens_actions(&buffer, cx);
                                async move { (buffer_id, task.await) }
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .ok()
            else {
                return;
            };

            let code_lens_per_buffer = join_all(tasks_per_buffer).await;
            if code_lens_per_buffer.is_empty() {
                return;
            }

            editor
                .update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    for (buffer_id, result) in code_lens_per_buffer {
                        let actions = match result {
                            Ok(Some(actions)) => actions,
                            Ok(None) => continue,
                            Err(e) => {
                                log::error!(
                                    "Failed to fetch code lenses for buffer {buffer_id:?}: {e:#}"
                                );
                                continue;
                            }
                        };
                        editor.apply_lens_actions_for_buffer(buffer_id, actions, &snapshot, cx);
                    }
                    editor.resolve_visible_code_lenses(cx);
                })
                .ok();
        });
    }

    /// Reconcile blocks for `buffer_id` against the latest `actions`.
    ///
    /// Lenses without a command cannot be rendered, as it's the only textual data in the [`lsp::CodeLens`].
    /// Worst case, we can know it only after asynchronously issuing a resolve request to the server.
    /// To avoid flickering during typing, keep a placeholder block for each lens and replace it with a resolved command's block when available,
    /// or with a synthetic "0 references" title if the server resolve did not return a command.
    ///
    /// Also keep the old block until the fresh resolve lands, to avoid flickering during typing.
    fn apply_lens_actions_for_buffer(
        &mut self,
        buffer_id: BufferId,
        actions: CodeLensActions,
        snapshot: &MultiBufferSnapshot,
        cx: &mut Context<Self>,
    ) {
        let mut all_lenses = Vec::new();
        for (id, action) in actions.iter().sorted_by_key(|(id, _)| **id) {
            let Some(position) = snapshot.anchor_in_excerpt(action.range.start) else {
                continue;
            };
            if let project::LspAction::CodeLens(lens) = &action.lsp_action {
                let title = lens
                    .command
                    .as_ref()
                    .filter(|cmd| !cmd.title.is_empty())
                    .map(|cmd| SharedString::from(&cmd.title));
                all_lenses.push((
                    position,
                    CodeLensItem {
                        id: *id,
                        title,
                        action: action.clone(),
                    },
                ));
            }
        }

        let mut new_lines_by_row = group_lenses_by_row(all_lenses, snapshot)
            .map(|line| (MultiBufferRow(line.position.to_point(snapshot).row), line))
            .collect::<HashMap<_, _>>();

        let editor_handle = cx.entity().downgrade();
        let code_lens = self.code_lens.get_or_insert_with(CodeLensState::default);
        let old_blocks = code_lens.blocks.remove(&buffer_id).unwrap_or_default();

        let mut kept_blocks = Vec::new();
        let mut renderers_to_replace = HashMap::default();
        let mut blocks_to_remove = HashSet::default();

        for mut old in old_blocks {
            let row = MultiBufferRow(old.anchor.to_point(snapshot).row);
            let Some(new_line) = new_lines_by_row.remove(&row) else {
                blocks_to_remove.insert(old.block_id);
                continue;
            };
            let new_all_pending = new_line
                .items
                .lock()
                .iter()
                .all(|item| item.title.is_none() && !item.action.resolved);
            let old_has_rendered = old
                .line
                .items
                .lock()
                .iter()
                .any(|item| displayed_title(item).is_some());
            if new_all_pending && old_has_rendered {
                old.pending_line = Some(new_line);
                kept_blocks.push(old);
                continue;
            }
            if rendered_text_matches(&old.line, &new_line) {
                *old.line.items.lock() = std::mem::take(&mut *new_line.items.lock());
                old.pending_line = None;
                kept_blocks.push(old);
            } else {
                let mut updated = old;
                updated.line = new_line.clone();
                updated.pending_line = None;
                renderers_to_replace.insert(
                    updated.block_id,
                    build_code_lens_renderer(new_line, editor_handle.clone()),
                );
                kept_blocks.push(updated);
            }
        }

        let mut to_insert = Vec::new();
        for new_line in new_lines_by_row.into_values() {
            let anchor = new_line.position;
            let props = BlockProperties {
                placement: BlockPlacement::Above(anchor),
                height: Some(1),
                style: BlockStyle::Spacer,
                render: build_code_lens_renderer(new_line.clone(), editor_handle.clone()),
                priority: 0,
            };
            to_insert.push((props, anchor, new_line));
        }

        if !blocks_to_remove.is_empty() {
            self.remove_blocks(blocks_to_remove, None, cx);
        }
        if !renderers_to_replace.is_empty() {
            self.replace_blocks(renderers_to_replace, None, cx);
        }
        if !to_insert.is_empty() {
            let mut props = Vec::with_capacity(to_insert.len());
            let mut metadata = Vec::with_capacity(to_insert.len());
            for (p, anchor, line) in to_insert {
                props.push(p);
                metadata.push((anchor, line));
            }
            let block_ids = self.insert_blocks(props, None, cx);
            for (block_id, (anchor, line)) in block_ids.into_iter().zip(metadata) {
                kept_blocks.push(CodeLensBlock {
                    block_id,
                    anchor,
                    line,
                    pending_line: None,
                });
            }
        }

        let code_lens = self.code_lens.get_or_insert_with(CodeLensState::default);
        match snapshot.buffer_for_id(buffer_id) {
            Some(buffer_snapshot) if !actions.is_empty() => {
                let positions = actions
                    .iter()
                    .map(|(lens_id, action)| {
                        let anchor = action.range.start;
                        (anchor.to_offset(buffer_snapshot), *lens_id, anchor)
                    })
                    .sorted_unstable_by_key(|(offset, lens_id, _)| (*offset, *lens_id))
                    .map(|(_, lens_id, anchor)| (anchor, lens_id))
                    .collect::<Arc<[_]>>();
                let mut items = HashMap::default();
                for (block_index, block) in kept_blocks.iter().enumerate() {
                    let line = block.pending_line.as_ref().unwrap_or(&block.line);
                    for (item_index, item) in line.items.lock().iter().enumerate() {
                        items.insert(item.id, (block_index, item_index));
                    }
                }
                code_lens.indices.insert(
                    buffer_id,
                    CodeLensIndex {
                        positions,
                        items,
                        edit_count: snapshot.edit_count(),
                    },
                );
            }
            _ => {
                code_lens.indices.remove(&buffer_id);
            }
        }
        if actions.is_empty() {
            code_lens.actions.remove(&buffer_id);
        } else {
            code_lens.actions.insert(buffer_id, actions);
        }
        if kept_blocks.is_empty() {
            code_lens.blocks.remove(&buffer_id);
        } else {
            code_lens.blocks.insert(buffer_id, kept_blocks);
        }
        cx.notify();
    }

    pub fn supports_code_lens(&self, cx: &ui::App) -> bool {
        let Some(project) = self.project.as_ref() else {
            return false;
        };
        let lsp_store = project.read(cx).lsp_store().read(cx);
        lsp_store
            .lsp_server_capabilities
            .values()
            .any(|caps| caps.code_lens_provider.is_some())
    }

    pub fn code_lens_enabled(&self) -> bool {
        self.code_lens.is_some()
    }

    pub fn toggle_code_lens_action(
        &mut self,
        _: &ToggleCodeLens,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let currently_enabled = self.code_lens.is_some();
        self.toggle_code_lens(!currently_enabled, window, cx);
    }

    pub(super) fn toggle_code_lens(
        &mut self,
        enabled: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if enabled {
            self.code_lens.get_or_insert_with(CodeLensState::default);
            self.refresh_code_lenses(None, window, cx);
        } else {
            self.clear_code_lenses(cx);
        }
    }

    pub(super) fn resolve_visible_code_lenses(&mut self, cx: &mut Context<Self>) {
        let Some(code_lens) = self.code_lens.as_mut() else {
            return;
        };
        code_lens.resolve_task = Task::ready(());
        if !self.lsp_data_enabled() || self.project.is_none() {
            return;
        }
        let Some(max_items_per_row) = self.max_visible_code_lens_items(cx) else {
            return;
        };

        let inputs = self
            .visible_buffer_ranges(cx)
            .into_iter()
            .filter_map(|(snapshot, range, _)| {
                let positions = self
                    .code_lens
                    .as_ref()?
                    .indices
                    .get(&snapshot.remote_id())?
                    .positions
                    .clone();
                Some((snapshot, range, positions))
            })
            .collect::<Vec<_>>();
        let rows = cx.background_spawn(async move {
            let mut lenses_by_row = BTreeMap::<(BufferId, u32), Vec<CodeLensActionId>>::default();
            for (snapshot, range, positions) in inputs {
                let first = positions
                    .partition_point(|(anchor, _)| anchor.to_offset(&snapshot) < range.start.0);
                for (index, (anchor, lens_id)) in positions[first..].iter().enumerate() {
                    let offset = anchor.to_offset(&snapshot);
                    if offset > range.end.0 {
                        break;
                    }
                    lenses_by_row
                        .entry((snapshot.remote_id(), snapshot.offset_to_point(offset).row))
                        .or_default()
                        .push(*lens_id);
                    if index % CODE_LENS_RESOLVE_BATCH_SIZE == CODE_LENS_RESOLVE_BATCH_SIZE - 1 {
                        yield_now().await;
                    }
                }
            }
            let rows = lenses_by_row
                .into_iter()
                .map(|((buffer_id, _), mut lenses)| {
                    lenses.sort_unstable();
                    lenses.dedup();
                    VisibleLensRow {
                        buffer_id,
                        next: 0,
                        remaining: max_items_per_row,
                        lenses,
                    }
                })
                .collect::<Vec<_>>();
            let queue = (0..rows.len()).collect::<VecDeque<_>>();
            (rows, queue)
        });

        let code_lens = self.code_lens.get_or_insert_with(CodeLensState::default);
        let mut resolves = code_lens
            .pending_resolves
            .iter()
            .map(|(&(buffer_id, lens_id), task)| {
                await_code_lens_resolve(buffer_id, lens_id, task.clone())
            })
            .collect::<FuturesUnordered<_>>();
        code_lens.resolve_task = cx.spawn(async move |editor, cx| {
            let (mut rows, mut queue) = rows.await;
            let mut resolving_rows = HashMap::default();
            loop {
                yield_now().await;
                if editor
                    .update(cx, |editor, cx| {
                        let visible_ranges = editor.visible_buffer_ranges(cx);
                        for _ in 0..CODE_LENS_RESOLVE_BATCH_SIZE {
                            if resolves.len() == CODE_LENS_RESOLVE_BATCH_SIZE {
                                break;
                            }
                            let Some(row_index) = queue.pop_front() else {
                                break;
                            };
                            let row = &mut rows[row_index];
                            if row.remaining == 0 {
                                continue;
                            }
                            let Some(&lens_id) = row.lenses.get(row.next) else {
                                continue;
                            };
                            row.next += 1;
                            let (slot, task) = editor.resolve_code_lens_candidate(
                                row.buffer_id,
                                lens_id,
                                &visible_ranges,
                                cx,
                            );
                            if slot != LensSlot::Hidden {
                                row.remaining -= 1;
                            }
                            if slot == LensSlot::Unresolved {
                                resolving_rows.insert((row.buffer_id, lens_id), row_index);
                            }
                            if let Some(task) = task {
                                resolves.push(await_code_lens_resolve(
                                    row.buffer_id,
                                    lens_id,
                                    task,
                                ));
                            }
                            if row.remaining > 0 && row.next < row.lenses.len() {
                                queue.push_back(row_index);
                            }
                        }
                    })
                    .is_err()
                {
                    return;
                }
                if !queue.is_empty() && resolves.len() < CODE_LENS_RESOLVE_BATCH_SIZE {
                    continue;
                }
                let Some((buffer_id, lens_id, result)) = resolves.next().await else {
                    return;
                };
                let row_index = resolving_rows.remove(&(buffer_id, lens_id));
                if result.is_none()
                    && let Some(row_index) = row_index
                {
                    let row = &mut rows[row_index];
                    if row.remaining == 0 && row.next < row.lenses.len() {
                        queue.push_back(row_index);
                    }
                    row.remaining += 1;
                }
                if editor
                    .update(cx, |editor, cx| {
                        if let Some(code_lens) = &mut editor.code_lens {
                            code_lens.pending_resolves.remove(&(buffer_id, lens_id));
                        }
                        if let Some((resolved_id, resolved)) = result {
                            editor.apply_resolved_code_lens(buffer_id, resolved_id, resolved, cx);
                        }
                    })
                    .is_err()
                {
                    return;
                }
            }
        });
    }

    fn resolve_code_lens_candidate(
        &mut self,
        buffer_id: BufferId,
        lens_id: CodeLensActionId,
        visible_ranges: &[(
            BufferSnapshot,
            Range<BufferOffset>,
            ExcerptRange<text::Anchor>,
        )],
        cx: &mut Context<Self>,
    ) -> (LensSlot, Option<CodeLensResolveTask>) {
        let Some(code_lens) = self.code_lens.as_mut() else {
            return (LensSlot::Hidden, None);
        };
        let Some(action) = code_lens
            .actions
            .get(&buffer_id)
            .and_then(|actions| actions.get(&lens_id))
        else {
            return (LensSlot::Hidden, None);
        };
        let slot = lens_slot(action);
        if slot != LensSlot::Unresolved {
            return (slot, None);
        }
        if !visible_ranges.iter().any(|(snapshot, range, _)| {
            snapshot.remote_id() == buffer_id
                && (range.start.0..=range.end.0).contains(&action.range.start.to_offset(snapshot))
        }) {
            return (LensSlot::Hidden, None);
        }
        if code_lens
            .pending_resolves
            .contains_key(&(buffer_id, lens_id))
        {
            return (LensSlot::Unresolved, None);
        }
        let Some(buffer) = self.buffer.read(cx).buffer(buffer_id) else {
            return (LensSlot::Hidden, None);
        };
        let Some(project) = &self.project else {
            return (LensSlot::Hidden, None);
        };
        let server_id = action.server_id;
        let task = project.read(cx).lsp_store().update(cx, |lsp_store, cx| {
            lsp_store.resolve_code_lens(&buffer, server_id, lens_id, cx)
        });
        code_lens
            .pending_resolves
            .insert((buffer_id, lens_id), task.clone());
        (LensSlot::Unresolved, Some(task))
    }

    fn apply_resolved_code_lens(
        &mut self,
        buffer_id: BufferId,
        resolved_id: CodeLensActionId,
        resolved: CodeAction,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.buffer().read(cx).snapshot(cx);
        let Some(code_lens) = &mut self.code_lens else {
            return;
        };
        let Some(index) = code_lens.indices.get(&buffer_id) else {
            return;
        };
        if index.edit_count != snapshot.edit_count() {
            let Some(mut actions) = code_lens.actions.remove(&buffer_id) else {
                return;
            };
            if let Some(slot) = actions.get_mut(&resolved_id) {
                *slot = resolved;
            }
            self.apply_lens_actions_for_buffer(buffer_id, actions, &snapshot, cx);
            return;
        }
        let Some(&(block_index, item_index)) = index.items.get(&resolved_id) else {
            return;
        };
        let Some(action) = code_lens
            .actions
            .get_mut(&buffer_id)
            .and_then(|actions| actions.get_mut(&resolved_id))
        else {
            return;
        };
        *action = resolved.clone();
        let Some(block) = code_lens
            .blocks
            .get_mut(&buffer_id)
            .and_then(|blocks| blocks.get_mut(block_index))
        else {
            return;
        };
        let replace_renderer = if let Some(line) = block.pending_line.take() {
            block.line = line;
            true
        } else {
            false
        };
        if let Some(item) = block.line.items.lock().get_mut(item_index) {
            item.title = match &resolved.lsp_action {
                project::LspAction::CodeLens(lens) => lens
                    .command
                    .as_ref()
                    .filter(|command| !command.title.is_empty())
                    .map(|command| SharedString::from(&command.title)),
                _ => None,
            };
            item.action = resolved;
        }
        if replace_renderer {
            let block_id = block.block_id;
            let renderer = build_code_lens_renderer(block.line.clone(), cx.entity().downgrade());
            self.replace_blocks(HashMap::from_iter([(block_id, renderer)]), None, cx);
        }
        cx.notify();
    }

    fn max_visible_code_lens_items(&mut self, cx: &mut Context<Self>) -> Option<usize> {
        let display_snapshot = self.display_snapshot(cx);
        let right_edge = *self.visible_x_range(&display_snapshot, cx)?.end();
        let (font, font_size) = self.display_map.read(cx).font_with_size(cx);
        let wrapper = cx
            .text_system()
            .line_wrapper(font.clone(), *font_size * CODE_LENS_FONT_SCALE);
        let separator_width = wrapper
            .layout_line(
                CODE_LENS_SEPARATOR,
                &[FontRun {
                    len: CODE_LENS_SEPARATOR.len(),
                    font_id: wrapper.font_id(),
                }],
            )
            .width;
        (separator_width > Pixels::ZERO)
            .then(|| (right_edge / f64::from(separator_width)).max(0.).floor() as usize + 1)
    }

    pub(super) fn clear_code_lenses(&mut self, cx: &mut Context<Self>) {
        if let Some(code_lens) = self.code_lens.take() {
            let all_blocks = code_lens
                .blocks
                .into_values()
                .flatten()
                .map(|block| block.block_id)
                .collect::<HashSet<_>>();
            if !all_blocks.is_empty() {
                self.remove_blocks(all_blocks, None, cx);
            }
            cx.notify();
        }
        self.refresh_code_lens_task = Task::ready(());
    }

    fn activate_code_lens(
        &mut self,
        position: Anchor,
        action: CodeAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.input_is_deferred(cx) {
            self.defer_input(
                move |editor, window, cx| editor.activate_code_lens(position, action, window, cx),
                window,
                cx,
            );
            return;
        }
        self.change_selections(SelectionEffects::default(), window, cx, |selections| {
            selections.select_anchor_ranges([position..position]);
        });
        if let Some(workspace) = self.workspace() {
            if try_handle_client_command(&action, self, &workspace, window, cx) {
                return;
            }
            let project = workspace.read(cx).project().clone();
            if let Some(buffer) = self.buffer().read(cx).buffer(action.range.start.buffer_id) {
                project
                    .update(cx, |project, cx| {
                        project.apply_code_action(buffer, action, true, cx)
                    })
                    .detach_and_log_err(cx);
            }
        }
    }
}

/// Whether two lens lines would render the same on screen — same indent
/// and same titles in the same order. Used to skip recreating a renderer
/// (and thus a click handler) when nothing about the displayed line
/// changed; the captured [`CodeAction`] inside the existing renderer keeps
/// pointing at the right spot because its anchors track buffer edits.
fn rendered_text_matches(a: &CodeLensLine, b: &CodeLensLine) -> bool {
    a.indent_column == b.indent_column && {
        let left = a.items.lock();
        let right = b.items.lock();
        left.len() == right.len()
            && left
                .iter()
                .zip(right.iter())
                .all(|(left, right)| displayed_title(left) == displayed_title(right))
    }
}

/// Text rendered for a code lens item, or `None` if it should not render
/// (placeholder while resolve is in flight).
fn displayed_title(item: &CodeLensItem) -> Option<&SharedString> {
    item.title
        .as_ref()
        .or_else(|| item.action.resolved.then_some(&EMPTY_LENS_FALLBACK_TITLE))
}

fn group_lenses_by_row(
    lenses: Vec<(Anchor, CodeLensItem)>,
    snapshot: &MultiBufferSnapshot,
) -> impl Iterator<Item = CodeLensLine> {
    lenses
        .into_iter()
        .into_group_map_by(|(position, _)| {
            let row = position.to_point(snapshot).row;
            MultiBufferRow(row)
        })
        .into_iter()
        .filter_map(|(row, entries)| {
            let position = entries.first()?.0;
            let items = Arc::new(Mutex::new(
                entries.into_iter().map(|(_, item)| item).collect(),
            ));
            let indent_column = snapshot.indent_size_for_line(row).len;
            Some(CodeLensLine {
                position,
                indent_column,
                items,
            })
        })
}

async fn await_code_lens_resolve(
    buffer_id: BufferId,
    lens_id: CodeLensActionId,
    task: CodeLensResolveTask,
) -> (
    BufferId,
    CodeLensActionId,
    Option<(CodeLensActionId, CodeAction)>,
) {
    (buffer_id, lens_id, task.await)
}

fn lens_slot(action: &CodeAction) -> LensSlot {
    let project::LspAction::CodeLens(lens) = &action.lsp_action else {
        return LensSlot::Hidden;
    };
    let has_title = lens
        .command
        .as_ref()
        .is_some_and(|command| !command.title.is_empty());
    if has_title || action.resolved {
        LensSlot::Displayed
    } else if lens.command.is_none() {
        LensSlot::Unresolved
    } else {
        LensSlot::Hidden
    }
}

fn build_code_lens_renderer(line: CodeLensLine, editor: WeakEntity<Editor>) -> RenderBlock {
    Arc::new(move |cx| {
        let resolved_items = line
            .items
            .lock()
            .iter()
            .filter_map(|item| {
                let title = displayed_title(item)?;
                let action = item.title.is_some().then(|| item.action.clone());
                Some((title.clone(), action))
            })
            .collect::<Vec<_>>();
        let mut children = Vec::with_capacity((2 * resolved_items.len()).saturating_sub(1));
        let text_style = &cx.editor_style.text;
        let font = text_style.font();
        let font_size = text_style.font_size.to_pixels(cx.window.rem_size()) * CODE_LENS_FONT_SCALE;
        let colors = cx.app.theme().colors();
        let code_lens_color = colors
            .editor_code_lens_foreground
            .unwrap_or(colors.text_muted);

        for (i, (title, action)) in resolved_items.into_iter().enumerate() {
            if i > 0 {
                children.push(
                    div()
                        .font(font.clone())
                        .text_size(font_size)
                        .text_color(code_lens_color)
                        .child(CODE_LENS_SEPARATOR)
                        .into_any_element(),
                );
            }

            children.push(
                div()
                    .id(ElementId::from(i))
                    .font(font.clone())
                    .text_size(font_size)
                    .text_color(code_lens_color)
                    .child(title)
                    .when_some(action, |code_lens_div, action| {
                        let position = line.position;
                        let editor_handle = editor.clone();

                        code_lens_div
                            .cursor_pointer()
                            .hover(|style| style.text_color(cx.app.theme().colors().text))
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_mouse_down(MouseButton::Right, |_, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_click({
                                move |_event, window, cx| {
                                    if let Some(editor) = editor_handle.upgrade() {
                                        editor.update(cx, |editor, cx| {
                                            editor.activate_code_lens(
                                                position,
                                                action.clone(),
                                                window,
                                                cx,
                                            );
                                        });
                                    }
                                }
                            })
                    })
                    .into_any_element(),
            );
        }

        div()
            .id(cx.block_id)
            .pl(cx.em_width * (line.indent_column as f32 + 0.5))
            .h_full()
            .flex()
            .flex_row()
            .items_end()
            .children(children)
            .into_any_element()
    })
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    use collections::HashSet;
    use futures::{FutureExt as _, StreamExt};
    use gpui::{TestAppContext, VisualTestContext, px};
    use indoc::indoc;
    use language::language_settings::SoftWrap;
    use settings::CodeLens;
    use util::path;

    use multi_buffer::{MultiBufferRow, ToPoint as _};
    use text::Point;

    use super::{CODE_LENS_SEPARATOR, displayed_title};
    use crate::{
        Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT,
        editor_tests::{init_test, update_test_editor_settings},
        test::{
            editor_lsp_test_context::EditorLspTestContext, editor_test_context::EditorTestContext,
        },
    };

    #[gpui::test]
    async fn test_captured_input_code_lens_ordering(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(
                format!("ab\n{}\ntarget\ntail", "x".repeat(2_000)),
                window,
                cx,
            );
            editor.change_selections(
                crate::SelectionEffects::no_scroll(),
                window,
                cx,
                |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                },
            );
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let position = snapshot.anchor_after(Point::new(2, 0));
            let buffer_position = snapshot
                .anchor_to_buffer_anchor(position)
                .expect("lens position")
                .0;
            let action = project::CodeAction {
                server_id: lsp::LanguageServerId(0),
                range: buffer_position..buffer_position,
                lsp_action: project::LspAction::Command(lsp::Command {
                    title: String::from("lens"),
                    command: String::from("lens"),
                    arguments: None,
                }),
                resolved: true,
            };
            editor.move_down(&crate::MoveDown, window, cx);
            assert!(editor.input_is_deferred(cx));
            editor.activate_code_lens(position, action, window, cx);
            assert_eq!(
                editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head(),
                Point::new(0, 1)
            );
            editor.input_completion(cx)
        });
        completion.await.expect("lens input completed");
        cx.editor(|editor, _, cx| {
            assert_eq!(
                editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head(),
                Point::new(2, 0)
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_blocks(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                        command: Some(lsp::Command {
                            title: "2 references".to_owned(),
                            command: "lens_cmd".to_owned(),
                            arguments: None,
                        }),
                        data: None,
                    },
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 19)),
                        command: Some(lsp::Command {
                            title: "0 references".to_owned(),
                            command: "lens_cmd".to_owned(),
                            arguments: None,
                        }),
                        data: None,
                    },
                ]))
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received a code lens request"
        );
        cx.run_until_parked();

        cx.editor(|editor, _, cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                true,
                "code lens should be enabled"
            );
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 2 references
                    Line 1: function hello() {}

                    Lenses: 0 references
                    Line 2: function world() {}
                "#},
                "both lenses should render their server-provided titles"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_refresh_requeries_open_document(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let lens_title = Arc::new(Mutex::new("Initial lens".to_string()));
        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>({
                let lens_title = lens_title.clone();
                move |_, _, _| {
                    let lens_title = lens_title.clone();
                    async move {
                        let title = lens_title.lock().unwrap().clone();
                        Ok(Some(vec![lsp::CodeLens {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 0),
                                lsp::Position::new(0, 19),
                            ),
                            command: Some(lsp::Command {
                                title,
                                command: "lens_cmd".to_owned(),
                                arguments: None,
                            }),
                            data: None,
                        }]))
                    }
                }
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");
        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();
        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: Initial lens
                    Line 1: function hello() {}
                "#},
                "initial fetch should render the server title"
            );
        });

        *lens_title.lock().unwrap() = "Refreshed lens".to_string();
        cx.lsp
            .request::<lsp::request::CodeLensRefresh>((), lsp::DEFAULT_LSP_REQUEST_TIMEOUT)
            .await
            .into_response()
            .expect("code lens refresh request failed");
        cx.executor()
            .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT * 2);
        cx.run_until_parked();
        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: Refreshed lens
                    Line 1: function hello() {}
                "#},
                "refresh should update the displayed lens to the new server title"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_dynamic_registration_requeries_open_document(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        // The server advertises no code lens capability up front; it registers
        // `textDocument/codeLens` dynamically only after the document is open.
        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let _code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: Some(lsp::Command {
                        title: "Dynamic lens".to_owned(),
                        command: "lens_cmd".to_owned(),
                        arguments: None,
                    }),
                    data: None,
                }]))
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");
        // Drain any debounced refresh scheduled before the capability exists, so
        // the post-registration re-query can only come from the dynamic
        // registration handling itself.
        cx.executor()
            .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT * 2);
        cx.run_until_parked();
        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                "\n",
                "no lenses should render before the capability is registered"
            );
        });

        cx.lsp
            .request::<lsp::request::RegisterCapability>(
                lsp::RegistrationParams {
                    registrations: vec![lsp::Registration {
                        id: "code-lens".to_string(),
                        method: "textDocument/codeLens".to_string(),
                        register_options: Some(
                            serde_json::to_value(lsp::CodeLensOptions {
                                resolve_provider: None,
                            })
                            .unwrap(),
                        ),
                    }],
                },
                lsp::DEFAULT_LSP_REQUEST_TIMEOUT,
            )
            .await
            .into_response()
            .expect("register capability request failed");
        cx.executor()
            .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT * 2);
        cx.run_until_parked();
        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: Dynamic lens
                    Line 1: function hello() {}
                "#},
                "dynamic textDocument/codeLens registration should re-query and display lenses for the open document"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_blocks_kept_across_refresh(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: Some(lsp::Command {
                        title: "1 reference".to_owned(),
                        command: "lens_cmd".to_owned(),
                        arguments: None,
                    }),
                    data: None,
                }]))
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        let initial_block_ids = cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 1 reference
                    Line 1: function hello() {}
                "#},
                "initial fetch should render the server title"
            );
            editor
                .code_lens
                .as_ref()
                .map(|s| {
                    s.blocks
                        .values()
                        .flatten()
                        .map(|b| b.block_id)
                        .collect::<HashSet<_>>()
                })
                .unwrap_or_default()
        });

        cx.update_editor(|editor, window, cx| {
            editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
            editor.handle_input("\n// trailing comment", window, cx);
        });
        cx.executor()
            .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT + Duration::from_millis(50));
        assert!(
            code_lens_request.next().await.is_some(),
            "should have received another code lens request after edit"
        );
        cx.run_until_parked();

        let refreshed_block_ids = cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 1 reference
                    Line 1: function hello() {}
                "#},
                "refreshed block should keep rendering the same title"
            );
            editor
                .code_lens
                .as_ref()
                .map(|s| {
                    s.blocks
                        .values()
                        .flatten()
                        .map(|b| b.block_id)
                        .collect::<HashSet<_>>()
                })
                .unwrap_or_default()
        });
        assert_eq!(
            refreshed_block_ids, initial_block_ids,
            "Code lens blocks should be preserved across refreshes when their content is unchanged"
        );
    }

    #[gpui::test]
    async fn test_code_lens_blocks_kept_when_only_resolve_fills_titles(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        // The LSP returns shallow code lenses on every fetch; only `resolve`
        // populates the command/title. This is the realistic flow with
        // servers like rust-analyzer and exercises the path where each
        // post-edit refresh comes back unresolved before the resolve catches
        // up.
        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: None,
                    data: Some(serde_json::json!({"id": "lens_1"})),
                }]))
            });

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>(|lens, _| async move {
                Ok(lsp::CodeLens {
                    command: Some(lsp::Command {
                        title: "1 reference".to_owned(),
                        command: "resolved_cmd".to_owned(),
                        arguments: None,
                    }),
                    ..lens
                })
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        let initial = cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 1 reference
                    Line 1: function hello() {}
                "#},
                "resolve should fill the placeholder with the server title"
            );
            editor
                .code_lens
                .as_ref()
                .map(|s| {
                    s.blocks
                        .values()
                        .flatten()
                        .map(|b| b.block_id)
                        .collect::<HashSet<_>>()
                })
                .unwrap_or_default()
        });

        for keystroke in [" ", "x", "y"] {
            cx.update_editor(|editor, window, cx| {
                editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
                editor.handle_input(keystroke, window, cx);
            });
            cx.executor()
                .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT + Duration::from_millis(50));
            assert!(
                code_lens_request.next().await.is_some(),
                "should have received another (shallow) code lens request after edit"
            );
            cx.run_until_parked();

            let after = cx.editor(|editor, _, cx| {
                assert_eq!(
                    code_lens_assertion_text(editor, cx),
                    indoc! {r#"
                        Lenses: 1 reference
                        Line 1: function hello() {}
                    "#},
                    "refresh+resolve cycle should keep rendering the same title"
                );
                editor
                    .code_lens
                    .as_ref()
                    .map(|s| {
                        s.blocks
                            .values()
                            .flatten()
                            .map(|b| b.block_id)
                            .collect::<HashSet<_>>()
                    })
                    .unwrap_or_default()
            });
            assert_eq!(
                after, initial,
                "Block IDs must survive the unresolved-fetch → resolve cycle without churn"
            );
        }
    }

    #[gpui::test]
    async fn test_code_lens_placeholder_block_before_resolve(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                let mut lenses = Vec::new();
                lenses.push(lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: None,
                    data: Some(serde_json::json!({"id": "lens_1"})),
                });
                Ok(Some(lenses))
            });

        let (resolve_tx, resolve_rx) = futures::channel::oneshot::channel::<()>();
        let resolve_rx = std::sync::Mutex::new(Some(resolve_rx));
        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>(move |lens, _| {
                let rx = resolve_rx.lock().unwrap().take();
                async move {
                    if let Some(rx) = rx {
                        rx.await.ok();
                    }
                    Ok(lsp::CodeLens {
                        command: Some(lsp::Command {
                            title: "1 reference".to_owned(),
                            command: "resolved_cmd".to_owned(),
                            arguments: None,
                        }),
                        ..lens
                    })
                }
            });

        cx.set_state("ˇfunction hello() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: <placeholder>
                    Line 1: function hello() {}
                "#},
                "placeholder spacer should be reserved with no rendered text before resolve"
            );
        });

        resolve_tx.send(()).ok();
        cx.run_until_parked();

        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 1 reference
                    Line 1: function hello() {}
                "#},
                "after resolve the placeholder should display the server title"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_placeholder_kept_when_resolve_yields_empty_title(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                let mut lenses = Vec::new();
                lenses.push(lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: None,
                    data: Some(serde_json::json!({"id": "lens_1"})),
                });
                Ok(Some(lenses))
            });

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>(|lens, _| async move {
                Ok(lsp::CodeLens {
                    command: Some(lsp::Command {
                        title: String::new(),
                        command: "noop".to_owned(),
                        arguments: None,
                    }),
                    ..lens
                })
            });

        cx.set_state("ˇfunction hello() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 0 references
                    Line 1: function hello() {}
                "#},
                "lens resolved to an empty title should fall back to the synthetic label"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_same_range_lenses_resolve_independently(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        // Two shallow lenses on the same range, distinguished only by `data`
        // — exactly the shape vtsls/TypeScript-LS uses for the
        // "references" + "implementations" pair on the same line.
        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                        command: None,
                        data: Some(serde_json::json!({"kind": "references"})),
                    },
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                        command: None,
                        data: Some(serde_json::json!({"kind": "implementations"})),
                    },
                ]))
            });

        let resolve_calls = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));
        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>({
                let resolve_calls = resolve_calls.clone();
                move |lens, _| {
                    let resolve_calls = resolve_calls.clone();
                    async move {
                        let kind = lens
                            .data
                            .as_ref()
                            .and_then(|d| d.get("kind"))
                            .cloned()
                            .unwrap_or(serde_json::Value::Null);
                        resolve_calls.lock().unwrap().push(kind.clone());
                        let title = match kind.as_str() {
                            Some("references") => "2 references",
                            Some("implementations") => "1 implementation",
                            _ => "",
                        };
                        Ok(lsp::CodeLens {
                            command: Some(lsp::Command {
                                title: title.to_owned(),
                                command: "noop".to_owned(),
                                arguments: None,
                            }),
                            ..lens
                        })
                    }
                }
            });

        cx.set_state("ˇfunction hello() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        let calls = resolve_calls.lock().unwrap().clone();
        assert_eq!(
            calls.len(),
            2,
            "both same-range lenses should be resolved independently, got {calls:?}"
        );
        let kinds: Vec<&str> = calls.iter().filter_map(|v| v.as_str()).collect();
        assert_eq!(kinds.contains(&"references"), true);
        assert_eq!(kinds.contains(&"implementations"), true);

        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 2 references | 1 implementation
                    Line 1: function hello() {}
                "#},
                "both same-range lenses should render their resolved titles"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_placeholder_kept_when_resolve_yields_no_command(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: None,
                    data: Some(serde_json::json!({"id": "lens_1"})),
                }]))
            });

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>(|lens, _| async move {
                Ok(lsp::CodeLens {
                    command: None,
                    ..lens
                })
            });

        cx.set_state("ˇfunction hello() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        cx.editor(|editor, _, cx| {
            assert_eq!(
                code_lens_assertion_text(editor, cx),
                indoc! {r#"
                    Lenses: 0 references
                    Line 1: function hello() {}
                "#},
                "lens resolved without a command should fall back to the synthetic label"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_disabled_by_default(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensRequest, _, _>(|_, _| async move {
                panic!("Should not request code lenses when disabled");
            });

        cx.set_state("ˇfunction hello() {}");
        cx.run_until_parked();

        cx.editor(|editor, _, _cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                false,
                "code lens should not be enabled when setting is off"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_toggling(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: Some(lsp::Command {
                        title: "1 reference".to_owned(),
                        command: "lens_cmd".to_owned(),
                        arguments: None,
                    }),
                    data: None,
                }]))
            });

        cx.set_state("ˇfunction hello() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received a code lens request"
        );
        cx.run_until_parked();

        cx.editor(|editor, _, _cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                true,
                "code lens should be enabled"
            );
            let total_blocks: usize = editor
                .code_lens
                .as_ref()
                .map(|s| s.blocks.values().map(|v| v.len()).sum())
                .unwrap_or(0);
            assert_eq!(total_blocks, 1, "Should have one code lens block");
        });

        cx.update_editor(|editor, _window, cx| {
            editor.clear_code_lenses(cx);
        });

        cx.editor(|editor, _, _cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                false,
                "code lens should be disabled after clearing"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_resolve(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                        command: None,
                        data: Some(serde_json::json!({"id": "lens_1"})),
                    },
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 19)),
                        command: None,
                        data: Some(serde_json::json!({"id": "lens_2"})),
                    },
                ]))
            });

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>(|lens, _| async move {
                let id = lens
                    .data
                    .as_ref()
                    .and_then(|d| d.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let title = match id {
                    "lens_1" => "3 references",
                    "lens_2" => "1 implementation",
                    _ => "unknown",
                };
                Ok(lsp::CodeLens {
                    command: Some(lsp::Command {
                        title: title.to_owned(),
                        command: format!("resolved_{id}"),
                        arguments: None,
                    }),
                    ..lens
                })
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received a code lens request"
        );
        cx.run_until_parked();

        cx.editor(|editor, _, _cx| {
            let total_blocks: usize = editor
                .code_lens
                .as_ref()
                .map(|s| s.blocks.values().map(|v| v.len()).sum())
                .unwrap_or(0);
            assert_eq!(
                total_blocks, 2,
                "Unresolved lenses should have been resolved and displayed"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_resolve_only_visible(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let line_count: u32 = 100;
        let lens_every: u32 = 10;
        let lines = (0..line_count)
            .map(|i| format!("function func_{i}() {{}}"))
            .collect::<Vec<_>>()
            .join("\n");

        let lens_lines = (0..line_count)
            .filter(|i| i % lens_every == 0)
            .collect::<Vec<_>>();

        let resolved_lines = Arc::new(Mutex::new(Vec::<u32>::new()));

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), serde_json::json!({ "main.ts": lines }))
            .await;

        let project = project::Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "TypeScript".into(),
                matcher: (language::LanguageMatcher {
                    path_suffixes: vec!["ts".into()],
                    ..language::LanguageMatcher::default()
                })
                .into(),
                ..language::LanguageConfig::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        )));

        let mut fake_servers = language_registry.register_fake_lsp(
            "TypeScript",
            language::FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    code_lens_provider: Some(lsp::CodeLensOptions {
                        resolve_provider: Some(true),
                    }),
                    ..lsp::ServerCapabilities::default()
                },
                ..language::FakeLspAdapter::default()
            },
        );

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    std::path::PathBuf::from(path!("/dir/main.ts")),
                    workspace::OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let fake_server = fake_servers.next().await.unwrap();

        let lens_lines_for_handler = lens_lines.clone();
        fake_server.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _| {
            let lens_lines = lens_lines_for_handler.clone();
            async move {
                Ok(Some(
                    lens_lines
                        .iter()
                        .map(|&line| lsp::CodeLens {
                            range: lsp::Range::new(
                                lsp::Position::new(line, 0),
                                lsp::Position::new(line, 10),
                            ),
                            command: None,
                            data: Some(serde_json::json!({ "line": line })),
                        })
                        .collect(),
                ))
            }
        });

        {
            let resolved_lines = resolved_lines.clone();
            fake_server.set_request_handler::<lsp::request::CodeLensResolve, _, _>(
                move |lens, _| {
                    let resolved_lines = resolved_lines.clone();
                    async move {
                        let line = lens
                            .data
                            .as_ref()
                            .and_then(|d| d.get("line"))
                            .and_then(|v| v.as_u64())
                            .unwrap() as u32;
                        resolved_lines.lock().unwrap().push(line);
                        Ok(lsp::CodeLens {
                            command: Some(lsp::Command {
                                title: format!("{line} references"),
                                command: format!("show_refs_{line}"),
                                arguments: None,
                            }),
                            ..lens
                        })
                    }
                },
            );
        }

        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();

        let initial_resolved = resolved_lines
            .lock()
            .unwrap()
            .drain(..)
            .collect::<HashSet<_>>();
        assert_eq!(
            initial_resolved,
            HashSet::from_iter([0, 10, 20, 30, 40]),
            "Only lenses visible at the top should be resolved"
        );

        editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();

        let after_scroll_resolved = resolved_lines
            .lock()
            .unwrap()
            .drain(..)
            .collect::<HashSet<_>>();
        // Once the lenses are first applied we insert a placeholder block per
        // lens row so the line is reserved while the resolve is in flight.
        // Those placeholder blocks add display height, so after scrolling to
        // the end the visible buffer-row range is slightly smaller than it
        // would be without them, and lens row 60 is just outside it.
        assert_eq!(
            after_scroll_resolved,
            HashSet::from_iter([70, 80, 90]),
            "Only newly visible lenses at the bottom should be resolved, not middle ones"
        );
    }

    #[gpui::test]
    async fn test_code_lens_resolve_on_horizontally_hidden_column(cx: &mut TestAppContext) {
        let lens_column = 1_500;
        let text = format!(
            "/*{}*/function f() {{}}\n",
            "x".repeat(lens_column as usize - 4)
        );
        let (resolved, _) =
            resolve_single_row_code_lenses(text, vec![lens_column], ViewportChange::None, 0, cx)
                .await;
        assert_eq!(resolved, 1);
    }

    #[gpui::test]
    async fn test_code_lens_failed_resolves_do_not_use_visible_slots(cx: &mut TestAppContext) {
        let lens_columns = (0..2_000).map(|index| index * 2).collect::<Vec<_>>();
        let text = format!("{}\n", "x".repeat(5_000));
        let failing_resolves = 5;
        let (resolved, visible_columns) = resolve_single_row_code_lenses(
            text,
            lens_columns,
            ViewportChange::None,
            failing_resolves,
            cx,
        )
        .await;
        let separator_columns = CODE_LENS_SEPARATOR.len() as f64 * 0.9;
        assert_eq!(
            resolved,
            (visible_columns / separator_columns).floor() as usize + 1 + failing_resolves
        );
    }

    #[gpui::test]
    async fn test_code_lens_failed_resolves_retry_on_later_visibility_passes(
        cx: &mut TestAppContext,
    ) {
        let text = format!("{}\n", "x".repeat(5_000));
        let (resolved, _) =
            resolve_single_row_code_lenses(text, vec![0], ViewportChange::Scroll(600.), 1, cx)
                .await;
        assert_eq!(resolved, 2);
    }

    #[gpui::test]
    async fn test_code_lens_failed_resolves_stop_after_their_row_leaves_view(
        cx: &mut TestAppContext,
    ) {
        let lens_columns = (0..2_000).map(|index| index * 2).collect::<Vec<_>>();
        let text = format!("{}{}", "x".repeat(5_000), "\n".repeat(500));
        resolve_single_row_code_lenses(
            text,
            lens_columns,
            ViewportChange::ScrollAwayWhileResolving,
            2_000,
            cx,
        )
        .await;
    }

    #[gpui::test]
    async fn test_code_lens_resolve_follows_viewport_growth(cx: &mut TestAppContext) {
        let lens_columns = (0..2_000).map(|index| index * 2).collect::<Vec<_>>();
        let text = format!("{}\n", "x".repeat(5_000));
        let (resolved, visible_columns) =
            resolve_single_row_code_lenses(text, lens_columns, ViewportChange::Widen, 0, cx).await;
        let separator_columns = CODE_LENS_SEPARATOR.len() as f64 * 0.9;
        assert_eq!(
            resolved,
            (visible_columns / separator_columns).floor() as usize + 1
        );
    }

    #[gpui::test]
    async fn test_code_lens_resolve_follows_horizontal_scroll(cx: &mut TestAppContext) {
        let lens_columns = (0..2_000).map(|index| index * 2).collect::<Vec<_>>();
        let text = format!("{}\n", "x".repeat(5_000));
        let scroll_columns = 600.;
        let (resolved, visible_columns) = resolve_single_row_code_lenses(
            text,
            lens_columns,
            ViewportChange::Scroll(scroll_columns),
            0,
            cx,
        )
        .await;
        let separator_columns = CODE_LENS_SEPARATOR.len() as f64 * 0.9;
        assert_eq!(
            resolved,
            ((scroll_columns + visible_columns) / separator_columns).floor() as usize + 1
        );
    }

    #[gpui::test]
    async fn test_code_lens_dense_row_resolves_in_bounded_batches(cx: &mut TestAppContext) {
        let lens_columns = (0..97).rev().map(|index| index * 2).collect::<Vec<_>>();
        let (resolved, _) = resolve_single_row_code_lenses(
            format!("{}{}x", "x".repeat(5_000), "\n".repeat(100)),
            lens_columns,
            ViewportChange::BoundedScroll(600.),
            0,
            cx,
        )
        .await;
        assert_eq!(resolved, 98);
    }

    async fn resolve_single_row_code_lenses(
        text: String,
        lens_columns: Vec<u32>,
        viewport_change: ViewportChange,
        failing_resolves: usize,
        cx: &mut TestAppContext,
    ) -> (usize, f64) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });
        let resolved = Arc::new(Mutex::new(0));
        let (release_tx, release_rx) = futures::channel::oneshot::channel::<()>();
        let release = release_rx.shared();
        let bounded = matches!(viewport_change, ViewportChange::BoundedScroll(_));
        let expected_columns = bounded.then(|| lens_columns.clone());
        let hold_resolves =
            bounded || matches!(viewport_change, ViewportChange::ScrollAwayWhileResolving);

        let mut context = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        if let ViewportChange::Widen = viewport_change {
            context.simulate_resize(gpui::size(px(600.), px(800.)));
        } else if bounded {
            context.simulate_resize(gpui::size(px(2_400.), px(800.)));
        }
        let fake_server = &context.lsp;

        fake_server.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _| {
            let lens_columns = lens_columns.clone();
            async move {
                Ok(Some(
                    lens_columns
                        .into_iter()
                        .map(|column| lsp::CodeLens {
                            range: lsp::Range::new(
                                lsp::Position::new(0, column),
                                lsp::Position::new(0, column + 1),
                            ),
                            command: None,
                            data: Some(serde_json::json!({ "column": column })),
                        })
                        .chain(bounded.then(|| lsp::CodeLens {
                            range: lsp::Range::new(
                                lsp::Position::new(100, 0),
                                lsp::Position::new(100, 1),
                            ),
                            command: None,
                            data: Some(serde_json::json!({ "column": 0 })),
                        }))
                        .collect(),
                ))
            }
        });
        {
            let resolved = resolved.clone();
            fake_server.set_request_handler::<lsp::request::CodeLensResolve, _, _>(
                move |lens, _| {
                    let request_count = {
                        let mut resolved = resolved.lock().unwrap();
                        *resolved += 1;
                        *resolved
                    };
                    let release = release.clone();
                    async move {
                        if hold_resolves {
                            release.await.ok();
                        }
                        if request_count <= failing_resolves {
                            return Err(anyhow::anyhow!("Simulated resolve failure"));
                        }
                        Ok(lsp::CodeLens {
                            command: Some(lsp::Command {
                                title: if bounded {
                                    format!(
                                        "lens {}:{}",
                                        lens.range.start.line, lens.range.start.character
                                    )
                                } else {
                                    "1 reference".to_string()
                                },
                                command: "show_refs".to_string(),
                                arguments: None,
                            }),
                            ..lens
                        })
                    }
                },
            );
        }

        context.set_state(&format!("ˇ{text}"));
        let editor = context.editor.clone();
        let cx = &mut context.cx.cx;
        if bounded {
            editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(SoftWrap::None, cx)
            });
        }
        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();

        let visible_columns = |cx: &mut VisualTestContext| {
            editor
                .read_with(cx, |editor, _| editor.visible_column_count())
                .expect("a rendered editor")
        };
        match viewport_change {
            ViewportChange::None => {}
            ViewportChange::Scroll(scroll_columns) => {
                editor.update_in(cx, |editor, window, cx| {
                    editor.set_scroll_position(gpui::point(scroll_columns, 0.), window, cx);
                });
                cx.executor().advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                let scrolled = editor.update(cx, |editor, cx| editor.scroll_manager.offset(cx).x);
                assert!(
                    (scrolled - scroll_columns).abs() < 0.01,
                    "{scrolled} != {scroll_columns}"
                );
            }
            ViewportChange::BoundedScroll(scroll_columns) => {
                assert_eq!(*resolved.lock().unwrap(), 32);
                let positions = editor.read_with(cx, |editor, _| {
                    editor
                        .code_lens
                        .as_ref()
                        .expect("code lenses enabled")
                        .indices
                        .values()
                        .next()
                        .expect("a lens index")
                        .positions
                        .clone()
                });
                editor.update_in(cx, |editor, window, cx| {
                    editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
                });
                cx.executor().advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                assert_eq!(*resolved.lock().unwrap(), 32);
                release_tx.send(()).expect("resolves are waiting");
                cx.run_until_parked();
                assert_eq!(*resolved.lock().unwrap(), 33);
                editor.update_in(cx, |editor, window, cx| {
                    editor.set_scroll_position(gpui::point(scroll_columns, 0.), window, cx);
                });
                cx.executor().advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                editor.update_in(cx, |editor, window, cx| {
                    let em_width = f64::from(
                        editor
                            .last_position_map
                            .as_ref()
                            .expect("painted editor")
                            .em_layout_width,
                    );
                    let expected_x = window.pixel_snap_f64(scroll_columns * em_width) / em_width;
                    assert_eq!(editor.scroll_position(cx), gpui::point(expected_x, 0.));
                });
                editor.read_with(cx, |editor, cx| {
                    let code_lens = editor.code_lens.as_ref().expect("code lenses enabled");
                    assert_eq!(code_lens.pending_resolves.len(), 0);
                    assert!(Arc::ptr_eq(
                        &positions,
                        &code_lens
                            .indices
                            .values()
                            .next()
                            .expect("a lens index")
                            .positions,
                    ));
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let mut blocks = code_lens.blocks.values().flatten().collect::<Vec<_>>();
                    blocks.sort_by_key(|block| block.anchor.to_point(&snapshot).row);
                    assert_eq!(blocks.len(), 2);
                    assert_eq!(
                        blocks
                            .iter()
                            .map(|block| {
                                block
                                    .line
                                    .items
                                    .lock()
                                    .iter()
                                    .map(|item| {
                                        displayed_title(item).expect("a resolved title").to_string()
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>(),
                        vec![
                            expected_columns
                                .as_ref()
                                .expect("expected lens columns")
                                .iter()
                                .map(|column| format!("lens 0:{column}"))
                                .collect::<Vec<_>>(),
                            vec!["lens 100:0".to_string()],
                        ],
                    );
                });
            }
            ViewportChange::Widen => {
                let narrow_columns = visible_columns(cx);
                cx.simulate_resize(gpui::size(px(2_400.), px(800.)));
                cx.run_until_parked();
                cx.executor().advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                assert!(
                    visible_columns(cx) > 2. * narrow_columns,
                    "{} <= 2 * {narrow_columns}",
                    visible_columns(cx)
                );
            }
            ViewportChange::ScrollAwayWhileResolving => {
                let requested = *resolved.lock().unwrap();
                assert!(requested > 0);
                editor.update_in(cx, |editor, window, cx| {
                    editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
                });
                cx.executor().advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                release_tx.send(()).ok();
                cx.run_until_parked();
                cx.executor().advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                assert_eq!(*resolved.lock().unwrap(), requested);
            }
        }
        let resolved = *resolved.lock().unwrap();
        (resolved, visible_columns(cx))
    }

    enum ViewportChange {
        None,
        Scroll(f64),
        BoundedScroll(f64),
        Widen,
        ScrollAwayWhileResolving,
    }

    fn code_lens_assertion_text(editor: &Editor, cx: &ui::App) -> String {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let mut blocks = editor
            .code_lens
            .as_ref()
            .map(|state| state.blocks.values().flatten().collect::<Vec<_>>())
            .unwrap_or_default();
        blocks.sort_by_key(|block| block.anchor.to_point(&snapshot).row);

        let lens_label = "Lenses";
        let line_label = "Line";
        let mut text = blocks
            .into_iter()
            .map(|block| {
                let row = block.anchor.to_point(&snapshot).row;
                let line_len = snapshot.line_len(MultiBufferRow(row));
                let line_text = snapshot
                    .text_for_range(Point::new(row, 0)..Point::new(row, line_len))
                    .collect::<String>();
                let lens_text = block
                    .line
                    .items
                    .lock()
                    .iter()
                    .map(|item| {
                        displayed_title(item)
                            .map(|title| title.to_string())
                            .unwrap_or_else(|| "<placeholder>".to_string())
                    })
                    .collect::<Vec<_>>()
                    .join(CODE_LENS_SEPARATOR);
                let line_number = row + 1;
                let line_label = format!("{line_label} {line_number}");
                let label_width = line_label.len().max(lens_label.len());
                format!(
                    "{lens_label:<label_width$}: {lens_text}\n\
                     {line_label:<label_width$}: {line_text}"
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        text.push('\n');
        text
    }
}
