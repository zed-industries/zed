pub mod items;

use anyhow::Result;
use collections::{BTreeSet, HashSet};
use editor::{
    diagnostic_block_renderer,
    display_map::{BlockDisposition, BlockId, BlockProperties, RenderBlock},
    highlight_diagnostic_message, Editor, ExcerptId, MultiBuffer, ToOffset,
};
use gpui::{
    action, elements::*, fonts::TextStyle, keymap::Binding, AnyViewHandle, AppContext, Entity,
    ModelHandle, MutableAppContext, RenderContext, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use language::{
    Bias, Buffer, Diagnostic, DiagnosticEntry, DiagnosticSeverity, Point, Selection, SelectionGoal,
};
use project::{DiagnosticSummary, Project, ProjectPath};
use std::{
    any::{Any, TypeId},
    cmp::Ordering,
    mem,
    ops::Range,
    path::PathBuf,
    sync::Arc,
};
use util::TryFutureExt;
use workspace::{ItemHandle as _, ItemNavHistory, Settings, Workspace};

action!(Deploy);

const CONTEXT_LINE_COUNT: u32 = 1;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([Binding::new("alt-shift-D", Deploy, Some("Workspace"))]);
    cx.add_action(ProjectDiagnosticsEditor::deploy);
}

type Event = editor::Event;

struct ProjectDiagnosticsEditor {
    project: ModelHandle<Project>,
    workspace: WeakViewHandle<Workspace>,
    editor: ViewHandle<Editor>,
    summary: DiagnosticSummary,
    excerpts: ModelHandle<MultiBuffer>,
    path_states: Vec<PathState>,
    paths_to_update: BTreeSet<ProjectPath>,
}

struct PathState {
    path: ProjectPath,
    diagnostic_groups: Vec<DiagnosticGroupState>,
}

struct DiagnosticGroupState {
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

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        if self.path_states.is_empty() {
            let theme = &cx.global::<Settings>().theme.project_diagnostics;
            Label::new(
                "No problems in workspace".to_string(),
                theme.empty_message.clone(),
            )
            .aligned()
            .contained()
            .with_style(theme.container)
            .boxed()
        } else {
            ChildView::new(&self.editor).boxed()
        }
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        if !self.path_states.is_empty() {
            cx.focus(&self.editor);
        }
    }
}

impl ProjectDiagnosticsEditor {
    fn new(
        project_handle: ModelHandle<Project>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.subscribe(&project_handle, |this, _, event, cx| match event {
            project::Event::DiskBasedDiagnosticsFinished => {
                this.update_excerpts(cx);
                this.update_title(cx);
            }
            project::Event::DiagnosticsUpdated(path) => {
                this.paths_to_update.insert(path.clone());
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
        cx.subscribe(&editor, |_, _, event, cx| cx.emit(*event))
            .detach();

        let project = project_handle.read(cx);
        let paths_to_update = project.diagnostic_summaries(cx).map(|e| e.0).collect();
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
        this.update_excerpts(cx);
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

    fn update_excerpts(&mut self, cx: &mut ViewContext<Self>) {
        let paths = mem::take(&mut self.paths_to_update);
        let project = self.project.clone();
        cx.spawn(|this, mut cx| {
            async move {
                for path in paths {
                    let buffer = project
                        .update(&mut cx, |project, cx| project.open_buffer(path.clone(), cx))
                        .await?;
                    this.update(&mut cx, |view, cx| view.populate_excerpts(path, buffer, cx))
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
                .diagnostic_groups()
                .into_iter()
                .filter(|group| {
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
                    (Some(_), None) => to_remove = old_groups.next(),
                    (Some((_, old_group)), Some(new_group)) => {
                        let old_primary = &old_group.primary_diagnostic;
                        let new_primary = &new_group.entries[new_group.primary_ix];
                        match compare_diagnostics(old_primary, new_primary, &snapshot) {
                            Ordering::Less => to_remove = old_groups.next(),
                            Ordering::Equal => {
                                to_keep = old_groups.next();
                                new_groups.next();
                            }
                            Ordering::Greater => to_insert = new_groups.next(),
                        }
                    }
                }

                if let Some(group) = to_insert {
                    let mut group_state = DiagnosticGroupState {
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
                                    &prev_excerpt_id,
                                    buffer.clone(),
                                    [excerpt_start..excerpt_end],
                                    excerpts_cx,
                                )
                                .pop()
                                .unwrap();

                            prev_excerpt_id = excerpt_id.clone();
                            first_excerpt_id.get_or_insert_with(|| prev_excerpt_id.clone());
                            group_state.excerpts.push(excerpt_id.clone());
                            let header_position =
                                (excerpt_id.clone(), language::Anchor::build_min());

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
                                        position: (excerpt_id.clone(), entry.range.start.clone()),
                                        height: diagnostic.message.matches('\n').count() as u8 + 1,
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
                    excerpts.remove_excerpts(group_state.excerpts.iter(), excerpts_cx);
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
            editor.remove_blocks(blocks_to_remove, cx);
            let block_ids = editor.insert_blocks(
                blocks_to_add.into_iter().map(|block| {
                    let (excerpt_id, text_anchor) = block.position;
                    BlockProperties {
                        position: excerpts_snapshot.anchor_in_excerpt(excerpt_id, text_anchor),
                        height: block.height,
                        render: block.render,
                        disposition: block.disposition,
                    }
                }),
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
                .unwrap()
                .then_with(|| range_a.end.cmp(&range_b.end, &snapshot).unwrap())
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
                new_excerpt_ids_by_selection_id = editor.refresh_selections(cx);
                selections = editor.local_selections::<usize>(cx);
            }

            // If any selection has lost its position, move it to start of the next primary diagnostic.
            for selection in &mut selections {
                if let Some(new_excerpt_id) = new_excerpt_ids_by_selection_id.get(&selection.id) {
                    let group_ix = match groups.binary_search_by(|probe| {
                        probe.excerpts.last().unwrap().cmp(&new_excerpt_id)
                    }) {
                        Ok(ix) | Err(ix) => ix,
                    };
                    if let Some(group) = groups.get(group_ix) {
                        let offset = excerpts_snapshot
                            .anchor_in_excerpt(
                                group.excerpts[group.primary_excerpt_ix].clone(),
                                group.primary_diagnostic.range.start.clone(),
                            )
                            .to_offset(&excerpts_snapshot);
                        selection.start = offset;
                        selection.end = offset;
                    }
                }
            }
            editor.update_selections(selections, None, cx);
            Some(())
        });

        if self.path_states.is_empty() {
            if self.editor.is_focused(cx) {
                cx.focus_self();
            }
        } else {
            if cx.handle().is_focused(cx) {
                cx.focus(&self.editor);
            }
        }
        cx.notify();
    }

    fn update_title(&mut self, cx: &mut ViewContext<Self>) {
        self.summary = self.project.read(cx).diagnostic_summary(cx);
        cx.emit(Event::TitleChanged);
    }
}

impl workspace::Item for ProjectDiagnosticsEditor {
    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox {
        render_summary(
            &self.summary,
            &style.label.text,
            &cx.global::<Settings>().theme.project_diagnostics,
        )
    }

    fn project_path(&self, _: &AppContext) -> Option<project::ProjectPath> {
        None
    }

    fn project_entry_id(&self, _: &AppContext) -> Option<project::ProjectEntryId> {
        None
    }

    fn navigate(&mut self, data: Box<dyn Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, cx))
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.excerpts.read(cx).read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.excerpts.read(cx).read(cx).has_conflict()
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

    fn can_save_as(&self, _: &AppContext) -> bool {
        false
    }

    fn save_as(
        &mut self,
        _: ModelHandle<Project>,
        _: PathBuf,
        _: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn should_activate_item_on_event(event: &Self::Event) -> bool {
        Editor::should_activate_item_on_event(event)
    }

    fn should_update_tab_on_event(event: &Event) -> bool {
        matches!(event, Event::Saved | Event::Dirtied | Event::TitleChanged)
    }

    fn set_nav_history(&mut self, nav_history: ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn clone_on_split(&self, cx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(ProjectDiagnosticsEditor::new(
            self.project.clone(),
            self.workspace.clone(),
            cx,
        ))
    }

    fn act_as_type(
        &self,
        type_id: TypeId,
        self_handle: &ViewHandle<Self>,
        _: &AppContext,
    ) -> Option<AnyViewHandle> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.into())
        } else if type_id == TypeId::of::<Editor>() {
            Some((&self.editor).into())
        } else {
            None
        }
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| editor.deactivated(cx));
    }
}

fn diagnostic_header_renderer(diagnostic: Diagnostic) -> RenderBlock {
    let (message, highlights) = highlight_diagnostic_message(&diagnostic.message);
    Arc::new(move |cx| {
        let settings = cx.global::<Settings>();
        let theme = &settings.theme.editor;
        let style = &theme.diagnostic_header;
        let font_size = (style.text_scale_factor * settings.buffer_font_size).round();
        let icon_width = cx.em_width * style.icon_width_factor;
        let icon = if diagnostic.severity == DiagnosticSeverity::ERROR {
            Svg::new("icons/diagnostic-error-10.svg")
                .with_color(theme.error_diagnostic.message.text.color)
        } else {
            Svg::new("icons/diagnostic-warning-10.svg")
                .with_color(theme.warning_diagnostic.message.text.color)
        };

        Flex::row()
            .with_child(
                icon.constrained()
                    .with_width(icon_width)
                    .aligned()
                    .contained()
                    .boxed(),
            )
            .with_child(
                Label::new(
                    message.clone(),
                    style.message.label.clone().with_font_size(font_size),
                )
                .with_highlights(highlights.clone())
                .contained()
                .with_style(style.message.container)
                .with_margin_left(cx.gutter_padding)
                .aligned()
                .boxed(),
            )
            .with_children(diagnostic.code.clone().map(|code| {
                Label::new(code, style.code.text.clone().with_font_size(font_size))
                    .contained()
                    .with_style(style.code.container)
                    .aligned()
                    .boxed()
            }))
            .contained()
            .with_style(style.container)
            .with_padding_left(cx.gutter_padding + cx.scroll_x * cx.em_width)
            .expanded()
            .named("diagnostic header")
    })
}

pub(crate) fn render_summary(
    summary: &DiagnosticSummary,
    text_style: &TextStyle,
    theme: &theme::ProjectDiagnostics,
) -> ElementBox {
    if summary.error_count == 0 && summary.warning_count == 0 {
        Label::new("No problems".to_string(), text_style.clone()).boxed()
    } else {
        let icon_width = theme.tab_icon_width;
        let icon_spacing = theme.tab_icon_spacing;
        let summary_spacing = theme.tab_summary_spacing;
        Flex::row()
            .with_children([
                Svg::new("icons/diagnostic-summary-error.svg")
                    .with_color(text_style.color)
                    .constrained()
                    .with_width(icon_width)
                    .aligned()
                    .contained()
                    .with_margin_right(icon_spacing)
                    .named("no-icon"),
                Label::new(
                    summary.error_count.to_string(),
                    LabelStyle {
                        text: text_style.clone(),
                        highlight_text: None,
                    },
                )
                .aligned()
                .boxed(),
                Svg::new("icons/diagnostic-summary-warning.svg")
                    .with_color(text_style.color)
                    .constrained()
                    .with_width(icon_width)
                    .aligned()
                    .contained()
                    .with_margin_left(summary_spacing)
                    .with_margin_right(icon_spacing)
                    .named("warn-icon"),
                Label::new(
                    summary.warning_count.to_string(),
                    LabelStyle {
                        text: text_style.clone(),
                        highlight_text: None,
                    },
                )
                .aligned()
                .boxed(),
            ])
            .boxed()
    }
}

fn compare_diagnostics<L: language::ToOffset, R: language::ToOffset>(
    lhs: &DiagnosticEntry<L>,
    rhs: &DiagnosticEntry<R>,
    snapshot: &language::BufferSnapshot,
) -> Ordering {
    lhs.range
        .start
        .to_offset(&snapshot)
        .cmp(&rhs.range.start.to_offset(snapshot))
        .then_with(|| {
            lhs.range
                .end
                .to_offset(&snapshot)
                .cmp(&rhs.range.end.to_offset(snapshot))
        })
        .then_with(|| lhs.diagnostic.message.cmp(&rhs.diagnostic.message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{
        display_map::{BlockContext, TransformBlock},
        DisplayPoint, EditorSnapshot,
    };
    use gpui::TestAppContext;
    use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity, PointUtf16};
    use serde_json::json;
    use unindent::Unindent as _;
    use workspace::WorkspaceParams;

    #[gpui::test]
    async fn test_diagnostics(cx: &mut TestAppContext) {
        let params = cx.update(WorkspaceParams::test);
        let project = params.project.clone();
        let workspace = cx.add_view(0, |cx| Workspace::new(&params, cx));

        params
            .fs
            .as_fake()
            .insert_tree(
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

        project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/test", true, cx)
            })
            .await
            .unwrap();

        // Create some diagnostics
        project.update(cx, |project, cx| {
            project
                .update_diagnostic_entries(
                    PathBuf::from("/test/main.rs"),
                    None,
                    vec![
                        DiagnosticEntry {
                            range: PointUtf16::new(1, 8)..PointUtf16::new(1, 9),
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
                            range: PointUtf16::new(2, 8)..PointUtf16::new(2, 9),
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
                            range: PointUtf16::new(3, 6)..PointUtf16::new(3, 7),
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
                            range: PointUtf16::new(4, 6)..PointUtf16::new(4, 7),
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
                            range: PointUtf16::new(7, 6)..PointUtf16::new(7, 7),
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
                            range: PointUtf16::new(8, 6)..PointUtf16::new(8, 7),
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
        let view = cx.add_view(0, |cx| {
            ProjectDiagnosticsEditor::new(project.clone(), workspace.downgrade(), cx)
        });

        view.next_notification(&cx).await;
        view.update(cx, |view, cx| {
            let editor = view.editor.update(cx, |editor, cx| editor.snapshot(cx));

            assert_eq!(
                editor_blocks(&editor, cx),
                [
                    (0, "path header block".into()),
                    (2, "diagnostic header".into()),
                    (15, "collapsed context".into()),
                    (16, "diagnostic header".into()),
                    (25, "collapsed context".into()),
                ]
            );
            assert_eq!(
                editor.text(),
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
                    editor.selected_display_ranges(cx),
                    [DisplayPoint::new(12, 6)..DisplayPoint::new(12, 6)]
                );
            });
        });

        // Diagnostics are added for another earlier path.
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_started(cx);
            project
                .update_diagnostic_entries(
                    PathBuf::from("/test/consts.rs"),
                    None,
                    vec![DiagnosticEntry {
                        range: PointUtf16::new(0, 15)..PointUtf16::new(0, 15),
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
            project.disk_based_diagnostics_finished(cx);
        });

        view.next_notification(&cx).await;
        view.update(cx, |view, cx| {
            let editor = view.editor.update(cx, |editor, cx| editor.snapshot(cx));

            assert_eq!(
                editor_blocks(&editor, cx),
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
                editor.text(),
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
                    editor.selected_display_ranges(cx),
                    [DisplayPoint::new(19, 6)..DisplayPoint::new(19, 6)]
                );
            });
        });

        // Diagnostics are added to the first path
        project.update(cx, |project, cx| {
            project.disk_based_diagnostics_started(cx);
            project
                .update_diagnostic_entries(
                    PathBuf::from("/test/consts.rs"),
                    None,
                    vec![
                        DiagnosticEntry {
                            range: PointUtf16::new(0, 15)..PointUtf16::new(0, 15),
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
                            range: PointUtf16::new(1, 15)..PointUtf16::new(1, 15),
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
            project.disk_based_diagnostics_finished(cx);
        });

        view.next_notification(&cx).await;
        view.update(cx, |view, cx| {
            let editor = view.editor.update(cx, |editor, cx| editor.snapshot(cx));

            assert_eq!(
                editor_blocks(&editor, cx),
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
                editor.text(),
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

    fn editor_blocks(editor: &EditorSnapshot, cx: &AppContext) -> Vec<(u32, String)> {
        editor
            .blocks_in_range(0..editor.max_point().row())
            .filter_map(|(row, block)| {
                let name = match block {
                    TransformBlock::Custom(block) => block
                        .render(&BlockContext {
                            cx,
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
    }
}
