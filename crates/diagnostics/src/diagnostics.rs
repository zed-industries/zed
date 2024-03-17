pub mod items;
mod project_diagnostics_settings;
mod toolbar_controls;

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use editor::{
    diagnostic_block_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock},
    highlight_diagnostic_message,
    scroll::Autoscroll,
    Editor, EditorEvent, ExcerptId, ExcerptRange, MultiBuffer, ToOffset,
};
use futures::future::try_join_all;
use gpui::{
    actions, div, svg, AnyElement, AnyView, AppContext, Context, EventEmitter, FocusHandle,
    FocusableView, HighlightStyle, InteractiveElement, IntoElement, Model, ParentElement, Render,
    SharedString, Styled, StyledText, Subscription, Task, View, ViewContext, VisualContext,
    WeakView, WindowContext,
};
use language::{
    Anchor, Bias, Buffer, Diagnostic, DiagnosticEntry, DiagnosticSeverity, Point, Selection,
    SelectionGoal,
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
    path::PathBuf,
    sync::Arc,
};
use theme::ActiveTheme;
pub use toolbar_controls::ToolbarControls;
use ui::{h_flex, prelude::*, Icon, IconName, Label};
use util::TryFutureExt;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle},
    ItemNavHistory, Pane, ToolbarItemLocation, Workspace,
};

actions!(diagnostics, [Deploy, ToggleWarnings]);

const CONTEXT_LINE_COUNT: u32 = 1;

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
    paths_to_update: HashMap<LanguageServerId, HashSet<ProjectPath>>,
    current_diagnostics: HashMap<LanguageServerId, HashSet<ProjectPath>>,
    include_warnings: bool,
    _subscriptions: Vec<Subscription>,
}

struct PathState {
    path: ProjectPath,
    diagnostic_groups: Vec<DiagnosticGroupState>,
}

#[derive(Clone, Debug, PartialEq)]
struct Jump {
    path: ProjectPath,
    position: Point,
    anchor: Anchor,
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element {
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

    fn new(
        project_handle: Model<Project>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let project_event_subscription =
            cx.subscribe(&project_handle, |this, _, event, cx| match event {
                project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                    log::debug!("Disk based diagnostics finished for server {language_server_id}");
                    this.update_excerpts(Some(*language_server_id), cx);
                }
                project::Event::DiagnosticsUpdated {
                    language_server_id,
                    path,
                } => {
                    log::debug!("Adding path {path:?} to update for server {language_server_id}");
                    this.paths_to_update
                        .entry(*language_server_id)
                        .or_default()
                        .insert(path.clone());
                    if this.editor.read(cx).selections.all::<usize>(cx).is_empty()
                        && !this.is_dirty(cx)
                    {
                        this.update_excerpts(Some(*language_server_id), cx);
                    }
                }
                _ => {}
            });

        let focus_handle = cx.focus_handle();

        let focus_in_subscription =
            cx.on_focus_in(&focus_handle, |diagnostics, cx| diagnostics.focus_in(cx));

        let excerpts = cx.new_model(|cx| {
            MultiBuffer::new(
                project_handle.read(cx).replica_id(),
                project_handle.read(cx).capability(),
            )
        });
        let editor = cx.new_view(|cx| {
            let mut editor =
                Editor::for_multibuffer(excerpts.clone(), Some(project_handle.clone()), cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor
        });
        let editor_event_subscription =
            cx.subscribe(&editor, |this, _editor, event: &EditorEvent, cx| {
                cx.emit(event.clone());
                if event == &EditorEvent::Focused && this.path_states.is_empty() {
                    cx.focus(&this.focus_handle);
                }
            });

        let project = project_handle.read(cx);
        let summary = project.diagnostic_summary(false, cx);
        let mut this = Self {
            project: project_handle,
            summary,
            workspace,
            excerpts,
            focus_handle,
            editor,
            path_states: Default::default(),
            paths_to_update: HashMap::default(),
            include_warnings: ProjectDiagnosticsSettings::get_global(cx).include_warnings,
            current_diagnostics: HashMap::default(),
            _subscriptions: vec![
                project_event_subscription,
                editor_event_subscription,
                focus_in_subscription,
            ],
        };
        this.update_excerpts(None, cx);
        this
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        if let Some(existing) = workspace.item_of_type::<ProjectDiagnosticsEditor>(cx) {
            workspace.activate_item(&existing, cx);
        } else {
            let workspace_handle = cx.view().downgrade();
            let diagnostics = cx.new_view(|cx| {
                ProjectDiagnosticsEditor::new(workspace.project().clone(), workspace_handle, cx)
            });
            workspace.add_item_to_active_pane(Box::new(diagnostics), cx);
        }
    }

    fn toggle_warnings(&mut self, _: &ToggleWarnings, cx: &mut ViewContext<Self>) {
        self.include_warnings = !self.include_warnings;
        self.paths_to_update = self.current_diagnostics.clone();
        self.update_excerpts(None, cx);
        cx.notify();
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        if self.focus_handle.is_focused(cx) && !self.path_states.is_empty() {
            self.editor.focus_handle(cx).focus(cx)
        }
    }

    fn update_excerpts(
        &mut self,
        language_server_id: Option<LanguageServerId>,
        cx: &mut ViewContext<Self>,
    ) {
        log::debug!("Updating excerpts for server {language_server_id:?}");
        let mut paths_to_recheck = HashSet::default();
        let mut new_summaries: HashMap<LanguageServerId, HashSet<ProjectPath>> = self
            .project
            .read(cx)
            .diagnostic_summaries(false, cx)
            .fold(HashMap::default(), |mut summaries, (path, server_id, _)| {
                summaries.entry(server_id).or_default().insert(path);
                summaries
            });
        let mut old_diagnostics = if let Some(language_server_id) = language_server_id {
            new_summaries.retain(|server_id, _| server_id == &language_server_id);
            self.paths_to_update.retain(|server_id, paths| {
                if server_id == &language_server_id {
                    paths_to_recheck.extend(paths.drain());
                    false
                } else {
                    true
                }
            });
            let mut old_diagnostics = HashMap::default();
            if let Some(new_paths) = new_summaries.get(&language_server_id) {
                if let Some(old_paths) = self
                    .current_diagnostics
                    .insert(language_server_id, new_paths.clone())
                {
                    old_diagnostics.insert(language_server_id, old_paths);
                }
            } else {
                if let Some(old_paths) = self.current_diagnostics.remove(&language_server_id) {
                    old_diagnostics.insert(language_server_id, old_paths);
                }
            }
            old_diagnostics
        } else {
            paths_to_recheck.extend(self.paths_to_update.drain().flat_map(|(_, paths)| paths));
            mem::replace(&mut self.current_diagnostics, new_summaries.clone())
        };
        for (server_id, new_paths) in new_summaries {
            match old_diagnostics.remove(&server_id) {
                Some(mut old_paths) => {
                    paths_to_recheck.extend(
                        new_paths
                            .into_iter()
                            .filter(|new_path| !old_paths.remove(new_path)),
                    );
                    paths_to_recheck.extend(old_paths);
                }
                None => paths_to_recheck.extend(new_paths),
            }
        }
        paths_to_recheck.extend(old_diagnostics.into_iter().flat_map(|(_, paths)| paths));

        if paths_to_recheck.is_empty() {
            log::debug!("No paths to recheck for language server {language_server_id:?}");
            return;
        }
        log::debug!(
            "Rechecking {} paths for language server {:?}",
            paths_to_recheck.len(),
            language_server_id
        );
        let project = self.project.clone();
        cx.spawn(|this, mut cx| {
            async move {
                let _: Vec<()> = try_join_all(paths_to_recheck.into_iter().map(|path| {
                    let mut cx = cx.clone();
                    let project = project.clone();
                    let this = this.clone();
                    async move {
                        let buffer = project
                            .update(&mut cx, |project, cx| project.open_buffer(path.clone(), cx))?
                            .await
                            .with_context(|| format!("opening buffer for path {path:?}"))?;
                        this.update(&mut cx, |this, cx| {
                            this.populate_excerpts(path, language_server_id, buffer, cx);
                        })
                        .context("missing project")?;
                        anyhow::Ok(())
                    }
                }))
                .await
                .context("rechecking diagnostics for paths")?;

                this.update(&mut cx, |this, cx| {
                    this.summary = this.project.read(cx).diagnostic_summary(false, cx);
                    cx.emit(EditorEvent::TitleChanged);
                })?;
                anyhow::Ok(())
            }
            .log_err()
        })
        .detach();
    }

    fn populate_excerpts(
        &mut self,
        path: ProjectPath,
        language_server_id: Option<LanguageServerId>,
        buffer: Model<Buffer>,
        cx: &mut ViewContext<Self>,
    ) {
        let was_empty = self.path_states.is_empty();
        let snapshot = buffer.read(cx).snapshot();
        let path_ix = match self.path_states.binary_search_by_key(&&path, |e| &e.path) {
            Ok(ix) => ix,
            Err(ix) => {
                self.path_states.insert(
                    ix,
                    PathState {
                        path: path.clone(),
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
        let mut groups_to_add = Vec::new();
        let mut group_ixs_to_remove = Vec::new();
        let mut blocks_to_add = Vec::new();
        let mut blocks_to_remove = HashSet::default();
        let mut first_excerpt_id = None;
        let max_severity = if self.include_warnings {
            DiagnosticSeverity::WARNING
        } else {
            DiagnosticSeverity::ERROR
        };
        let excerpts_snapshot = self.excerpts.update(cx, |excerpts, excerpts_cx| {
            let mut old_groups = path_state.diagnostic_groups.iter().enumerate().peekable();
            let mut new_groups = snapshot
                .diagnostic_groups(language_server_id)
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
                        if language_server_id.map_or(true, |id| id == old_group.language_server_id)
                        {
                            to_remove = old_groups.next();
                        } else {
                            to_keep = old_groups.next();
                        }
                    }
                    (Some((_, old_group)), Some((_, new_group))) => {
                        let old_primary = &old_group.primary_diagnostic;
                        let new_primary = &new_group.entries[new_group.primary_ix];
                        match compare_diagnostics(old_primary, new_primary, &snapshot) {
                            Ordering::Less => {
                                if language_server_id
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
                                if entry.range.start.row
                                    <= range.end.row + 1 + CONTEXT_LINE_COUNT * 2
                                {
                                    range.end = range.end.max(entry.range.end);
                                    continue;
                                }
                            }

                            let excerpt_start =
                                Point::new(range.start.row.saturating_sub(CONTEXT_LINE_COUNT), 0);
                            let excerpt_end = snapshot.clip_point(
                                Point::new(range.end.row + CONTEXT_LINE_COUNT, u32::MAX),
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
                                    excerpts_cx,
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

                    groups_to_add.push(group_state);
                } else if let Some((group_ix, group_state)) = to_remove {
                    excerpts.remove_excerpts(group_state.excerpts.iter().copied(), excerpts_cx);
                    group_ixs_to_remove.push(group_ix);
                    blocks_to_remove.extend(group_state.blocks.iter().copied());
                } else if let Some((_, group)) = to_keep {
                    prev_excerpt_id = *group.excerpts.last().unwrap();
                    first_excerpt_id.get_or_insert_with(|| prev_excerpt_id);
                }
            }

            excerpts.snapshot(excerpts_cx)
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
            for group_state in &mut groups_to_add {
                group_state.blocks = block_ids.by_ref().take(group_state.block_count).collect();
            }
        });

        for ix in group_ixs_to_remove.into_iter().rev() {
            path_state.diagnostic_groups.remove(ix);
        }
        path_state.diagnostic_groups.extend(groups_to_add);
        path_state.diagnostic_groups.sort_unstable_by(|a, b| {
            let range_a = &a.primary_diagnostic.range;
            let range_b = &b.primary_diagnostic.range;
            range_a
                .start
                .cmp(&range_b.start, &snapshot)
                .then_with(|| range_a.end.cmp(&range_b.end, &snapshot))
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
        cx.notify();
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

    fn tab_content(&self, _detail: Option<usize>, selected: bool, _: &WindowContext) -> AnyElement {
        if self.summary.error_count == 0 && self.summary.warning_count == 0 {
            Label::new("No problems")
                .color(if selected {
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
                                if selected {
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
                                if selected {
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
        _workspace_id: workspace::WorkspaceId,
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
        _: PathBuf,
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

fn diagnostic_header_renderer(diagnostic: Diagnostic) -> RenderBlock {
    let (message, code_ranges) = highlight_diagnostic_message(&diagnostic);
    let message: SharedString = message;
    Arc::new(move |cx| {
        let highlight_style: HighlightStyle = cx.theme().colors().text_accent.into();
        h_flex()
            .id("diagnostic header")
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

fn compare_diagnostics<L: language::ToOffset, R: language::ToOffset>(
    lhs: &DiagnosticEntry<L>,
    rhs: &DiagnosticEntry<R>,
    snapshot: &language::BufferSnapshot,
) -> Ordering {
    lhs.range
        .start
        .to_offset(snapshot)
        .cmp(&rhs.range.start.to_offset(snapshot))
        .then_with(|| {
            lhs.range
                .end
                .to_offset(snapshot)
                .cmp(&rhs.range.end.to_offset(snapshot))
        })
        .then_with(|| lhs.diagnostic.message.cmp(&rhs.diagnostic.message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{
        display_map::{BlockContext, TransformBlock},
        DisplayPoint, GutterDimensions,
    };
    use gpui::{px, Stateful, TestAppContext, VisualTestContext, WindowContext};
    use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity, PointUtf16, Unclipped};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_diagnostics(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/test",
            json!({
                "consts.rs": "
                    const a: i32 = 'a';
                    const b: i32 = c;
                "
                .unindent(),

                "main.rs": "
                    fn main() {
                        let x = vec![];
                        let y = vec![];
                        a(x);
                        b(y);
                        // comment 1
                        // comment 2
                        c(y);
                        d(x);
                    }
                "
                .unindent(),
            }),
        )
        .await;

        let language_server_id = LanguageServerId(0);
        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let workspace = window.root(cx).unwrap();

        // Create some diagnostics
        project.update(cx, |project, cx| {
            project
                .update_diagnostic_entries(
                    language_server_id,
                    PathBuf::from("/test/main.rs"),
                    None,
                    vec![
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(1, 8))..Unclipped(PointUtf16::new(1, 9)),
                            diagnostic: Diagnostic {
                                message:
                                    "move occurs because `x` has type `Vec<char>`, which does not implement the `Copy` trait"
                                        .to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(2, 8))..Unclipped(PointUtf16::new(2, 9)),
                            diagnostic: Diagnostic {
                                message:
                                    "move occurs because `y` has type `Vec<char>`, which does not implement the `Copy` trait"
                                        .to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(3, 6))..Unclipped(PointUtf16::new(3, 7)),
                            diagnostic: Diagnostic {
                                message: "value moved here".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(4, 6))..Unclipped(PointUtf16::new(4, 7)),
                            diagnostic: Diagnostic {
                                message: "value moved here".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(7, 6))..Unclipped(PointUtf16::new(7, 7)),
                            diagnostic: Diagnostic {
                                message: "use of moved value\nvalue used here after move".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(8, 6))..Unclipped(PointUtf16::new(8, 7)),
                            diagnostic: Diagnostic {
                                message: "use of moved value\nvalue used here after move".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                    ],
                    cx,
                )
                .unwrap();
        });

        // Open the project diagnostics view while there are already diagnostics.
        let view = window.build_view(cx, |cx| {
            ProjectDiagnosticsEditor::new(project.clone(), workspace.downgrade(), cx)
        });

        view.next_notification(cx).await;
        view.update(cx, |view, cx| {
            assert_eq!(
                editor_blocks(&view.editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                    (15, "collapsed context".into()),
                    (16, "diagnostic header".into()),
                    (25, "collapsed context".into()),
                ]
            );
            assert_eq!(
                view.editor.update(cx, |editor, cx| editor.display_text(cx)),
                concat!(
                    //
                    // main.rs
                    //
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n", // primary message
                    "\n", // padding
                    "    let x = vec![];\n",
                    "    let y = vec![];\n",
                    "\n", // supporting diagnostic
                    "    a(x);\n",
                    "    b(y);\n",
                    "\n", // supporting diagnostic
                    "    // comment 1\n",
                    "    // comment 2\n",
                    "    c(y);\n",
                    "\n", // supporting diagnostic
                    "    d(x);\n",
                    "\n", // context ellipsis
                    // diagnostic group 2
                    "\n", // primary message
                    "\n", // padding
                    "fn main() {\n",
                    "    let x = vec![];\n",
                    "\n", // supporting diagnostic
                    "    let y = vec![];\n",
                    "    a(x);\n",
                    "\n", // supporting diagnostic
                    "    b(y);\n",
                    "\n", // context ellipsis
                    "    c(y);\n",
                    "    d(x);\n",
                    "\n", // supporting diagnostic
                    "}"
                )
            );

            // Cursor is at the first diagnostic
            view.editor.update(cx, |editor, cx| {
                assert_eq!(
                    editor.selections.display_ranges(cx),
                    [DisplayPoint::new(12, 6)..DisplayPoint::new(12, 6)]
                );
            });
        });

        // Diagnostics are added for another earlier path.
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_started(language_server_id, cx);
            project
                .update_diagnostic_entries(
                    language_server_id,
                    PathBuf::from("/test/consts.rs"),
                    None,
                    vec![DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(0, 15))..Unclipped(PointUtf16::new(0, 15)),
                        diagnostic: Diagnostic {
                            message: "mismatched types\nexpected `usize`, found `char`".to_string(),
                            severity: DiagnosticSeverity::ERROR,
                            is_primary: true,
                            is_disk_based: true,
                            group_id: 0,
                            ..Default::default()
                        },
                    }],
                    cx,
                )
                .unwrap();
            project.disk_based_diagnostics_finished(language_server_id, cx);
        });

        view.next_notification(cx).await;
        view.update(cx, |view, cx| {
            assert_eq!(
                editor_blocks(&view.editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                    (7, "path header block".into()),
                    (9, "diagnostic header".into()),
                    (22, "collapsed context".into()),
                    (23, "diagnostic header".into()),
                    (32, "collapsed context".into()),
                ]
            );
            assert_eq!(
                view.editor.update(cx, |editor, cx| editor.display_text(cx)),
                concat!(
                    //
                    // consts.rs
                    //
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n", // primary message
                    "\n", // padding
                    "const a: i32 = 'a';\n",
                    "\n", // supporting diagnostic
                    "const b: i32 = c;\n",
                    //
                    // main.rs
                    //
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n", // primary message
                    "\n", // padding
                    "    let x = vec![];\n",
                    "    let y = vec![];\n",
                    "\n", // supporting diagnostic
                    "    a(x);\n",
                    "    b(y);\n",
                    "\n", // supporting diagnostic
                    "    // comment 1\n",
                    "    // comment 2\n",
                    "    c(y);\n",
                    "\n", // supporting diagnostic
                    "    d(x);\n",
                    "\n", // collapsed context
                    // diagnostic group 2
                    "\n", // primary message
                    "\n", // filename
                    "fn main() {\n",
                    "    let x = vec![];\n",
                    "\n", // supporting diagnostic
                    "    let y = vec![];\n",
                    "    a(x);\n",
                    "\n", // supporting diagnostic
                    "    b(y);\n",
                    "\n", // context ellipsis
                    "    c(y);\n",
                    "    d(x);\n",
                    "\n", // supporting diagnostic
                    "}"
                )
            );

            // Cursor keeps its position.
            view.editor.update(cx, |editor, cx| {
                assert_eq!(
                    editor.selections.display_ranges(cx),
                    [DisplayPoint::new(19, 6)..DisplayPoint::new(19, 6)]
                );
            });
        });

        // Diagnostics are added to the first path
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_started(language_server_id, cx);
            project
                .update_diagnostic_entries(
                    language_server_id,
                    PathBuf::from("/test/consts.rs"),
                    None,
                    vec![
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(0, 15))
                                ..Unclipped(PointUtf16::new(0, 15)),
                            diagnostic: Diagnostic {
                                message: "mismatched types\nexpected `usize`, found `char`"
                                    .to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(1, 15))
                                ..Unclipped(PointUtf16::new(1, 15)),
                            diagnostic: Diagnostic {
                                message: "unresolved name `c`".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                    ],
                    cx,
                )
                .unwrap();
            project.disk_based_diagnostics_finished(language_server_id, cx);
        });

        view.next_notification(cx).await;
        view.update(cx, |view, cx| {
            assert_eq!(
                editor_blocks(&view.editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                    (7, "collapsed context".into()),
                    (8, "diagnostic header".into()),
                    (13, "path header block".into()),
                    (15, "diagnostic header".into()),
                    (28, "collapsed context".into()),
                    (29, "diagnostic header".into()),
                    (38, "collapsed context".into()),
                ]
            );
            assert_eq!(
                view.editor.update(cx, |editor, cx| editor.display_text(cx)),
                concat!(
                    //
                    // consts.rs
                    //
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n", // primary message
                    "\n", // padding
                    "const a: i32 = 'a';\n",
                    "\n", // supporting diagnostic
                    "const b: i32 = c;\n",
                    "\n", // context ellipsis
                    // diagnostic group 2
                    "\n", // primary message
                    "\n", // padding
                    "const a: i32 = 'a';\n",
                    "const b: i32 = c;\n",
                    "\n", // supporting diagnostic
                    //
                    // main.rs
                    //
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n", // primary message
                    "\n", // padding
                    "    let x = vec![];\n",
                    "    let y = vec![];\n",
                    "\n", // supporting diagnostic
                    "    a(x);\n",
                    "    b(y);\n",
                    "\n", // supporting diagnostic
                    "    // comment 1\n",
                    "    // comment 2\n",
                    "    c(y);\n",
                    "\n", // supporting diagnostic
                    "    d(x);\n",
                    "\n", // context ellipsis
                    // diagnostic group 2
                    "\n", // primary message
                    "\n", // filename
                    "fn main() {\n",
                    "    let x = vec![];\n",
                    "\n", // supporting diagnostic
                    "    let y = vec![];\n",
                    "    a(x);\n",
                    "\n", // supporting diagnostic
                    "    b(y);\n",
                    "\n", // context ellipsis
                    "    c(y);\n",
                    "    d(x);\n",
                    "\n", // supporting diagnostic
                    "}"
                )
            );
        });
    }

    #[gpui::test]
    async fn test_diagnostics_multiple_servers(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/test",
            json!({
                "main.js": "
                    a();
                    b();
                    c();
                    d();
                    e();
                ".unindent()
            }),
        )
        .await;

        let server_id_1 = LanguageServerId(100);
        let server_id_2 = LanguageServerId(101);
        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let cx = &mut VisualTestContext::from_window(*window, cx);
        let workspace = window.root(cx).unwrap();

        let view = window.build_view(cx, |cx| {
            ProjectDiagnosticsEditor::new(project.clone(), workspace.downgrade(), cx)
        });

        // Two language servers start updating diagnostics
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_started(server_id_1, cx);
            project.disk_based_diagnostics_started(server_id_2, cx);
            project
                .update_diagnostic_entries(
                    server_id_1,
                    PathBuf::from("/test/main.js"),
                    None,
                    vec![DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(0, 0))..Unclipped(PointUtf16::new(0, 1)),
                        diagnostic: Diagnostic {
                            message: "error 1".to_string(),
                            severity: DiagnosticSeverity::WARNING,
                            is_primary: true,
                            is_disk_based: true,
                            group_id: 1,
                            ..Default::default()
                        },
                    }],
                    cx,
                )
                .unwrap();
        });

        // The first language server finishes
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_finished(server_id_1, cx);
        });

        // Only the first language server's diagnostics are shown.
        cx.executor().run_until_parked();
        view.update(cx, |view, cx| {
            assert_eq!(
                editor_blocks(&view.editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                ]
            );
            assert_eq!(
                view.editor.update(cx, |editor, cx| editor.display_text(cx)),
                concat!(
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n",     // primary message
                    "\n",     // padding
                    "a();\n", //
                    "b();",
                )
            );
        });

        // The second language server finishes
        project.update(cx, |project, cx| {
            project
                .update_diagnostic_entries(
                    server_id_2,
                    PathBuf::from("/test/main.js"),
                    None,
                    vec![DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(1, 0))..Unclipped(PointUtf16::new(1, 1)),
                        diagnostic: Diagnostic {
                            message: "warning 1".to_string(),
                            severity: DiagnosticSeverity::ERROR,
                            is_primary: true,
                            is_disk_based: true,
                            group_id: 2,
                            ..Default::default()
                        },
                    }],
                    cx,
                )
                .unwrap();
            project.disk_based_diagnostics_finished(server_id_2, cx);
        });

        // Both language server's diagnostics are shown.
        cx.executor().run_until_parked();
        view.update(cx, |view, cx| {
            assert_eq!(
                editor_blocks(&view.editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                    (6, "collapsed context".into()),
                    (7, "diagnostic header".into()),
                ]
            );
            assert_eq!(
                view.editor.update(cx, |editor, cx| editor.display_text(cx)),
                concat!(
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n",     // primary message
                    "\n",     // padding
                    "a();\n", // location
                    "b();\n", //
                    "\n",     // collapsed context
                    // diagnostic group 2
                    "\n",     // primary message
                    "\n",     // padding
                    "a();\n", // context
                    "b();\n", //
                    "c();",   // context
                )
            );
        });

        // Both language servers start updating diagnostics, and the first server finishes.
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_started(server_id_1, cx);
            project.disk_based_diagnostics_started(server_id_2, cx);
            project
                .update_diagnostic_entries(
                    server_id_1,
                    PathBuf::from("/test/main.js"),
                    None,
                    vec![DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(2, 0))..Unclipped(PointUtf16::new(2, 1)),
                        diagnostic: Diagnostic {
                            message: "warning 2".to_string(),
                            severity: DiagnosticSeverity::WARNING,
                            is_primary: true,
                            is_disk_based: true,
                            group_id: 1,
                            ..Default::default()
                        },
                    }],
                    cx,
                )
                .unwrap();
            project
                .update_diagnostic_entries(
                    server_id_2,
                    PathBuf::from("/test/main.rs"),
                    None,
                    vec![],
                    cx,
                )
                .unwrap();
            project.disk_based_diagnostics_finished(server_id_1, cx);
        });

        // Only the first language server's diagnostics are updated.
        cx.executor().run_until_parked();
        view.update(cx, |view, cx| {
            assert_eq!(
                editor_blocks(&view.editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                    (7, "collapsed context".into()),
                    (8, "diagnostic header".into()),
                ]
            );
            assert_eq!(
                view.editor.update(cx, |editor, cx| editor.display_text(cx)),
                concat!(
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n",     // primary message
                    "\n",     // padding
                    "a();\n", // location
                    "b();\n", //
                    "c();\n", // context
                    "\n",     // collapsed context
                    // diagnostic group 2
                    "\n",     // primary message
                    "\n",     // padding
                    "b();\n", // context
                    "c();\n", //
                    "d();",   // context
                )
            );
        });

        // The second language server finishes.
        project.update(cx, |project, cx| {
            project
                .update_diagnostic_entries(
                    server_id_2,
                    PathBuf::from("/test/main.js"),
                    None,
                    vec![DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(3, 0))..Unclipped(PointUtf16::new(3, 1)),
                        diagnostic: Diagnostic {
                            message: "warning 2".to_string(),
                            severity: DiagnosticSeverity::WARNING,
                            is_primary: true,
                            is_disk_based: true,
                            group_id: 1,
                            ..Default::default()
                        },
                    }],
                    cx,
                )
                .unwrap();
            project.disk_based_diagnostics_finished(server_id_2, cx);
        });

        // Both language servers' diagnostics are updated.
        cx.executor().run_until_parked();
        view.update(cx, |view, cx| {
            assert_eq!(
                editor_blocks(&view.editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                    (7, "collapsed context".into()),
                    (8, "diagnostic header".into()),
                ]
            );
            assert_eq!(
                view.editor.update(cx, |editor, cx| editor.display_text(cx)),
                concat!(
                    "\n", // filename
                    "\n", // padding
                    // diagnostic group 1
                    "\n",     // primary message
                    "\n",     // padding
                    "b();\n", // location
                    "c();\n", //
                    "d();\n", // context
                    "\n",     // collapsed context
                    // diagnostic group 2
                    "\n",     // primary message
                    "\n",     // padding
                    "c();\n", // context
                    "d();\n", //
                    "e();",   // context
                )
            );
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            client::init_settings(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            crate::init(cx);
            editor::init(cx);
        });
    }

    fn editor_blocks(editor: &View<Editor>, cx: &mut WindowContext) -> Vec<(u32, SharedString)> {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            snapshot
                .blocks_in_range(0..snapshot.max_point().row())
                .enumerate()
                .filter_map(|(ix, (row, block))| {
                    let name: SharedString = match block {
                        TransformBlock::Custom(block) => cx.with_element_context({
                            |cx| -> Option<SharedString> {
                                let mut element = block.render(&mut BlockContext {
                                    context: cx,
                                    anchor_x: px(0.),
                                    gutter_dimensions: &GutterDimensions::default(),
                                    line_height: px(0.),
                                    em_width: px(0.),
                                    max_width: px(0.),
                                    block_id: ix,
                                    editor_style: &editor::EditorStyle::default(),
                                });
                                let element = element.downcast_mut::<Stateful<Div>>().unwrap();
                                element.interactivity().element_id.clone()?.try_into().ok()
                            }
                        })?,

                        TransformBlock::ExcerptHeader {
                            starts_new_buffer, ..
                        } => {
                            if *starts_new_buffer {
                                "path header block".into()
                            } else {
                                "collapsed context".into()
                            }
                        }
                    };

                    Some((row, name))
                })
                .collect()
        })
    }
}
