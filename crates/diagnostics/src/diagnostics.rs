pub mod items;

use anyhow::Result;
use collections::{BTreeSet, HashSet};
use editor::{
    diagnostic_block_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock},
    highlight_diagnostic_message,
    scroll::autoscroll::Autoscroll,
    Editor, ExcerptId, ExcerptRange, MultiBuffer, ToOffset,
};
use gpui::{
    actions, elements::*, fonts::TextStyle, serde_json, AnyViewHandle, AppContext, Entity,
    ModelHandle, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::{
    Anchor, Bias, Buffer, Diagnostic, DiagnosticEntry, DiagnosticSeverity, Point, Selection,
    SelectionGoal,
};
use lsp::LanguageServerId;
use project::{DiagnosticSummary, Project, ProjectPath};
use serde_json::json;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    cmp::Ordering,
    ops::Range,
    path::PathBuf,
    sync::Arc,
};
use theme::ThemeSettings;
use util::TryFutureExt;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle},
    ItemNavHistory, Pane, PaneBackdrop, ToolbarItemLocation, Workspace,
};

actions!(diagnostics, [Deploy]);

const CONTEXT_LINE_COUNT: u32 = 1;

pub fn init(cx: &mut AppContext) {
    cx.add_action(ProjectDiagnosticsEditor::deploy);
    items::init(cx);
}

type Event = editor::Event;

struct ProjectDiagnosticsEditor {
    project: ModelHandle<Project>,
    workspace: WeakViewHandle<Workspace>,
    editor: ViewHandle<Editor>,
    summary: DiagnosticSummary,
    excerpts: ModelHandle<MultiBuffer>,
    path_states: Vec<PathState>,
    paths_to_update: BTreeSet<(ProjectPath, LanguageServerId)>,
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

impl Entity for ProjectDiagnosticsEditor {
    type Event = Event;
}

impl View for ProjectDiagnosticsEditor {
    fn ui_name() -> &'static str {
        "ProjectDiagnosticsEditor"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if self.path_states.is_empty() {
            let theme = &theme::current(cx).project_diagnostics;
            PaneBackdrop::new(
                cx.view_id(),
                Label::new("No problems in workspace", theme.empty_message.clone())
                    .aligned()
                    .contained()
                    .with_style(theme.container)
                    .into_any(),
            )
            .into_any()
        } else {
            ChildView::new(&self.editor, cx).into_any()
        }
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() && !self.path_states.is_empty() {
            cx.focus(&self.editor);
        }
    }

    fn debug_json(&self, cx: &AppContext) -> serde_json::Value {
        let project = self.project.read(cx);
        json!({
            "project": json!({
                "language_servers": project.language_server_statuses().collect::<Vec<_>>(),
                "summary": project.diagnostic_summary(cx),
            }),
            "summary": self.summary,
            "paths_to_update": self.paths_to_update.iter().map(|(path, server_id)|
                (path.path.to_string_lossy(), server_id.0)
            ).collect::<Vec<_>>(),
            "paths_states": self.path_states.iter().map(|state|
                json!({
                    "path": state.path.path.to_string_lossy(),
                    "groups": state.diagnostic_groups.iter().map(|group|
                        json!({
                            "block_count": group.blocks.len(),
                            "excerpt_count": group.excerpts.len(),
                        })
                    ).collect::<Vec<_>>(),
                })
            ).collect::<Vec<_>>(),
        })
    }
}

impl ProjectDiagnosticsEditor {
    fn new(
        project_handle: ModelHandle<Project>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(&project_handle, |this, _, event, cx| match event {
            project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                this.update_excerpts(Some(*language_server_id), cx);
                this.update_title(cx);
            }
            project::Event::DiagnosticsUpdated {
                language_server_id,
                path,
            } => {
                this.paths_to_update
                    .insert((path.clone(), *language_server_id));
            }
            _ => {}
        })
        .detach();

        let excerpts = cx.add_model(|cx| MultiBuffer::new(project_handle.read(cx).replica_id()));
        let editor = cx.add_view(|cx| {
            let mut editor =
                Editor::for_multibuffer(excerpts.clone(), Some(project_handle.clone()), cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor
        });
        cx.subscribe(&editor, |this, _, event, cx| {
            cx.emit(event.clone());
            if event == &editor::Event::Focused && this.path_states.is_empty() {
                cx.focus_self()
            }
        })
        .detach();

        let project = project_handle.read(cx);
        let paths_to_update = project
            .diagnostic_summaries(cx)
            .map(|(path, server_id, _)| (path, server_id))
            .collect();
        let summary = project.diagnostic_summary(cx);
        let mut this = Self {
            project: project_handle,
            summary,
            workspace,
            excerpts,
            editor,
            path_states: Default::default(),
            paths_to_update,
        };
        this.update_excerpts(None, cx);
        this
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        if let Some(existing) = workspace.item_of_type::<ProjectDiagnosticsEditor>(cx) {
            workspace.activate_item(&existing, cx);
        } else {
            let workspace_handle = cx.weak_handle();
            let diagnostics = cx.add_view(|cx| {
                ProjectDiagnosticsEditor::new(workspace.project().clone(), workspace_handle, cx)
            });
            workspace.add_item(Box::new(diagnostics), cx);
        }
    }

    fn update_excerpts(
        &mut self,
        language_server_id: Option<LanguageServerId>,
        cx: &mut ViewContext<Self>,
    ) {
        let mut paths = Vec::new();
        self.paths_to_update.retain(|(path, server_id)| {
            if language_server_id
                .map_or(true, |language_server_id| language_server_id == *server_id)
            {
                paths.push(path.clone());
                false
            } else {
                true
            }
        });
        let project = self.project.clone();
        cx.spawn(|this, mut cx| {
            async move {
                for path in paths {
                    let buffer = project
                        .update(&mut cx, |project, cx| project.open_buffer(path.clone(), cx))
                        .await?;
                    this.update(&mut cx, |this, cx| {
                        this.populate_excerpts(path, language_server_id, buffer, cx)
                    })?;
                }
                Result::<_, anyhow::Error>::Ok(())
            }
            .log_err()
        })
        .detach();
    }

    fn populate_excerpts(
        &mut self,
        path: ProjectPath,
        language_server_id: Option<LanguageServerId>,
        buffer: ModelHandle<Buffer>,
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
            prev_path_last_group.excerpts.last().unwrap().clone()
        } else {
            ExcerptId::min()
        };

        let path_state = &mut self.path_states[path_ix];
        let mut groups_to_add = Vec::new();
        let mut group_ixs_to_remove = Vec::new();
        let mut blocks_to_add = Vec::new();
        let mut blocks_to_remove = HashSet::default();
        let mut first_excerpt_id = None;
        let excerpts_snapshot = self.excerpts.update(cx, |excerpts, excerpts_cx| {
            let mut old_groups = path_state.diagnostic_groups.iter().enumerate().peekable();
            let mut new_groups = snapshot
                .diagnostic_groups(language_server_id)
                .into_iter()
                .filter(|(_, group)| {
                    group.entries[group.primary_ix].diagnostic.severity
                        <= DiagnosticSeverity::WARNING
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

                            prev_excerpt_id = excerpt_id.clone();
                            first_excerpt_id.get_or_insert_with(|| prev_excerpt_id.clone());
                            group_state.excerpts.push(excerpt_id.clone());
                            let header_position = (excerpt_id.clone(), language::Anchor::MIN);

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
                                        position: (excerpt_id.clone(), entry.range.start),
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
                    prev_excerpt_id = group.excerpts.last().unwrap().clone();
                    first_excerpt_id.get_or_insert_with(|| prev_excerpt_id.clone());
                }
            }

            excerpts.snapshot(excerpts_cx)
        });

        self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, None, cx);
            let block_ids = editor.insert_blocks(
                blocks_to_add.into_iter().map(|block| {
                    let (excerpt_id, text_anchor) = block.position;
                    BlockProperties {
                        position: excerpts_snapshot.anchor_in_excerpt(excerpt_id, text_anchor),
                        height: block.height,
                        style: block.style,
                        render: block.render,
                        disposition: block.disposition,
                    }
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
                        let offset = excerpts_snapshot
                            .anchor_in_excerpt(
                                group.excerpts[group.primary_excerpt_ix].clone(),
                                group.primary_diagnostic.range.start,
                            )
                            .to_offset(&excerpts_snapshot);
                        selection.start = offset;
                        selection.end = offset;
                    }
                }
            }
            editor.change_selections(None, cx, |s| {
                s.select(selections);
            });
            Some(())
        });

        if self.path_states.is_empty() {
            if self.editor.is_focused(cx) {
                cx.focus_self();
            }
        } else if cx.handle().is_focused(cx) {
            cx.focus(&self.editor);
        }
        cx.notify();
    }

    fn update_title(&mut self, cx: &mut ViewContext<Self>) {
        self.summary = self.project.read(cx).diagnostic_summary(cx);
        cx.emit(Event::TitleChanged);
    }
}

impl Item for ProjectDiagnosticsEditor {
    fn tab_content<T: View>(
        &self,
        _detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<T> {
        render_summary(
            &self.summary,
            &style.label.text,
            &theme::current(cx).project_diagnostics,
        )
    }

    fn for_each_project_item(&self, cx: &AppContext, f: &mut dyn FnMut(usize, &dyn project::Item)) {
        self.editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &AppContext) -> bool {
        false
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.added_to_workspace(workspace, cx));
    }

    fn navigate(&mut self, data: Box<dyn Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, cx))
    }

    fn tab_tooltip_text(&self, _: &AppContext) -> Option<Cow<str>> {
        Some("Project Diagnostics".into())
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
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(project, cx)
    }

    fn reload(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.editor.reload(project, cx)
    }

    fn save_as(
        &mut self,
        _: ModelHandle<Project>,
        _: PathBuf,
        _: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn to_item_events(event: &Self::Event) -> SmallVec<[ItemEvent; 2]> {
        Editor::to_item_events(event)
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
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Some(ProjectDiagnosticsEditor::new(
            self.project.clone(),
            self.workspace.clone(),
            cx,
        ))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a ViewHandle<Self>,
        _: &'a AppContext,
    ) -> Option<&AnyViewHandle> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle)
        } else if type_id == TypeId::of::<Editor>() {
            Some(&self.editor)
        } else {
            None
        }
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| editor.deactivated(cx));
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("diagnostics")
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft { flex: None }
    }

    fn deserialize(
        project: ModelHandle<Project>,
        workspace: WeakViewHandle<Workspace>,
        _workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<Result<ViewHandle<Self>>> {
        Task::ready(Ok(cx.add_view(|cx| Self::new(project, workspace, cx))))
    }
}

fn diagnostic_header_renderer(diagnostic: Diagnostic) -> RenderBlock {
    let (message, highlights) = highlight_diagnostic_message(Vec::new(), &diagnostic.message);
    Arc::new(move |cx| {
        let settings = settings::get::<ThemeSettings>(cx);
        let theme = &settings.theme.editor;
        let style = theme.diagnostic_header.clone();
        let font_size = (style.text_scale_factor * settings.buffer_font_size(cx)).round();
        let icon_width = cx.em_width * style.icon_width_factor;
        let icon = if diagnostic.severity == DiagnosticSeverity::ERROR {
            Svg::new("icons/circle_x_mark_12.svg")
                .with_color(theme.error_diagnostic.message.text.color)
        } else {
            Svg::new("icons/triangle_exclamation_12.svg")
                .with_color(theme.warning_diagnostic.message.text.color)
        };

        Flex::row()
            .with_child(
                icon.constrained()
                    .with_width(icon_width)
                    .aligned()
                    .contained()
                    .with_margin_right(cx.gutter_padding),
            )
            .with_children(diagnostic.source.as_ref().map(|source| {
                Label::new(
                    format!("{source}: "),
                    style.source.label.clone().with_font_size(font_size),
                )
                .contained()
                .with_style(style.message.container)
                .aligned()
            }))
            .with_child(
                Label::new(
                    message.clone(),
                    style.message.label.clone().with_font_size(font_size),
                )
                .with_highlights(highlights.clone())
                .contained()
                .with_style(style.message.container)
                .aligned(),
            )
            .with_children(diagnostic.code.clone().map(|code| {
                Label::new(code, style.code.text.clone().with_font_size(font_size))
                    .contained()
                    .with_style(style.code.container)
                    .aligned()
            }))
            .contained()
            .with_style(style.container)
            .with_padding_left(cx.gutter_padding)
            .with_padding_right(cx.gutter_padding)
            .expanded()
            .into_any_named("diagnostic header")
    })
}

pub(crate) fn render_summary<T: View>(
    summary: &DiagnosticSummary,
    text_style: &TextStyle,
    theme: &theme::ProjectDiagnostics,
) -> AnyElement<T> {
    if summary.error_count == 0 && summary.warning_count == 0 {
        Label::new("No problems", text_style.clone()).into_any()
    } else {
        let icon_width = theme.tab_icon_width;
        let icon_spacing = theme.tab_icon_spacing;
        let summary_spacing = theme.tab_summary_spacing;
        Flex::row()
            .with_child(
                Svg::new("icons/circle_x_mark_12.svg")
                    .with_color(text_style.color)
                    .constrained()
                    .with_width(icon_width)
                    .aligned()
                    .contained()
                    .with_margin_right(icon_spacing),
            )
            .with_child(
                Label::new(
                    summary.error_count.to_string(),
                    LabelStyle {
                        text: text_style.clone(),
                        highlight_text: None,
                    },
                )
                .aligned(),
            )
            .with_child(
                Svg::new("icons/triangle_exclamation_12.svg")
                    .with_color(text_style.color)
                    .constrained()
                    .with_width(icon_width)
                    .aligned()
                    .contained()
                    .with_margin_left(summary_spacing)
                    .with_margin_right(icon_spacing),
            )
            .with_child(
                Label::new(
                    summary.warning_count.to_string(),
                    LabelStyle {
                        text: text_style.clone(),
                        highlight_text: None,
                    },
                )
                .aligned(),
            )
            .into_any()
    }
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
        DisplayPoint,
    };
    use gpui::{TestAppContext, WindowContext};
    use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity, PointUtf16, Unclipped};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_diagnostics(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

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
        let view = cx.add_view(window_id, |cx| {
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

        let fs = FakeFs::new(cx.background());
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

        let view = cx.add_view(window_id, |cx| {
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
        });

        // The first language server finishes
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_finished(server_id_1, cx);
        });

        // Only the first language server's diagnostics are shown.
        cx.foreground().run_until_parked();
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
            project.disk_based_diagnostics_finished(server_id_2, cx);
        });

        // Both language server's diagnostics are shown.
        cx.foreground().run_until_parked();
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
        cx.foreground().run_until_parked();
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
        cx.foreground().run_until_parked();
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
            cx.set_global(SettingsStore::test(cx));
            theme::init((), cx);
            language::init(cx);
            client::init_settings(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
        });
    }

    fn editor_blocks(editor: &ViewHandle<Editor>, cx: &mut WindowContext) -> Vec<(u32, String)> {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            snapshot
                .blocks_in_range(0..snapshot.max_point().row())
                .filter_map(|(row, block)| {
                    let name = match block {
                        TransformBlock::Custom(block) => block
                            .render(&mut BlockContext {
                                view_context: cx,
                                anchor_x: 0.,
                                scroll_x: 0.,
                                gutter_padding: 0.,
                                gutter_width: 0.,
                                line_height: 0.,
                                em_width: 0.,
                            })
                            .name()?
                            .to_string(),
                        TransformBlock::ExcerptHeader {
                            starts_new_buffer, ..
                        } => {
                            if *starts_new_buffer {
                                "path header block".to_string()
                            } else {
                                "collapsed context".to_string()
                            }
                        }
                    };

                    Some((row, name))
                })
                .collect()
        })
    }
}
