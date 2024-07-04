pub mod items;
mod project_diagnostics_settings;
mod toolbar_controls;

#[cfg(test)]
mod diagnostics_tests;

use anyhow::Result;
use collections::{BTreeSet, HashMap, HashSet};
use editor::{
    diagnostic_block_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties, BlockStyle},
    scroll::Autoscroll,
    Bias, Editor, EditorEvent, ExcerptId, MultiBuffer,
};
use futures::{
    channel::mpsc::{self, UnboundedSender},
    StreamExt as _,
};
use gpui::{
    actions, div, AnyElement, AnyView, AppContext, Context, EventEmitter, FocusHandle,
    FocusableView, InteractiveElement, IntoElement, Model, ParentElement, Render, SharedString,
    Styled, Subscription, Task, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use language::{
    Buffer, BufferSnapshot, DiagnosticEntry, DiagnosticSeverity, OffsetRangeExt, ToPoint,
};
use lsp::LanguageServerId;
use multi_buffer::{build_excerpt_ranges, ExpandExcerptDirection};
use project::{DiagnosticSummary, Project, ProjectPath};
use project_diagnostics_settings::ProjectDiagnosticsSettings;
use settings::Settings;
use std::{
    any::{Any, TypeId},
    cmp::Ordering,
    ops::Range,
};
use theme::ActiveTheme;
pub use toolbar_controls::ToolbarControls;
use ui::{h_flex, prelude::*, Icon, IconName, Label};
use util::ResultExt;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle, TabContentParams},
    ItemNavHistory, Pane, ToolbarItemLocation, Workspace,
};

actions!(diagnostics, [Deploy, ToggleWarnings]);

pub fn init(cx: &mut AppContext) {
    ProjectDiagnosticsSettings::register(cx);
    cx.observe_new_views(ProjectDiagnosticsEditor::register)
        .detach();
}

struct ProjectDiagnosticsEditor {
    project: Model<Project>,
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    editor: View<Editor>,
    summary: DiagnosticSummary,
    excerpts: Model<MultiBuffer>,
    path_states: Vec<PathState>,
    paths_to_update: BTreeSet<(ProjectPath, LanguageServerId)>,
    include_warnings: bool,
    context: u32,
    update_paths_tx: UnboundedSender<(ProjectPath, Option<LanguageServerId>)>,
    _update_excerpts_task: Task<Result<()>>,
    _subscription: Subscription,
}

struct PathState {
    path: ProjectPath,
    diagnostics: Vec<(DiagnosticData, BlockId)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagnosticData {
    language_server_id: LanguageServerId,
    is_primary: bool,
    entry: DiagnosticEntry<language::Anchor>,
}

impl EventEmitter<EditorEvent> for ProjectDiagnosticsEditor {}

impl Render for ProjectDiagnosticsEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let child = if self.path_states.is_empty() {
            div()
                .bg(cx.theme().colors().editor_background)
                .flex()
                .items_center()
                .justify_center()
                .size_full()
                .child(Label::new("No problems in workspace"))
        } else {
            div().size_full().child(self.editor.clone())
        };

        div()
            .track_focus(&self.focus_handle)
            .when(self.path_states.is_empty(), |el| {
                el.key_context("EmptyPane")
            })
            .size_full()
            .on_action(cx.listener(Self::toggle_warnings))
            .child(child)
    }
}

impl ProjectDiagnosticsEditor {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(Self::deploy);
    }

    fn new_with_context(
        context: u32,
        project_handle: Model<Project>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let project_event_subscription =
            cx.subscribe(&project_handle, |this, project, event, cx| match event {
                project::Event::DiskBasedDiagnosticsStarted { .. } => {
                    cx.notify();
                }
                project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                    log::debug!("disk based diagnostics finished for server {language_server_id}");
                    this.enqueue_update_stale_excerpts(Some(*language_server_id));
                }
                project::Event::DiagnosticsUpdated {
                    language_server_id,
                    path,
                } => {
                    this.paths_to_update
                        .insert((path.clone(), *language_server_id));
                    this.summary = project.read(cx).diagnostic_summary(false, cx);
                    cx.emit(EditorEvent::TitleChanged);

                    if this.editor.focus_handle(cx).contains_focused(cx) || this.focus_handle.contains_focused(cx) {
                        log::debug!("diagnostics updated for server {language_server_id}, path {path:?}. recording change");
                    } else {
                        log::debug!("diagnostics updated for server {language_server_id}, path {path:?}. updating excerpts");
                        this.enqueue_update_stale_excerpts(Some(*language_server_id));
                    }
                }
                _ => {}
            });

        let focus_handle = cx.focus_handle();
        cx.on_focus_in(&focus_handle, |this, cx| this.focus_in(cx))
            .detach();
        cx.on_focus_out(&focus_handle, |this, _event, cx| this.focus_out(cx))
            .detach();

        let excerpts = cx.new_model(|cx| {
            MultiBuffer::new(
                project_handle.read(cx).replica_id(),
                project_handle.read(cx).capability(),
            )
        });
        let editor = cx.new_view(|cx| {
            let mut editor =
                Editor::for_multibuffer(excerpts.clone(), Some(project_handle.clone()), false, cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor
        });
        cx.subscribe(&editor, |this, _editor, event: &EditorEvent, cx| {
            cx.emit(event.clone());
            match event {
                EditorEvent::Focused => {
                    if this.path_states.is_empty() {
                        cx.focus(&this.focus_handle);
                    }
                }
                EditorEvent::Blurred => this.enqueue_update_stale_excerpts(None),
                _ => {}
            }
        })
        .detach();

        let (update_excerpts_tx, mut update_excerpts_rx) = mpsc::unbounded();

        let project = project_handle.read(cx);
        let mut this = Self {
            project: project_handle.clone(),
            context,
            summary: project.diagnostic_summary(false, cx),
            workspace,
            excerpts,
            focus_handle,
            editor,
            path_states: Vec::new(),
            paths_to_update: BTreeSet::new(),
            include_warnings: ProjectDiagnosticsSettings::get_global(cx).include_warnings,
            update_paths_tx: update_excerpts_tx,
            _update_excerpts_task: cx.spawn(move |this, mut cx| async move {
                while let Some((path, language_server_id)) = update_excerpts_rx.next().await {
                    if let Some(buffer) = project_handle
                        .update(&mut cx, |project, cx| project.open_buffer(path.clone(), cx))?
                        .await
                        .log_err()
                    {
                        this.update(&mut cx, |this, cx| {
                            this.update_excerpts(path, language_server_id, buffer, cx);
                        })?;
                    }
                }
                anyhow::Ok(())
            }),
            _subscription: project_event_subscription,
        };
        this.enqueue_update_all_excerpts(cx);
        this
    }

    fn new(
        project_handle: Model<Project>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new_with_context(
            editor::DEFAULT_MULTIBUFFER_CONTEXT,
            project_handle,
            workspace,
            cx,
        )
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        if let Some(existing) = workspace.item_of_type::<ProjectDiagnosticsEditor>(cx) {
            workspace.activate_item(&existing, cx);
        } else {
            let workspace_handle = cx.view().downgrade();
            let diagnostics = cx.new_view(|cx| {
                ProjectDiagnosticsEditor::new(workspace.project().clone(), workspace_handle, cx)
            });
            workspace.add_item_to_active_pane(Box::new(diagnostics), None, cx);
        }
    }

    fn toggle_warnings(&mut self, _: &ToggleWarnings, cx: &mut ViewContext<Self>) {
        self.include_warnings = !self.include_warnings;
        self.enqueue_update_all_excerpts(cx);
        cx.notify();
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if self.focus_handle.is_focused(cx) && !self.path_states.is_empty() {
            self.editor.focus_handle(cx).focus(cx)
        }
    }

    fn focus_out(&mut self, cx: &mut ViewContext<Self>) {
        if !self.focus_handle.is_focused(cx) && !self.editor.focus_handle(cx).is_focused(cx) {
            self.enqueue_update_stale_excerpts(None);
        }
    }

    /// Enqueue an update of all excerpts. Updates all paths that either
    /// currently have diagnostics or are currently present in this view.
    fn enqueue_update_all_excerpts(&mut self, cx: &mut ViewContext<Self>) {
        self.project.update(cx, |project, cx| {
            let mut paths = project
                .diagnostic_summaries(false, cx)
                .map(|(path, _, _)| path)
                .collect::<BTreeSet<_>>();
            paths.extend(self.path_states.iter().map(|state| state.path.clone()));
            for path in paths {
                self.update_paths_tx.unbounded_send((path, None)).unwrap();
            }
        });
    }

    /// Enqueue an update of the excerpts for any path whose diagnostics are known
    /// to have changed. If a language server id is passed, then only the excerpts for
    /// that language server's diagnostics will be updated. Otherwise, all stale excerpts
    /// will be refreshed.
    fn enqueue_update_stale_excerpts(&mut self, language_server_id: Option<LanguageServerId>) {
        for (path, server_id) in &self.paths_to_update {
            if language_server_id.map_or(true, |id| id == *server_id) {
                self.update_paths_tx
                    .unbounded_send((path.clone(), Some(*server_id)))
                    .unwrap();
            }
        }
    }

    fn update_excerpts(
        &mut self,
        path_to_update: ProjectPath,
        server_to_update: Option<LanguageServerId>,
        buffer: Model<Buffer>,
        cx: &mut ViewContext<Self>,
    ) {
        self.paths_to_update.retain(|(path, server_id)| {
            *path != path_to_update
                || server_to_update.map_or(false, |to_update| *server_id != to_update)
        });

        let was_empty = self.path_states.is_empty();
        let buffer_snapshot = buffer.read(cx).snapshot();
        let path_ix = match self
            .path_states
            .binary_search_by_key(&&path_to_update, |e| &e.path)
        {
            Ok(ix) => ix,
            Err(ix) => {
                self.path_states.insert(
                    ix,
                    PathState {
                        path: path_to_update.clone(),
                        diagnostics: Vec::new(),
                    },
                );
                ix
            }
        };

        let max_severity = if self.include_warnings {
            DiagnosticSeverity::WARNING
        } else {
            DiagnosticSeverity::ERROR
        };

        let path_state = &mut self.path_states[path_ix];
        let mut new_diagnostics = path_state
            .diagnostics
            .iter()
            .filter(|(diagnostic_data, _)| {
                server_to_update.map_or(false, |server_id| {
                    diagnostic_data.language_server_id != server_id
                })
            })
            .filter(|(diagnostic_data, _)| {
                diagnostic_data.entry.diagnostic.severity <= max_severity
            })
            .map(|(diagnostic, block_id)| (diagnostic.clone(), Some(block_id)))
            .collect::<Vec<_>>();
        for (server_id, group) in buffer_snapshot
            .diagnostic_groups(server_to_update)
            .into_iter()
            .filter(|(_, group)| {
                group.entries[group.primary_ix].diagnostic.severity <= max_severity
            })
        {
            for (diagnostic_index, diagnostic) in group.entries.iter().enumerate() {
                let new_data = DiagnosticData {
                    language_server_id: server_id,
                    is_primary: diagnostic_index == group.primary_ix,
                    entry: diagnostic.clone(),
                };
                let (Ok(i) | Err(i)) = new_diagnostics.binary_search_by(|probe| {
                    compare_data_locations(&probe.0, &new_data, &buffer_snapshot)
                });
                new_diagnostics.insert(i, (new_data, None));
            }
        }

        let multi_buffer_snapshot = self.excerpts.read(cx).snapshot(cx);
        let mut current_excerpts = multi_buffer_snapshot.excerpts().fuse().peekable();
        let mut current_diagnostics = path_state.diagnostics.iter().fuse().peekable();
        let mut blocks_to_remove = HashSet::default();
        let mut blocks_to_add = Vec::new();
        let mut unchanged_blocks = HashMap::default();
        let mut excerpts_with_new_diagnostics = HashSet::default();
        let mut excerpts_to_remove = HashSet::default();
        let mut excerpts_to_add = HashMap::<ExcerptId, Vec<Range<language::Anchor>>>::default();
        let mut excerpts_to_expand =
            HashMap::<ExcerptId, HashMap<ExpandExcerptDirection, u32>>::default();
        let mut latest_excerpt_id = ExcerptId::min();
        for (diagnostic_index, (new_diagnostic, _)) in new_diagnostics.iter().enumerate() {
            loop {
                match current_excerpts.peek() {
                    None => {
                        let excerpt_ranges = excerpts_to_add.entry(latest_excerpt_id).or_default();
                        let new_range = new_diagnostic.entry.range.clone();
                        let (Ok(i) | Err(i)) = excerpt_ranges.binary_search_by(|probe| {
                            compare_diagnostic_ranges(probe, &new_range, &buffer_snapshot)
                        });
                        excerpt_ranges.insert(i, new_range);
                        break;
                    }
                    Some((current_excerpt_id, _, current_excerpt_range)) => {
                        match (
                            current_excerpt_range
                                .context
                                .start
                                .cmp(&new_diagnostic.entry.range.start, &buffer_snapshot),
                            current_excerpt_range
                                .context
                                .end
                                .cmp(&new_diagnostic.entry.range.end, &buffer_snapshot),
                        ) {
                            /*
                                  new_s new_e
                            ----[---->><<----]--
                             cur_s         cur_e
                            */
                            (
                                Ordering::Less | Ordering::Equal,
                                Ordering::Greater | Ordering::Equal,
                            ) => {
                                excerpts_with_new_diagnostics.insert(*current_excerpt_id);
                                excerpts_to_remove.remove(current_excerpt_id);
                                break;
                            }
                            /*
                                  cur_s cur_e
                            ---->>>>>[--]<<<<<--
                             new_s         new_e
                            */
                            (
                                Ordering::Greater | Ordering::Equal,
                                Ordering::Less | Ordering::Equal,
                            ) => {
                                let expand_up = current_excerpt_range
                                    .context
                                    .start
                                    .to_point(&buffer_snapshot)
                                    .row
                                    .saturating_sub(
                                        new_diagnostic
                                            .entry
                                            .range
                                            .start
                                            .to_point(&buffer_snapshot)
                                            .row,
                                    );
                                let expand_down = new_diagnostic
                                    .entry
                                    .range
                                    .end
                                    .to_point(&buffer_snapshot)
                                    .row
                                    .saturating_sub(
                                        current_excerpt_range
                                            .context
                                            .end
                                            .to_point(&buffer_snapshot)
                                            .row,
                                    );
                                let expand_value = excerpts_to_expand
                                    .entry(*current_excerpt_id)
                                    .or_default()
                                    .entry(ExpandExcerptDirection::UpAndDown)
                                    .or_default();
                                *expand_value = (*expand_value).max(expand_up).max(expand_down);
                                excerpts_with_new_diagnostics.insert(*current_excerpt_id);
                                excerpts_to_remove.remove(current_excerpt_id);
                                break;
                            }
                            /*
                                    new_s   new_e
                                     >       <
                            ----[---->>>]<<<<<--
                             cur_s    cur_e

                            or
                                      new_s new_e
                                        >    <
                            ----[----]-->>><<<--
                             cur_s cur_e
                            */
                            (Ordering::Less, Ordering::Less) => {
                                if current_excerpt_range
                                    .context
                                    .end
                                    .cmp(&new_diagnostic.entry.range.start, &buffer_snapshot)
                                    .is_ge()
                                {
                                    let expand_down = new_diagnostic
                                        .entry
                                        .range
                                        .end
                                        .to_point(&buffer_snapshot)
                                        .row
                                        .saturating_sub(
                                            current_excerpt_range
                                                .context
                                                .end
                                                .to_point(&buffer_snapshot)
                                                .row,
                                        );
                                    let expand_value = excerpts_to_expand
                                        .entry(*current_excerpt_id)
                                        .or_default()
                                        .entry(ExpandExcerptDirection::Down)
                                        .or_default();
                                    *expand_value = (*expand_value).max(expand_down);
                                    excerpts_with_new_diagnostics.insert(*current_excerpt_id);
                                    excerpts_to_remove.remove(current_excerpt_id);
                                    break;
                                } else if !excerpts_with_new_diagnostics
                                    .contains(current_excerpt_id)
                                {
                                    excerpts_to_remove.insert(*current_excerpt_id);
                                }
                            }
                            /*
                                  cur_s      cur_e
                            ---->>>>>[<<<<----]--
                                >        <
                               new_s    new_e

                            or
                                      cur_s cur_e
                            ---->>><<<--[----]--
                                >    <
                               new_s new_e
                            */
                            (Ordering::Greater, Ordering::Greater) => {
                                if current_excerpt_range
                                    .context
                                    .start
                                    .cmp(&new_diagnostic.entry.range.end, &buffer_snapshot)
                                    .is_le()
                                {
                                    let expand_up = current_excerpt_range
                                        .context
                                        .start
                                        .to_point(&buffer_snapshot)
                                        .row
                                        .saturating_sub(
                                            new_diagnostic
                                                .entry
                                                .range
                                                .start
                                                .to_point(&buffer_snapshot)
                                                .row,
                                        );
                                    let expand_value = excerpts_to_expand
                                        .entry(*current_excerpt_id)
                                        .or_default()
                                        .entry(ExpandExcerptDirection::Up)
                                        .or_default();
                                    *expand_value = (*expand_value).max(expand_up);
                                    excerpts_with_new_diagnostics.insert(*current_excerpt_id);
                                    excerpts_to_remove.remove(current_excerpt_id);
                                    break;
                                } else {
                                    let excerpt_ranges =
                                        excerpts_to_add.entry(latest_excerpt_id).or_default();
                                    let new_range = new_diagnostic.entry.range.clone();
                                    let (Ok(i) | Err(i)) =
                                        excerpt_ranges.binary_search_by(|probe| {
                                            compare_diagnostic_ranges(
                                                probe,
                                                &new_range,
                                                &buffer_snapshot,
                                            )
                                        });
                                    excerpt_ranges.insert(i, new_range);
                                    break;
                                }
                            }
                        }
                        if let Some((next_id, ..)) = current_excerpts.next() {
                            latest_excerpt_id = next_id;
                        }
                    }
                }
            }

            // TODO kb diagnostics rendered have X button that does not work (need to hide it)
            // TODO kb deduplicate identical diagnostics text for the same line
            loop {
                match current_diagnostics.peek() {
                    None => {
                        blocks_to_add.push(diagnostic_index);
                        break;
                    }
                    Some((current_diagnostic, current_block)) => {
                        match compare_data_locations(
                            current_diagnostic,
                            new_diagnostic,
                            &buffer_snapshot,
                        ) {
                            Ordering::Less => {
                                blocks_to_remove.insert(*current_block);
                            }
                            Ordering::Equal => {
                                if current_diagnostic == new_diagnostic {
                                    unchanged_blocks.insert(diagnostic_index, *current_block);
                                } else {
                                    blocks_to_remove.insert(*current_block);
                                    blocks_to_add.push(diagnostic_index);
                                }
                                let _ = current_diagnostics.next();
                                break;
                            }
                            Ordering::Greater => break,
                        }
                        let _ = current_diagnostics.next();
                    }
                }
            }
        }
        excerpts_to_remove.extend(
            current_excerpts
                .filter(|(excerpt_id, ..)| !excerpts_with_new_diagnostics.contains(excerpt_id))
                .map(|(excerpt_id, ..)| excerpt_id),
        );
        blocks_to_remove.extend(current_diagnostics.map(|&(_, block_id)| block_id));

        let mut excerpt_expands = HashMap::default();
        for (excerpt_id, directions) in excerpts_to_expand {
            let excerpt_expand = if directions.len() > 1 {
                Some((
                    ExpandExcerptDirection::UpAndDown,
                    directions
                        .values()
                        .max()
                        .copied()
                        .unwrap_or_default()
                        .max(self.context),
                ))
            } else {
                directions
                    .into_iter()
                    .next()
                    .map(|(direction, expand)| (direction, expand.max(self.context)))
            };
            if let Some(expand) = excerpt_expand {
                excerpt_expands
                    .entry(expand)
                    .or_insert_with(|| Vec::new())
                    .push(excerpt_id);
            }
        }

        drop(multi_buffer_snapshot);
        self.excerpts.update(cx, |multi_buffer, cx| {
            let max_point = buffer_snapshot.max_point();
            for (after_excerpt_id, ranges) in excerpts_to_add {
                let ranges = ranges
                    .into_iter()
                    .map(|range| {
                        let mut extended_point_range = range.to_point(&buffer_snapshot);
                        extended_point_range.start.row =
                            extended_point_range.start.row.saturating_sub(self.context);
                        extended_point_range.start.column = 0;
                        extended_point_range.end.row =
                            (extended_point_range.end.row + self.context).min(max_point.row);
                        extended_point_range.end.column = u32::MAX;
                        let extended_start =
                            buffer_snapshot.clip_point(extended_point_range.start, Bias::Left);
                        let extended_end =
                            buffer_snapshot.clip_point(extended_point_range.end, Bias::Right);
                        extended_start..extended_end
                    })
                    .collect::<Vec<_>>();
                let (joined_ranges, _) =
                    build_excerpt_ranges(&buffer_snapshot, &ranges, self.context);
                multi_buffer.insert_excerpts_after(
                    after_excerpt_id,
                    buffer.clone(),
                    joined_ranges,
                    cx,
                );
            }
            for ((direction, line_count), excerpts) in excerpt_expands {
                multi_buffer.expand_excerpts(excerpts, line_count, direction, cx);
            }
            multi_buffer.remove_excerpts(excerpts_to_remove, cx);
        });

        let editor_snapshot = self.editor.update(cx, |editor, cx| editor.snapshot(cx));
        let mut updated_excerpts = editor_snapshot.buffer_snapshot.excerpts().fuse().peekable();
        let mut diagnostics_with_blocks = HashMap::default();
        let new_blocks =
            blocks_to_add
                .into_iter()
                .enumerate()
                .flat_map(|(block_index, diagnostic_index)| {
                    diagnostics_with_blocks.insert(diagnostic_index, block_index);
                    let new_diagnostic = &new_diagnostics[diagnostic_index].0.entry;
                    let block_position = new_diagnostic.range.start;
                    let excerpt_id = loop {
                        match updated_excerpts.peek() {
                            None => break None,
                            Some((excerpt_id, excerpt_buffer_snapshot, excerpt_range)) => {
                                let excerpt_range = &excerpt_range.context;

                                match block_position
                                    .cmp(&excerpt_range.start, excerpt_buffer_snapshot)
                                {
                                    Ordering::Less => break None,
                                    Ordering::Equal | Ordering::Greater => match block_position
                                        .cmp(&excerpt_range.end, excerpt_buffer_snapshot)
                                    {
                                        Ordering::Equal | Ordering::Less => {
                                            break Some(*excerpt_id)
                                        }
                                        Ordering::Greater => {}
                                    },
                                }
                            }
                        }
                        let _ = updated_excerpts.next();
                    }?;

                    Some(BlockProperties {
                        position: editor_snapshot
                            .buffer_snapshot
                            .anchor_in_excerpt(excerpt_id, block_position)?,
                        height: new_diagnostic.diagnostic.message.matches('\n').count() as u8 + 1,
                        style: BlockStyle::Sticky,
                        render: diagnostic_block_renderer(new_diagnostic.diagnostic.clone(), true),
                        disposition: BlockDisposition::Above,
                    })
                });
        // TODO kb rework block approach: need to unite them if they belong to the same display_row
        let new_block_ids = self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, None, cx);
            editor.insert_blocks(new_blocks, Some(Autoscroll::fit()), cx)
        });
        if new_diagnostics.is_empty() {
            self.path_states.remove(path_ix);
        } else {
            path_state.diagnostics = new_diagnostics
                .into_iter()
                .enumerate()
                .filter_map(|(diagnostic_index, (diagnostic, block_id))| {
                    let &block_id = block_id
                        .or_else(|| unchanged_blocks.get(&diagnostic_index))
                        .or_else(|| {
                            let &block_index = diagnostics_with_blocks.get(&diagnostic_index)?;
                            new_block_ids.get(block_index)
                        })?;
                    Some((diagnostic, block_id))
                })
                .collect();
        }

        // TODO kb
        // self.editor.update(cx, |editor, cx| {
        //     let groups;
        //     let mut selections;
        //     let new_excerpt_ids_by_selection_id;
        //     if was_empty {
        //         groups = self.path_states.first()?.diagnostic_groups.as_slice();
        //         new_excerpt_ids_by_selection_id = [(0, ExcerptId::min())].into_iter().collect();
        //         selections = vec![Selection {
        //             id: 0,
        //             start: 0,
        //             end: 0,
        //             reversed: false,
        //             goal: SelectionGoal::None,
        //         }];
        //     } else {
        //         groups = self.path_states.get(path_ix)?.diagnostic_groups.as_slice();
        //         new_excerpt_ids_by_selection_id =
        //             editor.change_selections(Some(Autoscroll::fit()), cx, |s| s.refresh());
        //         selections = editor.selections.all::<usize>(cx);
        //     }

        //     // If any selection has lost its position, move it to start of the next primary diagnostic.
        //     let snapshot = editor.snapshot(cx);
        //     for selection in &mut selections {
        //         if let Some(new_excerpt_id) = new_excerpt_ids_by_selection_id.get(&selection.id) {
        //             let group_ix = match groups.binary_search_by(|probe| {
        //                 probe
        //                     .excerpts
        //                     .last()
        //                     .unwrap()
        //                     .cmp(new_excerpt_id, &snapshot.buffer_snapshot)
        //             }) {
        //                 Ok(ix) | Err(ix) => ix,
        //             };
        //             if let Some(group) = groups.get(group_ix) {
        //                 if let Some(offset) = editor_snapshot.buffer_snapshot
        //                     .anchor_in_excerpt(
        //                         group.excerpts[group.primary_excerpt_ix],
        //                         group.primary_diagnostic.range.start,
        //                     )
        //                     .map(|anchor| anchor.to_offset(&editor_snapshot.buffer_snapshot))
        //                 {
        //                     selection.start = offset;
        //                     selection.end = offset;
        //                 }
        //             }
        //         }
        //     }
        //     editor.change_selections(None, cx, |s| {
        //         s.select(selections);
        //     });
        //     Some(())
        // });

        if self.path_states.is_empty() {
            if self.editor.focus_handle(cx).is_focused(cx) {
                cx.focus(&self.focus_handle);
            }
        } else if self.focus_handle.is_focused(cx) {
            let focus_handle = self.editor.focus_handle(cx);
            cx.focus(&focus_handle);
        }

        #[cfg(test)]
        self.check_invariants(cx);

        cx.notify();
    }

    #[cfg(test)]
    fn check_invariants(&self, cx: &mut ViewContext<Self>) {
        let mut excerpts = Vec::new();
        for (id, buffer, _) in self.excerpts.read(cx).snapshot(cx).excerpts() {
            if let Some(file) = buffer.file() {
                excerpts.push((id, file.path().clone()));
            }
        }

        let mut prev_path = None;
        for (_, path) in &excerpts {
            if let Some(prev_path) = prev_path {
                if path < prev_path {
                    panic!("excerpts are not sorted by path {:?}", excerpts);
                }
            }
            prev_path = Some(path);
        }
    }
}

impl FocusableView for ProjectDiagnosticsEditor {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ProjectDiagnosticsEditor {
    type Event = EditorEvent;

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| editor.deactivated(cx));
    }

    fn navigate(&mut self, data: Box<dyn Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, cx))
    }

    fn tab_tooltip_text(&self, _: &AppContext) -> Option<SharedString> {
        Some("Project Diagnostics".into())
    }

    fn tab_content(&self, params: TabContentParams, _: &WindowContext) -> AnyElement {
        if self.summary.error_count == 0 && self.summary.warning_count == 0 {
            Label::new("No problems")
                .color(if params.selected {
                    Color::Default
                } else {
                    Color::Muted
                })
                .into_any_element()
        } else {
            h_flex()
                .gap_1()
                .when(self.summary.error_count > 0, |then| {
                    then.child(
                        h_flex()
                            .gap_1()
                            .child(Icon::new(IconName::XCircle).color(Color::Error))
                            .child(Label::new(self.summary.error_count.to_string()).color(
                                if params.selected {
                                    Color::Default
                                } else {
                                    Color::Muted
                                },
                            )),
                    )
                })
                .when(self.summary.warning_count > 0, |then| {
                    then.child(
                        h_flex()
                            .gap_1()
                            .child(Icon::new(IconName::ExclamationTriangle).color(Color::Warning))
                            .child(Label::new(self.summary.warning_count.to_string()).color(
                                if params.selected {
                                    Color::Default
                                } else {
                                    Color::Muted
                                },
                            )),
                    )
                })
                .into_any_element()
        }
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("project diagnostics")
    }

    fn for_each_project_item(
        &self,
        cx: &AppContext,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::Item),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &AppContext) -> bool {
        false
    }

    fn set_nav_history(&mut self, nav_history: ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(|cx| {
            ProjectDiagnosticsEditor::new(self.project.clone(), self.workspace.clone(), cx)
        }))
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.excerpts.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.excerpts.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _: &AppContext) -> bool {
        true
    }

    fn save(
        &mut self,
        format: bool,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(format, project, cx)
    }

    fn save_as(
        &mut self,
        _: Model<Project>,
        _: ProjectPath,
        _: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn reload(&mut self, project: Model<Project>, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        self.editor.reload(project, cx)
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a View<Self>,
        _: &'a AppContext,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.added_to_workspace(workspace, cx));
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("diagnostics")
    }

    fn deserialize(
        project: Model<Project>,
        workspace: WeakView<Workspace>,
        _workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<Result<View<Self>>> {
        Task::ready(Ok(cx.new_view(|cx| Self::new(project, workspace, cx))))
    }
}

fn compare_data_locations(
    old: &DiagnosticData,
    new: &DiagnosticData,
    snapshot: &BufferSnapshot,
) -> Ordering {
    compare_diagnostics(&old.entry, &new.entry, snapshot)
        .then_with(|| old.language_server_id.cmp(&new.language_server_id))
}

fn compare_diagnostics(
    old: &DiagnosticEntry<language::Anchor>,
    new: &DiagnosticEntry<language::Anchor>,
    snapshot: &BufferSnapshot,
) -> Ordering {
    compare_diagnostic_ranges(&old.range, &new.range, snapshot)
        .then_with(|| old.diagnostic.message.cmp(&new.diagnostic.message))
}

fn compare_diagnostic_ranges(
    old: &Range<language::Anchor>,
    new: &Range<language::Anchor>,
    snapshot: &BufferSnapshot,
) -> Ordering {
    use language::ToOffset;

    // The diagnostics may point to a previously open Buffer for this file.
    if !old.start.is_valid(snapshot) || !new.start.is_valid(snapshot) {
        return Ordering::Greater;
    }

    old.start
        .to_offset(snapshot)
        .cmp(&new.start.to_offset(snapshot))
        .then_with(|| {
            old.end
                .to_offset(snapshot)
                .cmp(&new.end.to_offset(snapshot))
        })
}
