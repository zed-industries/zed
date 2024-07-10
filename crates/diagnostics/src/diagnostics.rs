pub mod items;
mod project_diagnostics_settings;
mod toolbar_controls;

#[cfg(test)]
mod diagnostics_tests;

use anyhow::Result;
use collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use editor::{
    diagnostic_block_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties, BlockStyle},
    scroll::Autoscroll,
    Bias, Editor, EditorEvent, ExcerptId, ExcerptRange, MultiBuffer, MultiBufferSnapshot, ToPoint,
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
    Buffer, BufferSnapshot, DiagnosticEntry, DiagnosticSeverity, OffsetRangeExt, ToOffset,
    ToPoint as _,
};
use lsp::LanguageServerId;
use multi_buffer::{build_excerpt_ranges, ExpandExcerptDirection, MultiBufferRow};
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
use util::{debug_panic, ResultExt};
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
    first_excerpt_id: Option<ExcerptId>,
    last_excerpt_id: Option<ExcerptId>,
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

        // TODO kb change selections as in the old panel, to the next primary diagnostics
        let was_empty = self.path_states.is_empty();
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
                        last_excerpt_id: None,
                        first_excerpt_id: None,
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

        let excerpt_borders = self.excerpt_borders_for_path(path_ix);
        let path_state = &mut self.path_states[path_ix];
        let buffer_snapshot = buffer.read(cx).snapshot();
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
            .map(|(diagnostic, block_id)| (diagnostic.clone(), Some(*block_id)))
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

        let mut path_update = PathUpdate::new(excerpt_borders, new_diagnostics);
        path_update.prepare_excerpt_data(
            self.context,
            self.excerpts.read(cx).snapshot(cx),
            buffer.read(cx).snapshot(),
            path_state.diagnostics.iter(),
        );
        // TODO kb panics on wrong exceprt insert order (wrong excerpt selected), on invalid excerpt range edges
        self.excerpts.update(cx, |multi_buffer, cx| {
            path_update.apply_excerpt_changes(
                path_state,
                self.context,
                buffer_snapshot,
                multi_buffer,
                buffer,
                cx,
            );
        });

        let new_multi_buffer_snapshot = self.excerpts.read(cx).snapshot(cx);
        let blocks_to_insert = path_update.prepare_blocks_to_insert(new_multi_buffer_snapshot);

        let new_block_ids = self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(std::mem::take(&mut path_update.blocks_to_remove), None, cx);
            editor.insert_blocks(blocks_to_insert, Some(Autoscroll::fit()), cx)
        });
        path_state.diagnostics = path_update.new_blocks(new_block_ids);

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

    fn excerpt_borders_for_path(&self, path_ix: usize) -> (Option<ExcerptId>, Option<ExcerptId>) {
        let previous_path_state_ix =
            Some(path_ix.saturating_sub(1)).filter(|&previous_path_ix| previous_path_ix != path_ix);
        let next_path_state_ix = path_ix + 1;
        let start = previous_path_state_ix.and_then(|i| {
            self.path_states[..=i]
                .iter()
                .rev()
                .find_map(|state| state.last_excerpt_id)
        });
        let end = self.path_states[next_path_state_ix..]
            .iter()
            .find_map(|state| state.first_excerpt_id);
        (start, end)
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

// TODO kb wrong? What to do here instead?
fn compare_diagnostic_range_edges(
    old: &Range<language::Anchor>,
    new: &Range<language::Anchor>,
    snapshot: &BufferSnapshot,
) -> (Ordering, Ordering) {
    // The diagnostics may point to a previously open Buffer for this file.
    let start_cmp = match (old.start.is_valid(snapshot), new.start.is_valid(snapshot)) {
        (false, false) => old.start.offset.cmp(&new.start.offset),
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (true, true) => old.start.cmp(&new.start, snapshot),
    };

    let end_cmp = old
        .end
        .to_offset(snapshot)
        .cmp(&new.end.to_offset(snapshot));
    (start_cmp, end_cmp)
}

#[derive(Debug)]
struct PathUpdate {
    path_excerpts_borders: (Option<ExcerptId>, Option<ExcerptId>),
    latest_excerpt_id: ExcerptId,
    new_diagnostics: Vec<(DiagnosticData, Option<BlockId>)>,
    diagnostics_by_row_label: BTreeMap<
        MultiBufferRow,
        (
            editor::Anchor,
            BTreeMap<(Option<String>, String), Vec<(usize, Option<BlockId>)>>,
        ),
    >,
    blocks_to_remove: HashSet<BlockId>,
    unchanged_blocks: HashMap<usize, BlockId>,
    excerpts_with_new_diagnostics: HashSet<ExcerptId>,
    excerpts_to_remove: Vec<ExcerptId>,
    excerpt_expands: HashMap<(ExpandExcerptDirection, u32), Vec<ExcerptId>>,
    excerpts_to_add: HashMap<ExcerptId, Vec<Range<language::Anchor>>>,
    first_excerpt_id: Option<ExcerptId>,
    last_excerpt_id: Option<ExcerptId>,
}

impl PathUpdate {
    fn new(
        path_excerpts_borders: (Option<ExcerptId>, Option<ExcerptId>),
        new_diagnostics: Vec<(DiagnosticData, Option<BlockId>)>,
    ) -> Self {
        let latest_excerpt_id = path_excerpts_borders.0.unwrap_or_else(|| ExcerptId::min());
        Self {
            new_diagnostics,
            latest_excerpt_id,
            path_excerpts_borders,
            diagnostics_by_row_label: BTreeMap::new(),
            blocks_to_remove: HashSet::default(),
            excerpts_with_new_diagnostics: HashSet::default(),
            unchanged_blocks: HashMap::default(),
            excerpts_to_add: HashMap::default(),
            excerpt_expands: HashMap::default(),
            excerpts_to_remove: Vec::new(),
            first_excerpt_id: None,
            last_excerpt_id: None,
        }
    }

    fn prepare_excerpt_data<'a>(
        &'a mut self,
        context: u32,
        multi_buffer_snapshot: MultiBufferSnapshot,
        buffer_snapshot: BufferSnapshot,
        current_diagnostics: impl Iterator<Item = &'a (DiagnosticData, BlockId)> + 'a,
    ) {
        let mut current_diagnostics = current_diagnostics.fuse().peekable();
        let mut excerpts_to_expand =
            HashMap::<ExcerptId, HashMap<ExpandExcerptDirection, u32>>::default();
        let mut current_excerpts = path_state_excerpts(
            self.path_excerpts_borders.0,
            self.path_excerpts_borders.1,
            &multi_buffer_snapshot,
        )
        .fuse()
        .peekable();

        for (diagnostic_index, (new_diagnostic, existing_block)) in
            self.new_diagnostics.iter().enumerate()
        {
            if let Some(existing_block) = existing_block {
                self.unchanged_blocks
                    .insert(diagnostic_index, *existing_block);
            }

            loop {
                match current_excerpts.peek() {
                    None => {
                        let excerpt_ranges = self
                            .excerpts_to_add
                            .entry(self.latest_excerpt_id)
                            .or_default();
                        let new_range = new_diagnostic.entry.range.clone();
                        let (Ok(i) | Err(i)) = excerpt_ranges.binary_search_by(|probe| {
                            compare_diagnostic_ranges(probe, &new_range, &buffer_snapshot)
                        });
                        excerpt_ranges.insert(i, new_range);
                        break;
                    }
                    Some((current_excerpt_id, _, current_excerpt_range)) => {
                        match compare_diagnostic_range_edges(
                            &current_excerpt_range.context,
                            &new_diagnostic.entry.range,
                            &buffer_snapshot,
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
                                self.excerpts_with_new_diagnostics
                                    .insert(*current_excerpt_id);
                                if self.first_excerpt_id.is_none() {
                                    self.first_excerpt_id = Some(*current_excerpt_id);
                                }
                                self.last_excerpt_id = Some(*current_excerpt_id);
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
                                self.excerpts_with_new_diagnostics
                                    .insert(*current_excerpt_id);
                                if self.first_excerpt_id.is_none() {
                                    self.first_excerpt_id = Some(*current_excerpt_id);
                                }
                                self.last_excerpt_id = Some(*current_excerpt_id);
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
                                    self.excerpts_with_new_diagnostics
                                        .insert(*current_excerpt_id);
                                    if self.first_excerpt_id.is_none() {
                                        self.first_excerpt_id = Some(*current_excerpt_id);
                                    }
                                    self.last_excerpt_id = Some(*current_excerpt_id);
                                    break;
                                } else if !self
                                    .excerpts_with_new_diagnostics
                                    .contains(current_excerpt_id)
                                {
                                    self.excerpts_to_remove.push(*current_excerpt_id);
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
                                    self.excerpts_with_new_diagnostics
                                        .insert(*current_excerpt_id);
                                    if self.first_excerpt_id.is_none() {
                                        self.first_excerpt_id = Some(*current_excerpt_id);
                                    }
                                    self.last_excerpt_id = Some(*current_excerpt_id);
                                    break;
                                } else {
                                    let excerpt_ranges = self
                                        .excerpts_to_add
                                        .entry(self.latest_excerpt_id)
                                        .or_default();
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
                            self.latest_excerpt_id = next_id;
                        }
                    }
                }
            }

            loop {
                match current_diagnostics.peek() {
                    None => break,
                    Some((current_diagnostic, current_block)) => {
                        match compare_data_locations(
                            current_diagnostic,
                            new_diagnostic,
                            &buffer_snapshot,
                        ) {
                            Ordering::Less => {
                                self.blocks_to_remove.insert(*current_block);
                            }
                            Ordering::Equal => {
                                if current_diagnostic == new_diagnostic {
                                    self.unchanged_blocks
                                        .insert(diagnostic_index, *current_block);
                                } else {
                                    self.blocks_to_remove.insert(*current_block);
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

        self.excerpts_to_remove.retain(|excerpt_id| {
            !self.excerpts_with_new_diagnostics.contains(excerpt_id)
                && !excerpts_to_expand.contains_key(excerpt_id)
        });
        self.excerpts_to_remove.extend(
            current_excerpts
                .filter(|(excerpt_id, ..)| {
                    !self.excerpts_with_new_diagnostics.contains(excerpt_id)
                        && !excerpts_to_expand.contains_key(excerpt_id)
                })
                .map(|(excerpt_id, ..)| excerpt_id),
        );
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
                        .max(context),
                ))
            } else {
                directions
                    .into_iter()
                    .next()
                    .map(|(direction, expand)| (direction, expand.max(context)))
            };
            if let Some(expand) = excerpt_expand {
                excerpt_expands
                    .entry(expand)
                    .or_insert_with(|| Vec::new())
                    .push(excerpt_id);
            }
        }
        self.blocks_to_remove
            .extend(current_diagnostics.map(|(_, block_id)| block_id));
    }

    fn apply_excerpt_changes(
        &mut self,
        path_state: &mut PathState,
        context: u32,
        buffer_snapshot: BufferSnapshot,
        multi_buffer: &mut MultiBuffer,
        buffer: Model<Buffer>,
        cx: &mut gpui::ModelContext<MultiBuffer>,
    ) {
        let max_point = buffer_snapshot.max_point();
        for (after_excerpt_id, ranges) in std::mem::take(&mut self.excerpts_to_add) {
            let ranges = ranges
                .into_iter()
                .map(|range| {
                    let mut extended_point_range = range.to_point(&buffer_snapshot);
                    extended_point_range.start.row =
                        extended_point_range.start.row.saturating_sub(context);
                    extended_point_range.start.column = 0;
                    extended_point_range.end.row =
                        (extended_point_range.end.row + context).min(max_point.row);
                    extended_point_range.end.column = u32::MAX;
                    let extended_start =
                        buffer_snapshot.clip_point(extended_point_range.start, Bias::Left);
                    let extended_end =
                        buffer_snapshot.clip_point(extended_point_range.end, Bias::Right);
                    extended_start..extended_end
                })
                .collect::<Vec<_>>();
            let (joined_ranges, _) = build_excerpt_ranges(&buffer_snapshot, &ranges, context);
            let excerpts = multi_buffer.insert_excerpts_after(
                after_excerpt_id,
                buffer.clone(),
                joined_ranges,
                cx,
            );
            if self.first_excerpt_id.is_none() {
                self.first_excerpt_id = excerpts.last().copied();
            }
            self.last_excerpt_id = excerpts.last().copied();
        }
        for ((direction, line_count), excerpts) in std::mem::take(&mut self.excerpt_expands) {
            multi_buffer.expand_excerpts(excerpts, line_count, direction, cx);
        }
        multi_buffer.remove_excerpts(std::mem::take(&mut self.excerpts_to_remove), cx);
        path_state.first_excerpt_id = self.first_excerpt_id;
        path_state.last_excerpt_id = self.last_excerpt_id;
    }

    fn prepare_blocks_to_insert(
        &mut self,
        multi_buffer_snapshot: MultiBufferSnapshot,
    ) -> Vec<BlockProperties<editor::Anchor>> {
        let mut updated_excerpts = path_state_excerpts(
            self.path_excerpts_borders.0,
            self.path_excerpts_borders.1,
            &multi_buffer_snapshot,
        )
        .fuse()
        .peekable();
        self.diagnostics_by_row_label = self.new_diagnostics.iter().enumerate().fold(
            BTreeMap::new(),
            |mut diagnostics_by_row_label, (diagnostic_index, (diagnostic, existing_block))| {
                let new_diagnostic = &diagnostic.entry;
                let block_position = new_diagnostic.range.start;
                let excerpt_id = loop {
                    match updated_excerpts.peek() {
                        None => break None,
                        Some((excerpt_id, excerpt_buffer_snapshot, excerpt_range)) => {
                            let excerpt_range = &excerpt_range.context;
                            match block_position.cmp(&excerpt_range.start, excerpt_buffer_snapshot)
                            {
                                Ordering::Less => break None,
                                Ordering::Equal | Ordering::Greater => match block_position
                                    .cmp(&excerpt_range.end, excerpt_buffer_snapshot)
                                {
                                    Ordering::Equal | Ordering::Less => break Some(*excerpt_id),
                                    Ordering::Greater => {
                                        let _ = updated_excerpts.next();
                                    }
                                },
                            }
                        }
                    }
                };

                let Some(position_in_multi_buffer) = excerpt_id.and_then(|excerpt_id| {
                    multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, block_position)
                }) else {
                    return diagnostics_by_row_label;
                };

                let multi_buffer_row = MultiBufferRow(
                    position_in_multi_buffer
                        .to_point(&multi_buffer_snapshot)
                        .row,
                );

                let diagnostics_by_labels = &mut diagnostics_by_row_label
                    .entry(multi_buffer_row)
                    .or_insert_with(|| (position_in_multi_buffer, BTreeMap::new()))
                    .1;
                let grouped_diagnostics = diagnostics_by_labels
                    .entry((
                        new_diagnostic.diagnostic.source.clone(),
                        new_diagnostic.diagnostic.message.clone(),
                    ))
                    .or_default();
                if grouped_diagnostics.len() > 0 {
                    if grouped_diagnostics.len() == 1 {
                        if let Some((i, block_id)) = grouped_diagnostics.first() {
                            if let Some(block_id) = block_id {
                                self.blocks_to_remove.insert(*block_id);
                            }
                            if let Some(block_id) = self.unchanged_blocks.remove(i) {
                                self.blocks_to_remove.insert(block_id);
                            }
                        }
                    }
                    if let Some(existing_block) = existing_block {
                        self.blocks_to_remove.insert(*existing_block);
                    }
                    if let Some(block_id) = self.unchanged_blocks.remove(&diagnostic_index) {
                        self.blocks_to_remove.insert(block_id);
                    }
                }
                grouped_diagnostics.push((diagnostic_index, *existing_block));

                diagnostics_by_row_label
            },
        );

        self.diagnostics_by_row_label
            .values_mut()
            .flat_map(|(earliest_in_row_position, diagnostics_by_label)| {
                diagnostics_by_label
                    .values_mut()
                    .filter_map(|diagnostics_with_same_label| {
                        diagnostics_with_same_label.sort_by(|&(index_a, _), &(index_b, _)| {
                            let a = &self.new_diagnostics[index_a].0.entry.diagnostic;
                            let b = &self.new_diagnostics[index_b].0.entry.diagnostic;
                            a.is_primary
                                .cmp(&b.is_primary)
                                .then_with(|| a.group_id.cmp(&b.group_id))
                                .then_with(|| a.severity.cmp(&b.severity))
                        });
                        let (index, _) = diagnostics_with_same_label.first()?;
                        if self.unchanged_blocks.contains_key(index) {
                            None
                        } else {
                            let new_diagnostic =
                                self.new_diagnostics[*index].0.entry.diagnostic.clone();
                            Some(BlockProperties {
                                position: *earliest_in_row_position,
                                height: new_diagnostic.message.matches('\n').count() as u8 + 1,
                                style: BlockStyle::Sticky,
                                // TODO kb need to render something that will toggle diagnostics for the same line
                                render: diagnostic_block_renderer(new_diagnostic, false, true),
                                disposition: BlockDisposition::Above,
                            })
                        }
                    })
            })
            .collect()
    }

    fn new_blocks(mut self, new_block_ids: Vec<BlockId>) -> Vec<(DiagnosticData, BlockId)> {
        let mut new_block_ids = new_block_ids.into_iter().fuse();
        for (_, diagnostics_by_label) in self.diagnostics_by_row_label {
            for grouped_diagnostics in diagnostics_by_label
                .1
                .into_values()
                .map(|grouped_diagnostics| grouped_diagnostics)
            {
                let mut created_block_id = None;
                let grouped_diagnostics_len = grouped_diagnostics.len();
                for (index, block_id) in grouped_diagnostics {
                    match block_id {
                        Some(block_id) => self.new_diagnostics[index].1 = Some(block_id),
                        None => match self.unchanged_blocks.get(&index) {
                            Some(&block_id) => {
                                debug_assert!(
                                    grouped_diagnostics_len == 1,
                                    "Should only allow unchanged blocks for labels without duplicates"
                                );
                                self.new_diagnostics[index].1 = Some(block_id);
                            }
                            None => {
                                let Some(block_id) =
                                    created_block_id.get_or_insert_with(|| new_block_ids.next())
                                else {
                                    debug_panic!("Expected a new block for each new diagnostic");
                                    continue;
                                };
                                self.new_diagnostics[index].1 = Some(*block_id);
                            }
                        },
                    }
                }
            }
        }

        self.new_diagnostics
            .into_iter()
            .filter_map(|(diagnostic, block_id)| Some((diagnostic, block_id?)))
            .collect()
    }
}

fn path_state_excerpts<'a>(
    after_excerpt_id: Option<ExcerptId>,
    before_excerpt_id: Option<ExcerptId>,
    multi_buffer_snapshot: &'a editor::MultiBufferSnapshot,
) -> impl Iterator<
    Item = (
        ExcerptId,
        &'a BufferSnapshot,
        ExcerptRange<language::Anchor>,
    ),
> {
    multi_buffer_snapshot
        .excerpts()
        .skip_while(move |&(excerpt_id, ..)| match after_excerpt_id {
            Some(after_excerpt_id) => after_excerpt_id != excerpt_id,
            None => false,
        })
        .filter(move |&(excerpt_id, ..)| after_excerpt_id != Some(excerpt_id))
        .take_while(move |&(excerpt_id, ..)| match before_excerpt_id {
            Some(before_excerpt_id) => before_excerpt_id != excerpt_id,
            None => true,
        })
}
