pub mod items;
mod project_diagnostics_settings;
mod toolbar_controls;

#[cfg(test)]
mod diagnostics_tests;

use anyhow::Result;
use collections::{BTreeSet, HashSet};
use editor::{
    diagnostic_block_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock},
    highlight_diagnostic_message,
    scroll::Autoscroll,
    Editor, EditorEvent, ExcerptId, ExcerptRange, MultiBuffer, ToOffset,
};
use futures::{
    channel::mpsc::{self, UnboundedSender},
    StreamExt as _,
};
use gpui::{
    actions, div, svg, AnyElement, AnyView, AppContext, Context, EventEmitter, FocusHandle,
    FocusableView, HighlightStyle, InteractiveElement, IntoElement, Model, ParentElement, Render,
    SharedString, Styled, StyledText, Subscription, Task, View, ViewContext, VisualContext,
    WeakView, WindowContext,
};
use language::{
    Bias, Buffer, Diagnostic, DiagnosticEntry, DiagnosticSeverity, Point, Selection, SelectionGoal,
};
use lsp::LanguageServerId;
use project::{DiagnosticSummary, Project, ProjectPath};
use project_diagnostics_settings::ProjectDiagnosticsSettings;
use settings::Settings;
use std::{
    any::{Any, TypeId},
    cmp::Ordering,
    mem,
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
    diagnostic_groups: Vec<DiagnosticGroupState>,
}

struct DiagnosticGroupState {
    language_server_id: LanguageServerId,
    primary_diagnostic: DiagnosticEntry<language::Anchor>,
    primary_excerpt_ix: usize,
    excerpts: Vec<ExcerptId>,
    blocks: HashSet<BlockId>,
    block_count: usize,
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

                    if this.editor.read(cx).is_focused(cx) || this.focus_handle.is_focused(cx) {
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
        cx.on_focus_out(&focus_handle, |this, cx| this.focus_out(cx))
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
            path_states: Default::default(),
            paths_to_update: Default::default(),
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
        let snapshot = buffer.read(cx).snapshot();
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
                        diagnostic_groups: Default::default(),
                    },
                );
                ix
            }
        };

        let mut prev_excerpt_id = if path_ix > 0 {
            let prev_path_last_group = &self.path_states[path_ix - 1]
                .diagnostic_groups
                .last()
                .unwrap();
            *prev_path_last_group.excerpts.last().unwrap()
        } else {
            ExcerptId::min()
        };

        let path_state = &mut self.path_states[path_ix];
        let mut new_group_ixs = Vec::new();
        let mut blocks_to_add = Vec::new();
        let mut blocks_to_remove = HashSet::default();
        let mut first_excerpt_id = None;
        let max_severity = if self.include_warnings {
            DiagnosticSeverity::WARNING
        } else {
            DiagnosticSeverity::ERROR
        };
        let excerpts_snapshot = self.excerpts.update(cx, |excerpts, cx| {
            let mut old_groups = mem::take(&mut path_state.diagnostic_groups)
                .into_iter()
                .enumerate()
                .peekable();
            let mut new_groups = snapshot
                .diagnostic_groups(server_to_update)
                .into_iter()
                .filter(|(_, group)| {
                    group.entries[group.primary_ix].diagnostic.severity <= max_severity
                })
                .peekable();
            loop {
                let mut to_insert = None;
                let mut to_remove = None;
                let mut to_keep = None;
                match (old_groups.peek(), new_groups.peek()) {
                    (None, None) => break,
                    (None, Some(_)) => to_insert = new_groups.next(),
                    (Some((_, old_group)), None) => {
                        if server_to_update.map_or(true, |id| id == old_group.language_server_id) {
                            to_remove = old_groups.next();
                        } else {
                            to_keep = old_groups.next();
                        }
                    }
                    (Some((_, old_group)), Some((new_language_server_id, new_group))) => {
                        let old_primary = &old_group.primary_diagnostic;
                        let new_primary = &new_group.entries[new_group.primary_ix];
                        match compare_diagnostics(old_primary, new_primary, &snapshot)
                            .then_with(|| old_group.language_server_id.cmp(new_language_server_id))
                        {
                            Ordering::Less => {
                                if server_to_update
                                    .map_or(true, |id| id == old_group.language_server_id)
                                {
                                    to_remove = old_groups.next();
                                } else {
                                    to_keep = old_groups.next();
                                }
                            }
                            Ordering::Equal => {
                                to_keep = old_groups.next();
                                new_groups.next();
                            }
                            Ordering::Greater => to_insert = new_groups.next(),
                        }
                    }
                }

                if let Some((language_server_id, group)) = to_insert {
                    let mut group_state = DiagnosticGroupState {
                        language_server_id,
                        primary_diagnostic: group.entries[group.primary_ix].clone(),
                        primary_excerpt_ix: 0,
                        excerpts: Default::default(),
                        blocks: Default::default(),
                        block_count: 0,
                    };
                    let mut pending_range: Option<(Range<Point>, usize)> = None;
                    let mut is_first_excerpt_for_group = true;
                    for (ix, entry) in group.entries.iter().map(Some).chain([None]).enumerate() {
                        let resolved_entry = entry.map(|e| e.resolve::<Point>(&snapshot));
                        if let Some((range, start_ix)) = &mut pending_range {
                            if let Some(entry) = resolved_entry.as_ref() {
                                if entry.range.start.row <= range.end.row + 1 + self.context * 2 {
                                    range.end = range.end.max(entry.range.end);
                                    continue;
                                }
                            }

                            let excerpt_start =
                                Point::new(range.start.row.saturating_sub(self.context), 0);
                            let excerpt_end = snapshot.clip_point(
                                Point::new(range.end.row + self.context, u32::MAX),
                                Bias::Left,
                            );

                            let excerpt_id = excerpts
                                .insert_excerpts_after(
                                    prev_excerpt_id,
                                    buffer.clone(),
                                    [ExcerptRange {
                                        context: excerpt_start..excerpt_end,
                                        primary: Some(range.clone()),
                                    }],
                                    cx,
                                )
                                .pop()
                                .unwrap();

                            prev_excerpt_id = excerpt_id;
                            first_excerpt_id.get_or_insert_with(|| prev_excerpt_id);
                            group_state.excerpts.push(excerpt_id);
                            let header_position = (excerpt_id, language::Anchor::MIN);

                            if is_first_excerpt_for_group {
                                is_first_excerpt_for_group = false;
                                let mut primary =
                                    group.entries[group.primary_ix].diagnostic.clone();
                                primary.message =
                                    primary.message.split('\n').next().unwrap().to_string();
                                group_state.block_count += 1;
                                blocks_to_add.push(BlockProperties {
                                    position: header_position,
                                    height: 2,
                                    style: BlockStyle::Sticky,
                                    render: diagnostic_header_renderer(primary),
                                    disposition: BlockDisposition::Above,
                                });
                            }

                            for entry in &group.entries[*start_ix..ix] {
                                let mut diagnostic = entry.diagnostic.clone();
                                if diagnostic.is_primary {
                                    group_state.primary_excerpt_ix = group_state.excerpts.len() - 1;
                                    diagnostic.message =
                                        entry.diagnostic.message.split('\n').skip(1).collect();
                                }

                                if !diagnostic.message.is_empty() {
                                    group_state.block_count += 1;
                                    blocks_to_add.push(BlockProperties {
                                        position: (excerpt_id, entry.range.start),
                                        height: diagnostic.message.matches('\n').count() as u8 + 1,
                                        style: BlockStyle::Fixed,
                                        render: diagnostic_block_renderer(diagnostic, true),
                                        disposition: BlockDisposition::Below,
                                    });
                                }
                            }

                            pending_range.take();
                        }

                        if let Some(entry) = resolved_entry {
                            pending_range = Some((entry.range.clone(), ix));
                        }
                    }

                    new_group_ixs.push(path_state.diagnostic_groups.len());
                    path_state.diagnostic_groups.push(group_state);
                } else if let Some((_, group_state)) = to_remove {
                    excerpts.remove_excerpts(group_state.excerpts.iter().copied(), cx);
                    blocks_to_remove.extend(group_state.blocks.iter().copied());
                } else if let Some((_, group_state)) = to_keep {
                    prev_excerpt_id = *group_state.excerpts.last().unwrap();
                    first_excerpt_id.get_or_insert_with(|| prev_excerpt_id);
                    path_state.diagnostic_groups.push(group_state);
                }
            }

            excerpts.snapshot(cx)
        });

        self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, None, cx);
            let block_ids = editor.insert_blocks(
                blocks_to_add.into_iter().flat_map(|block| {
                    let (excerpt_id, text_anchor) = block.position;
                    Some(BlockProperties {
                        position: excerpts_snapshot.anchor_in_excerpt(excerpt_id, text_anchor)?,
                        height: block.height,
                        style: block.style,
                        render: block.render,
                        disposition: block.disposition,
                    })
                }),
                Some(Autoscroll::fit()),
                cx,
            );

            let mut block_ids = block_ids.into_iter();
            for ix in new_group_ixs {
                let group_state = &mut path_state.diagnostic_groups[ix];
                group_state.blocks = block_ids.by_ref().take(group_state.block_count).collect();
            }
        });

        if path_state.diagnostic_groups.is_empty() {
            self.path_states.remove(path_ix);
        }

        self.editor.update(cx, |editor, cx| {
            let groups;
            let mut selections;
            let new_excerpt_ids_by_selection_id;
            if was_empty {
                groups = self.path_states.first()?.diagnostic_groups.as_slice();
                new_excerpt_ids_by_selection_id = [(0, ExcerptId::min())].into_iter().collect();
                selections = vec![Selection {
                    id: 0,
                    start: 0,
                    end: 0,
                    reversed: false,
                    goal: SelectionGoal::None,
                }];
            } else {
                groups = self.path_states.get(path_ix)?.diagnostic_groups.as_slice();
                new_excerpt_ids_by_selection_id =
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| s.refresh());
                selections = editor.selections.all::<usize>(cx);
            }

            // If any selection has lost its position, move it to start of the next primary diagnostic.
            let snapshot = editor.snapshot(cx);
            for selection in &mut selections {
                if let Some(new_excerpt_id) = new_excerpt_ids_by_selection_id.get(&selection.id) {
                    let group_ix = match groups.binary_search_by(|probe| {
                        probe
                            .excerpts
                            .last()
                            .unwrap()
                            .cmp(new_excerpt_id, &snapshot.buffer_snapshot)
                    }) {
                        Ok(ix) | Err(ix) => ix,
                    };
                    if let Some(group) = groups.get(group_ix) {
                        if let Some(offset) = excerpts_snapshot
                            .anchor_in_excerpt(
                                group.excerpts[group.primary_excerpt_ix],
                                group.primary_diagnostic.range.start,
                            )
                            .map(|anchor| anchor.to_offset(&excerpts_snapshot))
                        {
                            selection.start = offset;
                            selection.end = offset;
                        }
                    }
                }
            }
            editor.change_selections(None, cx, |s| {
                s.select(selections);
            });
            Some(())
        });

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

const DIAGNOSTIC_HEADER: &'static str = "diagnostic header";

fn diagnostic_header_renderer(diagnostic: Diagnostic) -> RenderBlock {
    let (message, code_ranges) = highlight_diagnostic_message(&diagnostic);
    let message: SharedString = message;
    Box::new(move |cx| {
        let highlight_style: HighlightStyle = cx.theme().colors().text_accent.into();
        h_flex()
            .id(DIAGNOSTIC_HEADER)
            .py_2()
            .pl_10()
            .pr_5()
            .w_full()
            .justify_between()
            .gap_2()
            .child(
                h_flex()
                    .gap_3()
                    .map(|stack| {
                        stack.child(
                            svg()
                                .size(cx.text_style().font_size)
                                .flex_none()
                                .map(|icon| {
                                    if diagnostic.severity == DiagnosticSeverity::ERROR {
                                        icon.path(IconName::XCircle.path())
                                            .text_color(Color::Error.color(cx))
                                    } else {
                                        icon.path(IconName::ExclamationTriangle.path())
                                            .text_color(Color::Warning.color(cx))
                                    }
                                }),
                        )
                    })
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                StyledText::new(message.clone()).with_highlights(
                                    &cx.text_style(),
                                    code_ranges
                                        .iter()
                                        .map(|range| (range.clone(), highlight_style)),
                                ),
                            )
                            .when_some(diagnostic.code.as_ref(), |stack, code| {
                                stack.child(
                                    div()
                                        .child(SharedString::from(format!("({code})")))
                                        .text_color(cx.theme().colors().text_muted),
                                )
                            }),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .when_some(diagnostic.source.as_ref(), |stack, source| {
                        stack.child(
                            div()
                                .child(SharedString::from(source.clone()))
                                .text_color(cx.theme().colors().text_muted),
                        )
                    }),
            )
            .into_any_element()
    })
}

fn compare_diagnostics(
    old: &DiagnosticEntry<language::Anchor>,
    new: &DiagnosticEntry<language::Anchor>,
    snapshot: &language::BufferSnapshot,
) -> Ordering {
    use language::ToOffset;
    // The old diagnostics may point to a previously open Buffer for this file.
    if !old.range.start.is_valid(snapshot) {
        return Ordering::Greater;
    }
    old.range
        .start
        .to_offset(snapshot)
        .cmp(&new.range.start.to_offset(snapshot))
        .then_with(|| {
            old.range
                .end
                .to_offset(snapshot)
                .cmp(&new.range.end.to_offset(snapshot))
        })
        .then_with(|| old.diagnostic.message.cmp(&new.diagnostic.message))
}
